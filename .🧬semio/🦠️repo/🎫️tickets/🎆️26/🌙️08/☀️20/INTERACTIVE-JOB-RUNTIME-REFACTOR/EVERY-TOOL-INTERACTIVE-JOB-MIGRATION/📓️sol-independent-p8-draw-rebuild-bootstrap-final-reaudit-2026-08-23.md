# Sol Independent P8 Draw Rebuild and Bootstrap Final Re-Audit — 2026-08-23

## Verdict

**REJECT — Draw source cohort only.** The move-log rollback repair is materially correct: every
container move has an inverse, cancellation and stale authority restore the original live `Vec`
backing, reorder retains its removed owner until source reinsertion, and the process slot stays
leased until reverse, output, page catalog, and duplicate-ID owners return in four distinct
handoffs. The bootstrap repair does not meet the required governed-before-allocation boundary.
Its public pump has no budget or admission argument, and `DrawPlayApp::default` advances it outside
the store job. Two consecutive app constructions therefore create the retained builder and then
execute the first `try_reserve_exact` without a `StepContext` grant. The permanent verifier also
accepts two behavior-destroying mutations when reconstructed against the real sources.

This was an independent Sol High source audit. I did not author the repair and made no production
edit. Cargo, Nx, native, Wasm, browser, runtime, network, allocator timing, and the Rust test binary
were not run. Phase 8 remains **RED at 0/884 admitted commands, 18 global failure classes, and
runtime unverified**.

## Blocking Findings

### 1. Process bootstrap allocation remains reachable outside a governed turn

`DrawMutationArenaPoolBootstrap::step` takes only `&mut self`; it has no `StepContext`, deadline,
fuel, item credit, byte credit, or pre-admitted allocation claim (`owned/component.rs:965`). When
the active builder is present, one call reaches its phase-specific `try_reserve_exact` allocation
(`:983`, builder allocations at `:756-803`). The aggregate `maximum_items` and `maximum_bytes`
values are passive counters: actual allocator capacities are accumulated after each completed
bundle and compared only after all four bundles have already been built (`:972-1018`). They are not
a scheduler/resource claim consumed before allocation.

The intended store route does invoke this pump from
`DrawStoreInitializationAuthority::InitializeArena` and charges one fuel afterward
(`:4870-4878`). That call-site convention is not exclusive. `initialize_draw_mutation_arena_pool`
is public and independently advances the same state (`:1095-1141`). `DrawPlayApp::default` invokes
it without any job context (`editor/component.rs:159-166`). The first app construction installs the
builder; the second app construction advances builder phase zero and reserves the reverse vector.
`borrow_draw_mutation_arena` is another zero-budget caller that advances the pump before reporting
bootstrap pending (`owned/component.rs:1164-1167`).

Consequently the source proves only fixed-size, one-allocation-per-call construction, not fixed
governed claims before allocation. The retained error topology is otherwise substantially
improved: successful partial roots live in the owner builder or fixed bootstrap array, and
`Building -> Retiring -> Fault` releases one root per call. The owner types still use
`ManuallyDrop` plus terminal assertions (`:866-869`, `:1057-1060`), and an app-started bootstrap can
remain parked without a mandatory close turn.

### 2. The reported mutation matrix is not discriminating on the live sources

The source-verifier self-test reports 285 clean tests, but its repair mutations operate on compact
synthetic strings where each marker occurs once. I extracted the real
`toolJobDrawEnvelopeCallerRetainedExact` function, loaded the real store/Draw/editor/plugin files,
and replayed the repair-facing mutations in memory.

The current repair block contains fifteen observable checks rather than the report's stated
fourteen. Thirteen reject against the live sources. Two behavior-destroying targeted mutations are
falsely accepted:

1. Replacing the actual source-undo call
   `self.start_rebuild(..., DrawContainerRebuildRole::CloseSourceUndo)?` with role `Destination`
   is accepted. Other enum/match occurrences preserve the verifier token. Close then rolls back
   the reinsertion rebuild rather than driving it forward, so the removed reorder owner is never
   restored.
2. Replacing the governed store-phase call `match initialize_draw_mutation_arena_pool()` with an
   immediate typed `Ok(true)` is accepted. The enum token, fixtures, and ungoverned editor call
   remain, so the predicate cannot prove that the store job advances bootstrap.

The first-occurrence mutations at `script.ts:3506` and `:3511` reject their synthetic fixture but
do not distinguish these live changes. The requested fourteen meaningful mutations are therefore
not independently established.

## Verified Positive Evidence

