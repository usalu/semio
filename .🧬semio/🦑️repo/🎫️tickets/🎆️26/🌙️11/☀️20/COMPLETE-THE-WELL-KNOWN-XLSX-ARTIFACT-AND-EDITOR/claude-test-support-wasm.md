# Test-support wasm compile break and dev-only split

## 1. Original break (crate `semio-os-store-test-support`)
- `RetainedCloneGrant` capacity/release bytes in `with_original_identity`: `1 << 32` -> `1 << 31` (fits wasm32 `usize`, above `maximum_copy_bytes: 1 << 30`).
- `with_original_identity<'s, R>`: the closure takes `(identity, PhantomData<&'a &'s ()>)` so `'s: 'a` is implied and borrowed captures work. All 15 callers use `|identity, _|`.
- Verified in practice by the coordinator (every stdio and stdio-office wasm build compiled the crate).

## 2. Dev-only split (done)
- New production crate `semio-os-store-schema-exports` (`🏪️store/🗂️schema-exports`): `STORE_SCHEMA_EXPORTS`, `register_store_schema_exports`. Kernel depends on it; the store root re-exports it. `lifted_store` removed from test-support.
- New production crate `semio-os-store-mutation-report` (`🏪️store/📑️mutation-report`): `mutation_report_json`, previously called from 44 production `🧬️mutations` files through the kernel's `test_support` re-export. All 42 owning crates depend on it normally; the other agent's `mutation-report` feature and optional dependency in the two pdf crates is removed.
- Removed the `test_support` re-export from the kernel root and the store root. 164+10 test crates now take `semio-os-store-test-support` as a dev-dependency and import `semio_os_store_test_support` directly; kernel has it as dev-dependency. The pdf presence crate no longer has it as a normal dependency.
- Both new crates are workspace members and listed in crate-boundaries (`🔣️.json`, `🧬️cuts.json`) and `📋️build-owners.json`.
- Guard test: `kernel_production_closure_never_links_test_support` in test-support (`🧪️tests/🔬️unit`), a `toml`-parsed walk of the kernel's production dependency closure (not yet executed, only compiled).

## 3. Verification (guarded, MIN_DISK_GIB=20)
- Passed: kernel native and wasm32-wasip2 `cargo check`; test-support `--tests`; mutation-report; pdf presence/transient crates native.
- Not verified: pdf and dag/wfc artifact workspaces for wasm and native (blocked by item 4), the guard test run, the remaining ~40 plugin crates and the ~170 dev-dependency consumers (heuristic crate mapping, not compiled).

## 4. Foreign breakage met
- `semio-framework-job-context-step`: `unresolved import crate::OriginalCallerReturnPlace` (native source macro). Another agent had moved `original_return_place` into the facade, but the file uses step-crate `crate::` paths. Fix: mounted it again in `⏱️context/🦀️.rs` and removed the mount from `🪟️facade/🦀️.rs`. The step crate now compiles.
- `semio-framework-job-context` (facade crate): 162 errors in the in-progress facade split (`🎟️source/🏭️admission` and siblings lack their `use` items). Not fixed; the other agent is still editing it.

## 5. Follow-up
- Compile the plugin workspaces once the facade split settles; fix any consumer that lacks the dev-dependency.
- Add a language-agnostic case for the guard (a manifest-closure vector) beside the Rust test.
