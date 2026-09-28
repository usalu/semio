# -*- coding: utf-8 -*-
"""S18 §14c: document actor recovery re-open (design approved by the coordinator 00:0x). A mounted browser actor lost without
anyone asking (an unconfirmed action invocation today; C12 wires the inbound-frame child fault into the same entry) no longer
leaves the document dead: `requestDocumentActorRecoveryV1` steps the `🚑️actor-recovery` policy, puts in-flight batches back
at the outbox front, retires the mounted child with the cause `actor-lost` (the Shell shows "reopening", not "fresh
authoritative restore"), ends the socket with the new close row `actorLost` and the reconnect loop reopens at once; past the
bound it posts the typed fault `document-actor.recovery-exhausted` and activates no further actor for the document. The
unconfirmed input is refused `not-applied` (en/de, retryable, never replayed). Idempotent."""
import json
import pathlib

OS = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os")
WORKER = OS / "🔨️modules/🏪️store/👷️worker/🟦️.ts"
CLOSE = OS / "🔨️modules/📇️directory/🔌️client/🚪️socket-close/🔣️.json"
REQUESTS = OS / "🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📮️requests/🟦️.ts"
LEDGER = OS / "🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts"
BOOT = OS / "🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx"
BOOT_FIXTURE = OS / "🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🪪️host-bootstrap/🔣️.json"
QUICK = OS / "🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts"
ENVELOPE_LAW = OS / "🧪️tests/🧪️backbone-envelope-io/🟦️.ts"
OS_CONFIG = OS / "🧪️tests/🎚️config/🟦️.ts"

NOT_APPLIED_EN = "Not applied — the document is reopening. Try again."
NOT_APPLIED_DE = "Nicht angewendet — das Dokument wird neu geöffnet. Bitte erneut versuchen."
REOPEN_EN = "The document is reopening from its last confirmed state; your last change was not applied."
EXHAUSTED_EN = "This document could not be reopened after repeated failures. Close it and open it again."
EXHAUSTED_DE = "Dieses Dokument ließ sich nach wiederholten Fehlern nicht neu öffnen. Schließe es und öffne es erneut."
REOPEN_DE = "Das Dokument wird aus seinem letzten bestätigten Stand neu geöffnet; deine letzte Änderung wurde nicht angewendet."

