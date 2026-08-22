# P10ae Owned Dropdown and Collapsible

## Outcome

Packets 1–2 are implemented without touching the held Rust lanes or the Tabs, ToggleGroup, Popover, Slider, and compose surfaces.

- Removed the unused `DropdownMenuPrimitive` target-barrel import and the sole direct `@radix-ui/react-dropdown-menu` manifest row after an exact live-source scan proved that no DropdownMenu API existed.
- Replaced the `@radix-ui/react-collapsible` facade with a repository-owned React disclosure implementation and removed its direct target-manifest row and target-barrel import.
- Reduced the live JavaScript third-party identity inventory from 89 to 87.
- Preserved the existing Tree `CollapsibleTrigger asChild` integration without changing Tree production source.
- Exported owned `CollapsibleProps`, `CollapsibleTriggerProps`, and `CollapsibleContentProps` contracts from the React public barrel; no third-party prop type crosses the public boundary.

## Exact Source Evidence

The pre-edit live scan excluded `compose`, `node_modules`, and ticket history. It found `@radix-ui/react-dropdown-menu` only in:

1. The unused import in the React target barrel.
2. The direct dependency row in the React target manifest.
3. Historical/generated dependency records (`bun.lock` and `🔒️dependencies.json`), which are not live source APIs.

The equivalent Collapsible scan found only the React target-barrel import, target-manifest row, and the leaf facade. The only production consumer of the leaf API was Tree.

The final exact UI-source scan, excluding `node_modules` and Rust build targets, returned no match for any of:

```text
@radix-ui/react-dropdown-menu
DropdownMenuPrimitive
@radix-ui/react-collapsible
CollapsiblePrimitive
```

`bun.lock` was intentionally not regenerated because this packet explicitly prohibited installs. Its resolved package records, and the committed dependency-freeze baseline in `🔒️dependencies.json`, remain historical records rather than live direct manifest declarations.

## Owned Collapsible Contract

The owned component provides:

- Controlled `open` and uncontrolled `defaultOpen` state.
- `onOpenChange` proposals without mutating controlled state.
- Root- and trigger-level disabled suppression, native `disabled`, `aria-disabled`, and `data-disabled` where applicable.
- Stable React-generated content IDs, an optional root `contentId` override, unique IDs across multiple mounted roots, and exact trigger `aria-controls` association.
- `aria-expanded` and `data-state="open|closed"` on triggers, `data-state` on root/content, and mounted content using the native `hidden` semantic while closed.
- Owned state/association attributes applied after host props so callers cannot override `data-state`, `hidden`, or the content ID accidentally.
- The existing exactly-one-child `asChild` trigger path through the owned Slot implementation, including merged classes, composed refs, child-first handlers, and child cancellation through `preventDefault()`.
- Manual Enter/Space activation only for non-native slotted hosts such as Tree's `div[role=button]`. Native buttons retain browser keyboard-to-click behavior, preventing the double-toggle risk caused by synthesizing state changes in both keyboard and click handlers.
- Ref forwarding for the root, default/slotted trigger, and content.
- No rendered or accessible labels are introduced by the primitive, keeping the implementation localization-neutral and host-customizable.

## Focused Proof

Two focused test files were added to the public UI test target.

The Collapsible matrix covers:

- Uncontrolled state, callback emission, association, `data-state`, and mounted-hidden content.
- Controlled proposals and rerendered controlled state.
- Protection of owned accessibility/state attributes from caller overrides.
- Enter and Space activation for non-native `asChild` hosts.
- Native button key-plus-synthesized-click sequences toggling exactly once.
- Child-first Slot event composition and cancellation.
- Disabled pointer/keyboard suppression.
- Root, trigger, child, and content ref forwarding.
- Stable, distinct generated IDs for multiple instances.

The Tree matrix covers:

- The production `TreeSection` slotted branch row.
- Controlled branch proposals.
- Row/content ID association.
- Closed hidden semantics, controlled open rerender, and non-native Enter activation.

## Executed Gates

Disk availability was checked immediately before every Bun/Nx command and remained approximately `1.5GiB` free. No install, build, cache deletion, or artifact-heavy command ran.

| Gate | Result |
| --- | --- |
| Focused `nx format:check` over the new leaf/tests, test config, and target manifest | Final pass. The first check identified only the new Collapsible source/test; focused `nx format:write` normalized those two files, and the repeated check passed. |
| `@semio-tech/ui-react:typecheck --skip-nx-cache` | Final pass. The first run found one owned `data-disabled` typing omission; after defining the owned data-attribute shape, the repeated run passed. |
| `@semio-tech/ui-react:test-quick --skip-nx-cache` | Pass: 12 test files and 605 tests. This executed both new Collapsible and Tree matrices. Only the existing Bun `NO_COLOR`/`FORCE_COLOR` warning was printed. |
| `@semio-tech/ui-react:lint --skip-nx-cache` | Pass. Only the existing Bun `NO_COLOR`/`FORCE_COLOR` warning was printed. |
| `@semio-tech/ui-react:check-ui-primitives --skip-nx-cache` | Pass: no UI primitive violations; two existing allowlisted files. |
| `verify dependencies list js` | Pass: 87 identities; neither removed Radix identity is present. |
| `verify dependencies parity js --format json` | Clean: 83 manifests, 272 external rows, 123 evidenced rows, 149 unowned rows, and 0 undeclared imports. |
| Final exact live UI-source dependency/import scan | Pass: zero matches for both dependency names and both primitive aliases. |

