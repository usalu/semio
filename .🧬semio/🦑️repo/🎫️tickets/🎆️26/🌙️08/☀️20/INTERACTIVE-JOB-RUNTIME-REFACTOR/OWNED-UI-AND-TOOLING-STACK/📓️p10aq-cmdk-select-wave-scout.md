# P10aq Command And Select Wave Scout

## Verdict

Implement the owned **Command** packet first and serialize the owned **Select** packet immediately after it. They are leaf-code-disjoint, but both change the UI manifest, the monolithic React target barrel, `bun.lock`, and shared renderer-facing behavior; do not run their integrations concurrently.

The Command packet removes one currently counted JavaScript identity (`cmdk`), and its lockfile reconciliation also removes the now-unreachable `@radix-ui/react-dialog` resolution. Dialog's direct declaration was already removed by P10ao, so that transitive resolution is leverage but not a second current manifest-identity removal. The Select packet then removes one further counted identity. The expected sequence is **145 → 144 → 143** total identities and **82 → 81 → 80** JavaScript identities, assuming no unrelated concurrent manifest changes.

## Read-Only Snapshot

Commands run on 2026-08-22:

```text
bun ./📜️script.ts verify dependencies list js --format json | jq 'length'
bun ./📜️script.ts verify dependencies parity js --format json
rg --glob '*.{ts,tsx,js,jsx,json}' cmdk/@radix-ui/react-select source scans
rg bun.lock/package.json dependency and adjacency scans
```

The JS list reported **82** identities. JS parity was clean: **0 undeclared imports, 0 lock mismatches, 5 lock fixtures, and 44 lock workspaces**. The P10ao/P10ap Dialog acceptance records the same 145-total/82-JS baseline and establishes that `@radix-ui/react-dialog` is already absent from live source and all manifests.

Exact lock occurrence counts are two each: one workspace dependency edge plus one package resolution for `cmdk`, `@radix-ui/react-select`, and the residual `@radix-ui/react-dialog`.

## Complete Direct-Import Inventory

| Identity | Live import | Classification | Required disposition |
| --- | --- | --- | --- |
| `cmdk` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🟦️component.tsx:10` | Active Command facade implementation | Replace with repository-owned implementation and contracts. |
| `cmdk` | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:136` | Stale barrel adapter import; no other `CommandPrimitive` use exists in this file | Delete in the Command integration. |
| `@radix-ui/react-select` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Select/🟦️component.tsx:10` | Active Select facade implementation | Replace with repository-owned implementation and contracts. |
| `@radix-ui/react-select` | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:16` | Stale barrel adapter import; no other `SelectPrimitive` use exists in this file | Delete in the Select integration. |

The only direct declarations are the two rows in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` (lines 37 and 42). No other executable source imports either external identity.

## Command Packet: Highest Leverage, Moderate Risk

### Lockfile Result

`cmdk@1.1.1` is a direct UI-target dependency and its only listed dependencies are `@radix-ui/react-compose-refs`, `@radix-ui/react-dialog`, `@radix-ui/react-id`, and `@radix-ui/react-primitive`. The P10ao/P10ap scans and the current `bun.lock` scan establish that the Dialog resolution has exactly one incoming package edge: `cmdk`. Therefore, after the owned Command replacement and lockfile-only reconciliation, `cmdk` **and** `@radix-ui/react-dialog` must disappear from `bun.lock`.

The other three cmdk dependencies remain reachable through Select's Radix graph, so this packet must not claim their removal.

### Actual Public Surface

Current public exports are exactly `Command`, `CommandDialog`, `CommandInput`, `CommandList`, `CommandEmpty`, `CommandGroup`, `CommandItem`, and `CommandShortcut` from the Command leaf and target barrel. There is **no** current `CommandSeparator` export, import, or consumer; do not introduce an unused compatibility surface.

Current wrappers leak third-party types through `React.ComponentProps<typeof CommandPrimitive...>` for Root, Input, List, Empty, Group, and Item. The replacement must define repository-owned prop contracts and must not export a third-party-derived type. The needed live behavior is:

- Root controlled/uncontrolled query (`value`, `defaultValue`, `onValueChange`), optional host filtering (`shouldFilter={false}`), and a deterministic owned text match/filter policy when filtering is enabled.
- Input controlled/uncontrolled value forwarding, normal text-entry behavior, `data-slot="command-input"`, and association with the owned command list.
- Groups with an optional heading, items with `value`, `disabled`, and `onSelect`, empty state, and decorative shortcut text.
- Roving active item, `aria-activedescendant`, `role="combobox"`/`role="listbox"`/`role="option"` semantics, ArrowUp/ArrowDown/Home/End movement, Enter/Space selection, disabled-item skipping, and pointer selection without double firing.
- Dialog composition strictly through the already-owned Dialog packet: controlled open proposal, focus entry/return, Escape/outside behavior, title/description, and no second dialog or portal ownership.

There is no live virtualization API or virtual-list consumer. The packet must make that limitation explicit and prove that filtering/roving operates over the rendered item set; it must not silently depend on `ResizeObserver`, scrolling internals, or cmdk's attribute names.

### Consumers, DOM Assumptions, And Tests

| Consumer / observer | Current use that must remain true |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellSearch/🟦️component.tsx` | `UISearch` and `UIFind` use controlled `CommandDialog`, controlled `CommandInput`, host-owned fuzzy ranking, `shouldFilter={false}`, grouped items, and `onSelect`. Their renderer test at `📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts:4298+` asserts both surfaces and their owned fuzzy filtering. |
| UI target `📦️index.tsx:10255+` | Search autocomplete uses `Command`/List/Group/Item/Empty inside owned Popover, marks an active item, selects via prevented pointer-down and `onSelect`, and must keep text input focus. |
| UI target `📦️index.tsx:9663` | `isSearchSuggestionActionTarget` presently recognizes both `[cmdk-item]` and `[data-slot="command-item"]`; make the owned selector exclusively `data-slot`. |
| OS dev script `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:2504,2562,2567` | Browser E2E locates command rows through `[cmdk-item]`; update those owned selectors to `[data-slot="command-item"]` in the same packet. |
| Renderer test setup `.../🧪️index.test.ts:267-275` | The existing ResizeObserver and `scrollIntoView` polyfills are documented as cmdk-only. Remove or narrow them only after confirming no other test user depends on them. |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️story.tsx` | Controlled root story uses Input, Empty, Groups, Items, and Shortcut; preserve story surface. |

No separate Command component test currently exists. Add `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️component.test.tsx` before removing cmdk. Required matrix: controlled lag, default/uncontrolled state, normalized matching and empty state, `shouldFilter={false}`, active-descendant and item IDs, keyboard movement/skip/wrap policy, pointer/Enter select exactly once, disabled items, actual Search autocomplete pointer-focus preservation, actual UISearch/UIFind selection and Dialog focus restoration. The native browser focus/scroll timing and assistive-technology behavior remain browser-only residuals after JSDOM.

### Command Paths And Gates

Implementation-owned paths:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️component.test.tsx` (new)
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
- UI target `package.json`, `bun.lock`, and the implementation report.

