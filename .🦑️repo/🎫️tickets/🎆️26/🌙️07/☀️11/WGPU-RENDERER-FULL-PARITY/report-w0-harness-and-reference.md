# w0-harness-and-reference — report

Scope was three deliverables, all inside `.repo/🎫️/26/07/11/WGPU-RENDERER-FULL-PARITY/`. Deliverables 1 and 3 (code) are done and code-complete. Deliverable 2's live capture and deliverable 3's first live run are **blocked by a repo-wide build/lock contention issue** unrelated to this harness.

## 1. `parity-verify.ts` hardening — DONE

Rewrote in place (re-read immediately before the rewrite; `region-claims.json` confirms this file is exclusively claimed by this agent).

- **`REFERENCE_DIR` override**: `process.env.REFERENCE_DIR ?? join(ticketDir, "../../05/FIX-WGPU-WORLD3D-EMPTY-PREVIEW")` — defaults to the old ticket's captures, overridable without editing the script.
- **`parity-thresholds.json`** (new file, global thresholds + empty `perPlugin` map) wired via a new `Thresholds` region (`loadThresholds`, `thresholdsFor`) and a new `numericGate` check added to `compareStats`: per-region `|Δ meanLuma| ≤ 12`, per-channel `|Δ meanColor| ≤ 15`, relative `Δ nonBgRatio ≤ 25%`, gating navbar/body/footer independently. Breaching any gate escalates the plugin to FAIL. `perPlugin` is still empty — no real run has completed to calibrate overrides yet (see Blocker).
- **Double-capture anti-flake** (new `AntiFlake` region): `captureSettledPlugin` takes two canvas screenshots ~1200ms apart, compares region stats with a 5% relative tolerance (`statsStabilityNotes`); an unstable pair gets one retry pair; if still unstable, the capture is used but flagged `settled: false`. `finalizeSettlement` downgrades an otherwise-computed report to WARN (never FAIL, never touches SKIPPED) when unsettled.
- **`--consecutive-pass` / `CONSECUTIVE_PASS=1` mode**: extracted the per-plugin flow into `evaluatePlugin`; when set and a plugin's first run is PASS, the loop re-runs `evaluatePlugin` independently and only keeps PASS if the second run also passes.
- All existing behavior preserved: region-stat comparison, watchdog/retry/browser-poisoning handling, markdown+JSON report generation (extended with a `Settled` column/field and active `referenceDir`/`thresholds`/`consecutivePassMode`).

Not yet verified against real data — a `--plugin raster` sanity run was attempted specifically to exercise the new code independent of the react-shell issue, but it hit the same build-lock contention before reaching any actual capture.

## 2. Fresh React reference screenshots — CODE DONE, CAPTURE BLOCKED

Wrote `capture-react-reference.ts`: rather than edit the old ticket's `verify-react-playgrounds-e2e.ts` (hardcodes output paths via `import.meta.dir`, and it's someone else's ticket), this ticket got its own copy — same 25-plugin list, same functional assertions, writing `screenshot-react-<plugin>.png` + `capture-react-reference.log` into this ticket folder. Default port `7401`, checked free before use.

**0 of 25 `screenshot-react-<plugin>.png` files exist.** Two failure modes observed over ~40 minutes:

1. Early attempts: dev server booted and responded, but `framework/renderer/react/os-shell.tsx` / `ui-interpreter.tsx` were being actively edited by a concurrent, unrelated session (`git status` showed `MM`/churning mtimes). Every plugin threw `Cannot read properties of undefined (reading 'native')` inside `<FrameworkOsShell>`, with Vite repeatedly logging "server connection lost. Polling for restart."
2. Later attempts: `net::ERR_CONNECTION_REFUSED` on both port 7401 (react) and 7301 (wgpu). Root cause, per direct process inspection: repo-wide cargo target-dir lock contention — `trunk serve --port 7301` (pid 59732) alive but never finished booting because its wasm build child (`cargo build --target=wasm32-unknown-unknown --manifest-path framework/renderer/wgpu/rs/Cargo.toml`, pid 60076) had accumulated only ~0.44s CPU after over an hour — genuinely stalled on a lock, with ~20 concurrent cargo processes queued repo-wide (including an unrelated `RUST-WIDE-CLEAN-REFACTOR-CAMPAIGN` full-workspace check).

Per instruction, no further boot attempts were made once confirmed, and the still-running `trunk serve` / react dev processes were left alone rather than killed or duplicated.

**To finish**: once the lock clears and `os-shell.tsx` is stable, run `bun capture-react-reference.ts` from the ticket folder, then `REFERENCE_DIR=$(pwd) bun parity-verify.ts --compare` for a real 25-plugin report, and use that run's real deltas to fill in `parity-thresholds.json`'s `perPlugin` overrides.

## 3. `interaction-parity.ts` — CODE DONE, FIRST RUN BLOCKED

Wrote `interaction-parity.ts`: boots a react dev server and a wgpu dev server side by side (react default port 7300, wgpu default port 7301, each falling back to the next free port and reporting if it did; overridable via `INTERACTION_REACT_PORT`/`INTERACTION_WGPU_PORT`), drives an identical scripted interaction sequence against each, compares BEFORE/AFTER region-stat deltas.

Three probes, modeled on `verify-wgpu-playgrounds-e2e.ts`'s smokes:
- `commandPaletteProbe` (plugin `s`): Meta/Control+P, expects same-direction non-background increase in both renderers, then Escape returns both within 0.01 of baseline.
- `formsProbe` (plugin `forms`): two scripted clicks (0.55,0.5 then 0.82,0.5), expects a measurable body-region change in both renderers.
- `generateModeProbe` (flow, procedural2d, procedural3d): clicks the mode-toggle coordinate (0.76,0.04), expects same-direction body-region change within 2 orders of magnitude between renderers.

Each returns PASS/FAIL/WARN with reasons; writes `interaction-parity-report.md` + `.json` in the same convention as `parity-report.md`.

**0 probes executed.** The one launch attempt hit the identical lock-contention `ERR_CONNECTION_REFUSED` before either dev server was ready — the failure was entirely inside Playwright's `page.goto`, before any harness logic ran. No code bugs found.

**To finish**: once the lock clears, run `bun interaction-parity.ts` from the ticket folder.

## Files created/touched

All inside `.repo/🎫️/26/07/11/WGPU-RENDERER-FULL-PARITY/`:
- `parity-verify.ts` — modified (hardened, deliverable 1)
- `parity-thresholds.json` — created
- `interaction-parity.ts` — created (deliverable 3)
- `capture-react-reference.ts` — created (deliverable 2)
- `debug-react-boot.ts` — created (diagnostic script used to root-cause the react shell error)
- `build-react-reference.log`, `capture-react-reference.log`, `parity-sanity-raster.log` — created (blocked-attempt logs)

`ui/wgpu/rs/lib.rs` and `framework/renderer/wgpu/rs/lib.rs` were not touched. The still-running `trunk serve` (port 7301, pid 59732) and any react dev process from these attempts were left running/untouched per instruction — they're idle/stalled on the shared lock and need no cleanup from this session.
