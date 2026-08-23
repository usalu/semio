# Sol Independent P8 Draw Process-Arena and Duplicate-Name Repair Audit — 2026-08-23

## Verdict

**REJECT — source cohort only.** The two prior coordinator blockers are materially repaired on the
normal path: Draw now borrows one of four process-retained, generation-tagged arena bundles without
allocating a per-candidate arena, and duplicate names use a pre-admitted page without
`base.name.try_reserve_exact`. The repair is not terminally lossless. A cancel or stale fault after
a container rebuild has started retires the original live container and both pooled scratch vectors
instead of restoring and returning them. The process-pool bootstrap also ordinary-drops partially
allocated owners on any fallible construction or final admission failure.

This was an independent Sol High source audit. I did not author the repair and made no production
edit. Cargo, Nx, native, Wasm, browser, network, root lint, allocator/runtime timing, and the Rust
test binary were not run. Phase 8 therefore remains **RED at 0/884 admitted commands, 18 global
failure classes, and runtime unverified**. The independently accepted structural census is not
decremented: the raw repository census remains one shared fail-closed definition plus thirteen live
callers, with Draw at zero.

## Blocking Findings

### 1. Mid-rebuild cancel/stale destroys the last-valid container and strands the pooled slot

The success path has the intended shape. `start_rebuild` moves the live container's exact `Vec`
backing into `DrawContainerRebuildAuthority`, takes the pool's reverse and output vectors, and
`finish_rebuild` restores the rebuilt source vector to the live container before returning both
empty scratch vectors to the candidate (`owned/component.rs:3554-3584`). The candidate's retained
pool-return cursor then requires those exact scratch roots and returns reverse, output, page
catalog, and duplicate-ID owners one at a time (`:3477-3533`).

The cancellation/close path does not preserve that contract. `DrawContainerRebuildAuthority::close_step`
detaches `source`, then `reverse`, then `output` into `DrawOwnedRetirement` (`:1866-1893`). The
`source` root is the original last-valid live container backing moved by `std::mem::take`; the other
two roots are the process pool's exact scratch vectors. The live `DrawSnapshot` container remains
the empty replacement installed by `mem::take`. After rebuild retirement reaches terminal, the
candidate drops the rebuild authority (`:4000-4008`) and calls `return_arena_owner` (`:4044-4049`).
That return necessarily faults at the missing reverse owner (`:3490`) or missing output owner
(`:3497`), so the candidate cannot reach `terminal_is_empty`, its generation-tagged pool slot stays
leased, and capacity permanently shrinks.

The named cancellation fixture does not exercise the defect. It stops as soon as
`authority.phase == RebuildSource` or `RebuildDestination`, before the next step creates the rebuild
authority and detaches any owner (`:5467-5525`). It also asserts only `source.id`; it never checks
the layer container contents/backing or the pool roots. The pool fixture bypasses public candidate
close and directly invokes `return_arena_owner` four times (`:4984-5040`), so it cannot detect the
active-rebuild close failure. Its alleged ABA assertion only observes that the next generation is
larger; it does not inject a stale return token.

This violates exact rejected-owner cleanup, truthful Pending/Blocked close behavior, original live
`Vec` backing restoration, cancellation/stale last-valid preservation, and four-root terminal pool
handback.

### 2. Pool bootstrap failure ordinary-drops partially constructed fixed owners

`DrawMutationArenaOwner::try_new` builds two vectors, the page-catalog vector, sixteen independently
reserved String pages, and the duplicate-ID String through a sequence of fallible
`try_reserve_exact` calls (`owned/component.rs:683-699`). `DrawMutationArenaPool::try_new` then builds
four owners with `DrawMutationArenaOwner::try_new()?`, accumulates actual capacities, and can still
reject the completed set at the final aggregate comparison (`:771-790`).

Any failure after the first successful allocation ordinary-drops the local partial owner. A later
bundle failure or final aggregate rejection ordinary-drops every already-completed bundle in the
local `[Option<DrawMutationArenaOwner>; 4]`. There is no retained bootstrap authority, rejected-owner
cursor, cancellation/progress handoff, or terminal witness for these roots. `DrawPlayApp::default`
invokes initialization but discards the result (`editor/component.rs:151-155`), so this cleanup
failure is not even surfaced as an owned boot fault.

Creating the pool before candidate admission correctly removes per-request arena allocation from
the normal path. It does not authorize an all-at-once deep destructor on bootstrap rejection, and it
does not satisfy the requested exact rejected-owner cleanup.

## Verified Positive Source Evidence

