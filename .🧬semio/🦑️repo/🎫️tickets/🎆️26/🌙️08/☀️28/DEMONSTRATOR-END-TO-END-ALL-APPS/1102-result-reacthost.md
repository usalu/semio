STATUS: GREEN. 13 React host suites pass in vitest: 9 original (reacthost-2/4), action-window-scope + window-frame-render-isolation (reacthost-8 rc 0, 16 tests), window-action-context + root-animation-storm (reacthost-7). No production fault, no patches.

React host = engine `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine` (pkg `@semio-tech/framework-renderer-react`, ShellHost under elements/, suites in engine/🧪️tests/*). Run: `1102-tools/reacthost/run.sh <n> <name-substring filters...>` = slot.sh + `SEMIO_TEST_LEVEL=long repo run @semio-tech/framework-renderer-react:test -- <filters> --run`. Gotchas: paths like `../../../../🧪️tests/..` match nothing ("No test files found", reacthost-1); use name substrings; default level only includes ⚡️quick, so SEMIO_TEST_LEVEL=long is required.

## Vitest (reacthost-2, rc 1 because of the one stale test)
| suite | covers past fault | result |
|---|---|---|
| 🔁️refresh-coalescing (48) | active utility per window (buildActiveUtilityByWindowId), refresh coalescing | pass |
| 🪪️window-action-panel (21) | panel actions dispatch in active window, utility palette en/de | pass |
| 🧩️contributions-push (6) | contributions publish | pass |
| 💼️job-ledger (4), 🎮️browser-interactive-job-port (17) | job progress | pass |
| 🎯️input-ledger (47) | action dispatch queueing / echo-off / per-frame gesture lane | pass |
| 🔄️shell-utility-leaves (4), 🌐️settings-locale-panel-refresh (2) | utilities, locale refresh | pass |
| 🎭️actor-window-actions (2) | action funnel for actor windows | 1 FAIL (stale test) -> fixed, reacthost-4 rc 0 (2 pass) |

## Fix (test only)
`🧪️tests/🎭️actor-window-actions/🟦️.ts`: `panel-hosting/🟦️.tsx` was split; the three `<InterpretedUiNode onAction={onActionStable}>` windows now live in `panel-hosting/🟠️mode-windows/`. Test now reads the panel-hosting module tree (`readModuleTreeSource`) instead of the single `?raw` file (verified: tree has 3 InterpretedUiNode, one `browserActorStore ?? builtNodeStoreFor` store). Also `document-transfer-tabs` was split (child `document-transfer-ref`), so that source is read as module tree too (reacthost-3 caught it; reacthost-4 green).

## Wave 2 (coordinator follow-up)
- Config `engine/targets/react/🧪️tests/🎚️config/🟦️.ts`: added `engineSuite` for 🪟️action-window-scope, 🔬️window-action-context, 🌪️root-animation-storm, 🎞️window-frame-render-isolation (long level).
- 🔬️window-action-context: was a bare exported function (0 tests). Now 3 vitest tests: RFC 6902 oracle over the shared fixture, plus the production `inputActionWithWindowV1`/`inputActionWindowV1` resolve the clicked window (not the focused one) with args untouched, then args.windowId, then focused window.
- 🪟️action-window-scope: failed at import (`The URL must be of scheme file`) because `🐚️Shell/🧬️schema/📌️panel-state/🧪️tests/🔬️unit/🟦️.ts` read its JSON via `new URL(.., import.meta.url)`; switched to static JSON imports (test file). 15 tests green (bun and vitest).
- 🌪️root-animation-storm (new, postcss oracle on `ui/🎨️styling/🖌️ui/🎨️.css`): every keyframe-animated custom property is `@property inherits: false`; no animation on :root/html/body; non-vacuous guard. Pass.
- 🎞️window-frame-render-isolation (new, React Profiler render-count probe over two `InterpretedUiNode` windows): 30 per-frame snapshot loads into window A commit A (<=30, >0) and never commit sibling B. Pass. A probe that an identical re-load of the same snapshot does not re-commit FAILED (11 commits for 10 reloads): `UiDocumentStore.loadSnapshot` notifies without a content check. Not treated as a fault (upstream `applyUiRefreshResponseToCache` preserves identity and is covered in refresh-coalescing); probe removed. Possible production hardening: skip notify when snapshot is deep-equal.
- Infra: reacthost-5/6 failed from the Nx daemon ("Native watcher physical canary failed"); `run-nodaemon.sh` (NX_DAEMON=false) works.

## Findings (earlier)
- The blind-gate item above is now fixed. bun test (explicit paths) passes for the pure suites; DOM suites cannot run under bun.

## Wave 3
- FIXED (TS host code, W3): `UiDocumentStore.loadSnapshot` (`engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx`) now keeps the previous record reference of every structurally equal node (`sameUiStructure`: JSON data, bigint, typed arrays; identity first via `Object.is`), so an identical resync notifies no node/root/revision listener and commits nothing. Regression: `🎞️window-frame-render-isolation` third test (10 identical reloads: same node reference, zero Profiler commits; a real change still commits). reacthost-9 rc 0: window-frame-render-isolation + refresh-coalescing + one more, 3 files / 53 tests pass. The store's own suites (`📃️UiDocumentStore/🧪️tests/*`) were not in that run; run them via the react package at level long if the coordinator wants proof.
- Nx daemon "Native watcher physical canary failed" (for actinfra): thrown in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔭️watch/🟨️.mjs:170` when `canary()` = `fence()` (line 143) returns not-ok in terminal failure mode. The fence writes a probe into the private fence directory and waits for the native watcher to report it within `policy.fenceTimeoutMilliseconds` = 1500 ms (`resident/🧬️schema/🔣️.json:184`). Under fleet load (host load 15-30 on 10 cores) the event arrives later than 1.5 s, so the canary reports "not exact" and terminal mode refuses instead of degrading to the portable backend. Not a watcher bug. Mitigations: budget the fence timeout by host load (as the test budget does: "Busy host: load N"), or use the portable fallback instead of refusing; `NX_DAEMON=false` (run-nodaemon.sh) sidesteps it. Not verified by reproducing under load.
