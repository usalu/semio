# Fill Runtime Review

Read-only production/domain review plus coordinated private-test harness changes. No native command was launched by this reviewer. Runtime owns kernel and Lowpoly retained production edits; reviewer owns the retained private unit test file until handoff.

## Existing Red Contract And Harness Correction

The neutral fill-budget fixture uses a real256×256 RGBA layer,256 original fuel/turn, the exact mounted maintenance grant, minimum65536 paid work, minimum256 progress boundaries, one final mutation and exact channel sums. Algorithm work is separate from ArtifactCommandWork.extent, which counts staged edit rows.

The old test counted StepContext.original_work_units_spent but constructed fresh manual contexts/new operation IDs each turn. It did not cancel or restore a checkpoint despite its name. A native source-bound cooperative workspace needs stable original custody, not a newly invented context.

The test now uses one OriginalJobFixtureCaller with matching operation/generation and one decoder across turns. Each work call receives an original work loan narrowed to256; the test records the spent delta around that call rather than attributing caller preparation to the fill. Per-turn spent must fit fuel. The final Emit is retired through original retained close, then work, decoder and caller are closed under paid turns. One-shot fill has only its declared artifact publication lane, so an unexpected ephemeral publication is rejected.

These edits are authored, not compiled/executed by this reviewer. Separate actual cancellation and checkpoint assertions await runtime's settled workspace/replay seam, as coordinated with runtime.

## Domain Colour And Topology Law

Current Lowpoly flood fill matches all four RGBA channels exactly, traverses four-connected pixels, maps UV with inverted V and nearest pixel, and replaces selected pixels directly with the requested RGBA. Pixel runs coalesce consecutive changed whole RGBA pixels, not individual changed bytes. Same-colour replacement yields no changed runs/edit.

The existing framework flood_selection is synchronous; fill_in_place uses source-over/coverage filtering. PixelEditJob advances bounded output pixels but its constructor/resizing/cancel APIs do not establish the original source/paid-close contract for a flood cursor. Reusing source-over fill would change Lowpoly's colour law, especially transparent/partial-alpha replacement.

Added ticket-only fill-review-cases.json, fill-review-schema.json and fill-review-oracle.mjs. Strict Ajv plus existing third-party Graphlib independently passes six cases: diagonal disconnection, barrier separation, alpha-sensitive matching with transparent direct replacement, exact no-op, top-left UV, bottom-right UV. It verifies reached/changed indices, final exact RGBA and whole-pixel run offsets. First schema check exposed a strictRequired schema issue; schema was corrected without disabling strict mode, then the six cases passed. This validates neutral expectations, not Rust/TS production execution.

## Required Cancellation And Checkpoint Tests

Use the actual fixture caller token after representative phase prefixes and all prefixes of small cursor cases. Keep the same original source, operation, generation, base revision and context. Assert no mutation/effect publication before completion, no committed change after cancellation, and no further domain work after cancellation is observed. Closing must remain possible through original retirement even when ordinary work is cancelled.

Checkpoint at each phase and useful algorithm prefix. Restore under identical inputs and compare final pixels/runs with uninterrupted execution and the independent oracle. A deterministic replay is acceptable only if it replays in bounded, paid steps; it must not synchronously rebuild the entire workspace in restore(). Stale operation/generation/base/context/tool identity must refuse. Do not hardcode the previous72-byte frame once the workspace/checkpoint format changes.

Each dynamic buffer/queue/visited set/run list must have truthful work_demands, construction/copy capacity and bounded close demand. Denied resource axes must not mutate cursor/storage or perform uncharged algorithm work. Do not free whole work buffers by assigning empty Vec in a cancellation callback. Paid close must reach terminal_is_empty and retire transferred output separately.

A count of Progress returns alone is insufficient: dummy yields followed by one whole-image reduction would satisfy it. Inspect actual bounded traversal/assembly phases and measure real original-work units consumed. Domain work, materialization, diff/run assembly and replay all need explicit bounds; preparation labels do not prove it.

## Generic Import Producer Risk Raised By Coordinator

The generic OpBinary strategy still performs encode_op and shrink_to_fit monolithically. This was pre-existing with65536-byte defaults; Lowpoly's new2101280-byte selector expands the accepted worst-case work/memory for that app. The selector correctly bounds declared capacity and wire length, but does not make encoding algorithm-interruptible or prove the7.5ms turn claim.

Also, oversized encoded bytes currently cause an Err from inside construction while the local Vec is dropped rather than retained for an explicit paid close. This refusal design predates the selector, but larger accepted/attempted commands increase its possible release/copy cost. The new ordinary oversized test checks refusal text, not paid release/memory receipts.

Needed red evidence: actual large accepted encode under original caller/decoder and exact grants; heap/capacity/copy/release receipts at encoding and shrink; just-over-bound result with complete paid custody/retirement; cancellation at pre/post encode phases with no publication. Measure actual turn duration as supporting evidence, not a substitute for work accounting. Do not claim the generic producer is cooperative merely because later wire paging is bounded. No production change made during the coordinator's Native source capture.

## Current Acceptance

Kernel TS can run its language-neutral cursor red/green via the public light Bun route independently of pending Rust compilation. Rust paid fill/cancel/replay tests still need the coordinator's genuine native execution. Live progress/cancellation and paint/history/reload remain browser acceptance. None is claimed by this audit.

## Connected Comb Retained Regression

Authored retained_fill_comb_preserves_sparse_pixel_topology in the retained private unit file, consuming the runtime-owned neutral fill-comb.json. It uses the same persistent OriginalJobFixtureCaller and decoder, original fuel256 and real interactive maintenance grant. It requires32,896 changed pixels and32,641 whole-pixel sparse runs, compares the exact output pixel buffer, refuses runs spanning unchanged gaps, bounds each originalWorkUnitsSpent delta, and pays emit/work/decoder/caller retirement. This is authored source only; no native execution is claimed. Actual cancellation/checkpoint tests remain pending the Lowpoly integration contract.
