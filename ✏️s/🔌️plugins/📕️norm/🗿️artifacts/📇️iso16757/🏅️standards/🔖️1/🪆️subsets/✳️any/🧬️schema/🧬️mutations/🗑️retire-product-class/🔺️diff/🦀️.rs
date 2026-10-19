//! 🔄️ Diff for `retire-product-class`.

use crate::retire_product_class::mutation::RetireProductClass;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_artifact_norm_iso16757_diff::{Iso16757Diff, Iso16757ProductClassesRows};

pub fn diff(payload: &RetireProductClass, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let Some(index) = base.catalogue.product_classes.iter().position(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("No entity with id \"{}\" exists.", payload.id),
            [payload.id.clone()],
        );
    };
    protocol::MutationOutcome::new(Iso16757Diff { product_classes: Some(Iso16757ProductClassesRows::removal(&base.catalogue.product_classes, index)), ..Default::default() })
}
