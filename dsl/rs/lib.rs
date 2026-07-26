//! 🧬 `dsl` — facade for the token-native declarative DSL engine. Technologies depend on this one
//! crate (plus `vcs` for the `DocumentDsl`/`OpText` trait definitions themselves) to get the
//! derive macros, the `DslField` binding trait primitive Rust types implement, and the `__rt`
//! runtime the generated code calls into.

// The derive macros emit `::dsl::...` paths so generated code reads identically regardless of
// which technology crate invokes them. That only resolves for the crates that depend on `dsl` as
// an external crate — which is every real consumer, but NOT this crate's own tests (a crate is
// never its own dependency). `extern crate self as dsl;` is the standard fix: it makes `::dsl`
// resolve to this crate even when the derive is exercised in-crate, as the `🧪Tests` region below does.
// Only needed for the in-crate tests, so it's cfg-gated to avoid an "unused extern crate" warning
// in ordinary (non-test) builds, where every real consumer already has `dsl` as a true dependency.
#[cfg(test)]
extern crate self as dsl;

pub use dsl_core::*;
pub use dsl_derive::{DslDocument, DslOps, DslRecord, DslScalar};
pub use dsl_schema::*;

//#region 🔖Field
/// @emoji 🔗 Bridges a concrete Rust field type to the engine's `Shape`/`FieldValue` — every
/// primitive implements it directly; `#[derive(DslRecord)]`/`#[derive(DslScalar)]` implement it
/// for technology-declared nested types, so composition (a record field whose type is another
/// derived record or enum) works transparently through the same trait.
pub trait DslField: Sized {
    fn shape() -> Shape;
    fn to_value(&self) -> FieldValue;
    fn from_value(value: &FieldValue) -> Result<Self, String>;
}

macro_rules! impl_dsl_field_int {
    ($ty:ty, $shape:expr, $variant:ident, $as_ty:ty) => {
        impl DslField for $ty {
            fn shape() -> Shape {
                $shape
            }
            fn to_value(&self) -> FieldValue {
                FieldValue::$variant(*self as $as_ty)
            }
            fn from_value(value: &FieldValue) -> Result<Self, String> {
                match value {
                    FieldValue::$variant(v) => Ok(*v as $ty),
                    other => Err(format!("expected {}, found {other:?}", stringify!($variant))),
                }
            }
        }
    };
}

impl_dsl_field_int!(i8, Shape::Int, Int, i64);
impl_dsl_field_int!(i16, Shape::Int, Int, i64);
impl_dsl_field_int!(i32, Shape::Int, Int, i64);
impl_dsl_field_int!(i64, Shape::Int, Int, i64);
impl_dsl_field_int!(isize, Shape::Int, Int, i64);
impl_dsl_field_int!(u8, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u16, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u32, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u64, Shape::UInt, UInt, u64);
impl_dsl_field_int!(usize, Shape::UInt, UInt, u64);

impl DslField for bool {
    fn shape() -> Shape {
        Shape::Bool
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Bool(*self)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Bool(b) => Ok(*b),
            other => Err(format!("expected Bool, found {other:?}")),
        }
    }
}

impl DslField for f32 {
    fn shape() -> Shape {
        Shape::Float
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Float(*self as f64)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Float(f) => Ok(*f as f32),
            other => Err(format!("expected Float, found {other:?}")),
        }
    }
}

impl DslField for f64 {
    fn shape() -> Shape {
        Shape::Float
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Float(*self)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Float(f) => Ok(*f),
            other => Err(format!("expected Float, found {other:?}")),
        }
    }
}

