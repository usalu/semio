#!/usr/bin/env python3
"""🏭️ Writes every committed `📸️mutate-remodeling-1` specification vector, its mounted Rust module,
its `#[path]` mount, its feature rows and both references' registrations.

Nothing here is anyone's oracle: it authors bytes from a hand-chosen `(before, mutation)` pair and
the guard order each `🔺️diff/🦀️.rs` documents, and Rust production, the case's Python reference and
the subset's TypeScript twin then judge those bytes independently.

Usage: `python3 🐍️w2c-generate.py [--apply]` — without `--apply` it only reports what it would write.
"""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import re
import shutil
import sys
from pathlib import Path


def _load(name: str):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_").replace(".py", ""), Path(__file__).with_name(name))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


bases = _load("🐍️w2c-bases.py")
diffs = _load("🐍️w2c-diff.py")
cat = _load("🐍️w2c-vectors.py")

REPO = bases.REPO
SUBSET = bases.SUBSET
MUTATIONS = SUBSET / "🧬️schema/🧬️mutations"
CASE = SUBSET / "🧪️tests/📸️mutate-remodeling-1"
WIRING = REPO / "✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/🦀️.rs"
ORACLE = SUBSET / "🔮️oracle/🔣️.json"
APPLY = "--apply" in sys.argv
SUBSET_REL = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
MOUNT_PREFIX = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
SAFE_MAX = 190


# region 🔖️Naming
def _u16(text: str) -> int:
    return len(text.encode("utf-16-le")) // 2


def shorten(kind_dir: str, case: str) -> str:
    """✂️ The 2026-09-05 path-shortening pass's own `truncateWithHash`, reproduced so a NEW case
    directory carries the name that pass would have minted for it (verified against three of the
    names it actually produced here)."""
    dir_path = f"{SUBSET_REL}/🧬️schema/🧬️mutations/{kind_dir}/🧪️tests/{case}"
    longest = max(_u16(f"{dir_path}/{leaf}") for leaf in ("📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json", "🦠️mutation/🔣️.json", "🔺️diff/🔣️.json", "🎯️outcome/🔣️.json", "🦀️.rs"))
    if longest <= SAFE_MAX:
        return case
    target = max(20, _u16(case) - (longest - SAFE_MAX))
    digest = hashlib.sha1(dir_path.encode("utf-8")).hexdigest()[:6]
    keep = max(20, target - 1 - len(digest))
    cut = case.encode("utf-16-le")[: keep * 2].decode("utf-16-le", errors="ignore")
    dash = cut.rfind("-")
    if dash > keep * 0.5:
        cut = cut[:dash]
    return f"{cut}-{digest}"


def leading_emoji_rest(name: str) -> str:
    index = 0
    while index < len(name) and not (name[index].isascii() and (name[index].isalnum() or name[index] == "-")):
        index += 1
    return name[index:]


def module_ident(case: str) -> str:
    return "tests_" + re.sub(r"[^a-z0-9]+", "_", leading_emoji_rest(case).lower()).strip("_")
# endregion 🔖️Naming


# region 🔖️Writing
WRITES: list[tuple[Path, str]] = []


def write(path: Path, text: str) -> None:
    WRITES.append((path, text))


def dump(value) -> str:
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"
# endregion 🔖️Writing


# region 🔖️CaseModule
HEADER = '''//! 🧪️ `{kind}` fixture — `{case}`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). `{scenario}` is this vector's scenario id in
//! `../../../../🧪️tests/📸️mutate-remodeling-1/🥒️.feature`, where the same bytes are replayed against
//! this subset's independent Python reference.
//!
//! 🏞️ {note}

use crate::artifacts::remodeling::mutations::{{apply_remodeling_mutation, inverse_remodeling_mutation, RemodelingMutation}};
use crate::artifacts::remodeling::{{RemodelingDiff, RemodelingSnapshot}};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");
{diff_const}
fn before() -> RemodelingSnapshot {{
    pack::from_json_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> RemodelingSnapshot {{
    pack::from_json_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> RemodelingMutation {{
    pack::from_json_str(MUTATION).expect("mutation decodes")
}}
fn produced() -> protocol::MutationOutcome<RemodelingDiff> {{
    <RemodelingMutation as protocol::Mutation<RemodelingSnapshot>>::diff(&mutation(), &before())
}}
fn json_of<T: dsl::ToValue>(value: &T) -> pack::JsonValue {{
    pack::json_from_dsl_value(&dsl::ToValue::to_value(value))
}}
'''

