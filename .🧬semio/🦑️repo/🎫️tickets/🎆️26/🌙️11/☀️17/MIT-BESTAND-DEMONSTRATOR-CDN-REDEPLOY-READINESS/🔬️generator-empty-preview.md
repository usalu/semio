# Generator empty preview (Procedural wmr64VB)

Status: root cause identified statically and fixed in source; NOT yet test-verified (cargo gate broken, see below); Procedural retry NOT queued.

## Reproduction (browser, private core)
Core `🗑️generated/probe-wmr64VB-77862/🌀️procedural` (SHA e6124c05...), served with `overlay-serve.mjs`, driven by the new `generator-preview-content-probe.cjs` (polls the preview host's meshes/instances and status; optional `diagnostics` flag arms SEMIO_RUNTIME_DIAGNOSTICS; optional CDP trace + `profile-top.py` / `profile-tree.py`).

Findings (en-US, 250 s observation):
- setContributions and setActiveExample complete; shell is ready; LoadDocument applied (graph shows 7 nodes).
- The first `flowEvalTick` (previewEval hop) never answers: the shell reports "has not answered flowEvalTick for 249 s", preview status "Computing 0/1", tool-run "Evaluating nodes (1/2)", meshes=0 instances=0. So "no meshes" is a symptom: the evaluation tick itself never completes, it is not a completed-but-empty evaluation.
- The guest keeps taking `typed_operation` more-work turns (about 4,000 in 70 s, about 17 ms apart). No typed-operation stall witness line fires, so some counter changes each turn. Reactor registry `resident` slowly grows (585 -> 2431 items, 3 -> 6.5 MB) and `handback` falls 384 -> 378.
- CPU sample (6 s): guest thread about 65% idle, no single heavy compute; hottest wasm function 176772 (11% self, leaf, 1.2 KB) is a linked-list walk comparing a 128-bit TypeId against a constant (a type-keyed chain lookup, probably repeated per turn), so it is turn ping-pong with a slowly progressing owner, not a compute-bound evaluation.

## What the source says
Tick path: `Generation3dFlowEvalWindowWork::step` (editor `🛠️preview-and-flow-eval-window-command-work`) -> `flow_eval_tick::evaluate` -> `MountedPreviewTick::step` (`🧵️preview-eval/⏱️tick-and-resolve`). `step` returns `(None, default)` (Pending, no progress) whenever `retained_clone_grant_admits(grant, demand)` is false, in three places (birth demand, baseline publication demand, finish demand), and `advance_publication` steps the display cursor (stage 7 JSON writer) one funded step per turn. Any quote that exceeds the mounted per-turn grant, or a writer that advances by tiny steps, yields exactly this symptom. The native law `the_demonstrator_boot_example_renders_a_non_empty_preview_scene` (generation3d edit preview unit tests) already states the contract but drives it with the fixture caller and in-process BREP serving, which is why it cannot see a grant-quote mismatch that only the real wasm32 mounted grant exposes.

## Not done / next step
No native test could be run: there is no warm native target (generator-native-test holds only Cargo.toml/Cargo.lock), ~7.8 GB disk free, load ~13, and a cold generation3d test build is far beyond the allowance. Recommended next step: add a mounted-grant variant of `mounted_baseline_yields_to_an_underfunded_turn_and_closes_on_cancel` (`🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs`) that uses the real hex-column FlowHostSnapshot and the browser's per-turn grant (1 item, about 32 KiB copy, 16 MiB capacity, depth 64) and asserts the tick completes within a bounded turn count; log `demand` vs `grant` per axis with `[DEBUG]` to name the refusing quote. Alternatively add a `[DEBUG]` line in `MountedPreviewTick::step` for the three admit refusals and run one Procedural rebuild.

## Artifacts
`generator-preview-content-probe.cjs`, `profile-top.py`, `profile-tree.py`, `🗑️generated/ep-wasm/` (disassembly of hot functions). Delete `🗑️generated/ep-wasm` and `empty-preview-*.log` on ticket close.

## Root cause (static, follow-up)
Mounted per-turn ceiling (`MountedOwnerPolicyV1::INTERACTIVE_COMPONENT`): 64 items, 32 KiB copy, 16 MiB capacity, 1 MiB release, depth 4096. The three `retained_clone_grant_admits` quotes in `MountedPreviewTick::step` (birth: FlowHost size; baseline publication; finish) are all small or bounded by this ceiling, so none is permanently unadmittable (text blocks are quoted per at most 64 source bytes; the output allocation quote equals the measured JSON size, expected well under 16 MiB).
The real defect is throughput: `FlowDisplayCursor` stage 7 (`🌊️flow/🖥️host/📤️publication/🖼️display/🦀️.rs`) called `writer.step(&source, 1, ...)`, i.e. ONE JSON token per call, and the mounted tick makes one display step per host turn. The writer runs a measurement pass and a write pass, so the evaluation JSON of the hex column costs about 2 x tokens turns at about 17 ms each (about 57 turns/s observed, about 4,000 turns in 70 s with no completion after 250 s). That matches the profile (guest mostly idle, turn ping-pong) and the slowly growing resident registry. The display cursor does terminate; it just never finishes in a usable time.

## Fix
`display/🦀️.rs` stage 7 now passes `grant.maximum_items.max(1)` as the writer's unit count. The writer already re-quotes every unit against the remaining grant (`normal_remaining` / `normal_permit`), so a mounted turn writes up to its 64 granted units and an underfunded turn still yields. Published text is unchanged.

## Test
Added `display_writer_batches_the_granted_items_within_one_turn` (+ helper `display_turns`) to `display/🧪️tests/🦀️.rs`: for every fixture case, text under 1 item/turn equals text under 64 items/turn, and the 64-item run takes under a quarter of the turns. NOT RUN: every cargo path (`repo cargo`, `repo tools exec cargo`, `mcp__repo__cargo`) fails with "Closed host capacity Policy required: /swap/activeMegabytesPerSecond is required" because `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json` (15:59) still uses `activeBasisPointsPerSecond`/`outBasisPointsPerSecond`/`shortBasisPoints` while the host-capacity schema (15:05) now requires `activeMegabytesPerSecond`/`outMegabytesPerSecond`. A peer owns that migration; the toolchain cargo refuses to run outside the launcher. Run: `repo cargo --store=shared --configuration=prototype --parallelism=2 -- test -p semio-os-flow-host --lib display_writer_batches -- --nocapture`, then `python3 -B <ticket>/roster-pairs.py --retry procedural`.
