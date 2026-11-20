//! 📦️ Layout archive replacement through the mounted application.

use super::*;
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_os_kernel::os_pack::snapshot::ArtifactPack;
use semio_plugin::{App, ArtifactApp, EditorApp};
use semio_s_artifact_draw_drawing_model::{DRAWING_DOCUMENT_SCHEMA, default_drawing_document};
use semio_s_artifact_layout_layout_host::LayoutDrawingMembers;
use store::os_store::{serialization::ArtifactPackFiles, text_format};

const STORE_REPLACEMENT_STAGE: u8 = 14;

fn prioritize_replacement_turn(
    app: &mut semio_plugin::VcsArtifactApp<EditorApp<LayoutPlayApp>, LayoutDrawingMembers>,
    operation: u64,
    caller: &mut artifact_app_laws::OriginalFixtureScope,
    policy: semio_plugin::MountedOwnerPolicyV1,
) {
    let grant = caller.cx().retained_grant();
    let fuel = artifact_app_laws::OriginalFixtureCaller::turn_fuel();
    for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        let handle = app.ingest.document_archive_loads.get(operation).and_then(|archive| archive.replacement);
        let waiting = handle.and_then(|handle| app.ingest.store_replacement_jobs.get(handle.operation.0))
            .is_some_and(|replacement| replacement.candidate_awaiting_members().is_some() && replacement.member_ingress.is_none());
        if waiting {
            app.maintenance.maintenance_stage = STORE_REPLACEMENT_STAGE;
        }
        artifact_app_laws::fixture_renew_turn(caller.cx(), fuel, grant).unwrap();
        PluginApp::maintenance_step(app, caller.cx(), policy.maintenance).unwrap();
        if waiting {
            let handle = handle.unwrap();
            let replacement = app.ingest.store_replacement_jobs.get(handle.operation.0).unwrap();
            assert!(replacement.candidate_awaiting_members().is_some(), "an ordinary replacement turn cannot seal the archive's pending Drawing roster");
            if replacement.member_ingress.is_some() { return; }
        }
    }
    panic!("the archive did not reach its member handoff boundary");
}

fn reference(value: &ArtifactRef) -> protocol::DocumentArchiveArtifactRef {
    protocol::DocumentArchiveArtifactRef {
        artifact_id: value.artifact_id.clone(),
        artifact_kind: value.dialect.artifact_kind.clone(),
        standard: value.dialect.standard.clone(),
        subset: value.dialect.subset.clone(),
    }
}

