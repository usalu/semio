"""🎯️ The ONE one-step tool (design §22.32 (b), report `📓️s4-tools-a-report.md` § S5.14): `tool_once_emit` in the pure
tool-machine crate with its law, and `Emit::tool_once` in the plugin runtime, so a one-click tool of ANY plugin publishes ONE
tool transaction without a statechart or a new dependency of its own. Counted anchors, idempotent.

Usage (cwd: repo root): python3 🧪️s5-tools-once.py [--apply | --restore]   (`--apply` under `landing`; Rust only, no `serve`)
"""

import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
TM = "🧰️framework/🔨️modules/🛠️tool-machine"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"

ONCE = '''
//#region 🎯️Once
/// 🎯️ The ONE one-step tool (design §22.32): a single dispatch — a click that places, a keyboard nudge — yields `leaves` and
/// commits them as ONE tool transaction of `<app_id>#<verb>`, its ref minted from the admission's `authoring_seed` and
/// [`authoring_clock`]: the continuous-control machine driven as a press that opens and releases in one event
/// ([`node_drag_commit`]). Nothing yielded leaves zero trace; without an admission (a render or test view) the leaves publish
/// plainly.
pub fn tool_once_emit<M: Clone + 'static>(app_id: &str, verb: &str, authoring_seed: &str, leaves: Vec<M>) -> NodeDragEmit<M> {
    node_drag_emit(app_id, verb, authoring_seed, verb, leaves)
}
//#endregion 🎯️Once
'''

ONCE_LAW = '''
//#region 🎯️OnceLaws
/// 🎯️ LAW (design §22.32): a one-step tool dispatch is ONE transaction of its leaves stamped `<appId>#<verb>`; a dispatch
/// that yields nothing leaves zero trace; without an admission the leaves publish plainly.
#[test]
fn a_one_step_tool_commits_one_transaction_of_its_leaves() {
    let NodeDragEmit::Commit(transaction, leaves) = tool_once_emit("demo@1/*#editor", "place", "seed", vec![1, 2]) else { panic!("an admitted one-step dispatch commits") };
    assert!(transaction.id.starts_with("tx-"), "{transaction:?}");
    assert_eq!((transaction.tool.as_str(), leaves), ("demo@1/*#editor#place", vec![1, 2]));
    assert_eq!(tool_once_emit::<i64>("demo@1/*#editor", "place", "seed", Vec::new()), NodeDragEmit::Nothing);
    assert_eq!(tool_once_emit("demo@1/*#editor", "place", "", vec![3]), NodeDragEmit::Plain(vec![3]));
}
//#endregion 🎯️OnceLaws
'''

EMIT_ANCHOR = '''        pub fn commit_transaction(transaction: protocol::TransactionRef, artifact_mutations: Vec<Mutation>) -> Self {
            Self { artifact_mutations, transaction: Some(transaction), ..Default::default() }
        }
'''
EMIT_ONCE = '''
        /// 🎯️ ONE one-step tool dispatch (a click that places, a keyboard nudge): `artifact_mutations` through the
        /// framework's one-step tool machine (`semio_framework_tool_machine::tool_once_emit`) as ONE transaction of the tool
        /// `<app_id>#<verb>`, its ref minted from the admission's `authoring_seed` — the history lists it as one row whose
        /// mutations stay editable. Nothing yielded: the empty emission; no admission (a render or test view): a plain edit.
        pub fn tool_once(app_id: &str, verb: &str, authoring_seed: &str, artifact_mutations: Vec<Mutation>) -> Self
        where
            Mutation: Clone + 'static,
        {
            semio_framework_tool_machine::tool_once_emit(app_id, verb, authoring_seed, artifact_mutations).into()
        }
'''
EDITS = {
    f"{TM}/🦀️.rs": ("//#endregion 🔖️NodeDrag\n", "//#endregion 🔖️NodeDrag\n" + ONCE, ONCE),
    f"{TM}/🧪️tests/🔬️unit/🦀️.rs": ("//#endregion 🔖️NodeDragLaws\n", "//#endregion 🔖️NodeDragLaws\n" + ONCE_LAW, ONCE_LAW),
    PLUGIN: (EMIT_ANCHOR, EMIT_ANCHOR + EMIT_ONCE, EMIT_ONCE),
}


def main():
    restore, staged, moved = "--restore" in sys.argv, [], 0
    for relative, (anchor, replacement, block) in EDITS.items():
        path = REPO / relative
        text = path.read_text(encoding="utf-8")
        if text.count(anchor) != 1 or text.count(block) > 1:
            raise SystemExit(f"{relative}: anchor moved")
        if restore and text.count(block) == 1:
            text, moved = text.replace(block, ""), moved + 1
        elif not restore and text.count(block) == 0:
            text, moved = text.replace(anchor, replacement), moved + 1
        staged.append((path, text))
    if "--apply" in sys.argv or restore:
        for path, text in staged:
            path.write_text(text, encoding="utf-8")
    print(f"{'restored' if restore else 'applied' if '--apply' in sys.argv else 'would apply'} {moved} block(s) in {len(staged)} files")


if __name__ == "__main__":
    main()
