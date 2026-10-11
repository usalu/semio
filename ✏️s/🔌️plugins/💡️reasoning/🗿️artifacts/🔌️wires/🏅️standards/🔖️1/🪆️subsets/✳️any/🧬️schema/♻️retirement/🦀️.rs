//! ♻️ Wires document retirement: the parent's identity layer, its board child handle and its meta retire as owned values.

use crate::{WiresMutation, WiresSnapshot};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};

impl RetireOwned for WiresSnapshot {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let WiresSnapshot { wires_snapshot, content, meta } = self;
        semio_framework_value::retirement::sequence(vec![wires_snapshot.retirement(), content.retirement(), meta.retirement()])
    }
}

impl RetireOwned for WiresMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {}
    }
}