/// @emoji 🔤 `String` binds as quoted, escaped `Text` by default. Fields wanting the bare-token
/// `Ident` shape instead (identifiers/tags) opt in via `#[dsl(ident)]`, which the derive handles
/// by generating a direct `Shape::Ident`/`FieldValue::Ident` binding instead of routing through
/// this impl — so this blanket impl only ever needs to cover the `Text` case.
impl DslField for String {
    fn shape() -> Shape {
        Shape::Text
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Text(self.clone())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Text(s) => Ok(s.clone()),
            FieldValue::Ident(s) => Ok(s.clone()),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}
//#endregion 🔖Field

//#region 🔖Variants
/// @emoji 🌿 Bridges an enum whose variants are each their own keyword-tagged record — the type
/// bound for `#[dsl(statements)] Vec<T>` collection fields and for `#[derive(DslOps)]` operation
/// enums. `#[derive(DslEnum)]`-with-struct-variants and `#[derive(DslOps)]` both implement this.
pub trait DslVariants: Sized {
    fn variants() -> Vec<(String, RecordSpec)>;
    fn to_named_record(&self) -> (String, RecordValue);
    /// @emoji ⚠️ Returns `TextError` (not `String`, unlike [`DslField::from_value`]) so
    /// generated bodies can `?`-propagate it directly — this is the same error type
    /// `vcs::OpText::parse_op`/`vcs::DocumentDsl::parse_dsl` already return, and the derive's
    /// `#[dsl(statements)]` field codegen composes it without any conversion at every nesting depth.
    fn from_named_record(keyword: &str, record: &RecordValue) -> Result<Self, TextError>;
}
//#endregion 🔖Variants

//#region 🔖Runtime
/// @emoji ⚙️ Thin wrappers the derive-generated `impl vcs::DocumentDsl`/`impl vcs::OpText` bodies
/// call into — kept as free functions (not methods) so generated code never has to name this
/// crate's internal types, only `dsl::__rt::*`.
pub mod __rt {
    use super::*;

    pub fn parse_document_record(text: &str, spec: &RecordSpec) -> Result<RecordValue, TextError> {
        parse(text, spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })
    }

    pub fn print_document_record(value: &RecordValue, spec: &RecordSpec) -> String {
        print(value, spec, JoinMode::Document)
    }

    pub fn parse_inline_record(text: &str, spec: &RecordSpec) -> Result<RecordValue, TextError> {
        parse(text, spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })
    }

    pub fn print_inline_record(value: &RecordValue, spec: &RecordSpec) -> String {
        print(value, spec, JoinMode::Inline)
    }

    pub fn field_error(message: impl Into<String>) -> TextError {
        TextError::new(message, TextSpan::at(1, 1))
    }
}
//#endregion 🔖Runtime

//#region 🔖TestSupport
/// @emoji 🧪 Round-trip/property helpers every derived (or hand-declared) grammar's own tests
/// call — the facade-level analogue of `vcs::test_support`, scoped to the engine's own laws
/// rather than the VCS store's.
pub mod test_support {
    use super::*;

    /// @emoji 🔁 `parse(print(value)) == value` for a `RecordSpec` and an already-built `RecordValue`.
    pub fn assert_schema_round_trip(value: &RecordValue, spec: &RecordSpec) {
        let printed = print(value, spec, JoinMode::Document);
        let opts = ParseOptions::default();
        let reparsed = parse(&printed, spec, &opts).unwrap_or_else(|e| panic!("reparse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(value, &reparsed, "schema round trip diverged;\nprinted:\n{printed}");
    }

    /// @emoji ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)`.
    pub fn assert_idempotent(text: &str, spec: &RecordSpec) {
        let once = canonicalize(text, spec, &ParseOptions::default()).unwrap_or_else(|e| panic!("canonicalize failed: {e}"));
        let twice = canonicalize(&once, spec, &ParseOptions::default()).unwrap_or_else(|e| panic!("second canonicalize failed: {e}"));
        assert_eq!(once, twice, "canonicalization must be idempotent");
    }

    /// @emoji 📏 Document and Inline renders of the same value must parse back to equal values,
    /// and the Inline render must be exactly one line — the newline law, checked generically.
    pub fn assert_document_inline_agree(value: &RecordValue, spec: &RecordSpec) {
        let inline_text = print(value, spec, JoinMode::Inline);
        assert!(!inline_text.contains('\n'), "inline render must be one line: {inline_text:?}");
        let inline_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Inline };
        let reparsed = parse(&inline_text, spec, &inline_opts).unwrap_or_else(|e| panic!("inline reparse failed: {e}\ninline:\n{inline_text}"));
        assert_eq!(value, &reparsed, "Document and Inline renders must parse to the same value");
    }
}
//#endregion 🔖TestSupport

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_dsl_field_impls_round_trip() {
        assert_eq!(i32::from_value(&42i32.to_value()), Ok(42));
        assert_eq!(u64::from_value(&7u64.to_value()), Ok(7));
        assert_eq!(bool::from_value(&true.to_value()), Ok(true));
        assert_eq!(f64::from_value(&1.5f64.to_value()), Ok(1.5));
        assert_eq!(String::from_value(&"hi".to_string().to_value()), Ok("hi".to_string()));
    }