EDITS = [
    # worker: state
    (WORKER, "  artifactRebootstrapRequired: boolean;\n  artifactBootstrapProgress: ArtifactBootstrapProgress[];\n",
     "  artifactRebootstrapRequired: boolean;\n  /** 🚑️ The document's actor-recovery memory (`🚑️actor-recovery`) and whether the socket closing now is a recovery close that\n   * reconnects at once. */\n  actorRecovery: DocumentActorRecoveryMemoryV1;\n  actorRecoveryRequested: boolean;\n  artifactBootstrapProgress: ArtifactBootstrapProgress[];\n"),
    (WORKER, "    artifactRebootstrapRequired: false,\n    artifactBootstrapProgress: [],\n",
     "    artifactRebootstrapRequired: false,\n    actorRecovery: DOCUMENT_ACTOR_RECOVERY_FRESH_V1,\n    actorRecoveryRequested: false,\n    artifactBootstrapProgress: [],\n"),
    # worker: retirement cause + recovery entry
    (WORKER, "function retireMountedDocumentChild(state: ArtifactState): void {\n  const mounted = state.verifiedColdPair !== null || state.browserActorReservation !== null;\n  dropDocumentExecutionTargetLease(state);\n  if (!mounted || state.closed || state.docAbort.signal.aborted || state.artifactRebootstrapRequired) return;\n  const scope = artifactScope(state);\n  post({ kind: \"artifact-rebootstrap-required\", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), message: \"rebootstrap-required\", retryable: true });\n}\n",
     """function retireMountedDocumentChild(state: ArtifactState, cause: "rebootstrap-required" | "actor-lost" = "rebootstrap-required"): void {
  const mounted = state.verifiedColdPair !== null || state.browserActorReservation !== null;
  dropDocumentExecutionTargetLease(state);
  if (!mounted || state.closed || state.docAbort.signal.aborted || state.artifactRebootstrapRequired) return;
  const scope = artifactScope(state);
  post({ kind: "artifact-rebootstrap-required", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), message: cause, retryable: true });
}

/** 🚑️ A mounted browser actor was lost without anyone asking — its action's outcome unconfirmed, its child faulted on an inbound
 * hub frame, a render turn failed ({@link DOCUMENT_ACTOR_RECOVERY_V1}). The in-flight batches go back to the outbox front (their
 * outcome is unknown; the hub settles what it already holds), the mounted child retires as `actor-lost`, and the socket ends
 * with `actorLost` so the reconnect loop reopens AT ONCE — a new plan, lease and actor seeded from the hub's checkpoint pair and
 * tail, the outbox resent in order. Past the policy's bound the document gets the typed fault instead and no further actor. */
function requestDocumentActorRecoveryV1(state: ArtifactState, cause: DocumentActorLossCauseV1): void {
  if (state.closed || state.docAbort.signal.aborted) return;
  const step = documentActorRecoveryStepV1(state.actorRecovery, { kind: "lost", cause, atMs: Date.now() });
  state.actorRecovery = step.memory;
  requeuePendingBatches(state);
  if (step.decision === "exhausted") {
    dropDocumentExecutionTargetLease(state);
    const scope = artifactScope(state);
    post({ kind: "artifact-bootstrap-failed", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), code: "recovery-exhausted", message: `${DOCUMENT_ACTOR_RECOVERY_V1.exhaustedFault} (${cause})`, retryable: false });
    return;
  }
  state.actorRecoveryRequested = true;
  retireMountedDocumentChild(state, "actor-lost");
  const socket = state.socket;
  if (socket !== null && socket.readyState === WebSocket.OPEN) closeHubSocketV1(socket, "actorLost");
}
"""),
    # worker: unconfirmed invocation triggers the recovery
    (WORKER, "      const explicitRefusal = error instanceof BrowserActorGuestRefusalV1;\n      if (invoked && !explicitRefusal) this.close();\n",
     "      const explicitRefusal = error instanceof BrowserActorGuestRefusalV1;\n      if (invoked && !explicitRefusal) {\n        this.close();\n        requestDocumentActorRecoveryV1(this.state, \"action-unconfirmed\");\n      }\n"),
    # worker: an applied action and a mounted actor feed the policy
    (WORKER, "        return browserActorActionDisposition(request, \"guest-applied\", mutationCount, parseBrowserActorHostEffectBytesV1(publication.hostEffects), undefined, parseBrowserActorHistoryPatchBytesV1(publication.historyPatches));",
     "        this.state.actorRecovery = documentActorRecoveryStepV1(this.state.actorRecovery, { kind: \"applied\", atMs: Date.now() }).memory;\n        return browserActorActionDisposition(request, \"guest-applied\", mutationCount, parseBrowserActorHostEffectBytesV1(publication.hostEffects), undefined, parseBrowserActorHistoryPatchBytesV1(publication.historyPatches));"),
    (WORKER, "    await owner.activate(socket);\n",
     "    await owner.activate(socket);\n    state.actorRecovery = documentActorRecoveryStepV1(state.actorRecovery, { kind: \"mounted\", atMs: Date.now() }).memory;\n"),
    # worker: an exhausted document activates no further actor
    (WORKER, "    if (documentBrowserActorLease(state).fields().browserActor.kind === \"none\") return;\n    lease.assertBrowserActorDescribeCapacity();\n",
     "    if (documentBrowserActorLease(state).fields().browserActor.kind === \"none\") return;\n    if (state.actorRecovery.exhausted) return;\n    lease.assertBrowserActorDescribeCapacity();\n"),
    # worker: a recovery close reconnects at once
    (WORKER, "      if (sustainedHealthReached || state.artifactRebootstrapRequired) {\n",
     "      if (sustainedHealthReached || state.artifactRebootstrapRequired || state.actorRecoveryRequested) {\n        state.actorRecoveryRequested = false;\n"),
    # close row
    (CLOSE, '    "actorMismatch": { "code": 4008, "rfc6455": 1008, "reason": "socket actor mismatch" }\n',
     '    "actorMismatch": { "code": 4008, "rfc6455": 1008, "reason": "socket actor mismatch" },\n    "actorLost": { "code": 4008, "rfc6455": 1008, "reason": "browser actor lost" }\n'),
    # mailbox: unconfirmed -> not-applied
    (REQUESTS, 'export type BrowserActorActionRefusalReasonV1 = "queue-full" | "owner-mismatch" | "dispatch-failed";',
     'export type BrowserActorActionRefusalReasonV1 = "queue-full" | "owner-mismatch" | "dispatch-failed" | "not-applied";'),
    (REQUESTS, '  if (/mailbox closed|completion unconfirmed|action-state-unconfirmed|action-refused|action-command-ingress-/u.test(message)) return "dispatch-failed";',
     '  if (/action-state-unconfirmed/u.test(message)) return "not-applied";\n  if (/mailbox closed|completion unconfirmed|action-refused|action-command-ingress-/u.test(message)) return "dispatch-failed";'),
    (ENVELOPE_LAW, '      expect(browserActorActionRefusalReasonV1(new Error("action-state-unconfirmed"))).toBe("dispatch-failed");',
     '      expect(browserActorActionRefusalReasonV1(new Error("action-state-unconfirmed"))).toBe("not-applied");'),
    # input ledger: the not-applied reason
    (LEDGER, '  | "mutation-rejected"\n  | "dispatch-failed";', '  | "mutation-rejected"\n  | "dispatch-failed"\n  | "not-applied";'),
    (LEDGER, '  "mutation-rejected": false,\n  "dispatch-failed": true,\n};\n\n/** 🔇️', '  "mutation-rejected": false,\n  "dispatch-failed": true,\n  "not-applied": true,\n};\n\n/** 🔇️'),
    (LEDGER, '  "mutation-rejected": true,\n  "dispatch-failed": true,\n};\n\nexport function inputAppliedV1', '  "mutation-rejected": true,\n  "dispatch-failed": true,\n  "not-applied": true,\n};\n\nexport function inputAppliedV1'),
    (LEDGER, '    "mutation-rejected": 0,\n    "dispatch-failed": 0,\n  };', '    "mutation-rejected": 0,\n    "dispatch-failed": 0,\n    "not-applied": 0,\n  };'),
    (LEDGER, '  "dispatch-failed": { en: "The input could not be delivered.", de: "Die Eingabe konnte nicht zugestellt werden." },\n};',
     f'  "dispatch-failed": {{ en: "The input could not be delivered.", de: "Die Eingabe konnte nicht zugestellt werden." }},\n  "not-applied": {{ en: "{NOT_APPLIED_EN}", de: "{NOT_APPLIED_DE}" }},\n}};'),
    # bootstrap notice: reopen copy
    (BOOT, '    rebootstrap: "The server requires a fresh authoritative restore. Stale document UI was discarded while reconnecting.",\n',
     f'    rebootstrap: "The server requires a fresh authoritative restore. Stale document UI was discarded while reconnecting.",\n    reopen: "{REOPEN_EN}",\n'),
    (BOOT, '    rebootstrap: "Der Server verlangt eine neue autoritative Wiederherstellung. Veraltete Dokumentansichten wurden beim Neuverbinden verworfen.",\n',
     f'    rebootstrap: "Der Server verlangt eine neue autoritative Wiederherstellung. Veraltete Dokumentansichten wurden beim Neuverbinden verworfen.",\n    reopen: "{REOPEN_DE}",\n'),
    (BOOT, '  const text = status.kind === "artifact-rebootstrap-required" ? copy.rebootstrap : `${copy.failed}: ${status.message}`;',
     '  const text = status.kind === "artifact-rebootstrap-required" ? (status.message === "actor-lost" ? copy.reopen : copy.rebootstrap) : status.code === "recovery-exhausted" ? copy.exhausted : `${copy.failed}: ${status.message}`;'),
    # quick law: the reopen notice in both languages
    (QUICK, '    expect(view.getByRole("alert").textContent).toContain(hostBootstrapFixture.bootstrap.expected.rebootstrapEn);\n',
     '    expect(view.getByRole("alert").textContent).toContain(hostBootstrapFixture.bootstrap.expected.rebootstrapEn);\n'
     '    for (const locale of ["en", "de"] as const) {\n'
     '      view.rerender(React.createElement(BootstrapStatusNotice, { status: hostBootstrapFixture.bootstrap.reopen as Extract<BootstrapUiStatus, { kind: "artifact-rebootstrap-required" }>, locale, onCancel: () => {} }));\n'
     '      expect(view.getByRole("alert").textContent).toBe(hostBootstrapFixture.bootstrap.expected[locale === "en" ? "reopenEn" : "reopenDe"]);\n'
     '    }\n'),
    # typed exhausted fault code (schema-first) and its notice
    (OS / "🔨️modules/📺️renderer/🧬️schema/🔣️.json", '            "invalid-bootstrap",\n            "transport-failure"\n          ]', '            "invalid-bootstrap",\n            "transport-failure",\n            "recovery-exhausted"\n          ]'),
    (OS / "🔨️modules/📺️renderer/🧬️schema/🟦️.ts", 'readonly code: "cancelled" | "deadline-exceeded" | "invalid-bootstrap" | "transport-failure"; readonly message: string; readonly retryable: boolean }', 'readonly code: "cancelled" | "deadline-exceeded" | "invalid-bootstrap" | "transport-failure" | "recovery-exhausted"; readonly message: string; readonly retryable: boolean }'),
    (OS / "🟦️.ts", '      readonly code: "cancelled" | "deadline-exceeded" | "invalid-bootstrap" | "transport-failure";\n', '      readonly code: "cancelled" | "deadline-exceeded" | "invalid-bootstrap" | "transport-failure" | "recovery-exhausted";\n'),
    (BOOT, '    failed: "Document restore failed",\n', f'    failed: "Document restore failed",\n    exhausted: "{EXHAUSTED_EN}",\n'),
    (BOOT, '    failed: "Dokumentwiederherstellung fehlgeschlagen",\n', f'    failed: "Dokumentwiederherstellung fehlgeschlagen",\n    exhausted: "{EXHAUSTED_DE}",\n'),
]


