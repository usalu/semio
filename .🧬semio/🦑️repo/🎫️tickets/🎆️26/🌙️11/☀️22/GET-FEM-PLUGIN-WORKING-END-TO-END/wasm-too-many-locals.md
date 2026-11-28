# Wasm "too many locals" in the FEM component transpile (2026-11-27)

## Symptom
`native-fem-materialize.ts` on the 02:18 `prototype` wasm fails with `locals exceed maximum (at offset 0xc5ede98)`: wasmparser allows 50000 locals per function.

## Cause
- `wasm_locals_scan.py wasm <wasm> 0xc5ede98` finds the function around that offset in the final core module: 59197 locals, 1274443 body bytes.
- `wasm_locals_scan.py rlib 20000 <rlibs>` over every rlib in `C:\t\wdb` finds exactly one match (59197 locals, 1274443 bytes, same numbers): `semio_framework_replication_history::retained_fold::fold_history_for_controlled::{closure#0}`, the poll function of the `async fn`, instantiated inside crate `semio-os-history-schema`.
- Why there: rustc treats closures and coroutine bodies as always cross-crate inlinable, so an `async fn` body is codegenned in every crate that polls its future, at that crate's profile. `semio-os-history-schema` is not in the opt-level 2 list of `native-fem-profile.toml` and gets opt-level 1, where the huge fold coroutine produces 59197 locals. The function was also generic (`impl HistoryTransitionEvents + ?Sized`), which forced instantiation there as well. Source file unchanged since 11-26 04:57; the earlier good build had the instance in an optimized crate.
- The same would hit the official `repo` prototype profile, so the fix is in source, not in the profile.

## Fix
`🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs`:
- `fold_history_for_controlled` is now a plain `fn` returning `Pin<Box<dyn Future<Output = Result<HistoryFold, ProtocolError>> + Send + 'a>>`. It boxes the private `async fn fold_history_for_controlled_body`. The vtable pins the coroutine's code generation to `semio-framework-replication-history` (opt-level 2).
- The `transitions` parameter is `&dyn HistoryTransitionEvents` instead of a generic `impl`, so there is one instance, not one per event container.

`🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🦀️.rs`: `fold_history_for` wraps its argument in the private sized adapter `HistoryTransitionEventsRef` so slices (unsized) coerce to `&dyn`. Callers of `fold_history_for_controlled` (`semio-os-spr` fold, companion unit tests) pass `&Vec<MutationEnvelope>` and need no change. There is no 2D or 3D twin: the code is framework-level, shared by both.

Two intermediate attempts did not work and are not in the tree: `&dyn` alone (closure still instantiated downstream), and `#[inline(never)]` on the `async fn` (no effect on the closure).

## Verification
Rebuilt only `-p semio-os-history-schema` (and its deps) for `wasm32-wasip2` with `--profile prototype` and `native-fem-profile.toml`, in `C:\t\tl`, `-j 3`:
- Before: schema rlib 17 MB, max function 59197 locals.
- After: schema rlib 10 MB, max function 8378 locals. The fold body lives in the history crate: `fold_history_for_controlled_body::{closure#0}` has 257 locals, 119 KB.
- `wasm-locals-guard.sh 30000` over all 140 rlibs of that dependency subset: `ok: no function >= 30000 locals`.

The native companion unit tests (`HistoryFoldJob` byte and work grants) were not recompiled here; the change is signature-compatible for them.

## Guard law
`wasm-locals-guard.sh [threshold] rlib...` (default 40000) exits 1 and prints offenders when any function reaches the threshold; it uses `wasm_locals_scan.py` (`rlib` mode over ar archives, `wasm` mode for a final component plus offset). Run it over `C:\t\wdb\wasm32-wasip2\prototype\build\**\out\*.rlib` before transpile.

## Files
- Source: the two `🦀️.rs` files above.
- Tools in this folder: `wasm_locals_scan.py`, `wasm-locals-guard.sh`, `tl-rlib-run.sh` (detached launcher for the single-crate build).

## Next
Coordinator: rebuild and publish the full wasm; the transpile should pass. Expect the fold function to be about 257 locals.
