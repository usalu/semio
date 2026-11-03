# Catalogue and Tree Connector Termination

## Result

Implemented in the real Tree connector renderer; Catalogue inherits the repair through its existing Tree composition. No story masks, shared styling changes, property-row/Label geometry changes, Scene changes, generators, server restarts or builds. Existing Tree drag/drop story edits are untouched. MCP remains unavailable; no lifecycle operation was fabricated.

## Root Cause and Repair

- `IndentationLines` painted the immediate parent guide across the entire branch, including below its final item's elbow. `TreeItem.isLastItem` was not passed to the actual row gutter, so that full-height guide could not terminate there.
- Nested guide selection used the ancestor flag one level too early, effectively consulting the containing section instead of the item whose sibling line it was drawing. Drawing all ancestor indices at every nesting level also overpainted inherited ancestor lines.
- Each row now owns its incoming parent guide: non-final siblings retain a full guide; the final sibling ends exactly at the existing elbow anchor. Expanded final groups retain their separate outgoing child stem.
- A branch contributes only its owner's non-final sibling continuation. Outer branches already cover earlier ancestors, avoiding duplicate translucent guides.
- Reversed/upward lists mark their first displayed row as the logical endpoint. Virtual windows do not mistake the end of a materialised slice for the last logical sibling; their spacers retain the connector and terminate at the final virtual row's center.
- Existing explicit `TreeItem.isLastItem` semantics remain; manually composed TreeItems should continue publishing that flag. Data-backed Tree and Catalogue compute it automatically.

## Exact Changed Files

All source/test paths below are relative to `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/`:

- `🎨️chrome/🟦️.tsx`: branch guide ownership/ancestor termination, inspectable guide index.
- `📏️row-alignment/🟦️.tsx`: connector-only `TreeDocumentGutter` and `TreeAlignedRow` prop plumbing. No metrics, grid/Label/property geometry definitions changed.
- `🍃️item/🟦️.tsx`: pass the final-sibling flag for default and property layouts.
- `↕️sortable-item/🟦️.tsx`: identical flag plumbing for sortable rows.
- `🪢️views/🟦️.tsx`: directional/logical final-sibling calculation, including window tails.
- `🔭️virtual-window/🟦️.tsx`: preserve parent connector within virtual spacers.
- `🧪️tests/🧩️component/🟦️.tsx`: 12 focused regression tests.
- `🧪️tests/🧫️fixtures/🌿️connectors/🔣️.json`: language-neutral nested sibling/ancestor corpus.

Ticket inputs added: `test-connectors.ts` and this report.

## Validation

- Test-first: four actual Tree/Catalogue connector tests failed before the repair; the independent hierarchy oracle passed.
- Added window regression first: both directional spacer checks failed before spacer preservation.
- Added effective-ancestry assertions first: both directional tests exposed duplicate ancestor overpainting before branch ownership was corrected.
- Final complete Tree component suite: **71 passed**, including **12 new connector tests** and all 59 existing tests.
- New coverage: one/multiple Catalogue items, insert/remove/empty transitions, nested last/non-last ancestry with both final/non-final containing sections, up/down direction, default/property/sortable expanded groups, hidden connector mode, and virtual slices at both logical ends.
- The existing root dev dependency `d3-hierarchy` independently produces sibling-finality and ancestor-guide output matching the JSON corpus. No runtime dependency was added.
- Tree diff whitespace check passed. New fixture/runner whitespace and path-length checks passed.
- Runner outputs/cache stay under the ticket's `🗑️generated/connectors` and are removed in `finally`; unrelated live inspection outputs were preserved.

Reproduce from the repository root:

```sh
PATH=$HOME/.local/share/semio/launcher/bin:$PATH repo tools exec --light bun -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FEATURE-COMPLETE-COMPOSABLE-STORYBOOK-COVERING-THE-MONOREPO/test-connectors.ts'
```

Append `--testNamePattern='Tree connector termination'` for only the 12 focused checks.

## Lead Browser Acceptance

The running 6010 static server is still the OLD read-only build. Browser geometry acceptance/rebuild belongs to the lead and is not claimed here.

- **Catalogue / Default** and **Catalogue / Palette**: expand the section. `[id="file"]` retains its guide through the row; `[id="example"]` ends at its elbow, with no tail below. Empty has no connector strokes.
- **Tree / Default**: expand Kit → Types → Capsules → Variants/Balcony, and Types → Bases. Capsule P, With Balcony L and Base Blob end at their elbows. Expanded final sibling Balcony has no Variants-level continuation through its children, while Capsules' non-final continuation toward Bases remains. Kit's non-final containing section must not force a phantom item continuation.
- **Tree / Alternatives**: switching branches should recompute the final sibling.
- **Tree / WindowedSection**: connector continues across blank virtual spacers, not an artificial end at the current slice boundary.
- Preserve the existing **Tree / DragAndDrop** interaction acceptance.

Useful selectors:

- `[data-slot="tree-parent-guide"]`: incoming per-row (or virtual spacer) vertical connector.
- `[data-slot="tree-branch-elbow"]`: existing horizontal elbow and anchor.
- `[data-slot="tree-branch-stem"]`: outgoing expanded child stem, intentionally retained for a final group.
- `[data-slot="tree-guide"] [data-tree-guide-index]`: branch-owned continuation; inherited ancestor lines belong to outer branch containers, not duplicated inner containers.

For a final downward row, the parent guide's bottom must equal its elbow anchor (normally row center), rather than the row bottom. Upward growth mirrors this at the guide's top. Inline positioning still uses existing logical CSS properties; no fixed left/right positioning was introduced.
