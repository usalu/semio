# Phase 10ac — Owned Table Avatar and Toggle

## Scope

Packets B and D from `📓️p10aa-next-zero-dependency-wave.md` are implemented without touching `compose/`, Cargo, git state, or the concurrent generated-barrel content.

## Implementation

### Table Avatar

- Replaced the Radix root/image/fallback trio with native `span`, `img`, and fallback `span` nodes.
- `TableAvatar` now forwards its native root ref and root HTML props while retaining supplied class/style, fallback style, size defaults, and selected/hovered ring tokens.
- A source-keyed image state keeps fallback content visible immediately, reveals an image only after its current source loads, restores fallback on error, and prevents a previously loaded source from revealing a newly requested source.
- The image retains the normalized name as semantic `alt`; the visible fallback exposes the full normalized name while rendering the existing initials or React icon.

### Toggle

- Replaced the third-party root type boundary with an owned native-button prop contract.
- The standard branch now owns controlled `pressed`, uncontrolled `defaultPressed`, `onPressedChange`, `aria-pressed`, `data-state`, disabled suppression, `type="button"`, and deterministic Enter/Space activation.
- Existing group chrome, level, label, tooltip, hotkey, icon, selected presentation, and root data slots remain present.
- The existing `withAction` ToggleGroup branch and dropdown ToggleGroup/Popover branch remain structurally unchanged and are covered by focused branch fixtures.

### Dependency Surface

- Removed direct `@radix-ui/react-avatar` and `@radix-ui/react-toggle` rows from the UI React manifest only after the exact production-import scan was empty.
- No matching direct row existed in the root manifest, and the generated barrel already imported only the owned leaf components, so neither required an edit.
- `🔒️dependencies.json` remains the intentional historical freeze baseline. `bun.lock` reconciliation is deferred with the coordinated full dependency gates; ToggleGroup may retain a transitive Toggle identity independently of the removed direct row.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Toggle/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Toggle/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`

## Focused Evidence

- Two-file targeted Vitest: **PASS**, 2 files and 7 tests.
- Targeted Prettier check over both implementations, both tests, Vitest config, and UI manifest: **PASS**.
- UI manifest JSON parse: **PASS**.
- Outside-`compose/` package-manifest scan for both direct identities: exit `1`, no matches.
- Exact owned production-import scan for both identities: exit `1`, no matches.
- Generated-barrel and root-manifest scans for both identities: exit `1`, no matches.

## Deferred Gates

Per the P4 timing hold, full UI React Vitest/typecheck/lint, dependency freeze/parity, Nx project gates, primitives verification, demonstrator/build coverage, and lockfile reconciliation remain deferred. No Cargo command was run.
