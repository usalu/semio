# P10au Independent Owned Command Final Audit

## Verdict: PASS

I read P10aq, P10ar, P10as, and P10at in full, then independently re-inspected the current owned Command source, focused matrix, public barrel, window Search integration, renderer `UISearch`/`UIFind` coverage, package manifest, and `bun.lock`.

The locale repair is correct. `normalizeCommandText` applies NFKD decomposition, removes combining marks, then calls locale-invariant Unicode `toLowerCase()` before whitespace normalization. Production Command code contains no `toLocaleLowerCase()` call. The focused regression proves Turkish locale-sensitive conversion would turn ASCII `Istanbul` into `ıstanbul`, while the owned ranker deterministically scores both `Istanbul` and `İSTANBUL` as exact matches for `istanbul`; the dotless `ı` remains distinct and scores zero. This preserves both host-locale independence and the required dotless-I distinction.

## Rechecked Owned Surface

- The exact consumed `Command`, `CommandDialog`, `CommandInput`, `CommandList`, `CommandEmpty`, `CommandGroup`, `CommandItem`, and `CommandShortcut` surface remains repository-owned and exported through the React barrel. No third-party-derived public contract or unconsumed compatibility export is present.
- Controlled query and selection authority, mounted-hidden filtering, stable injective React-derived option IDs, active descendant, disabled skips, ranked deterministic order, loop/page navigation, IME rejection, and exact-once pointer/keyboard selection remain covered by the focused real-DOM matrix.
- `CommandDialog` still composes only the owned Dialog and retains its title/description portal, entry focus, Escape proposal, cleanup, and focus return behavior.
- Window Search still uses `shouldFilter={false}` with its host-owned ranking; prevented row pointer-down preserves input focus and suppresses the later Command click. `isSearchSuggestionActionTarget` now recognizes only the owned `[data-slot="command-item"]` selector. Renderer `UISearch` and `UIFind` still exercise their actual Enter-selection paths.
- No `cmdk`, `CommandPrimitive`, or `[cmdk-item]` occurrence remains in executable framework/hub TypeScript, JavaScript, or JSON, nor in live manifests or `bun.lock`. The only whole-repository scan occurrence is the intentionally immutable historical baseline `🔒️dependencies.json`, which records the original 238-identity freeze and is not executable source or a live declaration.

## Fresh Independent Evidence

| Gate | Result |
| --- | --- |
| Focused Command test through Nx | PASS — 1 file, 9 tests |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 18 files, 662 tests |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS — only the pre-existing Bun color-environment warning |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache` | PASS — 0 violations, 2 existing allowlisted files |
| `bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache` | PASS — 4 files, 438 tests |
| Focused `nx format:check` for Command source and matrix | PASS |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile` | PASS — lifecycle scripts disabled |
| Dependency freeze | PASS — historical 238, current 144, 94 removed, no new dependency |
| JavaScript dependency list | PASS — 81 identities |
| JavaScript dependency parity | PASS — 83 manifests, 266 external rows, 117 evidenced rows, 149 unowned rows, 0 undeclared imports, 0 lock mismatches, 5 lock fixtures, 44 lock workspaces |
| Regenerated manifest/source audit | PASS — 64 manifests, 578 direct rows, 266 external rows, 75 rows without package-scope static evidence |
| Exact production normalization scan | PASS — owned source has `toLowerCase()` and no `toLocaleLowerCase()` |
| Exact retired-identity source/manifest/lock scan | PASS — no executable/live matches; baseline-only historical `cmdk` record as noted above |
| Packet `[DEBUG]` scan | PASS — 0 matches |
| Targeted `git diff --check` | PASS |

## Residuals

No browser/Playwright, Storybook, production build, full monorepo suite, SSR/hydration, or native assistive-technology run was performed. JSDOM validates state authority, filtering/ranking, event precedence, ARIA attributes, focus transitions, and the real Search/Find consumer paths; native pointer-to-focus ordering, screen-reader announcement timing, portal-focus timing, and hydration remain browser-only residuals.

No Cargo/Rust command, Select edit, cache deletion, Git mutation, or ticket metadata operation was performed by this audit.
