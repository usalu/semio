# Prepared GPU Overrun Streak Audit

Read-only source and recorded-browser audit on 2026-09-21. No build, test, or browser action was performed.

## Recorded failure

The physical journey recorded a WGPU quarantine at 2026-09-21T19:40:07.118Z:

> prepared GPU opportunity exceeded the two millisecond ceiling for 4 consecutive opportunities: Commands took 3100 us

The receipt names only the broad `Commands` phase. It does not include the command page index, `DrawMeasureCursor`, clip-piece index, render packet generation, or the preceding timing samples. The run predates the current clip-piece browser artifact, as root reported, so it cannot establish that the current clip-piece implementation either fixes or fails the concrete command.

Source: [journey receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-runtime/checkpoint20-iab/journey-physical.json:1911).

## Confirmed streak-accounting defect

The current `PreparedGpuPresentCursor::overrun_run` represents neither a same-scalar failure nor literally consecutive physical opportunities.

`prepared_present_step` measures one start/end wall-clock interval and carries `overrun_run` across every phase and every command. A successful `Commands` scalar advances `cursor.command` or `cursor.clip_piece` before common timing admission. Thus four expensive but distinct command pages can fault as a single “consecutive” run despite visible forward progress.

More seriously, several successful branches return before the shared timing admission. They neither add a breach nor clear a prior breach:
- command list exhaustion, after moving to `Present`;
- a command with no draw cursor, after incrementing `command`;
- terminal blur transition, after moving to `CompositeGlass`;
- empty and complete glass clip pieces, after incrementing the clip piece or command.

This means a later set of measured over-ceiling command calls can inherit a run through one or more unmeasured successful opportunities. The current diagnostic’s “4 consecutive opportunities” statement is therefore false as an accounting invariant.

This is independent of the old WASM artifact and should be repaired before interpreting any later sustained-breach result.

## Minimal correct repair

Keep the two-millisecond ceiling and quarantine threshold. Do **not** reset the streak merely because a cursor moved: that would convert a repeated slow scalar into endless accepted work and weaken the budget gate.

Instead, make the common timing/admission tail mandatory after every successful nonterminal `prepared_present_step` branch:
1. Each branch changes exactly one phase/index as it does today.
2. It falls through to the one elapsed-time sample and `admit_prepared_gpu_opportunity`.
3. A sample at or below the ceiling resets `overrun_run`; an over-ceiling sample advances the run; the threshold remains terminal.
4. Only already-terminal `Complete`, explicit errors, and `Closing` may bypass timing.

This restores the stated “consecutive opportunities” law without hiding any slow operation. It also makes browser task pacing visible: the WASM loop yields only after one completed `present_step` exceeds its half-interactive deadline, so it cannot preempt a costly WebGPU/JS call before that call returns.

Add a compact diagnostic payload on the terminal error:
`phase, command, draw_cursor, packet_overlay, clip_piece, elapsed_us, prior_run, scene_revision, preview_generation`.
The source already owns every field except an easy `Debug` rendering of `draw_cursor`. This distinguishes a slow clipped scalar, an unclipped scalar, metadata-only command, and a cross-phase streak on the next browser run.

## Clip-piece scope

The current clip-piece cursor is a necessary boundedness improvement for color commands with disjoint layer clip regions. It is **not** sufficient to establish this fault’s cause:
- `PreparedCommandClipPiece::Scissor` splits one color scalar one piece at a time;
- `Unclipped` commands still encode the entire scalar in one opportunity;
- metadata commands with no draw cursor still call `queue.write_buffer`;
- scene shadow work and phase transitions have their own command paths.

Therefore, take a fresh artifact after the exact streak-accounting repair and inspect the enriched terminal payload. If it names a clipped color scalar with a large clip piece, further split that scalar at its existing draw-unit boundary. If it names an unclipped scalar, splitting clips cannot help; its own encoder or resource operation needs a bounded subcursor.

The existing native clip law proves clip-piece arithmetic and cursor visibility but cannot prove browser wall time. It should remain separate from the browser runtime law.

## Required fail-first laws

1. **Timing completeness.** With a deterministic timing seam, drive a `Commands → metadata → Commands` sequence where the first and third are over ceiling while metadata is under ceiling. The second sample must reset the run; no terminal fourth-run result is allowed. Repeat for empty and completed glass pieces and the terminal blur transition.

2. **Sustained same-unit breach.** Drive four measured over-ceiling calls with no under-ceiling opportunity. It must remain terminal at the existing threshold. This protects the ceiling while adding timing completeness.

3. **Browser command attribution.** Run a neutral retained packet with clipped and unclipped color commands in a mocked clock/browser-port harness. Assert an overrun error names its exact command and clip piece. This is an observability law, not a string-only source test.

4. **No premature submission.** For every `Pending` return after the first over-ceiling command, assert the packet remains owned by the presenter, the prior completed presentation remains accepted, and no input candidate is acknowledged until the final present succeeds.

## Source anchors

| Concern | Source |
|---|---|
| Ceiling, global run, and reset rule | [GPU implementation](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:37) |
| Per-command cursor advances and early timing bypasses | [GPU implementation](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:634) |
| Common measurement tail reached only by fall-through branches | [GPU implementation](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:711) |
| Cursor already exposes phase/command/clip-piece to the stall watcher | [GPU implementation](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:233) |
| Browser loop can only yield after a completed `present_step` | [Winit app](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:285) |
| Existing pure timing law lacks cursor/progress interleavings | [GPU prepared-present test](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs:50) |
| Existing clip law's bounded scope | [GPU prepared-present test](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs:156) |

