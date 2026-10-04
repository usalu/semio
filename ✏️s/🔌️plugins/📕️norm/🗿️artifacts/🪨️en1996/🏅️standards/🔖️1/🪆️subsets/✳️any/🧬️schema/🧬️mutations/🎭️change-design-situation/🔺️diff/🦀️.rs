//! 🔺️ `change-design-situation` diff — sets the document's design situation; the same value is a `mutation.no-op`.

use super::ChangeDesignSituation;
use crate::{En1996Diff, En1996Snapshot};

pub fn diff(payload: &ChangeDesignSituation, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.design_situation == payload.new_design_situation {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The design situation already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { design_situation: Some(payload.new_design_situation), ..Default::default() })
}
