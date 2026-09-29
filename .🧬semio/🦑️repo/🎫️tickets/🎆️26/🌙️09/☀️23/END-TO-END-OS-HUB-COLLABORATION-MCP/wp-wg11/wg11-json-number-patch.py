#!/usr/bin/env python3
"""🔢️ WG11 session 14d — set for the first train after the chain: an `f64`-carried JSON number crosses every Rust boundary the
way its JSON text reads back (`JSON.stringify(7.0)` is `"7"`).

Measured: (a) LW1 `wg11-laws-1.txt`, `reconcile::tests::an_editable_table_row_keeps_one_child_per_cell_and_its_remove_action_as_a_row_
action` — an integer row-action argument (`row: 7`) reached the guest as `Float(7.0)`: the UI contract carries every JSON number as
`UiValue::Number(f64)` and its DERIVED `Serialize` wrote `7.0`, so the wgpu reconcile's `serde_json::to_value` bridge produced a float;
(b) WG11 overlay proof 07:49 — `the_reserved_media_transport_projects_only_truthful_localized_host_status` red for the same reason:
an Extension's `params_json` is `serde_json::to_string(&UiValue)`, `"schemaVersion": 1.0` fails `as_u64`, and every valid media
transport reads "invalid contract"; (c) the SDK's typed-intent producer mapped `UiValue::Number` through `DslValue::float`. React's
wire is JSON text, which is why stdio (`window_kit_required_index_argument`) and tool-run (`tool_run_arg_u64`) grew per-call-site
tolerance for integral floats.

Fix — ONE rule, in the value module (`protocol::value`, no `DslValue` in its signature so the UI contract may use it):
`json_integer(f64) -> Option<JsonInteger>` = the integer a finite, integral, safe (`Number.isSafeInteger`) number reads back as.
`DslValue::json_number` (non-finite → `Null`, else integer or float) is built on it and used by the SDK producer; the UI contract's
`UiValue` gains a hand-written `Serialize` that writes a number through the same rule (integers as JSON integers, non-finite as
`null`), so EVERY JSON projection of a UI value — reconcile args, extension params, wire snapshots — reads back as React's does.
Laws: shared vectors `🌱️value/🧫️fixtures/🔣️json-projection/🔣️.json` `numbers` — TS pins the `json` column to `JSON.stringify`
(the platform's own JSON text); Rust `json_number` against `serde_json`'s parse of that text (third-party oracle); the contract's
`UiValue` serialization against the same column; the wgpu reconcile laws (`row: 7` → `uint(7)`, media transport) are the
integration witnesses; the tool-run panel laws, which pinned the old float echo (`generation: 0.0`), now expect the JSON
integer React sends (`generation: 0`).

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/json-number/` and applies;
`--revert` restores the backups.
"""

import difflib
import json
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
VALUE = ROOT / "🧰️framework/🔨️modules/🌱️value"
VALUE_RS = VALUE / "🦀️.rs"
VALUE_LAWS = VALUE / "🧪️tests/🔬️unit/🦀️.rs"
FIXTURE = VALUE / "🧫️fixtures/🔣️json-projection/🔣️.json"
FIXTURE_TS = VALUE / "🧪️tests/🔣️json-projection/🟦️.ts"
CONTRACT = ROOT / "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs"
CONTRACT_LAWS = ROOT / "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️action-unit/🦀️.rs"
SDK = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
TOOL_RUN_LAWS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/json-number"

NUMBERS = [
    {"literal": "7.0", "json": "7", "kind": "uint"},
    {"literal": "0.0", "json": "0", "kind": "uint"},
    {"literal": "-0.0", "json": "0", "kind": "uint"},
    {"literal": "-3.0", "json": "-3", "kind": "int"},
    {"literal": "2.5", "json": "2.5", "kind": "float"},
    {"literal": "9007199254740991.0", "json": "9007199254740991", "kind": "uint"},
    {"literal": "-9007199254740991.0", "json": "-9007199254740991", "kind": "int"},
    {"literal": "9007199254740992.0", "json": "9007199254740992", "kind": "float"},
    {"literal": "1e21", "json": "1e+21", "kind": "float"},
    {"literal": "NaN", "json": "null", "kind": "null"},
    {"literal": "Infinity", "json": "null", "kind": "null"},
]

