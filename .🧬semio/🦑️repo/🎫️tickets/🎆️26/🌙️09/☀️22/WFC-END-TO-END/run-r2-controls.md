# WFC Observable Controls Review

Read-only review on 2026-11-04. No source changed or extra execution queued. This is a bounded acceptance proposal, not a measured runtime claim.

## Determinism Limit

There is no public start-paused argument. Start accepts toolId and optional windowId; Pause is legal only while Running. Consequently a larger finite document cannot guarantee that a Playwright process on every machine reaches Pause before completion. Fixed sleeps and a larger default do not solve the scheduling race. Treat a missing observed pause as an acceptance failure requiring an admitted larger case, never as a passing cancellation proof.

There is an immediately useful correction: Abort is legal in Starting, Running, Paused, Complete and Finalizing. Current scenario skips cancellation when its second run is Complete. Instead abort Complete and require Aborted plus unchanged ledger. This proves cancellation before publication, explicitly distinct from cancelling active search.

## Portable Bounded Setup

Use a separate controls phase after the small completion/keep case. Preserve the default model and seed for reproducibility; do not construct an adversarial contradiction/backtracking instance. Capture ledger after setup, then compare cancellation against that baseline.

| Variant | Public setup | Initial bounded candidate |
| --- | --- | --- |
| bitmap | select output, resize-output(width,height,periodic) | 64x64, periodic=true; keep rooms-16 model |
| grid2d | select grid, resize-grid(width,height) | 64x64; keep pipes example |
| grid3d | select grid, resizeGrid(width,height,depth) | 16x16x16; keep blocks example |
| wfc2d | select graph, repeated create-slot(id,x,y,width,height) | 1024 uniquely named unpinned slots in32x32 placement |
| wfc3d | select graph, repeated create-slot(id,x,y,width,height) | 1024 uniquely named unpinned slots in32x32 graph placement |

Graph create-slot is the existing graph form for both dimensions; its layout fields are graph x/y/width/height, not a grid resize. Adding disconnected slots avoids accidental conflicting edges and needs no invented graph batch API. It costs1024 real edits per locale and must be bounded/cancellable itself. A plugin-owned authored larger example loaded through the existing example picker is preferable to this many UI edits, but no such large registered example was established by this review. Do not use raw internal guest calls to bypass setup.

Candidates are deliberately modest:4096 grid cells and1024 graph slots. They are candidates for observation, not guaranteed durations. If the first size races, permit exactly one declared increase (grid2d/bitmap128x128, grid3d24x24x24, graph2048 slots) within the same120second controls deadline. Refuse further automatic escalation. Avoid maximum512x512 bitmap merely to slow a test.

## Why Bound Below Schema Maximum

Shared engine protocol has MAX_CHECKPOINT_BYTES=1MiB and MAX_COMMIT_BYTES=1MiB; MAX_COMMIT_ITEMS=(1048576-160)/11=95310. Checkpoint state additionally counts domains/trail/decisions/observations, so commit count alone is not a valid memory estimate. Bitmap output schema512x512 permits262144 cells, exceeding the shared engine commit-item bound; maximum bitmap pin capacity is therefore not evidence that maximum solve is admitted. Grid resize schemas specify minimum1 without a maximum, which likewise does not establish affordable solving. The proposed small counts leave substantial commit headroom; checkpoint admission still must be observed.

The shared WFC job emits preview after16 work units or16ms and caps preview items256. Each fill adapter checks fuel/deadline during work. These establish yielding opportunities, not an end-to-end browser pause timing guarantee.

## Required Observations

For each variant, await a new public run scope and an enabled scoped Pause control with Running status. Click immediately without the existing400ms pre-hold delay. Require Paused/Pausiert, unchanged progress during the hold, then Step while paused and require a new progress/stage observation while returning to Paused. Resume and require subsequent progress. Abort that same run while it is still Running or Paused; require Aborted and no document edits after the setup baseline. A later separate small run owns completion/finalize/keep.

Abort of Complete should be retained as an additional inexpensive law. It must not be reported as mid-search cancellation. If the bounded controls instance completes before Pause, report the missing control witness rather than converting it into a passing note. Engine unit tests with one-unit StepContext budgets can deterministically prove cancellation frontiers separately, but cannot replace this public browser interaction.

Sources inspected: framework tool-run actions-labels TypeScript and Rust; shared engine job protocol-and-determinism and job-run interactive loops; all five fill adapters; graph create-slot action definitions; grid resize schemas; bitmap resize-output schema. No runtime admission or timing measurement was performed.

## Independent Portable Validation Executed

All three commands completed2026-11-04 with exit0:

- repo tools exec --light python -- "<ticket>/bitmap-retained-oracle.py": Pillow matched9 retained bitmap fixture cases.
- repo tools exec --light python -- "<ticket>/pinned-output-oracle.py": Pillow independently confirmed complete pinned output row-major indices.
- repo tools exec --light python -- "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-bitmap-1/🐍️.py": checked14 committed vectors,9 pin-batch cases and52 retained/Pillow cases,0 failures.

Here <ticket> is .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END. These portable/reference checks do not compile or execute the native retained owner or browser guest.
