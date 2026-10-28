# Law audit wave 1026g (aud-laws-g)

Read-only audit by `aud-laws-g` (Haiku 5.5). Nothing was run, built or edited; no graph-loading commands, no git writes.

## Scope and method

- Lanes: every `result-*-g.md` (1026g) and `result-*-f.md` (1025f) in this ticket folder. Law test paths were extracted from their text (345 path tokens), then matched to test files under `🧰️framework` changed since 2026-10-24 (`🗑️generated/aud-laws-g/recent-tests.txt`, `matched.txt`). 328 files matched; 274 are `.ts`/`.rs` law files (`.py`/`.mjs`/`.json` oracles and fixtures are excluded from the law count).
- (a) negative control: result text names a red-before or mutant run for the owning dir (`neg.tsv`). This is lane-attested only; I did not re-run any red. 102 of 274 law files have such a mention.
- (b) independent oracle: keyword heuristic (spawn, python, oracle, sqlite) over the file. 203 of 274 mention one; independence was not verified per file.
- (c) can it fail: heuristics for zero test declarations, zero asserts, `skipIf`/`describe.skip`, early platform `return`, env gates, mocks (`scores.tsv`, `flags` scans). 25 files declare no test; 35 have a skip, env or platform gate; 13 use mocks. Manual reads for the top candidates.
- (d) official target: suite tables `🏗️build/🗂️suites/🔣️.json` (51 suites), `⚡️caching/🗂️suites/🔣️.json`, `🏃️process/🧪️testing/🗂️suites/🔣️.json`. 167 law files are inside a suite tree, 107 are not. Routed means listed in a suite; it does not mean the target is green.

## Systemic findings

1. Silent greens: at least six law files (ranks 1-6) return early on the other platform (`process.platform!=='darwin'`/`'win32'` then `return`). They pass without asserting anything on Linux or Windows. AGENTS requires macOS, Linux and Windows coverage, so these are fake passes.
2. Never-run Bazel laws: `SEMIO_TEST_BAZEL_BINARY` and `SEMIO_BAZEL_BIN` are read in 29 and 3 files and set by no code in the framework or product trees (about 31,700 source files scanned, `🚀️launcher` not scanned). Their `skipIf` guards therefore skip on every official run. `SEMIO_BAZEL_INTEGRATION` is set only by `🏗️build/📜️script.ts` for child runs. The `SEMIO_GRAPH_WRITER_CONTENDER`, `SEMIO_GRAPH_MAINTENANCE_DEFAULT_SOURCE` and `SEMIO_DEPENDENCY_PLAN_SOURCE_CASE` gates are set only inside a parent test. These child-mode laws run only when that parent runs. None of the 1039 `📋️project.json` files sets any of these gates.
3. Darwin-only laws: about 12 law files are gated `skipIf(platform!=='darwin')` with no routed Linux or Windows twin, mostly in `native-build/💾️storage` (image, mounted-image, fold, heal, ledger). The 16 GB cross-platform goal needs a fake-backend twin for each.
4. Unrouted: 107 law files sit outside every suite tree. Most of the process module's `🏗️build`, `🔌️plugin`, `🖱️ui`, `📁️filesystem` and the library-level `🧪️tests` laws (the route-g `library-laws` item) have no official target at all.
5. Reporting shape: about 25 files are scripts or re-export shims rather than named laws (`pixels` companion shims, two vite configs filed under `🧪️tests`, top-level asserts). A red one is reported as a single file failure or not at all.

## Weakest 25

