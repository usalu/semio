# P10ar Owned Command

## Verdict: AUDIT-READY

The React UI target now owns its complete consumed Command surface. The runtime and public facade no longer import `cmdk`; repository-owned contracts cover exactly `Command`, `CommandDialog`, `CommandInput`, `CommandList`, `CommandEmpty`, `CommandGroup`, `CommandItem`, and `CommandShortcut`. No unused Separator, Loading, or virtualization compatibility surface was introduced.

Lockfile-only reconciliation removed both `cmdk` and the now-unreachable `@radix-ui/react-dialog` package resolution. The dependency ratchet is at the expected **144 total / 81 JavaScript identities**, with no new identity, undeclared import, or workspace lock mismatch.

## Owned Behavior

- Root owns controlled/uncontrolled selected values and exact-once value proposals. Input separately owns controlled/uncontrolled query state, so controlled parent lag remains authoritative and never optimistically changes filtering or the visible input value.
- The default filter applies NFKD, strips combining marks, uses locale-invariant Unicode `toLowerCase()`, and normalizes whitespace before scoring exact, prefix, token-prefix, substring, and ordered-subsequence matches deterministically. Explicit keywords participate. `shouldFilter={false}` leaves result ownership entirely with Shell Search/Find or the window Search host.
- Items and groups remain mounted and use `hidden` when filtered or explicitly hidden. Stable React-derived list, group, heading, and item IDs preserve duplicate-label injectivity. The highest-ranked enabled result owns `aria-activedescendant`.
- Input/list/item/group semantics are an owned combobox/listbox/option/group model. ArrowUp/ArrowDown, Home/End, PageUp/PageDown, optional looping, disabled skips, and Enter activation are implemented without layout observers or scrolling internals. Space activates only when focus is not in a text field.
- Keyboard activation ignores IME-composing and key-code-229 events. Pointer movement updates the active descendant; click activates once. A consumer-prevented pointer-down suppresses Command's later click activation, preserving the actual window Search autocomplete's input focus and avoiding duplicate host selection.
- `CommandDialog` composes only the already-owned Dialog. Its accessible title and description stay inside the active portal; Dialog retains focus entry, Escape/outside dismissal, controlled open proposals, cleanup, and focus restoration.
- Styling and automation selectors now use only owned `data-slot` attributes. The stale target-barrel adapter import and every `[cmdk-item]` browser selector were removed.

## Runtime Evidence

The new focused real-DOM matrix contains 9 tests for locale-invariant normalized ranking, mounted-hidden filtering, empty state, `shouldFilter={false}`, controlled query/value lag, duplicate labels and stable IDs, disabled/loop/page navigation, IME safety, pointer exact-once behavior, grouped/hidden results, shortcuts, and owned Dialog focus entry/dismissal/restoration. The locale regression explicitly demonstrates that Turkish locale-sensitive lowercasing maps ASCII `I` differently while the owned ranker produces the same intended exact result for `Istanbul` and `İSTANBUL` against `istanbul`.

The existing real window Search tests continue to prove inline ranking, chevron-opened autocomplete, keyboard selection, and pointer selection while focus remains in the input. Renderer tests were extended so the actual `UISearch` and `UIFind` consumers filter and activate their ranked result through Enter exactly once. Their repository-owned test adapter gained the missing semantic `keyDown` operation rather than exposing the external test library.

## Final Bounded Gates

| Gate                                                                      | Result                                                                                                                               |
| ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Focused Command matrix                                                    | PASS — 1 file, 9 tests.                                                                                                              |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`               | PASS.                                                                                                                                |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`              | PASS — 18 files, 662 tests.                                                                                                          |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`                    | PASS — only the existing Bun `NO_COLOR`/`FORCE_COLOR` warning.                                                                       |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache`     | PASS — 0 violations, 2 existing allowlisted files.                                                                                   |
| `bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache`    | PASS — 4 files, 438 tests.                                                                                                           |
| Exact-file Prettier check                                                 | PASS.                                                                                                                                |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary` | PASS — lifecycle scripts disabled.                                                                                                   |
| Frozen lockfile-only reconciliation                                       | PASS.                                                                                                                                |
| `bun ./📜️script.ts verify dependencies`                                   | PASS — historical 238, current 144, removed 94, no new dependency.                                                                   |
| JavaScript dependency list                                                | PASS — 81 identities.                                                                                                                |
| JavaScript dependency parity                                              | PASS — 83 manifests, 266 external rows, 117 evidenced rows, 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 lock workspaces. |
| Manifest/source audit                                                     | PASS — 64 manifests, 578 direct rows, 266 external rows, 75 rows without owned-scope evidence.                                       |
| Exact source scan for `cmdk`, `CommandPrimitive`, and `[cmdk-item]`       | PASS — 0 matches in executable framework/hub TypeScript, JavaScript, and JSON.                                                       |
| Exact `bun.lock` scan for `cmdk` and `@radix-ui/react-dialog`             | PASS — 0 matches.                                                                                                                    |
| Packet `[DEBUG]` scan                                                     | PASS — 0 matches.                                                                                                                    |
| Targeted `git diff --check`                                               | PASS.                                                                                                                                |
| Final disk check                                                          | 2.2 GiB available.                                                                                                                   |

## Changed Paths

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️render.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
- `bun.lock`
- regenerated `📊️p10-manifest-source-parity.json` and `📓️p10-manifest-source-parity.md`
- this report.

## Explicit Residuals

No browser/Playwright, Storybook, production build, full monorepo suite, SSR/hydration, or native assistive-technology run was performed. JSDOM proves owned state authority, event ordering, accessibility attributes, focus transitions, mounted-hidden semantics, and actual consumer activation, but cannot prove native pointer-to-focus ordering, screen-reader announcement timing, browser portal focus timing, or hydration behavior.

No Cargo/Rust command, Select edit, cache deletion, package installation, Git mutation, or ticket metadata operation was performed.