VALUE_EDITS = [
    (
        '''impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Number::Float(v)
    }
}
//#endregion 🔖️Number''',
        '''impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Number::Float(v)
    }
}

/// 🔢️ The integer one finite JSON number reads back as — [`json_integer`]'s answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JsonInteger {
    Unsigned(u64),
    Signed(i64),
}

/// 🔢️ The integer an `f64` reads back as from its JSON text: `JSON.stringify(7.0)` is `"7"`, so a finite, integral number
/// within the safe integer range (`Number.isSafeInteger`, |v| ≤ 2^53 − 1) is an integer — unsigned unless negative — and
/// anything else (a fraction, a non-finite value, an integral value past the safe range) is not. The ONE rule every Rust
/// boundary that carries a JSON number as `f64` reads it by (`DslValue::json_number`, the UI contract's `UiValue` serializer).
pub fn json_integer(v: f64) -> Option<JsonInteger> {
    const SAFE_INTEGER_MAX: f64 = 9_007_199_254_740_991.0;
    if !v.is_finite() || v.fract() != 0.0 || v.abs() > SAFE_INTEGER_MAX {
        return None;
    }
    Some(if v >= 0.0 { JsonInteger::Unsigned(v as u64) } else { JsonInteger::Signed(v as i64) })
}
//#endregion 🔖️Number''',
    ),
    (
        '''    pub fn float(v: f64) -> Self {
        Self::Number(Number::Float(v))
    }
''',
        '''    pub fn float(v: f64) -> Self {
        Self::Number(Number::Float(v))
    }

    /// 🔢️ A JSON number carried as `f64` (the UI contract's `UiValue::Number`), read the way its JSON text reads back
    /// ([`json_integer`]): an integer stays an integer, a non-finite value is `Null` (JSON text has neither NaN nor Infinity),
    /// anything else a float — so an integer argument a renderer hands back decodes as the integer React's wire delivers.
    pub fn json_number(v: f64) -> Self {
        match json_integer(v) {
            Some(JsonInteger::Unsigned(value)) => Self::uint(value),
            Some(JsonInteger::Signed(value)) => Self::int(value),
            None if v.is_finite() => Self::float(v),
            None => Self::Null,
        }
    }
''',
    ),
]

VALUE_LAW = r'''
/// 🔢️ `DslValue::json_number` reads an `f64` the way its JSON text reads back — the shared `numbers` vectors of
/// `🧫️fixtures/🔣️json-projection/🔣️.json`, whose `json` column the TypeScript law pins to `JSON.stringify`: every integer, null and
/// in-range case equals `serde_json`'s parse of that text (third-party oracle); past the safe integer range an `f64` stays a float.
#[test]
fn a_json_number_reads_back_as_its_json_text() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️json-projection/🔣️.json")).expect("the json-projection fixture parses");
    let numbers = fixture["numbers"].as_array().expect("numbers");
    assert!(!numbers.is_empty());
    for case in numbers {
        let literal = case["literal"].as_str().expect("literal");
        let value: f64 = literal.parse().expect("an f64 literal");
        let bridged = DslValue::json_number(value);
        let text = serde_json::from_str::<serde_json::Value>(case["json"].as_str().expect("json")).expect("json text parses");
        match case["kind"].as_str().expect("kind") {
            "uint" => assert!(matches!(bridged, DslValue::Number(Number::UInt(_))) && bridged == DslValue::from(&text), "{literal}: {bridged:?}"),
            "int" => assert!(matches!(bridged, DslValue::Number(Number::Int(_))) && bridged == DslValue::from(&text), "{literal}: {bridged:?}"),
            "null" => assert_eq!((bridged, DslValue::from(&text)), (DslValue::Null, DslValue::Null), "{literal}"),
            "float" => assert_eq!(bridged, DslValue::float(value), "{literal}"),
            kind => panic!("unknown kind {kind}"),
        }
    }
}
'''