CANONICAL = '''
/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point over `pack::json`, which is the codec the crate uses since serde left this type graph.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: RemodelingSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert_eq!(json_of(&decoded), original, "{label_prefix}: committed {{label}} JSON is not canonical");
    }}
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert_eq!(json_of(&mutation()), original, "{label_prefix}: committed mutation JSON is not canonical");
}}
'''

APPLIED_BODY = '''
/// ▶️ The verb reaches its committed after-document, and moved it.
#[semio_framework_async_macros::async_test]
async fn reaches_the_committed_after_document() {{
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("{kind} applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "{label_prefix}: applied state differs from committed after-snapshot");
    assert_ne!(applied, before(), "{label_prefix}: an applied vector must move the document");
}}

/// 🔺️ The sparse delta this leaf produces is EXACTLY the committed diff — the load-bearing
/// assertion, because it pins WHICH lanes the verb is allowed to touch, not merely the end state.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {{
    let outcome = produced();
    let committed = pack::parse_json(DIFF).expect("committed diff decodes");
    assert_eq!(json_of(outcome.diff()), committed, "{label_prefix}: produced diff differs from the committed 🔺️diff/🔣️.json");
    let decoded: RemodelingDiff = pack::from_json_str(DIFF).expect("committed diff decodes into the diff type");
    assert_eq!(json_of(&decoded), committed, "{label_prefix}: committed diff JSON is not canonical");
}}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a COMPLETE
/// description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_carries_before_to_after() {{
    let decoded: RemodelingDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "{label_prefix}: committed diff did not carry before to after");
}}

/// 🎯️ The declared outcome — its status and every diagnostic it names — is what this leaf emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {{
    let declared = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(|status| status.as_str()), Some("applied"), "{label_prefix} declares an applied outcome");
    let produced = produced();
    let declared_codes: Vec<String> = match declared.get("messages") {{
        Some(pack::JsonValue::Array(entries)) => entries.iter().filter_map(|entry| entry.get("code").and_then(|code| code.as_str()).map(str::to_string)).collect(),
        _ => Vec::new(),
    }};
    let emitted: Vec<String> = produced.messages().iter().map(|message| message.code.0.clone()).collect();
    assert_eq!(emitted, declared_codes, "{label_prefix}: emitted diagnostics differ from the declared ones");
}}
'''

INVERSE_BODY = '''
/// ↩️ Applying the verb and then EVERY step of its own computed inverse restores the committed
/// before-document — member positions included, which a delete undone by re-appending would fail.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_before_document() {{
    let base = before();
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse_remodeling_mutation(&base, &mutation()) {{
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{label_prefix}: inverse did not restore the before-snapshot");
}}
'''

REJECTED_BODY = '''
/// 🚫️ A refused `{kind}` leaves the document byte-identical to its committed after-document, which
/// for a refusal IS the before-document.
#[semio_framework_async_macros::async_test]
async fn refusal_leaves_the_document_untouched() {{
    let base = before();
    let applied = apply_remodeling_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "{label_prefix}: applied state differs from committed after-snapshot");
    assert_eq!(applied, base, "{label_prefix}: a refused mutation must not move the document");
}}

/// 🎯️ The declared refusal — status, code, level and diagnostic target — is exactly what this
/// leaf's own guard emits, and the diff it carries is empty rather than half-built.
#[semio_framework_async_macros::async_test]
async fn declared_refusal_holds() {{
    let declared = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(|status| status.as_str()), Some("rejected"), "{label_prefix} declares a rejected outcome");
    let produced = produced();
    assert_eq!(produced.diff(), &RemodelingDiff::default(), "{label_prefix}: a refusing leaf must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "{label_prefix}: exactly one diagnostic is expected, got {{messages:?}}");
    assert_eq!(messages[0].code.0, "{code}", "{label_prefix}: the declared code must be the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::{severity}, "{label_prefix}: the declared level must be the emitted one");
    assert_eq!(declared.get("code").and_then(|code| code.as_str()), Some(messages[0].code.0.as_str()), "the committed outcome must name the emitted code");
    let declared_path: Vec<String> = match declared.get("path") {{
        Some(pack::JsonValue::Array(entries)) => entries.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    }};
    assert_eq!(declared_path, messages[0].target, "{label_prefix}: the declared path must be the emitted target");
}}

/// 🚫️ A refusal ships no `🔺️diff/🔣️.json` at all — the `🚫️.absent` marker beside it is the
/// repository's own statement that there is no delta to commit, not a forgotten file.
#[semio_framework_async_macros::async_test]
async fn no_diff_is_committed() {{
    assert!(include_str!("🔺️diff/🚫️.absent").is_empty(), "{label_prefix}: the absent-diff marker must stay empty");
}}
'''

