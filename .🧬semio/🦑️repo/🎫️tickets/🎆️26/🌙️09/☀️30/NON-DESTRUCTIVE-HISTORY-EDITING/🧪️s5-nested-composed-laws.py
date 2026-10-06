#!/usr/bin/env python3
"""🧒 S5-NESTED: three composed builder-contract laws follow the rules commit 670 put on disk (test-only).

Usage: python3 🧪️s5-nested-composed-laws.py check|land|restore
1. `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` persisted the parent pack and loaded the children
   afterwards (`LoadChildren`). Since the ownership closure validates every candidate parent, a parent that declares a member never
   loads without it: the law persists and reloads the document ARCHIVE (the route every host uses) and pins the refusal of the halved one.
2. `created_children_survive_absorb_into_the_child_store_map` absorbed a child its parent never declared, in a slot the parent does
   not have; the refusal dropped the live member. The law declares the child in the parent's real slot first.
3. `retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires` names what a group undo skipped.
Counted anchors, fails closed before the first write.
"""
import pathlib
import shutil
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
TICKET = pathlib.Path(__file__).resolve().parent
TARGET = REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
PRE = TICKET / "🗑️generated/s5-nested/pre-composed-laws/builder-contract.rs"
MARKER = "async fn drive_test_archive_load("

RELOAD_START = "    /// 🧒️ Registering a member is the RUNTIME's job; declaring it on the parent snapshot is the\n"
RELOAD_END = "    /// 🪆️ LAW (design §20.15, W-b): `child_head_packs`"
RELOAD_NEW = '''    /// 📥️ Drives one stepped document archive load on `app` to its terminal status and acknowledges it.
    async fn drive_test_archive_load(app: &mut VcsArtifactApp<TestApp, TestMembers>, operation: u64, archive: protocol::DocumentArchivePack) -> protocol::DocumentArchiveLoadStatus {
        PluginApp::begin_document_archive_load(app, operation, archive).expect("the runtime admits the archive");
        let status = loop {
            let status = PluginApp::poll_document_archive_load(app, operation).await.expect("poll");
            if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                break status;
            }
        };
        PluginApp::acknowledge_document_archive_load(app, operation).expect("acknowledge");
        status
    }

    /// 🧒️ Registering a member is the RUNTIME's job; declaring it on the parent snapshot is the
    /// APP's: the runtime admits a member only under the identity its parent's snapshot declares.
    ///
    /// 📤️ Persist exactly what the host does: the document archive (`ReadDocumentArchive`) — the
    /// parent pack and `.spr` plus one owner-stamped entry per live member (design §21.6: the
    /// recursive archive is the single carrier).
    ///
    /// 📥️ Reload into a FRESH app through the stepped archive load (`LoadDocumentArchive`). The
    /// child comes back as its OWN live store, at the value its own history ended on, owned by the
    /// reloaded document.
    ///
    /// 🚫️ The members travel WITH the parent: the same archive without them is refused by the
    /// ownership closure (`closure-rejected`), so a composed parent never loads half — there is no
    /// "parent first, children later" route.
    #[semio_framework_async_macros::async_test]
    async fn a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames() {
        let mut app = contract_composed_app_raw().await;
        register_test_child(&mut app, "child-1").await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta()).await.expect("composite edit");
        artifact_app_laws::settle_registered_typed_operation(&mut app, meta().instance_id).await.expect("the migrated composite gesture settles before it is persisted");
        assert_eq!(app.test_snapshot().await.slot.len(), 1, "the live parent declares exactly its one member");

        let archive = PluginApp::document_archive(&app).await.expect("the composed document archives its closure");
        let document = app.store.envelope().id.clone();
        assert_eq!(
            archive.members.iter().map(|entry| (entry.owner.parent.artifact_id.as_str(), entry.owner.slot.as_str(), entry.owner.child_id.as_str(), entry.reference.artifact_id.as_str())).collect::<Vec<_>>(),
            vec![(document.as_str(), "slot", "child-1", "child-1")],
            "the archive carries the one member, owner-stamped to the document"
        );

        let mut reloaded = contract_composed_app_raw().await;
        let status = drive_test_archive_load(&mut reloaded, 21, archive.clone()).await;
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{:?}", semio_framework_diagnostic::decode_fault_bytes(&status.fault));
        assert_eq!(reloaded.test_snapshot().await.slot.len(), 1, "the reloaded parent carries its own declaration through the pack");
        let child = reloaded.child_store("slot", "child-1").await.expect("child restored");
        let restored: TestSnapshot = <TestSnapshot as ArtifactPack>::decode_pack(&child.document_pack_bytes().await.expect("child pack")).expect("decode child");
        assert_eq!(restored.count, 7, "the reloaded child lost its own edit history");
        assert_eq!(child.owner_ref().map(|owner| (owner.parent.artifact_id, owner.slot, owner.child_id)), Some((document.clone(), "slot".to_string(), "child-1".to_string())), "the reloaded child is owned by the reloaded document");

        let mut halved = contract_composed_app_raw().await;
        let mut parent_alone = archive;
        parent_alone.members.clear();
        let refused = drive_test_archive_load(&mut halved, 22, parent_alone).await;
        let refusal = format!("{:?}", semio_framework_diagnostic::decode_fault_bytes(&refused.fault));
        assert_eq!(refused.state, protocol::DocumentArchiveLoadState::Fault, "a parent that declares a member never loads without it: {refusal}");
        assert!(refusal.contains("closure-rejected"), "the ownership closure names the refusal: {refusal}");
        assert!(halved.test_snapshot().await.slot.is_empty() && halved.child_store("slot", "child-1").await.is_none(), "a refused load leaves the open document as it was");

        drain_and_close_composed_fixture(&mut halved);
        drain_and_close_composed_fixture(&mut reloaded);
        drain_and_close_composed_fixture(&mut app);
    }

'''

