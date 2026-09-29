#!/usr/bin/env python3
"""⚖️ S19 one-off codemod (test-only, rule 22): flow's declared-verb law after P9's agent-lane preview landed (T1).
Measured 2026-09-29 (`s14-s19-logs/flow-cohort-1.txt`): `evaluate` acts on the shell lane, while the agent lane now SETTLES
it silently — `preview_addressed_action` drives the host-only job but neither carries nor refuses the host effects the shell
lane requests, and `declared_verb_findings` never counts a silent agent lane against an acting shell lane. Not a carrier,
not a regression of `evaluate`: a fail-open gap. The pin drops `evaluate` and asserts both lanes explicitly (shell acts,
agent settles silent), so the fail-closed SDK fix (train T6) turns this law red and restores `evaluate` to the list.
Idempotent. usage: s19-flow-evaluate-pin.py <root>"""
import os
import sys

root = sys.argv[1]
LAW = os.path.join(root, "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/⚖️declared-verbs/🦀️.rs")
text = open(LAW, encoding="utf-8").read()
EDITS = [
    ("use semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences};",
     "use semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences, DeclaredVerbOutcome};"),
    ("""/// 26/09/23 `wp-p8.md` § routed) — `evaluate`, whose host-only job the agent lane cannot preview, and
/// `focusSelection`, which frames the camera of the main window the agent address does not name. The list is pinned so
/// a carrier that lands turns this law red until the pin is removed.""",
     """/// 26/09/23 `wp-p8.md` § routed) — and `focusSelection`, which frames the camera of the main window the agent address
/// does not name. The list is pinned so a carrier that lands turns this law red until the pin is removed. `evaluate` left
/// it when the agent lane began previewing plugin-owned jobs (P9, 2026-09-28): the preview settles its host-only job but
/// neither carries nor refuses the host effects the shell lane requests, and a silent agent lane does not count as a
/// divergence — pinned below (shell acts, agent settles silent) until the fail-closed preview restores it to the list."""),
    ("""        ["addWidget", "removeWidget", "duplicateWidget", "disconnect", "connectMediaPorts", "moveMediaNode", "reorganize", "patchFlowWidgets", "renameFlowWidget", "setActiveExample", "evaluate", "focusSelection"],
        "agent-lane divergences"
    );
}""",
     """        ["addWidget", "removeWidget", "duplicateWidget", "disconnect", "connectMediaPorts", "moveMediaNode", "reorganize", "patchFlowWidgets", "renameFlowWidget", "setActiveExample", "focusSelection"],
        "agent-lane divergences"
    );
    let evaluate = probes.iter().find(|probe| probe.verb == "evaluate").expect("evaluate is a declared verb");
    assert!(evaluate.windows.iter().all(|window| matches!(&window.staged, DeclaredVerbOutcome::Settled(effect) if !effect.is_silent())), "evaluate acts on the shell lane: {:?}", evaluate.windows.iter().map(|window| &window.staged).collect::<Vec<_>>());
    assert!(matches!(&evaluate.agent, Some(DeclaredVerbOutcome::Settled(effect)) if effect.is_silent()), "the agent lane settles evaluate without its host effects: {:?}", evaluate.agent);
}"""),
]
STALE = [
    ("""/// does not name. The list is pinned so a carrier that lands turns this law red until the pin is removed. `evaluate` left
/// it when the agent lane began previewing plugin-owned jobs (P9, 2026-09-28): both of its lanes are asserted to act, so
/// the pin can never be satisfied by a verb gone silent on both.""", EDITS[1][1].split("\n", 1)[1]),
    ("""    assert!(matches!(&evaluate.agent, Some(DeclaredVerbOutcome::Settled(effect)) if !effect.is_silent()), "the agent lane runs evaluate's host-only job: {:?}", evaluate.agent);""", """    assert!(matches!(&evaluate.agent, Some(DeclaredVerbOutcome::Settled(effect)) if effect.is_silent()), "the agent lane settles evaluate without its host effects: {:?}", evaluate.agent);"""),
]
for old, new in STALE:
    text = text.replace(old, new)
for old, new in EDITS:
    if new in text:
        continue
    assert text.count(old) == 1, f"anchor x{text.count(old)}: {old[:80]!r}"
    text = text.replace(old, new)
open(LAW, "w", encoding="utf-8").write(text)
print("ok")
