# N5: ground truth of `workspace:setup` on g1-server (14 Nov 2026, 11:46 - 12:40 CET)

Observer only, no code changed. Variant script: `🧰️scripts/N5/run.sh <n> [diag]` (worker environment of `native-bootstrap.sh`, worktree `/Users/Shared/semio`, no marker, `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=16384`). Logs: `n5-logs/n5-run-2.log`, `n5-run-3.log` and the `.mem.txt` samples (every 30 s, three largest processes machine-wide, memory-free percentage, swap, load). **The first two logs were lost**: the folder `🗑️generated` that the task named was deleted by someone at about 12:17 while run 1b was in flight (the log became an unlinked file; I stopped that run and restarted it). Hence the logs live in `n5-logs/`, and run 1 and run 1b are quoted from my terminal, not from a file. Times in the logs are UTC (CET = UTC+1).

## Final line

**native_rc = 1 in every run. `workspace:setup` has not been reached in a form that runs a single task.** Failing: `repo-mcp:install` (first step of the bootstrap, before the dependency scopes) and, behind a workaround that skips it, the graph (`The disk is full` of the graph image) before any of the 25 tasks starts.

## 1. What `setup` does (`🔩️native/🥾️bootstrap/🐚️.sh`, 1357 lines)

Order in Main: `persist_linux_session` (Linux only) -> `ensure_native_toolchain` (macOS: Xcode CLT probe; `brew install pkg-config cairo pango` and the DejaVu cask only if `brew` is on PATH; `ensure_user_cc`, `ensure_user_pkg_config`, `CMAKE_PRESET=macos-prototype`; it only logs when something is missing) -> `ensure_launcher` (`🚀️launcher/📥️install/🐚️.sh`: builds the Go `repo` launcher into `~/.local/share/semio/launcher/bin`, downloads the pinned Go if absent, links `~/.local/bin/repo`) -> `link_pinned_go` -> `ensure_node` (pinned Node from `🧬️schema/🔣️.json`, sha256-checked) -> `ensure_rust_toolchain` (rustup if absent, `rustup toolchain install` of `rust-toolchain.toml`) -> `link_installed_rust` -> `ensure_dotnet` (`dotnet-install` of the SDK of `global.json`) -> `ensure_uv` -> `repo_bootstrap`:

1. `repo run repo-mcp:install` (builds the native repo command; this is the Go build of `💻️client/🔌️mcp` and the coordinator)
2. `repo --help`
3. `repo dependencies sync --all`
4. `repo run workspace:setup`
5. `repo run @semio-tech/repo-developer-tools:setup-debugger` (failure only logged)

`SKIP_REPO_BOOTSTRAP=1` skips 1-5. The log line "Homebrew not available" in my runs is the effect of the worker PATH (`/usr/local/bin:/usr/bin:...`, no `/opt/homebrew`), as on a fresh worker.

### The 26 nodes of `workspace:setup` (root `📋️project.json`, `dependsOn` closure, 14 direct)

`workspace:setup` itself: `repo ./📜️script.ts setup`, `cache:false`.

