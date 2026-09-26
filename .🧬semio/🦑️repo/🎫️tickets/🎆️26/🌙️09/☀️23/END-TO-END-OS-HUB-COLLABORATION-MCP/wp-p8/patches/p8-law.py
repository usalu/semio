#!/usr/bin/env python3
"""⚖️ P8 post-publish patch set: declared-verb law refinements found by the fleet run (ticket-local harness).

A  The boot example: a surface whose `setActiveExample` declares its example as a closed choice without a default is
   booted with the choice's FIRST option — the example the shell's navbar opens — instead of an empty id every such
   surface refuses (wfc grid2d: `wfc-grid2d-unknown-example`). The staged probe of `setActiveExample` itself still runs
   with the declared defaults.
B  `composedChildOrphaned`: a verb that leaves the parent naming a composed child no store holds (flow's parent-lane edits
   re-minted its content-addressed child; every later verb, window and reload read nothing). The probe records every
   declared child that is unheld after the verb and was not before; the verdict fixture gains the case; the Rust verdict
   test reads `orphanedChildren`.
C  The third-party twin the fixture always named but nobody wrote: `🧪️tests/⚖️declared-verb-verdicts/🟦️.ts` states every
   verdict rule as one JSON Schema over the probe (AJV evaluates it; `$data` compares the two perturbed outcomes), answers
   all 29 cases exactly as the Rust law does (red-checked by mutating an orphan, an ignored-argument and an agent case), and
   runs in the SDK's `test` script; the fixture's `why` names its real path and the orphan rule.
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

replace("B", SDK, """            pub host_effects: usize,
            pub fingerprint: u64,
        }

        impl DeclaredVerbEffect {""", """            pub host_effects: usize,
            pub fingerprint: u64,
            /// 🪆️ `slot/child_id` of every composed child the parent names after the verb and no store holds,
            /// that it did not already name unheld before the verb.
            pub orphaned_children: Vec<String>,
        }

        impl DeclaredVerbEffect {""")
replace("B", SDK, """            AgentLaneDiverges { verb: String, shell: String, agent: String },
        }""", """            AgentLaneDiverges { verb: String, shell: String, agent: String },
            /// 🪆️ The verb left the parent naming a composed child no store holds — every later verb, window and
            /// reload that reads the child finds nothing (flow's parent-lane edits re-minted its content child).
            ComposedChildOrphaned { verb: String, children: String },
        }""")
replace("B", SDK, """                    DeclaredVerbFinding::AgentLaneDiverges { verb, shell, agent } => write!(formatter, "{verb}: the agent lane diverges from the shell lane (shell {shell}; agent {agent})"),
                }""", """                    DeclaredVerbFinding::AgentLaneDiverges { verb, shell, agent } => write!(formatter, "{verb}: the agent lane diverges from the shell lane (shell {shell}; agent {agent})"),
                    DeclaredVerbFinding::ComposedChildOrphaned { verb, children } => write!(formatter, "{verb}: leaves the parent naming composed children no store holds ({children})"),
                }""")
replace("B", SDK, """            if probe.destructive && settled().next().is_some() && settled().all(|effect| !effect.touches_document() && !effect.user_path_written) {
                findings.push(DeclaredVerbFinding::DestructiveWithoutDiscard { verb: verb.clone() });
            }""", """            if probe.destructive && settled().next().is_some() && settled().all(|effect| !effect.touches_document() && !effect.user_path_written) {
                findings.push(DeclaredVerbFinding::DestructiveWithoutDiscard { verb: verb.clone() });
            }
            if let Some(effect) = settled().find(|effect| !effect.orphaned_children.is_empty()) {
                findings.push(DeclaredVerbFinding::ComposedChildOrphaned { verb: verb.clone(), children: effect.orphaned_children.join(", ") });
            }""")
replace("B", SDK, """        /// 🎯️ Dispatches one declared verb exactly as the shell does and reads back what it did.""", """        /// 🪆️ `slot/child_id` of every composed child the parent names right now that no child store holds.
        async fn declared_verb_unheld_children<A, M>(app: &VcsArtifactApp<A, M>) -> Vec<String>
        where
            A: ArtifactApp,
            M: super::SpaceMember + super::MemberFactory + Send + 'static,
        {
            let Ok(snapshot) = app.snapshot() else { return Vec::new() };
            let declared: Vec<(String, String)> = match store::ChildRestoreProjection::from_snapshot(&snapshot) {
                Ok(projection) => (0..projection.len()).filter_map(|index| projection.get(index)).map(|(slot, fields)| (slot.to_string(), fields.child_id.to_string())).collect(),
                Err(error) => vec![(String::from("projection"), error.to_string())],
            };
            let mut unheld = Vec::new();
            for (slot, child_id) in declared {
                if app.child_store(&slot, &child_id).await.is_none() {
                    unheld.push(format!("{slot}/{child_id}"));
                }
            }
            unheld
        }

        /// 🎯️ Dispatches one declared verb exactly as the shell does and reads back what it did.""")
