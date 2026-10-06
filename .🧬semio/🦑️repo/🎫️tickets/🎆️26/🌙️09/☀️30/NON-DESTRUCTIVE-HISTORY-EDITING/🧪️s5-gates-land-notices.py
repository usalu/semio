"""🏛️ Lands the staged framework notice package (`🗑️generated/s4-gates/stage-r45/`, `📓️s4-gates-report.md` M6 and § S5.2).

Re-derived against the tree of 2026-10-05: every anchor is a literal of the live sources and its occurrence count is asserted,
so a drifted file refuses the whole run (exit 2, nothing written). `--check` is the default and prints the plan.

    python3 🧪️s5-gates-land-notices.py --root <repository or scratch root> --rows <framework-notice-rows.json> [--apply]

Parts, all or nothing:
  1. kernel `FRAMEWORK_FAULT_NOTICE_LABELS` (Rust): the staged rows after `window-transient.kind-unknown`, array length raised;
  2. its TypeScript twin: the same rows as `{ code, en, de }` objects;
  3. the fixture `🧫️fixtures/🧫️framework-notices/🔣️.json`: the same rows;
  4. the fixture schema's code pattern: the staged pattern;
  5. the plugin SDK: 21 anonymous flow refusals (`plugin_sdk_fault(…)`) named with 17 of those codes, origin `Plugin` kept.
"""
import json
import os
import sys

KERNEL = "🧰️framework/🔨️modules/🎠️kernel"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
RUST = f"{KERNEL}/🦀️.rs"
TWIN = f"{KERNEL}/🟦️.ts"
FIXTURE = f"{KERNEL}/🧫️fixtures/🧫️framework-notices/🔣️.json"
SCHEMA = f"{KERNEL}/🧬️schema/🔣️framework-notices/🔣️.json"
LAST = "window-transient.kind-unknown"
OLD_PATTERN = '"pattern": "^(app|mutation|plugin|history-filter|window-transient)(\\\\.[a-z][a-z0-9]*(-[a-z0-9]+)*)+$"'


def named(code, message):
    """🏷️ The coded fault expression that replaces `plugin_sdk_fault(message)`."""
    return f'Fault::new(FaultOrigin::Plugin, FaultCode::new("{code}"), {message})'


