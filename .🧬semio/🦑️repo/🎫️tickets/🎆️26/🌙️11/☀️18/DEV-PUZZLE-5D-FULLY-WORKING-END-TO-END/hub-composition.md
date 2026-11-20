# Hub puzzle composition status

No source fixes were made. All results below are incomplete: the host was memory-starved for the whole session.

## Findings
- The retained project `@semio-tech/hub-puzzle-retained-test` has `test-ownership` and `test-native`. `test-native` only reruns each artifact's own `:test` target and does not run the hub crate.
- The surface tests are not a separate cargo test target. `🌎️hub/🧩️compositions/🧩️puzzle/🦀️.rs` mounts them as `mod surface_tests` (a `#[path]` module of the lib), so they run under `cargo test -p semio-hub-puzzle --lib`.
- The crate `semio-hub-puzzle` is a member of the hub workspace `🌎️hub/Cargo.toml`, not the root workspace. `repo cargo ... -p semio-hub-puzzle` therefore has to run with cwd `🌎️hub`. From the repo root it fails with "package ID specification did not match".
- The wasm target of npm package `@semio-tech/puzzle-wasm` is project `@semio-tech/puzzle-plugin`, target `wasm`. Its other targets are `test`, `fixtures-lint`, `canonical-architecture` and `test-guest-io`.

## Runs
1. `repo run @semio-tech/hub-puzzle-retained-test:test-ownership --lang=en` failed with the resident-service "Error: closed" (ref dee8ab03). The coordinator says this is transient; I did not rerun it.
2. `repo run @semio-tech/puzzle-plugin:test` sat in "Reading the project graph slice" for over 2900 s and never started.
3. `repo cargo --store=shared --configuration=prototype --lang=en -- test -p semio-hub-puzzle --lib`, run from `🌎️hub`, compiled about 1320 crates with warnings only and no errors, including the puzzle-5d crates and `semio-hub-puzzle`. It then stalled in `zig ld.lld` linking the test binary for over 42 minutes (about 9% CPU, 13 of 15 GB RAM used, swapping). It was still running at the time of writing. Log: `🗑️generated/hub-cargo.log`.

## Blockers
- Host memory pressure.
- Whether the 5D surface tests pass is unknown.
- The `puzzle-plugin:wasm` and `test` targets were not run.

## Update: cargo run finished (exit 1)
`semio-hub-puzzle --lib`: 5 passed, 4 failed. Passed: the three `*_editor_and_viewer_share_dialect` tests (including 5D) and the two io-routes tests.

Failed:
- `puzzle{2d,3d,5d}_viewer_never_mutates`. The panic is at `semio-framework-job-testing-caller` `🦀️.rs:106`: "original fixture context origin refresh: ... WorkLimit: original Source clock declined new construction". It hits 2D, 3D and 5D alike, so it is not specific to 5D.
- `guest_codec_tables_answer_like_the_declared_native_codecs`. The fault is "artifact codec requires an original retained application job" (module.vcs). It first surfaced for `board.ports.directed.v1`.

All four failures come from the framework job and original-clock test harness, which other agents are porting to new APIs. I did not change anything. Re-run after that port lands.
