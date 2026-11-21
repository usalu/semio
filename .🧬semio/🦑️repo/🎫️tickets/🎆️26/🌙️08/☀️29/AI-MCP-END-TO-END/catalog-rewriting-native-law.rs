//! 🧪️ Verifies the actual Trinity Rewriting window owner with its authored configuration witnesses.

use semio_os_kernel::{ColdMutation, OpBinary, OpText};
use semio_plugin::WindowConfigOwner;
use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::window_config::*;
use serde_json::Value;
use std::{collections::BTreeMap, env, fs, path::PathBuf};

fn assert_owner<O: WindowConfigOwner>() {
    assert!(O::native_pair_demands().unwrap().capacity_bytes > 0);
    assert!(O::native_operation_wire().original_receiving.is_some());
    assert_eq!(
        O::store_disposer_birth().unwrap().capacity_bytes,
        std::alloc::Layout::for_value(O::build_store_disposer().as_ref()).size(),
    );
}

fn main() {
    let root = PathBuf::from(env::args_os().nth(1).expect("fixture directory"));
    let fixture: Value = serde_json::from_str(
        &fs::read_to_string(root.join("🔬️window-config-ownership/🔣️.json")).unwrap(),
    ).unwrap();
    let base: RewritingWindowConfig = semio_framework_pack_json::from_json_str(
        &fixture["base"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
    ).unwrap();
    let mut windows = BTreeMap::from([
        (fixture["leftWindowId"].as_str().unwrap().to_owned(), base.clone()),
        (fixture["rightWindowId"].as_str().unwrap().to_owned(), base),
    ]);
    for row in fixture["cases"].as_array().unwrap() {
        let witness = row["witness"].as_str().unwrap();
        let mutation_file = root.join("🧬️mutations").join(witness)
            .join("🧾️wire-witness/🦠️mutation/🔣️.json");
        let mutation: RewritingWindowConfigMutation = semio_framework_pack_json::from_json_str(
            &fs::read_to_string(mutation_file).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject,
        ).unwrap();
        let window = row["windowId"].as_str().unwrap();
        let before = &windows[window];
        let after = semio_os_kernel::apply_diff_cold(mutation.diff(before).diff(), before).unwrap();
        let restored = mutation.inverse(before).unwrap().into_iter().fold(
            after.clone(),
            |state, inverse| semio_os_kernel::apply_diff_cold(inverse.diff(&state).diff(), &state).unwrap(),
        );
        assert_eq!(&restored, before);
        assert_eq!(RewritingWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(RewritingWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        semio_os_kernel::os_dsl::original_receiving::check_receiving_round_trip(
            std::slice::from_ref(&mutation),
        ).unwrap();
        windows.insert(window.to_owned(), after);
        for (id, state) in &windows {
            let actual: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(state)).unwrap();
            let expected = &row["expected"][id];
            assert_eq!(actual["lodMode"], expected["lodMode"]);
            assert_eq!(actual["camera"].is_null(), expected["camera"].is_null());
            for field in ["x", "y", "zoom"] {
                assert_eq!(actual["camera"][field].as_f64(), expected["camera"][field].as_f64());
            }
        }
    }
    assert_owner::<BeforeWindowConfigOwner>();
    assert_owner::<AfterWindowConfigOwner>();
    assert_owner::<LhsWindowConfigOwner>();
    assert_owner::<RhsWindowConfigOwner>();
    println!("[DEBUG] Trinity native receiving, inverse and four window owner quotes passed");
}
