# W-F6 — The command-page authority, and the quarter-megabyte it asked for per command

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave F6, following W-F5
(`📓️2026-09-10-wave-F5-fill-budget-cancel.md`) and W-F4 (`📓️2026-09-09-wave-F-fill-oom.md`).
Written incrementally while the work ran.

## 0. Assignment

Release build #29 (2026-09-10 11:10, all W-F4/W-F5 fixes in), Fill armed:

- the plan advanced for **184 ticks / ~44 s** (`fillBuildTick` 250–350 ms, count slider `ready: 31`,
  `Cancel fill` visible);
- then **every** `fillBuildTick` answered
  `{"code":"plugin.command-page-allocation","message":"fixed command page authority could not
  reserve its exact 64 slots","origin":"framework","retryable":false}` — 32 in a row;
- ~2 s later the guest trapped with wasm `unreachable` and stayed dead (`shard 0 lost`).

Three deliverables: (1) name who reserves and who releases the 64 slots and prove the defect with a
native law driving ≥ 300 commands through the ingress the host actually uses; (2) fix at the cause;
(3) make exhaustion a refusal the host can display, never a trap, with a law.

## 1. Who reserves the 64 slots

### 1.1 The one production site

`grep -a` for the fault text finds exactly one producer:

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:156` —

```rust
pub fn try_new() -> Result<Self, crate::Fault> {
    let mut pages = std::collections::VecDeque::new();
    pages.try_reserve_exact(COMMAND_MAXIMUM_PAGES).map_err(|_| crate::Fault::new(…, "plugin.command-page-allocation", "fixed command page authority could not reserve its exact 64 slots"))?;
```

and exactly one guest caller: the reactor's command-ingress prologue,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:781`, in the
`cursor.page_index == 0 && retained.is_none()` arm — i.e. **once per command**, on page 0.

Everything else that constructs a `CommandPageSet` is host-side (`CommandPageWriter` /
`encode_app_command` in `🏃️run`, `🌉️mcp`, the wgpu renderer) or a test.

### 1.2 What the reservation costs

`FixedCommandPage` is `{ bytes: [u8; 4096], len: u16 }` → `size_of` **4 098 B** (align 2).
`VecDeque::try_reserve_exact(COMMAND_MAXIMUM_PAGES = 64)` on an empty deque allocates exactly 64
elements:

```
64 * 4 098 = 262 272 contiguous bytes, per command, whatever the command's declared page count is.
```

A `fillBuildTick` is **one** page of a few hundred bytes. At 4 Hz the guest was asking its single
fixed linear memory for a quarter-megabyte contiguous block four times a second, and throwing it
away again.

### 1.3 Who releases them — and why the fault is not a leak

The set is transient by construction. Its pages are popped one per `close_step`, the deque dies with
the `PagedCommand` (`PagedCommand::try_from_pages` MOVES the same deque; `PagedCommandReader` and
`PagedAppCommandDecodeCursor` move it again — there is never a second copy), and the retained
authority itself is a **fixed two-slot array**
(`COMMAND_INGRESS: RefCell<[Option<RetainedCommandIngress>; 2]>`, `🔄️turn/🦀️.rs:46`) that holds an
owner only while its own command is mid-assembly, mid-dispatch or mid-close.

So the browser fault is **not** slot saturation. Slot saturation has its own code,
`plugin.command-page-count` ("command page authority is saturated"), and it never fired. What fired
is `try_reserve_exact` refusing — an **allocation** refusal. The guest's 512 MiB linear memory
(`.cargo/config.toml`'s `--max-memory=536870912`) could no longer produce a 262 272-byte contiguous
block.

### 1.4 Why that block is the first thing a guest refuses — and the theory that turned out wrong

The first hypothesis was that `dlmalloc` routes ≥ 256 KiB requests through an mmap path whose freed
chunks wasm can never return, so every command would leak 256 KiB outright. **That is false, and it
was checked rather than assumed.** `dlmalloc-rs`
(`https://raw.githubusercontent.com/alexcrichton/dlmalloc-rs/main/src/dlmalloc.rs`) has no
`mmap_threshold` and no `mmap_alloc`; `sys_alloc` always goes through
`system_allocator.alloc(align_up(size + …, self.granularity))` and manages the block through
`top`/bins like any other, so a freed 256 KiB chunk IS recycled.

What is true, and is what matters:

- `dlmalloc-rs`'s wasm backend (`src/wasm.rs`) returns `false` from `free`, `free_part` and
  `can_release_part` and `null` from `remap` — **the guest's linear memory grows and never shrinks**;
- its `granularity` is `64 * 1024` — the unit `memory.grow` moves in.

A request at or under one granularity unit is served from a small bin, a `dv`/`top` split, or one
page of growth. A quarter-megabyte contiguous request needs either a pre-existing large free chunk or
a multi-page grow, so on a memory that only ever grows it is **the first request a fragmented or
nearly-full guest refuses** — which is exactly the order build #29 observed: 32 ingress refusals
first, the guest still answering, then a smaller infallible allocation finally failing and taking the
guest down with `rust_oom` → `unreachable`.

That is the honest causal statement. The command-page prologue is both a **victim** (largest routine
request, refuses first) and a **contributor** (a quarter-megabyte of contiguous churn four times a
second is the single biggest fragmentation source on the command path).

## 1.5 And the `unreachable` two seconds later — localised

The coordinator's browser capture (`📋️master-plan-2026-09-08.md`, W-F6 evidence) named the trap
exactly:

```
[handler/stepJob] panicked at …/✏️editor/⏳️precompute/📐️geometry/🦀️.rs:263:39: live fixed owner page
```

`📐️geometry/🦀️.rs` carries the fill session's owner pages —
`FixedOwnerVec<T, N>` / `FixedOwnerMap<K, V, N>` / `FixedOwnerSet<K, N>`, each one
`Option<Box<[…; N]>>` allocated with `try_reserve_exact` so that, in the words of its own docstring,
*"allocation failure [is] a handled `None` page (capacity zero) instead of an abort"*.

It was not capacity zero. It was a landmine:

```rust
pub(crate) const fn capacity(&self) -> usize { N }              // even with page: None

pub(crate) fn try_push(&mut self, value: T) -> Result<(), T> {
    if self.len == N { return Err(value); }                      // 0 == 2048 → false
    self.page.as_mut().expect("live fixed owner page")[self.len].write(value);   // 💥
```

and identically in `FixedOwnerMap::try_insert` (line 263). An owner whose page the guest could not
spare reported its FULL declared width, passed the length guard, and `expect`ed the page it never
got. Every collision-mutation preflight in the same file
(`self.entries.len() == self.entries.capacity()`, `bucket.len() == bucket.capacity()`,
`self.cells.len() + missing - reclaimed > self.cells.capacity()`) was comparing against that same
lie, so all four `unreachable!("preflighted …")` arms below them were reachable too.

The widest of these pages is `FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS = 2048>` — **≈432
KiB contiguous**, with `DOCUMENT_OWNER_PAGE_BYTES` allowing up to 1 MiB per page. So the fill
session's own owners are the same request class the command prologue was in, and they were being
asked for on a memory that had just refused a 262 KiB block. That is the whole sequence:

1. the guest's linear memory grows and never shrinks;
2. the command-ingress prologue asks for 262 272 contiguous bytes per command, 4×/s;
3. the memory can no longer serve a request that size — the prologue refuses **visibly**, 32 times,
   and the host displays each one;
4. a fill-job owner page is refused at the same time — **invisibly**, because `new()` swallows it;
5. the next `try_push`/`try_insert` on that owner `expect`s → panic → wasm `unreachable` →
   `shard 0 lost`.

## 2. The fix


`CommandPageSet` now reserves **exactly the pages its command declares**:

```rust
pub fn try_new(declared: usize) -> Result<Self, crate::Fault>   // refuses 0 and > 64
pub const fn reservation_bytes(declared: usize) -> usize        // declared * size_of::<FixedCommandPage>()
pub fn declared(&self) -> usize
```

`try_push` now saturates at `self.declared`, not at the 64-page ceiling — the authority is the
command's own declaration, and the declaration is already validated `1..=COMMAND_MAXIMUM_PAGES` by
the ingress cursor guard and is identical for every page of one command
(`same_command_cursor` compares `page_count`), so an exact reservation still admits every page of
that command without a second allocation. Nothing about the "one allocation, no mid-assembly growth"
doctrine is weakened.

Call sites:

| site | declares |
| --- | --- |
| `⚛️reactor/🔄️turn/🦀️.rs` ingress prologue (**the guest's only one**) | `cursor.page_count` |
| `encode_app_command`, Presence branch | `peers.len().max(1)` |
| `CommandPageWriter::try_new` | `COMMAND_MAXIMUM_PAGES` — the generic encoder discovers its page count while writing, and it runs on a host, never on the guest's fixed memory (documented at the site) |

Effect on the browser path: a one-page `fillBuildTick` now reserves **4 098 B instead of
262 272 B** — a 64× reduction, and below the growth granularity, so it is served from a bin instead
of demanding a multi-page grow.

### 2.1 A named budget for it

`semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES = 65_536` (one wasm page / one
`dlmalloc` growth unit) is new, pinned to `🧮️memory/🧬️schema/🔣️.json`'s
`contiguousRequestCeilingBytes`, and documented with the reasoning above. It is the bound a routine
per-command or per-turn guest path must stay under, so the next path that asks the guest for a large
contiguous block has a number to fail against rather than a browser trap to be discovered by.

## 3. The trap fix — a refused page must refuse

`⏳️precompute/📐️geometry/🦀️.rs`:

| before | after |
| --- | --- |
| `FixedOwnerVec::capacity()` did not exist; `FixedOwnerMap`/`FixedOwnerSet::capacity()` returned `N` unconditionally | all three answer `0` when `page.is_none()` — the honest capacity of an owner that has no page |
| `try_push`: `if self.len == N { Err } else { self.page.as_mut().expect("live fixed owner page")… }` | `let Some(page) = self.page.as_mut() else { return Err(value) }` |
| `FixedOwnerMap::try_insert`: same `expect` at line 263 | `let Some(page) = self.page.as_mut() else { return Err((key, value)) }` |
| a freshly created cell-member bucket was inserted unchecked, so a refused bucket reached `unreachable!("preflighted fixed collision member")` | the one owner the preflight cannot weigh is checked at the site: `if bucket.capacity() == 0 { return Self::reject_mutation(mutation) }` |
| the page-less state was reachable only through a failed `try_reserve_exact` | `FixedOwnerVec/Map/Set::refused()` is that state, named, and `new()` returns it on refusal — so a law can reach it without exhausting 512 MiB |

The four `unreachable!("preflighted …")` arms in `step_replacement` stay exactly as they are: with an
honest `capacity()`, every preflight that guards them (`entries`, an existing cell bucket,
`oversized`, `cells`) now refuses first, and the fifth path (a brand-new bucket) is guarded at the
site. An exhausted guest answers `CollisionMutationStep::Rejected(Capacity(id))` — the refusal the
fill lane already knows how to report — instead of aborting the whole component.

## 4. Laws

`⚛️reactor/🔄️turn/🧪️tests/📄️command-page-authority/🦀️.rs` (included next to `📏️future-size`
in the plugin-runtime builder-contract test module, which owns `TestRuntimeApps` and the
`reactor_native_lifecycle_*` harness):

| law | states |
| --- | --- |
| `a_command_page_authority_reserves_only_the_pages_its_command_declares` | For 1, 2, 8 and 64 declared pages the heap witness's own peak across `try_new` is at least the declared reservation and, below 64, strictly under the 64-page ceiling — so a one-page command no longer pays for 64. A one-page reservation stays inside `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`; the 64-page ceiling does not, which is why it may not be reserved for every command. |
| `a_saturated_command_page_authority_refuses_without_panicking` | `try_new(0)` and `try_new(65)` refuse with `plugin.command-page-count`; a third page against a two-page authority refuses and **hands the page back** rather than dropping it; a full 64-page authority refuses the next push; closing a saturated authority terminates; and the refusal survives `encode_fault_bytes`/`decode_fault_bytes` — the exact wire `CommandIngressStatus::Fault` carries it on — with `retryable == false`. No panic on any path. |
| `a_long_command_stream_never_pins_the_retained_ingress_authority` | 320 real commands, encoded by `encode_app_command` and paged by the real `CommandBatchDriver`, each driven page-by-page through `reactor::poll_kernel` exactly as the shard drives it. After every command the retained ingress occupancy is 0, and retained heap over the last three quarters of the run stays under 512 B per command. |

`retained_command_ingress_occupancy()` is new in `🔄️turn/🦀️.rs` and re-exported from `reactor` — the
two-slot authority's occupancy had no reader outside the module.

`⏳️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs`:

| law | states |
| --- | --- |
| `an_owner_whose_page_was_refused_refuses_every_insert_instead_of_trapping` | A `FixedOwnerVec`/`FixedOwnerMap`/`FixedOwnerSet` in the refused state reports capacity 0, refuses its first push/insert and hands the value back whole, reads as empty and terminal, and never panics — while an owner that got its page still reports its declared width. |

`🧮️memory/🧪️tests/🔬️memory/🦀️.rs` gains
`the_contiguous_request_ceiling_is_one_guest_growth_unit` and the schema assertion for the new
`contiguousRequestCeilingBytes` field.

## 5. Verification

All native runs under
`CARGO_TARGET_DIR=…/target-p3d-f RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 RUST_MIN_STACK=67108864`.

| run | result |
| --- | --- |
| `cargo test -p semio-framework-plugin --lib -j 4 -- --test-threads=1 command_page_authority long_command_stream` | **ok. 3 passed; 0 failed** (2.59 s, 627 filtered out) |
| its census | `[DEBUG] command page authority: 320 commands, 1286 turns, 3 faulted, peak ingress occupancy 0, 16368 B over the last 240, 68 B/command` |
| `cargo test -p semio-framework-trace --lib -j 4` | **ok. 31 passed; 0 failed** (0.03 s) |
| `cargo test -p semio-framework-os-kernel --lib -j 4 -- command_page paged_generic decoded_generic presence` | **ok. 29 passed; 0 failed** (1045 filtered out) |
| `cargo test -p semio-framework --lib -j 4 -- generic_ presence_ rejected_command retained_batch` (the kernel's own page/batch/driver laws) | **ok. 10 passed; 0 failed** (217 filtered out) |
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 refused_refuses_every_insert -- --test-threads=1` | **ok. 1 passed; 0 failed** (646 filtered out) |
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 geometry -- --test-threads=1` (the whole geometry family, the `capacity()`/`try_push`/`try_insert` change's blast radius) | **ok. 45 passed; 0 failed** (602 filtered out, 0.30 s) |
| `cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle -j 4` | **`Finished dev profile` in 4m 04s, EXIT 0** |

Two things the 320-command run settles: the guest's retained ingress authority is back to **0 of its
2 slots after every single command** (peak occupancy 0 across the whole stream — no owner is ever
pinned into the next command's turn), and the stream retains **68 B per command**, well under the
512 B ceiling, so the page authority is not a leak. The 3 faulted commands are the first ones, before
the fixture app has an admitted document; every one of the 320 reached a terminal ingress status.

### 5.1 What the law would have said before the fix

The first law measures with the heap witness's own peak, per declared page count, and asserts the
measurement is at least the declared reservation and — below 64 pages — strictly under the 64-page
ceiling of 262 272 B. On the old constant reservation every one of those readings is 262 272 B, so
the `declared = 1 / 2 / 8` cases fail on that second assertion, and
`reservation_bytes(1) <= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` fails as well (262 272 > 65 536).
The `declared = 64` case is the old behaviour and is the one the law still measures at 262 272 B.

### 5.2 Traces

One `[DEBUG]` line survives, in the 320-command law: its census
(`{commands} commands, {turns} turns, … B/command`). It is the law's own measured evidence and has
the same shape and lifetime as the neighbouring `📏️future-size` law's `[DEBUG] settled reactor turn
retention` census — a test-binary census, never production. No `[DEBUG]` was added to any production
path by this wave.

## 6. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` | `CommandPageSet::try_new(declared)` reserves exactly the declared pages and refuses `0`/`>64`; new `declared()` and `const reservation_bytes()`; `try_push` saturates at the declaration; `CommandPageWriter::try_new` and `encode_app_command`'s Presence branch declare theirs |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | the ingress prologue declares `cursor.page_count`; new `retained_command_ingress_occupancy()` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` | re-exports `retained_command_ingress_occupancy` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/📄️command-page-authority/🦀️.rs` | **new** — the three ingress laws |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | includes the new law file next to `📏️future-size` |
| `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs`, `🧰️framework/🔨️modules/⏱️trace/🦀️.rs` | `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` + its export |
| `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧬️schema/🔣️.json` | `contiguousRequestCeilingBytes` |
| `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️tests/🔬️memory/🦀️.rs` | schema assertion + the growth-unit law |
| `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/⏳️precompute/📐️geometry/🦀️.rs` | honest `capacity()` on all three fixed owners; `try_push`/`try_insert` refuse instead of `expect`; `refused()` constructors; the fresh cell-member bucket is checked before insertion |
| `✏️s/🔌️plugins/🧩️puzzle/…/⏳️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs` | the refused-owner law |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🔬️extension-activation/🦀️.rs`, `📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs`, `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs`, `🔌️plugin/🧪️tests/🔬️plugin-runtime-paged-command-ingress/🦀️.rs` | declare their page counts at the `try_new` call sites |

## 7. Served wasm

**The served wasm MUST be rebuilt.** Both halves of this wave are guest Rust: the ingress prologue
(`semio-framework-os-kernel` + the plugin reactor, linked into every guest) and the puzzle3d fill
owners. Nothing here reaches the browser on a reload.
`cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle` is green on this tree, so the rebuild
has nothing to fix first.

## 8. Open

1. **The command page authority was not the only large contiguous request per command.** Two others
   on the same guest path are over `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` and this wave did not
   touch them:
   - `plugin_exchange`'s future is **76 848 B**, boxed once per command
     (`🔄️turn/🦀️.rs`'s `plugin_exchange_boxed`, deliberate — inlining it made the whole turn
     generator that size);
   - `wit_bindgen`'s `start_task` boxes the whole turn future once per `poll`; boot #9b of ticket
     26/09/09 died on that allocation at **189 328 B**. `📏️future-size`'s
     `the_reactor_turn_future_reports_its_generator_size` measures it, and a peer wave was running
     `the_reactor_turn_future_fits` against it while this one ran.
   Neither is this wave's to change, but both are in the request class that a fixed, never-shrinking
   linear memory refuses first, and both happen far more often than 4 Hz.
2. **The fill session's own owner pages are large by design.**
   `FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>` is ≈432 KiB and `DOCUMENT_OWNER_PAGE_BYTES`
   admits up to 1 MiB per page. They are now honest about refusal, but they are still the largest
   single blocks the guest asks for; if the trap recurs after a wasm rebuild it will be one of these
   being refused, and the refusal will now show as `CollisionMutationStep::Rejected(Capacity(..))`
   rather than as a dead component.
3. `CommandEnvelopeSet::try_new()` still reserves 64 command slots AND 64 page slots (another
   262 272 B) unconditionally. Every caller is a HOST (`🏃️run`, `🌉️mcp`, the wgpu renderer) on the
   system allocator, so it is waste rather than a hazard — but it is the same shape and should get
   the same declared-count treatment when someone touches those hosts.
4. The `CommandBatchDriver` (Rust host) rejects the guest's very first answer to a one-page generic
   command with `plugin.command-cursor-mismatch`: the guest answers
   `CommandPending(page_index + 1)` straight out of the ingress prologue, while the driver only
   advances its own `page_index` on `PageAccepted`. The browser host is TypeScript and clearly does
   not, so this is a latent Rust-host-only mismatch, found while writing the 320-command law (the law
   drives the cursors directly instead). Not this wave's to fix; worth a ticket.
