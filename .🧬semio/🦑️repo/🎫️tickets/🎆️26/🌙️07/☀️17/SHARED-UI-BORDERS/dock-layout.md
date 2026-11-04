# React Dock/Layout Shared Boundaries

## Result

Implemented physical adjacency in the assigned Canvas/Resizable scope. Layout was inspected and already places navbar, canvas and footer in consecutive flow rows without reserved floating-panel columns; no Layout source edit was needed.

- Canvas root and Mode body now use zero padding; HorizontalWindows/VerticalWindows use zero gaps.
- Dock-stack body no longer adds a padding-standard inset around its macro window.
- Splitters consume zero layout pixels. Existing primitive hit testing still supplies a centered 20px fine-pointer / 28px coarse-pointer invisible region; keyboard separator semantics remain intact.
- Split drop targets now measure the whole stack, including chip rows. Existing half-frame previews and full-axis resize percentage calculations therefore use the same physically allocated extent.
- Corner grab markers are noninteractive. The existing window-capture corner interceptor finds their measured rectangles instead, preserving dual-axis resizing and cancellation without placing hit-test overlays above window controls.
- Window-capture target tracking makes the primitive's expanded hit region yield to semantic controls, links, inputs, editable content and drag handles. No pointer event is swallowed or synthetically replayed for controls.
- No shared-border DOM attributes were added: docked WindowChrome instances already register their true silhouettes.
- WindowChrome, silhouette geometry, CSS, controls, menus, detached floating Panel spacing and floating drag-preview decoration were not modified.

## Integration Findings

The installed react-resizable-panels implementation installs its pointerdown listener on document capture and normally accepts any target contained by its group. Merely using zero-width separators with the existing expanded hit region demonstrably canceled pointerdown on nearby window buttons. A bubble handler cannot fix that ordering. Resizable now records the target on window capture and exposes dynamic minimum-size getters, so control targets reduce expansion to zero before the primitive reads it. This stays behind the existing Resizable boundary and introduces no dependency or external API.

The compositor API is unchanged. The internal `ModeStackDropTargets.body` field now intentionally carries full stack bounds; a native docstring records that meaning. Tab-corner targets still win before split targeting. The active/effect silhouette and chip measurements are untouched.

## Changed Files

Under `🧰️framework/🔨️modules/🖱️ui/🧱️elements/`:

- `↔️Resizable/🟦️.tsx`
- `🎨️Canvas/🟦️.tsx`
- `🎨️Canvas/⚓️dock/🟦️.tsx`
- `🎨️Canvas/🎯️drop/🟦️.ts`
- `🎨️Canvas/🧪️tests/dock-cases.schema.json` (new)
- `🎨️Canvas/🧪️tests/dock-cases.json` (new)
- `🎨️Canvas/🧪️tests/shared-border-dock.test.tsx` (new; matches the lead's shared-border test discovery)

Ticket-local `dock.config.ts` is the isolated validation configuration; shared configurations were not changed.

## Validation

Test-first failures reproduced padding/gap/inset allocation, nonzero separator width, control pointerdown cancellation, and corner overlay interception before the corresponding changes.

Final focused command:

```text
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light vitest -- run --config '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/SHARED-UI-BORDERS/dock.config.ts' -t 'flush dock boundaries|Layout panel overlay|Layout subfooter row|stack drop destination'
```

Result: **3 files passed; 28 tests passed, 3 skipped**. This includes existing Canvas stack-drop vectors and Layout panel-overlay/subfooter regressions, real Layout/Canvas/Mode rendering, both splitter axes, centered hit-region boundaries, control ownership, corner movement/cancellation, JSON schema validation, half-frame previews and resize percentages. Existing Layout browser/native-ownership cases were outside this focused selection (two skips). The new Playwright geometry oracle explicitly skips because Chromium is not provisioned (one skip); no browser was installed.

Neutral fixture resize/boundary results are independently reproduced with existing test-only Decimal.js arithmetic. Actual pointer dispatch uses the installed third-party resizable primitive, not a mock. Ajv validates the neutral fixture schema. A Playwright flex-layout comparison is ready to execute when Chromium is available.

Focused strict TypeScript check of Resizable and its imports passed:

```text
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light tsc -- --noEmit --target esnext --module esnext --moduleResolution bundler --jsx react-jsx --strict --esModuleInterop --skipLibCheck --allowImportingTsExtensions '🧰️framework/🔨️modules/🖱️ui/🧱️elements/↔️Resizable/🟦️.tsx'
```

Scoped final diff inspected; `git diff --check` passed. No modifying git commands, installs, service/configuration changes or generated output files were created by this assignment.

## Shared Suite Integration Follow-up

Additional ownership was granted for `ui/🧪️tests/🧪️owned-locale-detector-retirement/📐️shell-components-4/🟦️.tsx`. Updated the obsolete horizontal/vertical gutters, canvas inset and stack-body padding expectations to the flush contract. Corner assertions now check nonintercepting markers, actual minimum 20px dimensions and populated join specifications rather than requiring a cursor class on a pointer-transparent marker.

Fixed the corner drag regression in Resizable: explicit pointer dispatch to a registered corner now resolves that target directly; pointer-transparent browser hit testing still uses measured corner rectangles. The previous implementation required measured rectangles even for direct corner events, breaking the mounted Mode drag test where jsdom has no corner layout. Control priority and geometric pane-based corner detection are unchanged.

The existing `shell-components-5` mounted-group drag test was **not modified**: it now passes its original assertions that root, left and right group sizes all change to 55/45 and `onLayoutChange` fires. This is a functional source fix, not a relaxed drag assertion.

Lead configuration, exact shared-suite file selected to avoid unrelated Bun-only suites:

```text
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light vitest -- run --config '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/SHARED-UI-BORDERS/vitest.config.ts' '🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx' -t 'control chrome|Shell components'
```

Result: **250 passed, 315 excluded by name filter; no failures**. The narrower `Mode.*(gutter|corner|tab stack)` run also passed all **10 selected tests**. Dock-focused regression rerun remains **28 passed, 3 skipped**.

Running the lead config without an exact file selector additionally discovers two existing `bun:test` suites in WindowChrome/WindowSilhouette that cannot initialize under Vitest; no changes to those outside-owned files were made.

## Remaining Verification

Run the new browser oracle and visually verify navbar/name-chip contact, shared seams and resize cursors once a browser is provisioned. jsdom verifies composition and event ownership, not final CSS pixels. Full repository type checking and unrelated UI suites were not run. Ticket lifecycle remains owned by the lead; this assignment did not open/close the shared ticket.
