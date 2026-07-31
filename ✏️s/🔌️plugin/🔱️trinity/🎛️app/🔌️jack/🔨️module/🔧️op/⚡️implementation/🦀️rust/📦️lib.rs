//! ⚡️ Trinity Jack app — operation type + laws (constitutional: op).
//!
//! 📌️ Jack has no bespoke operation enum: `trinity_ram::TrinityGraphOperation` is shared directly by
//! both the `jack` and `rewrite` apps (it already carries its own `Operation`/`OpText`/`OpBinary`
//! impls), so `Operation` here is a re-export, not a wrapper.

pub use trinity_ram::TrinityGraphOperation as Operation;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&Operation::Rename { id: "node-1".into(), name: "Renamed".into() });
    }
}
//#endregion 🧪️Tests
