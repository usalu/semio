//! 🦀️ EN 1997 mutation cross-check — Wave C rebuild.
//! Full fixture corpus will be regenerated after contract compile unblocks.
//! Per-kind apply/inverse coverage lives in each mutation leaf `🧪️tests/` directory.

#[test]
fn kinds_catalog_non_empty() {
    use crate::mutations::KINDS;
    assert_eq!(KINDS.len(), 20);
}
