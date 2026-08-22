# P10ad Live Dependency Wave Scout

## Snapshot

Read-only snapshot on 2026-08-22, excluding `compose/`. The live commands were:

```text
bun ./📜️script.ts verify dependencies list js
bun ./📜️script.ts verify dependencies list rust
bun ./📜️script.ts verify dependencies parity js --format json
```

The inventory at capture contains **89 JavaScript** and **63 Rust** third-party identities. JS parity reports **83 manifests**, **274 external rows**, **125 evidenced rows**, **149 unowned rows**, and **0 undeclared imports**. The unowned count is a triage signal, not deletion authority: it includes package-scope blind spots such as root tooling and type/runtime conventions. Rust has no proposed packet here: its 59 runtime-only identities are active platform/domain boundaries, and this wave must not consume the held Cargo lane.

## Ranked Packets

All listed JS identities have one direct inventory user: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`. The count below is sequential from the 89-identity snapshot. Every active-wrapper packet must first replace third-party-derived public prop types with owned contracts.

| Order | Packet and exact evidence | Owned seam and focused differential proof | Overlap / identity effect |
| --- | --- | --- | --- |
| 1 | **Remove stale DropdownMenu facade** — `@radix-ui/react-dropdown-menu`; the only non-manifest occurrence is the unused `DropdownMenuPrimitive` import at `📦️index.tsx:18`. | Delete that import and the direct row; no replacement API exists or is needed. Prove absence with the exact import scan, then UI `typecheck`, `test-quick`, `lint`, dependency list, and JS parity. | Touches the shared UI manifest and monolithic barrel only. **-1: 89 → 88.** This is the top recommendation. |
| 2 | **Own Collapsible** — `@radix-ui/react-collapsible`; facade at `🧱️elements/↕️Collapsible/🟦️component.tsx:10,23-37`, target-barrel import at `📦️index.tsx:16`, and live tree integration at `🧱️elements/🪵️Tree/🟦️component.tsx:1579-1655`. | Implement an owned context with controlled/uncontrolled `open`, `defaultOpen`, `onOpenChange`, generated `aria-controls`, `aria-expanded`, `data-state`, content hiding, and the existing `asChild` trigger contract. Add component tests for control modes, keyboard activation, child-slot behavior, and the tree branch. Then UI typecheck/quick/lint/primitives plus list/parity. | Leaf and tree are isolated; manifest/barrel integration must be serialized. **-1: 88 → 87.** |
| 3 | **Own Tabs** — `@radix-ui/react-tabs`; facade at `🧱️elements/📑️Tabs/🟦️component.tsx:10,23-59`, barrel import at `📦️index.tsx:21` and public import at `:8264`; live product use includes `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🛡️AdminApp/🟦️component.tsx:64-91`. | Own a value context and the actual root/list/trigger/content subset: controlled/uncontrolled selection, stable tab/panel IDs, `role=tablist/tab/tabpanel`, roving focus, disabled triggers, and selected-panel visibility. Fixture the Admin app selection path. Run UI plus Admin focused tests, then UI typecheck/quick/lint/primitives and list/parity. | Own leaf/tests are disjoint; shared UI manifest/barrel is serialized. **-1: 87 → 86.** |
| 4 | **Own ToggleGroup** — `@radix-ui/react-toggle-group`; facade at `🧱️elements/🎛️ToggleGroup/🟦️component.tsx:10,41-143`, barrel import at `📦️index.tsx:7961`, and direct Toggle delegation at `🧱️elements/🎚️Toggle/🟦️component.tsx:15,228,375`. | Own the existing `single` and `multiple` value contracts, controlled/uncontrolled updates, disabled items, `aria-pressed`, keyboard navigation, and the action child’s event isolation. Fixture Toggle action/dropdown branches and an engine consumer. Then UI typecheck/quick/lint/primitives and list/parity. | Must coordinate with Toggle, but does not overlap Collapsible/Tabs leaf code. Shared manifest/barrel is serialized. **-1: 86 → 85.** |
| 5 | **Own Popover** — `@radix-ui/react-popover`; facade at `🧱️elements/🗨️Popover/🟦️component.tsx:10,25-68`, target-barrel import at `📦️index.tsx:7540`, and active consumers include `⚡️ActionGroup/🟦️component.tsx:20,168-189`, `🎚️Toggle/🟦️component.tsx:18,304-347`, target search at `📦️index.tsx:9355-9440`, and OS `ShellSync/🟦️component.tsx:67-79`. | Own the presently used controlled/uncontrolled open state, trigger/anchor `asChild`, portal, outside/Escape dismissal, focus-return policy, and measured side/alignment/offset placement. Test every used `side`/`align`, outside click, Escape, anchor update, focus restoration, and search’s prevented auto-focus. Then UI and renderer focused tests, UI typecheck/quick/lint/primitives, list/parity. | Separate from ToggleGroup source, but it overlaps Toggle integration and the target barrel/manifest. **-1: 85 → 84.** |
| 6 | **Own Slider** — `@radix-ui/react-slider`; facade at `🧱️elements/🎚️Slider/🟦️component.tsx:10,92,266-305`, barrel import at `📦️index.tsx:20`, with an additional root override at `package.json:238`. | Preserve the existing owned draft/ready helpers and implement only the current value tuple surface: pointer capture, keyboard increments, min/max/step clamping, disabled state, thumb ARIA, commit/cancel, and ready extent. Add pointer, keyboard, multi-thumb, controlled-lag, ready-clamp, and cancellation fixtures. Then UI typecheck/quick/lint/primitives plus list/parity; remove the root override only after no package needs it. | Do not share implementation work with DnD/Tree or Rust slider conformance; only final manifest/barrel/override cleanup overlaps. **-1: 84 → 83.** |

## Boundaries That Must Stay Separate

- Do not combine **Popover**, **Dialog**, and **Select**. Their shared hard problems are portal ownership, measurement, outside interaction, and focus restoration; a failure can make a shell inaccessible. Dialog and Select are deliberately not in this small wave.
- Do not combine **Slider** with the DnD/Table/Tree cohort. Pointer capture, drag cancellation, keyboard increment, and pending controlled values are independent interaction contracts.
- Tabs and ToggleGroup can have disjoint leaf implementations, but their roving-focus matrices must remain independent. They may only share the final serialized manifest/barrel deletion.
- Every listed UI packet modifies the same target manifest and the large `📦️index.tsx`; parallel work must leave those shared edits to a single integrator.
- Keep all Rust/Cargo work out of this wave. Single-manifest Rust identities such as `ash`, `rfd`, `sqlx`, `vello_svg`, and `windows` are active platform/runtime seams, not declaration-only removals.

## Explicit Non-Candidates

- `@mdx-js/rollup`, `rehype-*`, and `remark-*` are **not** source-empty: `.storybook/main.ts:14,17-20,151-168` imports/configures them.
- `@tailwindcss/typography` is active in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️tailwind/🎨️tailwind.config.ts:15` and `🎨️ui.css:4`.
- `@nxlv/python`, `binaryen`, and `@vitest/coverage-v8` have live configuration or tooling paths. They are not safe declaration deletions.

No build, test, or Cargo success is asserted by this scout; the gates above are the required differential validation for implementation packets.