| # | file (repo-relative) | failed criteria | one-line fix | owning lane |
|---|---|---|---|---|
| 1 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🟦️.ts` | c: darwin-only, early `return` on other platforms = silent pass; d: unrouted | Turn the early `return` into an explicit skip the suite reports, and add a Linux branch against a fake statvfs | alloc-f |
| 2 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/♻️pressure/🟦️.ts` | c: darwin-only early `return` = silent pass off macOS; d: unrouted | Same as rank 1 | alloc-f |
| 3 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📦️generation/🧪️tests/🟦️.ts` | c: darwin-only early `return` (silent pass); d: unrouted | Same as rank 1; route into a darwin-only suite | ver-f |
| 4 | `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🔒️busy/🟦️.ts` | c: win32 early `return` (silent pass on Windows); d: unrouted | Real Windows branch or a routed windows suite that reports skipped, not passed | churn-g |
| 5 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🟦️.ts` | c: win32 early `return` in the unit-cache law (silent pass on Windows) | Real Windows assertion for the rename/lock case instead of a return | xplat-g |
| 6 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🧪️tests/🟦️.ts` | c: win32 early `return` at line 87 (silent pass on Windows) | Assert the Windows expectation explicitly | xplat-g |
| 7 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts` | c: `skipIf(!SEMIO_TEST_BAZEL_BINARY)` (lines 72, 109); no setter in the repo, so these laws never run on the official route | Bind the Bazel binary from `repo tools setup` into the test-suite target; fail the suite when it is absent | bzsrv-g |
| 8 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🦀️cargo/🧪️tests/🟦️.ts` | c: `skipIf(!SEMIO_BAZEL_BIN)` (line 68); SEMIO_BAZEL_BIN has no setter; never runs on the official route | Same binding as rank 7; no silent skip | route2-g |
| 9 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/♻️scoped/🔄️lifetime/🍎️nx-recovery-after-maintenance/🟦️.ts` | c: `test.skipIf(platform!=='darwin')`, silent on Linux and Windows; d: unrouted | Name it as a darwin suite and add a Linux fake-backend twin | purity-f |
| 10 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤫️silence/🧪️tests/🟦️.ts` | c: darwin-only skipIf x2 (lines 85, 103); d: unrouted | Linux twin against a fake image backend; real case in a routed darwin suite | ver-f |
| 11 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🧪️tests/🟦️.ts` | c: darwin-only skipIf (line 34); d: unrouted | Same as rank 10 | store-g |
| 12 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🍎️mounted-image/✍️sealing/🧪️tests/🟦️.ts` | c: darwin-only skipIf x2 (lines 10, 20); d: unrouted | Same as rank 10 | store-g |
| 13 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🧪️tests/🪞️fold/🟦️.ts` | c: darwin-only skipIf (line 35); d: unrouted | Same as rank 10 | store-g |
| 14 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🩺️heal/🟦️.ts` | c: darwin-only skipIf x2 (lines 76, 91); the only heal proof is a darwin-only real mount | Routed darwin suite plus a Linux fake-mount law that fails on a wrong heal | ver-f |
| 15 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts` | c: darwin-only skipIf x2 (lines 25, 78) and 3 mock hits | Replace mocks with a seeded real ledger; Linux twin | cache-f |
| 16 | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📡️sampling/🟦️.ts` | c: darwin-only skipIf (line 59); d: unrouted | Route to a process suite; Linux statvfs twin | ram-g |
| 17 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts` | c: 8 mock hits and env gate `SEMIO_CARGO_PREPARATION_STORAGE` (line 27); d: unrouted (25 tests) | Real temp workspace instead of mocks; route to the cargo suite; declare the env as a fixture | cache-f |
| 18 | `🧰️framework/🔨️modules/🏃️process/🔒️leases/⏳️wait/🧪️tests/🟦️.ts` | c: 2 mock hits; d: unrouted | Drive the wait with real lease holder processes; route it | lease-g |
| 19 | `🧰️framework/🔨️modules/🏃️process/🔒️leases/⚖️fairness/🧪️tests/🟦️.ts` | c: 2 mock hits; d: unrouted | Same as rank 18 | lease-g |
| 20 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️typescript-package-commands/🏗️build-and-direction-routes/🟦️.ts` | c: 146 lines and 0 test declarations; checks run as one script, so a red check has no name; d: unrouted | Wrap each check in a named test() | knot-f |
| 21 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟠️mutations/🟦️.ts` | c: 0 test declarations; mutations are not each shown to fail the law | Each mutation is a test that must fail (negative control as a test) | knot-f |
| 22 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🌱️birth/🟦️.ts` | c: 0 test declarations (39 lines of top-level code) | Wrap in named test() calls | route2-g |
| 23 | `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🟦️.ts` | c: 0 test declarations, top-level node:assert; d: unrouted | Wrap the asserts in named test() calls; route to the filesystem suite | ver-f |
| 24 | `🧰️framework/🔨️modules/🔲️pixels/🧪️tests/🥣️companion/🖊️coverage/🦀️.rs`<br>`🧰️framework/🔨️modules/🔲️pixels/🧪️tests/🥣️companion/🗂️layers/🦀️.rs`<br>`🧰️framework/🔨️modules/🔲️pixels/🧪️tests/🥣️companion/🧩️compositing/🦀️.rs` | c: re-export shims with 0 #[test]; d: unrouted | Delete the shims or put the laws in the pixels crate with #[test]; route | ext-f |
| 25 | `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`<br>`🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | c: vite config filed under a test folder, 0 tests (not a law) | Move out of `🧪️tests`; drop from the law set | purity-f |

Owners are the lane that last changed the file (from the lane result files). Ranks 1-6 are silent passes, 7-8 are never-run Bazel laws, 9-16 are darwin-only, 17-19 are mock or unrouted, 20-23 declare no tests, 24-25 are shims or misfiled config.

## Caveats

- No law was executed. (a) is what the lanes wrote, not re-verified. (b) is keyword-level. Counts are per file, not per test case.
- The matcher only covers files under `🧰️framework` (where the build library and the process module live). Law files in `🚀️launcher` and under the product trees outside `🧰️framework` are not counted.
- The line-based test counter can miss minified one-line files. Zero-declaration rows were checked by reading the declarations (`conformance` and `nx-recovery` checked by hand).
- Tool outputs: `C/aud-laws-g-tools/` (extract, match, score, neg, flags, show). Generated tables: `C/🗑️generated/aud-laws-g/` (`scores.tsv`, `neg.tsv`, `matched.txt`, `weakest-final.json`).
