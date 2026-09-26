#!/usr/bin/env python3
"""⚖️ P8 post-publish patch set: declared-verb law refinements found by the fleet run (ticket-local harness).

A  The boot example: a surface whose `setActiveExample` declares its example as a closed choice without a default is
   booted with the choice's FIRST option — the example the shell's navbar opens — instead of an empty id every such
   surface refuses (wfc grid2d: `wfc-grid2d-unknown-example`). The staged probe of `setActiveExample` itself still runs
   with the declared defaults.
C  The third-party twin the fixture always named but nobody wrote: `🧪️tests/⚖️declared-verb-verdicts/🟦️.ts` states every
   verdict rule as one JSON Schema over the probe (AJV evaluates it; `$data` compares the two perturbed outcomes), answers
   every case exactly as the Rust law does (red-checked by mutating an ignored-argument and an agent case), and runs in the
   SDK's `test` script; the fixture's `why` names its real path.
The orphan rule is its own set (`p8-orphan.py`), held until the content-addressed-child class is fixed.
Usage: p8-law.py --dry-run | --write"""
import json

from p8_patch import ROOT, create, finish, replace, text

SDK = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
VERDICT_TEST = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🦀️.rs"
VERDICT_FIXTURE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/⚖️declared-verb-verdicts.json"
VERDICT_TWIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts"
SDK_SCRIPT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts"

replace(
    "A",
    SDK,
    """                .find_map(|(index, window)| window_kind_actions(&definition, window).into_iter().find(|action| action.id == "setActiveExample").map(|action| DeclaredVerbBoot { args: staged_of(action), view: view_of(index) }));""",
    """                .find_map(|(index, window)| window_kind_actions(&definition, window).into_iter().find(|action| action.id == "setActiveExample").map(|action| DeclaredVerbBoot { args: declared_verb_boot_args(action, staged_of(action)), view: view_of(index) }));""",
)
replace(
    "A",
    SDK,
    """        /// 🧫️ The per-probe fixture: the registered app, bound as instance 1, booted with the app's own
        /// example when it declares one.""",
    """        /// 🎬️ The arguments the shell's example picker boots with: the staged declared defaults, and for every
        /// closed choice still without a value its first option — the example the navbar opens first.
        fn declared_verb_boot_args(action: &semio_framework::ActionDefinition, staged: semio_framework::DslValue) -> semio_framework::DslValue {
            use semio_framework::{ArgSchema, DslValue};
            let DslValue::Object(mut entries) = staged else { return staged };
            for argument in &action.args {
                let ArgSchema::String { options, .. } = &argument.schema else { continue };
                let Some(first) = options.first() else { continue };
                if !entries.iter().any(|(key, value)| key == &argument.id && !matches!(value, DslValue::Null) && value.as_str() != Some("")) {
                    entries.retain(|(key, _)| key != &argument.id);
                    entries.push((argument.id.clone(), DslValue::String(first.value.clone())));
                }
            }
            DslValue::Object(entries)
        }

        /// 🧫️ The per-probe fixture: the registered app, bound as instance 1, booted with the app's own
        /// example when it declares one.""",
)

fixture = text(VERDICT_FIXTURE)
document = json.loads(fixture)
TWIN_PATH_OLD = "the AJV twin (`📦️packages/🟦️typescript/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts`, which states the same rules as JSON Schema)"
TWIN_PATH_NEW = "the AJV twin (`🧪️tests/⚖️declared-verb-verdicts/🟦️.ts`, run by the SDK's `test` script, which states every rule as one JSON Schema over the probe and lets AJV evaluate it)"
if document["why"].count(TWIN_PATH_OLD) != 1:
    raise SystemExit("C: the verdict fixture's why no longer names the twin path as expected")