WARNED_BODY = '''
/// 🔁️ A warned no-op leaves the document byte-identical to its committed after-document, which for
/// a no-op IS the before-document — and, unlike a refusal, it is still an APPLIED outcome.
#[semio_framework_async_macros::async_test]
async fn no_op_leaves_the_document_untouched() {{
    let base = before();
    let applied = apply_remodeling_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "{label_prefix}: applied state differs from committed after-snapshot");
    assert_eq!(applied, base, "{label_prefix}: a no-op must not move the document");
}}

/// 🎯️ The declared Warning — not an Error — is what this leaf emits, over an empty diff that is
/// still committed as a real all-null delta rather than as an absent one.
#[semio_framework_async_macros::async_test]
async fn declared_warning_holds() {{
    let declared = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(|status| status.as_str()), Some("applied"), "{label_prefix} declares an applied outcome");
    let produced = produced();
    assert_eq!(produced.diff(), &RemodelingDiff::default(), "{label_prefix}: a no-op leaf must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "{label_prefix}: exactly one diagnostic is expected, got {{messages:?}}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "{label_prefix}: an identical resubmission is a no-op");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "{label_prefix}: a no-op is a Warning, never an Error");
    let committed = pack::parse_json(DIFF).expect("committed diff decodes");
    assert_eq!(json_of(produced.diff()), committed, "{label_prefix}: produced diff differs from the committed 🔺️diff/🔣️.json");
}}
'''

SEVERITY_IDENT = {"error": "Error", "fatal": "Fatal", "warning": "Warning", "info": "Info"}


def case_module(kind: str, case: str, scenario: str, note: str, outcome_kind: str, code: str | None, severity: str | None, has_inverse: bool) -> str:
    diff_const = '' if outcome_kind == "rejected" else 'const DIFF: &str = include_str!("🔺️diff/🔣️.json");\n'
    label = f"{kind}/{leading_emoji_rest(case)}"
    text = HEADER.format(kind=kind, case=case, scenario=scenario, note=note, diff_const=diff_const)
    body = {"applied": APPLIED_BODY, "rejected": REJECTED_BODY, "warned": WARNED_BODY}[outcome_kind]
    text += body.format(kind=kind, label_prefix=label, code=code or "", severity=SEVERITY_IDENT.get(severity or "error", "Error"))
    assert has_inverse, f"{kind}/{case}: every committed vector must carry a restoring inverse"
    text += INVERSE_BODY.format(kind=kind, label_prefix=label)
    text += CANONICAL.format(label_prefix=label)
    return text
# endregion 🔖️CaseModule


# region 🔖️Vectors
class Built:
    __slots__ = ("kind", "role", "scenario", "case_dir", "kind_dir", "note", "before", "payload", "after", "diff", "outcome", "outcome_kind", "code", "severity", "has_inverse")


def build() -> list[Built]:
    built: list[Built] = []
    for kind, case_dir in cat.TOY_CASES.items():
        kind_dir = cat.KIND_DIRECTORY[kind]
        override = cat.TOY_OVERRIDES.get(kind)
        if override is None:
            payload_path = MUTATIONS / kind_dir / "🧪️tests" / case_dir / "🦠️mutation/🔣️.json"
            wire = json.loads(payload_path.read_text(encoding="utf-8"))
            payload = {key: value for key, value in wire.items() if key != "mutation"}
            note = "the pre-existing two-stream unit vector, regenerated onto the corrected toy base"
        else:
            case_dir, payload = shorten(kind_dir, override[0]), copy.deepcopy(override[1])
            note = "the pre-existing two-stream unit vector, re-aimed at the one member of the same scene this verb's referential guard accepts"
        built.append(_finish(kind, "toy", kind, case_dir, kind_dir, note, cat.TOY, payload))
    for row in cat.catalogue():
        kind_dir = cat.KIND_DIRECTORY[row.kind]
        case_dir = shorten(kind_dir, row.case)
        built.append(_finish(row.kind, row.role, row.scenario, case_dir, kind_dir, row.note, cat.BASES[row.base], row.payload))
    stems: dict[tuple[str, str], str] = {}
    for entry in built:
        key = (entry.kind_dir, entry.case_dir.rsplit("-", 1)[0])
        if key in stems:
            raise SystemExit(f"[build] {entry.kind_dir}: {entry.case_dir} and {stems[key]} truncate to the same readable stem")
        stems[key] = entry.case_dir
    scenarios = [entry.scenario for entry in built]
    assert len(set(scenarios)) == len(scenarios), "scenario ids must be unique"
    return built


