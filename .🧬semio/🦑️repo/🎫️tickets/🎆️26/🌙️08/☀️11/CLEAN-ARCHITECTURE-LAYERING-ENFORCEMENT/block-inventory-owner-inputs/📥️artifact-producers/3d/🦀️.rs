//! 🧱️ Production-owned 3d mutation inventory.
use semio_framework_os_kernel::Mutation;
use semio_framework_test_mutation_inventory::{inventory_for_command, replication_inventory_leaves, MutationInventoryContribution, MutationInventoryCoordinate, MutationInventoryLeaf, MutationInventoryProgress};
use std::io::Write;
const DOCUMENT_DESCRIPTORS: &[semio_framework_os_kernel::MutationLeafDescriptor] = <semio_s_artifact_block_3d::standards::v1::subsets::any::schema::mutations::Block3dMutation as Mutation<semio_s_artifact_block_3d::standards::v1::subsets::any::schema::snapshot::Block3dSnapshot>>::DESCRIPTORS;
const DOCUMENT_LEAVES: [MutationInventoryLeaf<'static>; DOCUMENT_DESCRIPTORS.len()] = replication_inventory_leaves(DOCUMENT_DESCRIPTORS);
const TRANSIENT_DESCRIPTORS: &[semio_framework_os_kernel::MutationLeafDescriptor] = <semio_s_artifact_block_3d::editor::block3d::modes::edit::windows::world::transient::Block3dWorldWindowTransientMutation as Mutation<semio_s_artifact_block_3d::editor::block3d::modes::edit::windows::world::transient::Block3dWorldWindowTransient>>::DESCRIPTORS;
const TRANSIENT_LEAVES: [MutationInventoryLeaf<'static>; TRANSIENT_DESCRIPTORS.len()] = replication_inventory_leaves(TRANSIENT_DESCRIPTORS);
fn main() {
    let args=std::env::args().skip(1).collect::<Vec<_>>();
    let contributions=[MutationInventoryContribution {coordinate:MutationInventoryCoordinate {artifact:"s.block.3d",standard:"1",subset:"any",surface:None,owner:"✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any"},aggregates:&[&DOCUMENT_LEAVES],excluded_owner_segments:&["👁️viewer","✏️editor"]},
        MutationInventoryContribution {coordinate:MutationInventoryCoordinate {artifact:"s.block.3d",standard:"1",subset:"any",surface:Some("✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient"),owner:"✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient"},aggregates:&[&TRANSIENT_LEAVES],excluded_owner_segments:&[]}];
    let mut last=0;
    let mut observe=|progress:MutationInventoryProgress| {
        if progress.completed_units>=last+1024 || progress.completed_units==0 {eprintln!("[mutation-inventory] units={} ownedBytes={} emittedBytes={}",progress.completed_units,progress.owned_bytes,progress.emitted_bytes);last=progress.completed_units;}
        Ok(())
    };
    match inventory_for_command("semio-s-artifact-block-3d-mutation-inventory",&contributions,&args,&mut observe) {
        Ok(output)=>{let stdout=std::io::stdout();let mut stdout=stdout.lock();if let Err(error)=stdout.write_all(&output).and_then(|_|stdout.write_all(b"\n")) {eprintln!("[mutation-inventory] {error}");std::process::exit(1);}},
        Err(error)=>{eprintln!("[mutation-inventory] {error}");std::process::exit(2);}
    }
}