| Requirement | Result | Evidence |
| --- | --- | --- |
| Fixed inverse move catalog | **PASS** | 257 entries cover source/reverse/output/pending/removed moves (`owned/component.rs:2093-2140`, `:2283-2327`). |
| Pre-detach validation and exact rejection | **PASS** | Reservation, destination, capacities, and both empty scratch owners are checked before `mem::take`; rejection restores all owners (`:4033-4062`, `:2142-2180`). |
| Exact inverse rollback | **PASS** | Reverse replay moves one layer per call, clears one log entry, and restores the original source before handoff (`:2283-2327`, `:4468-4519`). |
| Destination insertion | **PASS** | `PendingToOutput` inverses to the exact pending owner; active destination rollback precedes source undo (`:2298-2305`, `:4559-4567`). |
| Reorder source removal | **PASS** | `ReverseToRemoved` returns the exact node, then `CloseSourceUndo` reinserts at recorded parent/index (`:2306-2310`, `:4563-4567`). |
| Pointer/FIFO fixtures | **PASS structurally** | Phase 0-3 cancel/stale and reorder interruption compare source pointer, FIFO order, scratch/page/ID pointers, slot reuse, and generation (`:5816-5945`). |
| Four-root terminal handback | **PASS** | Reverse, output, page catalog, and duplicate ID return in four phases; `leased` clears only at phase four (`:3933-4013`). |
| Contention and ABA | **PASS structurally** | `try_lock`, checked generation increment, and exact return-generation validation are present (`:1152-1161`, `:3933-3943`). Existing fixture checks generation advancement but does not inject an old return token. |
| Retained partial construction | **PASS narrowly** | Allocation, bundle, arithmetic, and aggregate faults retain all successful partial roots for cursor close (`:709-870`, `:915-1060`). |
| Editor-visible terminal fault | **PASS narrowly** | Editor caches immediate terminal error and observes process-terminal `Fault`; Building/Retiring is not observable as a fault (`editor/component.rs:151-166`, owned `:1143-1150`). |

## Gates

| Gate | Result |
| --- | --- |
| Edition-2021 parser and scoped rustfmt check on Draw owned/editor | **PASS** |
| Tool-job verifier self-test | **PASS: 285 clean** |
| Independent live-source mutation replay | **REJECT: 13/15 reject; source-undo and governed-pump bypasses accepted** |
| Draw baseline predicate | **PASS mechanically** |
| Broad `verify interactivity` | **PASS: DENY clean; one test-only bridge and two predeclared future entries** |
| Full verifier A/B | Expected global **RED** both times: 50 hosts, 50 invocations, 775 rows, 773 unique, 0 bounded, 884 remaining, 8 reserved, 35 importers, 35 globals, 18 failures, 285 self-tests |
| Deterministic ledgers | **PASS**: both 312,560 bytes, SHA-256 `bc1a768b0f2d4f00848bf9e5fe141d4f4be55801b208e4b344cda8b25a1efad4` |
| Scoped Draw owned/editor/verifier working/staged/HEAD diff checks | **PASS** |
| Whole working diff | **RED outside Draw**: existing trailing whitespace in `.🧬semio/🦠️repo/💬️prompts/🐙️ueli.md:459` |
| Whole staged/HEAD diffs | **RED outside Draw**: existing blank EOF in staged P3 raster audit; HEAD also includes prompt whitespace |
| Builds/runtime/network | **Not run by instruction** |

The Rust fixtures were inspected but not executed because builds were prohibited.

## Exact Repair Packet

1. Make bootstrap advancement a required typed maintenance/job step with operation, generation,
   cancellation, deadline, fuel, item credit, and byte credit. Atomically claim the fixed pool
   maximum before the first allocation. Remove advancement from `DrawPlayApp::default` and
   `borrow_draw_mutation_arena`; those APIs may only observe Ready/Fault and return Pending/Blocked.
2. Give Building/Retiring a mandatory app/store maintenance owner through terminal Ready/Fault.
   Prove repeated app construction cannot allocate, parked bootstrap resumes, cancellation closes
   one root per grant, and no unexpected Drop relies on assertion-only `ManuallyDrop` leakage.
3. Mutate live behavior blocks, not first marker occurrences. Reject bypass of the actual
   `InitializeArena` pump and changes to the exact source-undo start role while other tokens remain.
   Add a real stale old-generation return mutation/fixture.

Until these repairs pass independent audit and serialized build/runtime gates, Draw remains
**REJECTED** and Phase 8 remains **RED: 0/884, 18 failure classes, runtime unverified**.
