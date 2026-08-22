# P10ah Owned Tabs and ToggleGroup

## Outcome

The serialized Tabs/ToggleGroup dependency packet is implemented without touching Popover, Slider, Dialog, Select, Rust, compose, ticket metadata/status, or important markers.

- Replaced both Radix facades with repository-owned React implementations and owned public contracts.
- Repaired every blocking finding from the independent `p10ai` audit: generated associations are injective for exact Unicode strings and root-scoped, Toggle action/dropdown controls are accessible siblings rather than nested interactives, and inactive tab descendants are unmounted before their production effects can run.
- Removed `@radix-ui/react-tabs` and `@radix-ui/react-toggle-group` from the live React manifest, target barrel import surface, configured Bun workspace snapshot, and package resolution table.
- Reconciled `bun.lock` with lockfile-only/no-script mode and proved frozen lock consistency. The cumulative live lock diff against the repository base is 44 deletions and contains neither retired identity; this includes earlier serialized Phase 10 removals already present in the shared worktree.
- Kept both names in `🔒️dependencies.json` because that file is the historical no-new-dependency ratchet, not a live manifest or lock snapshot.

## Owned Tabs Contract

The owned `Tabs`, `TabsList`, `TabsTrigger`, and `TabsContent` provide:

- Controlled `value` proposals and uncontrolled `defaultValue` state with `onValueChange`.
- Deterministic domain/length-prefixed exact UTF-16 code-unit trigger/panel ID encoding, combined with the stable React root instance ID; this is injective for slash/literal escape-like strings, NUL, emoji, and distinct combining/precomposed sequences, while remaining stable across server rendering and hydration.
- Native `tablist`, `tab`, and `tabpanel` roles with state-owned `aria-selected`, `aria-controls`, `aria-labelledby`, `hidden`, `tabIndex`, `data-state`, and orientation data.
- Active-panel-only rendering by default; inactive content descendants are unmounted, preventing hidden requests, subscriptions, and other production effects.
- Independent roving focus for horizontal and vertical lists, RTL-aware horizontal arrows, disabled-trigger skipping, wraparound, Home/End, automatic activation, and exposed manual activation.
- Native disabled buttons and ref support for all four parts.
- Owned exported prop/direction/orientation/activation types; no Radix-derived public type remains.

The real AdminApp fixture proves that its explicit `admin-tab-*` IDs remain the panel labels, controlled selection changes the sole mounted panel, ARIA associations remain exact, inactive routes issue zero requests, leaving Connections closes its `DirectoryClient.stream`, and returning opens one fresh stream.

## Owned ToggleGroup Contract

The owned data-driven `ToggleGroup` and `ToggleGroupItem` provide:

- Discriminated single and multiple contracts with their exact scalar/array `value`, `defaultValue`, and `onValueChange` shapes.
- Controlled proposals without optimistic local mutation, including repeated controlled-lag activations with exactly one callback per activation.
- Uncontrolled selection, single-item deselection to `""`, multiple add/remove arrays, group/item disabled suppression, native buttons, `aria-pressed`, and state-owned `data-state`.
- Independent roving focus with horizontal/vertical orientation, RTL-aware arrows, disabled-item skipping, Home/End, configurable loop, and optional roving focus.
- Stable distinct generated item IDs when a group ID is supplied, caller ARIA/title customization, preserved chrome/data-slot styling, refs, levels, labels, icons, hotkeys, and action content.
- Native primary toggle buttons plus accessible sibling action/dropdown controls; no action or Popover trigger is a descendant of a button, link, or input.
- Primary-only roving selection and focus, independently focusable/clickable action controls, an `asChild` Popover trigger, native disabled propagation, primary refs, and action/dropdown activation that cannot bubble through the primary toggle.
- Owned exported root/item/single/multiple/direction/orientation types; no Radix-derived public type remains.

Toggle's `withAction` integration was corrected so an uncontrolled `defaultPressed` ToggleGroup is no longer accidentally forced into controlled-off state. Focused Toggle tests prove that its action and dropdown branches execute without toggling the main item.

## Focused Runtime Proof

The Tabs matrix covers uncontrolled selection, controlled lag, stable multiple-group IDs, explicit IDs, adversarial Unicode/NUL collision resistance, server-render/hydration ID stability, exact ARIA association, inactive-descendant cleanup-before-mount ordering, disabled triggers, horizontal LTR/RTL arrows, vertical arrows, Home/End, and automatic/manual activation.

