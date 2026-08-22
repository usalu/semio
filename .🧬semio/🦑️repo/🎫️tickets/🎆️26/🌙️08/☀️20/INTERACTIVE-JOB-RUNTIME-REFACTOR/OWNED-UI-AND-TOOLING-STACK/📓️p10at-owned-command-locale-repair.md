# P10at Owned Command Locale Repair

## Verdict: AUDIT-READY

The sole P1 from the independent Command audit is repaired. Owned Command normalization now uses Unicode `toLowerCase()` after NFKD decomposition and combining-mark removal, so filter and ranking results do not inherit the host's default locale.

## Repair

- Replaced the default-locale-dependent `toLocaleLowerCase()` call in `⌨️Command/🟦️component.tsx` with locale-invariant `toLowerCase()`.
- Added a focused regression that compares the owned behavior with explicit Turkish locale-sensitive output without mutating process locale.
- The regression proves explicit Turkish lowercasing changes `Istanbul` to `ıstanbul`, while the owned ranker gives exact score `10_000` to both `Istanbul` and `İSTANBUL` for the query `istanbul`. It also confirms dotless `ı` remains a distinct Unicode character and is not silently collapsed into ASCII `i`.
- No public contract, ranking tier, selected-value behavior, Dialog composition, consumer integration, or dependency surface changed.

## Fresh Gate Evidence

| Gate                                                                   | Result                                                                                                                              |
| ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Focused Command matrix                                                 | PASS — 1 file, 9 tests                                                                                                              |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`            | PASS                                                                                                                                |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`           | PASS — 18 files, 662 tests                                                                                                          |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`                 | PASS — only existing Bun color-env warning                                                                                          |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache`  | PASS — 0 violations, 2 allowlisted files                                                                                            |
| `bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache` | PASS — 4 files, 438 tests                                                                                                           |
| Frozen lockfile-only install with lifecycle scripts disabled           | PASS                                                                                                                                |
| Dependency ratchet                                                     | PASS — historical 238, current 144, removed 94, no additions                                                                        |
| JavaScript dependency list                                             | PASS — 81 identities                                                                                                                |
| JavaScript dependency parity                                           | PASS — 83 manifests, 266 external rows, 117 evidenced rows, 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 lock workspaces |
| Manifest/source audit                                                  | PASS — 64 manifests, 578 direct rows, 266 external rows, 75 rows without owned-scope evidence                                       |
| Exact executable source/manifest removal scan                          | PASS — 0 `cmdk`, `CommandPrimitive`, or `[cmdk-item]` matches in framework/hub TypeScript, JavaScript, and JSON                     |
| Exact `bun.lock` removal scan                                          | PASS — 0 `cmdk` or `@radix-ui/react-dialog` matches                                                                                 |
| Production normalization scan                                          | PASS — owned Command source contains `toLowerCase()` and no `toLocaleLowerCase()`                                                   |
| Packet `[DEBUG]` scan                                                  | PASS — 0 matches                                                                                                                    |
| Exact-file Prettier check                                              | PASS                                                                                                                                |
| Targeted `git diff --check`                                            | PASS                                                                                                                                |

## Changed Paths

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️component.test.tsx`
- `OWNED-UI-AND-TOOLING-STACK/📓️p10ar-owned-command.md`
- this report.

## Residuals And Scope

The browser-only residuals from P10ar remain: native pointer-to-focus ordering, assistive-technology announcement timing, portal focus timing, and hydration were not exercised by JSDOM. No Select or Cargo path was touched, and no Git mutation or ticket metadata operation was performed.