Focused gates: UI `typecheck`, `test-quick`, `lint`, and `check-ui-primitives`; renderer React `test`; exact-file Prettier; `bun install --lockfile-only --ignore-scripts --no-progress --no-summary` followed by its frozen-lockfile form; dependency verification/list/parity; exact source/manifest zero scan for `cmdk` and `CommandPrimitive`; exact `bun.lock` zero scan for both `cmdk` and `@radix-ui/react-dialog`; `[DEBUG]` scan; and targeted `git diff --check`.

## Select Packet: Broadest Consumer Risk, One Identity

### Existing Surface And Contract Leak

The Select facade exports exactly `Select`, `SelectContent`, `SelectGroup`, `SelectItem`, `SelectLabel`, `SelectScrollDownButton`, `SelectScrollUpButton`, `SelectSeparator`, `SelectTrigger`, and `SelectValue`. Every public wrapper currently derives props from `SelectPrimitive`; replacement contracts must be repository-owned.

Live facade behavior includes controlled/uncontrolled `value` and `defaultValue`, `onValueChange`, `onOpenChange`, an id/show-label wrapper, a fallback to the first nested item, trigger size/id, value placeholder, groups/labels/separators, disabled items, item icon and indicator, scroll buttons, portal content, `position="popper"`, RTL direction, and CSS variables derived from Radix's trigger/content geometry. The owned implementation must replace those `--radix-select-*` assumptions with owned geometry variables rather than retaining an implementation-specific CSS contract.

It must prove a repository-owned accessible combobox/listbox: label association; trigger `aria-expanded`, `aria-controls`, and active option; portal mount/unmount; controlled-lag proposal behavior; pointer selection; Escape/outside dismissal and focus return; Arrow/Home/End/typeahead navigation; Enter/Space selection; disabled skips; groups/separators; RTL; and scroll-button behavior. None of the current consumers needs virtualization, so virtualized options are explicitly out of scope and must not be implied.

### Live Consumer Breadth

The direct Select consumers are `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️component.tsx`, `🔣️IconSelector/🟦️component.tsx`, `🧪️NavbarExampleSelect/🟦️component.tsx`, and `🪵️Tree/🟦️component.tsx`; Admin `🏛️SpacesPage/🟦️component.tsx`, `📄️DocumentsPage/🟦️component.tsx`, and `🛡️AdminApp/🟦️component.tsx`; and renderer `ChromePanels/🟦️component.tsx`, `Interpreter/🟦️component.tsx`, and `ShellHelpers/🟦️component.tsx` (plus declarative Select construction in the renderer target barrel).

All observed product instances are controlled values with `onValueChange`; only `IconSelector` explicitly passes `position="popper"`. No production consumer was found passing a custom portal container, custom collision settings, virtualized collection, or non-default positioning mode. The Select story is the sole complete Group/Label/Separator/ScrollButton specimen. No dedicated live Select test exists, so the owned packet must add one and exercise representative real Admin and renderer consumers.

### Select Paths And Gates

Implementation-owned paths:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Select/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Select/🧪️component.test.tsx` (new)
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- UI target `package.json`, `bun.lock`, and the implementation report.

Likely real-consumer test changes belong in `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️admin.test.tsx` and `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`; keep product components unchanged unless a discovered owned-contract gap requires a deliberate simultaneous refactor.

Run the same UI/dependency/format/frozen-lock/source-scan/debug/diff gates as Command, plus Admin `test` and renderer React `test`. Exact source/manifest and lock scans must show zero `@radix-ui/react-select` matches after reconciliation. A real browser test remains required to retire native portal placement, pointer-focus order, scrolling, and assistive-technology residuals.

## Rank And Sequencing

1. **Command first** — removes a direct identity and the last transitive Dialog resolution with one active application family (Search/Find plus autocomplete); Dialog is already owned, and the existing renderer test gives a focused integration seam. Risk is moderate because keyboard and active-descendant semantics must be fully re-owned.
2. **Select second** — removes one direct identity but has a much larger, portal/geometry/focus-heavy consumer matrix. Its risk is high, so it merits an isolated implementation/audit cycle rather than pairing it with Command.

The two leaf files can be authored independently, but the shared barrel, manifest, lockfile, current count, and final dependency/parity proof must be serialized by one integrator. No Cargo, install, cache deletion, or source mutation occurred during this scout.