    // --- end-to-end derive tests: mirrors the norm-family "flat scalar document" worked example ---

    #[derive(Clone, Debug, PartialEq, DslScalar, serde::Serialize, serde::Deserialize)]
    enum ClimateZone {
        Cold,
        Temperate,
        Warm,
    }

    #[derive(Clone, Debug, PartialEq, DslDocument, serde::Serialize, serde::Deserialize)]
    #[dsl(extension = "derivedoc")]
    struct DerivedDocument {
        category: String,
        climate: ClimateZone,
        airtightness_n50: f64,
        occupants: u32,
        note: Option<String>,
    }

    #[test]
    fn derived_document_round_trips_through_vcs_document_dsl() {
        let doc = DerivedDocument { category: "external_wall".to_string(), climate: ClimateZone::Cold, airtightness_n50: 0.6, occupants: 4, note: None };
        let printed = <DerivedDocument as vcs::DocumentDsl>::print_dsl(&doc);
        assert!(!printed.contains("note"), "absent optional field must be omitted: {printed}");
        let parsed = <DerivedDocument as vcs::DocumentDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(parsed, doc, "derived DocumentDsl round trip diverged;\nprinted:\n{printed}");
    }

    #[test]
    fn derived_document_round_trips_with_optional_field_present() {
        let doc = DerivedDocument { category: "roof".to_string(), climate: ClimateZone::Warm, airtightness_n50: 1.2, occupants: 2, note: Some("re-inspect in 2027".to_string()) };
        let printed = <DerivedDocument as vcs::DocumentDsl>::print_dsl(&doc);
        let parsed = <DerivedDocument as vcs::DocumentDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(parsed, doc);
    }

    // --- end-to-end derive test: an Operation enum via #[derive(DslOps)] ---

    #[derive(Clone, Debug, PartialEq, DslOps, serde::Serialize, serde::Deserialize)]
    enum DerivedOperation {
        #[dsl(key = "setCategory")]
        SetCategory { category: String },
        #[dsl(key = "setAirtightness")]
        SetAirtightness { n50: f64 },
        #[dsl(key = "reset")]
        Reset,
    }

    #[test]
    fn derived_op_text_round_trips_every_variant_as_one_line() {
        let ops = vec![
            DerivedOperation::SetCategory { category: "roof".to_string() },
            DerivedOperation::SetAirtightness { n50: 0.9 },
            DerivedOperation::Reset,
        ];
        for op in ops {
            let printed = <DerivedOperation as vcs::OpText>::print_op(&op);
            assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
            let parsed = <DerivedOperation as vcs::OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
            assert_eq!(parsed, op, "OpText round trip diverged for {printed:?}");
        }
    }

    #[test]
    fn derived_document_dsl_satisfies_vcs_test_support_helpers() {
        let doc = DerivedDocument { category: "floor".to_string(), climate: ClimateZone::Temperate, airtightness_n50: 0.4, occupants: 3, note: None };
        vcs::test_support::assert_dsl_round_trip(&doc);
    }

    #[test]
    fn derived_op_satisfies_vcs_test_support_helpers() {
        vcs::test_support::assert_op_line_round_trip(&DerivedOperation::SetCategory { category: "wall".to_string() });
    }
}
//#endregion 🧪Tests
