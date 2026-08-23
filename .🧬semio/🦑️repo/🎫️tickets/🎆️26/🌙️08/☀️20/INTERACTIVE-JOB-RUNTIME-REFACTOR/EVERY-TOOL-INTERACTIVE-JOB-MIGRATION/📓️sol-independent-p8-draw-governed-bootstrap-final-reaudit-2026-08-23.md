# Sol Independent P8 Draw Governed-Bootstrap Final Re-Audit — 2026-08-23

## Verdict

**ACCEPT — Draw governed-bootstrap source cohort only.** The repair closes both blockers from
`sol-independent-p8-draw-rebuild-bootstrap-final-reaudit-2026-08-23.md`. Process-pool construction
is no longer advanced by app construction, the public request path, or an early arena borrow. The
only production path that constructs or retires bootstrap roots is the retained
`DrawMutationArenaBootstrapJob::step(&mut StepContext)` path reached by the real Draw store
initializer. The live source-undo and store-pump mutations now reject against the actual files.

I independently read the prior rejection, the updated P8 report, the staged Draw diff, the live
owned-schema/editor/verifier sources, and the retained rollback/pool fixtures. I did not author the
repair and made no production edit. Cargo, Nx, native, Wasm, browser, runtime, network, and allocator
timing were not run. Phase 8 remains **RED at 0/884 admitted commands, 18 global failure classes,
and runtime unverified**.

## Governed bootstrap boundary

### Allocation-free callers

- `DrawPlayApp::default` calls only `request_draw_mutation_arena_pool` and records a typed terminal
  fault (`editor/component.rs:159-168`). It contains no `Vec`, `String`, retained-page construction,
  `try_reserve`, bootstrap construction, or bootstrap step.
- `request_draw_mutation_arena_pool` coalesces one atomic request, initializes only inline
  `OnceLock<Mutex<Inert>>` metadata, uses `try_lock`, and returns `Ready`, `NotReady`, `Contended`,
  or `Fault` (`owned/component.rs:1288-1300`). It never constructs or advances a pool.
- `borrow_draw_mutation_arena` first observes that typed availability and fails closed on
  `NotReady`, `Contended`, or `Fault`; only the already-published `Ready(Arc<...>)` state can yield a
  fixed slot (`:1322-1334`). It does not bootstrap or reserve storage.
- The old production `initialize_draw_mutation_arena_pool` symbol is absent. The only similarly
  named helper is inside the `#[cfg(test)]` module.
- The second-app source fixture holds the process lock, performs two requests plus an early borrow,
  and compares the state/allocation witness before and after (`:5930-5955`). It is meaningful
  source evidence but was not executed because Rust builds were prohibited.

### Exclusive retained job

`DrawMutationArenaBootstrapJob` owns operation, generation, a checked fixed item/byte admission,
and terminal state (`owned/component.rs:1130-1249`). Its production step:

1. rejects an exhausted deadline/fuel budget before taking the process lock;
2. returns `Blocked` on lock contention;
3. starts `Inert -> Building` only after consuming the coalesced request;
4. advances one builder allocation or one retained close root, then consumes one fuel unit;
5. transitions allocation/arithmetic faults through `Building -> Retiring -> Fault`; and
6. publishes `Ready` only after all four fixed owners pass allocator-capacity reconciliation.

All production `Vec::try_reserve_exact`/`String::try_reserve_exact` calls for the two 64-slot
container roots, the 16-slot page catalog, sixteen 16-KiB pages, and the duplicate-ID owner are in
`DrawMutationArenaOwnerBuilder::step` (`:748-805`), which is invoked from
`DrawMutationArenaPoolBootstrap::step` (`:963-1022`), in turn invoked from the retained job
(`:1197`). Production construction of `DrawMutationArenaPoolBootstrap::production`, publication by
`take_pool`, and retirement by `close_step` are likewise inside that job. The one direct
`DrawMutationArenaPoolBootstrap::step` loop is `DrawMutationArenaPool::try_new`, explicitly
`#[cfg(test)]` (`:1255-1278`), and has no production reachability.

The real store route is not a marker-only occurrence:
`DrawStoreInitializationPhase::InitializeArena` directly matches
`self.arena_bootstrap_job.step(cx)` and maps its five typed outcomes before any envelope work
(`:5040-5048`). The cancellation fixture advances a local process by governed one-fuel turns,
cancels after a partial root, observes no `advanced_items > 1`, then checks terminal cancellation;
the same fixture covers zero budget, contention, and fifth-slot saturation (`:5958-6010`). The
80-allocation, four-bundle, exact/+1 aggregate, incomplete-publication, and one-root-close fixtures
remain present (`:5852-5928`).