#[semio_framework_async_macros::async_test]
async fn persisted_layout_archives_publish_fresh_root_media_and_drawing_owners() {
    use artifact_app_laws::{OriginalFixtureScope, admit_document_archive_load, close_registered_fixture_app, new_app_with_registry_and_members, settle_document_archive_load};
    use semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document;

    fn manifest() -> App { App { definition: create_layout_app(), examples: Vec::new() } }
    type Mounted = EditorApp<LayoutPlayApp>;
    assert!(<Mounted as ArtifactApp>::build_envelope_decode_owner_bundle().is_some());
    let policy = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let transport: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️schema/🧬️mutations/🖊️set-page-background-drawing/🧪️tests/🔣️transport.json")).unwrap();
    let mut identity = artifact_app_laws::fixture_identity();
    let mut caller = OriginalFixtureScope::admit();
    let mut app = new_app_with_registry_and_members::<Mounted, LayoutDrawingMembers>(manifest, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), policy, &mut identity).await;

    for (operation, name) in [(201, "Archive root"), (202, "Archive linked media"), (203, "Archive Drawing"), (204, "Cancelled Drawing")] {
        let mut document = default_document();
        if operation == 202 {
            let linked = semio_s_artifact_layout_layout_examples::linked_asset::PRIMARY_TEXT;
            document = <LayoutSnapshot as store::os_pack::snapshot::ArtifactDsl>::parse_dsl(linked).expect("linked example document");
            assert!(!document.links.is_empty());
        }
        if operation >= 203 {
            document.pages[0].background_drawing = Some(semio_framework_pack_json::from_json_str(&transport["cases"][0]["input"]["SetPageBackgroundDrawing"]["drawing"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
        }
        document.name = name.into();
        let parent = ArtifactRef { artifact_id: app.stores.store.envelope().id.clone(), dialect: ArtifactDialect::from(<Mounted as ArtifactApp>::DIALECT) };
        let schema = <Mounted as ArtifactApp>::DOCUMENT_SCHEMA;
        let files = ArtifactPackFiles { pack: document.encode_pack(), spr: text_format::empty_document_spr(&parent.artifact_id, schema).await, ops: String::new() };
        let mut archive = PluginApp::document_load_archive(&app, &files).expect("registered parent identity stamp");
        if let Some(member) = document.pages[0].background_drawing.as_ref() {
            let owner = store::OwnerRef { parent, slot: "pages".into(), child_id: member.handle.child_id.clone() };
            let drawing = default_drawing_document(&member.handle.target.artifact_id, Some("Archive Drawing child"));
            archive.members.push(protocol::OwnedDocumentMemberPackEntry {
                ordinal: 0,
                reference: reference(&member.handle.target),
                owner: protocol::DocumentArchiveOwnerRef { parent: reference(&owner.parent), slot: owner.slot.clone(), child_id: owner.child_id.clone() },
                envelope_pack: text_format::genesis_member_envelope_pack(DRAWING_DOCUMENT_SCHEMA, &member.handle.target, &owner, &drawing.encode_pack()).await.unwrap(),
            });
        }
        let original_generation = app.artifact_generation_now();
        admit_document_archive_load(&mut app, operation, archive, caller.cx()).expect("exact original archive ingress");
        if operation >= 203 {
            prioritize_replacement_turn(&mut app, operation, &mut caller, policy);
        }
        if operation == 204 {
            PluginApp::cancel_document_archive_load(&mut app, operation).expect("cancel active Drawing archive after member handoff");
            assert!(settle_document_archive_load(&mut app, operation, &mut identity, caller.cx()).await.is_err());
            assert_eq!(app.artifact_generation_now(), original_generation);
            assert_eq!(app.snapshot().unwrap().name, "Archive Drawing");
            continue;
        }
        settle_document_archive_load(&mut app, operation, &mut identity, caller.cx()).await.expect("fresh archive owner publication");
        assert_ne!(app.artifact_generation_now(), original_generation);
        assert_eq!(app.snapshot().unwrap(), document);
        let emitted = PluginApp::document_archive(&app).await.expect("published archive is saveable");
        assert_eq!(emitted.members.len(), usize::from(operation == 203));
    }

    close_registered_fixture_app(&mut app, policy, caller.cx()).unwrap();
    caller.retire();
}

#[semio_framework_async_macros::async_test]
async fn fresh_json_envelope_applies_a_native_history_operation() {
    use semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document;
    use semio_s_artifact_layout_layout_mutations::LayoutMutation;
    use semio_s_artifact_layout_layout_mutations_content::replace_document::ReplaceDocument;
    use store::os_store::owned_schema_decode::{ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, ArtifactEnvelopeDecodePage};

    let mut initial = default_document();
    initial.name = "From native envelope".into();
    let mut edited = initial.clone();
    edited.name = "From native edit history".into();
    let mutation = LayoutMutation::ReplaceDocument(ReplaceDocument { document: edited.clone() });
    let hex = |bytes: &[u8]| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": <EditorApp<LayoutPlayApp> as ArtifactApp>::DOCUMENT_SCHEMA,
        "id": "layout-retained-envelope",
        "vcs": {
            "initialPack": hex(&initial.encode_pack()),
            "edits": [{
                "id": "layout-retained-edit",
                "actor": "layout-retained-actor",
                "line": null,
                "forwards": [hex(&protocol::OpBinary::encode_op(&mutation).unwrap())],
                "inverse": [hex(&protocol::OpBinary::encode_op(&LayoutMutation::ReplaceDocument(ReplaceDocument { document: initial })).unwrap())],
                "sequenceNumber": 1,
                "startedAt": "2026-08-23T00:00:00.000Z"
            }],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    })).unwrap();

    const PAGE_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    let mut app = layout_app().await;
    let generation = app.artifact_generation_now();
    let handle = app.begin_artifact_envelope_ingress(wire.len().div_ceil(PAGE_BYTES), wire.len()).unwrap();
    for chunk in wire.chunks(PAGE_BYTES) {
        let mut bytes = [0; PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).unwrap();
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _)| panic!("{fault:?}"));
    }
    assert!(app.0.seal_artifact_envelope_ingress(handle, app.1.cx()).unwrap());
    for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        use semio_plugin::ArtifactEnvelopeDecodeOperationPoll::{Cancelled, Fault, Ready};
        let fuel = artifact_app_laws::OriginalFixtureCaller::turn_fuel();
        let grant = app.1.cx().retained_grant();
        artifact_app_laws::fixture_renew_turn(app.1.cx(), fuel, grant).unwrap();
        app.0.maintenance_step(app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).unwrap();
        match app.0.advance_artifact_envelope_load(handle, app.1.cx()).unwrap() {
            Ready => {
                assert_ne!(app.artifact_generation_now(), generation);
                assert_eq!(app.snapshot().unwrap(), edited);
                assert!(app.acknowledge_artifact_store_replacement(handle).unwrap());
                app.close_fixture();
                return;
            }
            Cancelled | Fault => panic!("fresh Layout envelope refused: {:?}", app.artifact_store_replacement_refusal(handle)),
            _ => std::thread::yield_now(),
        }
    }
    panic!("fresh Layout envelope did not reach terminal publication");
}