def _finish(kind, role, scenario, case_dir, kind_dir, note, base, payload) -> Built:
    notes, diff = diffs.diff_of(kind, base, payload)
    refusal = diffs.refused(notes)
    entry = Built()
    entry.kind, entry.role, entry.scenario, entry.case_dir, entry.kind_dir, entry.note = kind, role, scenario, case_dir, kind_dir, note
    entry.before, entry.payload = base, payload
    entry.code = refusal.code if refusal else None
    entry.severity = refusal.level if refusal else None
    if refusal:
        entry.after, entry.diff, entry.outcome_kind = base, None, "rejected"
        entry.outcome = {"status": "rejected", "code": refusal.code, "path": refusal.target}
        entry.has_inverse = diffs.inverse_restores(kind, base, payload, base)
        assert entry.has_inverse, f"{scenario}: a refused vector's inverse must leave the document untouched"
    else:
        entry.after = diffs.apply_diff(diff, base)
        entry.diff = diff
        warn = next((note_ for note_ in notes if note_.level == "warning"), None)
        entry.outcome_kind = "warned" if warn else "applied"
        entry.outcome = {"status": "applied"}
        if notes:
            entry.outcome["messages"] = [{"level": note_.level, "code": note_.code} for note_ in notes]
        entry.has_inverse = diffs.inverse_restores(kind, base, payload, entry.after)
        assert entry.has_inverse, f"{scenario}: inverse(m, before) applied to after does not restore before"
        if warn:
            entry.code = "mutation.no-op"
            entry.severity = "warning"
            assert entry.after == base, f"{scenario}: a no-op must not move the document"
        else:
            assert entry.after != base, f"{scenario}: an applied vector must move the document"
    return entry
# endregion 🔖️Vectors


# region 🔖️Emit
def _assert_printer_agnostic(label: str, node) -> None:
    """🎚️ Every number a committed vector carries that an `f32` COULD hold must print to the same
    shortest lexeme at either width, so no committed byte depends on F1's `*self as f64` widening."""
    import struct as _struct

    for number in bases._numbers(node):
        if _struct.unpack("<f", _struct.pack("<f", number))[0] != number:
            continue
        assert bases.f32_lexeme_agrees(number), f"{label}: {number!r} prints differently as an f32 than as an f64"


def emit(entries: list[Built]) -> None:
    for entry in entries:
        for lane, document in (("before", entry.before), ("after", entry.after), ("mutation", entry.payload), ("diff", entry.diff)):
            _assert_printer_agnostic(f"{entry.scenario}/{lane}", document)
        root = MUTATIONS / entry.kind_dir / "🧪️tests" / entry.case_dir
        write(root / "📸️snapshot/⬅️before/🔣️.json", dump(entry.before))
        write(root / "📸️snapshot/➡️after/🔣️.json", dump(entry.after))
        write(root / "🦠️mutation/🔣️.json", dump({"mutation": diffs.TAGS[entry.kind]} | entry.payload))
        write(root / "🎯️outcome/🔣️.json", dump(entry.outcome))
        if entry.diff is None:
            write(root / "🔺️diff/🚫️.absent", "")
        else:
            write(root / "🔺️diff/🔣️.json", dump(entry.diff))
        write(root / "🦀️.rs", case_module(entry.kind, entry.case_dir, entry.scenario, entry.note, entry.outcome_kind, entry.code, entry.severity, entry.has_inverse))
# endregion 🔖️Emit


# region 🔖️Mounts
def mounts(entries: list[Built]) -> str:
    text = WIRING.read_text(encoding="utf-8")
    keep = {(entry.kind_dir, entry.case_dir) for entry in entries}
    for match in reversed(list(re.finditer(r' *#\[cfg\(test\)\]\n *#\[path = "' + re.escape(MOUNT_PREFIX) + r'/([^/"]+)/🧪️tests/([^/"]+)/🦀️\.rs"\]\n *mod tests_\w+;\n', text))):
        if (match.group(1), match.group(2)) not in keep:
            text = text[: match.start()] + text[match.end() :]
    by_kind: dict[str, list[Built]] = {}
    for entry in entries:
        by_kind.setdefault(entry.kind_dir, []).append(entry)
    for kind_dir, rows in by_kind.items():
        anchor = None
        for match in re.finditer(r'( +)#\[cfg\(test\)\]\n +#\[path = "' + re.escape(f"{MOUNT_PREFIX}/{kind_dir}/🧪️tests/") + r'[^"]+"\]\n +mod tests_\w+;\n', text):
            anchor = match
        if anchor is None:
            component = re.search(r'#\[path = "' + re.escape(f"{MOUNT_PREFIX}/{kind_dir}/🦀️.rs") + r'"\]\n', text)
            if component is None:
                raise SystemExit(f"[mounts] no module block for {kind_dir}")
            anchor = re.compile(r"( +)pub use component::\*;\n").search(text, component.end())
        indent = anchor.group(1)
        existing = set(re.findall(r'#\[path = "' + re.escape(f"{MOUNT_PREFIX}/{kind_dir}/🧪️tests/") + r'([^"]+)/🦀️\.rs"\]', text))
        block = ""
        for row in sorted(rows, key=lambda entry: entry.scenario):
            if row.case_dir in existing:
                continue
            block += f'{indent}#[cfg(test)]\n{indent}#[path = "{MOUNT_PREFIX}/{kind_dir}/🧪️tests/{row.case_dir}/🦀️.rs"]\n{indent}mod {module_ident(row.case_dir)};\n'
        if block:
            text = text[: anchor.end()] + block + text[anchor.end() :]
    return text
