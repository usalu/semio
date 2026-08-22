# P10af Independent Dropdown and Collapsible Audit

## Verdict: REJECT

Audited independently on 2026-08-22. The source-level removal and most of the owned disclosure mechanics are present, and the focused quick suite passes. The packet nevertheless cannot be accepted: `bun.lock` still declares both removed packages in the *current workspace manifest stanza*, contrary to the target manifest; this is a lock/manifest inconsistency that the reported source-parity command does not inspect. The implementation and focused tests additionally leave required interaction assertions absent, and content `hidden` is still caller-overridable while open.

## Evidence

### DropdownMenu source removal: PASS

- The React target manifest has no `@radix-ui/react-dropdown-menu` dependency. The surrounding dependency range is `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:30-60`.
- The target barrel begins its Radix adapter imports with Dialog at `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:14-20`; there is no DropdownMenu import or alias there.
- A tracked-source scan scoped to `🧰️framework/🔨️modules/🖱️ui`, searching `@radix-ui/react-(dropdown-menu|collapsible)|DropdownMenuPrimitive|CollapsiblePrimitive`, produced zero hits. This establishes no live UI source API/import for DropdownMenu (or the retired Collapsible facade); ticket history and lock/freeze records were intentionally not treated as source API evidence.

### Lock and freeze parity: FAIL

- The checked-in target `package.json` does not contain either package (`…/⚛️react/package.json:30-60`), but `bun.lock:512-530` still records that same workspace package's `dependencies`, including `@radix-ui/react-collapsible` at line 521 and `@radix-ui/react-dropdown-menu` at line 523. This stanza is a workspace manifest snapshot, not merely an unused transitive resolution record.
- `git diff --numstat -- bun.lock` returned no row: the stale workspace snapshot was not updated in this packet.
- `🔒️dependencies.json:185-213` also still lists both identities with the React target manifest as their user. That file may be a baseline/freeze record, but it reinforces that the claimed inventory reduction is not reproduced by all checked-in dependency records.
- `bun ./📜️script.ts verify dependencies parity js --format json` exited 0 and reported 83 manifests, 272 external rows, 123 evidenced rows, 149 unowned rows, and no undeclared imports. Its success is not evidence of lock consistency: it does not flag the contradictory `bun.lock` workspace stanza above.

### Owned Collapsible state and public boundary: PARTIAL PASS

- Controlled/uncontrolled state, default state, and controlled proposal behavior are implemented in `…/↕️Collapsible/🟦️component.tsx:53-66`. Controlled calls do not mutate the internal state (`:61-62`), while uncontrolled calls do.
- The public barrel exports only the owned `CollapsibleProps`, `CollapsibleTriggerProps`, and `CollapsibleContentProps` at `…/⚛️react/📦️index.tsx:7973-7976`. The leaf contracts derive only from React DOM attributes (`…/↕️Collapsible/🟦️component.tsx:17-33`); no Radix-derived public type remains.
- React `useId` is used at `…/↕️Collapsible/🟦️component.tsx:55-57`, with a root `contentId` override. The trigger/content association and state attributes are written after caller props at `:111-123` and `:145`, so the generated content ID, trigger `aria-controls`, `aria-expanded`, and `data-state` cannot be overwritten.
- The multi-instance/stability test (`…/↕️Collapsible/🧪️component.test.tsx:195-237`) asserts distinct trigger/content IDs and stability after rerender. This is evidence for client rerender stability, but no SSR/hydration test exists; SSR safety is inferred only from the React `useId` primitive.
- **Hidden invariant failure:** `CollapsibleContent` destructures caller `hidden` and computes `hidden={!context.open || hidden}` at `…/↕️Collapsible/🟦️component.tsx:143-145`. A caller can therefore force an *open* content region hidden with `hidden={true}`. This contradicts the packet's claimed non-overridable owned `hidden` state. The override test only supplies `hidden={false}` while closed (`…/↕️Collapsible/🧪️component.test.tsx:63-84`), so it cannot detect this failing direction.
- The controlled test proves one proposal and a rerender (`…/↕️Collapsible/🧪️component.test.tsx:37-61`) but does not exercise repeated activations before a parent applies the controlled value. Callback semantics under controlled lag are therefore unproven.

