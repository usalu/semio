#!/usr/bin/env python3
"""🧪️ Emitter for the three flat scalar norm trees (en1996 / en1997 / en1995).

Each of those dispatch enums is a pure `change-<field>` vocabulary over a flat, id-less document
root, so every case is: one snapshot field moves, one diff field is `Some`, every sibling stays.
The per-leaf VALUES, case names, prose and bespoke assertions come from the caller's table — this
module writes the seven test bodies around them. Ticket scratch, not a permanent script.
"""

import importlib.util
import os
import textwrap

_spec = importlib.util.spec_from_file_location("emit_common", os.path.join(os.path.dirname(__file__), "\U0001f4dc️emit-common.py"))
common = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(common)


def scalar_case(*, artifact, artifact_dir, types, tag_style, leaf, base_snapshot, base_diff_nulls):
    """🔧️ Builds and writes one `change-<scalar>` fixture case."""
    snapshot_ty, mutation_ty, diff_ty = types
    kind = leaf["kind"]
    case = leaf["case"]
    label = "%s/%s" % (kind, case)

    after_snapshot = dict(base_snapshot)
    after_snapshot[leaf["snapshot_key"]] = leaf["new_json"]

    if tag_style == "internal":
        mutation_json = {"mutation": leaf["variant_tag"], leaf["payload_key"]: leaf["new_json"]}
    else:
        mutation_json = {leaf["variant"]: {leaf["payload_key"]: leaf["new_json"]}}

    diff_json = dict(base_diff_nulls)
    diff_json[leaf["diff_key"]] = leaf["new_json"]

    tests = [
        _applies(label, kind, leaf),
        _inverse(label, kind, leaf, types),
        _canonical_json(label, kind, leaf, snapshot_ty),
        _outcome(label, kind, leaf),
        _produces_diff(label, kind, leaf),
        _diff_canonical(label, kind, leaf, diff_ty),
        _diff_applies(label, kind, leaf, diff_ty),
    ]

    rust = common.render_test(
        artifact=artifact,
        types=(snapshot_ty, mutation_ty, diff_ty),
        kind=kind,
        case=case,
        header_note=[
            "`%s.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane)." % diff_ty,
            "`%s` never writes it, so it stays `None` and rides the JSON round trip as a plain" % kind,
            "`null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's",
            "committed diff, and nothing here asserts that they are.",
        ],
        tests=tests,
    )
    common.emit_case(
        artifact_dir,
        common.resolve_leaf_dir(artifact_dir, kind),
        case,
        before=base_snapshot,
        after=after_snapshot,
        mutation=mutation_json,
        diff=diff_json,
        outcome={"status": "applied"},
        rust=rust,
    )


def doc(emoji, text):
    """📝️ Wraps one doc comment at 110 columns so the emitted file reads like the hand-written ones."""
    body = textwrap.wrap(text, width=106)
    return "\n".join(["/// %s %s" % (emoji, body[0])] + ["/// " + line for line in body[1:]])


def _applies(label, kind, leaf):
    return """%s
#[semio_framework_async_macros::async_test]
async fn %s() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("%s applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "%s: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.%s, %s, "%s: %s must read %s once the change lands");
    assert_eq!(applied.%s, before().%s, "%s: %s");
}""" % (
        doc("▶️", leaf["applies_doc"] + "."),
        leaf["applies_fn"],
        kind,
        label,
        leaf["rust_field"],
        leaf["new_rust"],
        label,
        leaf["rust_field"],
        leaf["new_prose"],
        leaf["sibling_field"],
        leaf["sibling_field"],
        label,
        leaf["sibling_prose"],
    )


