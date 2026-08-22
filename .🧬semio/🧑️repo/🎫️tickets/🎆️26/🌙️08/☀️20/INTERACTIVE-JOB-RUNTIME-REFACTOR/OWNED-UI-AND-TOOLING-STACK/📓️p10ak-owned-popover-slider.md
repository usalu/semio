# P10ak Owned Popover and Slider

## Verdict

PASS and ready for independent audit. The live `@radix-ui/react-popover` and `@radix-ui/react-slider` implementations, manifest declarations, lock resolutions, and the Slider-only root override are removed. The historical `🔒️dependencies.json` baseline remains unchanged. The dependency ratchet is exactly 146 current identities, matching the serialized packet target.

## Implementation

### Popover

- Replaced the Radix wrapper with an owned controlled/uncontrolled/default-open root and owned public component/event/placement types. No exported type references a third-party contract.
- Trigger and anchor support owned `asChild`, forwarded/composed refs, child-first preventable activation, disabled trigger suppression, and state-owned `aria-expanded`, `aria-controls`, `aria-haspopup`, and `data-state`.
- Content is owned by a `document.body` portal and is absent while closed, so hidden descendants cannot mount effects. The open content uses a focusable nonmodal `role="dialog"` without `aria-modal`.
- Implemented preventable open/close autofocus, Escape, outside pointer/focus/interact dismissal, trigger focus return, and logical nested-popover boundaries. Escape dismisses only the deepest/topmost open popover; interaction in a nested portal is inside its ancestor's logical boundary.
- Added measured fixed placement for top/right/bottom/left, start/center/end, side/align offsets, RTL inline alignment, main-axis flip, viewport clamp, configurable collision padding, and remeasurement on content/anchor resize, viewport resize, and capturing scroll.
- Repaired the real `ActionDropdown` consumer so its native `ActionGroupItem` is the slotted trigger and no interactive control contains another interactive control.

### Slider

- Replaced Radix with owned `SliderProps`, `SliderValue`, range, direction, and orientation contracts plus exported normalization/draft helpers.
- Implemented controlled-lag drafts and uncontrolled/default tuples; finite range normalization; invalid/inverted range and nonpositive step handling; step snap/clamp; one or multiple sorted thumbs; crossing by stable sorted tuple; and optional minimum-step spacing.
- Implemented nearest-thumb pointer targeting, pointer capture, move, up, and cancel rollback; change publication during a gesture; and exactly one commit on a changed release. Cancellation restores the gesture-start tuple and does not commit.
- Implemented arrows, PageUp/PageDown, Home/End, RTL, vertical orientation, inverted direction, keyboard gesture coalescing, Escape cancellation, disabled/read-only suppression, and per-thumb `aria-valuemin`, `aria-valuemax`, `aria-valuenow`, `aria-orientation`, label, disabled, and read-only state.
- Preserved the existing loading/waiting shell, editable readout, ready-extent presentation, optional hard ready clamp, snap values, and interaction command hooks.

## Focused Runtime Evidence

- The new Popover matrix has 10 real-DOM tests covering state ownership and controlled lag; trigger associations; `asChild` ref/event composition; disabled activation; autofocus prevention; Escape and focus return; pointer/focus prevention; nested outside boundaries and topmost Escape; closed descendant effects; every side/alignment/offset combination; RTL alignment; flip/clamp; ResizeObserver anchor updates; and the real `ActionDropdown` DOM/selection path.
- The new Slider matrix has 9 real-DOM tests covering invalid numeric inputs and helpers; controlled lag; uncontrolled repeated-key commits; multiple-thumb crossing/minimum spacing; pointer capture/move/up; cancel rollback; RTL/vertical arrow, page, Home, and End mappings; disabled/read-only ARIA; and ready presentation versus hard clamp.
- The complete UI quick suite exercises the established real Search and Toggle consumers alongside the new matrices: 16 files and 641 tests passed.
- The renderer quick suite now mounts the real `SyncAttachCard`, verifies its Popover is a focused nonmodal dialog, and dismisses it with Escape: 4 files and 438 tests passed. The first draft incorrectly fixed `data-side="top"` under jsdom's zero-size rectangles and leaked its mount into the monolithic suite; the assertion was corrected to test semantic behavior while placement is covered by deterministic geometry tests, cleanup was added, and the final suite passed.

