# P10as Independent Owned Command Audit

## Verdict: REJECT — One P1 Repair Required

I read the Command/Select wave scout and the implementation report in full, then independently inspected the owned Command facade, its focused matrix, owned Dialog composition, window Search autocomplete, renderer `UISearch`/`UIFind`, the public barrel, test adapter, UI manifest, and `bun.lock`.

## Release Blocker

### P1 — Host Locale Changes The Claimed Deterministic Ranker

`normalizeCommandText` calls `String.prototype.toLocaleLowerCase()` without an explicit locale in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🟦️component.tsx:133`. That operation depends on the runtime's default locale (notably for Turkish dotted/dotless I), so the same `value`, `keywords`, and query can produce a different filter/ranking result on different hosts. The packet explicitly promises deterministic normalized filtering/ranking, and the repository requires multi-language UI without a default language.

Repair with locale-invariant Unicode lowercasing (`toLowerCase()` after the existing NFKD/combining-mark normalization) and add a focused regression proving the intended invariant behavior. Re-run the focused Command, UI quick, renderer, and lock/dependency gates after the source correction.

## Verified Positive Surface

- The public leaf and barrel export exactly the consumed Root/Dialog/Input/List/Empty/Group/Item/Shortcut surface. The contracts are repository-owned; no `cmdk` component/type or unused compatibility API remains.
- Query state is separate from root selected value. Controlled query/value proposals retain parent authority during lag; `shouldFilter={false}` leaves visible rows under host control.
- Rendered items and groups remain mounted and are hidden in place. React-derived root/list/group/heading/item IDs are stable and injective for duplicate labels; the listbox, group, option, combobox, and active-descendant associations are present.
- Ranking is otherwise stable by score then registration order; keywords participate. Disabled options are skipped; looping, Home/End/Page movement, IME-safe Enter, Space outside text inputs, caller-prevented pointer-down, and exact-once click activation are implemented and covered.
- `CommandDialog` composes through the owned Dialog only; the focused matrix exercises portal title/description ownership, entry focus, Escape proposal, cleanup, and focus restoration.
- Window Search keeps its input focused by preventing the autocomplete row pointer-down; `CommandItem` respects that prevention and avoids the subsequent owned click activation. Renderer `UISearch` and `UIFind` both exercise Enter selection through the real consumer composition.
- The `@semio-tech/ui-react/test` `keyDown` adapter addition is a generic semantic DOM event alongside `change`/`click`/drag/drop; it exposes no product-specific behavior or external test-library type.

## Independent Gate Evidence

| Gate | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 18 files, 661 tests |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS — only existing Bun color-env warning |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache` | PASS — 0 violations, 2 allowlisted files |
| `bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache` | PASS — 4 files, 438 tests |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile` | PASS |
| Dependency ratchet | PASS — historical 238, current 144, removed 94, no additions |
| JavaScript dependency list/parity | PASS — 81 identities; 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 lock workspaces |
| Exact executable source scan for `cmdk`, `CommandPrimitive`, `[cmdk-item]` | PASS — 0 matches in framework/compose source and manifests |
| Exact `bun.lock` scan for `cmdk`, `@radix-ui/react-dialog` | PASS — 0 matches |
| Packet `[DEBUG]` scan | PASS — 0 matches |
| Targeted `git diff --check` | PASS |

The lock reconciliation was lifecycle-script-disabled. No Cargo, browser/E2E, or source mutation was performed by this audit.

## Browser Residuals

JSDOM validates the state, event-precedence, ARIA, mounted-hidden, and owned-dialog paths above. Native browser pointer-to-focus ordering, assistive-technology announcement timing, portal focus timing, and hydration remain unverified browser-only residuals. They do not replace the P1 repair.