replace("B", SDK, """            let (document_before, config_before) = declared_verb_state(&mut app).await;
            let action_meta = ActionMeta { view_state: Some(view.clone()), ..meta("local") };""", """            let (document_before, config_before) = declared_verb_state(&mut app).await;
            let unheld_before = declared_verb_unheld_children(&app).await;
            let action_meta = ActionMeta { view_state: Some(view.clone()), ..meta("local") };""")
replace("B", SDK, """                        let (document_after, config_after) = declared_verb_state(&mut app).await;
                        let rendered = match app.render(body_key, None, view).await {""", """                        let (document_after, config_after) = declared_verb_state(&mut app).await;
                        let orphaned_children = declared_verb_unheld_children(&app).await.into_iter().filter(|child| !unheld_before.contains(child)).collect();
                        let rendered = match app.render(body_key, None, view).await {""")
replace("B", SDK, """                            host_effects: effects.len() + events.len(),
                            fingerprint,
                        })""", """                            host_effects: effects.len() + events.len(),
                            fingerprint,
                            orphaned_children,
                        })""")
replace("B", SDK, """                    DeclaredVerbOutcome::Settled(DeclaredVerbEffect { lanes, document_changed: document != 0, document_replaced: false, config_changed: config != 0, user_path_written: false, host_effects, fingerprint })""", """                    DeclaredVerbOutcome::Settled(DeclaredVerbEffect { lanes, document_changed: document != 0, document_replaced: false, config_changed: config != 0, user_path_written: false, host_effects, fingerprint, orphaned_children: Vec::new() })""")
replace("B", VERDICT_TEST, """            fingerprint: settled["fingerprint"].as_u64().expect("fixture fingerprint"),
        })""", """            fingerprint: settled["fingerprint"].as_u64().expect("fixture fingerprint"),
            orphaned_children: settled["orphanedChildren"].as_array().map_or_else(Vec::new, |children| children.iter().map(|child| child.as_str().expect("fixture orphaned child").to_string()).collect()),
        })""")
replace("B", VERDICT_TEST, """            DeclaredVerbFinding::AgentLaneDiverges { .. } => ("agentLaneDiverges", None),""", """            DeclaredVerbFinding::AgentLaneDiverges { .. } => ("agentLaneDiverges", None),
            DeclaredVerbFinding::ComposedChildOrphaned { .. } => ("composedChildOrphaned", None),""")
fixture = text(VERDICT_FIXTURE)
document = json.loads(fixture)
document["cases"].append({
    "name": "a verb that leaves the parent naming a composed child no store holds orphans it — every later verb, window and reload reads nothing",
    "probe": {"verb": "removeWidget", "kind": "mutation", "audience": "agent", "destructive": True, "bridge": None,
              "windows": [{"window": "main", "staged": {"settled": {"lanes": ["artifact", "ui", "terminal"], "documentChanged": True, "documentReplaced": False, "configChanged": False, "userPathWritten": False, "hostEffects": 0, "fingerprint": 29, "orphanedChildren": ["content/flow-content-sha256-0f"]}}, "arguments": []}],
              "agent": None},
    "findings": [{"finding": "composedChildOrphaned"}],
})
TWIN_PATH_OLD = "the AJV twin (`📦️packages/🟦️typescript/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts`, which states the same rules as JSON Schema)"
TWIN_PATH_NEW = "the AJV twin (`🧪️tests/⚖️declared-verb-verdicts/🟦️.ts`, run by the SDK's `test` script, which states every rule as one JSON Schema over the probe and lets AJV evaluate it)"
ORPHAN_RULE_OLD = "is not refused where the shell acts;"
ORPHAN_RULE_NEW = "is not refused where the shell acts; no verb leaves the parent naming a composed child no store holds (`orphanedChildren`);"
if document["why"].count(TWIN_PATH_OLD) != 1 or document["why"].count(ORPHAN_RULE_OLD) != 1:
    raise SystemExit("B: the verdict fixture's why no longer names the twin path or the agent rule as expected")
