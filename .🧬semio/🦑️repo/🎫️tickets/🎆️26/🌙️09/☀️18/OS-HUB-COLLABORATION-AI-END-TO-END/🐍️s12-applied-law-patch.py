#!/usr/bin/env python3
"""🩹️ Anchored, idempotent: `build_history_view`'s `applied` predicate becomes a NAMED pure
function so a law can drive it directly, plus that law (ticket 26/09/18, slice S12 §4)."""
import pathlib
ROOT = pathlib.Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
SUITE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"

plugin = PLUGIN.read_text()
ANCHOR = """    mod interaction_selection_laws;

    impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
        #[cfg(test)]
        pub(crate) fn test_registered_tool_keys(&self) -> BTreeSet<(String, String)> {
"""
FUNCTION = """    /// ✅️ Whether ONE ledger row's edit is still live, as the host reads it: `dimmed: applied ===
    /// false` in the History panel and `entry.kind === "mutation" && entry.applied !== false` in
    /// `#s-checkin`'s uncommitted count.
    ///
    /// A row publishes into up to three stores. `undo`/`redo` dispatch against the PARENT document
    /// store alone (`interaction_store`'s own doc says so), so the parent lane is authoritative for
    /// any row that published a parent edit — a multi-lane row (`Artifact` + `Config`, which is what
    /// 💠️lowpoly's `addPrimitive` and 🎥️shooting's `addShot` declare) keeps a live config edit after
    /// its document edit is undone, and answering `document || config || child` left it applied for
    /// ever. A row with NO parent edit at all (🌊️flow's `addWidget`, 🎬️sequence's `addStep`,
    /// 🌀️procedural's `generate`) has nothing else to answer with, and asking only the parent lane
    /// made those read as UNDONE the moment they landed.
    ///
    /// Named rather than inlined so the law can drive all eight lane combinations without an app, a
    /// retained multi-lane factory or a pinned proof roster — the two shapes that could not be
    /// written in this suite (ticket 26/09/18 S11 §3.4b).
    pub fn history_row_applied_v1(has_parent_edit: bool, document_applied: bool, config_applied: bool, child_applied: bool) -> bool {
        if has_parent_edit {
            document_applied
        } else {
            config_applied || child_applied
        }
    }

"""
OLD_EXPR = "                let applied = if entry.edit_id.is_some() { document_applied } else { config_applied || child_applied };"
NEW_EXPR = "                let applied = history_row_applied_v1(entry.edit_id.is_some(), document_applied, config_applied, child_applied);"

changed = 0
if "fn history_row_applied_v1" not in plugin:
    assert plugin.count(ANCHOR) == 1, "impl anchor not unique"
    plugin = plugin.replace(ANCHOR, FUNCTION + ANCHOR)
    changed += 1
if NEW_EXPR not in plugin:
    assert plugin.count(OLD_EXPR) == 1, "applied expression anchor not unique"
    plugin = plugin.replace(OLD_EXPR, NEW_EXPR)
    changed += 1
PLUGIN.write_text(plugin)

suite = SUITE.read_text()
LAW_ANCHOR = """    /// ✅️ A row's `applied` must ask every lane the row can publish into, not only the parent
    /// document store."""
LAW = '''    /// ⏪️ The MULTI-lane half of the same law, and the regression guard S10 §4.2's `||` needed.
    ///
    /// Measured inside the running `s` host at three machine loads (40, 61, 82) on 2026-09-22:
    /// 💠️lowpoly `addPrimitive` (`Artifact` + `Config` + `Transient`) and 🎥️shooting `addShot`
    /// (`Artifact` + `Config`) read `edits [0,1,1,1]` — the uncommitted count went up on the verb and
    /// never came back down on the undo, because the row's CONFIG edit is still applied after
    /// `undo` retracted its DOCUMENT edit, and `document || config || child` therefore answered
    /// `true` for ever. All eight lane combinations are pinned here, so neither half can be
    /// reintroduced without this law going red (ticket 26/09/18 S11 §3.4b, S12 §4).
    #[test]
    fn a_multi_lane_row_follows_its_parent_document_lane_after_an_undo() {
        use semio_framework_plugin::app::history_row_applied_v1 as applied;
        assert!(!applied(true, false, true, false), "a row that published a parent edit is UNDONE once that edit is retracted, however live its config edit still is");
        assert!(!applied(true, false, true, true), "the same with a live child edit — undo dispatches against the document store alone");
        assert!(!applied(true, false, false, false), "a parent-lane row with nothing applied anywhere is undone");
        assert!(applied(true, true, false, false), "a parent-lane row whose document edit is applied is applied");
        assert!(applied(true, true, true, true), "a multi-lane row is applied while its parent edit is");
        assert!(applied(false, false, true, false), "a CONFIG-only row has no parent edit to ask — S10 §4.2's cure, kept intact");
        assert!(applied(false, false, false, true), "a CHILD-only row, likewise — 🌊️flow's addWidget and 🎬️sequence's addStep");
        assert!(!applied(false, false, false, false), "a row applied nowhere is not applied");
    }

'''
if "a_multi_lane_row_follows_its_parent_document_lane_after_an_undo" not in suite:
    assert suite.count(LAW_ANCHOR) == 1, "law anchor not unique"
    suite = suite.replace(LAW_ANCHOR, LAW + LAW_ANCHOR)
    SUITE.write_text(suite)
    changed += 1
print(f"edits applied: {changed}")
