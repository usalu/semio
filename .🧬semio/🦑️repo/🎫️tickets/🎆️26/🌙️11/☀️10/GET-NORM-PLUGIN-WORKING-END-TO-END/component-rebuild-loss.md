# Component rebuild "loss": completed units are kept; edits invalidated them

## Finding
The first norm build (16:33 to 18:29, killed by `timeout`) did not lose its units. The wasm target dir `.cache/b/r/m/u/l/p997c4d/t-ce555540` and build dir are stable across runs (same hash dirs, same lane), and the unit cache `.cache/b/r/m/u/e` keeps 1324 entries published incrementally by run 1.
The rerun recompiles only units whose inputs changed during run 1:
- `🗣️dsl/✨️derive/🔣️mutation-authority.json` (edited 18:26:16) is read by the derive macro of 508 dependent crates, so all 508 are invalid in Cargo's fingerprints and in the cache (dependency digest mismatch).
- `🌱️value/♻️retirement/🎮️controlled/🦀️.rs` (edited 18:25:18) invalidates that crate and its dependents (extern digests change: value, retirement, retained_clone, ...).
- Run 2 lookups: 336, all misses, none for crates with unchanged inputs; the other ~800 units were reused by Cargo fingerprints without invoking rustc.
The "0 units" start is an artefact: the progress counter counts only wrapper-compiled units (broker), never Cargo-fresh ones.

## Fix
The progress line now reports `N unit(s) compiled, M reused`; M counts `compiler-artifact` messages with `fresh:true` (`nativeBuildReusedUnitsV1`).
Test: `🧪️tests/🟦️.ts` in `component-pass` ("a resumed build counts the units Cargo reused ..."), passes.
No cache or store change is needed; a shared-worktree edit to a widely read input forces recompilation of its readers by design.
