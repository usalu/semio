mod dff_public_action_admission_tests {
    use super::*;

    fn padded_action(controller_id: &str, action_id: &str, bytes: usize) -> String {
        let mut body = format!(r#"{{"address":{{"pluginId":"fixture","appId":"{controller_id}","modeId":"mode","windowKindId":"window","windowInstanceId":"instance","actionId":"{action_id}"}},"arguments":{{}}}}"#);
        body.extend(std::iter::repeat(' ').take(bytes - body.len()));
        body
    }

    fn padded_command(controller_id: &str, command_id: &str, bytes: usize) -> String {
        let mut body = format!(r#"{{"address":{{"owner":{{"app":{{"pluginId":"fixture","appId":"{controller_id}"}}}},"commandId":"{command_id}"}},"arguments":{{}}}}"#);
        body.extend(std::iter::repeat(' ').take(bytes - body.len()));
        body
    }

    #[doc(hidden)]
pub fn owner(controller_id: &str) -> semio_framework_plugin_decl_app::tool_contract::ToolOwnerWitness {
        let owner_type_name = match controller_id {
            "s.draw.draw@1/*#editor" => "semio_framework_plugin::app::EditorApp<semio_s_plugin_draw::editor::draw::DrawPlayApp>",
            "s.flow.flow@1/*#editor" => "semio_framework_plugin::app::EditorApp<semio_s_plugin_flow::editor::flow::FlowPlayApp>",
            "s.forms.forms@1/*#editor" => "semio_framework_plugin::app::EditorApp<semio_s_plugin_forms::editor::forms::FormsPlayApp>",
            "s.remodel.remodel@1/*#editor" => "semio_framework_plugin::app::EditorApp<semio_s_plugin_remodel::editor::remodel::RemodelPlayApp>",
            _ => "unproved::Owner",
        };
        semio_framework_plugin_decl_app::tool_contract::ToolOwnerWitness::test_with_name(owner_type_name)
    }

    /// 🧾️ The interactive wire caps are no longer a frozen framework table keyed by verb: since the
    /// public predecode was rebuilt on `ArtifactToolPublicContract`, the bound IS the live app's own
    /// declared `max_raw_wire_bytes` for that exact `(owner, controller, tool)` registration. These are
    /// the rows the four Draw/Flow/Forms editors contribute, restated so the laws below drive the
    /// landed lookup instead of an empty contract slice (which declares NO bound at all, so every
    /// oversized body was admitted).
    fn dff_public_contracts() -> Vec<semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicContract> {
        [
            ("s.forms.forms@1/*#editor", "setTryValue", 16_384usize),
            ("s.forms.forms@1/*#editor", "setTryValueStep", 16_384),
            ("s.draw.draw@1/*#editor", "canvasPointerDown", 8_192),
            ("s.flow.flow@1/*#editor", "duplicateWidget", 8_192),
            ("s.flow.flow@1/*#editor", "duplicateWidgetStep", 8_192),
        ]
        .into_iter()
        .map(|(controller_id, tool_id, max_raw_wire_bytes)| semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicContract {
            owner: owner(controller_id),
            controller_id: controller_id.to_string(),
            tool_id: tool_id.to_string(),
            schema_id: format!("semio.{tool_id}.v1"),
            max_raw_wire_bytes,
            publication_lanes: &[],
        })
        .collect()
    }

    fn admit_action(body: &str, runtime_controller_id: &str) -> Result<(), Fault> {
        validate_public_action_envelope(body, owner(runtime_controller_id), runtime_controller_id, &dff_public_contracts())
    }

    fn admit_command(body: &str, runtime_controller_id: &str) -> Result<(), Fault> {
        validate_public_command_envelope(body, owner(runtime_controller_id), runtime_controller_id, &dff_public_contracts())
    }

    #[test]
    fn action_and_command_specific_dff_wire_caps_are_enforced_before_deserialization() {
        for (controller_id, command_id, limit) in [
            ("s.forms.forms@1/*#editor", "setTryValue", 16_384),
            ("s.forms.forms@1/*#editor", "setTryValueStep", 16_384),
            ("s.draw.draw@1/*#editor", "canvasPointerDown", 8_192),
            ("s.flow.flow@1/*#editor", "duplicateWidget", 8_192),
            ("s.flow.flow@1/*#editor", "duplicateWidgetStep", 8_192),
        ] {
            assert!(admit_action(&padded_action(controller_id, command_id, limit), controller_id).is_ok(), "{command_id} action must admit its exact maximum");
            assert!(admit_action(&padded_action(controller_id, command_id, limit + 1), controller_id).is_err(), "{command_id} action must reject one byte beyond its maximum");
            assert!(admit_command(&padded_command(controller_id, command_id, limit), controller_id).is_ok(), "{command_id} command must admit its exact maximum");
            assert!(admit_command(&padded_command(controller_id, command_id, limit + 1), controller_id).is_err(), "{command_id} command must reject one byte beyond its maximum");
        }
    }

    #[test]
    fn same_schema_and_id_cannot_inherit_another_controllers_public_limit() {
        let limit = 8_192;
        assert!(admit_action(&padded_action("s.draw.draw@1/*#editor", "canvasPointerDown", limit + 1), "s.draw.draw@1/*#editor").is_err());
        assert!(admit_action(&padded_action("s.other.draw@1/*#editor", "canvasPointerDown", limit + 1), "s.other.draw@1/*#editor").is_ok());
        assert!(admit_command(&padded_command("s.draw.draw@1/*#editor", "canvasPointerDown", limit + 1), "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(&padded_command("s.other.draw@1/*#editor", "canvasPointerDown", limit + 1), "s.other.draw@1/*#editor").is_ok());
    }

    #[test]
    fn malformed_and_hostile_strings_are_rejected_by_predecode_admission() {
        assert!(admit_action(r#"{"address":{"actionId":"canvasPointerDown""#, "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(r#"{"address":{"commandId":"canvasPointerDown""#, "s.draw.draw@1/*#editor").is_err());
        let hostile_action = format!(r#"{{"address":{{"actionId":"setTryValue"}},"arguments":{{"valueJson":"{}"}}}}"#, "x".repeat(MAX_PUBLIC_ACTION_STRING_BYTES + 1));
        let hostile_command = format!(r#"{{"address":{{"commandId":"setTryValue"}},"arguments":{{"valueJson":"{}"}}}}"#, "x".repeat(MAX_PUBLIC_ACTION_STRING_BYTES + 1));
        assert!(admit_action(&hostile_action, "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(&hostile_command, "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(r#"{"addr\u0065ss":{"commandId":"canvasPointerDown"},"arguments":{}}"#, "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(r#"{"address":{"command\u0049d":"canvasPointerDown"},"arguments":{}}"#, "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(r#"{"address":{"commandId":"canvasPointer\u0044own"},"arguments":{}}"#, "s.draw.draw@1/*#editor").is_err());
    }

    #[test]
    fn set_contributions_pack_keeps_body_cap_and_skips_string_cap() {
        let json = "x".repeat(200_000);
        let body = format!(r#"{{"address":{{"owner":{{"app":{{"pluginId":"procedural","appId":"generation3d"}}}},"commandId":"setContributions"}},"arguments":{{"json":"{json}","page":0,"pageCount":1}}}}"#);
        assert!(body.len() <= MAX_PUBLIC_ACTION_BODY_BYTES, "scoped pack fixture must fit the public body");
        assert!(admit_command(&body, "s.flow.flow@1/*#editor").is_ok(), "setContributions must cross as one body-bounded pack");
        let over = format!(r#"{{"address":{{"owner":{{"app":{{"pluginId":"procedural","appId":"generation3d"}}}},"commandId":"setContributions"}},"arguments":{{"json":"{}","page":0,"pageCount":1}}}}"#, "x".repeat(MAX_PUBLIC_ACTION_BODY_BYTES));
        assert!(over.len() > MAX_PUBLIC_ACTION_BODY_BYTES);
        assert!(admit_command(&over, "s.flow.flow@1/*#editor").is_err(), "setContributions must still refuse an over-body pack");
    }

    #[test]
    fn command_classifier_uses_only_the_exact_address_command_id() {
        let mut decoy = r#"{"address":{"owner":"os","commandId":"ordinary"},"arguments":{"commandId":"canvasPointerDown"}}"#.to_string();
        decoy.extend(std::iter::repeat(' ').take(8_193 - decoy.len()));
        assert!(admit_command(&decoy, "s.draw.draw@1/*#editor").is_ok());
    }

    #[test]
    fn retained_receiver_lookup_requires_a_live_instance_without_decoding_source() {
        let runtime=PluginRuntime::<semio_framework_plugin_reactor_app::NoPluginApp>::new(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy()).expect("explicit original mounted owner policy");
        let refusal=match runtime_instance_cell(&runtime,1){Err(refusal)=>refusal,Ok(_)=>panic!("absent original instance must refuse")};
        assert!(refusal.message.contains("unknown instance"));
    }

    #[test]
    fn finite_invocation_admission_refuses_malformed_and_hostile_original_sources() {
        let malformed=validate_public_command_json(r#"{"address":{"commandId":"setTryValue""#).expect_err("structurally incomplete original source");
        assert!(malformed.message.contains("structurally incomplete"));
        let hostile=format!(r#"{{"address":{{"owner":"os","commandId":"setTryValue"}},"arguments":{{"valueJson":"{}"}}}}"#, "x".repeat(MAX_PUBLIC_ACTION_STRING_BYTES+1));
        let refusal=validate_public_command_json(&hostile).expect_err("original string body admission");
        assert!(refusal.message.contains("oversized string"));
    }

    #[test]
    fn action_entry_points_drive_spawn_admit_instead_of_awaiting_the_retained_job() {
        let source = semio_framework_plugin_test_support::read_module_tree_source(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../⚛️reactor/🏃️runtime/🦀️.rs")).expect("the runtime module tree reads");
        let exchange_start = source.find("protocol::AppCommand::Command { seq, command, view_state }").expect("reactor command exchange");
        let exchange_end = source[exchange_start..].find("protocol::AppCommand::CommandText").map(|offset| exchange_start + offset).expect("reactor command exchange boundary");
        let exchange = &source[exchange_start..exchange_end];
        assert!(exchange.contains("runtime_instance_cell(runtime, instance_id)?"));
        assert!(exchange.contains("drive_self_waking_ready(instance.app.handle_action_invocation"));
        assert!(!exchange.contains("resolve_ready(instance.app.handle_action_invocation"));
        assert!(!exchange.contains("run_framework_reserved_job"));
    }
}