| Task | Command | Result in my runs | Wall | First error (verbatim) | Code location | Owner |
|---|---|---|---|---|---|---|
| `repo-mcp:install` (not in the closure of my diag run; run first by the bootstrap and again as dependency) | native build of `repo` and coordinator | **fail** (runs 1 and 3) | 2 m 17 s (run 1), 23 s (run 3) | `Internal error: Unowned artifact directory: /Users/Shared/semio/.🧬semio/🦑️repo/⚡️cache/🗃️bin/prototype/repo` | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts:80`, second throw: marker owner differs. The marker `.semio-artifact.json` there names `owner: /Users/ueli/Documents/semio/…/💻️client/🔌️mcp:go:prototype`, the coordinator one `/Users/ueli/Documents/semio/…/📦️main:go:prototype` (both written 23/28 Oct, directories copied 11 Nov 04:38). The live tree is `/Users/Shared/semio`, so the owner string differs. | nobody yet |
| `deps-build-tools` | `repo ./📜️script.ts repo tools setup all` | skipped-after-failure (never planned) | - | - | known open from the worker report: nested repo operation, `Bounded complete dependency directory required` | N1 landed the `operation` declaration and partition roles (landings.md 12:31); not run |
| `deps-js-all` | `dependencies/📜️script.ts sync --all` | skipped | - | - (its own `repo dependencies sync --all` in step 3 **passes**, see below) | - | N1 |
| `deps-python` -> `@semio-tech/ui-styling-py:deps`, `@semio-tech/energy-oracle-py:deps` | `sync python` | skipped | - | - | worker report: `uv python install 3.12` vs `python-downloads = "never"` | N3 |
| `deps-cargo` | `sync cargo` | skipped | - | - | - | N1 |
| `deps-go` | `sync go` | skipped | - | - | - | N1 |
| `deps-dotnet` -> `@semio-tech/repo-test-dotnet:deps` | `sync dotnet` | skipped | - | - | - | N3 |
| `deps-cpp` -> `@semio-tech/repo-lib:cpp-setup` | `sync cpp` | skipped | - | - | worker report: `/usr/bin/c++` shim with 78 hard links | N1 |
| `deps-browsers` | `sync browsers` | skipped | - | - | - | nobody yet |
| `deps-wasm` -> `deps-wasm-opt` (`🛠️tools/🕸️wasm/📜️script.ts prepare`), `deps-trunk` (`sync trunk`) | | skipped | - | - | - | nobody yet |
| `deps-tools` | `sync tools` | skipped | - | - | - | N1 |
| `setup-git` | `repo ./📜️script.ts setup git` | skipped | - | - | - | nobody yet |
| `prepare` (noop) -> `@semio-tech/framework-schema:generate`, `@semio-tech/ui-styling-tokens:generate`, `@semio-tech/assets:build`, `@semio-tech/framework-graph:generate`, `@semio-tech/plugin-registry:generate` | | skipped | - | - | - | nobody yet |
| `repo-mcp:install` | see first row | fail | | | | nobody yet |
| `@semio-tech/framework-os-mcp-rs:build` | cargo build | skipped | - | - | worker report: crate path 262 chars | N2 |

"skipped" means: the run died before the task graph started (run 1/3: at `repo-mcp:install`; run 2: reading the project graph slice, see section 2). Nothing in my runs measured a task of the list. The right-hand column of the worker report (runs 8/9) is the last known state of the tasks and is not repeated here.

## 2. The runs

| Run | What | Window (UTC) | Wall | Result |
|---|---|---|---|---|
| 1 | full bootstrap | 10:46:30 - 10:48:47 | 2 m 17 s | rc 1 at step 1 (`repo run repo-mcp:install`): the error above. The launcher built in 2 s, rustup/dotnet/uv probes passed. The Go build downloaded Go 1.26.9 (64 675 343 bytes). |
| 1b | steps 3 + 4 only (`diag`), skipping step 1 | 10:49:49 - about 11:20 | stopped by me after ~30 min | log lost (see top). Dependencies sync finished; `workspace:setup` was reading the graph slice at 23 min. |
| 2 | `diag` again: steps 3 + 4 | 11:20:30 - 11:36:21 | 15 m 51 s | rc 1: `dependencies sync --all` rc 0 in 4 m 59 s; then `repo run workspace:setup` read the project graph slice for 651 s (105 of 701 owners) and failed: `The disk is full: partition / has 91.92 GiB free, so the resident service could not finish writing. Free space with \`repo cache cas gc\` (it works on a full disk), then run the command again.` |
| 3 | full bootstrap again, after N1 and N4 landings | 11:38:49 - 11:39:16 | 27 s | rc 1, the same `Unowned artifact directory` at step 1 (N1 and N4 landed, N2 and N3 reported nothing). |

Total time to the first failure that stops the whole setup: 2 m 17 s (run 1). Total time spent observing: about 53 min of runs.

### Cause of "The disk is full" with 92 GiB free (found, not fixed)
The failing store is not `/`: the graph partition is a fixed-size sparse image `/Users/Shared/semio/.cache/b/g/fixed.dmg` (12 GiB, `maximumPhysicalBytes` 12 884 901 888) mounted at `.cache/b/g/m`: `df -g` shows 11 of 11 GiB, 100 %. 10.7 GiB of it is `inference/selections/<id>/ce` (11 233 812 KiB), `np` 0.36 GiB, `derived-nodes` 0.35 GiB. The budget report in `graph-collector.log` says `graph: 11.96 GiB against a quota of 20.23 GiB (image 12.00 GiB)`: the quota is larger than the image, so the sweep sees "within the budget" and evicts nothing while the image is full. The user-facing message names the host partition (91.92 GiB free), not the image that is full. Any run that must write a slice on this machine now fails the same way. A new worker starts with an empty image; whether one full slice read fits into 12 GiB is unknown (105 of 701 owners were done at 11.2 GiB here, but those include generations of earlier days).

## 3. Memory and capacity

