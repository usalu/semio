#!/usr/bin/env python3
"""🏠️ S19 set `load-rehome` (guest SDK, T6 round 4+): a document seeded through `AppCommand::LoadDocument` (the React
worker's hub/file open path and the MCP gateway's session seed, `🌉️mcp/🏠️workspace` `load_session_document`) kept the
BOOT document's composition: `load_document_pack`/`load_document_text` reset the parent store only, so every held child
stayed owned by the boot parent id in `self.composition`'s graph (and in its member/map owner refs). The first
owned-child edit then failed at commit — `dispatch_relation_group` phase 1: "ownership violation: <child> is not a
currently-tracked owned child of <loaded parent>" — measured on hub 7800 p33 (flow `addWidget`, sequence `addStep`,
G12 MCP + S18 React sweep) and reproduced natively (`s14-s19-logs/census-t7-1.txt`: sequence boot doc commits
`edit-736d…`, the same doc after `load_document_pack(artifact_app_genesis_pair(..))` → `transaction.commit-failed
ownership violation: sequence-content-13e3545f6a91fe1f …`). Now both loads re-home the composition on the loaded
parent (`rehome_composition`): a held child the loaded parent names at the same (slot, child id) — content-addressed,
so identical — moves its graph edge, member owner and map owner onto the loaded parent; every other held child is
retired; then the loaded parent's derivable children follow. Law (sequence):
`a_loaded_document_commits_an_owned_child_edit`. usage: s19-load-rehome.py [--dry-run|--write|--revert]"""
import importlib.util
import os
import sys

spec = importlib.util.spec_from_file_location("s19_setlib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "s19_setlib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
TESTS = "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"

TEXT_OLD = '''            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_text(&files.dsl, &files.ops).await.map_err(|error| error.into_fault())?;
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(loaded_with_app_dialect::<A>(parsed.into_envelope())).await.map_err(|error| error.into_fault())?;
            self.commit_document_window_reset(window_reset);
            self.cache = None;
            Ok(())
'''
TEXT_NEW = '''            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_text(&files.dsl, &files.ops).await.map_err(|error| error.into_fault())?;
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(loaded_with_app_dialect::<A>(parsed.into_envelope())).await.map_err(|error| error.into_fault())?;
            self.commit_document_window_reset(window_reset);
            self.cache = None;
            self.rehome_composition().await
'''
PACK_OLD = TEXT_OLD.replace("parse_document_text(&files.dsl, &files.ops)", "parse_document_pack(&files.pack, &files.spr)")
PACK_NEW = TEXT_NEW.replace("parse_document_text(&files.dsl, &files.ops)", "parse_document_pack(&files.pack, &files.spr)")

REHOME_ANCHOR = '''        pub fn artifact_generation_now(&self) -> semio_framework_job::Generation {
'''
REHOME = '''        /// 🏠️ A loaded document replaces the parent its composition hangs off: every held child the loaded parent names at
        /// the same (slot, child id) — content-addressed, so the same content — moves its ownership-graph edge, member
        /// owner and map owner onto the loaded parent; every other held child is retired; then the loaded parent's
        /// derivable children follow. Without it a `LoadDocument`-seeded document (hub/file open, MCP session seed) kept
        /// the boot parent's edges and its first owned-child edit failed "ownership violation: … is not a
        /// currently-tracked owned child" at commit.
        async fn rehome_composition(&mut self) -> Result<(), Fault> {
            let parent = ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: A::DIALECT.into() };
            let declared: Vec<(String, String)> = {
                let projection = store::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("loaded child projection failed: {error}")))?;
                (0..projection.len()).filter_map(|index| projection.get(index).map(|(slot, fields)| (slot.to_string(), fields.child_id.to_string()))).collect()
            };
            let held: Vec<(String, String)> = self.children.entries().filter(|entry| entry.owner.parent != parent).map(|entry| (entry.owner.slot.clone(), entry.reference.artifact_id.clone())).collect();
            for key in held {
                if !declared.contains(&key) {
                    self.retire_followed_child(&key.0, &key.1).await?;
                    continue;
                }
                let owner = store::OwnerRef { parent: parent.clone(), slot: key.0.clone(), child_id: key.1.clone() };
                self.composition.graph_mut().await.remove_owns(&key.1).await;
                self.composition.graph_mut().await.insert_owns(&parent.artifact_id, &key.0, &key.1).await.map_err(plugin_sdk_fault)?;
                let entry = self.children.get_mut(&key).ok_or_else(|| plugin_sdk_fault("re-homed child left the member map"))?;
                entry.member.set_owner(Some(owner.clone())).await;
                entry.owner = owner;
            }
            self.follow_derivable_children().await
        }

'''


def sdk(text):
    text = lib.replace_once(TEXT_OLD, TEXT_NEW)(text)
    text = lib.replace_once(PACK_OLD, PACK_NEW)(text)
    if "async fn rehome_composition(&mut self)" not in text:
        assert text.count(REHOME_ANCHOR) == 1, f"rehome anchor x{text.count(REHOME_ANCHOR)}"
        text = text.replace(REHOME_ANCHOR, REHOME + REHOME_ANCHOR)
    return text


LAW = '''/// 🏠️ LAW: a document seeded through `LoadDocument` (the hub/file open path and the MCP session seed) commits an
/// agent's owned-child edit. Before, the load kept the boot document's ownership edges and the commit failed
/// "ownership violation: sequence-content-… is not a currently-tracked owned child of artifact-…" (hub 7800 p33).
#[semio_framework_async_macros::async_test]
async fn a_loaded_document_commits_an_owned_child_edit() {
    use semio_framework_plugin::PluginApp;
    let mut app = context::new_app_with_registry_wired().await;
    let pair = semio_framework_plugin::artifact_app_genesis_pair::<semio_framework_plugin::EditorApp<super::SequencePlayApp>>("artifact-00000000000000000000000000000019").await.expect("genesis pair");
    app.load_document_pack(&pair).await.expect("load the seeded pair");
    let definition = super::create_sequence_app();
    semio_framework_plugin::artifact_app_laws::agent_preview(&mut app.0, &definition, "addStep", &semio_framework::DslValue::Object(Vec::new())).await.expect("addStep preview");
    let wire = app.take_last_emit_wire().await.expect("addStep emits its wire");
    let ops = if wire.document.is_empty() { Vec::new() } else { protocol::os_spr::causal::decode_ops_vec(&wire.document).expect("document ops") };
    assert!(!wire.children.is_empty(), "addStep edits the owned content child");
    let outcome = app.transaction_prepare("load-rehome", "", &[], &ops, &wire.children, "addStep", None).await;
    assert!(outcome.rejection.is_none(), "prepare: {:?}", outcome.rejection);
    app.transaction_commit("load-rehome", &semio_framework_plugin::artifact_app_laws::meta("agent")).await.expect("the loaded document's owned-child edit commits");
}

'''
LAW_ANCHOR = "//#endregion 🔖️HostTests\n"


def tests(text):
    if "fn a_loaded_document_commits_an_owned_child_edit" in text:
        return text
    assert text.count(LAW_ANCHOR) == 1, "HostTests region end"
    return text.replace(LAW_ANCHOR, LAW + LAW_ANCHOR)


lib.run("load-rehome", [(SDK, sdk), (TESTS, tests)], sys.argv[1:])