## Dependency and Lock Evidence

- Free space was checked before reconciliation: 2.4 GiB was available. `bun install --lockfile-only --ignore-scripts` passed without running lifecycle scripts or installing packages.
- The frozen repeat, `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile`, passed with 2.3 GiB available.
- The serialized shared `bun.lock` diff against HEAD is exactly 0 additions and 51 deletions. The prior packet established 44 deletions; this packet adds seven deletion lines: the two workspace declarations, the unique root Slider override, and the two resolution rows with their separating blank rows. Neither retired identity remains in the live lock.
- `bun ./📜️script.ts verify dependencies list js` passed and emitted neither retired identity. The live JavaScript identity inventory is 83 (the prior 85 minus Popover and Slider).
- `bun ./📜️script.ts verify dependencies` passed: 238 historical baseline identities, 146 current cross-ecosystem identities, 92 allowed removals, and no new dependency.
- `bun ./📜️script.ts verify dependencies parity js --format json` passed: 83 manifests, 268 external rows, 119 evidenced rows, 149 advisory-unowned rows, zero undeclared imports, zero lock mismatches, 44 lock workspaces, and five passing lock fixtures.
- The existing serialized manifest/source audit passed: 64 manifests, 580 direct rows, 268 external rows, and 75 rows without owned-scope evidence.
- Exact live source/manifest/barrel/lock scans found none of `@radix-ui/react-popover`, `@radix-ui/react-slider`, `PopoverPrimitive`, or `SliderPrimitive`. The only two identity matches in the repository-wide non-compose scan are the intentionally unchanged historical baseline rows.
- Targeted `git diff --check` passed.

## Executed Gates

| Gate | Result |
| --- | --- |
| Focused `nx format:write` then `nx format:check` over owned leaves/tests and touched consumers/barrel/config/manifests | PASS |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 16 files, 641 tests |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS — existing Bun colour warning only |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache` | PASS — no violations, two existing allowlisted files |
| `bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache` | PASS — 4 files, 438 tests |
| Lock-only reconciliation and frozen lock-only repeat | PASS |
| Dependency list, freeze ratchet, and JavaScript parity | PASS — 146 current identities; zero undeclared imports/lock mismatches |
| Existing Phase 10 manifest/source audit | PASS — 64 manifests, 580 direct rows, 268 external rows, 75 advisory no-owned-scope-evidence rows |
| Live identity/alias scan, raw-lock scan, lock diff inspection, targeted whitespace check | PASS |

## Known Broad Gate and Intentionally Unrun Gates

- `bun nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache` was run and remains red on its established cross-package backlog (956 diagnostics), beginning in the demonstrator tutorial snapshot/config schema and continuing through Infinite World Three/R3F typing, renderer host contracts, shell schema, and worker globals/envelopes. The owned UI typecheck passes, the edited renderer test compiles and executes in the 438-test renderer quick suite, and no reported renderer-wide diagnostic points to Popover, Slider, ActionGroup, or the new consumer assertion.
- No browser/Playwright, Storybook, production build, exhaustive UI or repository suite, full monorepo format/typecheck/lint, package-installing command, lifecycle script, cache deletion, or production-browser geometry/focus run was performed. jsdom and deterministic measured-geometry coverage are not represented as a production-browser claim.
- No Rust/Cargo gate ran and no Rust, compose, Dialog, Select, ticket metadata, or Git state was mutated.
- Repository ticket lifecycle MCP was unavailable as declared by the coordinator; this source/report packet used the already-open ticket and did not manually alter ticket JSON, status, or importance metadata.

## Changed Paths

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
- `package.json`
- `bun.lock`
- this report
