use semio_os_dsl::original_receiving::{ReceivingStock, RetainedCloneGrant, receive_to_completion};
use semio_os_spr::OpBinary;
use semio_s_artifact_gis_gisterrain_operations::schema::mutations::{ChangeExaggeration, GisTerrainMutation};

fn main() {
    let mutation = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 2.5 });
    let bytes = mutation.encode_op().unwrap();
    let stock = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4096 };
    for maximum_copy_bytes in [16_384, 32_768, 49_152, 65_536, 131_072] {
        let quantum = RetainedCloneGrant { maximum_items: 64, maximum_capacity_bytes: 16_777_216, maximum_release_bytes: 1_048_576, maximum_copy_bytes, maximum_depth: 4096 };
        let result = receive_to_completion::<GisTerrainMutation>(&bytes, (0, bytes.len()), ReceivingStock { stock, quantum, fuel: 1 << 20 });
        println!("[DEBUG] copy budget {maximum_copy_bytes}: {}", match result {
            Ok(value) => { assert_eq!(value, mutation); "exact replay".to_owned() }
            Err(error) => error.message().to_string(),
        });
    }
}