| Requirement | Independent result | Evidence |
| --- | --- | --- |
| Fixed process authority | **PASS, normal path** | `DRAW_MUTATION_ARENA_POOL_CAPACITY = 4`; fixed slot array and `OnceLock` process owner (`owned/component.rs:670-769`). |
| Actual capacity accounting | **PASS, normal path** | Each owner totals allocator-returned vector/page/String capacities, and the pool checked-adds all four bundles before publication (`:701-710`, `:771-790`). |
| Nonblocking generation-tagged borrow | **PASS** | `try_lock`, fixed `position` over four slots, checked generation increment, and exact owner take (`:799-813`). |
| Candidate does not allocate a replacement arena | **PASS narrowly** | Candidate construction only receives the borrowed arena owner and moves its four roots into retained fields (`:3381-3422`). |
| Four-turn exact pool handback | **PASS on success/direct fixture path** | Reverse, output, page catalog, then duplicate ID; the slot remains leased until phase four and contention maps to `Blocked` (`:3477-3533`, `:4044-4049`). |
| Duplicate hash framing | **PASS** | Domain, independent ID length/bytes, and independent name length/bytes are incrementally hashed (`:3155-3190`). |
| Duplicate name staging | **PASS, normal/rejection path** | Fixed pending ID/name owners, destination and scratch capacity preflight, separate ID/name copy and suffix stages, and no live name reserve (`:3074-3254`). |
| Previous fixed-owner seams | **PASS structurally** | Fixed-depth traversal, fixed asset range cursor, fixed digests, moved shared history IDs, all seven layer variants, fourteen mutations, and no fabricated `exact_for_test`/`structural_copies` seam remain present. |

These positives do not override the two terminal ownership failures.

## Evidence and Gate Results

| Gate | Result |
| --- | --- |
| Read repair report and recent independent Draw rejection/re-audits | **PASS** |
| Edition-2021 `rustfmt --check --config skip_children=true` on Draw owned/editor and shared store | **PASS** |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | **PASS: 271 source-verifier self-tests clean** |
| `bun ./📜️script.ts verify interactivity` | **PASS: broad DENY clean; one recorded test-only bridge and two predeclared future entries** |
| Full tool-job verifier, independent run A/B | Expected global **RED**: 50 hosts, 50 invocations, 775 rows, 773 unique, 0 bounded, 884 remaining, 8 reserved, 35 importer owners, 34 globals, 18 failures, 271 self-tests |
| Independent ledgers | **PASS deterministic**: two 312,305-byte files, byte-identical, SHA-256 `bac5c05663843af9f70290b449192e6844b59cd8c836b4408761e09ec7e3861f` |
| Direct placeholder census | **PASS**: 14 Rust occurrences repository-wide; Draw subtree zero |
| Forbidden duplicate/allocation scans | **PASS narrowly**: zero `base.name.try_reserve_exact`, asset `iter().nth`, fabricated `exact_for_test`, or `structural_copies` in the owned Draw file |
| Discriminating source probe | **REJECT**: proves active rebuild close takes source/reverse/output before candidate's required pool return; proves fallible partial pool-owner construction has no retained cleanup |
| Scoped and whole working/staged/HEAD `git diff --check` | **PASS** |
| Cargo, Nx, native, Wasm, browser, network, root lint, allocator/runtime timing | **Not run; RED/unverified by instruction** |

Independent verifier ledgers are:

- `📊️sol-independent-p8-draw-process-arena-audit-a.json`
- `📊️sol-independent-p8-draw-process-arena-audit-b.json`

The 271 self-tests are structural/source tests, not an executed Rust test binary. The thirteen new
Draw mutations verify pool names, direct four-root return markers, no name reserve, and fixture
presence. They do not cancel after an active rebuild has moved an owner and do not inject a partial
pool-bootstrap allocation failure.

## Exact Two-Blocker Repair Packet

1. **Make rebuild cancellation an exact rollback/handoff authority.** Once `start_rebuild` moves the
   live container into the rebuild job, close/cancel/stale must retain that exact root and restore
   its original contents and backing one layer per grant. It must return the exact reverse and
   output pool vectors to the candidate, not retire them. Pending/removed layers must either be
   restored to their precise FIFO position or retired through a separately witnessed owner after
   last-valid restoration. Only then may candidate close advance its four pool-return phases.
   Add executable fixtures that cancel/stale after a nonzero source pop, pending insert/removal,
   output move, reverse move, and destination rebuild; assert exact layer order, source `Vec::as_ptr`,
   every scratch/page/catalog pointer, truthful Pending/Blocked, terminal-empty, and immediate slot
   reuse. Add a verifier mutation that replaces rollback/return with the current retirement path.
2. **Cursorize pool bootstrap and its rejection owners.** Build the four bundles through a retained
   app/process bootstrap job that performs at most one vector/page/String owner allocation or
   retirement per governed grant, records exact allocator-returned capacities before publication,
   and on any allocation/cap/cancel fault retains all partial owners until cursorized terminal
   cleanup. Surface the terminal initialization fault from `DrawPlayApp` instead of discarding it.
   Add injected-failure fixtures after each vector, page catalog, page String, duplicate String,
   bundle, and final aggregate boundary; assert exact owner counts/bytes, one root per close grant,
   terminal-empty, and no operation admission while bootstrap is incomplete/faulted. Add mutations
   that restore `?`-driven partial ordinary Drop or ignored initialization failure.

Until both repairs pass an independent source audit and the serialized build/runtime gates execute,
Draw remains **REJECTED** and Phase 8 remains **RED: 0/884, 18 failure classes, runtime unverified**.
