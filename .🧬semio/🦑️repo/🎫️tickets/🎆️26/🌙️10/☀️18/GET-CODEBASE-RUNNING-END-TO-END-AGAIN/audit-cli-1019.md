# Audit cli 1019: `bun repo` help versus catalogue, README, dashboard registry, CI

Scope: read-only audit by `aud-cli`. Only `bun repo --help` and `bun repo <command> --help` were run. No source file was edited. Captured help texts: `🗑️generated/aud-cli-*.txt` (root, 13 top-level commands, 8 subcommands; 22 files). All paths below are relative to `/Users/ueli/Documents/semio`. `B/` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`. `R/` = `🧰️framework/🛍️products/🦑️repo/`.

Method: static reading of the validator, dispatch and translation code, plus the documents. Items marked "static" were not run.

## 1. What the captured help shows

- Every subcommand `--help` prints the same root usage line (all 22 files identical). There is no per-command help.
- `bun repo preflight --help` exits 1 with a stack trace: `error: Expected repo preflight without arguments` (`B/🩺️preflight/🟦️.ts:17`).
- `bun repo --help` writes `[script] {...}` stage JSON lines and a `$ bun ./…/📜️script.ts repo --help` echo to stdout, around the usage line (`🗑️generated/aud-cli-root.txt`). CI prints this at `.github/workflows/build-system.yml:117`.
- The usage line (`B/📜️script.ts:90`) lists no flags at all. Documented flags are absent from it (see 3.4).

## 2. Operation coverage (catalogue, README, help, registry)

| Operation | Catalogue | README EN (165-178) | Help (root line) | Registry | Result |
|---|---|---|---|---|---|
| run, run-many, affected, exec | yes | yes | yes | yes (`build-task`, `run-targets`, `build-affected`, `build-exec`) | ok |
| `--dry-run` | dryRun flag | yes | no | yes (`build-exec` 352, `build-affected` 288) | help missing |
| graph, show | yes | yes | yes | yes | ok |
| cache status/clear | yes | yes | yes | yes (338, 345) | ok |
| `--tool` | not in catalogue | yes (161) | yes, leading position | axis `build-tool` | catalogue gap |
| preflight | yes | yes | yes | no | registry gap (see 3.6 item 21) |
| dependencies sync/lock | yes | yes | yes | no (`dependencies` at registry line 116 is an axis, not a tool) | registry gap (see 3.6 item 21) |
| reset, tools setup, cargo, watch | yes | yes | yes | yes | ok |

No documented top-level command is missing from the CLI. No CLI command is undocumented in the README.

## 3. Mismatches with fix list

### 3.1 CLI help (`B/📜️script.ts`, `B/🧭️invocation/🟦️.ts`, `B/🩺️preflight/🟦️.ts`)

1. Per-command help is missing. Every `<cmd> --help` prints the root line. Fix: render usage per command (`B/📜️script.ts:90`), or at least one line per command including its flags.
2. `preflight --help` crashes. Fix: accept `--help` before the argument check in `B/🩺️preflight/🟦️.ts:17`, and make the help detector cover `preflight` (`B/🧭️invocation/🟦️.ts:19`; route at `B/📜️script.ts:91`).
3. Root usage line (`B/📜️script.ts:90`) omits flags documented in README 180-191 and in the catalogue notes: `--dry-run`/`--graph`, `--base`, `--head`, `--files`, `--all`, `--uncommitted`, `--untracked`, `--skip-cache`, `--skip-remote-cache`, `--exclude-dependencies`, `--parallel`, `--bail`, `--verbose`, `--output`, `-c/--configuration`, `-p/--projects`, `--exclude`, `-t/--targets`, `--include-global-files`.
4. Help output is not clean: `[script]` stage lines and the `$ bun …` echo appear on stdout for `--help` (`🗑️generated/aud-cli-root.txt`). Fix: suppress stage logging on help requests. Source of the `[script]` lines not located (static).

### 3.2 Catalogue (`B/⚙️operations/🔣️catalog.json`)

5. Line 57 (`exec` note) names the Nx variables `NX_PROJECT_NAME` and `NX_PROJECT_ROOT_PATH` in the neutral catalogue. README 170 says a tool-neutral pair is not defined, which contradicts a neutral catalogue. Fix: define a neutral pair (for example `SEMIO_PROJECT_NAME`, `SEMIO_PROJECT_ROOT`) exported by both adapters, or move the note to the Nx section.
6. `--tool` is a global selector (README 161) but has no catalogue entry. Minor: add it to the catalogue as a global option.

### 3.3 README EN (`R/README.md`)

7. Line 189: "`--dry-run` (alias `--graph`)" is not an alias on Nx. `B/🧭️invocation/🟦️.ts:142` maps only `--dry-run` (to `--graph=stdout`, line 148). A bare `--graph` or `--graph=<file>` passes to Nx unchanged, where Nx opens its graph viewer or writes its own HTML. Bazel honours `--graph` (`B/📜️script.ts:200-202`). Fix: translate `--graph` in `nxBuildArgumentsV1` as well, or document the difference. Static; not run.
8. Line 190: `--all` is listed only as "`affected` change selection", but `dependencies sync|lock --all` (lines 141-142) uses it. Fix: list `dependencies` in that row.
9. Lines 169 and 190: `--base`, `--head`, `--files`, `--uncommitted`, `--untracked`, `--all` are documented for `affected`. The shared validator accepts them for every operation (`B/🧭️selection/🟦️.ts:153-154`), so `run` or `exec` silently ignore them. Fix: refuse them outside `affected`, or document the acceptance.
10. Undocumented accepted flags: `--output-style` (`B/🧭️selection/🟦️.ts:163`), `--exclude-task-dependencies` (`B/🧭️selection/🟦️.ts:154`, `B/🧭️invocation/🟦️.ts:142`), `--help` (`B/🧭️selection/🟦️.ts:154`). Fix: document them as aliases or remove them.
11. Line 130 says the German version "follows its last section". The EN chapter's last section is "Go layout" (411-418), which the DE part does not contain (see 3.4 item 15). Fix together with item 15.

### 3.4 README DE parity (`R/README.md` 419-604)

12. The DE chapter has no command block. EN 132-144 has no counterpart. Fix: add the same `bash` block after the DE intro (around 421-427).
13. The DE chapter has no flags table. EN 182-191 has none in DE; DE 454 is a single bullet. Fix: add the table after DE 454.
14. The DE chapter lacks the EN example at line 348 (`bun repo run @semio-tech/semio-tech-play:serve --tool=bazel`). Fix: add it under "Produktketten" (DE 574-584).
15. The DE chapter lacks EN "Developer Dashboard" (405-410) and "Go layout" (411-418). The DE part ends at 604, before `# 🔨️ Modules` (605). Fix: add both sections in German.
16. The DE operations table merges rows: `run`/`run-many`, `watch`/`graph`/`show`, `tools setup`/`preflight`/`dependencies`/`cargo` (DE 444-453), while EN has 12 rows (165-178). Notes for the merged rows are lost. Fix: one row per EN row.
17. Verification status is a table in EN (350-367, Claim | Evidence) and prose in DE (585-588). Fix: same table form in DE.

