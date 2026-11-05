# 🚪 Close Lifecycle Audit

Read-only current-source audit. No expensive commands or tests run.

H = 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost.

| Priority | Path under H | Current hazard | Required regression |
| --- | --- | --- | --- |
| P0 | 🗄️shell-uri-documents/🟠️close-document/🟦️.ts:151 | requestCloseDocument captures entry but saved reads latest runtimeKey status; final closeDocument(runtimeKey) omits generation. | Replace session while flush/confirm waits: no replacement close or afterClose effects. |
| P0 | same file:181 | detach forgets local folder before guard; Keep loses remembered binding. | Keep leaves binding intact; accepted close forgets once. |
| P0 | 🖼️canvas-footer/🔸️canvas/🟦️.tsx:91 | Dock onWindowClose removes last spawned panel and destroys app with no save guard or attachment retirement. | Failed/pending save then close: Keep retains panel/app/owner. |
| P1 | close-document:151 | Concurrent requests overwrite one closePrompt; stale completion clears latest prompt. | Concurrent closes must own prompt and settle all retained promises. |
| P1 | close-document:182 | Delayed detach afterClose clears newer sync URI/card. | Switch attachment during prompt: old completion preserves new attachment. |
| P1 | 🛫️spawned-session/🔸️update-space-panel/🟦️.ts:91 | App switch closes all owned document sessions immediately then destroys app. | Keep aborts switch before predecessor removal/successor commit. |
| P1 | 🚀️plugin-lifecycle/🔄️reload-and-extension-callbacks/🟦️.ts:62/:129 | Reload/uninstall destroy live instances directly. | Guard dirty documents before destroy. |

Low-level closeDocument already supports clientInstanceId and checks it first. Request close must check ownership before saved probes/prompts/final retirement/afterClose, not merely pass generation to final close.

Shell unmount in 🎮️focus-and-input/⌨️registers-and-context-menu/🟦️.tsx:321 force-closes sessions and terminates worker. Admission must happen before route/unmount commits; asynchronous unmount guard is insufficient. Bootstrap/execution-target cancel in H/🟦️.tsx:218/:225 closes by runtimeKey only; confirm pre-ready-only behavior or pin generation. Error/backbone detach and failed-opening cleanup mostly already check entry identity and pass generation.

Existing test base: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests.

- 💾️saved-state/🧮️derivation.ts tests pure guard/await.
- 💾️saved-state/🧫️fixtures/🔣️.json is neutral status/close evidence and has retirement during flush/confirm.
- 💾️saved-state/🟦️.tsx tests modal/buttons/marker/beforeunload only, not useCloseDocumentState integration.
- 🧫️fixtures/📎️local-folder-bindings/🔣️.json covers remembered bindings.
- 🔀️surface-switch/🟦️.ts and ShellHost/🧪️tests/🧫️fixtures/🔀️surface-switch/🔣️.json cover quiesce/retirement, not acknowledgment.
- ♻️frame-close and 🏁️ui-close-and-job-admission cover resource retirement, not data retention.

Test production requestCloseDocument/onWindowClose or extract a production admission interface, with neutral fixture validated independently by Ajv. A model mirroring desired behavior alone leaves bypasses untested.

## Review of Current Guarded Canvas Wiring

Production guardDocumentCloseV1 and all current derivation-test literals supply the new required isCurrent callback. requestCloseDocument, openDocumentSessionsRef and updateSpacePanelFrom forwarding through canvas-footer/plugin-filter/useCanvas appears consistent by source inspection; no typechecker run.

Remaining source concerns sent to root:

- Old close prompt cleanup on stale entry with no successor request (root already addressing).
- Host canvas captures spawned id/instance, awaits document guard, then filters latest panel by spawned id only. A replacement using same id must survive stale close; verify latest instance authority before side effects.
- Concurrent closes of different windows in one spawned app each compute another remaining window before awaiting. Both may skip last-window document guard, then both disappear. Serialize per spawned instance or re-evaluate close ownership at commitment.
- Panel removal/app destruction precedes await noteShellCommand. A rejected command note or destroyApp means UI admission retains dock layout after host already removed/destroyed owner. Record note before cleanup, or treat admitted cleanup reporting separately from admission failure.

No root files edited or tests run during this review.