# endregion 🔖️Mounts


# region 🔖️Feature
def feature(entries: list[Built]) -> str:
    source = (CASE / "🥒️.feature").read_text(encoding="utf-8")
    applied = [entry for entry in entries if entry.outcome_kind == "applied"]
    warned = [entry for entry in entries if entry.outcome_kind == "warned"]
    rejected = [entry for entry in entries if entry.outcome_kind == "rejected"]

    def table(rows, columns, cells):
        widths = [max(len(column), max((len(cells(row)[index]) for row in rows), default=0)) for index, column in enumerate(columns)]
        lines = ["      | " + " | ".join(column.ljust(widths[index]) for index, column in enumerate(columns)) + " |"]
        for row in rows:
            values = cells(row)
            lines.append("      | " + " | ".join(values[index].ljust(widths[index]) for index in range(len(columns))) + " |")
        return "\n".join(lines)

    def vector_of(entry: Built) -> str:
        return f"{entry.kind_dir}/🧪️tests/{entry.case_dir}"

    applied_rows = table(sorted(applied, key=lambda entry: entry.scenario), ["id", "kind", "vector"], lambda entry: [entry.scenario, entry.kind, vector_of(entry)])
    inverse_rows = table(sorted([entry for entry in applied if entry.has_inverse], key=lambda entry: entry.scenario), ["id", "kind", "vector"], lambda entry: [entry.scenario, entry.kind, vector_of(entry)])
    unmoved_inverse_rows = table(
        sorted([entry for entry in entries if entry.outcome_kind != "applied" and entry.has_inverse], key=lambda entry: entry.scenario),
        ["id", "kind", "vector", "code"],
        lambda entry: [entry.scenario, entry.kind, vector_of(entry), entry.code],
    )
    rejected_rows = table(sorted(rejected, key=lambda entry: entry.scenario), ["id", "kind", "vector", "code"], lambda entry: [entry.scenario, entry.kind, vector_of(entry), entry.code])
    warned_rows = table(sorted(warned, key=lambda entry: entry.scenario), ["id", "kind", "vector", "code"], lambda entry: [entry.scenario, entry.kind, vector_of(entry), entry.code])

    prose_end = source.index("  @id-mutate")
    prose = source[:prose_end]
    prose = prose.replace(
        "  📄️ `commit-reconstruction` is the one kind with NO committed leaf vector",
        "  📊️ Every kind carries four ROLES as far as its own semantics reach: the `toy` vector on the\n"
        "  two-stream unit scene these fixtures were first authored against, a `realworld` vector on the\n"
        "  ten-frame two-camera orbit survey (a durable-artifact store, a four-point GCP network observed\n"
        "  across both streams, a 104-point sparse cloud, a dense cloud with confidence and classification\n"
        "  lanes, a ten-pose trajectory, motion tracks, geo products and a QC report), a `refusal` vector\n"
        "  per distinct guard the kind's own `🔺️diff/🦀️.rs` raises, and an `edge` vector for first/last/\n"
        "  empty position — or, for a kind whose ONLY non-applying path is an identical resubmission, for\n"
        "  the `mutation.no-op` Warning it raises instead. A refusal ships no `🔺️diff/🔣️.json`: the\n"
        "  `🚫️.absent` marker beside it is this repository's own way of committing that there is no delta.\n"
        "\n"
        "  📄️ `commit-reconstruction` additionally keeps an owner-shared vector",
    )
    prose = prose.replace(
        "case's own fixtures — local://⬅️commit-reconstruction-before.json is a byte copy of",
        "owner's shared fixtures — shared://🏁️commit-reconstruction/⬅️before.json is a byte copy of",
    )
    prose = prose.replace(
        "  local://🦠️commit-reconstruction-mutation.json pairs that leaf's committed `job` payload with",
        "  shared://🏁️commit-reconstruction/🦠️mutation.json pairs that leaf's committed `job` payload with",
    )
    prose = prose.replace(
        "staging handle), and local://➡️commit-reconstruction-after.json is the before-document unchanged,",
        "staging handle), and shared://🏁️commit-reconstruction/➡️after.json is the before-document unchanged,",
    )
    prose = prose.replace(
        "  address that vector by the same doc-string mechanism as every other row — three `local://` URIs in",
        "  address that vector by the same doc-string mechanism as every other row — three `shared://` URIs in",
    )
    prose = prose.replace(
        "This kind's two scenarios",
        "Its OTHER three guards — invalid-reconstruction-asset, invalid-reconstruction-mesh and the sparse\n"
        "  guard again on the real-world survey — need no staging state at all and ship as ordinary leaf\n"
        "  vectors under `🧬️schema/🧬️mutations/🏁commit-reconstruction/🧪️tests/`. This vector's own two scenarios",
    )

    body = f'''  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> reaches its committed after-document
    Given the committed specification vector for the <id> kind
      """
      {{
        "kind": "<kind>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }}
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the resulting document is the committed after-document, the mutation moved it, and the two implementations agree
    Examples:
{applied_rows}

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused exactly as its vector declares
    Given the committed refusal vector for the <id> kind
      """
      {{
        "kind": "<kind>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json",
        "code": "<code>"
      }}
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
{rejected_rows}

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is warned as a no-op and moves nothing
    Given the committed no-op vector for the <id> kind
      """
      {{
        "kind": "<kind>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json",
        "code": "<code>"
      }}
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
{warned_rows}

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused by the case-local staging vector it declares
    Given the case-local refusal vector for the <id> kind
      """
      {{
        "kind": "<kind>",
        "before": "shared://🏁️commit-reconstruction/⬅️before.json",
        "mutation": "shared://🏁️commit-reconstruction/🦠️mutation.json",
        "after": "shared://🏁️commit-reconstruction/➡️after.json",
        "code": "<code>"
      }}
      """
    When <kind> is applied through apply_remodeling_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id                    | kind                  | code                                   |
      | commit-reconstruction | commit-reconstruction | mutation.invalid-reconstruction-sparse |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed specification vector for the <id> kind
      """
      {{
        "kind": "<kind>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }}
      """
    When <kind> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
{inverse_rows}

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document its own refusal never moved
    Given the committed refusal vector for the <id> kind
      """
      {{
        "kind": "<kind>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json",
        "code": "<code>"
      }}
      """
    When <kind> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
{unmoved_inverse_rows}

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document its case-local refusal never moved
    Given the case-local refusal vector for the <id> kind
      """
      {{
        "kind": "<kind>",
        "before": "shared://🏁️commit-reconstruction/⬅️before.json",
        "mutation": "shared://🏁️commit-reconstruction/🦠️mutation.json",
        "after": "shared://🏁️commit-reconstruction/➡️after.json",
        "code": "<code>"
      }}
      """
    When <kind> and then every step of its own computed inverse are applied through undo_remodeling_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
      | id                    | kind                  | code                                   |
      | commit-reconstruction | commit-reconstruction | mutation.invalid-reconstruction-sparse |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example this artifact ships
      """
      {{
        "kind": "identity-round-trip",
        "carrier": "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"
      }}
      """
    When it is parsed, printed back to DSL and parsed again through round_trip_remodeling_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
'''
    return prose + body
