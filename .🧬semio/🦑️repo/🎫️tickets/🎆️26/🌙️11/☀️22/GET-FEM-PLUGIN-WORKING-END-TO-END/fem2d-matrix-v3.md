# 2D FEM editor through the v3 browser matrix

Scope and honesty: against the 10:29 wasm I ran 2D boot, shell, util/selection/pan/wheel/gumball (first ~40 rows of `node matrix-v3/run.mjs 2d --skip-slow`, `🗑️generated/m2/full-part1.jsonl`, `full.log`) plus targeted re-runs (`t2`, `t3`). The full 175-row matrix was NOT completed (each row is 10-150 s under load). Rows after `gumball.rotate-small` in the run order (delete, history, checkpoints, inspection edits, tree, palette actions, remaining results, io, clipboard, role, windows, examples) are UNVERIFIED for 2D.

## Product defect (fixed, needed a wasm publish, live since 10:29)
- Symptom: the 2D Results window drew `Converged · 36 / 512 · Residual 7.460e-9` (and Pending/Failed/Mode/Phase captions) only as canvas text layers. No DOM/aria text, so screen readers could not read the solve state and the boot gate reported a false `2d.boot-hang` (caption="").
- Fix (Rust, so wasm publish): `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/.../📊️results/🦀️.rs` now builds caption lines once as strings (eigen/mode line, status line via `stale_caption`, `playback_caption`) and renders them as DOM text above the canvas (`with_captions`, `results_surface`), same structure as the 3D `with_caption` (column of text + growing scene stack). The canvas caption layers (`analysis-progress`, `eigen-result`, `modal-caption`, `buckling-caption`, `playback-caption`) and `caption_position` are removed: single source of truth, localized EN/DE through the existing `Fem2dLabels`. Cargo: `semio-ui-contract` moved from dev- to normal dependency of the results-window crate. 3D already exposed `Case: dead` as DOM text, no 3D change needed.
- Laws (native, `📊️results/🧪️tests/🔬️unit/🦀️.rs`, 25/25 green): `results_captions_are_dom_text_and_legend_sits_beside_the_model`, `results_caption_is_localized_dom_text_2d` (EN+DE, order, playback line from the same fn), modal/buckling/transport caption laws rewritten to read DOM captions. Live check: boot now `PASS`, `shell.results-window` and `res.caption-static` read `Converged · 36 / 512 · Residual 7.460e-9`, `res.resolve-after-edit` sees Pending then `Converged · 39 / 512 · Residual 9.901e-9`.
- Interim harness fallback: `run.mjs --legacy-caption` (boot/solve waits ignore the caption, fixed 15 s). Used only while waiting for the publish; not needed now.

## Harness defects fixed (`matrix-v3/*.mjs`)
| Check(s) | Cause | Fix | Status |
|---|---|---|---|
| all canvas checks (calibration `A=B=C`, marquee 1 px) | 2D lines are `#94a3b8` (148,163,184), outside the 3D line colour range; node dots split by lines into fragments | 2D line mask + merge fragments within 9 px | PASS (A ridge, B p0_l2, C p8_l2) |
| `sel.shift-additive`, `ctrl-subtractive`, `shift-ctrl-invertive` | `selIds` expected `Node ridge`, 2D Inspection says `Node: ridge` | regex `:?\s+` | PASS |
| `sel.mode-*` | `util()` toggled Direct Select off when already armed (radios vanished); fixed 3 s waits under 4-7 s round trips; Subtractive was trivially true | idempotent `util()`, polling `count(n)`, Subtractive builds A+B first, `after` resets Selective | PASS (4/4) |
| `sel.marquee`, `util.pan` (cascade) | drag started 0.7 s after arming the tool; the engine tool switch lags (up to ~2 s) so the drag panned the view instead; pan started on the Artifact panel edge | `util()` waits for `aria-pressed` then 2.5 s; pan drag from calibrated empty area; pan and wheel-zoom force a recalibrating reload | PASS |
| `sel.marquee-tiny` | SKIP/blind 3.5 s wait, Utilities not opened | `utilsOpen`, poll selection | PASS |
| `util.option-*` | gumClip included the options popover (checkbox ticks changed the hash), animated halo broke hash equality, gumball not yet drawn | `gumClip` x320-790 y60-820 for 2D, gumball colour pixel counts instead of hashes, wait for gumball | PASS (4/4) |
| `gumball.translate` | History count read while panel not rendered (`5->0`) | `commands()` waits for the Commands tree | PASS |
| `res.mode-modal` | read the combobox before it settled | poll until it shows the wanted option | PASS |

## Remaining / observations
- `sel.marquee` after a mode check only works with the tool-switch settle; the ~2 s UI-to-engine lag on tool switch and the 4-7 s selection round trip are product latency (Rust, needs profiling), not fixed here.
- Gumball rotate/scale drags after `gumball.rotate-small` were not observed (run stopped). `gumball.translate` and `rotate-small` PASS live (History +12 commands per drag: one command per move step, undo reverted all in one step).
- Not run: everything listed in the scope paragraph; run `node matrix-v3/run.mjs 2d --skip-slow --out ...` again for the full table.
- `C:\t\m2*` deleted. Harness helper `matrix-v3/m2-probe.mjs` added (single-step probe).
