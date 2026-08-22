# P10aa Next Zero-Dependency Wave

## Live Census

Read-only snapshot taken 2026-08-22 after the class-composition, UI-primitive, Animate Markdown, PDF-canvas, hotkey, reconciler, PostCSS, and diagram-type packets recorded in this ticket. `compose/`, ticket material, generated output, and Cargo validation were excluded.

| Measure | Result |
| --- | ---: |
| Live JavaScript third-party identities | 93 |
| Runtime/tooling classifications (overlapping) | 48 / 50 |
| JS manifests / direct external rows | 83 / 278 |
| Static-evidenced rows / rows without scope evidence | 129 / 149 |
| Undeclared external imports | 0 |

Evidence commands:

```text
bun ./📜️script.ts verify dependencies list js
bun ./📜️script.ts verify dependencies parity js --format json
```

Raw outputs are retained beside this report as `📝️p10aa-live-js-census.txt` and `📊️p10aa-live-manifest-parity.json`. The parity command prints its normal success trailer after JSON; consumers need to remove that trailer before parsing.

## Candidate Packets

These are the five smallest source seams still live. They are deliberately limited to owned React control wrappers and one demonstrably unused facade. They have no Cargo work and no `compose/` scope.

All four Radix control candidates share the UI target manifest and its generated barrel (`📦️index.tsx`). Their *implementation and test files* are disjoint, but an integration owner must serialize the final four manifest/barrel deletions rather than let agents concurrently patch those two shared files. This is the only overlap.

### A. Remove unused MDX provider facade — 1 identity, very low risk

- Identity and manifest: `@mdx-js/react` in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`.
- Exact evidence: its only live source occurrence outside lock/baseline metadata is `📦️index.tsx:17160`, `export { MDXProvider } from "@mdx-js/react"`. A whole-workspace `rg` found no `MDXProvider` consumer.
- Owned replacement seam: remove the unused public facade rather than replace it. No owned API uses it.
- Required focused proof: add a source/export absence assertion or snapshot to the UI target's suite; then `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`, `test-quick`, `lint`, dependency freeze, and JS parity.
- Identity impact: **-1** JavaScript identity, 93 → 92.
- Risk: a dynamically compiled MDX consumer could have imported the facade without static evidence. Before removal, search built demos and Storybook config; preserve no compatibility export because the repo is greenfield.

### B. Own the table-avatar primitive — 1 identity, low risk

- Identity and manifest: `@radix-ui/react-avatar` in the UI target manifest.
- Exact source: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🟦️component.tsx:10` imports the package. The wrapper uses only `Root`, `Image`, and `Fallback` (lines 17–39) and its only product surface is the co-located `TableAvatar` component.
- Owned replacement seam: make `TableAvatar` own a tiny `AvatarRoot`/`AvatarImage`/`AvatarFallback` implementation using native React nodes, an image-load/error state, semantic `alt`, and deterministic fallback visibility. It must retain forwarded refs, supplied class/style, the existing size defaults, and the selected/hovered ring behavior.
- Tests/gates: co-located Vitest coverage for load/error fallback, `alt`, forwarded ref, and selected/hovered classes; UI `typecheck`, `test-quick`, `lint`, `check-ui-primitives`, freeze/parity.
- Identity impact: **-1**, 92 → 91 after packet A.
- Risk: Radix delays fallback and controls image status; assert the desired owned timing explicitly instead of silently changing it.

### C. Own the controlled collapsible primitive — 1 identity, medium risk

- Identity and manifest: `@radix-ui/react-collapsible` in the UI target manifest.
- Exact source: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🟦️component.tsx:10`, with only `Root`, `CollapsibleTrigger`, and `CollapsibleContent`. Product use is contained to the co-located story and `🧱️elements/🪵️Tree/🟦️component.tsx:1579–1655`.
- Owned replacement seam: an owned context containing controlled/uncontrolled `open`, `defaultOpen`, `onOpenChange`, generated `aria-controls`, and one trigger/content pair. Support the existing `asChild` trigger contract by reusing the owned slot helper from packet E; `Content` must set `hidden`/`data-state` and unmount only when the owned API says so.
- Tests/gates: controlled and uncontrolled state, keyboard activation, `aria-expanded`/`aria-controls`, `asChild`, and tree collapse integration; UI typecheck/quick/lint/primitives/freeze/parity.
- Identity impact: **-1**, 91 → 90 after A+B.
- Risk: this is behaviorally small but accessibility-sensitive; do not reduce it to a raw `display:none` div.

### D. Own the pressed toggle primitive — 1 identity, medium risk

- Identity and manifest: `@radix-ui/react-toggle` in the UI target manifest.
- Exact source: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Toggle/🟦️component.tsx:10`; primitive types feed the standard, action, and dropdown branches at lines 40–78. The simple root rendering is isolated in the same component, while group/dropdown behavior delegates to owned `ToggleGroup`/`Popover`.
- Owned replacement seam: replace just the root primitive type/render with an owned native `button` adapter that provides controlled/uncontrolled `pressed`, `defaultPressed`, `onPressedChange`, `aria-pressed`, disabled behavior, and `data-state`. Keep the existing owned group/popover delegation intact.
- Tests/gates: controlled/uncontrolled transition, Space/Enter keyboard behavior, disabled suppression, `aria-pressed`, and existing action/dropdown flows; UI typecheck/quick/lint/primitives/freeze/parity.
- Identity impact: **-1**, 90 → 89 after A–C.
- Risk: `ToggleProps` currently inherits Radix's full prop type. Define an owned prop interface before deleting it so no third-party type leaks across the public API.

### E. Own the single-child slot adapter — 1 identity, medium risk

- Identity and manifest: `@radix-ui/react-slot` in the UI target manifest.
- Exact source: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ButtonGroup/🟦️component.tsx:10,114`; it is the sole leaf use, selected only when `asChild` is true. `📦️index.tsx:67` merely reimports it for a re-export and must stop doing so.
- Owned replacement seam: introduce an owned `Slot` utility beside `🏷️class-name-composition` or the React host port. Its explicit contract should require exactly one valid React element, merge class/style, compose child and wrapper event handlers in a documented order, and merge refs. `ButtonGroup` is then the only initial consumer.
- Tests/gates: single-child validation, child/wrapper handler order and `defaultPrevented`, merged `className`/style/ref, and `ButtonGroup asChild`; UI typecheck/quick/lint/primitives/freeze/parity.
- Identity impact: **-1**, 89 → 88 after A–D.
- Risk: slot cloning is subtle. A tested owned primitive is preferable to broad pseudo-Radix compatibility; retain only the single-child contract this repository actually uses.

## Recommended Execution

1. Land A by itself (a deletion with a clean 1-identity ratchet).
2. Implement E first, without its manifest deletion, because C's existing trigger relies on `asChild`.
3. Implement B and D in parallel in their disjoint element files; the shared UI manifest/barrel integration happens once afterward.
4. Implement C after E is present, then delete the four Radix rows together and run the full UI gate set once.

This four-stage shape gives a real **five-identity** JavaScript reduction (93 → 88) without claiming that high-risk active boundaries such as `reveal.js`, `three-mesh-bvh`, i18n, DnD, XYFlow, or the rendering stack are small removals. Those are active implementation redesigns and should be planned separately.