FIXTURE_TS_EDITS = [
    (
        '''  const child = { value: 1 }, shared = { left: child, right: child };
  assert.deepEqual(parseDslValue(shared), shared);
}''',
        '''  const child = { value: 1 }, shared = { left: child, right: child };
  assert.deepEqual(parseDslValue(shared), shared);
  for (const number of fixture.numbers) assert.equal(JSON.stringify(Number(number.literal)), number.json, number.literal);
}''',
    )
]

CONTRACT_EDITS = [
    (
        '''#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[expect(clippy::large_enum_variant, reason = "Text stays in its fixed inline byte ceiling; collection payloads use separately credited arena handles.")]
pub enum UiValue {''',
        '''#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(untagged)]
#[expect(clippy::large_enum_variant, reason = "Text stays in its fixed inline byte ceiling; collection payloads use separately credited arena handles.")]
pub enum UiValue {''',
    ),
    (
        '''impl UiValue {
    pub fn credited_clone(&self) -> Option<Self> {
        with_ui_value_arena(|arena| arena.try_clone_value(self))
    }
}
''',
        '''impl UiValue {
    pub fn credited_clone(&self) -> Option<Self> {
        with_ui_value_arena(|arena| arena.try_clone_value(self))
    }
}

/// 🔢️ A `UiValue` serializes as the JSON text React's wire carries: a number goes through the value module's one rule
/// ([`protocol::value::json_integer`]) — `7.0` is written `7`, a non-finite number `null` — so every JSON projection of a UI
/// value (reconciled action arguments, extension params, snapshots) reads back exactly as `JSON.parse(JSON.stringify(v))`.
impl Serialize for UiValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            UiValue::Null => serializer.serialize_unit(),
            UiValue::Bool(value) => serializer.serialize_bool(*value),
            UiValue::Number(value) => match protocol::value::json_integer(*value) {
                Some(protocol::value::JsonInteger::Unsigned(integer)) => serializer.serialize_u64(integer),
                Some(protocol::value::JsonInteger::Signed(integer)) => serializer.serialize_i64(integer),
                None if value.is_finite() => serializer.serialize_f64(*value),
                None => serializer.serialize_unit(),
            },
            UiValue::Text(value) => value.serialize(serializer),
            UiValue::List(value) => value.serialize(serializer),
            UiValue::Map(value) => value.serialize(serializer),
        }
    }
}
''',
    ),
]

CONTRACT_LAW = r'''
/// 🔢️ A `UiValue::Number` serializes as its JSON text reads back — the shared `numbers` vectors of
/// `🌱️value/🧫️fixtures/🔣️json-projection/🔣️.json` (TypeScript pins their `json` column to `JSON.stringify`): an integer, a
/// non-finite and an in-range number write exactly that text; a float past it reads back as the same `f64`.
#[test]
fn a_ui_number_serializes_as_its_json_text() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🌱️value/🧫️fixtures/🔣️json-projection/🔣️.json")).expect("the shared json-projection vectors");
    for case in fixture["numbers"].as_array().expect("numbers") {
        let literal = case["literal"].as_str().expect("literal");
        let value: f64 = literal.parse().expect("an f64 literal");
        let text = serde_json::to_string(&UiValue::Number(value)).expect("a number serializes");
        match case["kind"].as_str().expect("kind") {
            "float" => assert_eq!(serde_json::from_str::<f64>(&text).expect("float text"), value, "{literal}: {text}"),
            _ => assert_eq!(text, case["json"].as_str().expect("json"), "{literal}"),
        }
        value_round_trips(UiValue::Number(if value.is_finite() { value } else { 0.0 }));
    }
}
'''

