//! 🔺️ Wires diff schema — the delta of the uninhabited parent vocabulary (design §12, §20.15).

//#region 🔖️Diff
/// 🕳️ No parent leaf exists, so no delta is ever produced: the diff carries nothing and applies as the identity. Its codec
/// surface is gone (audit F13); this facet stays only because every artifact schema descriptor names four facets.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub struct WiresDiff {}

impl protocol::MutationDiff<crate::WiresSnapshot> for WiresDiff {
    fn apply(&self, base: &crate::WiresSnapshot) -> protocol::MutationApplyResult<crate::WiresSnapshot> {
        Ok(base.clone())
    }
    fn absorb(&mut self, _other: Self) {}
}
//#endregion 🔖️Diff
