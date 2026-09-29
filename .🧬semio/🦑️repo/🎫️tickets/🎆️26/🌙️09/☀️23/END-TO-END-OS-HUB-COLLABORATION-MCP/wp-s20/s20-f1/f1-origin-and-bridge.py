"""🧯️ S20 faults overlay, session 15 (coordinator decisions 20:0x):
(1) SDK-raised codes are Framework-origin catalog entries — every `FaultOrigin::App` a framework crate raises with (plugin SDK:
    `app.command.unsupported`, `app.intent.version-mismatch`, `app.command.no-effect`, `app.example.*`, `mutation.rejected`,
    `transaction.member-rejected`) becomes `FaultOrigin::Framework`: hosts render an `app` origin from the plugin's
    declarations only, so an app-origin framework code rendered as a bare code. The law gains `framework-app-origin`.
(2) The ONE pass-1 exclusion: the mutation-report bridge `impl MutationMessageCode for &'static str` (replication mutation
    module), named by path + item, removed by the pass-2 landing.
Both with self-test fixture cases. Idempotent. Usage: python3 f1-origin-and-bridge.py [--dry-run]"""
from __future__ import annotations

import json
import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
SDK = OVERLAY / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
CENSUS = OVERLAY / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts"
FIXTURE = OVERLAY / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧫️fixtures/🧮️source-census/🔣️.json"

CENSUS_EDITS: list[tuple[str, str]] = [
    (
        'const FAULT_PLUGIN_ROOT = "✏️s/🔌️plugins/";\n',
        'const FAULT_PLUGIN_ROOT = "✏️s/🔌️plugins/";\n'
        "/** 🌉️ The ONE exclusion of the fault law, for pass 1 only: the mutation-report bridge turns a report's literal code into a\n"
        " * `FaultCode` (`FaultCode::new(self)`). Mutation reports are localized by code in pass 2, whose landing removes the bridge\n"
        " * and this exclusion together; nothing else is ever excluded. */\n"
        'export const FAULT_PASS2_BRIDGE = { path: "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs", item: "impl MutationMessageCode for &\'static str" } as const;\n',
    ),
    (
        '  | "placeholder-mismatch" | "parameter-mismatch";\n',
        '  | "placeholder-mismatch" | "parameter-mismatch" | "framework-app-origin";\n',
    ),
    (
        "  const definitions = path === FAULT_DEFINITION_MODULE;\n",
        "  const definitions = path === FAULT_DEFINITION_MODULE;\n"
        "  const bridge = path === FAULT_PASS2_BRIDGE.path ? itemBodySpan(masked, FAULT_PASS2_BRIDGE.item) : null;\n",
    ),
    (
        "    else if (!definitions) violate(event, \"computed-code\", `FaultCode::new(${argument})`);\n",
        "    else if (!definitions && !(bridge !== null && event > bridge.start && event < bridge.end)) violate(event, \"computed-code\", `FaultCode::new(${argument})`);\n",
    ),
    (
        '  if (owner.kind === "app") for (const match of masked.matchAll(/(?<![\\w])(?:[A-Za-z_]\\w*::)*Fault::new\\s*\\(/gu)) violate(match.index!, "app-raise-form", "Fault::new — an app refuses through app_fault");\n',
        '  if (owner.kind === "app") for (const match of masked.matchAll(/(?<![\\w])(?:[A-Za-z_]\\w*::)*Fault::new\\s*\\(/gu)) violate(match.index!, "app-raise-form", "Fault::new — an app refuses through app_fault");\n'
        '  if (owner.kind === "framework" && !definitions) for (const match of masked.matchAll(/(?<![\\w])FaultOrigin::App\\b/gu)) violate(match.index!, "framework-app-origin", "a framework crate raises with a framework origin — hosts render an app origin from the plugin\'s declarations only");\n',
    ),
    (
        "export function faultFactsOfText(path: string, text: string): FaultFacts {\n",
        "/** 🧱️ The span of one item's `{…}` body in literal-masked Rust (`null` when the item is absent). */\n"
        "function itemBodySpan(masked: string, item: string): { start: number; end: number } | null {\n"
        "  const at = masked.indexOf(item);\n"
        "  const open = at < 0 ? -1 : masked.indexOf(\"{\", at + item.length);\n"
        "  if (open < 0) return null;\n"
        "  let depth = 0;\n"
        "  for (let index = open; index < masked.length; index += 1) {\n"
        "    if (masked[index] === \"{\") depth += 1;\n"
        "    else if (masked[index] === \"}\" && --depth === 0) return { start: open, end: index };\n"
        "  }\n"
        "  return null;\n"
        "}\n\n"
        "export function faultFactsOfText(path: string, text: string): FaultFacts {\n",
    ),
]

FIXTURE_CASES = [
    {
        "name": "the pass-2 mutation-report bridge is the one excluded computed code; the same form elsewhere in its module is not",
        "path": "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs",
        "text": "impl MutationMessageCode for &'static str {\n    fn into_fault_code(self) -> FaultCode {\n        FaultCode::new(self)\n    }\n}\nimpl MutationMessageCode for Label {\n    fn into_fault_code(self) -> FaultCode {\n        FaultCode::new(self.0)\n    }\n}\n",
        "expected": {"raises": [], "constRaises": [], "consts": [], "declarations": [], "violations": [{"line": 8, "rule": "computed-code"}]},
    },
    {
        "name": "a framework crate raises with a framework origin, never the app origin",
        "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
        "text": "fn unsupported(action: &str) -> Fault {\n    Fault::new(FaultOrigin::App, FaultCode::new(\"app.command.unsupported\"), \"m\").with_parameter(\"action\", action)\n}\nfn gone() -> Fault {\n    Fault::new(FaultOrigin::Framework, FaultCode::new(\"app.example.unknown\"), \"m\")\n}\n",
        "expected": {"raises": [{"line": 2, "code": "app.command.unsupported", "parameters": ["action"]}, {"line": 5, "code": "app.example.unknown", "parameters": []}], "constRaises": [], "consts": [], "declarations": [], "violations": [{"line": 2, "rule": "framework-app-origin"}]},
    },
]


def main(dry_run: bool) -> None:
    census = CENSUS.read_text()
    for old, new in CENSUS_EDITS:
        if new in census:
            continue
        assert census.count(old) == 1, old[:80]
        census = census.replace(old, new)
        print("census  ", new.splitlines()[0][:90])
    sdk = SDK.read_text()
    print(f"sdk      FaultOrigin::App → Framework ×{sdk.count('FaultOrigin::App')}")
    sdk = sdk.replace("FaultOrigin::App", "FaultOrigin::Framework")
    fixture = FIXTURE.read_text()
    names = {case["name"] for case in json.loads(fixture)["faultFacts"]}
    added = [case for case in FIXTURE_CASES if case["name"] not in names]
    end = '\n  ],\n  "faultLaw": ['
    assert fixture.count(end) == 1
    fixture = fixture.replace(end, "".join(",\n    " + json.dumps(case, ensure_ascii=False) for case in added) + end) if added else fixture
    json.loads(fixture)
    print(f"fixture  +{len(added)} fault-facts cases")
    if not dry_run:
        CENSUS.write_text(census)
        SDK.write_text(sdk)
        FIXTURE.write_text(fixture)


if __name__ == "__main__":
    main("--dry-run" in sys.argv[1:])
