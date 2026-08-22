# P10av Owned Select

## Verdict

**AUDIT-READY.** The UI React target now owns the Select runtime and public contracts. `@radix-ui/react-select` is absent from executable UI and Hub source, live manifests, and `bun.lock`. The dependency ratchet is at the expected **143 total / 80 JavaScript identities**. All bounded source, focused UI, real-consumer, type, lint, primitive-policy, formatting, frozen-lock, dependency, parity, and exact-scan gates pass.

No Cargo command, modifying Git command, ticket metadata mutation, DnD source change, or graph-stack source change was made for this packet.

## Consumed Surface Inventory

The retained runtime surface is the exact facade used by production code, stories, and tests:

- `Select`, `SelectTrigger`, `SelectValue`, `SelectContent`, `SelectViewport`
- `SelectItem`, `SelectItemText`, `SelectItemIndicator`
- `SelectGroup`, `SelectLabel`, `SelectSeparator`
- `SelectScrollUpButton`, `SelectScrollDownButton`

Repository-owned public contracts cover root value/open control, trigger/value/content, item/group/label/separator, scroll controls, preventable lifecycle events, and placement. No exported Select contract depends directly or indirectly on a Radix type.

Production consumers use controlled `value`/`onValueChange`; disabled roots occur in the generic facade, IconSelector, and shell helpers. Stories/tests also consume `defaultValue` and controlled `open`. The only production placement-specific input is IconSelector's `position="popper"`; no production consumer requires virtualization or a custom collision policy. Custom portal containers, RTL placement, flipping, clamping, and viewport scrolling remain owned and covered because they are part of the exposed content contract.

## Owned Behavior

- Controlled and uncontrolled `value` and `open` state preserve proposal-only controlled behavior without one-render internal lag.
- Stable trigger, listbox, label, and injective option IDs are derived with hydration-safe React IDs; explicit item IDs remain authoritative.
- The trigger uses button/combobox semantics, content uses listbox semantics, and items/groups expose option/group semantics with disabled, placeholder, selected, labelled, and active-descendant state.
- Selected-value projection is derived from the matching owned item while active navigation remains independent of the selected value.
- Arrow, Home, End, PageUp, PageDown, Enter, Space, Escape, and Tab behavior is owned. Typeahead uses NFKD normalization, combining-mark removal, and locale-invariant lowercasing, and ignores composing input.
- Keyboard and pointer selection commit exactly once. Touch movement does not synthesize hover activation, and touch pointer-down does not suppress the subsequent click.
- Open focus enters the listbox; selection and Escape restore trigger focus; Tab and outside interaction preserve the browser's destination focus. Preventable autofocus, Escape, pointer-outside, interact-outside, and close-autofocus events are respected.
- A logical surface stack implements deepest-open ordering plus most-recent sibling ordering across portals. A native event closes at most one eligible Select, including nested Select portals.
- Content portals to `document.body` by default or an owned custom container. SSR guards avoid document/window access during server rendering.
- Owned placement supports side, align, side offset, collision padding, RTL start/end, viewport flip/clamp, trigger-width variables, and resize/scroll recomputation. Scroll controls operate the owned viewport without introducing nested interactive DOM.

## Focused and Consumer Evidence

The focused real-DOM Select matrix has 10 passing cases covering fallback and projected text, controlled lag, active-versus-selected state, disabled navigation, keyboard/typeahead, labelled groups and duplicate labels, preventable Escape/outside dismissal, recent sibling ordering, nested logical portal ordering, touch exact-once selection, custom portals, RTL placement, scrolling, and no nested buttons.

Two real-consumer assertions were added:

- Hub Admin opens the authenticated locale Select and commits a locale through its rendered option.
- Renderer React renders a declarative Select control and projects its selected option.

## Changed Paths

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Select/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Select/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🕸️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️admin.test.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
- `bun.lock`
- regenerated `📊️p10-manifest-source-parity.json` and `📓️p10-manifest-source-parity.md`

Shared barrel, consumer-test, manifest, lock, and parity artifacts also contain concurrent Phase 10 work; this packet preserved those edits.

## Final Gates

| Gate | Result |
| --- | --- |
| UI React `test-quick` | PASS — 19 files, 672 tests |
| Focused Select real-DOM matrix | PASS — 1 file, 10 tests |
| Renderer React `test-quick` | PASS — 4 files, 439 tests |
| Hub Admin `test` | PASS — 2 files, 8 tests |
| UI React `typecheck` | PASS |
| UI React `lint` | PASS; only Bun's existing `NO_COLOR`/`FORCE_COLOR` warning |
| UI primitive policy | PASS — 0 violations, 2 existing allowlisted files |
| Exact-file Nx format check | PASS |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary` | PASS — lifecycle scripts disabled; lock reconciled |
| Frozen lockfile-only repeat with lifecycle scripts disabled | PASS |
| Dependency freeze | PASS — historical 238, current 143, removed 95, no additions |
| JavaScript dependency list | PASS — 80 identities |
| JavaScript dependency parity | PASS — 83 manifests, 265 external rows, 116 evidenced rows, 149 advisory unowned rows, 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 lock workspaces |
| Manifest/source audit regeneration | PASS — 64 manifests, 577 direct rows, 265 external rows, 75 no-package-scope-evidence candidates |
| Exact live source/manifest scan for `@radix-ui/react-select` and `SelectPrimitive` | PASS — zero matches |
| Exact `bun.lock` scan for `@radix-ui/react-select` | PASS — zero matches |
| Packet `[DEBUG]` scan | PASS — zero matches |
| Targeted `git diff --check` | PASS |

## Browser-Only Residuals

This packet did not run Storybook, a production build, Playwright, a full monorepo test sweep, or an assistive-technology session. JSDOM and pure placement tests establish the owned contract, including custom portal containment, logical stacking, focus restoration, touch event ordering, viewport scrolling, RTL, flip, and clamp calculations. A real browser remains the authority for native pointer/focus sequencing, physical viewport geometry during scrolling and resizing, hydration behavior, visual collision placement, scroll-control ergonomics, and screen-reader announcements.
