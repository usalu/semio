# O6: offload planner fixes

LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, planner = `LIB/🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts`.

## 0. Live priority cases (coordinator)
- `unmapped-path: environment NATIVE_PLAY_OUT / __CARGO_TEST_ROOT`: the env loop declined any variable whose value held the cwd. Now `environmentReadNamesV1` scans the shipped `.rs` for `env!/option_env!` names; only those, `CARGO_*`, `RUSTC_*`, `DEP_*`, `OUT_DIR`, `RUST_MIN_STACK`, SDK/deployment-target names and variables pointing into OUT_DIR travel; every other variable is dropped silently. A READ variable with a checkout path still declines. Test: `🧪️tests/🗺️paths/🟦️.ts` (exact NATIVE_PLAY_OUT case).
- `unmapped-path: dependency .../u/l/q.../build/serde_core/.../out`: a prior dependency-file row inside a mapped directory (OUT_DIR, shipped as `virtual.generated`) is skipped (already shipped). Test in the same file.

## 1. serde_core incomplete-inputs
Root cause: `crate_root.rs` declares `mod de/ser/format/private` inside a `macro_rules!` that `lib.rs` expands; rustc resolves them beside lib.rs (`src/de/mod.rs`), the scan looked only below `src/crate_root/`. Fix: `reachedFilesV1` searches the module homes of every ancestor too. Exact read set: new `exactReadsV1` (prior `.d` of the unit plus the slot's K2 recorded gaps, passed from script.ts as `recorded`) added to the shipped closure when inside the shipped root; scanner remains fallback. Tests (rustc dep-info as oracle): synthetic serde_core layout and the real registry serde_core (skips when absent).

## 2. fleet-store / signed-fleet 0/1
Not a double compile: the extra counted call was `rustc --print sysroot` (K2's portable-root probe), counted by the test's counting-rustc shim. Fix in the harness: ignore `--print` probes (fleet-store now also logs call arguments). Both pass.

## Tests (repo test-file; Explicit finite owned Bun flake needed retries)
offload 40/0 (1 file), paths (new, 4 tests) pass, fleet-store pass, signed-fleet pass, cluster pass, key sources pass.

## Open
Test file runs under `taskpolicy -b` stalled for 15+ min at load 60, so they were run without it (one test run at a time). Live proof of `serde_core` offload not re-run.
