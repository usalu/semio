# Semio Local Data Relocation

## One-Time Operation

The tracked `.🦑️repo` tree was reconstructed from `HEAD` with `git archive` in a ticket-local staging directory and merged into `.🧬semio/🦑️repo`.

- 15,526 tracked source files reconstructed from Git
- Existing files in `.🧬semio/🦑️repo` retained as canonical
- Missing Git files moved into the new location
- Old `.🦑️repo` path removed
- No compatibility symlink retained
- No `*.legacy*` conflict artifacts retained
- No permanent migration API or setup command retained

## Preservation

All 15,526 `.🦑️repo` Git status entries were byte-for-byte identical before and after relocation. Concurrent unrelated changes were not touched or reverted; the comparison is in `📓️repo-reset-status-comparison.md`.

## Verification

- `.🦑️repo` is absent.
- `.🧬semio/🦑️repo` is present.
- The local data tree contains zero `*.legacy*` artifacts.
- `git diff --check` passes for the affected source files.

## Git Relocation Correction

The broad `.🧬semio/` ignore initially hid the destination, making Git report 15,526 deletions. The ignore boundary now keeps only runtime data ignored:

- `.🧬semio/🌐hub/`
- `.🧬semio/🔗space/`
- `.🧬semio/🗺️map/`
- `.🧬semio/🦑️repo/⚡️cache/`
- `.🧬semio/🦑️repo/📊️metrics/coverage/`
- `.🧬semio/🦑️repo/🧹️tmp/`

The index was updated from the exact `HEAD` modes and blob IDs:

- 15,526 `R100` renames
- 0 staged deletions
- 0 staged additions
- 0 changed lines in the relocation
- 0 old `.🦑️repo` index entries
- 15,526 new `.🧬semio/🦑️repo` index entries
- Exact mode/blob equality between every old and new index entry

Eight newer destination modifications remain unstaged on top of their staged renames. Current new ticket files remain untracked. This preserves concurrent work separately from the exact relocation. The staged index outside the explicitly affected paths was unchanged.

## Stale Process and Binary Cleanup

Three repo MCP clients were still running a binary built at 12:17, before the repo-path source changed at 14:50. The stale executable embedded `.🦑️repo` seven times and `.🧬semio/🦑️repo` zero times.

Cleanup:

- Terminated three stale repo MCP clients and their three launchers.
- Terminated five orphaned polling shells with hard-coded `.🦑️repo/🎫️tickets` paths.
- Safely merged the three files recreated under `.🦑️repo` into the canonical ticket tree.
- Deleted the stale executable and rebuilt it from current source.
- Changed both setup and MCP launch paths to build the repo client before execution, even when a binary already exists.

Verification:

- Rebuilt binary: `.🦑️repo=0`, `.🦑repo=0`, `.repo/🎫️tickets=0`, `.🧬semio/🦑️repo=5`.
- `bun ./📜️script.ts dev mcp stdio client < /dev/null` rebuilt the executable before launch.
- `TestRepoMetaDirUsesSemioRoot` passed (`go test`, 0.737s).
- No stale repo MCP client, launcher, or old-path polling process remains.
- `.🦑️repo` remained absent after rebuild and launch verification.
