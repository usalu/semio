# Shared Boundary Design

## Decision
Keep existing silhouette outlines and chip geometry as the source of truth. Resolve coincident axis-aligned outline segments rather than removing component borders globally. Split partially overlapping collinear segments at their endpoints and paint each interval once. Hover is the highest boundary priority, independent of DOM order; existing active and transient border effects remain otherwise ordered as before.

## Parallel Ownership
- Border foundation: pure segment contract/resolver, neutral fixtures, canonical CSS and chrome presentation constants.
- Silhouette integration: WindowSilhouette geometry and WindowChrome runtime measurement/compositing, with scope isolation and cleanup.
- Controls: buttons, toggles, action groups, navigation/footer/ribbon and panel tab adoption; preserve keyboard/ARIA semantics.
- Menus: context-menu fusion/placement and Popover attachment/portal behavior.
- Lead: architecture/API integration, cross-component acceptance and final regression validation.

## Boundary Contract
`border-core-api.md` records the approved pure segment API. `data-shared-border-frame` marks participating control frames; `data-shared-border-attached` lists physical sides only when geometric attachment is established. Bar boundary descriptors and runtime registration must agree before integration is accepted. A detached or collision-flipped surface must keep a complete outline.

## Risks to Validate
- CSS and inset shadows currently draw competing separators.
- Broad chip-child border reset must not erase unrelated child outlines.
- Nested silhouettes must not donate their chips to enclosing geometry.
- Scaled roots and scroll must keep strokes aligned; no perpetual animation-frame polling.
- Floating surfaces must remain in their shell's portal host and escape clipping/neighbor stacking.
- Hover must win whether the hovered participant is earlier/later or parent/child.

## Infrastructure
The repo build launcher is repaired, with unchanged lockfiles. Native MCP ticket lifecycle is unavailable; details are in `bootstrap.md`. This directory is a report location, not a falsely claimed registered ticket.