def main() -> None:
    texts: dict[pathlib.Path, str] = {}
    for path, old, new in EDITS:
        text = texts.get(path) or path.read_text(encoding="utf-8")
        if new not in text:
            assert text.count(old) == 1, (path.name, old[:100])
            text = text.replace(old, new)
        texts[path] = text
    worker = texts[WORKER]
    recovery_import = 'import { DOCUMENT_ACTOR_RECOVERY_FRESH_V1, DOCUMENT_ACTOR_RECOVERY_V1, documentActorRecoveryStepV1, type DocumentActorLossCauseV1, type DocumentActorRecoveryMemoryV1 } from "./🚑️actor-recovery/🟦️.ts";\n'
    if recovery_import not in worker:
        anchor = 'import { browserActorChildCapacity, reserveBrowserActorChild, type BrowserActorChildValue } from "../../🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts";\n'
        assert worker.count(anchor) == 1
        worker = worker.replace(anchor, anchor + recovery_import)
    texts[WORKER] = worker
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
    raw = BOOT_FIXTURE.read_text(encoding="utf-8")
    if '"reopen"' not in raw:
        old = '    "rebootstrap": {\n      "kind": "artifact-rebootstrap-required",\n      "documentId": "document-a",\n      "message": "rebootstrap-required",\n      "retryable": true\n    },\n'
        assert raw.count(old) == 1
        raw = raw.replace(old, old + '    "reopen": {\n      "kind": "artifact-rebootstrap-required",\n      "documentId": "document-a",\n      "message": "actor-lost",\n      "retryable": true\n    },\n')
        old2 = '      "rebootstrapEn": "The server requires a fresh authoritative restore."\n'
        assert raw.count(old2) == 1
        raw = raw.replace(old2, f'      "rebootstrapEn": "The server requires a fresh authoritative restore.",\n      "reopenEn": "{REOPEN_EN}",\n      "reopenDe": "{REOPEN_DE}"\n')
        json.loads(raw)
        BOOT_FIXTURE.write_text(raw, encoding="utf-8")
    config = OS_CONFIG.read_text(encoding="utf-8")
    law = '"../../🧪️tests/🚑️actor-recovery/🟦️.ts"'
    if law not in config:
        anchor = '"../../🧪️tests/🔁️execution-target-retry/🟦️.ts"'
        assert config.count(anchor) >= 1
        config = config.replace(anchor, anchor + ", " + law, 1)
        OS_CONFIG.write_text(config, encoding="utf-8")
    print("ok")


main()
