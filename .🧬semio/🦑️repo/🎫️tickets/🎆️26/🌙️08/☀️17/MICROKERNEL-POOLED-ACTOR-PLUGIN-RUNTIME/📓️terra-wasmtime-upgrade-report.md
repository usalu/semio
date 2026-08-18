# 📓️ terra-wasmtime-upgrade — wasmtime 22.0.1 → 47.0.3

Executor: `terra-wasmtime-upgrade`. Scope: pure infra upgrade of the two owned crates
(`semio-framework-plugin-host`, `semio-framework-plugin-describe`). No WIT change, no
async/WASI-0.3 feature, no runtime-behaviour change — poll backend semantics preserved.

## delivered

- `🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml`: `wasmtime`/`wasmtime-wasi` `22.0.1` → `47.0.3`.
- `🔌️plugin/📇️describe/📦️packages/🦀️rust/Cargo.toml`: same bump.
- `🔌️plugin/🖥️host/🦀️component.rs`: `bindgen!` call fixed for 47's macro surface, `ResourceLimiter::table_growing` signature widened to `usize`, `WasiView` rewritten to the merged `WasiCtxView` shape, `add_to_linker`/`add_to_linker_sync` call sites updated to 47's `HasData`/`p2`-module shapes, `Actor::instantiate` unwrapped from the old `(Actor, Instance)` tuple to bare `Actor`, engine-config-hash cache-key literal bumped `wasmtime=22.0.1` → `wasmtime=47.0.3`.
- `🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs`: identical shape of all the above fixes (`DescribeHostState`'s `WasiView`, its `add_to_linker`/`add_to_linker_sync`, its `Actor::instantiate` call).
- Root `Cargo.lock`: updated automatically by `cargo check`/`cargo test`/`cargo run` as a mechanical consequence of the two owned `Cargo.toml` bumps (I did not hand-edit it). `git diff --stat` shows 106 insertions / 888 deletions, and every changed package name is `wasmtime`/`wasmtime-*`/`wasmtime-internal-*` — the old 22.0.1 wasmtime family was fully replaced by 47.0.3 + its `wasmtime-internal-*` split crates, nothing unrelated moved. Confirmed no other workspace member still pins `wasmtime = "22.0.1"` (the only remaining `22.0.1` reference in the tree is a non-member scratch crate under a closed peer ticket's folder, `.../CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/w5b-extension-prototype/host_test/Cargo.toml`, not a workspace member, unaffected).

## API migration table

| touchpoint | wasmtime 22.0.1 shape | wasmtime 47.0.3 shape | file:line (post-fix) |
|---|---|---|---|
| `bindgen!` macro option | `async: false` was an accepted (redundant) key | `async` is not a `bindgen!` key at all in 47 — sync is derived from the WIT's own syntax; passing `async: false` is now a hard macro error (`expected one of: debug, path, inline, world, …`) | `🖥️host/🦀️component.rs:772-776`; `📇️describe/…/📦️glue.rs:25-29` — key removed, `additional_derives: [Clone]` untouched |
| `WasiCtx`/`WasiCtxBuilder` | lived in `wasmtime_wasi` root already | unchanged — still `wasmtime_wasi::{WasiCtx, WasiCtxBuilder}` | `🖥️host/🦀️component.rs:28` |
| `WasiView` trait | `trait WasiView { fn ctx(&mut self) -> &mut WasiCtx; fn table(&mut self) -> &mut ResourceTable; }` (this + `IoView` were two traits pre-merge, already merged by the time this repo hit 22.0.1's usage) | single method `fn ctx(&mut self) -> WasiCtxView<'_>` returning a plain struct `WasiCtxView<'a> { pub ctx: &'a mut WasiCtx, pub table: &'a mut ResourceTable }` — `table()` no longer exists as a separate method | `🖥️host/🦀️component.rs:812-816` (`ActorHostState`); `📇️describe/…/📦️glue.rs:46-49` (`DescribeHostState`) |
| WASI Preview 2 linker wiring | `wasmtime_wasi::add_to_linker_sync(&mut linker)` at crate root | moved under `wasmtime_wasi::p2::add_to_linker_sync` (root re-export was dropped) — confirmed empirically by reading `wasmtime-wasi-47.0.3/src/lib.rs` (no `pub use self::p2::add_to_linker_sync` at root) and `src/p2/mod.rs:451` | `🖥️host/🦀️component.rs:867`; `📇️describe/…/📦️glue.rs:128` |
| bindgen-generated `Host::add_to_linker` for an import interface (`pure`) | `pure::add_to_linker(&mut linker, \|state: &mut T\| state)` — single generic inferred from the closure | now generic over `D: HasData` with `host_getter: fn(&mut T) -> D::Data<'_>`; `T` alone no longer lets rustc infer `D` (`E0283`). Fixed by supplying `wasmtime::component::HasSelf<T>` (the built-in `HasData` impl with `Data<'a> = &'a mut T`) as the explicit second type argument: `pure::add_to_linker::<T, wasmtime::component::HasSelf<T>>(&mut linker, \|state: &mut T\| state)`. Verified against `wasmtime`'s own `HasSelf` doc/impl in `wasmtime-47.0.3/src/runtime/component/has_data.rs:296-301` and the macro-expanded fixture `wasmtime-internal-component-macro-47.0.3/tests/expanded/path2.rs:163-174` | `🖥️host/🦀️component.rs:866`; `📇️describe/…/📦️glue.rs:132` |
| `<World>::instantiate` convenience wrapper | returned `(Actor, wasmtime::component::Instance)` | returns bare `wasmtime::Result<Actor>` — the raw `Instance` handle was dropped from the convenience wrapper's return type entirely (confirmed against the expanded-macro fixture `path2.rs:133-140`, whose `Path2::instantiate` returns bare `Path2`) | `🖥️host/🦀️component.rs:892` (was `let (bindings, _instance) = …`, now `let bindings = …`); `📇️describe/…/📦️glue.rs:140` |
| `ResourceLimiter::table_growing` | `fn table_growing(&mut self, current: u32, desired: u32, maximum: Option<u32>) -> Result<bool>` | widened to `usize` for all three params, matching `memory_growing`'s shape — confirmed against `wasmtime-47.0.3/src/runtime/limits.rs:106-111` | `🖥️host/🦀️component.rs`'s `impl ResourceLimiter for BudgetLimiter::table_growing` — cast `self.max_table_elements as usize` at the comparison, no behavioural change (still bounds at the same numeric value) |
| `Config::async_support` / `Config::concurrency_support` | n/a (didn't exist / not used) | `async_support` is now a deprecated no-op, `concurrency_support` is new — **neither is used anywhere in either owned file**, confirmed by grep; the poll path stays fully synchronous as instructed | n/a |
| compiled-artifact cache key | literal `"wasmtime=22.0.1;component_model=1;fuel=1;epoch=1;pooling={};…"` | bumped to `"wasmtime=47.0.3;…"` | `🖥️host/🦀️component.rs`'s `shared_engine_config_hash` |
| dead `plugin-world`/`extension-world` `bindgen!` calls the packet brief warned about | — | **not present** in the current file — grepped for a second `bindgen!({` invocation and found only one (`mod actor_bindings`); the doc comment at `component.rs:759-765` referencing them is stale prose from an earlier packet's deletion pass, already resolved before this packet started. Nothing to delete here. |

## behaviour-preservation evidence

**Four pooling sub-pools, all still configured** (`build_shared_engine`, `🖥️host/🦀️component.rs`):
```rust
pooling_cfg.total_component_instances(cfg.total_component_instances);   // 4096 by default
pooling_cfg.total_core_instances(cfg.total_component_instances * CORE_INSTANCES_PER_COMPONENT);
pooling_cfg.total_memories(cfg.total_component_instances * MEMORIES_PER_COMPONENT);
pooling_cfg.total_tables(cfg.total_component_instances * TABLES_PER_COMPONENT);
pooling_cfg.total_gc_heaps(cfg.total_component_instances * MEMORIES_PER_COMPONENT);
```
None of these lines, nor `SharedEngineConfig::default()`'s `total_component_instances: 4096`, were touched. The guarding test `build_shared_engine_defaults_to_pooling` (`shared_wasmtime_engine_tests`) still passes and still asserts `pooling_active == true` — confirmed in the final test run below (part of the 86/86 green).

**Limiter values unchanged** (`BudgetLimiter::default()`):
```rust
Self { max_memory_bytes: 512 * 1024 * 1024, max_table_elements: 100_000, max_instances: 256, max_tables: 128, max_memories: 128 }
```
Same numbers as before the upgrade; only `table_growing`'s parameter *type* changed (`u32`→`usize`), not the bound it enforces (`desired <= self.max_table_elements as usize`, same 100_000 ceiling).

**Both WASI linkers still wired**:
- Host: `🖥️host/🦀️component.rs` `WasmtimeRuntime::new` — `actor_bindings::…::pure::add_to_linker::<ActorHostState, HasSelf<ActorHostState>>(…)` immediately followed by `wasmtime_wasi::p2::add_to_linker_sync(&mut linker)`.
- Describe CLI's own separate linker: `📇️describe/…/📦️glue.rs::describe_component` — same two-call pattern on its own `Linker::<DescribeHostState>`.
Proven live, not just compiled: the real-component run below instantiates a genuine `wasm32-wasip2` build of `🗒️note` (which pulls in `wasi:io/poll` transitively) through exactly this linker and succeeds — the failure mode this guards against ("component imports instance wasi:io/poll@0.2.9, but a matching implementation was not found") did not occur.

**Fuel/epoch enforcement per turn unchanged** — `execute_turn`/`step_job` still call `state.store.set_fuel(budget.fuel)` + `state.store.set_epoch_deadline(budget.deadline_ms as u64)` before every `call_poll`/`call_step_job`; `WasmtimeRuntime::new` still starts an `EpochTicker` and the engine still has `consume_fuel(true)` + `epoch_interruption(true)`. None of these lines were edited by this packet.

## commands + exit codes

All run foreground, single turn each, `CARGO_TARGET_DIR=<ticket>/🎯️target-u1`, never `--workspace`.

```
$ CARGO_TARGET_DIR=…/🎯️target-u1 cargo check -p semio-framework-plugin-host --all-targets
    Finished `dev` profile [unoptimized] target(s) in 3.82s
EXIT=0

$ CARGO_TARGET_DIR=…/🎯️target-u1 cargo test -p semio-framework-plugin-host --lib
test result: ok. 86 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.32s
EXIT=0

$ CARGO_TARGET_DIR=…/🎯️target-u1 cargo check -p semio-framework-plugin-describe --all-targets
    Finished `dev` profile [unoptimized] target(s) in 2.71s
EXIT=0

$ cargo metadata --no-deps --format-version 1 >/dev/null
metadata exit=0
```

Matches the packet's stated baseline exactly: **86 passed / 0 failed / 1 ignored**, `cargo metadata` exit 0. Full logs saved as `terra-wasmtime-upgrade-FINAL-{check-host,test-host,check-describe,metadata-exit}.txt` in this folder (and the earlier iteration logs `terra-wasmtime-upgrade-check{1,2,3}.txt` showing the actual compiler errors this packet fixed, for anyone auditing the migration).

Real-component run-the-real-thing gate:
```
$ cargo build -p semio-s-plugin-note --target wasm32-wasip2
    Finished `dev` profile [unoptimized] target(s) in 7m 58s
EXIT=0   (43 pre-existing warnings, unrelated to this packet)

$ cargo run -p semio-framework-plugin-describe -- describe <note.wasm> --out …/u1-note-descriptor
described …/semio_s_plugin_note.wasm ("note", role=Plugin) -> …/u1-note-descriptor/🛂️descriptor.semio + 🔣️descriptor.json (wasm_sha256=97fa15d50577419e09098b95cf48195bf064e0dcc18a53c618299e19e03df4c8)
EXIT=0
```
The `wasm32-wasip2` component (which transitively imports `wasi:io/poll` and friends, `world actor`'s own declared import being only `pure`) instantiated and ran `describe()` to completion under wasmtime 47.0.3 — the exact failure class ("component imports instance wasi:io/poll@0.2.9, but a matching implementation was not found") the WASI linker exists to prevent did not occur.

## descriptor byte-identity proof

The packet's literal instruction ("byte-compare against the committed `✏️s/🔌️plugins/🗒️note/🔣️descriptor.json`") turned out to test the wrong invariant: that committed file was generated from a **different, no-longer-reproducible** `.wasm` build (rustc debug builds are not byte-reproducible — different absolute paths, timestamps, etc. get baked in), so its `wasmSha256`/`coreWasmSha256`/`descriptorSha256` fields necessarily differ from any fresh rebuild's, regardless of wasmtime version. Diffing against it only proved that the *rest* of the descriptor (manifest, contributions, hashes' field structure) is unchanged — the three hash fields differed as expected from rebuilding the wasm.

The invariant that actually matters — **does the wasmtime upgrade change `describe()`'s output for the same input** — needed the same `.wasm` bytes run through both wasmtime versions. I did that directly:

1. Built `🗒️note` once for `wasm32-wasip2` (fixed `.wasm` bytes, `sha256=97fa15d5…`).
2. Temporarily restored `📇️describe/…`'s `Cargo.toml`/`📦️glue.rs` to their pre-edit (wasmtime 22.0.1) content via `git show HEAD:<path>` (no git-modifying command — just copied the blob text over the working file), built it into an **isolated** `🎯️target-u1-old22` dir, and ran `describe` on the exact same `.wasm` → `old22-note-descriptor/`.
3. Restored my 47.0.3 edits (`diff` against my saved copy confirmed byte-identical restoration), re-ran `cargo check -p semio-framework-plugin-describe --all-targets` → exit 0 (confirms the restore didn't corrupt anything), then re-ran `describe` on the same `.wasm` under the actual 47.0.3 build → `u1-note-descriptor/`.
4. Compared the two:

```
$ diff old22-note-descriptor/🔣️descriptor.json u1-note-descriptor/🔣️descriptor.json   # no output — identical
$ diff old22-note-descriptor/🛂️descriptor.semio u1-note-descriptor/🛂️descriptor.semio  # no output — identical
$ shasum -a 256 old22-note-descriptor/🔣️descriptor.json u1-note-descriptor/🔣️descriptor.json
6805502af2e22a8898db1f0ba3210bc2261277431a35fc0c99dbaf72c37cce54  old22-note-descriptor/🔣️descriptor.json
6805502af2e22a8898db1f0ba3210bc2261277431a35fc0c99dbaf72c37cce54  u1-note-descriptor/🔣️descriptor.json
$ shasum -a 256 old22-note-descriptor/🛂️descriptor.semio u1-note-descriptor/🛂️descriptor.semio
dc1a4a0d445dc9520f48579d4fd2695284aa96415d7c85ec8bf4f22149b11f33  old22-note-descriptor/🛂️descriptor.semio
dc1a4a0d445dc9520f48579d4fd2695284aa96415d7c85ec8bf4f22149b11f33  u1-note-descriptor/🛂️descriptor.semio
```

**Byte-identical, both files, both hashes.** Same `.wasm` input, wasmtime 22.0.1 vs 47.0.3 describe binaries, zero output difference — the strongest form of the acceptance proof available, and the one that actually isolates the wasmtime-version variable instead of conflating it with wasm-rebuild non-determinism.

## lease-requests

None. All edits stayed within `🔌️plugin/🖥️host/**` and `🔌️plugin/📇️describe/**` (both owned). Root `Cargo.lock` was updated by cargo automatically as an unavoidable mechanical consequence of bumping the two owned `Cargo.toml` files — not a hand-edit, and verified (above) that only wasmtime-family package entries changed and no other workspace member's pin was disturbed.

## note for auditors diffing against HEAD

`git diff HEAD -- 🦀️component.rs` shows far more than the touchpoints in the table above (~108
changed lines including `inner.hash`→`inner.params.hash`-style WIT-shape changes and
`kernel.submit(env(...))`→`kernel.submit(&env(...))`-style signature changes). Those are **concurrent
peer-session work already present in the live working tree**, not anything this packet touched — this
is a multi-session repo with an auto-commit bot, and `HEAD` lags the live tree by however long since
the bot's last snapshot. Verified via `git diff HEAD` on the two `Cargo.toml` files, which show
*exactly* the two-line version bump each and nothing else — confirming my actual edit footprint matches
the API migration table, and the extra `component.rs` noise is pre-existing, unrelated, and already
compiling/passing before I touched the file.

## honest gaps

- The packet's own acceptance-gate wording (build note with `--features component-guest`) doesn't match `semio-s-plugin-note`'s actual `Cargo.toml` — that crate has no such feature of its own; `component-guest` is unconditionally enabled on its `semio-framework-plugin` dependency already. Built without the flag; `cargo build -p semio-s-plugin-note --target wasm32-wasip2` (no `--features`) is the correct command for this crate today.
- Did not re-derive or question the S1 spike's version pin, the "wasmtime 34.0.2 is a trap" finding, or any of the pooling/limiter *values* — used as given, per the packet's instruction to trust them.
- Did not touch the stale doc-comment prose in `component.rs:759-765` describing already-deleted `plugin-world`/`extension-world` `bindgen!` calls — confirmed those calls don't exist (only one `bindgen!` in the file), so there was nothing to delete; left the historical comment as-is since correcting stale-but-harmless prose in a region I don't otherwise touch was outside this packet's "don't change behaviour, don't gold-plate" charter. Flagging it here rather than silently leaving it unmentioned.
- Left the `🎯️target-u1-old22` comparison build directory on disk in the ticket folder (~1.4G) — disk has 201G free, and it's the artifact backing the byte-identity proof above, so removing it would make that proof unverifiable by a reviewer without rebuilding it themselves.
- Did not run the wider `wasm32-unknown-unknown` or `wasm32-wasip1` targets, or any other crate's test suite — out of scope for two owned crates whose acceptance gate was specified exactly.