# endregion 🔖️Feature


# region 🔖️Registrations
def rust_registration(entries: list[Built]) -> str:
    source = (CASE / "🦀️.rs").read_text(encoding="utf-8")
    mutate = sorted({entry.scenario for entry in entries} | {"commit-reconstruction"})
    inverse = sorted({entry.scenario for entry in entries if entry.has_inverse} | {"commit-reconstruction"})
    kinds = sorted(cat.KIND_DIRECTORY)
    def block(name, rows, doc):
        lines = "\n".join(f'    "{row}",' for row in rows)
        return f'/// {doc}\n#[cfg(feature = "sut")]\nconst {name}: &[&str] = &[\n{lines}\n];\n'
    region = (
        "//#region 🔖️Scenarios\n"
        + block("KINDS", kinds, "🏷️ Mirrors `KINDS` in `../../🧬️schema/🧬️mutations/🦀️.rs` — duplicated, not imported, because\n/// the host may not reach into the subject crate outside the `sut` feature. The contract's\n/// mutation-coverage gate keeps this list honest against the catalog.")
        + "\n"
        + block("MUTATE_SCENARIOS", mutate, "▶️ Every `@id-mutate` row `🥒️.feature` plans, applied and refused alike — generated with the\n/// feature itself, so a row that gains or loses a vector cannot leave this list behind.")
        + "\n"
        + block("INVERSE_SCENARIOS", inverse, "↩️ Every `@id-inverse` row — one per committed vector. A refused or warned vector's inverse is\n/// empty, which still restores: its forward step moved nothing.")
        + "//#endregion 🔖️Scenarios\n"
    )
    marker = "🔖️Kinds" if "//#region 🔖️Kinds" in source else "🔖️Scenarios"
    start = source.index(f"//#region {marker}")
    end = source.index(f"//#endregion {marker}") + len(f"//#endregion {marker}\n")
    source = source[:start] + region + source[end:]
    registration_start = source.index('    #[cfg(feature = "sut")]\n    for ')
    registration_end = source.index('    #[cfg(feature = "sut")]\n    {\n        built = built.subject("identity-round-trip", subject::round_trip);')
    source = (
        source[:registration_start]
        + '    #[cfg(feature = "sut")]\n'
        + "    for scenario in MUTATE_SCENARIOS {\n        built = built.subject(&format!(\"mutate-{scenario}\"), subject::mutate);\n    }\n"
        + '    #[cfg(feature = "sut")]\n'
        + "    for scenario in INVERSE_SCENARIOS {\n        built = built.subject(&format!(\"inverse-{scenario}\"), subject::inverse);\n    }\n"
        + source[registration_end:]
    )
    return source


