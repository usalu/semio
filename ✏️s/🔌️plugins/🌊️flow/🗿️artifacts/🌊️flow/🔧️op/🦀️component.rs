//! ⚡️ Flow artifact — the operation type (constitutional: op).
//!
//! `FlowOperation`, its `protocol::Operation<FlowFixture>` impl and the private `apply_flow_operation` fn
//! all live in the shared flow kernel crate (`flow_core`, `🔖️Operations` region) alongside the
//! `FlowFixture` projection they mutate — see `🗿️artifacts/🌊️flow/🦀️component.rs` for why. Re-exported
//! here so every taxonomy node names an artifact-owned symbol instead of reaching into the kernel path.

//#region 🔖️Types
pub use flow_core::FlowOperation;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::FlowFixture;
    use protocol::{Operation, OperationDiff};

    /// ⚖️ LAW: every operation's `backwards` restores the pre-operation projection.
    #[test]
    fn set_layout_backwards_restores_the_base_projection() {
        let base = FlowFixture::default();
        let operation = FlowOperation::SetLayout { entries: Vec::new() };
        let forward = operation.diff(&base).apply(&base);
        let restored = operation.backwards(&base).iter().fold(forward, |projection, inverse| inverse.diff(&projection).apply(&projection));
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
