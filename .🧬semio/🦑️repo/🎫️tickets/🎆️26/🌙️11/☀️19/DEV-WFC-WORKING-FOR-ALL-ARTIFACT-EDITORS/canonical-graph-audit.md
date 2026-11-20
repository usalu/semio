# Canonical graph and UI audit

Snapshot 2026-11-19. Five public `repo run @semio-tech/framework-os-dev:dev-<variant>-react-prototype --parallel=1 --lang=en` processes remained alive (grid2d 40204, bitmap 58519, wfc2d 66628, grid3d 66629, wfc3d 66630). No listener on ports 6041–6045 was observed. No process was stopped or duplicated.

Grid2d's `[DEBUG] canonical-serve-grid2d.log` reported host admission waiting about 12 minutes: 2048 MB requested/minimum, ten active grants and five or six queued; reported available headroom above the 2048 MB floor varied roughly 71–548 MB. Bitmap, wfc2d and grid3d report a checkout load ahead sharing the fact store; wfc3d reports the graph service holds its request. These are upstream of task execution. A component compiler was actively running, and other cargo requests were live. This supports capacity pressure and graph head-of-line blocking, not proof of a cyclic deadlock. Wait for active work to release capacity, then reassess; do not lower the memory floor or kill builds without establishing ownership. If the queue persists after adequate capacity is available, inspect grant owners, queued order and graph host's leader and worker state to establish a wait-for cycle.

The graph booking contract at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/📐️booking/🟦️.ts:1` documents the 2048 MB minimum and elastic grants. The queue message contract is `…/🏡️resident/🧬️schema/🔣️.json:2042`, explicitly explaining shared fact-store serialization. This exposes a liveness/observability opportunity: graph queue waiters should report the leading request's capacity wait, requested and available memory and actionable owner details, instead of merely implying active graph work. Prove a cycle before changing admission policy.

The public dev target is the correct requested path; generated playground targets are covered by `…/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:261`. The static dev project does not enumerate variants; task names are generated. No claim of public-target success or native-step equivalence is possible until graph admission and execution complete. Direct activation/serve is useful for diagnosis but does not establish the public dev target's dependency chain.

## Browser controls from source

- Bitmap: Output window declares `solve` / Solve (`…/🖼️bitmap/…/🎭️modes/✏️edit/🪟️windows/🧩️output/🦀️.rs:53`); `setActiveExample` is labelled Load Example in editor `🦀️.rs:105`. Output is the preview; Pin Solution persists it.
- Grid2d: Preview window declares Solve (`…/🔲️grid2d/…/🪟️windows/👁️preview/🦀️.rs:46`).
- Wfc2d: Preview window declares Solve (`…/◻️2d/…/✏️editor/🚀️editor-assembly/🦀️.rs:47`).
- Grid3d: Grid window declares Solve (`…/🧱️grid3d/…/🪟️windows/🧱️grid/🦀️.rs:133`); its editor describes a preview in the grid (`…/✏️editor/🦀️.rs:782`). The default layout also includes a Preview window.
- Wfc3d: Preview window declares Solve (`…/🧊️3d/…/✏️editor/🦀️.rs:109`).

The example picker dispatches `setActiveExample`; the bitmap action bridge identifies the navbar picker (`…/🖼️bitmap/…/🎛️command-and-action-bridge/🦀️.rs:80`). Labels, enabled state, completion and rendered preview remain to be verified in the live browser.
