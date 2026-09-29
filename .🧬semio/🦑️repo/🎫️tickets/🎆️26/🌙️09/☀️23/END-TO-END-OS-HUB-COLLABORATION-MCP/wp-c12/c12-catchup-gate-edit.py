"""⏳️ C12 14c: an input that reaches a document still catching up with the hub is refused TYPED (`action-catching-up` → ledger
reason `catching-up`, en/de notice, retryable) — never applied to a document older than the hub's. Before, the same gate threw
`action-owner-mismatch` ("The document owner changed"), which named the wrong cause. The catch-up facts are one pure predicate
(`documentCatchingUpV1`) with a law; the mailbox and the shell map the new code; the TextEditor host already drops a refused
keystroke and resyncs to the guest's text. Idempotent; `--dry-run`."""
import sys

OS = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/"
WORKER = OS + "🔨️modules/🏪️store/👷️worker/🟦️.ts"
LEDGER = OS + "🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts"
MAILBOX = OS + "🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📮️requests/🟦️.ts"
SHELL = OS + "🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
WORKER_LAW = OS + "🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts"
MAILBOX_LAW = OS + "🧪️tests/🧪️backbone-envelope-io/🟦️.ts"

PREDICATE = '''/** ⏳️ Whether a mounted document is still catching up with the hub: its actor is not bound to the document port or the retained
 * catch-up tail has not drained into it, an artifact bootstrap or rebuild is in flight, the tail the hub named in its `Welcome` is
 * not reached yet, or the cold pair is not applied. An input then would apply to a document older than the hub's — a whole-value
 * verb (typing's `textEdit`) would overwrite what the tail brings — so it is refused typed (`action-catching-up`), never applied. */
export function documentCatchingUpV1(facts: Readonly<{ bound: boolean; backboneReady: boolean; bootstrap: boolean; rebootstrap: boolean; tail: boolean; coldApplied: boolean; coldTransfer: boolean }>): boolean {
  return !facts.bound || !facts.backboneReady || facts.bootstrap || facts.rebootstrap || facts.tail || !facts.coldApplied || facts.coldTransfer;
}

'''
DEPS_OLD = "documentRuntimeKeyForConfig, documentRuntimeKeyV1, driveInferencePort, newArtifactState,"
DEPS_NEW = "documentRuntimeKeyForConfig, documentRuntimeKeyV1, driveInferencePort, documentCatchingUpV1, newArtifactState,"
HUNKS = [
    (WORKER, 1,
     "          !painted ||\n          request.actionSequence <= this.lastActionSequence ||\n          this.documentBinding === null ||\n          !this.documentBackboneReady ||\n"
     "          this.state.artifactBootstrap !== null ||\n          this.state.artifactRebootstrapRequired ||\n          this.state.requiredTailFrontier !== null ||\n"
     "          this.coldApplied === null ||\n          this.coldTransfer !== null\n        ) throw new Error(\"action-owner-mismatch\");\n",
     "          !painted ||\n          request.actionSequence <= this.lastActionSequence\n        ) throw new Error(\"action-owner-mismatch\");\n"
     "        if (\n          documentCatchingUpV1({\n            bound: this.documentBinding !== null,\n            backboneReady: this.documentBackboneReady,\n"
     "            bootstrap: this.state.artifactBootstrap !== null,\n            rebootstrap: this.state.artifactRebootstrapRequired,\n            tail: this.state.requiredTailFrontier !== null,\n"
     "            coldApplied: this.coldApplied !== null,\n            coldTransfer: this.coldTransfer !== null,\n          })\n        ) throw new Error(\"action-catching-up\");\n"),
    (WORKER, 1,
     "error instanceof Error && /^(action-owner-mismatch|action-child-unavailable)$/u.test(error.message) ? error.message",
     "error instanceof Error && /^(action-owner-mismatch|action-catching-up|action-child-unavailable)$/u.test(error.message) ? error.message"),
    (WORKER, 1, "function documentBackboneAdmissionReady(state: ArtifactState): boolean {\n", PREDICATE + "function documentBackboneAdmissionReady(state: ArtifactState): boolean {\n"),
    (WORKER, 1, "  readonly newArtifactState: typeof newArtifactState;\n", "  readonly newArtifactState: typeof newArtifactState;\n  readonly documentCatchingUpV1: typeof documentCatchingUpV1;\n"),
    (WORKER, 2, DEPS_OLD, DEPS_NEW),
    (LEDGER, 1, '  | "dispatch-failed"\n  | "not-applied";\n', '  | "dispatch-failed"\n  | "not-applied"\n  | "catching-up";\n'),
    (LEDGER, 2, '  "dispatch-failed": true,\n  "not-applied": true,\n};\n', '  "dispatch-failed": true,\n  "not-applied": true,\n  "catching-up": true,\n};\n'),
    (LEDGER, 1, '    "dispatch-failed": 0,\n    "not-applied": 0,\n', '    "dispatch-failed": 0,\n    "not-applied": 0,\n    "catching-up": 0,\n'),
    (LEDGER, 1,
     '  "not-applied": { en: "Not applied — the document is reopening. Try again.", de: "Nicht angewendet — das Dokument wird neu geöffnet. Bitte erneut versuchen." },\n',
     '  "not-applied": { en: "Not applied — the document is reopening. Try again.", de: "Nicht angewendet — das Dokument wird neu geöffnet. Bitte erneut versuchen." },\n'
     '  "catching-up": { en: "Not applied — the document is still catching up with the hub. Try again in a moment.", de: "Nicht angewendet — das Dokument gleicht sich noch mit dem Hub ab. Bitte gleich erneut versuchen." },\n'),
    (MAILBOX, 1, '"dispatch-failed" | "not-applied";', '"dispatch-failed" | "not-applied" | "catching-up";'),
    (MAILBOX, 1, '  if (/action-owner-mismatch|action-busy|owner retired/u.test(message)) return "owner-mismatch";\n',
     '  if (/action-catching-up/u.test(message)) return "catching-up";\n  if (/action-owner-mismatch|action-busy|owner retired/u.test(message)) return "owner-mismatch";\n'),
    (SHELL, 1, '        if (/action-owner-mismatch|owner retired|ambiguous document owner/u.test(text)) return { reason: "owner-mismatch", detail: text };\n',
     '        if (/action-catching-up/u.test(text)) return { reason: "catching-up", detail: text };\n        if (/action-owner-mismatch|owner retired|ambiguous document owner/u.test(text)) return { reason: "owner-mismatch", detail: text };\n'),
    (MAILBOX_LAW, 1, '      expect(browserActorActionRefusalReasonV1(new Error("action-state-unconfirmed"))).toBe("not-applied");\n',
     '      expect(browserActorActionRefusalReasonV1(new Error("action-state-unconfirmed"))).toBe("not-applied");\n      expect(browserActorActionRefusalReasonV1(new Error("action-catching-up"))).toBe("catching-up");\n'),
    (WORKER_LAW, 1, '    it("recovers a child that refuses an inbound hub frame: actor-lost reopen, frontier kept, never a malformed-frame rebuild", async () => {\n',
     '    it("refuses input while the document catches up with the hub: every catch-up fact alone gates, only the caught-up actor applies", () => {\n'
     '      const catchingUp = dependencies.documentCatchingUpV1;\n'
     '      const caughtUp = { bound: true, backboneReady: true, bootstrap: false, rebootstrap: false, tail: false, coldApplied: true, coldTransfer: false } as const;\n'
     '      expect(catchingUp(caughtUp), "a bound actor that applied its cold pair and the whole tail takes input").toBe(false);\n'
     '      for (const [fact, value] of [["bound", false], ["backboneReady", false], ["bootstrap", true], ["rebootstrap", true], ["tail", true], ["coldApplied", false], ["coldTransfer", true]] as const) {\n'
     '        expect(catchingUp({ ...caughtUp, [fact]: value }), `${fact} alone keeps the document catching up`).toBe(true);\n'
     '      }\n'
     '    });\n\n'
     '    it("recovers a child that refuses an inbound hub frame: actor-lost reopen, frontier kept, never a malformed-frame rebuild", async () => {\n'),
]
dry = "--dry-run" in sys.argv
texts, problems, plan = {}, [], []
for path, expected, old, new in HUNKS:
    text = texts.get(path) or open(path, encoding="utf-8").read()
    if new in text and (expected == 1 or text.count(new) == expected):
        plan.append(f"present {path.rsplit('/', 2)[-2]}")
        texts[path] = text
        continue
    if text.count(old) != expected:
        problems.append((path.rsplit("/", 2)[-2], old[:60], text.count(old)))
        continue
    texts[path] = text.replace(old, new)
    plan.append(f"planned {path.rsplit('/', 2)[-2]}")
print("\n".join(plan))
print(f"{len(problems)} problems {problems}")
if problems:
    sys.exit(1)
if not dry:
    for path, text in texts.items():
        open(path, "w", encoding="utf-8").write(text)
    print("written")