document["why"] = document["why"].replace(TWIN_PATH_OLD, TWIN_PATH_NEW)
replace("C", VERDICT_FIXTURE, fixture, json.dumps(document, indent=2, ensure_ascii=False) + "\n")
create("C", VERDICT_TWIN, 'import assert from "node:assert/strict";\nimport { readFileSync } from "node:fs";\nimport Ajv from "ajv";\n\ntype Effect = Readonly<{ lanes: string[]; documentChanged: boolean; documentReplaced: boolean; configChanged: boolean; userPathWritten: boolean; hostEffects: number; fingerprint: number }>;\ntype Outcome = Readonly<{ settled: Effect } | { refused: { code: string; detail: string } } | { unreachable: { code: string; detail: string } }>;\ntype Argument = Readonly<{ argument: string; othersSpecified: boolean; first: Outcome; second: Outcome }>;\ntype Probe = Readonly<{ verb: string; kind: string; audience: string; destructive: boolean; bridge: string | null; windows: Readonly<{ window: string; staged: Outcome; arguments: Argument[] }>[]; agent: Outcome | null }>;\ntype Finding = Readonly<{ finding: string; argument?: string }>;\ntype Fixture = Readonly<{ why: string; cases: Readonly<{ name: string; probe: Probe; findings: Finding[] }>[] }>;\n\nconst ajv = new Ajv({ strict: true, allErrors: true, $data: true });\n\nconst touches = { type: "object", anyOf: [{ required: ["documentChanged"], properties: { documentChanged: { const: true } } }, { required: ["documentReplaced"], properties: { documentReplaced: { const: true } } }, { required: ["lanes"], properties: { lanes: { type: "array", contains: { enum: ["artifact", "child"] } } } }] };\nconst silent = {\n  type: "object",\n  required: ["configChanged", "userPathWritten", "hostEffects", "lanes"],\n  not: touches,\n  properties: { configChanged: { const: false }, userPathWritten: { const: false }, hostEffects: { const: 0 }, lanes: { type: "array", items: { enum: ["terminal", "ui"] } } },\n};\nconst settled = (effect: object) => ({ type: "object", required: ["settled"], properties: { settled: effect } });\nconst refused = { type: "object", required: ["refused"], properties: { refused: { type: "object" } } };\nconst unreachable = { type: "object", required: ["unreachable"], properties: { unreachable: { type: "object" } } };\nconst refusedOrUnreachable = { anyOf: [refused, unreachable] };\n\n/** 📜️ Every rule of the verdict as one JSON Schema over the probe\'s flattened view, evaluated by AJV — the third-party engine. */\nconst rules = {\n  unbridged: ajv.compile({ type: "object", required: ["bridge", "audience"], properties: { bridge: { type: "string" }, audience: { not: { const: "input" } } } }),\n  bridged: ajv.compile({ type: "object", required: ["bridge"], properties: { bridge: { type: "null" } } }),\n  unreachable: ajv.compile({ type: "object", required: ["staged"], properties: { staged: { type: "array", minItems: 1, items: unreachable } } }),\n  documentWriteFromNonMutation: ajv.compile({ type: "object", required: ["kind", "effects"], properties: { kind: { enum: ["view", "shell"] }, effects: { type: "array", contains: touches } } }),\n  silentMutation: ajv.compile({ type: "object", required: ["kind", "audience", "outcomes"], properties: { kind: { const: "mutation" }, audience: { const: "agent" }, outcomes: { type: "array", items: settled(silent) } } }),\n  destructiveWithoutDiscard: ajv.compile({\n    type: "object",\n    required: ["destructive", "effects"],\n    properties: { destructive: { const: true }, effects: { type: "array", minItems: 1, items: { type: "object", not: { anyOf: [touches, { type: "object", required: ["userPathWritten"], properties: { userPathWritten: { const: true } } }] } } } },\n  }),\n  agentLaneDiverges: ajv.compile({\n    type: "object",\n    required: ["agent", "staged"],\n    properties: { agent: { type: "object" }, staged: { type: "array" } },\n    anyOf: [\n      { oneOf: [{ type: "object", properties: { staged: { type: "array", contains: settled(touches) } } }, { type: "object", properties: { agent: settled(touches) } }] },\n      { type: "object", properties: { agent: refusedOrUnreachable, staged: { type: "array", contains: settled({ type: "object", not: silent }) } } },\n    ],\n  }),\n  argumentAudience: ajv.compile({ type: "object", required: ["audience"], properties: { audience: { not: { const: "input" } } } }),\n  ignoredArgument: ajv.compile({\n    type: "array",\n    minItems: 1,\n    items: {\n      type: "object",\n      anyOf: [\n        { type: "object", required: ["first", "second"], properties: { first: settled({ type: "object" }), second: { const: { $data: "1/first" } } } },\n        { type: "object", required: ["first", "second", "staged", "othersSpecified"], properties: { first: refused, second: { const: { $data: "1/first" } }, staged: { const: { $data: "1/first" } }, othersSpecified: { const: true } } },\n      ],\n    },\n  }),\n};\n\n/** 🧮️ An outcome in the Rust verdict\'s own equality: lanes are a set. */\nconst normal = (outcome: Outcome): Outcome => ("settled" in outcome ? { settled: { ...outcome.settled, lanes: [...new Set(outcome.settled.lanes)].sort() } } : outcome);\n\n/** 🔍️ The findings of one probe, in the Rust verdict\'s order: an unbridged or unreachable verb reports that alone. */\nfunction findings(probe: Probe): Finding[] {\n  const staged = probe.windows.map((window) => normal(window.staged));\n  const outcomes = probe.windows.flatMap((window) => [window.staged, ...window.arguments.flatMap((argument) => [argument.first, argument.second])]).map(normal);\n  const effects = outcomes.flatMap((outcome) => ("settled" in outcome ? [outcome.settled] : []));\n  const view = { kind: probe.kind, audience: probe.audience, destructive: probe.destructive, bridge: probe.bridge, staged, outcomes, effects, ...(probe.agent === null ? {} : { agent: normal(probe.agent) }) };\n  if (!rules.bridged(view)) return rules.unbridged(view) ? [{ finding: "unbridged" }] : [];\n  if (rules.unreachable(view)) return [{ finding: "unreachable" }];\n  const found: Finding[] = (["documentWriteFromNonMutation", "silentMutation", "destructiveWithoutDiscard", "agentLaneDiverges"] as const).filter((rule) => rules[rule](view)).map((rule) => ({ finding: rule }));\n  if (!rules.argumentAudience(view)) return found;\n  const names = [...new Set(probe.windows.flatMap((window) => window.arguments.map((argument) => argument.argument)))];\n  for (const name of names) {\n    const rows = probe.windows.flatMap((window) => window.arguments.filter((argument) => argument.argument === name).map((argument) => ({ staged: normal(window.staged), first: normal(argument.first), second: normal(argument.second), othersSpecified: argument.othersSpecified })));\n    if (rules.ignoredArgument(rows)) found.push({ finding: "ignoredArgument", argument: name });\n  }\n  return found;\n}\n\n/** ⚖️ The AJV twin of `artifact_app_laws::declared_verb_findings`: every case of the language-agnostic verdict fixture answered exactly as written. */\nexport function declaredVerbVerdictOracle(): number {\n  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/⚖️declared-verb-verdicts.json", import.meta.url), "utf8")) as Fixture;\n  assert(fixture.cases.length >= 28, "the fixture keeps every rule\'s positive and negative case");\n  for (const { name, probe, findings: expected } of fixture.cases) assert.deepEqual(findings(probe), expected.map((finding) => (finding.argument === undefined ? { finding: finding.finding } : finding)), name);\n  return fixture.cases.length;\n}\n\nif (import.meta.main) console.log(`declared-verb-verdict-oracle cases=${declaredVerbVerdictOracle()}`);\n')
replace("C", SDK_SCRIPT, """import { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } from "../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts";
""", """import { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } from "../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts";
import { declaredVerbVerdictOracle } from "../../🧪️tests/⚖️declared-verb-verdicts/🟦️.ts";
""")
replace("C", SDK_SCRIPT, """    console.log(`completion-rejection-oracle assertions=${completionRejectionOracle(this.repoRoot)}`);
""", """    console.log(`completion-rejection-oracle assertions=${completionRejectionOracle(this.repoRoot)}`);
    console.log(`declared-verb-verdict-oracle cases=${declaredVerbVerdictOracle()}`);
""")

finish(__doc__)
