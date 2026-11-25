//! 🩹️ Declared-order sparse row patches of the norm diff crates: one ordered field list, one kind per field.
//!
//! The patch struct lists its fields exactly in the order the entries are declared, so the JSON key order of the Rust serialiser
//! is the order of the committed schema and of its TypeScript, protobuf and GraphQL twins (schema first).
//!
//! Entry kinds of `nested_patch!`:
//! - `leaf f: T` a direct leaf of the row;
//! - `nested f: T = (obj.inner)` a leaf of a nested struct every row owns;
//! - `nested_option f: T = (obj.inner)` a leaf behind an optional struct, refused when the struct is absent;
//! - `clearable f: T = (flag)` an optional row field, set by `f` and cleared by the companion `flag`;
//! - `clear flag: bool = (f)` the companion flag of a `clearable` field, declared next to it;
//! - `rows f: Rows = "label"` an owned row lane.

/// 🩹️ Sparse scalar patch of one keyed row: the struct, its commit, coalescing, inverse and emptiness; fields stay in declared order.
#[macro_export]
macro_rules! scalar_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { $($f:ident: $t:ty),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, $crate::__value_derive::ToValue, $crate::__value_derive::FromValue, $crate::__dsl_record_derive::DslRecord, $crate::__value::RetireOwned)]
        #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        pub struct $name { $(pub $f: Option<$t>),+ }

        impl $crate::__kernel::list_delta::RowPatch<$row> for $name {
            fn commit_into(&self, row: &mut $row, _capability: $crate::__kernel::ApplyCapability) -> Result<(), $crate::__kernel::MutationApplyError> {
                $(if let Some(value) = &self.$f { row.$f = value.clone(); })+
                Ok(())
            }
            fn absorb(&mut self, later: Self) { $(if later.$f.is_some() { self.$f = later.$f; })+ }
            fn inverse(&self, row: &$row) -> Self { Self { $($f: self.$f.as_ref().map(|_| row.$f.clone())),+ } }
            fn is_empty(&self) -> bool { true $(&& self.$f.is_none())+ }
        }
    };
}

/// 🩹️ Sparse patch of one keyed row from one ordered field list: the struct in declared order, its commit, coalescing, inverse and emptiness.
#[macro_export]
macro_rules! nested_patch {
    (@commit $s:ident $row:ident $cap:ident leaf $f:ident) => {
        if let Some(value) = &$s.$f { $row.$f = value.clone(); }
    };
    (@commit $s:ident $row:ident $cap:ident nested $f:ident = ($obj:ident . $inner:ident)) => {
        if let Some(value) = &$s.$f { $row.$obj.$inner = value.clone(); }
    };
    (@commit $s:ident $row:ident $cap:ident nested_option $f:ident = ($obj:ident . $inner:ident)) => {
        if let Some(value) = &$s.$f {
            $row.$obj.as_mut().ok_or_else(|| $crate::__kernel::MutationApplyError::new("mutation.apply.missing-target", format!("{} does not exist", stringify!($obj))))?.$inner = value.clone();
        }
    };
    (@commit $s:ident $row:ident $cap:ident clearable $f:ident = ($flag:ident)) => {
        if $s.$flag == Some(true) { $row.$f = None; } else if let Some(value) = &$s.$f { $row.$f = Some(value.clone()); }
    };
    (@commit $s:ident $row:ident $cap:ident clear $f:ident = ($of:ident)) => {};
    (@commit $s:ident $row:ident $cap:ident rows $f:ident = $label:literal) => {
        if let Some(rows) = &$s.$f { $row.$f = rows.commit_onto(&$row.$f, $cap).map_err(|error| error.under([$label]))?; }
    };

    (@absorb $s:ident $later:ident leaf $f:ident) => {
        if $later.$f.is_some() { $s.$f = $later.$f; }
    };
    (@absorb $s:ident $later:ident nested $f:ident = $x:tt) => {
        if $later.$f.is_some() { $s.$f = $later.$f; }
    };
    (@absorb $s:ident $later:ident nested_option $f:ident = $x:tt) => {
        if $later.$f.is_some() { $s.$f = $later.$f; }
    };
    (@absorb $s:ident $later:ident clearable $f:ident = ($flag:ident)) => {
        if $later.$flag == Some(true) { $s.$flag = Some(true); $s.$f = None; } else if $later.$f.is_some() { $s.$f = $later.$f; $s.$flag = None; }
    };
    (@absorb $s:ident $later:ident clear $f:ident = ($of:ident)) => {};
    (@absorb $s:ident $later:ident rows $f:ident = $label:literal) => {
        if let Some(theirs) = $later.$f {
            match $s.$f.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => $s.$f = Some(theirs),
            }
            $s.$f = $s.$f.take().filter(|rows| !rows.is_empty());
        }
    };

    (@inverse $s:ident $row:ident leaf $f:ident) => {
        $s.$f.as_ref().map(|_| $row.$f.clone())
    };
    (@inverse $s:ident $row:ident nested $f:ident = ($obj:ident . $inner:ident)) => {
        $s.$f.as_ref().map(|_| $row.$obj.$inner.clone())
    };
    (@inverse $s:ident $row:ident nested_option $f:ident = ($obj:ident . $inner:ident)) => {
        $s.$f.as_ref().and_then(|_| (|| Some($row.$obj.as_ref()?.$inner.clone()))())
    };
    (@inverse $s:ident $row:ident clearable $f:ident = ($flag:ident)) => {
        if $s.$f.is_some() || $s.$flag == Some(true) { $row.$f.clone() } else { None }
    };
    (@inverse $s:ident $row:ident clear $f:ident = ($of:ident)) => {
        if ($s.$of.is_some() || $s.$f == Some(true)) && $row.$of.is_none() { Some(true) } else { None }
    };
    (@inverse $s:ident $row:ident rows $f:ident = $label:literal) => {
        $s.$f.as_ref().map(|rows| rows.inverse(&$row.$f)).filter(|rows| !rows.is_empty())
    };

    (@empty $s:ident leaf $f:ident) => {
        $s.$f.is_none()
    };
    (@empty $s:ident nested $f:ident = $x:tt) => {
        $s.$f.is_none()
    };
    (@empty $s:ident nested_option $f:ident = $x:tt) => {
        $s.$f.is_none()
    };
    (@empty $s:ident clearable $f:ident = ($flag:ident)) => {
        ($s.$f.is_none() && $s.$flag != Some(true))
    };
    (@empty $s:ident clear $f:ident = ($of:ident)) => {
        true
    };
    (@empty $s:ident rows $f:ident = $label:literal) => {
        $s.$f.as_ref().map_or(true, |rows| rows.is_empty())
    };

    ($(#[$doc:meta])* $name:ident for $row:ty { $($kind:ident $f:ident : $t:ty $(= $x:tt)?),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, $crate::__value_derive::ToValue, $crate::__value_derive::FromValue, $crate::__dsl_record_derive::DslRecord, $crate::__value::RetireOwned)]
        #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        pub struct $name { $(pub $f: Option<$t>,)+ }

        impl $crate::__kernel::list_delta::RowPatch<$row> for $name {
            #[allow(unused_variables)]
            fn commit_into(&self, row: &mut $row, capability: $crate::__kernel::ApplyCapability) -> Result<(), $crate::__kernel::MutationApplyError> {
                $($crate::nested_patch!(@commit self row capability $kind $f $(= $x)?);)+
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                $($crate::nested_patch!(@absorb self later $kind $f $(= $x)?);)+
            }
            fn inverse(&self, row: &$row) -> Self {
                Self { $($f: $crate::nested_patch!(@inverse self row $kind $f $(= $x)?),)+ }
            }
            fn is_empty(&self) -> bool { true $(&& $crate::nested_patch!(@empty self $kind $f $(= $x)?))+ }
        }
    };
}