### Trigger, Slot, disabled, and keyboard mechanics: PARTIAL PASS

- `asChild` uses `React.Children.only` at `…/↕️Collapsible/🟦️component.tsx:125-130`; the owned Slot independently validates exactly one valid element at `…/🏷️class-name-composition/🟦️slot.tsx:61-69`.
- Slot event composition is child-first and honours `preventDefault()` at `…/🏷️class-name-composition/🟦️slot.tsx:39-58`; it composes child and forwarded refs at `:27-37,66-69`. Focused tests exercise event order, cancellation, and refs at `…/↕️Collapsible/🧪️component.test.tsx:133-172`.
- Runtime-owned trigger state/association attributes and disabled/tabbability are written at `…/↕️Collapsible/🟦️component.tsx:111-127`; native buttons receive native `disabled` and `type="button"` at `:133-136`.
- Non-native Enter and Space activation is manually handled at `…/↕️Collapsible/🟦️component.tsx:90-105`, while the native button path relies on the synthesized click (`:92,102`). The focused test verifies a manually dispatched key/click sequence (`…/↕️Collapsible/🧪️component.test.tsx:110-131`), not a browser keyboard activation sequence; it is useful unit coverage but not a complete native-event proof.
- Root-disabled native-button suppression is asserted (`…/↕️Collapsible/🧪️component.test.tsx:174-193`). There is no trigger-level-disabled test and no disabled `asChild` test. In particular, the disabled `handleClick` path invokes a slotted child's own click handler and does not prevent its default action (`…/↕️Collapsible/🟦️component.tsx:86-89`); an `asChild` anchor could still navigate. Disclosure state itself is suppressed by `activate` (`:83-85`), but complete disabled-host semantics are unproven.

### Tree branch integration: PARTIAL PASS

- The production branch row remains an `asChild` non-native trigger at `…/🪵️Tree/🟦️component.tsx:1578-1631`, inside a controlled `Collapsible` at `:1642-1655`. The Tree test covers its controlled association, closed hidden state, rerender, and Enter proposal (`…/🪵️Tree/🧪️component.test.tsx:8-38`).
- Child action clicks explicitly prevent default and propagation at `…/🪵️Tree/🟦️component.tsx:624-641`; the checkbox action does the same at `:597-621`. This is source evidence only: the sole new Tree test has no `actions` fixture, so no assertion verifies that a child action leaves the branch unchanged.
- The row keeps drag handlers and double-click handling at `…/🪵️Tree/🟦️component.tsx:1588-1605`, but the focused Tree test does not dispatch drag or double-click events. No test establishes that these paths preserve disclosure/callback semantics. A browser double-click conventionally comprises two clicks before `dblclick`; the trigger's click handler at `…/↕️Collapsible/🟦️component.tsx:86-89` will propose/toggle for those clicks before Tree's `onDoubleClick` handler runs. The required double-click interaction is therefore neither proven nor explicitly isolated.

## Executed Gates

| Gate | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 12 files, 605 tests. Existing `NO_COLOR`/`FORCE_COLOR` warnings only. |
| `bun ./📜️script.ts verify dependencies parity js --format json` | PASS — source/manifest scanner reports 0 undeclared imports, but does not validate the stale `bun.lock` workspace manifest. |
| Static tracked UI identity scan | PASS — no current UI source hit for either retired Radix module or primitive alias. |

## Unrun Gates

- No `bun install`, lockfile regeneration, frozen-lockfile install check, or dependency mutation ran; the packet disallows installs and free disk was only about 1.5 GiB. The stale lock conclusion is from direct static inspection.
- UI typecheck, lint, primitive check, format check, full repository verification, browser/Storybook tests, production build, exhaustive suite, and all Rust/Cargo gates were not rerun in this independent audit.
- No ticket metadata, status, important marker, source, manifest, lockfile, or cache was changed by this audit.

## Required Before Acceptance

