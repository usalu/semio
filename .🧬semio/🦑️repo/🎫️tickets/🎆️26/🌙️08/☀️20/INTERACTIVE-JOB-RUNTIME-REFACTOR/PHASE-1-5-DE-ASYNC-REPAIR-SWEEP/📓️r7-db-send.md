# R7 — `db` Crate `Send`-Safety Repair

## Scope

Packet R7 of Phase 1.5. Boundary: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/**`, and, once `db`
compiles, verification of `semio-hub` (`🌎️hub/**`, read-only — no file there was edited). `./compose`/
`semio-compose-rs` out of scope and untouched. Two sibling packets ran concurrently on the webgpu/
d3d12 render backends and were not touched here.

## The bug, confirmed at each of the three reported sites

Edition-2021 temporary-scope rules keep a `MutexGuard` produced inside an `if`/`if let` condition (or
bound to a `let` and used across the rest of its enclosing block) alive past the point a real `.await`
inside that scope suspends the state machine. `std::sync::MutexGuard` is not `Send`, so the generated
future captures a non-`Send` value across a suspension point, making the whole `async fn`'s future
non-`Send` — invisible until something needs to hand that future to a multi-threaded scheduler (axum's
`on_upgrade`/handlers in `semio-hub`).

- **`⚙️engine/🦀️component.rs`, `Database::document`** (now line 990, was ~906–907): `if let Some(authority)
  = self.open_artifacts.lock()....get(&id.0) { … to_core_document_id(id).await … }` — the `if let`
  scrutinee's `MutexGuard` temporary is scope-extended across the `.await` in the block body.
- **`⚙️engine/🦀️component.rs`, `Database::create_document`** (now line 953, was ~885–896): `let mut catalog
  = self.catalog.lock()...;` bound to a local, held across `now_ms().await` and `encode_catalog(..)
  .await` (and the pre-existing `db_actor::block_on(cas_root(..))` call, which is a plain blocking fn
  call, not a compiler-visible `.await`, so it wasn't itself part of the Send failure).
- **`🔒️security/🦀️component.rs`, `SecurityGate::admit_command`** (line 610, was line ~620): `if
  lock(&self.replay).check_and_record(..).await.is_err() { .. }` — here the guard doesn't need scope
  extension to explain the failure: the temporary is required to stay alive for the `.await` that's
  part of the same condition expression, by ordinary borrow-checking, regardless of if/if-let shape.

## Same pattern found elsewhere — a 4th, larger site

Grepped every `db` file containing `.await` for `if`/`if let`/`while let` scrutinees and bare `let`
bindings holding a `.lock()`/`lock(...)` guard (`grep -rnE "^\s*(if|if let|while|while let).*lock\("`
plus a manual triage of every remaining `.lock(` call site). Found one more, larger occurrence,
**deliberately introduced**, not mechanical:

**`⚙️engine/🦀️component.rs`, `vcs_integration::VcsVersionGraph`** (`ensure_store`, and the
`VersionGraph` impl's `record_change`/`checkpoint`/`merge_base`/`head` — lines 540/562/589/608/625) —
a doc comment already on `ensure_store` (kept, extended) explains why: a prior refactor needed
`&mut HashStore` held across real `.await` points, and a sync closure can't contain `.await` (R10
shape 1), so the guard was moved out of a closure and held directly across the awaits instead —
reintroducing the exact `MutexGuard`-across-`.await` shape R7 is about, at 5 call sites instead of 1.
This one was **not** caught by clippy/rustc until now for the same reason as the 3 reported sites:
nothing in `db`'s own test suite needs a `Send` future.

## The fix chosen, and why

Not a uniform mechanical patch — the four sites needed three different justifications:

1. **`document`**: genuinely shorten the critical section. `self.open_artifacts.lock()....get(&id.0)
   .cloned()` clones the `Arc<ArtifactAuthority>` and ends the guard's temporary at that `let`'s
   semicolon, before the real `.await` two lines later. Zero behavior change — same data read, same
   `Arc::clone` cost, just no longer inside the guard's lexical scope.

2. **`create_document`**: the guard must span check → build → CAS-write → commit atomically (that's
   what `catalog`'s mutex is *for* — this crate's "CAS-fenced" catalog-write contract, and shortening
   it for real would let two in-process `create_document` calls race the storage-level CAS and fail
   each other with spurious `Fenced` errors that could never happen before, since the guard used to
   serialize all `create_document` calls end-to-end). `now_ms`/`encode_catalog` were checked (their
   bodies contain no `.await` at all — `async fn` purely for this crate's interface uniformity, never
   truly suspending) and folded into the same `commit` future the pre-existing `cas_root` call already
   used — see the wasm note below for exactly how it's driven.

3. **`admit_command` / `ReplayGuard::check_and_record`**: `check_and_record`'s body (line 383) has zero
   `.await` — pure `HashMap`/`VecDeque` bookkeeping, `async fn` for no reason. Its sibling one line up,
   `BudgetRegistry::try_consume`, is already a plain sync `fn` carrying this crate's own `// 🚫️async:
   E1 pure accessor … — see R9` convention marker. `check_and_record` was the odd one out. Removed
   `async`, updated the one call site and the 10 test call sites (`sed`-scoped to the test region,
   verified it only touched `check_and_record(...).await` and not the `actor(...).await`/`op(...)
   .await` calls nested inside the same lines). This is the same class of fix R9 already codifies
   elsewhere in this crate, not a new pattern.

4. **`VcsVersionGraph`'s 5 functions**: these mutate one document's `ArtifactStore` in place and must
   stay atomic for the whole dispatch — unlike site 1/2, there's no cheap read/clone to hoist outside
   the lock; the lock genuinely has to span the mutation. Confirmed by reading `ArtifactStore::
   dispatch`/`envelope`/`current_checkpoint_id` and grepping the whole `🏪️store` crate for I/O/channel
   primitives (`mpsc`, `tokio`, `TcpStream`, `File::`, `sleep`, …: zero matches) that this call graph
   is pure in-memory computation, never genuinely suspending. So the guarded body is wrapped in
   `db_actor::block_on` — this crate's own sanctioned single executor bridge, already used elsewhere
   in this same file — which resolves it in one poll and removes the compiler-visible suspension point
   from the guard's scope entirely, preserving the exact original atomicity (unlike sites 1/2, nothing
   here got a smaller critical section — it kept the same span, just stopped suspending across it).
   `#[allow(clippy::await_holding_lock)]` added at each of the 5 sites with an inline rationale: the
   lint is syntactically correct but the risk it warns about (blocking another thread's genuine
   suspension) doesn't apply, per the proof above.

Rejected: swapping `std::sync::Mutex` for an async mutex anywhere (forbidden by the brief, and would
have hidden a real "lock held across suspension" liveness question behind a type that happens to be
`Send`, rather than answering it) — and, for `create_document`, an initial draft that shortened the
critical section with a CAS-retry loop, dropped because it would let concurrent same-process
`create_document` calls for different documents spuriously fail with `Fenced` where they always
succeeded before — a real regression the retry loop would have masked rather than the intended
Send-only fix.

## wasm32 correctness — the fix's own second-order bug, caught and fixed

`db_actor::block_on` is `#[cfg(not(target_arch = "wasm32"))]` — it doesn't exist on `wasm32`. The
first version of this fix used `db_actor::block_on` unconditionally in `create_document` and all 5
`VcsVersionGraph` functions, which compiled and passed every native check/test/clippy run **but broke
`cargo check --target wasm32-unknown-unknown`** with 7 new `E0425: cannot find function block_on`
errors (5 in `vcs_integration`, which held its guard across a real, wasm-compatible `.await` before
this packet touched it — a genuine regression; `create_document`'s block_on call was already
wasm-broken pre-existing via `cas_root`, so those 2 were not a net-new regression there). Caught only
by actually running the wasm target check, not by native testing — exactly the trap the packet brief
warned about ("wasm code never compiles during a native check").

Fixed by driving each guarded async block two different ways depending on target, since `Send` is only
required where a multi-threaded work-stealing scheduler needs to move the future across threads
(`semio-hub`'s axum handlers) — `wasm32` has no such scheduler, so holding the guard across a real
`.await` there is exactly as safe as it was before this packet:

```rust
let work = async { /* unchanged guarded body */ };
#[cfg(not(target_arch = "wasm32"))]
let result = db_actor::block_on(work);
#[cfg(target_arch = "wasm32")]
let result = work.await;
result
```

`entries`/`epoch`/`document` etc. are captured by reference (not moved) by `work`, so this costs
nothing beyond the two-line `#[cfg]` split — no duplicated business logic. Re-ran the wasm check after
this: **0 new errors** (101 pre-existing wasm errors remain, all in files/lines this packet never
touched — `📄️artifact`, `⌨️cli`, `🧪️testkit`, `👁️preview`, and other pre-existing `db_actor::block_on`
call sites in `⚙️engine` this packet didn't add, e.g. `open`/`open_at`/`compact`/`sync`/`submit`).
`use crate::db_actor;` inside `vcs_integration` is itself `#[cfg(not(target_arch = "wasm32"))]`-gated
to avoid an unused-import warning on wasm.

## Commands run — actual results

| Command | Result |
|---|---|
| `cargo check -p semio-framework-os-kernel-db --all-targets --features sqlite,postgres,neo4j` | 0 errors (was 5, all `E0433 cannot find db_actor` mid-fix after the first `vcs_integration` edit, before adding `use crate::db_actor;` inside the module) |
| `cargo test -p semio-framework-os-kernel-db --all-targets --features sqlite,postgres,neo4j` | 478 passed; 0 failed |
| `cargo test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --release` | 478 passed; 0 failed |
| `cargo test -p semio-framework-os-kernel-db --features sqlite` (defaults included) | 433 passed; 0 failed |
| `cargo test -p semio-framework-os-kernel-db --features postgres` (defaults included) | 447 passed; 0 failed |
| `cargo test -p semio-framework-os-kernel-db --features neo4j` (defaults included) | 446 passed; 0 failed |
| `cargo clippy -p semio-framework-os-kernel-db --all-targets --features sqlite,postgres,neo4j` | 0 errors |
| `cargo clippy -p semio-framework-os-kernel-db --all-targets --no-deps --features sqlite,postgres,neo4j -- -D warnings` | 2 pre-existing findings in `⚙️engine/🦀️component.rs` (an unrelated unused import at line 48, a pre-existing macro-export-absolute-path warning at line 655), 0 in `🔒️security/🦀️component.rs`; neither on a line this packet touched |
| `cargo check -p semio-framework-os-kernel-db --target wasm32-unknown-unknown` | 101 errors, all pre-existing, none on a line this packet touched (verified by cross-referencing every reported line number against this packet's diff) |
| `rustfmt --check` on both edited files | diffs only in pre-existing, untouched regions (top-of-file import ordering in `⚙️engine`; two long-line collapses inside `🔒️security` that predate this packet's one-line signature/call-site edits) — left alone per "format only files you edited," not blanket-reformatted |
| `bun ./📜️script.ts verify dependencies` | clean — 238 → 238 |
| `cargo check -p semio-hub --all-targets --features sqlite,postgres,neo4j` | **0 errors** (was 9 bin + 9 test = 18) |
| `cargo check -p semio-hub --all-targets --release --features sqlite,postgres,neo4j` | 0 errors |
| `cargo check -p semio-hub --all-targets --no-default-features --features sqlite` / `postgres` / `neo4j` | 0 errors each |
| `cargo test -p semio-hub --features sqlite,postgres,neo4j --no-run` | builds clean (bin + test) |
| `cargo test -p semio-hub --features sqlite,postgres,neo4j --no-fail-fast` | `os-hub` bin test binary (the one that had the 9+9 Send errors): **20 passed, 0 failed**. `semio_hub` lib test binary: 11 passed, 3 failed — all 3 failures are `directory::postgres::tests::*` panicking on `start postgres container: Client(Init(SocketNotFoundError("/var/run/docker.sock")))` — this sandbox has no reachable Docker daemon for hub's own postgres-testcontainer tests. Unrelated to this packet: hub's `directory` module, not `db_engine`/`db_security`, and pre-dates this packet's diff. |
| `cargo clippy -p semio-hub --all-targets --features sqlite,postgres,neo4j` | 0 errors |

## Hub error trajectory

| | bin errors | test errors |
|---|---|---|
| Baseline (R4's handoff, this packet's session start) | 9 | 9 |
| After this packet | **0** | **0** |

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs` — `Database::document` (guard
  extraction), `Database::create_document` (critical section restructured, wasm-dual-path `commit`),
  `vcs_integration` module (`use crate::db_actor;` added, wasm-gated; `ensure_store`/`record_change`/
  `checkpoint`/`merge_base`/`head` restructured onto the same wasm-dual-path pattern,
  `#[allow(clippy::await_holding_lock)]` + rationale on each).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔒️security/🦀️component.rs` — `ReplayGuard::
  check_and_record` de-asynced (R9), its one call site in `SecurityGate::admit_command`, and 10 test
  call sites in the `#[cfg(test)] mod tests` replay-guard block.

No file in `🌎️hub/**` was edited (verification only, matching this packet's ownership boundary). No
file under `./compose` was touched.
