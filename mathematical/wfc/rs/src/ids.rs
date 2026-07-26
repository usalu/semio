//! 🔖 Typed integer newtype identifiers used throughout the crate. Kept as plain `u32` newtypes
//! (never raw `usize`) so pattern/tile/node/relation/constraint/decision/region/port indices can
//! never be silently swapped at a call site.

// #region 🔖Macro
macro_rules! id_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize, serde::Deserialize)]
        pub struct $name(pub u32);

        impl $name {
            /// 🔖 The raw `u32` value.
            #[inline]
            pub const fn get(self) -> u32 {
                self.0
            }

            /// 🔖 The value as a `usize` index, for slice/vec indexing.
            #[inline]
            pub const fn index(self) -> usize {
                self.0 as usize
            }

            /// 🔖 Builds an id from a `usize` index (e.g. a loop counter). Truncates silently only
            /// if `i > u32::MAX`, which every builder in this crate rejects long before this point.
            #[inline]
            pub const fn from_index(i: usize) -> Self {
                Self(i as u32)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
// #endregion 🔖Macro

// #region 🔖Ids
id_newtype!(
    /// 🧩 One distinct pattern/tile value a variable can be assigned (the WFC "value").
    PatternId
);
id_newtype!(
    /// 🧱 A tile identity as authored (may map to several `PatternId`s under symmetry expansion).
    TileId
);
id_newtype!(
    /// 📍 One solver variable (grid cell or graph node). Distinct from `mathematical_graph::NodeId`
    /// (a `u64`); the only conversion boundary is `GraphTopology::from_graph_view`.
    NodeId
);
id_newtype!(
    /// ↔️ One directed compatibility relation (e.g. "north", "+X", or a graph edge label).
    RelationId
);
id_newtype!(
    /// 🧷 One registered global/soft constraint instance.
    ConstraintId
);
id_newtype!(
    /// 🌳 One search decision (a branch point in the backtracking tree).
    DecisionId
);
id_newtype!(
    /// 🗺️ One named region/zone used for scoped constraints and priorities.
    RegionId
);
id_newtype!(
    /// 🔌 One connector/socket slot on a tile or graph node.
    PortId
);
// #endregion 🔖Ids

// #region 🔖Tests
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
        let json = serde_json::to_string(&r).unwrap();
        let back: RelationId = serde_json::from_str(&json).unwrap();
        assert_eq!(r, back);
    }
}
// #endregion 🔖Tests