def python_registration(entries: list[Built]) -> str:
    """🐍️ The reference's scenario lists, generated with the feature itself so a row that gains or
    loses a vector cannot leave the reference's registration behind."""
    source = (CASE / "🐍️.py").read_text(encoding="utf-8")
    mutate = sorted({entry.scenario for entry in entries} | {"commit-reconstruction"})
    inverse = sorted({entry.scenario for entry in entries if entry.has_inverse} | {"commit-reconstruction"})

    def block(name: str, rows: list[str], doc: str) -> str:
        lines = "\n".join(f'    "{row}",' for row in rows)
        return f"# {doc}\n{name}: list[str] = [\n{lines}\n]\n"

    region = (
        "# region 🔖️Scenarios\n"
        + block("MUTATE_SCENARIOS", mutate, "▶️ Every `@id-mutate` row the feature plans, applied, refused and warned alike.")
        + "\n"
        + block(
            "INVERSE_SCENARIOS",
            inverse,
            "↩️ Every `@id-inverse` row — one per committed vector, with no exceptions: since the\n"
            "# canonical-order invariant and the referential guards landed, every kind's inverse restores its\n"
            "# before-document exactly, and the generator refuses to commit a vector for which it does not.",
        )
        + "# endregion 🔖️Scenarios\n"
    )
    start = source.index("# region 🔖️Scenarios")
    end = source.index("# endregion 🔖️Scenarios") + len("# endregion 🔖️Scenarios\n")
    return source[:start] + region + source[end:]

SHARED_COMMIT = SUBSET / "🧫️fixtures/🏁️commit-reconstruction"


def shared_commit_fixture(entries: list[Built]) -> dict[str, str]:
    """🏁️ The owner-shared `commit-reconstruction` vector, the one kind whose diff reads process-global
    staging state a `(before, mutation, after)` triple cannot carry. It is committed as the REFUSAL its
    own guard raises, so `after` is `before` unchanged: the payload's `sparse` argument is a plain point
    buffer rather than a `remodeling-content:` staging handle, which names no staged run to publish.
    Both documents are the regenerated `replace-job` before-snapshot, so the vector shares this subset's
    own committed scene rather than a hand-typed one."""
    source = next(entry for entry in entries if entry.kind == "replace-job" and entry.role == "toy")
    job_payload = next(entry for entry in entries if entry.kind == "replace-job" and entry.role == "toy").payload["job"]
    sparse = next(entry for entry in entries if entry.kind == "replace-sparse" and entry.role == "toy").payload["sparse"]
    mutation = {"mutation": "commitReconstruction", "job": job_payload, "sparse": sparse, "trajectory": None, "mesh": None, "geo": None, "qc": None, "assets": []}
    return {"⬅️before.json": dump(source.before), "🦠️mutation.json": dump(mutation), "➡️after.json": dump(source.before)}


