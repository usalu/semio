//! 🧭️ Middle-row law: every ordered-collection delete/move kind is run on a three-row document whose MIDDLE row it removes or moves — the
//! committed diff is produced, the inverse restores the row at its ORIGINAL position, and the inverse diffs sum to the negative diff.

use super::Din18599Mutation;
use crate::{Din18599Diff, Din18599Snapshot};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn decode<T: serde::de::DeserializeOwned>(text: &str) -> T {
    serde_json::from_str(text).expect("committed JSON")
}

#[semio_framework_async_macros::async_test]
async fn deleting_or_moving_a_middle_row_restores_its_position_and_sums_to_the_negative_diff() {
    let mut kinds: Vec<PathBuf> = std::fs::read_dir(root()).expect("fixture directory").map(|entry| entry.expect("entry").path()).filter(|path| path.join("🧭️middle").is_dir()).collect();
    kinds.sort();
    assert!(!kinds.is_empty(), "no committed middle-row scenario");
    for kind in kinds {
        let bundle = kind.join("🧭️middle");
        let name = bundle.display().to_string();
        let before: Din18599Snapshot = decode(&read(&bundle.join("📸️snapshot/⬅️before/🔣️.json")));
        let after: Din18599Snapshot = decode(&read(&bundle.join("📸️snapshot/➡️after/🔣️.json")));
        let mutation: Din18599Mutation = decode(&read(&bundle.join("🦠️mutation/🔣️.json")));
        let committed: Din18599Diff = decode(&read(&bundle.join("🔺️diff/🔣️.json")));
        let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation, &before);
        assert!(raised.messages().is_empty(), "{name} raised {:?}", raised.messages());
        assert_eq!(*raised.diff(), committed, "{name}: the produced diff is the committed diff");
        assert_eq!(protocol::apply_diff(raised.diff(), &before).expect("apply"), after, "{name}: applied document");
        let mut state = after.clone();
        for step in <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::inverse(&mutation, &before).expect("valid retained mutation inverse fixture").iter().rev() {
            let undo = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(step, &state);
            state = protocol::apply_diff(undo.diff(), &state).expect("inverse step applies");
        }
        assert_eq!(state, before, "{name}: the inverse restores the row at its original position");
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
    }
}
