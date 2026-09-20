# HS1 — the hub pool-worker stack overflow that kills the first document socket

Slice HS1 (session 6, 2026-09-20). Blocker of outcome 3 (two users editing one shared document):
`thread 'semio-pool-worker-N' has overflowed its stack / fatal runtime error: stack overflow,
aborting`, seconds after the hub's first `server.document.socket outcome=ok`
(C2 §0/§4, captures `🗑️generated/c2-hub-crash-gm1-hold.txt`, `c2-hub-crash-2.txt`).

## 0. Headline

**It is not a recursion and it is not a leak: it is six frames that together reserve 8.5 MiB on a
2 MiB stack.** Measured from the crash reports' own instruction bytes
(`🐍️hs1-frame-sizes.py`, capture `🗑️generated/hs1-frame-sizes.txt`), the engine-open path is

```
PoolWork::run  (a 2 MiB pool-worker thread)
 └ ArtifactRunner::<AllowAll,VersionGraphs>::run_turn                       3,598,608 B
    └ std::panic::catch_unwind::<…, Poll<Result<ArtifactEngine<…>, …>>>       529,392 B
       └ catch_unwind::do_call::<…, Poll<Result<ArtifactEngine<…>, …>>>       529,376 B
          └ run_turn::{closure}                                                     32 B
             └ Database::run_open_document_mount::{closure}{closure}{closure} 1,777,280 B
                └ ArtifactEngine::open_retained::{closure}                   1,718,992 B
                   └ db_artifact::DocumentState::new                       174,656 B × 2
                                                                        ───────────────
                                                            total         8,506,832 B
```

The repeating unit is `size_of::<Poll<Result<ArtifactEngine<A,V>, ArtifactEngineOpenRejected>>>` —
the two `catch_unwind` shim frames, which hold nothing else, price it at **~529 KB**. That value is
returned **by value** through `run_turn`'s `catch_unwind` on the open turn *and on every later
message turn* (`ArtifactTurn::Future(Pin<Box<dyn Future<Output = ArtifactEngine<A,V>>>>)`), and a
debug build gives every move its own stack slot, so `run_turn` alone holds ≈6.8 copies of it.

**Reproduced deterministically in ~1 s, headless, with no browser**, on this slice's own hub and own
data root: `bun 🐍️hs1-socket-repro.ts` (§2). The trigger is the `SocketHelloV1` frame — a probe that
only upgrades the WebSocket never gets past `handle_ws`'s 2 s hello timeout and therefore never
crashes the hub, which is why the fault looked browser-only.

