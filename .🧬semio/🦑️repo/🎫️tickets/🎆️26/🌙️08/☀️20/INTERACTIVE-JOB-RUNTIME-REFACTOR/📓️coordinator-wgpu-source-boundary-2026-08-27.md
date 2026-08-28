# WGPU Source Boundary

## Current Disposition

The planned mounted callback-verdict and candidate-publication packet is not coherently released. No current fleet lane is assigned a WGPU production write, but that is not implementation or runtime acceptance. Taxonomy must keep its complete 32-preimage refresh withheld; this report is a four-file read-only snapshot, not a catalog pin, source move, generated-output publication, or full source closure.

Root re-read the actual mounted callbacks and frame coordinator on 2026-08-27. No WGPU native, Wasm, browser, or timing test was run for this snapshot.

## Exact Snapshot

Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu`.

| Path | SHA-256 |
| --- | --- |
| `🦀️winit_app.rs` | `ea46cdbf206cc743a98c29556fe6978cf6e4da9cb26ccc61e9690ed660856ce3` |
| `🦀️frame_job.rs` | `ced742a20cb55b9b119b2371c0ec2ae0d20e31805ef1daf0af335541c556a0b3` |
| `📦️glue.rs` | `214c5ece5918ed0c3255828da5ac0f9441ddc164b7b2efa88cd879b5f6c01c28` |
| `🟦️typescript/🐚️plugin-bridge.ts` | `3a86f1735905126d20b40bf21b8c5a16d84e47667d41aebec7813e9bf50344a9` |

Actual `shasum -a 256` returned all four values with exit 0.

## Remaining Mounted Boundary

The four Winit wrappers still start an underscore-bound RAII watchdog, without explicitly checking admission or consuming its finish verdict before returning success or presenting:

- `enqueue_host_event`, lines 54–61.
- `enqueue_host_metrics`, lines 65–72.
- `WindowDelegate::redraw`, lines 117–120.
- `redraw_offscreen_worker`, lines 130–133.

The event/metrics wrappers mutate generation, invalidation and queue state inside that boundary. `redraw_core` advances resize, builds/publishes a snapshot, and presents. An eventual repair therefore needs candidate/commit ownership as well as a final timing check; adding a diagnostic after publication is not equivalent.

The worker-side frame coordinator is further along: `frame_job.rs` explicitly inspects session/preparation callback verdicts, checks watchdog admission for apply/build, and quarantines an overrun before accepting its outcome. `glue.rs::callback_verdict` delegates to its batch session. Those source facts do not establish the unfinished mounted Winit boundary or end-to-end timing.

An initial read of obsolete line numbers 1465–1665 in Winit returned no content; the exact current callback search above corrected the location. This is not evidence loss.

No source, output, cache, ticket evidence, or catalog preimage was edited, removed, restored, or repinned by this inspection.