- `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=16384` is read in `🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts:20` (and `📡️sampling/🟦️.ts:166`): it only shrinks, a malformed value means the real host, and the shared sample file is used only by processes declaring the same machine. It is an environment variable, so every child of `run.sh` inherits it; my logs show it took effect in the client gate (`[capacity] admitted dependencies: 1 job(s), 3560 MB reserved (16384 MB available, 2048 MB floor)`). **Not verified:** whether the resident graph service (long-lived, started earlier without the variable) and its slice workers plan with it; the 9 GB process below is a resident-service process and obviously did not stay inside 16 GB.
- Peak resident size seen: **9 026 016 KiB (8.6 GiB) by a `bun --eval …` process at 11:29:33Z, with a second `bun` of 4.1 GB next to it**, in the phase "Reading the project graph slice" of run 2 (260 s into it, "owners 35 of 701"); the next peaks were 5.9 GB at 11:28-11:29 and 5.7-5.8 GB until the end. The sample is machine-wide, so attribution is by time only (the process runs only while my slice read does). On a 16 GB host this is more than half of the machine for one process.
- Lowest free-memory percentage: 80 % at 11:28:33Z. Load average 17 - 31 (other agents).
- `[capacity]` waits longer than 30 s (run 1, queue behind another `projects` run of 14 889 MB; log file lost, quoted from the terminal), the longest verbatim: `[capacity] waiting 52s for run: not enough free memory (need 2372 MB; -3333 MB spare above the 2048 MB floor, load 22.86; 1 running, 0 ahead); behind projects (running, 1 job(s), 14889 MB)`, ended by `[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now`. The 34, 40 and 46 s lines read the same with -2156, -2854, -3065 MB. Runs 2 and 3: no wait.

## 4. Downloads above 50 MB
Only one in all runs: run 1, `Downloading the pinned go 1.26.9 for darwin-arm64 (64675343 bytes).` by the launcher installer (pinned URL in the launcher registry; the log does not print the host). It did not go through the fleet cache or mirror as far as the log shows (no mirror line). Run 3 did not download again (kept copy). The dependency sync of run 2 logged no download (nothing above 50 MB; `Store dependency sync stale … "reason":"installed:."` appeared).

## 5. Second run: what was repeated (run 2 vs run 1b/3)
- `repo dependencies sync --all`: **299 s** in run 2 although run 1b had completed it 10 minutes earlier and nothing in the tree that it reads had changed on purpose; the log starts with `Store dependency sync stale {"reason":"installed:."}` and then 15 600 lines of `bun:dependency-storage ... dependencies:preservation-ownership:<n>` lease activity (one exclusive lease per preserved entry, "elapsedMs":0). It should be a no-op of seconds when the generation already matches.
- Reading the project graph slice: 651 s and 105 of 701 owners before the image was full; run 1b needed over 23 min at the same stage. The slice is not shared from a fleet cache here (no `waited for another machine` or hit lines in the log).
- Launcher build: run 3 took 2 s for the launcher and 23 s to the failure, i.e. the launcher, rustup, dotnet and uv probes are fast on a second run (seconds).
- Run 1 / run 3 spend 4-7 s each in `build:source-admission` and 12 s in `build:implementation-source` before the first real step.

## 6. Reasoning (not measured): 16 GB Mac mini at `/Users/ueli/Documents/semio`
- Longer root: `/Users/ueli/Documents/semio` is 28 characters, `/Users/Shared/semio` 19; the worker report shows the Cargo path envelope refusing a 262-character path there (N2 renamed the crate).
- No toolchains installed: steps `ensure_rust_toolchain`, `ensure_dotnet`, `ensure_uv`, `ensure_node` and the Go download actually run (here all were "already installed" except Go in run 1). Homebrew is not on PATH, so pkg-config/Cairo/Pango are not installed; the log says so and continues.
- macOS consent for `~/Documents`: the first `repo`/bun/cargo process that reads the folder triggers the privacy prompt (Files and Folders), which blocks a non-interactive bootstrap (worker report: `fstatat EPERM`, reference `e07192d7`).
- The `Unowned artifact directory` refusal is a relocation effect here (directories copied from `~/Documents/semio` to `/Users/Shared/semio`). On a worker the `.🧬semio/🦑️repo/⚡️cache/🗃️bin` directory is probably not replicated (cache), so a worker would build it fresh and not meet the owner mismatch; unverified.
- Graph slice: 8.6 GB resident for one process plus 4 GB for another, with the capacity gate at 6144 MB asks: on 16 GB this is the pressure point; the worker report shows runs waiting 1 - 10 minutes at the door.

## 7. What each package should look at
- N1/N4 (artifact publication): the marker owner is an absolute worktree path; a copied or moved tree cannot republish its own `bin/prototype` (`publication/🟦️.ts:80`). Also `repo run repo-mcp:install` is the first step of the bootstrap and fails before everything else.
- N4 (graph): the graph image fills at 12 GiB while the budget quota is 20.23 GiB; the error names `/`. `repo cache cas gc` was not run by me (observer).
- N3 and N2: not reached by any run, so their landings are unverified by this observer.

## 8. Run 3+ status
Run 3 (the one after landings) = native_rc 1 at step 1. Further runs (at most three more, not within 30 min): none made after run 3; the graph image on this machine is full and must be emptied first (I clean nothing). Logs stay in `n5-logs/` for the other agents.