Measured stack requirement of the same path, by booting the same binary under a raised
`RUST_MIN_STACK` (`📜️hs1-stack-bisect.sh` — `WorkerPool::new` spawns with no `stack_size`, so that
variable IS the workers' budget):

| pool-worker stack | verdict |
|---|---|
| 2 MiB (today's default) | **overflow** — `🗑️generated/hs1-hub-hold-gm1.txt` |
| 4 MiB | **overflow** — `🗑️generated/hs1-stack-4194304.txt` |
| 8 MiB | **overflow**, total reservation measured at 8,506,832 B — `hs1-stack-8388608.txt` |
| 16 MiB | **survives**: `Welcome` + `Session` frames, socket held 20 s, hub alive — `hs1-stack-16777216.txt` |

## 1. The spawn site and the budget it gives (read, then measured)

`🧰️framework/🔨️modules/⏳️async/🦀️.rs:1868` —

```rust
let handle = thread::Builder::new().name(format!("semio-pool-worker-{index}"))
    .spawn(move || worker_loop(&worker_inner, index as u32))
    .expect("WorkerPool: failed to spawn worker thread");
```

No `stack_size`, so every pool worker gets **Rust's 2 MiB default** while the process main thread
gets macOS's 8 MiB. The crate's own docstring at `🦀️.rs:1478-1482` already states this ("The budget
to pin is the one `WorkerPool`'s workers actually get — Rust's 2 MiB default, since `native_pool`
spawns them with no `stack_size` of its own") and the repo test runner floors `RUST_MIN_STACK` at
32-256 MiB (`⏳️async/📦️packages/🦀️rust/📜️script.ts:12-13`,
`🔌️plugin/📦️packages/🦀️rust/📜️script.ts:40`), so **no cargo test can ever observe the production
budget** — which is why the suite is green and the binary aborts.

Measured, not inferred: `📜️hs1-stack-bisect.sh` boots the SAME binary under a raised
`RUST_MIN_STACK` (which `Builder::spawn` reads when no `stack_size` is given) and re-runs the repro.
The table in §0 is that experiment. It also settles the "is it recursion?" question from the other
side: a runaway chain would overflow at 16 MiB too.

## 2. Reproduction on an own hub

Nothing of GM1's or C2's was touched: hub 7611 (pid 83162), the `s` serve on 6190 and the `gis2d`
serve on 6191 were left running throughout.

| what | value |
|---|---|
| data root | `.🧬semio/🌐hub/hs1-boot` — a `cp -R` of `gm1-boot` (614 MB, `chmod 700`), so it carries the published trusted catalog AND the space/document/users C2 used |
| binaries | `⚡️cache/hs1/os-hub` (copy of the coordinator's 21:36 build) and `⚡️cache/hs1/os-hub-gm1` (copy of GM1's 19:38 build, the one C2 crashed) — both `rm`+`cp`+`codesign -f -s -`, never run in place |
| port | **7631** (7611 untouched) |
| hold | `📜️hs1-hub-hold.sh <port> <binary> [dataRoot]`, `OS_HUB_CREDENTIAL_SIGN_IN=true` |
| probe | `🐍️hs1-socket-repro.ts <origin> <spaceId> <documentId> [surface] [holdMs] [edits]` |

Boot note (measured, not a defect of this slice): `configured_artifact_authority` gives catalog
resolution a hard 30 s budget (`🌎️hub/🏗️bootstrap/🦀️.rs:590`) with no env knob, so under fleet load the
first one or two boots die with `Error: ArtifactAuthority(DeadlineExceeded)` before `/readyz` ever
answers. `SEMIO_BUILD_BUDGET_MS` does **not** reach the hub — no Rust file reads it; C2 §4's
attribution of the recovery to that variable is a coincidence of load. The scripts here simply retry
the boot up to four times, which is what actually works.

**Both binaries reproduce it identically**, so the crash is not something the coordinator's newer
build already fixed. The three crash reports share one shape:

| report | thread | innermost product frames |
|---|---|---|
| `os-hub-2026-09-20-213129.ips` (C2's `c2-hub-crash-2.txt`) | `semio-pool-worker-4` | `ArtifactRunner::submit_scheduled::{closure}` ×2 |
| `os-hub-2026-09-20-213407.ips` (C2's `c2-hub-crash-gm1-hold.txt`) | `semio-pool-worker-3` | `spawn_with_pool_use::{closure}` ×2 → `run_open_document_mount` |
| `os-hub-gm1-2026-09-20-220507.ips` (this slice) | `semio-pool-worker-8` | identical to the row above |

Every one is `EXC_BAD_ACCESS / KERN_PROTECTION_FAILURE` on the thread's own stack-guard page, with
29-39 frames in total — a bounded chain, not a runaway one.

## 3. The overflowing stack — frames

`🐍️hs1-frame-sizes.py <report.ips> <binary>` reads each frame's function start out of the report
(`imageOffset - symbolLocation`), seeks to it in the Mach-O and decodes the AArch64 prologue. These
frames do not use a plain `sub sp, sp, #imm`: LLVM emits a **probe loop** —

```
sub x9, sp, #1654784          ; ← the frame's true size
sub sp, sp, #4096             ; ┐ touch one page,
str xzr, [sp]                 ; │ compare, repeat
cmp sp, x9                    ; │
b.ne -12                      ; ┘
sub sp, sp, #3008             ; remainder
```

so the size is the FIRST instruction's immediate. Reading only the `sub sp` forms undercounts by
three orders of magnitude, which is why "33 frames" first read like an ordinary stack.

Two crashes, two depths, because a bigger stack lets the chain get further before it dies:

| stack | faulting frame | total reserved |
|---|---|---|
| 2 MiB | `spawn_with_pool_use::{closure}` (1,657,792 B, entered twice) | ≥ 2,129,920 B — the whole stack |
| 8 MiB | `db_artifact::DocumentState::new` (174,656 B, entered twice) | **8,506,832 B** (§0 table) |

`vmRegionInfo` in both reports puts the faulting address inside the thread's own `Stack Guard`
region, a few KB past the stack's low limit — a genuine "used it all", not a wild pointer.

**Nothing recurses.** The chain is 29-39 frames in every capture and every frame is a distinct
function. What repeats is a *value*: the two `catch_unwind` shim frames hold nothing but
`Poll<Result<ArtifactEngine<A,V>, ArtifactEngineOpenRejected>>` and price it at ~529 KB, and
`run_turn` — which moves that result out of `catch_unwind` into `self.engine` — reserves 3,598,608 B,
i.e. about seven slots of it. A debug build gives every move its own slot and reuses none.

## 4. Root fix

Two changes, in that order of importance.

### 4.1 The engine moves as a pointer, not as a 529 KB value (`🗿️artifact/🦀️.rs`)

`ArtifactRunner` owned its engine by value and every future it drives yielded it by value:

```rust
type ArtifactBuildFuture<A, V> = Pin<Box<dyn Future<Output = Result<ArtifactEngine<A, V>, ArtifactEngineOpenRejected>> + Send>>;
type ArtifactTurnFuture<A, V>  = Pin<Box<dyn Future<Output = ArtifactEngine<A, V>> + Send>>;
enum ArtifactTurn<A, V> { Future(…), History { engine: Option<ArtifactEngine<A, V>>, … } }
engine: std::sync::Mutex<Option<ArtifactEngine<A, V>>>,
```

so `run_turn`'s `std::panic::catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context)))`
carried ~529 KB out through `catch_unwind` → `do_call` → its own frame, **on the open turn and on
every later message turn** (which is why C2's second crash landed in `submit_scheduled` with no
document socket in the log at all). All four now carry `Box<ArtifactEngine<A, V>>`; `start_turn`
takes the box, and the two `db_engine` builders plus the four `🔬️unit` test builders say
`.map(Box::new)`. Method calls auto-deref, so no body changed.

### 4.1b The turn coroutine holds ONE method future, not ten (`🗿️artifact/🦀️.rs`)

Found only after 4.1/4.2 landed, because an explicit `stack_size` **overrides** `RUST_MIN_STACK` in
both directions and so removed a floor nobody had noticed: `.cargo/config.toml:65` sets
`RUST_MIN_STACK = "67108864"` repo-wide, so every pool worker in every cargo-launched process has had
**64 MiB** all along, while the shipped `os-hub` — started by a holder, not by cargo — had 2 MiB.
That single line is why the hub suite was green while the binary aborted. Declaring 16 MiB dropped
three previously green hub laws to `SIGABRT` (`socket_grant_revoke_before_command_admission_has_no_storage_effect`,
`checkpoint_publication_route_is_author_owned_actor_fenced_idempotent_and_cancellation_safe`,
`checkpoint_publication_route_rejects_stale_or_cross_scope_inputs_before_publication`) — a free,
in-process, 0.2-2.4 s repro that priced the deepest legitimate path honestly for the first time:

```
ArtifactRunner::start_turn::{closure}   10,711,040 B   ← entered twice
ArtifactRunner::run_turn                    94,208 B
… 34 further frames                          <1,000 B each
                                      ───────────────
                              total     21,520,944 B
```
(`🗑️generated/hs1-frame-sizes-testbin.txt`, from the nextest crash reports.)

`start_turn` wrapped **one** async block around a ten-arm `match` over `ArtifactMessage`, awaiting a
different engine method in each arm. A coroutine's frame is the sum of its suspend points' live
locals and a debug build overlaps none of them, so that one frame carried `submit`, `compact_retained`,
`query`, `snapshot_now`, `recover_durable_group_decisions` and the rest **simultaneously**. The match
now sits OUTSIDE the async block — one `Box::pin(async move { … })` per message kind, all erased to
the same `ArtifactTurnFuture` — which turns that sum into a max. Nothing downstream changed.

This also explains the report DB1 filed while this slice was measuring: hub 7611 aborting right
after the first `server.auth.session.read`, with no document socket anywhere. `start_turn` is on
**every** artifact-engine message, so any read that reaches a mounted engine hits the same frame;
the document socket was simply the first path anyone watched.

### 4.2 The pool states its workers' stack instead of inheriting one (`⏳️async/🦀️.rs`)

`WorkerPool::new` spawned through `thread::Builder` with **no `stack_size`**, so the budget was
`std::thread`'s 2 MiB — a number nobody chose. It is now
`.stack_size(WORKER_STACK_BYTES)` with `WORKER_STACK_BYTES = 16 MiB`, a public constant whose
docstring carries the 8,506,832 B measurement and the bisection. Thread stacks are lazily committed,
so this costs address space, not resident memory. The stale docstring on `on_bounded_stack`, which
told every future law author that the budget to pin is "Rust's 2 MiB default", now names the
constant.

This is deliberately belt-and-braces: 4.1 removes the inflation, 4.2 removes the *silent* budget.
Either alone leaves a cliff — 4.1 because the debug profile inflates every other frame on this path
too (`open_retained`'s own coroutine is 1.72 MiB before any engine move), 4.2 because a budget with
no headroom is a cliff by another name.

### 4.3 The law

`native_pool_workers_own_the_stated_worker_stack`
(`⏳️async/🧪️tests/🔬️native-pool-unit/🦀️.rs`) submits a real job to a real `WorkerPool` and consumes
**6 MiB** inside it — past the old 2 MiB default, short of the 16 MiB budget — in 64 KiB frames that
are written through `black_box` so nothing is elided, then asserts it ran on a `semio-pool-worker-*`
thread. `Builder::stack_size` overrides `RUST_MIN_STACK` in both directions, so this law sees the
production number however the runner exports it — which matters, because every repo runner floors
that variable at 32-256 MiB and therefore **no ordinary cargo test can observe the shipped budget**.
If the `stack_size` call is ever dropped the test binary aborts with the very message the hub
printed. Run and passing: `🗑️generated/hs1-pool-stack-law.txt`.

## 5. Verification

(filling)

## 6. Restart command for the collaboration successor (hub 7611)

(filling)

## 7. Honest gaps

(filling)

## 8. Files changed

(filling)
