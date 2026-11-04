# Border Core API

Approved and implemented exact exports from border-presentation/🟦️.ts:

- `BorderState = "normal" | "focus" | "active" | "waiting" | "loading" | "introduced" | "celebrated" | "hover"` (ascending priority; hover highest).
- `BorderPoint = { readonly x: number; readonly y: number }`.
- `BorderSegment = { readonly id: string; readonly owner: string; readonly from: BorderPoint; readonly to: BorderPoint; readonly state: BorderState }`.
- `ResolvedBorderSegment` extends BorderSegment with `readonly contributors: readonly string[]` (sorted source IDs).
- `resolveBorderSegments(segments: readonly BorderSegment[]): ResolvedBorderSegment[]`.

Axis-aligned finite coordinates only, exact coincidences (caller snaps measured coordinates); reversed endpoints normalize. Zero-length segments are omitted; diagonals/nonfinite coordinates and duplicate IDs throw. Output splits at every collinear endpoint, sorted horizontal before vertical then coordinate/start; winning highest state then lexically smallest owner then id. Output id is winning source id. Perpendicular intersections do not merge. Inputs are immutable.

CSS adoption contract: `data-shared-border-frame` identifies a frame whose direct `.shared-border-cell` or `[data-shared-border-cell]` children use a single logical-end border (vertical frames/groups use block-end); `data-shared-border-attached` value is a space-separated list of physical sides whose paint becomes transparent only after geometry takes ownership. Border widths remain unchanged. `data-shared-border-state` selects explicit paint state; resolved SVG stroke uses `data-shared-border-segment` and its state. Chip wrapper reset is narrowed to the wrapper itself; children retain frames until measured ownership. Navbar bottom/footer top pseudo-strokes also become transparent on explicit ownership. First CSS layers reserve important ownership suppression then hover priority, ahead of Tailwind and unlayered active overrides.

## Tooling

Initial `repo --help` failed; lead supplied functional absolute `/home/clstb/.local/share/semio/launcher/bin/repo`. All shell validation uses its `tools exec --light` interface. Ticket lifecycle belongs to lead/setup.