CLOSE = "retirement.close_step(maximum_items.max(1), maximum_bytes).map_err(|error| {})?"
MEMBER = '"a composed member\'s history-edit owners belong to another store kind"'
RUN_MEMBER = '"a member tool run\'s owners belong to another store kind"'
DIALECT = '"composition history requires the parent\'s exact declared dialect"'
MOVED = '"group history moved a child without exact immutable-root authority"'
TAIL = 'return Err({}format!(\n                    "transaction_{{action}}: no member of this instance carries group'
NAMINGS = [
    (f"{SDK}/⏪️time-travel/🦀️.rs", CLOSE.format("plugin_sdk_fault(error.into_message())"), CLOSE.format(named("timeTravel.snapshot-retirement", "error.into_message()")), 1),
    (f"{SDK}/⏪️time-travel/🦀️.rs", f"plugin_sdk_fault({MEMBER})", named("timeTravel.member-store-kind", MEMBER), 2),
    (f"{SDK}/⏪️time-travel/🦀️.rs", 'plugin_sdk_fault("a history-edit preview answered another command")', named("timeTravel.preview-mismatch", '"a history-edit preview answered another command"'), 1),
    (f"{SDK}/⏯️tool-run/🦀️.rs", 'plugin_sdk_fault("tool run publication handoff lost its exact retirement owner")', named("toolRun.publication-handoff", '"tool run publication handoff lost its exact retirement owner"'), 1),
    (f"{SDK}/⏯️tool-run/🦀️.rs", CLOSE.format("plugin_sdk_fault(error.into_message())"), CLOSE.format(named("toolRun.snapshot-retirement", "error.into_message()")), 1),
    (f"{SDK}/⏯️tool-run/🦀️.rs", f"plugin_sdk_fault({RUN_MEMBER})", named("toolRun.member-store-kind", RUN_MEMBER), 2),
    (f"{SDK}/⏯️tool-run/🦀️.rs", 'plugin_sdk_fault("a member tool run lost its base")', named("toolRun.member-base-lost", '"a member tool run lost its base"'), 1),
    (f"{SDK}/⏯️tool-run/🦀️.rs", "inject_tool_run_trace_lane_into(surface, &lane).map_err(|error| plugin_sdk_fault(error.to_string()))", "inject_tool_run_trace_lane_into(surface, &lane).map_err(|error| " + named("toolRun.trace-lane", "error.to_string()") + ")", 1),
    (f"{SDK}/⏯️tool-run/🦀️.rs", "publication.close_step(TOOL_RUN_PUBLICATION_GRANT).map_err(|error| plugin_sdk_fault(error.into_message()))?", "publication.close_step(TOOL_RUN_PUBLICATION_GRANT).map_err(|error| " + named("toolRun.publication-retirement", "error.into_message()") + ")?", 1),
    (f"{SDK}/🦀️.rs", f"plugin_sdk_fault({DIALECT})", named("transaction.group-history-dialect", DIALECT), 2),
    (f"{SDK}/🦀️.rs", f"plugin_sdk_fault({MOVED})", named("transaction.group-history-root", MOVED), 2),
    (f"{SDK}/🦀️.rs", TAIL.format("plugin_sdk_fault("), TAIL.format('Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-tail"), '), 1),
    (f"{SDK}/🦀️.rs", 'plugin_sdk_fault(format!("transaction_rollback: no pending transaction matches txn_id {txn_id:?}"))', named("transaction.rollback-unknown", 'format!("transaction_rollback: no pending transaction matches txn_id {txn_id:?}")'), 1),
    (f"{SDK}/🦀️.rs", 'plugin_sdk_fault(format!("transaction_undo: this instance\'s tail edit does not belong to group {group_id:?}"))', named("transaction.undo-foreign-tail", 'format!("transaction_undo: this instance\'s tail edit does not belong to group {group_id:?}")'), 1),
    (f"{SDK}/🦀️.rs", 'self.store.undo().await.map_err(|error| plugin_sdk_fault(format!("{error:?}")))?', "self.store.undo().await.map_err(|error| " + named("transaction.undo-failed", 'format!("{error:?}")') + ")?", 1),
    (f"{SDK}/🦀️.rs", 'self.store.redo().await.map_err(|error| plugin_sdk_fault(format!("{error:?}")))?', "self.store.redo().await.map_err(|error| " + named("transaction.redo-failed", 'format!("{error:?}")') + ")?", 1),
    (f"{SDK}/🦀️.rs", 'plugin_sdk_fault(format!("transaction_redo: this instance\'s redo-tail edit does not belong to group {group_id:?}"))', named("transaction.redo-foreign-tail", 'format!("transaction_redo: this instance\'s redo-tail edit does not belong to group {group_id:?}")'), 1),
]


def refuse(message):
    """🛑️ Fails closed: prints why and exits 2 before anything is written."""
    print(f"[land-notices] REFUSED: {message}", file=sys.stderr)
    sys.exit(2)


def quoted(text):
    """🔤️ A double-quoted literal valid in Rust, TypeScript and JSON; refuses a text that would need a language-specific escape."""
    if "\\" in text or '"' in text or "\n" in text:
        refuse(f"notice text needs an escape: {text!r}")
    return f'"{text}"'


def swap(texts, path, old, new, expected):
    """✍️ Replaces the `expected` occurrences of `old` in the planned text of `path`."""
    found = texts[path].count(old)
    if found != expected:
        refuse(f"{path} holds {found} occurrence(s) of {old[:90]!r}, expected {expected}")
    texts[path] = texts[path].replace(old, new)


