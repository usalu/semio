//! 🏭️ Owner-local production mutation inventory; all coordinates and descriptors belong to this artifact.
use semio_framework_os_kernel as protocol;
use protocol::Mutation;
use semio_framework_test_mutation_inventory::{inventory_for_command, replication_inventory_leaves, MutationInventoryContribution, MutationInventoryCoordinate, MutationInventoryError, MutationInventoryLeaf, MutationInventoryProgress};
use std::io::Write;

const PRODUCED_BY: &str = "semio-wfc-mutation-bridge";
const LEAVES_0: [MutationInventoryLeaf<'static>; <semio_s_artifact_wfc_bitmap::editor::bitmap::transient::BitmapTransientMutation as Mutation<semio_s_artifact_wfc_bitmap::editor::bitmap::transient::BitmapTransient>>::DESCRIPTORS.len()] = replication_inventory_leaves(<semio_s_artifact_wfc_bitmap::editor::bitmap::transient::BitmapTransientMutation as Mutation<semio_s_artifact_wfc_bitmap::editor::bitmap::transient::BitmapTransient>>::DESCRIPTORS);
const LEAVES_1: [MutationInventoryLeaf<'static>; <semio_s_artifact_wfc_bitmap::standards::v1::subsets::any::schema::mutations::BitmapMutation as Mutation<semio_s_artifact_wfc_bitmap::standards::v1::subsets::any::schema::snapshot::BitmapSnapshot>>::DESCRIPTORS.len()] = replication_inventory_leaves(<semio_s_artifact_wfc_bitmap::standards::v1::subsets::any::schema::mutations::BitmapMutation as Mutation<semio_s_artifact_wfc_bitmap::standards::v1::subsets::any::schema::snapshot::BitmapSnapshot>>::DESCRIPTORS);

const CONTRIBUTIONS: &[MutationInventoryContribution<'static>] = &[
    MutationInventoryContribution { coordinate: MutationInventoryCoordinate { artifact: "s.wfc.bitmap", standard: "1", subset: "any", surface: None, owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any" }, aggregates: &[&LEAVES_0, &LEAVES_1], excluded_owner_segments: &["👁️viewer", "✏️editor"] },
    MutationInventoryContribution { coordinate: MutationInventoryCoordinate { artifact: "s.wfc.bitmap", standard: "1", subset: "any", surface: Some("✏️editor/🫧️transient"), owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient" }, aggregates: &[&LEAVES_0, &LEAVES_1], excluded_owner_segments: &[] },
 ];

fn run() -> Result<(), MutationInventoryError> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let mut last = std::time::Instant::now();
    let mut observe = |progress: MutationInventoryProgress| {
        if last.elapsed().as_secs() >= 1 {
            eprintln!("[mutation-inventory] units={} bytes={}", progress.completed_units, progress.emitted_bytes);
            last = std::time::Instant::now();
        }
        Ok(())
    };
    let output = inventory_for_command(PRODUCED_BY, CONTRIBUTIONS, &arguments, &mut observe)?;
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&output).map_err(|_| MutationInventoryError::Output)?;
    stdout.write_all(b"\n").map_err(|_| MutationInventoryError::Output)
}

fn main() {
    if let Err(error) = run() { eprintln!("[mutation-inventory] {error}"); std::process::exit(1); }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
