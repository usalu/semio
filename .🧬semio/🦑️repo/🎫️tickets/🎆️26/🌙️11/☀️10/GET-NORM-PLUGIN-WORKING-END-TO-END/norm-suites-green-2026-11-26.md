# Norm suites run, 2026-11-26 (agent norm-suites)

## JS/TS contracts (repo test-file, bun)
- Plugin `✏️s/🔌️plugins/📕️norm/🧪️tests`: 29 files, 28 passed in the first run, about 12,070 test cases, 0 failing assertions. wire-twins alone ran 11,956.
- `🧬️schema-typecheck` was the one file reported failed with no output (host memory pressure during a 175 s run). A solo rerun passed (1 test, 26.7 s). No source defect found, nothing changed.
- `✏️s/🧑‍💻dev/📕️norm/🧪️tests` and `🌎️hub/🧩️compositions/📕️norm/🧪️tests/🔬️rust-package-commands`: both files pass (0 test cases, load-only).
- The 12,093 / 49-file total in the history includes the artifact TS packages, which were not rerun here.

## Rust
- `repo show project` and `show projects` refuse without a sealed graph (policy current, no inference allowed), so there was no target discovery.
- `repo cargo test -p semio-hub-norm` from the root failed: the package is not in the root workspace. Run it with `--manifest-path 🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/Cargo.toml`.
- Two attempts at that command were admitted at 1536 MB. Both ended in the host warden cancelling the cargo ingress helper (a bun --eval process of about 1.1 GB) after proc-macro2/syn started compiling. The cause was critical host memory pressure while another agent's native-exec (2-3 GB) and command jobs were running. Nothing compiled to completion, so there are no Rust results.
- This is not a Norm defect. It is host contention with other agents' jobs, and the plugin workspace Rust crates were not run.

## Rust follow-up (coordinator request)

### Attempts through `repo cargo` (prototype, shared store, parallelism 1, locked offline, hub norm via `--manifest-path`)
| # | Wait for admission | Outcome |
|---|---|---|
| 1 | about 0 s | Warden cancelled the `bun --eval` helper (1,143 MB) at the first unit (`proc-macro2`). |
| 2 | about 6 min | Warden cancelled `bun --eval` (1,076 MB) while `syn` compiled. |
| 3 | about 23 min | Warden cancelled `bun --eval` (1,062 MB) while `syn` waited for a 42 MB grant growth. |
| 4 | about 36 min | `cargo:selected-dependencies` killed by SIGTERM during native owner preparation (host pressure). |
| 5 | about 25 min | Warden cancelled `bun --eval` (1,095 MB) at `proc-macro2`. |

No unit past `syn` ever compiled, so no Rust test results exist for `semio-hub-norm`, and the plugin workspace crates were not attempted. At the time of the last check, `vm.swapusage` was 7,784 of 8,192 MB used, and `memory_pressure` reported 54-57% free. The host never became calm: a Monitor loop waiting for 60% free for 3 consecutive samples never completed. Other agents' `native-exec` and `command` jobs (2-3 GB each) held the capacity gate continuously.

### The 1.1 GB `bun --eval` helper
- It is the captured-source-closure loader (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts`, `sourceClosureLoaderV1`). The cargo ingress (`rustc-wrapper = repo`) launches a Bun process, and the process evaluates that loader inline. The loader decodes a snapshot of the whole captured module graph, hashes it, runs each module through a Bun transpiler and keeps the images in memory. That is why one wrapper costs about 1 GB.
- The warden counts it as the largest process in the cargo tree, so it is the first victim whenever the host goes critical. The cancellation is the designed reaction to host pressure, not a defect in the Norm crates.
- No safe local fix. Shrinking it would mean changing the closure design, for example a lazy/streamed image or a smaller captured entry per wrapper, and that is repo-infrastructure work owned by other in-flight tickets. Not changed.

### Retry recipe once the host is calm
`repo cargo --configuration=prototype --store=shared --parallelism=1 -- test --manifest-path 🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/Cargo.toml --locked --offline`, then the same with `--manifest-path ✏️s/🔌️plugins/📕️norm/Cargo.toml --workspace` or one `-p` per member (the workspace lists about 42 members).
