# Layout WGPU End-to-End State Audit (2026-11-22)

Read-only audit. No builds, servers, browsers or git commands were run. Only this file was written.

## 1. Last real journey run

- **No live run has ever happened.** `browser-journey.mjs` (mtime 2026-11-21 12:54) and `browser-journey.json` (2026-11-21 12:43) have never been executed against a live server on 6179.
- Evidence: `current-status.md:116` ("no 6179 server or browser journey has run"); `current-handoff.md:537` ("NO ACTUAL BROWSER HAS EVER RUN"); `browser-readiness-2026-11-05.md` (read-only review, "no activation/server/browser launched").
- `find 🗑️generated -name report.json` returns **0 files**. No journey report exists.
- Result: **0 of 13 journey steps verified** (boot, artifact-panel-baseline, baseline-media, rendered-text-contrast, exact-create-undo-redo, story-marker, export-svg, export-png, edited-geometry, export-pdf, export-archive, diverge-original, reopen-archive).
- Only jsdom or source-level checks exist (`browser-validation.md:52`, 4 media tests passed, not a live journey). These do not count as journey steps.
- Latest activity: `🗑️generated/claude-1122/activate-1.txt` (2026-11-22 01:58-01:59). It is a graph-slice wait that stops at "[repo] still working after 50 s; last phase started: door.script". It has no success receipt and no failure, and the excerpt does not name the target. No serve process on 6179 was observed in `ps`.
- Earlier wgpu attempt: `🗑️generated/layout-wgpu-activation-1/activation.log` (2026-10-31 16:53). It ends with `Owned Nx invocation failed (124)` for `activate-layout-wgpu-prototype`, after `@semio-tech/os-plugin-describe-rs: native:owner-command failed (1)`. It also logs 256-character staging refusals that fall back to in-process inference (lines 1632+).

## 2. Open blockers (activation, serve, browser)

1. **JS dependency bindings lease** held by a foreign dev server (pid 66263, `dev-site` of the teaching course; 66127 and 66936 also in its tree). This blocks every `repo run` chain. Source: `claude-blocker-handoff.md` (Blocker 1). Not stopped by this session; the owner must release it.
2. **Native cargo ingress** for `🌎️hub/Cargo.toml -p semio-hub-layout` stalled at `Checking source configuration: 1/762` and was SIGTERMed after 47, 8 and 3 min. Source: `claude-blocker-handoff.md` (Blocker 2). Logs are `🗑️generated/claude/check-layout-*.txt`.
3. **Nx native storage mapping custody.** Exclusive holder pid 10518 on `.cache/b/n/m` (key `9d349bf4…`), with a shared waiter in the same process. Source: `custody-blocker.md`. The declared alternative `repo --tool=bazel run …` is recorded as not yet exercised for the product path (`custody-blocker.md`, Decision).
4. **Native gate compile failures.** Renderer publication compiler failed at known SPR history/Store Shared/Op consumers (`current-status.md:116`). Writer run hit `ENOSPC` on `stdio-semio-video-model .rcgu.o` with 0 free compiler image (`current-handoff.md`, Current State section). Native lane is on HOLD per that note.
5. **256-character path limit** on input closure staging. The graph falls back to in-process inference (`layout-wgpu-activation-1/activation.log:1632`). This is a CLAUDE.md constraint that the activation does not satisfy in worker staging.
6. **Stale React artifacts.** Prepared React activation and receipts at 6079 are stale (`current-handoff.md:537`). `claude-layout-acceptance.mjs` (Playwright A1-A11) is React-selector based and was never executed (`claude-blocker-handoff.md`, acceptance status).

## 3. Renderer target

- **Accepted target: WGPU, port 6179.** Basis: `status-history.md:21` ("Historical React6079 instructions are superseded"); `current-handoff.md:537` (wgpu routes are current, React 6079 stale); `browser-readiness-2026-11-05.md` ("WGPU 6179"); `browser-journey.json` `renderer.kind = "wgpu"`, `port = 6179`.
- **Conflicting older text (React 6079).** `completion-evidence.md:56` still names "React 6079 boot" as the accepted goal, and `claude-discovery.md:17` lists both `react=6079, wgpu=6179`. `claude-blocker-handoff.md` (Nov 1 session) prescribes react 6079 and `📜️claude-layout-e2e.sh` serves 6079. These predate the wgpu decision and should not be followed.
- No ticket document dated after 2026-11-05 explicitly restates the renderer choice. The user's current goal (wgpu 6179) matches the newest notes.

## Suggested next step (not performed)

Release the dependency-bindings lease (or get owner approval), confirm the Bazel or Nx route for `activate-layout-wgpu-prototype`, then run `serve-layout-wgpu-prototype` and `browser-journey.mjs` under `repo tools exec bun`.
