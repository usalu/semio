# Adopt Path Matches Bootstrap Settle on macOS

Source of the three defects: `🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-m07-join.md`, "Findings For The Fleet Code".
All paths below are inside `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree`.

## What changed

### 1. `join --adopt` renames non-NFC names first

- `🔁️replica/🥾️bootstrap/🟦️.ts`: the private `healNamesV1` is now the exported `normaliseNamesV1(worktree, ignore, signal, sink, stage)`. It returns the new paths and reports each rename as `skipped` / `renamed-to-nfc` with the stage of its caller. `bootstrap-settle` calls it with `settle`, the adoption with `adopt`. One implementation, no copy.
- `🔁️replica/🧲️adopt/🟦️.ts`: `adoptV1` calls it before every survey. The renames go through the adoption journal, so the report lists them (`renamed`) also after a crash and a rerun.
- The dry run renames nothing. The survey reads every refused `not-nfc` name under its NFC name (`aliasesV1`, a path resolver for the disk reads), so the plan equals the plan after the rename, and lists the names in `renamed`.
- Plan and report carry `renamed`; the report schema (`🧬️schema/🔣️.json`) requires it; the markdown report and the CLI print it (English and German texts).

### 2. A differing executable bit is a difference

- `sameContent` compares the bit where both sides know it. Equal bytes with another bit are no longer `same`.
- The evidence is about bytes and bit together: journal history keys, the git base (`ls-tree` mode), the fleet's commits (`log --raw` modes) and the fleet identity all carry the bit. So the normal rule decides: the fleet had this bit before, or it is unchanged since the base: `stale`, the fleet's bit wins. Set here on an unchanged fleet version: `edit`, published. Otherwise `conflict`.
- A `modeless` replica (Windows) compares bytes only, as before.
- Decisions carry `modeOnly`; the CLI prints "(same bytes, only the executable bit differs)".
- The incident (`🗺️topology/📜️script.ts`, `0755` in the archive, cleared in the fleet) now classifies as `stale` / `journal`: the local bit loses and nothing is published.

### 3. `--plan` extracts nothing, and an extracted tree is settled

- `🗺️topology/🥾️provision/🐚️.sh` and `🔵️.ps1`: `--plan` / `-Plan` on an empty worktree stops before the archive is fetched.
- Both scripts write `.cache/f/bootstrap-extract`: `started` before tar, `complete` after it, removed after `join`. Until the machine has joined, a later run settles a `complete` tree (never adopts it) and extracts again over a `started` one. This replaces "a worktree from an interrupted extraction must be emptied by hand".
- A tree extracted by the old script has no marker and is still adopted; with changes 1 and 2 that ends in the same state as settle.

## Tests

Written before the implementation, next to the existing ones (they were first run after it, so they were not seen failing against the old code):

- `🔁️replica/🧲️adopt/🧪️tests/🧫️fixtures/🔣️.json`: 7 language-agnostic vectors for the executable bit; `🐍️.py` (independent oracle) decides the same.
- `🔁️replica/🧲️adopt/🧪️tests/🟦️.ts`: dry run and real adoption of a tree with NFD names (nested) and three executable-bit cases; checks the names on disk, the report against the schema and the Python oracle, the modes on both machines after convergence, the events the adopter published (only the one edit) and `verify`.
- `🔁️replica/🧲️adopt/🧪️tests/⌨️command.ts`: the CLI lines in English and German.
- `🗺️topology/🧪️tests/🟦️.ts`: runs step 1 and 2 of the shell script against a real tar: plan on an empty tree, first run, rerun, interrupted run, plan after an interrupted run, joined machine, own repository; order checks for the PowerShell script.

Also changed in tests: the copy filters of the adopt and relocate tests matched `.cache` anywhere in the absolute path, so they copied nothing when the scratch folder itself is below `.cache` (as under `repo tools exec`). They now look at the path inside the worktree.

## Results

Run with `repo tools exec bun -- test ./<file>` on g1-m07 (macOS, APFS):

| Files | Result |
| --- | --- |
| adopt `🟦️.ts`, `⌨️command.ts` | 30 pass, 0 fail |
| topology `🟦️.ts`, `🔵️.ts`, bootstrap `🟦️.ts`, relocate `🟦️.ts` | 92 pass, 20 skipped, 0 fail |
| replica `🟦️.ts`, `🌿️git.ts`, `🧭️agree.ts`, `💾️store.ts`, `🧷️sidecars.ts`, `🛡️resilience.ts`, `🛟️boot.ts`, `🏁️race.ts`, state, clash, blobs, entry, sidecars | 245 pass, 0 fail |
| replica `🌪️chaos.ts`, `💥️killpoints.ts` | 13 pass, 1 skipped, 0 fail |

## Not verified

- **The graph route `repo run <project>:<target>` was not used.** `repo show projects` and `repo show project` waited more than nine minutes for a capacity grant (graph plan wants 6627 MB, 6440 MB were free) and were stopped. The files above ran through the tool registry instead.
- **The PowerShell script was not executed.** There is no `pwsh` on this Mac; the 20 skipped tests are its harness. Only the text and order of the new lines are asserted.
- `tsc` over the changed files reports 32 errors in the module; none is in a line of this change (they are in `🩺️attest`, `🔁️replica/🟦️.ts`, `🌿️git`, `🚧️boundary` and older test lines).
- Not run against the real fleet store; the daemons restart on the changed code digest as usual.

## Left as it is

- Plain `join` (after settle) still adopts a matching file without comparing the bit; settle has already equalised the bits before it.
