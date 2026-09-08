//! 🔖️ Typed integer newtype identifiers used throughout the crate. Kept as plain `u32` newtypes
//! (never raw `usize`) so pattern/tile/node/relation/decision/region/port indices can
//! never be silently swapped at a call site.

// #region 🔖️Macro
macro_rules! id_newtype {
    ($(#[$meta:meta])* $name:ident; $($method:ident: $scope:meta),*) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
        pub struct $name(pub u32);

        impl $name {
            $(id_newtype!(@method $method, $scope);)*
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
    (@method get, $scope:meta) => {
        /// 🔖️ The raw `u32` value.
        #[inline]
        #[cfg($scope)]
        pub fn get(self) -> u32 {
            self.0
        }
    };
    (@method index, $scope:meta) => {
        /// 🔢️ The value as an index into a slice or vector.
        #[inline]
        #[cfg($scope)]
        pub fn index(self) -> usize {
            self.0 as usize
        }
    };
    (@method from_index, $scope:meta) => {
        /// 🏷️ Builds an id from an index already bounded by its owning builder.
        #[inline]
        #[cfg($scope)]
        pub fn from_index(i: usize) -> Self {
            Self(i as u32)
        }
    };
}
// #endregion 🔖️Macro

// #region 🔖️Ids
id_newtype!(
    /// 🧩️ One distinct pattern/tile value a variable can be assigned (the WFC "value").
    PatternId; get: all(), index: all(), from_index: all()
);
#[cfg(test)]
id_newtype!(
    /// 🧱️ A tile identity as authored (may map to several `PatternId`s under symmetry expansion).
    TileId; get: test, index: test, from_index: test
);
id_newtype!(
    /// 📍️ One solver variable (grid cell or graph node). Distinct from `semio_framework_graph::NodeId`
    /// (a `u64`); the only conversion boundary is `GraphTopology::from_graph_view`.
    NodeId; get: all(), index: all(), from_index: all()
);
id_newtype!(
    /// ↔ One directed compatibility relation (e.g. "north", "+X", or a graph edge label).
    RelationId; get: all(), index: all(), from_index: test
);
#[cfg(test)]
id_newtype!(
    /// 🌳️ One search decision (a branch point in the backtracking tree).
    DecisionId;
);
id_newtype!(
    /// 🗺️ One named region/zone used for scoped constraints and priorities.
    RegionId; get: test
);
#[cfg(test)]
id_newtype!(
    /// 🔌️ One connector/socket slot on a tile or graph node.
    PortId;
);
// #endregion 🔖️Ids

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_index_roundtrip() {
        let p = PatternId::from_index(7);
        assert_eq!(p.index(), 7);
        assert_eq!(p.get(), 7);
        assert_eq!(format!("{p}"), "7");
    }

    #[test]
    fn id_ordering_and_equality() {
        let a = NodeId(1);
        let b = NodeId(2);
        assert!(a < b);
        assert_eq!(a, NodeId(1));
        assert_ne!(a, b);
    }

    #[test]
    fn id_serde_roundtrip() {
        let r = RelationId(42);
        let json = protocol::json::to_json_string(&r);
        let back: RelationId = protocol::json::from_json_str(&json).unwrap();
        assert_eq!(r, back);
    }
}
// #endregion 🔖️Tests
