# Play release gate hardening (2026-11-22)

## Finding that changes the brief
The dev and release suites run the SAME spec (`🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts`; config `testMatch` has only that file). Its header promises a "non-uniform pixel census of the largest canvas", but the code only does it for `generation3d` (pixels of a WebGPU surface read back transparent and element screenshots include window chrome; ticket 26/09/19 `📓️raster.md` measured 118-316 colours for a BLANK raster). The 2026-09-22 proof therefore rested on retained-data witnesses (`data-meshes-json`, `data-layers-json`, DOM text/elements), not on a reusable pixel helper. There are NO exemptions in code: the only rule is "a pane passes when ANY window paints" (kept unchanged). I invented none.

## Files changed
- NEW `🧪️tests/🎭️acceptance/🖼️visible-content/🟦️.ts` shared helper (dev spec and release use it; generation3d's old inline census now calls it too).
- EDIT `🧪️tests/🎭️acceptance/🟦️.ts`: imports helper, SPA-fallback collector uses `spaFallbackAnswer`, `RELEASE_RUN` (= `PLAY_E2E_RELEASE_ORIGINS` defined, always set by `TestReleaseScript`), `expectReleaseVisibleContent` called after the existing paint witness in the per-pane test.
- NEW `🧪️tests/🧪️playvisiblecontent/🟦️.ts` + `🔣️.json` bun tests (12 pass: `bun test ./🧪️tests/🧪️playvisiblecontent/🟦️.ts`).
- NEW ticket `tsconfig-hardening-1122.json` (tsc strict noEmit ES2024: clean).

## New release-only assertions (per pane, after boot paint witness)
1. Framework faults: no element matching `FRAMEWORK_FAULT_SELECTOR` anywhere in `[data-layered-pane=id]`. Markers taken from the UI sources: `data-semio-window-fault` (WindowFaultStatus), `data-semio-os-shell-error` (canvas error), `data-shell-fault-boundary`, `[data-slot=error-view]` (ErrorView), `data-layered-pane-error`, `data-shell-route-not-found`, `data-semio-bootstrap-status`, `[data-semio-execution-target-status][role=alert]`, `data-directory-bootstrap` fault/locale-missing, `data-unknown-component`, `data-media-state=invalid`, and the conditional refusal slots `input-refusal`/`slider-refusal`/`stepper-refusal`. Field `field-error` and generic `role=alert` were deliberately NOT included (validation can legitimately show at boot).
2. Empty DOM windows: every visible `[data-slot=window-body]` (layout area > 0) with no canvas and no retained 3D/raster surface must have a content host and text or >= 10 elements (same stripping and threshold as the dev domWitness). Hidden/inactive windows are skipped.
3. Canvas: if the pane has visible canvases (>= 32 px each edge), polled up to the existing 4 s paint deadline until at least one has >= 2 distinct interior colours (same inset/step census as generation3d). Pane-level "any canvas", mirroring "any window paints". generation3d keeps its stricter >= 3.
4. SPA fallback: release additionally covers `.wasm .js .mjs .cjs .json .css .map .webmanifest .txt .xml` (media list unchanged), for all panes and the overview test.

## Risks (not runnable now)
- Pixel census via element screenshot can false-GREEN a blank WebGPU canvas under chrome overlays; it cannot be tightened without per-app knowledge. It can false-RED a pane whose only canvas is legitimately flat (audit-visual-2 calls `sourcing`'s viewport near-empty); if the first release run reds, add a named exemption then, with evidence.
- Rule 2 is stricter than dev (dev passes if any window paints); a legitimately empty secondary window will red.
- Not run against a cohort; only helper unit tests and type-check done.
