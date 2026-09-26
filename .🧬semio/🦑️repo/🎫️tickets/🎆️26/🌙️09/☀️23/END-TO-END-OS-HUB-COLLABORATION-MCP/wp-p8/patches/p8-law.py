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
Usage: p8-law.py --dry-run | --write"""
import json

from p8_patch import ROOT, finish, replace, text

SDK = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
VERDICT_TEST = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🦀️.rs"
VERDICT_FIXTURE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/⚖️declared-verb-verdicts.json"

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
replace("B", VERDICT_FIXTURE, fixture, json.dumps(document, indent=2, ensure_ascii=False) + "\n")

finish(__doc__)
