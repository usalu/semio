# P10aj Independent Tabs and ToggleGroup Final Audit

## Verdict: PASS

Independent read-only re-audit on 2026-08-22 of the repaired Phase 10 Tabs and ToggleGroup packet. I read `AGENTS.md`, the applicable UI instruction, `p10ah`, and all of `p10ai`, including its repair disposition; then inspected the current implementation, focused tests, real Admin integration, manifests, lockfile, barrel, and worktree diff. No prior blocker remains reproducible.

## Blocking-Boundary Evidence

### Tabs IDs, ARIA, and hydration

- `idToken` in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🟦️component.tsx:65-70` emits the code-unit count followed by a fixed-width hexadecimal encoding of every UTF-16 code unit. It is injective for arbitrary JavaScript strings: `/` and literal `-2F` no longer collide; NUL, emoji (including surrogate pairs), unpaired surrogate code units, and combining/precomposed forms retain their exact sequences.
- `Tabs:75-76` domain-separates the optional root ID and React `useId()` instance, while `:95` separately domain-encodes trigger/panel values. Thus generated associations are scoped to a Tabs root and stable for SSR/hydration. Explicit caller IDs remain honoured.
- `📑️Tabs/🧪️component.test.tsx:55-97` mounts two roots with `/`, `-2F`, `%2F`, NUL, emoji, combining `e` + U+0301, and precomposed `é`; it proves all trigger/control IDs unique and the active panel's exact reverse association. `:99-113` proves server markup and hydrated IDs match with no React console error.
- The owned implementation supplies native `tablist`/`tab`/`tabpanel` roles, bidirectional ARIA references, disabled native buttons, state data attributes, controlled/uncontrolled proposals, refs, horizontal/vertical and RTL roving focus, disabled skipping, wrap, Home/End, and automatic/manual activation (`📑️Tabs/🟦️component.tsx:109-246`). The focused matrix exercises each of those behaviours.

### Tabs inactive descendants and Admin effects

- `TabsContent` still registers its association but returns `null` unless selected (`📑️Tabs/🟦️component.tsx:224-246`), so inactive descendants cannot mount or issue effects. Its lifecycle test asserts `cleanup:alpha` precedes `mount:beta` (`📑️Tabs/🧪️component.test.tsx:115-143`).
- The real `AdminApp` places each production page under `TabsContent` (`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🛡️AdminApp/🟦️component.tsx:75-93`). Its integration test only routes active overview/spaces/connections requests, explicitly asserts zero Users/Documents/Events and inactive Spaces/Connections requests, then proves exact active request counts plus one connection stream, close-on-leave, and a fresh second stream on return (`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️admin.test.tsx:183-249`). This does not use blanket hidden-route stubs.

### ToggleGroup and Toggle secondary controls

- The primary is a native button and any `action` is emitted as a sibling in the item shell, never a descendant (`🎛️ToggleGroup/🟦️component.tsx:183-242`). Primary-only selection/roving/focus is retained; sibling actions have independent native focus and cannot bubble through a parent button because no parent button exists.
- Toggle's with-action branch passes a native `Action as="button"`; the dropdown branch uses `PopoverTrigger asChild` around that native Action (`🎚️Toggle/🟦️component.tsx:196-208`, `:286-326`). Disabled state and the caller's primary ref propagate to both Toggle controls as asserted by the matrix.
- The ToggleGroup matrix asserts vertical roving, sibling action focus/click isolation, and absence of `button button`, `button a`, and `button input` (`🎛️ToggleGroup/🧪️component.test.tsx:137-165`). Toggle's action/dropdown matrix asserts primary refs, disabled propagation, focus, Enter/Space sequences, native `HTMLElement.click()`, Popover opening/item selection, exact non-selection of the primary, and the same invalid-nesting scan (`🎚️Toggle/🧪️component.test.tsx:60-151`).
- The full owned group retains discriminated single/multiple contracts, controlled-lag proposals, single deselection to `""`, multiple membership updates, group/item disabled suppression, `aria-pressed`, group orientation/direction, disabled skipping, Home/End, optional roving focus, and loop control (`🎛️ToggleGroup/🟦️component.tsx:28-241`).

## Dependency, Lock, and Diff Evidence

- Live React manifest, public barrel, executable sources, and `bun.lock` contain none of `@radix-ui/react-tabs`, `@radix-ui/react-toggle-group`, `TabsPrimitive`, or `ToggleGroupPrimitive`.
- The live `bun.lock` diff is exactly `0 additions, 44 deletions`; it deletes the two workspace declarations and their resolution rows. No lock additions are present.
- `bun ./📜️script.ts verify dependencies` reported the frozen historical baseline of 238, **148 current identities**, 90 allowed removals, and no new dependency. Parity reported 83 manifests, 270 external rows, 121 evidenced rows, 149 advisory-unowned rows, zero undeclared imports, zero lock mismatches, 44 lock workspaces, and five passing lock fixtures.

## Executed Gates

| Gate | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 14 files, 622 tests |
| `bun nx run os-hub-admin:test --skip-nx-cache` | PASS — 2 files, 7 tests |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS — existing Bun colour warning only |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache` | PASS — 2 existing allowlisted files |
| `bun ./📜️script.ts verify dependencies` | PASS — 148 identities, no new dependency |
| `bun ./📜️script.ts verify dependencies parity js --format json` | PASS — zero undeclared imports and lock mismatches; five lock fixtures |
| Exact source/manifest/barrel/lock scan and lock diff inspection | PASS — retired identities/primitive aliases absent; lock diff is 44 deletions |

The filesystem had 2.4 GiB free immediately before the final focused gates. Each Bun gate emitted only the pre-existing `NO_COLOR`/`FORCE_COLOR` warning; no gate failure or implementation diagnostic occurred.

## Intentionally Unrun

- No `bun install` or frozen-lockfile install was run: this audit was expressly constrained not to install packages or mutate the lockfile. Frozen lock consistency is therefore evidenced by the existing repair record rather than independently re-executed here.
- No browser/Playwright, Storybook, production build, exhaustive UI/repository suite, full monorepo format/typecheck/lint, Rust/Cargo, cache deletion, source repair, ticket metadata mutation, or Git mutation ran.

## Scope Note

This audit added only this report. It makes no production-browser claim; SSR/hydration and control-tree validity were independently exercised in the focused jsdom tests and inspected in source.