The process-global `Building` owner intentionally remains in the process authority if a mismatched
store context is rejected before a turn; it is neither moved nor dropped and can only be advanced
by a later matching retained job. Cancellation of a matching job instead transfers it to
`Retiring`. I found no ordinary partial-root release path.

## Preserved rollback and pool ownership

- The exact close-side source reinsertion still calls `start_rebuild(...,
  DrawContainerRebuildRole::CloseSourceUndo)` (`owned/component.rs:4724-4728`), and
  `pump_rebuild_close` drives that role forward while all other roles roll back (`:4629-4635`).
- Active rebuild cancellation/staleness fixtures inject after a real move in each of four rebuild
  phases, restore the exact original live `Vec::as_ptr`, preserve FIFO IDs, return the exact
  reverse/output/page/duplicate roots, and require an advanced slot generation (`:6095-6157`).
- The reorder fixture interrupts destination rebuild after source handoff, proves exact nested
  backing/FIFO restoration, and reborrows the exact pool roots (`:6162-6223`).
- Arena handback remains four distinct phases. `leased` clears only after reverse, output, page
  catalog, and duplicate-ID owners have returned under the exact slot generation
  (`:4115-4173`). The capacity/+1 and reuse fixture checks pointer identity and generation advance
  (`:5750-5804`). These Rust fixtures were inspected, not executed.

## Independent live-source mutation replay

I dynamically loaded the current `toolJobDrawEnvelopeCallerRetainedExact` predicate and its Rust
brace parser, then evaluated it against the actual shared store, Draw owned-schema, Draw editor,
and plugin sources. Results:

| Case | Predicate |
| --- | --- |
| Unmodified live sources | `true` |
| Exact `CloseSourceUndo` call changed to `Destination` | `false` |
| Exact `match self.arena_bootstrap_job.step(cx)` changed to a bypass | `false` |
| Allocation inserted into the live public request block | `false` |
| Bootstrap step inserted into the live borrow block | `false` |
| Editor default request changed back to an initializer call | `false` |

This repairs the prior false acceptance: the predicate now matches the structured live
`InitializeArena` arm and the exact close-side `start_rebuild` call, rather than relying on another
enum/role occurrence. The current permanent synthetic matrix also completes **285 clean
self-tests**.

## Gates

| Gate | Result |
| --- | --- |
| Edition-2021 scoped `rustfmt --check` on Draw owned schema and editor | **PASS** |
| Whole TypeScript verifier parse through Bun's TS transpiler | **PASS** |
| `verify interactivity tool-jobs --self-test --format json` | **PASS: 285 clean** |
| Independent live-source predicate/mutation replay | **PASS: baseline true; 5/5 destructive mutations false** |
| Broad `verify interactivity` | **PASS: DENY clean**; one test-only bridge and two future-scope allowlist records only |
| Full tool-job verifier | Expected global **RED**: 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importer owners, 35 global payload stores, **18** failures; no Draw predicate failure |
| Independently regenerated ledgers | **PASS**: two 312,560-byte outputs, identical SHA-256 `8e38a992ffd6c810b5caf6f02f374ac4c3854e0f6f04656804e2dc479962fcf0` |
| Direct whole-buffer placeholder census | **PASS**: 14 Rust occurrences = one shared definition plus 13 remaining callers; Draw subtree zero |
| Forbidden Draw production scan | **PASS**: no production initializer, partial-owner `?`, reverse `rposition`, post-admission duplicate-name reserve, fabricated exact helper, structural-copy counter, or asset `iter().nth` |
| Scoped working/staged/HEAD diff checks for Draw owned/editor/verifier | **PASS** |
| Whole working diff check | **PASS at audit time** |
| Whole staged/HEAD diff checks | **RED outside Draw**: blank EOF lines in two staged P3 raster reports and trailing whitespace in `🐙️ueli.md:459` |
| Cargo/Nx/native/Wasm/browser/network/runtime | **Not run by instruction** |

## Final disposition

The governed bootstrap and the previously accepted exact rollback/pool-handback behavior satisfy
this source cohort. Draw may be counted as independently **ACCEPTED for this source packet**. This
does not admit any interactive command, clear any of the 18 global failure classes, or establish
compile, allocator, timing, native, Wasm, or browser behavior. Phase 8 therefore remains **RED at
0/884 with runtime unverified**.