The ToggleGroup matrix covers uncontrolled/controlled single and multiple selection, lag proposals, distinct generated IDs across groups, group/item disabled behavior, horizontal RTL and vertical roving focus, disabled skipping, loop suppression, Home/End, sibling-action focus/click/key isolation, and explicit invalid-nesting DOM assertions. Toggle coverage exercises primary/action/dropdown focus, key sequences, native `HTMLElement.click()`, Popover selection, refs, disabled propagation, and absence of `button button`, `button a`, or `button input`. Admin coverage mounts the real authorized shell and asserts exact HTTP/WebSocket counts through repeated switches.

## Executed Gates

Free space was checked immediately before every Bun/Nx gate and remained approximately 1.4 GiB.

| Gate                                                                      | Result                                                                                                                                                                                                                                                                          |
| ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focused `nx format:check` over all repaired leaf/integration/test files   | PASS after focused format write normalized the two changed tests.                                                                                                                                                                                                               |
| `@semio-tech/ui-react:typecheck --skip-nx-cache`                          | PASS.                                                                                                                                                                                                                                                                           |
| `@semio-tech/ui-react:test-quick --skip-nx-cache`                         | Final PASS: 14 files and 622 tests. The first repair run exposed three test synchronization/environment assertions; the production contracts were unchanged, the tests were corrected, and the repeated full quick suite passed. Existing Bun color warnings only.              |
| `os-hub-admin:test --skip-nx-cache`                                       | Final PASS: 2 files and 7 tests. The first repair run exposed the fake WebSocket counting repeated `close()` calls unlike a real already-closed socket; the fake became idempotent and the repeated gate passed. Existing Bun color and Three duplicate-instance warnings only. |
| `@semio-tech/ui-react:lint --skip-nx-cache`                               | PASS; existing Bun color warning only.                                                                                                                                                                                                                                          |
| `@semio-tech/ui-react:check-ui-primitives --skip-nx-cache`                | PASS: no violations and two existing allowlisted files.                                                                                                                                                                                                                         |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary` | PASS; live workspace snapshots and resolutions reconciled without package installation or lifecycle scripts.                                                                                                                                                                    |
| Frozen lockfile-only repeat                                               | PASS with `--lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile`; the existing lock diff remains exactly 44 deletions.                                                                                                                                 |
| `verify dependencies list js`                                             | PASS; neither retired identity is emitted. The serialized Phase 10 inventory is 85 JavaScript identities (89 scout baseline minus Dropdown/Collapsible and Tabs/ToggleGroup).                                                                                                   |
| `verify dependencies`                                                     | PASS: 238 historical baseline identities, 148 current cross-ecosystem identities, 90 allowed removals, and no new dependency.                                                                                                                                                   |
| `verify dependencies parity js --format json`                             | PASS: 83 manifests, 270 external rows, 121 evidenced rows, 149 advisory unowned rows, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, and 5 passing lock fixtures.                                                                                                 |
| Exact executable source/manifest scan                                     | PASS: no `@radix-ui/react-tabs`, `@radix-ui/react-toggle-group`, `TabsPrimitive`, or `ToggleGroupPrimitive`.                                                                                                                                                                    |
| Exact `bun.lock` scan                                                     | PASS: neither retired identity remains.                                                                                                                                                                                                                                         |
| Static direct nested-control scan                                         | PASS in the repaired ToggleGroup/Toggle production sources; runtime DOM assertions additionally prove no button contains a button, link, or input in the action/dropdown paths.                                                                                                 |

## Unrun Gates

- No package-installing Bun command, lifecycle script, cache deletion, Storybook/browser/Playwright run, production build, exhaustive UI suite, full repository test suite, or full monorepo format/typecheck/lint gate ran under the critical disk constraint.
- No Rust/Cargo gate ran, preserving the held Rust lane.
- No renderer-wide or unrelated product suite ran; the shared UI typecheck, 622-test UI quick suite, direct ToggleGroup/Toggle matrices, SSR hydration test, and real Admin integration provide the focused TypeScript/runtime boundary proof for this packet.
- No production browser parser/hydration run was executed; DOM validity and hydration are covered in jsdom, and this limitation remains explicit rather than being represented as browser proof.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Toggle/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Toggle/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️admin.test.tsx`
- `bun.lock`