1. Reconcile the workspace dependency snapshot in `bun.lock` (and the repository's dependency-freeze policy/record) with the target manifest using the authorized lock maintenance process, then demonstrate the relevant frozen-lock consistency check.
2. Make content visibility wholly state-owned, or explicitly revise the contract and prove the chosen behavior; add the currently missing `defaultOpen hidden={true}` assertion.
3. Add focused controlled-lag, trigger-level/`asChild` disabled, child-action, drag, and double-click Tree tests. The latter must assert disclosure/callback behavior, not only that the incidental handler executes.

## Repair Disposition: AUDIT READY

Disposed on 2026-08-22 without touching Tabs, ToggleGroup, Popover, Slider, Rust, compose, ticket metadata, status, or important markers.

### Lock and freeze parity: RESOLVED

- After a `df -h .` reading of approximately `1.5GiB` free, `bun install --lockfile-only --ignore-scripts --no-progress --no-summary` reconciled the configured live workspace snapshots without installing workspace packages. `bun.lock` changed by exactly 32 deletions.
- The UI workspace snapshot no longer declares `@radix-ui/react-collapsible` or `@radix-ui/react-dropdown-menu`. Neither identity has a package resolution, and the solely DropdownMenu-owned `@radix-ui/react-menu` resolution is gone.
- A frozen repeat, `bun install --lockfile-only --frozen-lockfile --ignore-scripts --no-progress --no-summary`, exited successfully.
- `verify dependencies parity js` now compares all configured in-scope Bun workspace dependency/dev/optional/peer tables with their live manifests and rejects missing, stale, version-drifted, absent-workspace, or invalid-lock states. Five executable inline fixtures cover the clean and four mismatch directions on every parity run.
- Final parity: 83 manifests, 272 external rows, 123 evidenced rows, 149 unowned rows, 0 undeclared imports, 44 configured in-scope lock workspaces, 0 lock mismatches, and 5 passing lock fixtures.
- `🔒️dependencies.json` was deliberately preserved. It is the historical 238-identity new-dependency ratchet, not a live workspace snapshot: removals remain as allowance and any new live identity still fails. Deleting retired rows from it would lower the baseline and weaken the intended detection policy.

### Content visibility and trigger semantics: RESOLVED

- `CollapsibleContentProps` omits `hidden`, and runtime-owned `hidden={!context.open}` is applied after all host props. The regression test spreads `hidden={true}` into open/default-open content and proves it remains visible.
- Controlled-lag coverage performs three repeated activations while `open={false}`, proves three `true` proposals with unchanged rendered state, applies `open`, and proves the next proposal is exactly `false` without local mutation.
- Trigger-level native disabled behavior and disabled `asChild` anchor behavior cover pointer, Enter, Space keydown/up, `aria-disabled`, `data-disabled`, `tabIndex=-1`, default prevention, callback suppression, and unchanged disclosure state.

### Tree integration: RESOLVED

- A production `TreeSection` child-action click executes the child callback once while leaving disclosure callback/state unchanged.
- Native drag start/end events execute the row drag callbacks while leaving disclosure callback/state unchanged.
- Expandable rows with a double-click action defer pointer single-click activation for 300ms. A second click cancels pending disclosure before `dblclick`; the double-click handler runs once with zero disclosure callbacks and unchanged controlled branch state. An isolated single click still emits exactly one delayed open proposal. Keyboard disclosure remains immediate.

### Repair verification

| Gate | Result |
| --- | --- |
| Focused UI format check | PASS. |
| UI typecheck | PASS; successful task was labeled flaky by Nx. |
| UI quick tests | PASS — 12 files, 608 tests. |
| UI lint | PASS. |
| UI primitive check | PASS — no violations, two existing allowlisted files. |
| JS dependency list | PASS — 87 identities, neither retired identity present. |
| JS source/manifest parity plus lock parity/self-fixtures | PASS — exact counts above. |
| Frozen lockfile-only consistency | PASS. |
| Exact live source/manifest scan | PASS — zero retired identities or primitive aliases. |
| Exact `bun.lock` scan | PASS — zero retired identities. |

The full repository verify suite, exhaustive/long UI tests, Storybook/browser execution, production builds, and every Rust/Cargo gate remain unrun because they are outside this repair and unsafe or irrelevant under the approximately `1.4GiB` remaining-disk constraint.