REPLACEMENTS = [
    (
        '        let mut app = contract_composed_app().await;\n        let parent_id = app.store.envelope().id.clone();\n        app.composition.graph_mut().await.insert_owns(&parent_id, "genesisSlot", "genesis-child").await.expect("seed ownership so absorb\'s slot_of lookup resolves");\n',
        '        let mut app = contract_composed_app().await;\n        declare_test_child(&mut app, "genesis-child").await;\n        let parent_id = app.store.envelope().id.clone();\n        app.composition.graph_mut().await.insert_owns(&parent_id, "slot", "genesis-child").await.expect("seed ownership so absorb\'s slot_of lookup resolves");\n',
        1,
    ),
    (
        'app.children.get_mut(&("genesisSlot".to_string(), "genesis-child".to_string())).expect("genesis child absorbed into the live map under its real slot");',
        'app.children.get_mut(&("slot".to_string(), "genesis-child".to_string())).expect("genesis child absorbed into the live map under the slot its parent declares it in");',
        1,
    ),
    (
        '    /// 🌱️ Proves `VcsArtifactApp::absorb_created_children` — the mechanism a\n',
        '    /// 🧒️ The parent declares the created child first: absorb admits a member only under the\n    /// identity the published parent declares, like every other admission.\n    ///\n    /// 🌱️ Proves `VcsArtifactApp::absorb_created_children` — the mechanism a\n',
        1,
    ),
    (
        '            crate::app::settle_framework_reserved_admission(&mut active.app, admitted).await.expect("undo reserved-job commit");\n            artifact_app_laws::settle_registered_typed_operation(&mut active.app, id).await.expect("undo publication");\n            assert_eq!(active.app.snapshot().expect("undone parent snapshot").count, 0);\n',
        '            let undone = crate::app::settle_framework_reserved_admission(&mut active.app, admitted).await.expect("undo reserved-job commit");\n            artifact_app_laws::settle_registered_typed_operation(&mut active.app, id).await.expect("undo publication");\n            let skipped: Vec<String> = undone.diagnostics.iter().map(|diagnostic| diagnostic.message.clone()).collect();\n            assert!(skipped.is_empty(), "the group undo skips no member of its own gesture: {skipped:?}");\n            assert_eq!(active.app.snapshot().expect("undone parent snapshot").count, 0, "the parent\'s edit of the gesture is undone with its group (parent tail group {:?})", store::SpaceMember::tail_group_id(&active.app.store).await);\n',
        1,
    ),
]


def planned(text: str) -> str:
    if text.count(RELOAD_START) != 1 or text.count(RELOAD_END) != 1:
        raise SystemExit(f"region anchor drift: {text.count(RELOAD_START)} start, {text.count(RELOAD_END)} end")
    head, tail = text.index(RELOAD_START), text.index(RELOAD_END)
    if head >= tail:
        raise SystemExit("region anchors out of order")
    text = text[:head] + RELOAD_NEW + text[tail:]
    for old, new, count in REPLACEMENTS:
        if text.count(old) != count:
            raise SystemExit(f"anchor drift: expected {count}, found {text.count(old)}: {old[:110]!r}")
        text = text.replace(old, new)
    return text


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "check"
    if mode == "restore":
        shutil.copyfile(PRE, TARGET)
        print(f"restored {TARGET.name}")
        return
    text = TARGET.read_text(encoding="utf-8")
    if MARKER in text:
        print("already landed: 0 files to write")
        return
    result = planned(text)
    print(f"1 region + {len(REPLACEMENTS)} replacements; {len(text)} -> {len(result)} bytes")
    if mode == "land":
        PRE.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(TARGET, PRE)
        TARGET.write_text(result, encoding="utf-8")
        print(f"landed {TARGET.name}")


if __name__ == "__main__":
    main()