### 3.5 CI (`.github/workflows/build-system.yml`)

18. No invocation mismatch. Every `bun repo` form used exists: `--help` (117), `preflight` (119, 254), `tools setup all` (121, 256), `dependencies sync --all` (134, 275), `--tool=<t> cache status|clear` (148, 158), `--tool=<t> run $PROOF_TARGET` (160, 163, 277, 279).
19. Fragility, not a mismatch: the cache proof parses `CacheStatus` by searching `{\n  "version"` in the output (`build-system.yml` around 141-146). This depends on pretty-printed output and on `[script]` lines not interleaving (item 4).

### 3.6 Dashboard registry (`📋️project.json`) against `R/…` cmdreg report

20. Consistent: `build-exec` (352, with the `dry-run` choice before `--`), `build-cache-status` (338), `build-cache-clear` (345), `build-affected` dry-run flag (288), `run-targets` (658-671, `run-many`), `build-graph`/`build-projects`/`build-targets`/`build-project`/`build-help`/`build-reset`/`build-tools`/`cargo` all match the CLI.
21. Not in the registry: `preflight` and `dependencies sync|lock`. Either add tools (`build-preflight`, `build-deps-sync`, `build-deps-lock`) or record in the cmdreg report that they are CLI-only on purpose.

## 4. Open decisions for the owner

- 3.1 item 2 and 3.2 item 5 need a decision on the neutral env pair before a fix.
- 3.3 item 9: refuse or document the affected-only flags for other operations.
- 3.6 item 21: registry coverage of preflight and dependencies.

## 5. Not run

- No `bun repo` command other than `--help` was run. No runtime check of `--graph` on Nx (item 7 is static). No CI run.