SDK_EDITS = [
    (
        '''                    UiValue::Number(value) => self.accept(if value.is_finite() { DslValue::float(value) } else { DslValue::Null }),''',
        '''                    UiValue::Number(value) => self.accept(DslValue::json_number(value)),''',
    )
]


TOOL_RUN_LAW_EDITS = [
    (
        '''            serde_json::json!({ "action": "toolRunPause", "runId": "1", "generation": 0.0 }),
            serde_json::json!({ "action": "toolRunFinalize", "runId": "1", "generation": 0.0 }),
            serde_json::json!({ "action": "toolRunAbort", "runId": "1", "generation": 0.0 }),''',
        '''            serde_json::json!({ "action": "toolRunPause", "runId": "1", "generation": 0 }),
            serde_json::json!({ "action": "toolRunFinalize", "runId": "1", "generation": 0 }),
            serde_json::json!({ "action": "toolRunAbort", "runId": "1", "generation": 0 }),''',
    ),
    (
        '''vec![serde_json::json!({ "action": "toolRunPause", "runId": "1", "generation": 0.0 })], "Enter on the focused Pause button dispatches the run's pause");''',
        '''vec![serde_json::json!({ "action": "toolRunPause", "runId": "1", "generation": 0 })], "Enter on the focused Pause button dispatches the run's pause");''',
    ),
]


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def fixture_after(source: str) -> str:
    value = json.loads(source)
    if "numbers" in value:
        sys.exit("fixture already carries numbers — landed already")
    value["numbers"] = NUMBERS
    lines = ["{"]
    items = list(value.items())
    for index, (key, entry) in enumerate(items):
        comma = "," if index + 1 < len(items) else ""
        if key == "numbers":
            lines.append('  "numbers": [')
            lines.extend(f"    {json.dumps(case, ensure_ascii=False)}{',' if position + 1 < len(entry) else ''}" for position, case in enumerate(entry))
            lines.append(f"  ]{comma}")
        else:
            lines.append(f"  {json.dumps(key)}: {json.dumps(entry, ensure_ascii=False, separators=(', ', ': '))}{comma}")
    lines.append("}")
    after = "\n".join(lines) + "\n"
    if json.loads(after) != value:
        sys.exit("the rewritten fixture does not read back as the planned value")
    return after


FILES = [VALUE_RS, VALUE_LAWS, FIXTURE, FIXTURE_TS, CONTRACT, CONTRACT_LAWS, SDK, TOOL_RUN_LAWS]


def plans():
    read = {path: path.read_text(encoding="utf-8") for path in FILES}
    return [
        (VALUE_RS, read[VALUE_RS], replaced(VALUE_RS, read[VALUE_RS], VALUE_EDITS)),
        (VALUE_LAWS, read[VALUE_LAWS], read[VALUE_LAWS].rstrip("\n") + "\n" + VALUE_LAW),
        (FIXTURE, read[FIXTURE], fixture_after(read[FIXTURE])),
        (FIXTURE_TS, read[FIXTURE_TS], replaced(FIXTURE_TS, read[FIXTURE_TS], FIXTURE_TS_EDITS)),
        (CONTRACT, read[CONTRACT], replaced(CONTRACT, read[CONTRACT], CONTRACT_EDITS)),
        (CONTRACT_LAWS, read[CONTRACT_LAWS], read[CONTRACT_LAWS].rstrip("\n") + "\n" + CONTRACT_LAW),
        (SDK, read[SDK], replaced(SDK, read[SDK], SDK_EDITS)),
        (TOOL_RUN_LAWS, read[TOOL_RUN_LAWS], replaced(TOOL_RUN_LAWS, read[TOOL_RUN_LAWS], TOOL_RUN_LAW_EDITS)),
    ]


def main():
    if "--revert" in sys.argv:
        for path in FILES:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print(f"REVERTED: {len(FILES)} files restored from backups")
        return
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files (crates: semio-framework-replication (value), semio-framework-ui-contract, semio-framework-plugin; value TS law)")


if __name__ == "__main__":
    main()
