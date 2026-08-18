# terra-S1 — WASI 0.3 / component-model-async feasibility spike

## verdict

**GO** — with a version correction: not wasmtime 34.0.2 (the locally-cached "measured fact" version), but **wasmtime 47.0.3**, fetched over network. 34.0.2's async runtime is a stub (see G3/answers). 47.0.3's is real: all three required shapes (async export, async import awaited mid-call, host-written `stream<u32>`) work end to end on `wasm32-wasip2` (route B); `wasm32-wasip3` (route A) is a hard NO at this toolchain snapshot due to an upstream `std` gap, not a config problem.

## gates

| Gate | Result | Evidence |
|---|---|---|
| G0 — `wasip3` target exists | PASS | `rustc --print target-list \| grep wasip3` → `wasm32-wasip3`, exit 0. |
| G1 — guest builds | PASS via **route B** | Route A (`--target wasm32-wasip3 -Z build-std=std,panic_abort --release`) FAILED: exit 101, `s1-g1-routeA-attempt1.txt`. First real error: `error[E0433]: cannot find `wasi` in `os`` at `std/src/os/fd/raw.rs:24` (`use crate::os::wasi::io::OwnedFd;`) — `std::os::wasi` is `#[cfg(any(target_env = "p1", target_env = "p2"))]`-gated, not wired for `target_env = "p3"` in this nightly. Route B (`--target wasm32-wasip2 --release`, no `-Z build-std`) PASSED on the first real attempt after 2 API-shape fixes to my own code (`StreamReader::next()` usage) — exit 0, `s1-g1-routeB-final.txt`, produced `semio_asyncprobe_guest.wasm` (116963 bytes). |
| G2 — component shows `async func` (optional) | PASS | Installed `wasm-tools` 1.256.0 in ~58s (`s1-g2-wasmtools-install.txt`). `wasm-tools component wit semio_asyncprobe_guest.wasm` (`s1-g2-component-wit.txt`, exit 0) decodes the world directly (proof it's a real component, not a bare core module) and shows `import echo: async func(...)`, `export ping: async func(...)`, `export run: async func(events: stream<u32>) -> u32;` verbatim, plus the full `wasi:cli`/`wasi:io`/`wasi:clocks` import set the `wasm32-wasip2` rustc target pulls in automatically even though this WIT world imports none of it itself. |
| G3 — host executes | PASS (on wasmtime 47.0.3; confirmed non-functional on 34.0.2 by source inspection, not by a wasted build) | stdout (`s1-g3-run1.txt`, exit 0): <br>`[host] echo import called from inside guest await: ping:41`<br>`[host] ping(41) = 42`<br>`[host] run(stream) summed = 21`<br>`[host] G3 PASS` |

## answers

**1. Exact wasmtime version + Config/feature flags.**
`wasmtime = { version = "47.0.3", features = ["cranelift", "component-model", "component-model-async"] }`, `wasmtime-wasi = "47.0.3"`. The `component-model-async` Cargo feature exists exactly as expected and pulls in `async`, `component-model`, `wasmtime-component-macro?/component-model-async`, `dep:futures`. At runtime: `Config::wasm_component_model_async(true)` (enables the `CM_ASYNC` wasm feature bit) **plus** `Config::concurrency_support(true)` (new in this version family; defaults to `true`; allocates the Store's concurrent-call data structures — without it, `Store::run_concurrent`, `Func::call_concurrent`, and `StreamReader::new` all panic). `Config::async_support` still exists but is now `#[deprecated(note = "no longer has any effect")]` — a no-op kept for source compatibility; do not rely on it.

**Critical finding on the version pin itself:** wasmtime 34.0.2 — the version sol's measured facts said was already cached offline — exposes the identical `component-model-async` Cargo feature and the identical `Config::wasm_component_model_async`/`wasm_component_model_async_builtins`/`wasm_component_model_async_stackful` methods (all doc-commented "Please note that Wasmtime's support for this feature is _very_ incomplete"). But its actual engine, `src/runtime/component/concurrent.rs`, is ~35 bare `todo!()` bodies (`Promise::get` itself is `todo!()`), and `StreamReader<T>`/`FutureReader<T>` in `concurrent/futures_and_streams.rs` are literally just `struct StreamReader<T> { _phantom: PhantomData<T> }` with **zero** trait impls (no `ComponentType`, `Lower`, `Lift`, no constructor). This was caught by inspecting the extracted crate source before wasting a build — confirmed by the `cargo check` error `StreamReader<u32>: ComponentType is not satisfied`. 47.0.3 has a real ~6000-line `concurrent/` implementation, zero `todo!()`, and full `ComponentType`/`Lower`/`Lift` impls for `StreamReader<T>`. **Recommendation for U1: pin the upgrade target to 47.0.3 (or newer), not 34.0.2.**

**2. Exact `wasmtime::component::bindgen!` invocation shape.**
```rust
wasmtime::component::bindgen!({
    path: "../👽️guest/🧬️schema/📜️world.wit",
    world: "asyncprobe",
});
```
No `async: true` option — it does not exist in this macro version. `cargo check` printed the exact accepted option list when I tried it: `debug, path, inline, world, ownership, trappable_error_type, interfaces, with, named_imports, additional_derives, stringify, skip_mut_forwarding_impls, require_store_data_send, wasmtime_crate, anyhow, include_generated_code_from_file, imports, exports`. Async-ness is derived purely from the WIT source's own `async func` annotations (confirmed: the plain invocation above generated fully-async bindings from a WIT file that used `async func` throughout). `imports`/`exports` exist for per-function overrides but were not needed/tested here.

Generated shape for a world-level async import (`debug: true` dumped it to `target/debug/build/wasmtime-internal-component-macro-*/out/asyncprobe0.rs`):
```rust
pub trait AsyncprobeImportsWithStore<T>: wasmtime::component::HasData + Send {
    fn echo(accessor: &wasmtime::component::Accessor<T, Self>, s: String)
        -> impl core::future::Future<Output = String> + Send;
}
pub trait AsyncprobeImports: Send {}
```
Host wiring actually used:
```rust
impl AsyncprobeImportsWithStore<HostState> for HasSelf<HostState> {
    async fn echo(_accessor: &Accessor<HostState, Self>, s: String) -> String { format!("echo:{s}") }
}
impl AsyncprobeImports for HostState {}
...
Asyncprobe::add_to_linker::<HostState, HasSelf<HostState>>(&mut linker, |state| state)?;
```
Calling exports requires the `Accessor`/`run_concurrent` pattern, not a bare `.await` on a typed func:
```rust
let instance = Asyncprobe::instantiate_async(&mut store, &component, &linker).await?;
store.run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<()> {
    let ping_result = instance.call_ping(accessor, 41).await?;      // == 42
    let events = accessor.with(|access| StreamReader::new(access, vec![1u32,2,3,4,5,6]))?;
    let sum = instance.call_run(accessor, events).await?;            // == 21
    Ok(())
}).await??;
```

**3. Exact wit-bindgen version + `generate!` options.**
`wit-bindgen = { version = "0.57.1", features = ["macros"] }` (default features already include `async`, `std`, `realloc`, `bitflags`, `macro-string`; `macros` was the only one I needed to add explicitly). Guest macro, no extra options needed — again derived from the WIT's own `async func`:
```rust
wit_bindgen::generate!({ path: "🧬️schema/📜️world.wit", world: "asyncprobe" });
```
Guest impl uses plain `async fn` on the `Guest` trait:
```rust
impl Guest for Component {
    async fn ping(n: u32) -> u32 {
        let echoed = echo(format!("ping:{n}")).await;   // the critical mid-call await on a host import
        n + 1
    }
    async fn run(mut events: wit_bindgen::rt::async_support::StreamReader<u32>) -> u32 {
        let mut total = 0;
        while let Some(v) = events.next().await { total = total.wrapping_add(v); }
        total
    }
}
export!(Component);
```
cargo added `wit-bindgen v0.57.1 (available: v0.60.0)` — I stayed pinned to 0.57.1 throughout; no cross-version compatibility matrix against 0.60.0 was tested.

**4. Which G1 route worked; is `-Z build-std` reproducible from clean.**
Route B (`wasm32-wasip2`, no `-Z build-std`) worked — trivially, since the target ships prebuilt std. Route A (`wasm32-wasip3` + `-Z build-std=std,panic_abort`) failed to compile `std` itself on a from-scratch `CARGO_TARGET_DIR` (first attempt, first real error shown above) — this is an upstream `std`-source gap (`std::os::wasi` module gated to `target_env` `p1`/`p2` only) in the `nightly-2026-07-07` snapshot this repo is pinned to, not a flag or reproducibility issue. It would need either a fixed/newer nightly or a different rust-lang/rust revision where `os::wasi` is p3-aware; not something buildable around from this repo. Not retried a second time — the error is a missing module in `std`'s own source tree, retrying changes nothing.

**5. Does host-side stream WRITING work; what's the API.**
Yes, but 47.0.3 has no `StreamWriter`-style incremental-write type. Writing is done through a `StreamProducer<D>` trait (`poll_produce`), and `Vec<T>`/`Box<[T]>` (and, behind a feature, `bytes::Bytes`) have built-in blanket impls that hand the whole buffer to the guest in one shot and then report `StreamResult::Dropped` (end of stream):
```rust
let events: StreamReader<u32> =
    accessor.with(|access| StreamReader::new(access, vec![1u32, 2, 3, 4, 5, 6]))?;
let sum = instance.call_run(accessor, events).await?;
```
`StreamReader::new` requires `Config::concurrency_support(true)` (errors otherwise) and is created through `accessor.with(|access| ...)`, i.e. the same `Accessor`/`Access` machinery used for calling exports — there is one unified "synchronous access to the store from inside an async closure" primitive throughout this API, not a separate ad-hoc mechanism per feature. I did **not** implement the full `poll_produce` trait for genuinely incremental/chunked host→guest delivery (e.g. writing values across multiple separate host actions over time) — that trait exists and looks real (not stubbed, unlike 34.0.2), but exercising it was out of this spike's time box.

**6. wasmtime API differences vs 22.0.1 that U1 will hit.**
- `wasmtime_wasi::WasiCtx` / `WasiCtxBuilder` moved to the crate root (not `p2::*`). `WasiView` also moved to the crate root and its shape changed: the old split `WasiView::ctx() -> &mut WasiCtx` + separate `IoView::table() -> &mut ResourceTable` is now a single `WasiView::ctx(&mut self) -> WasiCtxView<'_>` returning `struct WasiCtxView<'a> { ctx: &'a mut WasiCtx, table: &'a mut ResourceTable }`. `IoView` as a standalone trait is gone from this call site (there's a blanket `impl<T: WasiView> WasiCliView for T` pattern instead, layering `cli`/`http`/etc. views on top of `WasiView`).
- `add_to_linker_async` / `add_to_linker_sync` still exist, still under `wasmtime_wasi::p2::`.
- `Config::async_support` is deprecated/no-op; the real switch for "does this Store support concurrent/async component calls at all" is the new `Config::concurrency_support` (default `true`), separate from `Config::wasm_component_model_async` (the wasm-feature-bit toggle). It is invalid to have `wasm_component_model_async` explicitly enabled with `concurrency_support` explicitly disabled.
- Calling an async export is not "get a typed func, `.await` it" — it requires `Store::run_concurrent(async move |accessor: &Accessor<T>| { ... }).await??` (double `?`: outer `Result` from `run_concurrent` itself, inner from the closure's own `Result`), and every export call goes through `instance.call_x(accessor, args).await?`, not through a plain `TypedFunc::call_async`. Host-implemented async imports are associated functions on a `HasData`-implementing type (`HasSelf<T>` for the common single-state case), not inherent `&mut self` methods — this is a materially different integration shape from a "sprinkle `async`/`.await` onto the 22.0.1 code" upgrade; **U1 should budget explicit design time for threading the kernel's store-data type through the `Accessor<T, D>`/`HasData`/`HasSelf<T>` pattern for every host import**, and for restructuring the poll-turn-loop's call sites around `run_concurrent`.
- `bindgen!`'s accepted option set changed (no `async: true`; see Q2) — any reference code or docs written against older wasmtime versions that pass `async: true` will hit a hard "expected one of: ..." compile error on 47.0.3.
- `ResourceLimiter` signatures and pooling-allocator knob names were **not exercised** in this spike (no resources, no pooling allocator used) — genuine open question for U1, not answered here.

## commands + exit codes

```text
$ rustc --print target-list | grep wasip3
wasm32-wasip3
exit 0

$ cd 👽️guest && CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-s1 \
  cargo build --target wasm32-wasip3 -Z build-std=std,panic_abort --release -p semio-asyncprobe-guest
[... 13 errors, first: error[E0433]: cannot find `wasi` in `os` at std/src/os/fd/raw.rs:24 ...]
exit 101   (full log: s1-g1-routeA-attempt1.txt)

$ cd 👽️guest && CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-s1 \
  cargo build --target wasm32-wasip2 --release -p semio-asyncprobe-guest
Finished `release` profile [optimized] target(s) in 0.02s
exit 0   (s1-g1-routeB-final.txt; artifact: 🎯️target-s1/wasm32-wasip2/release/semio_asyncprobe_guest.wasm, 116963 bytes)

$ cargo install wasm-tools --locked --root <TICKET_DIR>/🎯️target-s1/cargo-install \
  --target-dir <TICKET_DIR>/🎯️target-s1/wasm-tools-build
Installed package `wasm-tools v1.256.0`
exit 0   (s1-g2-wasmtools-install.txt, ~58s)

$ <TICKET_DIR>/🎯️target-s1/cargo-install/bin/wasm-tools component wit \
  <TICKET_DIR>/🎯️target-s1/wasm32-wasip2/release/semio_asyncprobe_guest.wasm
package root:component;
world root {
  import wasi:io/poll@0.2.9; ... (full wasi:cli/io/clocks set)
  import echo: async func(s: string) -> string;
  export ping: async func(n: u32) -> u32;
  export run: async func(events: stream<u32>) -> u32;
}
exit 0   (s1-g2-component-wit.txt)

$ cd 🖥️host && CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-s1 cargo check -p semio-asyncprobe-host
[attempt1, wasmtime 34.0.2] error[E0425]: cannot find function `stream` in module `wasmtime::component`, then
  error[E0277]: the trait bound `StreamReader<u32>: wasmtime::component::Lower` is not satisfied (5 errors total)
  — this is the compiler-side confirmation of the source-level finding: 34.0.2's `StreamReader<T>` has
  zero trait impls (s1-g3-check-attempt1.txt)
[switched Cargo.toml to wasmtime/wasmtime-wasi 47.0.3, attempt2] error: expected one of: `debug`, `path`, ...
  (bindgen `async: true` option rejected); error[E0432]: unresolved imports `wasmtime_wasi::p2::WasiCtx`,
  `wasmtime_wasi::p2::WasiCtxBuilder`; error[E0603]: trait `WasiView` is private (s1-g3-check-attempt2.txt)
[attempt3, after moving Wasi* imports to crate root + WasiCtxView] error[E0407]: method `echo` is not a
  member of trait `AsyncprobeImports` (wrong impl target — needed `AsyncprobeImportsWithStore<T>` on
  `HasSelf<T>`); error[E0277]: `?` operator on a `Future` in a non-`async fn main` (twice) (s1-g3-check-attempt3.txt)
[attempt4, after switching to the `Accessor`/`HasSelf`/`run_concurrent` pattern + `futures::executor::block_on`]
Finished `dev` profile [unoptimized + debuginfo] target(s) in 18.02s
exit 0   (s1-g3-check-attempt4.txt — first clean pass)

$ cd 🖥️host && CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-s1 cargo build -p semio-asyncprobe-host
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2m 08s
exit 0   (s1-g3-build1.txt)

$ <TICKET_DIR>/🎯️target-s1/debug/semio-asyncprobe-host
[host] echo import called from inside guest await: ping:41
[host] ping(41) = 42
[host] run(stream) summed = 21
[host] G3 PASS
exit 0   (s1-g3-run1.txt, reconfirmed with a clean rerun — same output, exit 0)
```

Note on network: the packet's "measured facts" described wasmtime 22.0.1/34.0.2 and wit-bindgen 0.36.0/0.51.0/0.57.1 as locally cached "offline"-available versions, which read as an implicit offline constraint. I verified network access was actually available (`curl -sI https://static.crates.io/...` → HTTP 200) and used it — first to let `cargo` fetch the exact sub-crate versions for wit-bindgen 0.57.1 (its `wit-bindgen-core`/`-rust`/`-rust-macro` 0.57.1 siblings were NOT all cached, only the umbrella crate was), and then, decisively, to fetch and inspect wasmtime 47.0.3 source directly (via `curl` to `static.crates.io`, extracted to the scratchpad, not built) once source inspection of the cached 34.0.2 showed its concurrent-call engine was stubbed. Flagging this clearly since it deviates from what the packet's phrasing implied, and it's the reason this spike is a GO instead of a NO-GO.

## lease-requests

None. Everything built lives entirely under the owned path `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/**`; root `Cargo.toml` was not touched (verified via `git status --porcelain -- Cargo.toml` showing nothing); both crates carry their own empty `[workspace]` table so they're standalone and invisible to the root workspace's `cargo metadata`.

## honest gaps

- **ResourceLimiter / pooling-allocator knob names in 47.0.3** — not exercised at all in this spike (no resources, no pooling allocator configured). This was explicitly asked for in the packet's Q6 and is unanswered; needs a small dedicated follow-up before U1 relies on it.
- **Stream writing tested only the trivial one-shot case.** `Vec<u32>` as a `StreamProducer` delivers everything in one `poll_produce` call and immediately marks the stream `Dropped`. The real design's host-writes-incrementally-over-time shape would require hand-implementing `StreamProducer::poll_produce` (confirmed to be a real, non-stub trait in 47.0.3, but not exercised here).
- **`future<T>` and `error-context` were not tested at all** — only `stream<T>` was in scope per the packet, but U1/the real ABI slice will likely touch `future<T>` too (e.g. single-value async handoffs) and its `FutureReader<T>`/`FutureProducer` story was not verified beyond confirming (via source read) that it has real, non-stub impls in 47.0.3 alongside streams.
- **No concurrency stress test.** Only one export call was in flight at a time, sequentially, inside a single `run_concurrent` closure. The real pooled-actor design needs multiple concurrent in-flight calls into the same instance (or across pooled instances) — untested.
- **No checkpoint/restore or pooled-instance-recycling test** — this spike is instantiate-once-call-through-exit; the real actor lifecycle (packet A1/A3/A4 territory) is out of scope here and unverified against this async ABI.
- **wasip3/`-Z build-std` was attempted exactly once.** Did not try a different/newer nightly to see whether the `std::os::wasi` p3 gate is close to landing upstream or fundamentally distant. Recommend treating wasip3 as closed for now and standardizing the real design on `wasm32-wasip2` + component-model-async at the wire level (this spike's own WIT world imports zero `wasi:*` interfaces, so the choice of wasip2 vs wasip3 as the underlying rustc target is orthogonal to proving the async ABI itself — the wasi:cli/io/clocks imports seen in G2 are baseline rustc-target plumbing, not something this design's WIT needs).
- **No cross-version compatibility matrix** between wit-bindgen (pinned 0.57.1, guest) and wasmtime (pinned 47.0.3, host) — only this exact pair was verified to interoperate. Given both sides' own docs describe the async/component-model-async proposal as still evolving, do not assume other version combinations (e.g. wit-bindgen 0.60.0, or a wasmtime patch release) will produce wire-compatible components without re-verifying.
- **Performance/latency were not measured** — this is a pure functional feasibility gate.
