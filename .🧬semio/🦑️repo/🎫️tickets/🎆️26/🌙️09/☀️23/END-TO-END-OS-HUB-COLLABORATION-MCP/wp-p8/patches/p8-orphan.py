#!/usr/bin/env python3
"""🪆️ P8 post-publish patch set, HELD: `composedChildOrphaned` — a verb that leaves the parent naming a composed child no
store holds. The probe records every declared child that is unheld after the verb and was not before; the verdict fixture
gains the case; the Rust verdict test and the AJV twin read `orphanedChildren`.

Held because it is true beyond flow: measured in the clone, reasoning wires `addNode`/`addRelationship` orphan their
content-addressed `wires-content-*` child, and 12 plugins mint such children from parent edits (source: dag, writer,
sequence, animate, playbook, imperative, trinity jack, raster, note, reasoning, flow, cad). Land it together with the
class fix (derivable children follow their coordinate), never before — it would turn the reasoning law red.
Usage: p8-orphan.py --dry-run | --write"""
import json

from p8_patch import ROOT, create, finish, replace, text

SDK = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
VERDICT_TEST = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🦀️.rs"
VERDICT_FIXTURE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/⚖️declared-verb-verdicts.json"
VERDICT_TWIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts"
SDK_SCRIPT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts"

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
ORPHAN_RULE_OLD = "is not refused where the shell acts;"
ORPHAN_RULE_NEW = "is not refused where the shell acts; no verb leaves the parent naming a composed child no store holds (`orphanedChildren`);"
if document["why"].count(ORPHAN_RULE_OLD) != 1:
    raise SystemExit("B: the verdict fixture's why no longer names the agent rule as expected")
document["why"] = document["why"].replace(ORPHAN_RULE_OLD, ORPHAN_RULE_NEW)
replace("B", VERDICT_FIXTURE, fixture, json.dumps(document, indent=2, ensure_ascii=False) + "\n")
replace("B", VERDICT_TWIN, '; fingerprint: number }>;', '; fingerprint: number; orphanedChildren?: string[] }>;')
replace("B", VERDICT_TWIN, '"destructiveWithoutDiscard", "agentLaneDiverges"', '"destructiveWithoutDiscard", "composedChildOrphaned", "agentLaneDiverges"')
replace("B", VERDICT_TWIN, "/** 🧮️ An outcome in the Rust verdict's own equality: lanes are a set. */", "/** 🧮️ An outcome in the Rust verdict's own equality: lanes are a set, a missing orphan list is empty. */")
replace("B", VERDICT_TWIN, 'lanes: [...new Set(outcome.settled.lanes)].sort() } }', 'lanes: [...new Set(outcome.settled.lanes)].sort(), orphanedChildren: outcome.settled.orphanedChildren ?? [] } }')
replace("B", VERDICT_TWIN, '  agentLaneDiverges: ajv.compile({', '  composedChildOrphaned: ajv.compile({ type: "object", required: ["effects"], properties: { effects: { type: "array", contains: { type: "object", required: ["orphanedChildren"], properties: { orphanedChildren: { type: "array", minItems: 1 } } } } } }),\n  agentLaneDiverges: ajv.compile({')

finish(__doc__)