## Unrun Gates

- No install or Bun lockfile regeneration ran, per packet instruction.
- No full repository verify gate, full repository test suite, long/exhaustive UI suite, Storybook/browser run, or production build ran because they are outside the focused packet and could materialize broader artifacts under the critical disk constraint.
- No Rust/Cargo command ran, preserving the held Rust lane.
- No ticket JSON, status, important file, goal, compose file, launch configuration, or unrelated UI primitive was edited.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪵️Tree/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`

## P10af Rejection Repair

This addendum supersedes the initial packet's statements that lockfile regeneration was unrun and that caller `hidden` was already protected in both directions. Every P10af finding is now repaired without touching Tabs, ToggleGroup, Popover, Slider, Rust, compose, ticket metadata, status, or important markers.

### Live lock parity versus historical freeze

After a `df -h .` reading of approximately `1.5GiB` free, `bun install --lockfile-only --ignore-scripts --no-progress --no-summary` reconciled the configured live workspace snapshots without installing workspace packages. `bun.lock` changed by exactly 32 deletions. The UI workspace snapshot no longer declares `@radix-ui/react-collapsible` or `@radix-ui/react-dropdown-menu`; neither identity has a package resolution, and the solely DropdownMenu-owned `@radix-ui/react-menu` resolution is gone. A frozen repeat with `--lockfile-only --frozen-lockfile` passed.

The existing `verify dependencies parity js` command now compares configured in-scope Bun workspace dependency, development, optional, and peer tables with their manifests. Missing/stale rows, version drift, missing workspace snapshots, and invalid locks fail. Five executable inline fixtures cover the clean case and all four mismatch categories on every parity run.

`🔒️dependencies.json` deliberately still contains both identities. It is the historical 238-identity dependency-freeze ratchet implemented by `dependencyFreezeCheck`, not a live workspace snapshot: removed identities remain as deletion allowance, while any new live identity still fails. Rewriting it after a deletion would lower the baseline and weaken its intended new-dependency protection.

### Owned interaction repairs

- `CollapsibleContentProps` omits `hidden`, and runtime applies state-owned `hidden={!open}` after all host props. The regression test spreads `hidden={true}` into open/default-open content and proves it remains visible.
- Controlled-lag coverage performs three repeated activations while `open={false}`, proves three `true` proposals with unchanged rendered state, applies `open`, and proves the next proposal is `false` without local mutation.
- Trigger-level native disabled behavior and disabled `asChild` anchor behavior cover pointer, Enter, Space keydown/up, `aria-disabled`, `data-disabled`, `tabIndex=-1`, default prevention, callback suppression, and unchanged disclosure state.
- A real `TreeSection` child-action click and native drag start/end sequence execute their intended callbacks with no disclosure callback or state change.
- Rows with a double-click action defer pointer single-click activation for 300ms. The second click cancels the pending activation before `dblclick`, so the action runs once with no disclosure callback; an isolated single click still emits one delayed open proposal, and keyboard disclosure remains immediate.

### Repair gates

| Gate | Result |
| --- | --- |
| Lockfile-only reconciliation | PASS; 32 lock deletions, free-space reading unchanged at about `1.5GiB`. |
| Frozen lockfile-only consistency | PASS. |
| Focused UI format check | PASS for Collapsible source/test and Tree source/test. An earlier check including the already broadly unformatted root `📜️script.ts` reported that file plus Tree; focused write normalized Tree, and unrelated whole-script formatting churn was removed before final verification. |
| UI typecheck | PASS; Nx labeled the successful task flaky but emitted no error. |
| UI quick tests | PASS — 12 files, 608 tests; existing Bun color warnings only. |
| UI lint | PASS; existing Bun color warning only. |
| UI primitive check | PASS — no violations, two existing allowlisted files. |
| JS dependency list | PASS — 87 identities, neither retired identity present. |
| JS parity | PASS — 83 manifests, 272 external rows, 123 evidenced rows, 149 unowned rows, 0 undeclared imports, 44 configured in-scope lock workspaces, 0 lock mismatches, 5 passing synthetic fixtures. |
| Exact live source/manifest and `bun.lock` scans | PASS — zero occurrences of both retired identities and primitive aliases. The historical baseline retains only its two documented ratchet entries. |

Free space was approximately `1.4GiB` after the safe gates. The full repository verify suite, long/exhaustive UI suite, Storybook/browser execution, production builds, and every Rust/Cargo gate remain unrun.

Repair files additionally include `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪵️Tree/🟦️component.tsx`, `📜️script.ts`, and `bun.lock`.