def main():
    arguments = sys.argv[1:]
    value = lambda flag: arguments[arguments.index(flag) + 1] if flag in arguments and arguments.index(flag) + 1 < len(arguments) else ""
    root, rows_path, apply = value("--root").rstrip("/"), value("--rows"), "--apply" in arguments
    if root == "" or not os.path.isdir(os.path.join(root, KERNEL)) or not os.path.isdir(os.path.join(root, SDK)):
        refuse("--root must name a directory that holds the kernel and the plugin SDK")
    if rows_path == "" or not os.path.isfile(rows_path):
        refuse("--rows must name the staged framework-notice-rows.json")
    staged = json.load(open(rows_path, encoding="utf-8"))
    rows = [(row["code"], row["en"], row["de"]) for row in staged["notices"]]
    if not rows or len({code for code, _, _ in rows}) != len(rows) or any(en == de or not en or not de for _, en, de in rows):
        refuse("the staged rows are empty, repeat a code or do not name both locales")
    paths = sorted({RUST, TWIN, FIXTURE, SCHEMA} | {path for path, _, _, _ in NAMINGS})
    texts = {}
    for path in paths:
        if not os.path.isfile(os.path.join(root, path)):
            refuse(f"{path} is missing under {root}")
        texts[path] = open(os.path.join(root, path), encoding="utf-8").read()
    for code, _, _ in rows:
        if any(f'"{code}"' in texts[path] for path in (RUST, TWIN, FIXTURE)):
            refuse(f"{code} already stands in a kernel table")
    rust_last = next((line for line in texts[RUST].split("\n") if line.startswith(f'    ("{LAST}", ')), None)
    twin_last = next((line for line in texts[TWIN].split("\n") if line.startswith(f'  {{ code: "{LAST}", ')), None)
    fixture_last = next((line for line in texts[FIXTURE].split("\n") if line.startswith(f'    {{ "code": "{LAST}", ')), None)
    if rust_last is None or twin_last is None or fixture_last is None or fixture_last.endswith(","):
        refuse(f"the row {LAST} is not the last row of every kernel table")
    present = texts[FIXTURE].count('{ "code": ')
    length = f"pub const FRAMEWORK_FAULT_NOTICE_LABELS: [(&str, &str, &str); {present}] = ["
    swap(texts, RUST, length, length.replace(f"; {present}]", f"; {present + len(rows)}]"), 1)
    swap(texts, RUST, rust_last + "\n];", rust_last + "\n" + "\n".join(f"    ({quoted(code)}, {quoted(en)}, {quoted(de)})," for code, en, de in rows) + "\n];", 1)
    swap(texts, TWIN, twin_last + "\n] as const;", twin_last + "\n" + "\n".join(f"  {{ code: {quoted(code)}, en: {quoted(en)}, de: {quoted(de)} }}," for code, en, de in rows) + "\n] as const;", 1)
    swap(texts, FIXTURE, fixture_last + "\n  ]", fixture_last + ",\n" + ",\n".join(f'    {{ "code": {quoted(code)}, "en": {quoted(en)}, "de": {quoted(de)} }}' for code, en, de in rows) + "\n  ]", 1)
    swap(texts, SCHEMA, OLD_PATTERN, '"pattern": ' + json.dumps(staged["schemaPattern"]), 1)
    for path, old, new, expected in NAMINGS:
        swap(texts, path, old, new, expected)
    json.loads(texts[FIXTURE])
    json.loads(texts[SCHEMA])
    sites = sum(expected for _, _, _, expected in NAMINGS)
    if apply:
        for path in paths:
            with open(os.path.join(root, path), "w", encoding="utf-8") as handle:
                handle.write(texts[path])
    print(f"[land-notices] {len(rows)} row(s) × 3 tables ({present} → {present + len(rows)}), 1 schema pattern, {sites} SDK refusal(s) named in {len(paths)} file(s) under {root}: {'WRITTEN' if apply else 'planned (check only; pass --apply)'}")


main()
