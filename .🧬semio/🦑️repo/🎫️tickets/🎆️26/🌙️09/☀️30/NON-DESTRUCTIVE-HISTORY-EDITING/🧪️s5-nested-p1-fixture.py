#!/usr/bin/env python3
"""🧒 S5-NESTED P1: the composed builder-contract fixture declares a member on its parent before registering it.

Usage: python3 🧪️s5-nested-p1-fixture.py check|land|restore
  check   -> verifies every anchor count against the live file, writes nothing
  land    -> keeps a pre-image under 🗑️generated/s5-nested/pre-p1/ and writes the file
  restore -> puts the pre-image back
Exact-string replacements with counted anchors: any drift fails closed and nothing is written.
"""
import pathlib
import shutil
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
TICKET = pathlib.Path(__file__).resolve().parent
TARGET = REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
PRE = TICKET / "🗑️generated/s5-nested/pre-p1/builder-contract.rs"


def register(receiver: str, child: str, construct: str, expect: str, constructor: str = "new_test_child") -> str:
    return f'{receiver}.register_child("slot", {child}, test_child_dialect().await, {constructor}({child}).await.expect("{construct}")).await.expect("{expect}");'


HELPER_ANCHOR = "    async fn new_test_child(id: &str) -> Result<TestMembers, store::VcsError> {"
HELPERS = '''    /// 📇️ The APP's half of a composition: declares `child_id` in the parent's one owned-child slot beside every member
    /// it already declares. The runtime admits a member only under the identity its parent's snapshot declares
    /// (`declared_child_reference`), so a law that registers a live member declares it first — exactly as a product
    /// parent's own leaf does before its member is opened.
    async fn declare_test_child(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str) {
        let mut children: Vec<String> = app.test_snapshot().await.slot.iter().map(|child| child.target.to_uri()).collect();
        children.push(ArtifactRef { artifact_id: child_id.into(), dialect: test_child_dialect().await }.to_uri());
        app.test_store_mut().await.dispatch(store::ArtifactCommand::Apply { mutations: vec![TestMutation::SetSlotChildren(SetSlotChildren { children })], transaction: None }).await.expect("the parent declares the member it is about to own");
    }

    /// 🤝️ [`declare_test_child`], then the RUNTIME's half: a fresh [`new_test_child`] registered as that declared member.
    async fn register_test_child(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str) {
        declare_test_child(app, child_id).await;
        app.register_child("slot", child_id, test_child_dialect().await, new_test_child(child_id).await.expect("construct child")).await.expect("register the declared child");
    }

'''

BARE = register("app", '"child-a"', "construct child-a", "register child-a", "new_bare_test_child")
REJECTED = 'let error = app.register_child("slot", "child-1", dialect, member).await.expect_err("direct transfer must not apply a deferred restore pin");'

REDUNDANT_SURVIVES = '''        let declared = ArtifactRef { artifact_id: "child-1".into(), dialect: test_child_dialect().await }.to_uri();
        app.test_store_mut()
            .await
            .dispatch(store::ArtifactCommand::Apply {
                mutations: vec![TestMutation::SetSlotChildren(SetSlotChildren { children: vec![declared] })],
                transaction: None,
            })
            .await
            .expect("the parent declares the member it owns");
        assert_eq!(app.test_snapshot().await.slot.len(), 1, "the live parent declares exactly its one member");
'''
KEPT_SURVIVES = '''        assert_eq!(app.test_snapshot().await.slot.len(), 1, "the live parent declares exactly its one member");
'''
REDUNDANT_MEDIA = '''        let declared = ArtifactRef { artifact_id: "child-1".into(), dialect: test_child_dialect().await }.to_uri();
        source
            .test_store_mut()
            .await
            .dispatch(store::ArtifactCommand::Apply { mutations: vec![TestMutation::SetSlotChildren(SetSlotChildren { children: vec![declared] })], transaction: None })
            .await
            .expect("the parent declares the member it owns");
'''

REPLACEMENTS = [
    (HELPER_ANCHOR, HELPERS + HELPER_ANCHOR, 1),
    (register("app", '"child-1"', "construct child", "register child"), 'register_test_child(&mut app, "child-1").await;', 5),
    (register("app", '"child-1"', "construct child", "register child seeds ownership"), 'register_test_child(&mut app, "child-1").await;', 1),
    (register("source", '"child-1"', "construct child", "register child"), 'register_test_child(&mut source, "child-1").await;', 1),
    (register("app", "child", "construct child", "register child"), "register_test_child(&mut app, child).await;", 2),
    (register("app", '"child-a"', "construct child-a", "register child-a"), 'register_test_child(&mut app, "child-a").await;', 1),
    (register("app", '"child-b"', "construct child-b", "register child-b"), 'register_test_child(&mut app, "child-b").await;', 1),
    (register("app", '"child-maximum"', "construct maximum child", "register maximum child"), 'register_test_child(&mut app, "child-maximum").await;', 1),
    (register("app", '"child-a"', "construct child", "register child seeds ownership"), 'register_test_child(&mut app, "child-a").await;', 1),
    (register("app", '"child-b"', "construct child", "register child seeds ownership"), 'register_test_child(&mut app, "child-b").await;', 1),
    (BARE, 'declare_test_child(&mut app, "child-a").await;\n        ' + BARE, 4),
    (REJECTED, 'declare_test_child(&mut app, "child-1").await;\n        ' + REJECTED, 1),
    (REDUNDANT_SURVIVES, KEPT_SURVIVES, 1),
    (REDUNDANT_MEDIA, "", 1),
]


def planned(text: str) -> str:
    for old, new, count in REPLACEMENTS:
        found = text.count(old)
        if found != count:
            raise SystemExit(f"anchor drift: expected {count}, found {found}: {old[:120]!r}")
        text = text.replace(old, new)
    return text


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "check"
    if mode == "restore":
        shutil.copyfile(PRE, TARGET)
        print(f"restored {TARGET.name} from {PRE}")
        return
    text = TARGET.read_text(encoding="utf-8")
    if "async fn declare_test_child(" in text:
        print("already landed: 0 files to write")
        return
    result = planned(text)
    print(f"{sum(count for _, _, count in REPLACEMENTS)} replacements over {len(REPLACEMENTS)} anchors; {len(text)} -> {len(result)} bytes")
    if mode == "land":
        PRE.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(TARGET, PRE)
        TARGET.write_text(result, encoding="utf-8")
        print(f"landed {TARGET}")


if __name__ == "__main__":
    main()
