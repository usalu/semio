#!/usr/bin/env python3
"""🩹️ Anchored, idempotent patch: every `replayShellCommand` gate in `🏛️ShellHost` raises a
localized transient notice beside its console line (ticket 26/09/18, slice S12 §2)."""
import pathlib, sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
SH = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
text = SH.read_text()

IMPORT_ANCHOR = 'import { ArtifactCreationCatalogNotice, ArtifactCreationProgressNotice, reduceArtifactCreationProgressUiV1, type ArtifactCreationProgressOwnerV1, type ArtifactCreationProgressUiStateV1 } from "./🌱️artifact-creation/🟦️.tsx";\n'
IMPORT_LINE = 'import { replayRefusalCodeV1, replayRefusalNoticeTextV1, type ReplayRefusalReasonV1 } from "./📣️replay-refusal/🟦️.ts";\n'

BRANCH_ANCHOR = '''        if ("replayShellCommand" in effect) {
          const { actionId, args } = effect.replayShellCommand;
          const argsRecord = args as Record<string, unknown> | undefined;
'''
BRANCH_NEW = '''        if ("replayShellCommand" in effect) {
          const { actionId, args } = effect.replayShellCommand;
          const argsRecord = args as Record<string, unknown> | undefined;
          const refuseReplay = (reason: ReplayRefusalReasonV1, line: string, ...rest: readonly unknown[]): void => {
            console.warn(`[os-shell] replayShellCommand: ${line}`, ...rest);
            showTransientNoticeRef.current(replayRefusalNoticeTextV1(reason, uiLocaleRef.current), "warning", replayRefusalCodeV1(reason));
          };
'''

GATES = [
    ('console.warn("[os-shell] replayShellCommand: administration requires an exact space id");',
     'refuseReplay("space-required", "administration requires an exact space id");'),
    ('console.warn("[os-shell] replayShellCommand: administration dropped, no signed-in identity");',
     'refuseReplay("sign-in-required", "administration dropped, no signed-in identity");'),
    ('console.warn("[os-shell] replayShellCommand: space artifact creation requires one mounted Space index");',
     'refuseReplay("space-index-required", "space artifact creation requires one mounted Space index");'),
    ('console.warn("[os-shell] replayShellCommand: space artifact creation dropped, no signed-in identity");',
     'refuseReplay("sign-in-required", "space artifact creation dropped, no signed-in identity");'),
    ('console.warn("[os-shell] replayShellCommand: invalid space artifact creation request");',
     'refuseReplay("invalid-request", "invalid space artifact creation request");'),
    ('console.warn("[os-shell] replayShellCommand: unrecognized directory action", actionId);',
     'refuseReplay("invalid-request", "unrecognized directory action", actionId);'),
    ('console.warn("[os-shell] replayShellCommand: directory command dropped, no signed-in identity", actionId);',
     'refuseReplay("sign-in-required", "directory command dropped, no signed-in identity", actionId);'),
    ('console.warn("[os-shell] replayShellCommand: artifact router is not ready", args);',
     'refuseReplay("router-not-ready", "artifact router is not ready", args);'),
    ('console.warn("[os-shell] replayShellCommand: artifact opening rejected", openingError, args);',
     'refuseReplay("open-rejected", "artifact opening rejected", openingError, args);'),
    ('console.warn("[os-shell] replayShellCommand: no shell route for chrome command", actionId);',
     'refuseReplay("unrouted-command", "no shell route for chrome command", actionId);'),
]

changed = 0
if IMPORT_LINE not in text:
    assert text.count(IMPORT_ANCHOR) == 1, "import anchor not unique"
    text = text.replace(IMPORT_ANCHOR, IMPORT_ANCHOR + IMPORT_LINE)
    changed += 1
if "const refuseReplay =" not in text:
    assert text.count(BRANCH_ANCHOR) == 1, "branch anchor not unique"
    text = text.replace(BRANCH_ANCHOR, BRANCH_NEW)
    changed += 1
for old, new in GATES:
    if new in text:
        continue
    assert text.count(old) == 1, f"gate anchor not unique: {old[:70]}"
    text = text.replace(old, new)
    changed += 1

SH.write_text(text)
print(f"edits applied: {changed}")
print("refuseReplay call sites:", text.count("refuseReplay("))