#[cfg(test)]
mod tests {
    use semio_os_kernel::list_delta::RowPatch;

    #[derive(Clone, Debug, Default, PartialEq)]
    struct Inner {
        size: f64,
        count: u32,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct Row {
        name: String,
        inner: Inner,
        maybe: Option<Inner>,
        tight: Option<u8>,
        width: f64,
    }

    nested_patch! {
        /// 🧪️ Patch of the test row with every entry kind interleaved against the order of the kinds.
        MixedPatch for Row {
            leaf name: String,
            nested inner_size: f64 = (inner.size),
            leaf width: f64,
            nested_option maybe_count: u32 = (maybe.count),
            clearable tight: u8 = (clear_tight),
            clear clear_tight: bool = (tight),
            nested inner_count: u32 = (inner.count),
        }
    }

    #[test]
    fn the_patch_serialises_its_keys_in_declared_order() {
        assert_eq!(
            serde_json::to_string(&MixedPatch::default()).unwrap(),
            r#"{"name":null,"innerSize":null,"width":null,"maybeCount":null,"tight":null,"clearTight":null,"innerCount":null}"#
        );
    }

    #[test]
    fn a_patch_is_empty_until_it_sets_a_field_or_raises_the_clear_flag() {
        assert!(MixedPatch::default().is_empty());
        assert!(MixedPatch { clear_tight: Some(false), ..Default::default() }.is_empty());
        assert!(!MixedPatch { clear_tight: Some(true), ..Default::default() }.is_empty());
        assert!(!MixedPatch { inner_count: Some(1), ..Default::default() }.is_empty());
    }

    #[test]
    fn absorb_keeps_the_later_setter_and_a_later_clear_wins_over_an_earlier_set() {
        let mut patch = MixedPatch { name: Some("a".into()), tight: Some(1), ..Default::default() };
        patch.absorb(MixedPatch { name: Some("b".into()), width: Some(2.0), ..Default::default() });
        assert_eq!(patch, MixedPatch { name: Some("b".into()), width: Some(2.0), tight: Some(1), ..Default::default() });
        patch.absorb(MixedPatch { clear_tight: Some(true), ..Default::default() });
        assert_eq!(patch, MixedPatch { name: Some("b".into()), width: Some(2.0), clear_tight: Some(true), ..Default::default() });
        patch.absorb(MixedPatch { tight: Some(3), ..Default::default() });
        assert_eq!(patch, MixedPatch { name: Some("b".into()), width: Some(2.0), tight: Some(3), ..Default::default() });
    }

    #[test]
    fn the_inverse_reads_the_absolute_pre_values_of_the_carried_fields() {
        let row = Row { name: "a".into(), inner: Inner { size: 5.0, count: 7 }, maybe: Some(Inner { size: 1.0, count: 9 }), tight: None, width: 3.0 };
        let patch = MixedPatch { name: Some("x".into()), inner_size: Some(1.0), maybe_count: Some(2), tight: Some(4), ..Default::default() };
        assert_eq!(
            patch.inverse(&row),
            MixedPatch { name: Some("a".into()), inner_size: Some(5.0), maybe_count: Some(9), clear_tight: Some(true), ..Default::default() }
        );
        assert_eq!(patch.inverse(&Row { maybe: None, ..row }).maybe_count, None);
    }
}