document["why"] = document["why"].replace(TWIN_PATH_OLD, TWIN_PATH_NEW).replace(ORPHAN_RULE_OLD, ORPHAN_RULE_NEW)
replace("B", VERDICT_FIXTURE, fixture, json.dumps(document, indent=2, ensure_ascii=False) + "\n")

create("C", VERDICT_TWIN, 'import assert from "node:assert/strict";\nimport { readFileSync } from "node:fs";\nimport Ajv from "ajv";\n\ntype Effect = Readonly<{ lanes: string[]; documentChanged: boolean; documentReplaced: boolean; configChanged: boolean; userPathWritten: boolean; hostEffects: number; fingerprint: number; orphanedChildren?: string[] }>;\ntype Outcome = Readonly<{ settled: Effect } | { refused: { code: string; detail: string } } | { unreachable: { code: string; detail: string } }>;\ntype Argument = Readonly<{ argument: string; othersSpecified: boolean; first: Outcome; second: Outcome }>;\ntype Probe = Readonly<{ verb: string; kind: string; audience: string; destructive: boolean; bridge: string | null; windows: Readonly<{ window: string; staged: Outcome; arguments: Argument[] }>[]; agent: Outcome | null }>;\ntype Finding = Readonly<{ finding: string; argument?: string }>;\ntype Fixture = Readonly<{ why: string; cases: Readonly<{ name: string; probe: Probe; findings: Finding[] }>[] }>;\n\nconst ajv = new Ajv({ strict: true, allErrors: true, $data: true });\n\nconst touches = { type: "object", anyOf: [{ required: ["documentChanged"], properties: { documentChanged: { const: true } } }, { required: ["documentReplaced"], properties: { documentReplaced: { const: true } } }, { required: ["lanes"], properties: { lanes: { type: "array", contains: { enum: ["artifact", "child"] } } } }] };\nconst silent = {\n  type: "object",\n  required: ["configChanged", "userPathWritten", "hostEffects", "lanes"],\n  not: touches,\n  properties: { configChanged: { const: false }, userPathWritten: { const: false }, hostEffects: { const: 0 }, lanes: { type: "array", items: { enum: ["terminal", "ui"] } } },\n};\nconst settled = (effect: object) => ({ type: "object", required: ["settled"], properties: { settled: effect } });\nconst refused = { type: "object", required: ["refused"], properties: { refused: { type: "object" } } };\nconst unreachable = { type: "object", required: ["unreachable"], properties: { unreachable: { type: "object" } } };\nconst refusedOrUnreachable = { anyOf: [refused, unreachable] };\n\n/** 📜️ Every rule of the verdict as one JSON Schema over the probe\'s flattened view, evaluated by AJV — the third-party engine. */\nconst rules = {\n  unbridged: ajv.compile({ type: "object", required: ["bridge", "audience"], properties: { bridge: { type: "string" }, audience: { not: { const: "input" } } } }),\n  bridged: ajv.compile({ type: "object", required: ["bridge"], properties: { bridge: { type: "null" } } }),\n  unreachable: ajv.compile({ type: "object", required: ["staged"], properties: { staged: { type: "array", minItems: 1, items: unreachable } } }),\n  documentWriteFromNonMutation: ajv.compile({ type: "object", required: ["kind", "effects"], properties: { kind: { enum: ["view", "shell"] }, effects: { type: "array", contains: touches } } }),\n  silentMutation: ajv.compile({ type: "object", required: ["kind", "audience", "outcomes"], properties: { kind: { const: "mutation" }, audience: { const: "agent" }, outcomes: { type: "array", items: settled(silent) } } }),\n  destructiveWithoutDiscard: ajv.compile({\n    type: "object",\n    required: ["destructive", "effects"],\n    properties: { destructive: { const: true }, effects: { type: "array", minItems: 1, items: { type: "object", not: { anyOf: [touches, { type: "object", required: ["userPathWritten"], properties: { userPathWritten: { const: true } } }] } } } },\n  }),\n  composedChildOrphaned: ajv.compile({ type: "object", required: ["effects"], properties: { effects: { type: "array", contains: { type: "object", required: ["orphanedChildren"], properties: { orphanedChildren: { type: "array", minItems: 1 } } } } } }),\n  agentLaneDiverges: ajv.compile({\n    type: "object",\n    required: ["agent", "staged"],\n    properties: { agent: { type: "object" }, staged: { type: "array" } },\n    anyOf: [\n      { oneOf: [{ type: "object", properties: { staged: { type: "array", contains: settled(touches) } } }, { type: "object", properties: { agent: settled(touches) } }] },\n      { type: "object", properties: { agent: refusedOrUnreachable, staged: { type: "array", contains: settled({ type: "object", not: silent }) } } },\n    ],\n  }),\n  argumentAudience: ajv.compile({ type: "object", required: ["audience"], properties: { audience: { not: { const: "input" } } } }),\n  ignoredArgument: ajv.compile({\n    type: "array",\n    minItems: 1,\n    items: {\n      type: "object",\n      anyOf: [\n        { type: "object", required: ["first", "second"], properties: { first: settled({ type: "object" }), second: { const: { $data: "1/first" } } } },\n        { type: "object", required: ["first", "second", "staged", "othersSpecified"], properties: { first: refused, second: { const: { $data: "1/first" } }, staged: { const: { $data: "1/first" } }, othersSpecified: { const: true } } },\n      ],\n    },\n  }),\n};\n\n/** 🧮️ An outcome in the Rust verdict\'s own equality: lanes are a set, a missing orphan list is empty. */\nconst normal = (outcome: Outcome): Outcome => ("settled" in outcome ? { settled: { ...outcome.settled, lanes: [...new Set(outcome.settled.lanes)].sort(), orphanedChildren: outcome.settled.orphanedChildren ?? [] } } : outcome);\n\n/** 🔍️ The findings of one probe, in the Rust verdict\'s order: an unbridged or unreachable verb reports that alone. */\nfunction findings(probe: Probe): Finding[] {\n  const staged = probe.windows.map((window) => normal(window.staged));\n  const outcomes = probe.windows.flatMap((window) => [window.staged, ...window.arguments.flatMap((argument) => [argument.first, argument.second])]).map(normal);\n  const effects = outcomes.flatMap((outcome) => ("settled" in outcome ? [outcome.settled] : []));\n  const view = { kind: probe.kind, audience: probe.audience, destructive: probe.destructive, bridge: probe.bridge, staged, outcomes, effects, ...(probe.agent === null ? {} : { agent: normal(probe.agent) }) };\n  if (!rules.bridged(view)) return rules.unbridged(view) ? [{ finding: "unbridged" }] : [];\n  if (rules.unreachable(view)) return [{ finding: "unreachable" }];\n  const found: Finding[] = (["documentWriteFromNonMutation", "silentMutation", "destructiveWithoutDiscard", "composedChildOrphaned", "agentLaneDiverges"] as const).filter((rule) => rules[rule](view)).map((rule) => ({ finding: rule }));\n  if (!rules.argumentAudience(view)) return found;\n  const names = [...new Set(probe.windows.flatMap((window) => window.arguments.map((argument) => argument.argument)))];\n  for (const name of names) {\n    const rows = probe.windows.flatMap((window) => window.arguments.filter((argument) => argument.argument === name).map((argument) => ({ staged: normal(window.staged), first: normal(argument.first), second: normal(argument.second), othersSpecified: argument.othersSpecified })));\n    if (rules.ignoredArgument(rows)) found.push({ finding: "ignoredArgument", argument: name });\n  }\n  return found;\n}\n\n/** ⚖️ The AJV twin of `artifact_app_laws::declared_verb_findings`: every case of the language-agnostic verdict fixture answered exactly as written. */\nexport function declaredVerbVerdictOracle(): number {\n  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/⚖️declared-verb-verdicts.json", import.meta.url), "utf8")) as Fixture;\n  assert(fixture.cases.length >= 28, "the fixture keeps every rule\'s positive and negative case");\n  for (const { name, probe, findings: expected } of fixture.cases) assert.deepEqual(findings(probe), expected.map((finding) => (finding.argument === undefined ? { finding: finding.finding } : finding)), name);\n  return fixture.cases.length;\n}\n\nif (import.meta.main) console.log(`declared-verb-verdict-oracle cases=${declaredVerbVerdictOracle()}`);\n')
replace("C", SDK_SCRIPT, """import { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } from "../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts";
""", """import { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } from "../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts";
import { declaredVerbVerdictOracle } from "../../🧪️tests/⚖️declared-verb-verdicts/🟦️.ts";
""")
replace("C", SDK_SCRIPT, """    console.log(`completion-rejection-oracle assertions=${completionRejectionOracle(this.repoRoot)}`);
""", """    console.log(`completion-rejection-oracle assertions=${completionRejectionOracle(this.repoRoot)}`);
    console.log(`declared-verb-verdict-oracle cases=${declaredVerbVerdictOracle()}`);
""")

finish(__doc__)