def _inverse(label, kind, leaf, types):
    snapshot_ty, mutation_ty, _ = types
    return """%s
#[semio_framework_async_macros::async_test]
async fn %s() {
    let base = before();
    let forward = <%s as protocol::Mutation<%s>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward %s applies");
    let inverse = <%s as protocol::Mutation<%s>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "%s: the inverse of one %s is exactly one %s back");
    for step in &inverse {
        let undo = <%s as protocol::Mutation<%s>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the %s inverse step applies");
    }
    assert_eq!(snapshot.%s, base.%s, "%s: the inverse must put the %s");
    assert_eq!(snapshot, base, "%s: replaying the inverse did not restore the whole before-snapshot");
}""" % (
        doc("↩️", leaf["inverse_doc"] + " " + leaf["old_prose"] + "."),
        leaf["inverse_fn"],
        mutation_ty, snapshot_ty,
        kind,
        mutation_ty, snapshot_ty,
        label, kind, kind,
        mutation_ty, snapshot_ty,
        kind,
        leaf["rust_field"], leaf["rust_field"],
        label, leaf["old_prose"],
        label,
    )


def _canonical_json(label, kind, leaf, snapshot_ty):
    return """%s
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: %s = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "%s: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the %s payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the %s payload reparses");
    assert_eq!(reencoded, original, "%s: the committed %s JSON is not canonical");
}""" % (
        doc("\U0001f523️", "Both committed snapshots and the committed `%s` payload are already canonical: decode → encode is a fixed point, so %s is spelled here exactly as this artifact's own serde attributes render it." % (kind, leaf["wire_key_prose"])),
        snapshot_ty,
        label,
        kind, kind,
        label, kind,
    )


def _outcome(label, kind, leaf):
    return """/// \U0001f3af️ %s
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "%s: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "%s: %s");
    assert!(produced.messages().is_empty(), "%s: an accepted %s emits no diagnostics at all");
}""" % (
        leaf["outcome_doc"],
        label,
        label, leaf["guard_prose"],
        label, kind,
    )


def _produces_diff(label, kind, leaf):
    return """/// \U0001f53a️ The sparse delta `%s` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `%s` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced %s diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "%s: the produced diff differs from the committed \U0001f53a️diff/\U0001f523️component.json");
}""" % (kind, leaf["diff_key"], kind, label)


def _diff_canonical(label, kind, leaf, diff_ty):
    lines = [
        doc("\U0001f523️", "The committed diff decodes to `%s`, re-encodes unchanged, and carries %s" % (diff_ty, leaf["diff_prose"])),
        "#[semio_framework_async_macros::async_test]",
        "async fn committed_diff_is_canonical() {",
        "    let decoded: %s = serde_json::from_str(DIFF).expect(\"the committed %s diff decodes\");" % (diff_ty, kind),
        "    assert_eq!(decoded.%s, %s, \"%s: the committed diff must carry %s = %s\");" % (leaf["rust_field"], leaf["diff_some_rust"], label, leaf["diff_key"], leaf["new_prose"]),
    ]
    for sibling in leaf["diff_none_fields"]:
        lines.append("    assert!(decoded.%s.is_none(), \"%s: %s writes %s and must leave `%s` untouched\");" % (sibling, label, kind, leaf["diff_key"], sibling))
    lines += [
        "    assert!(decoded.artifact.is_none(), \"%s: a field-scoped change must never fall back to a whole-artifact replacement\");" % label,
        "    let reencoded = serde_json::to_value(&decoded).expect(\"the committed diff re-encodes\");",
        "    let original: serde_json::Value = serde_json::from_str(DIFF).expect(\"the committed diff reparses\");",
        "    assert_eq!(reencoded, original, \"%s: the committed diff JSON is not canonical\");" % label,
        "}",
    ]
    return "\n".join(lines)


def _diff_applies(label, kind, leaf, diff_ty):
    return """/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the %s, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: %s = serde_json::from_str(DIFF).expect("the committed %s diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "%s: the committed diff did not carry before to after");
    assert_eq!(produced.%s, %s, "%s: applying the committed diff must land %s on %s");
}""" % (
        leaf["change_noun"],
        diff_ty, kind,
        label,
        leaf["rust_field"], leaf["new_rust"], label, leaf["rust_field"], leaf["new_prose"],
    )
