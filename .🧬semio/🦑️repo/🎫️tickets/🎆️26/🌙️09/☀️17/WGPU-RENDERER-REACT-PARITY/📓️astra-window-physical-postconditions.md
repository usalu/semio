# Window Physical Postconditions

Checkpoint 15 delivered clicks for pane chips, cap focus, individual cap close, and split gutter drag without asserting their outcomes. The next journey must assert physical consequences before those steps can count as passed.

## Existing Runtime Evidence

The full checkpoint 15 gutter step dragged the split center 120 CSS pixels. React published the first World window at width 644 after the drag. WGPU published width 524.9332 and its split center moved only about 1.2 pixels. The WGPU step reported no error because it checked only delivery. This is an observed resize discrepancy, pending a focused replay with outcome assertions.

Both renderers expanded the focused Top window to roughly 1587 pixels and hid the sibling World body. The following individual close removed Top and published the Perspective body. These saved observations support the outcome, but the journey still needs live identity and geometry assertions to prevent future false positives.

## Required Assertions

- A split drag keeps the exact live window identities, moves the selected separator by the requested safe distance, and changes neighboring World rectangles in the corresponding direction.
- Focus expands the selected window to the previous union of live bodies and hides sibling bodies; unfocus restores their identities and bounds.
- Individual close removes the selected identity and its physical body, including when closing a maximized window reveals a sibling without reducing the number of visible tabs.
- Pane chips expose and retire their own visible child controls; a delivered click or action ledger entry alone is insufficient.

Root owns the ticket journey edits. Palette implementation coordinates its separate Search/Find acceptance additions before editing that file.

## Focused Replay Probe Correction

The first focused replay independently confirmed the saved resize discrepancy. Its initial new divider assertion also incorrectly required React’s generated separator ID to remain stable; React remounts that separator and changes the ID while correctly moving it 120 pixels. The assertion now resolves the same divider by kind, axis, span, requested position, and unchanged concrete neighboring window identities. WGPU retained its divider ID and still moved only about 1.2 pixels. The focus, unfocus, refocus, and individual close checks completed with physical geometry and identity assertions in both renderers. A corrected replay is required before recording the divider acceptance result.

## Pane Probe and Row-Density Finding

The initial pane assertion chose `action.engagementAbort`, but React places that row below the visible window band (y=990), while WGPU places it at y≈565. This probe must use controls actually visible in both hosts, so it now checks the first two selection rows, `action.clearSelection` and `action.selectAll`. Saved React action rows are 24 pixels high while WGPU rows are about 12.04 pixels high; this is a separate visual density discrepancy, not an opening failure. Both child presence and removal still require physical bounds within the selected window. The earlier 15b run remains diagnostic because its first pane assertion failed before closing that pane.

## Final Checkpoint 15 Diagnostic Verdict

- 15b: React passes the corrected 120-pixel divider check; WGPU fails it.
- 15b: both renderers pass focus, unfocus, refocus and individual close with concrete identity and body-bound assertions; close-all and template reopening also pass their existing physical checks.
- 15b: both renderers pass utility, projection, and measures pane open/close with required child-control publication and retirement.
- 15c: both renderers pass Actions and window Search pane open/close using the first two visible selection rows. Six steps complete with zero physical failures; the runner exits 1 because its separate action-ledger comparison differs for boot and tour dismissal. It is not an uncached task pass.
- All five sealed checkpoint 15 browser artifacts remain unchanged after these diagnostic replays. Current source fixes are not in those measured artifacts.

The next canonical checkpoint must rerun these assertions after the split, General, raster, and palette implementation packets are built. Tree row height and other visual differences remain open even where pane interaction passes.

## Probe Diagnostics Extension

The React diagnostic snapshot now captures the first eight mounted Tree rows with computed font, line height, padding, minimum/actual height, flex shrink, icon rectangles, and immediate ancestor scroll geometry. A React-only boot/dismiss/Actions-open run is queued at `🗑️generated/astra-runtime/tree-style-checkpoint-15`; this supports the Terra density audit without changing the measured WGPU artifact.

## Measured React Tree Metrics

The React-only diagnostic completed all three steps with zero failures and exit code 0. All first eight Actions rows have actual/minimum height 24 pixels, line height 24 pixels, padding 0, and 16 × 16 icons. Their immediate `tree-section-content` has height and scroll height 1344 pixels; it preserves content extent instead of shrinking rows into the visible pane. The row inherits 16-pixel Anta text, but leaf label styling must be checked separately before using that as a glyph-size oracle. Root spacing is 3.2 pixels and text-xs is 11.2 pixels. Exact compact capture: `🗑️generated/astra-runtime/tree-style-checkpoint-15/tree-style.json`.

## Actions Tree Scroll Oracle

The strengthened shared probe now asserts 24-pixel row height and pitch for Clear Selection/Select All, requires Abort to begin offscreen, uses a physical wheel to expose its rectangle, verifies the React hit at that point, and reverses the wheel to restore row geometry and clip Abort again. The React-only five-step replay passed with zero physical failures (tree-scroll-react-15). New WGPU acceptance remains pending activation.
