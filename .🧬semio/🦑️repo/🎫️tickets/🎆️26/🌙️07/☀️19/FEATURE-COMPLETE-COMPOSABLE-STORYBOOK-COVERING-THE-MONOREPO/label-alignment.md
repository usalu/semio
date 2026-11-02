# Label Alignment

## Implementation

Only Label-owned files changed; shared Tree property geometry and connector logic remain untouched.

Paths relative to `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🏷️Label/`:

- `🟦️.tsx`: measures intrinsic caption scroll/bounding width plus tree padding, not allocated wrapper widths. Invisible, inaccessible absolute metrics resolve actual CSS label-track/control-minimum widths, including inherited custom properties and calc expressions. Default minimum reserves one label-track width (96px compact) for a short editable value; callers can override inherited `--property-control-min-width`. Computed column gap respects customization. Inline track grows for translated captions. Existing shared hysteresis controls return to inline. Observes caption/metric size changes and remeasures resolved locale text changes. Inline label wrapper is flex (no extra inline-span baseline box), row and control are vertically centered; stacked control has no retained value-column padding. `propertyLayout="auto" | "inline" | "stacked"` supports automatic and intentional forced modes. Native `label[for]` links matching IDs; caption click also focuses the contained control. Uses the same ID fallback caption as group headers.
- `📐️layout/🟦️.ts`: pure platform-neutral fit contract.
- `🧪️tests/🔣️.json`: eight language-neutral vectors: actual 280px baseline, narrow, long translated caption, wider translated caption, both hysteresis states and both forced modes.
- `🧪️tests/🟦️.test.tsx`: neutral-vector oracle and actual React component regressions, including nested Tree context and accessible input association.

## Validation

Test-first run failed on the absent layout implementation. Final command (through approved repo launcher):

`repo tools exec --light bun -- x --no-install vitest run '🧰️framework/🔨️modules/🖱️ui/🧱️elements/🏷️Label/🧪️tests/🟦️.test.tsx'`

**4 tests passed**, including all eight neutral vectors. Independent arithmetic oracle: existing third-party `@csstools/css-calc`, using CSS max/addition. Actual components use Testing Library/jsdom with explicitly mocked geometry; wrapper scroll widths deliberately report 1000px to catch the original allocated-width mistake. Verified label click focus, native accessible name, resize transitions, hysteresis, translated caption remeasurement, forced modes, customization class preservation and nested tree stacked geometry. Inspected final scoped diff and untracked test/helper files. No builds, generators, servers, installs, modifying git commands or shared geometry edits.

## Lead Browser Acceptance

Within Label default story iframe:

- `[data-slot="property-row"][data-property-layout="inline"]` at 280px, with Width + Input on the same row.
- `[data-slot="property-label-inline"]` flex wrapper; compare vertical centers of `[data-slot="property-label"]` and `[data-slot="input"]` (expected equal within subpixel rounding).
- `[data-slot="property-control"]` has no `padding-inline-start`, including stacked mode.
- `[data-slot="property-layout-metrics"]` must remain invisible and outside grid flow.
- Default compact short-caption threshold: 96 + 96 + computed gap (currently 3.2) = 195.2px. At 180px stacked; while expanding, retains stacked within shared hysteresis, then returns inline. Repeat resizing to ensure no oscillation.
- A 200px intrinsic translated caption needs approximately 299.2px inline; 280px stacks, 400px returns inline.
- Tree-group-header branch remains unchanged. Nested property caption retains intentional tree-depth padding, but stacked control has no artificial 102.4px value-column indentation.
- Verify clicking Width focuses the Input and label association remains usable.

## Remaining Work / Risks

Lead owns real-browser geometry acceptance and integration build. jsdom cannot validate actual CSS intrinsic grid sizing, screenshots or fonts. No browser acceptance is claimed. MCP unavailable; existing ticket lifecycle remains lead-owned. No temporary output files were created and no live generated/static-inspect files were removed.
