use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_derive::ToValue;
#[allow(unused_imports)]
use crate::SequenceCamera;
#[allow(unused_imports)]
use semio_framework_value_derive::FromValue;
use crate::editor::sequence::modes::edit::windows::main::config::*;
use crate::editor::sequence::modes::edit::windows::script::transient::{SequenceScriptWindowTransient, SequenceScriptWindowTransientMutation, SequenceScriptWindowTransientOwner};
use protocol::{ColdMutation, Mutation, OpBinary, OpText};

#[semio_framework_async_macros::async_test]
async fn sequence_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config/🧪️tests/🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
    let base_config: SequenceMainWindowConfig = semio_framework_pack_json::from_json_str(&fixture["baseConfig"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let base_transient: SequenceScriptWindowTransient = semio_framework_pack_json::from_json_str(&fixture["baseTransient"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in fixture["configMutations"].as_array().unwrap() {
        let mutation: SequenceMainWindowConfigMutation = semio_framework_pack_json::from_json_str(&row["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(serde_json::Value::from(&semio_framework_value::ToValue::to_value(&mutation)), row["mutation"], "set-mutation wire stays byte-identical to the committed neutral fixture");
        let after = protocol::apply_diff_cold(mutation.diff(&base_config).diff(), &base_config).unwrap();
        let restored = mutation.inverse(&base_config).expect("valid retained mutation inverse fixture").into_iter().rev().fold(after, |state, inverse| protocol::apply_diff_cold(inverse.diff(&state).diff(), &state).unwrap());
        assert_eq!(restored, base_config);
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base_config).await;
        assert_eq!(SequenceMainWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(SequenceMainWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    for row in fixture["transientMutations"].as_array().unwrap() {
        let mutation: SequenceScriptWindowTransientMutation = semio_framework_pack_json::from_json_str(&row["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let after = protocol::apply_diff_cold(mutation.diff(&base_transient).diff(), &base_transient).unwrap();
        let restored = mutation.inverse(&base_transient).expect("valid retained mutation inverse fixture").into_iter().rev().fold(after, |state, inverse| protocol::apply_diff_cold(inverse.diff(&state).diff(), &state).unwrap());
        assert_eq!(restored, base_transient);
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base_transient).await;
        assert_eq!(SequenceScriptWindowTransientMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(SequenceScriptWindowTransientMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
}

/// 📥️ The set mutations of the main window config round-trip through the original binary receiving cursor and refuse a foreign source.
#[test]
fn sequence_main_window_config_original_receiving_round_trips_every_mutation() {
    use protocol::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{RetainedCloneGrant, RetainedWorkPoll};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
    use semio_framework_value_testing_admission::{bank_run, EffectBank, Wallet};
    let mut bank = EffectBank::new(Wallet::from_profile(&serde_json::json!({"items": 1 << 16, "copyBytes": 1 << 28, "capacityBytes": 1 << 28, "releaseBytes": 1 << 28, "depth": 1 << 16, "fuel": 1 << 16})));
    let grant = RetainedCloneGrant { maximum_items: 1 << 10, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 };
    let camera = SequenceCamera { x: 1.5, y: -2.0, zoom: 2.0 };
    for expected in [SequenceMainWindowConfigMutation::SetOrientation("topBottom".into()), SequenceMainWindowConfigMutation::SetCamera(camera)] {
        let source = expected.encode_op().unwrap();
        let mut accept = |_| true;
        let mut native = NativeDecodeControl::new(1 << 24, &mut accept);
        let (mut receiver, admitted) = SequenceMainWindowConfigMutation::admit_receiving(&source, (0, source.len()), grant).result.unwrap().unwrap();
        assert!(admitted.fits(grant));
        let mut turns = 0;
        while !receiver.is_ready() {
            assert!(receiver.step(&source, &mut native, grant.maximum_copy_bytes, grant).result.unwrap().fits(grant));
            turns += 1;
            assert!(turns < 4096);
        }
        let mut output = None;
        assert!(receiver.take_into(&mut output, grant).result.unwrap().fits(grant));
        assert_eq!(output.as_ref(), Some(&expected));
        while !receiver.terminal_is_empty() {
            let demand = receiver.close_demands(grant.maximum_copy_bytes).unwrap();
            native.charge(demand.capacity_bytes).unwrap();
            match bank_run(&mut bank, grant, 4, |effect| receiver.close_step(effect)).0 {
                RetainedWorkPoll::Complete(turn) => assert!(turn.result.unwrap().progress().fits(grant)),
                RetainedWorkPoll::Pending { .. } => panic!("funded window config receiver close stayed pending"),
            }
            turns += 1;
            assert!(turns < 8192);
        }
    }
}
