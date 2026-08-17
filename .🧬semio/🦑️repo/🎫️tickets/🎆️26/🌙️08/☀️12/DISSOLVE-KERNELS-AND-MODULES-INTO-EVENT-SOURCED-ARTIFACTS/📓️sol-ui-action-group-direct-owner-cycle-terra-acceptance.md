# UI Action Group Direct-Owner Cycle Break Acceptance

## Lease Outcome

ActionGroup no longer reaches the React barrel at runtime. Its three presentation values now come from their specific UI module owners; component behavior, contracts, callbacks, class composition, and exports are unchanged.

## File Inventory and Hashes

| Status | Path | SHA-256 |
| --- | --- | --- |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🟦️component.tsx | b798641562d12be9eddb6e8cbdf25f321747f0c14616a649f924aa456e090eed |
| Direct owner, unchanged | 🧰️framework/🔨️modules/🖱️ui/🔨️modules/🎛️chrome-control-presentation/🟦️component.ts | b903d9d981f033d97ec16f0f28c765b61e53b80923579f183407fe9f8c1d6f17 |
| Direct owner, unchanged | 🧰️framework/🔨️modules/🖱️ui/🔨️modules/🌀️status-border-presentation/🟦️component.ts | f917be9da1cb4eda4f81dbb1857863380d60efab6f919e8378dadb9da67f6548 |

The accepted ActionGroup baseline was `1978fe34c166fceba6be70cd478d65b66a5e85e3301347cde77e077b7287e9b4` before this lease. Both owner hashes are unchanged before and after the import-only edit.

## Exact Diff and Static Acceptance

- Removed the React barrel import of `chromeControlGroupShellClass`, `loadingBorderElementClass`, and `waitingBorderElementClass`.
- Added `chromeControlGroupShellClass` alongside `chromeControlItemBaseClass` from `🎛️chrome-control-presentation`.
- Added `loadingBorderElementClass` and `waitingBorderElementClass` from `🌀️status-border-presentation`.
- ActionGroup has zero React-barrel imports. Its sole three values retain the original use sites: group shell composition, loading border, and waiting border.
- No barrel, public API, behavior, story, callback, function, type, or class composition was edited.
- Scoped ordinary and cached `git diff --check` commands completed cleanly.

## Nx Validation

Each required target ran once through uncached Nx with `--skip-nx-cache`.

| Command | Outcome |
| --- | --- |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | Passed; only the existing `NO_COLOR`/`FORCE_COLOR` warning was emitted. |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | Ran 520 tests: 510 passed, 10 failed, and 2 jsdom unhandled errors. The failures are the established Scene camera mock, icon hover CSS, CanvasPickMenu, shell measurement, Tree, VirtualFileSystem, and pointer-event cases. |

## Blockers

No ActionGroup cycle-break source blocker remains. The non-passing quick-test target is blocked by the established unrelated failures and jsdom errors.