def oracle_registry(entries: list[Built], shared: dict[str, str]) -> str:
    document = json.loads(ORACLE.read_text(encoding="utf-8"))
    manifest = document["fixtureManifests"][0]
    manifest["id"] = "commit-reconstruction-refused"
    manifest["outcome"] = "rejected"
    manifest["files"] = [
        {
            "role": role,
            "path": f"../🧫️fixtures/🏁️commit-reconstruction/{name}",
            "mediaType": "application/json",
            "sha256": "sha256:" + hashlib.sha256(shared[name].encode("utf-8")).hexdigest(),
            "bytes": len(shared[name].encode("utf-8")),
        }
        for role, name in (("expected-before", "⬅️before.json"), ("mutation-payload", "🦠️mutation.json"), ("expected-after", "➡️after.json"))
    ]
    manifest["notes"] = (
        "CommitReconstruction{job,sparse,trajectory,mesh,geo,qc,assets} publishes what a staged reconstruction produced, so every result argument it "
        "accepts must be a replayable `remodeling-content:` staging handle. This vector's `sparse` argument is a plain point buffer, which names no staged "
        "run and is therefore refused with mutation.invalid-reconstruction-sparse; a refused commit leaves the scene untouched, so ⬅️before.json and "
        "➡️after.json are the same document. Both are this subset's own committed replace-job before-snapshot, and the payload pairs that leaf's committed "
        "job with ⭐replace-sparse's committed sparse buffer — no byte of this vector is hand-typed. The kind's three OTHER guards "
        "(invalid-reconstruction-asset, invalid-reconstruction-mesh and the sparse guard again on the real-world survey) ship as ordinary leaf vectors "
        "under 🧬️schema/🧬️mutations/🏁commit-reconstruction/🧪️tests/."
    )
    by_kind: dict[str, list[Built]] = {}
    for entry in entries:
        by_kind.setdefault(entry.kind, []).append(entry)
    for catalog in document["mutationCatalogs"]:
        vectors = []
        for kind in sorted(by_kind):
            rows = sorted(by_kind[kind], key=lambda entry: entry.scenario)
            vectors.append({
                "mutationId": kind,
                "sourceMutationDirectoryName": cat.KIND_DIRECTORY[kind],
                "mutationDirectoryName": cat.KIND_DIRECTORY[kind],
                "scenarios": [{"id": leading_emoji_rest(row.case_dir), "directoryName": row.case_dir} for row in rows],
            })
        catalog["vectors"] = vectors
    evidence = document["oracles"][0]["nativeSecondImplementation"]
    evidence["fixtureCoverage"]["vectors"] = len(entries)
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"
# endregion 🔖️Registrations


def main() -> int:
    entries = build()
    emit(entries)
    write(WIRING, mounts(entries))
    write(CASE / "🥒️.feature", feature(entries))
    write(CASE / "🦀️.rs", rust_registration(entries))
    write(CASE / "🐍️.py", python_registration(entries))
    shared = shared_commit_fixture(entries)
    for name, text in shared.items():
        write(SHARED_COMMIT / name, text)
    write(ORACLE, oracle_registry(entries, shared))
    roles: dict[str, int] = {}
    outcomes: dict[str, int] = {}
    for entry in entries:
        roles[entry.role] = roles.get(entry.role, 0) + 1
        outcomes[entry.outcome_kind] = outcomes.get(entry.outcome_kind, 0) + 1
    print(f"[generate] {len(entries)} vectors — roles {roles}, outcomes {outcomes}")
    print(f"[generate] {sum(1 for entry in entries if entry.has_inverse)} inverse scenarios")
    print(f"[generate] {len(WRITES)} files; apply={APPLY}")
    if APPLY:
        for path, text in WRITES:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, encoding="utf-8")
        for entry in entries:
            root = MUTATIONS / entry.kind_dir / "🧪️tests" / entry.case_dir
            stale = root / ("🔺️diff/🔣️.json" if entry.diff is None else "🔺️diff/🚫️.absent")
            if stale.exists():
                stale.unlink()
        for stale_local in sorted((CASE / "🧫️fixtures").glob("*commit-reconstruction*.json")):
            stale_local.unlink()
            print(f"[generate] removed duplicated case-local fixture {stale_local.name}")
        keep = {(entry.kind_dir, entry.case_dir) for entry in entries}
        for kind_dir in sorted({entry.kind_dir for entry in entries}):
            tests = MUTATIONS / kind_dir / "🧪️tests"
            for case in sorted(tests.iterdir()):
                if case.is_dir() and (kind_dir, case.name) not in keep:
                    shutil.rmtree(case)
                    print(f"[generate] removed stale case {kind_dir}/{case.name}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
