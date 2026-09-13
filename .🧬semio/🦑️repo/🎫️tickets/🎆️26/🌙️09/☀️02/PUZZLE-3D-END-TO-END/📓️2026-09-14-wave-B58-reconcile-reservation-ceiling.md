# Wave B58 — pricing the reconcile reservation by the body it reconciles

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave B58, 2026-09-13/14. Written incrementally.

Predecessor: `📓️2026-09-14-wave-B56-window-alias-retirement.md` §1.1 and §6 — the ceiling, named and
instrumented but not fixed. Also B2 (page-priced retirement grants), B8/B52 (retirement termination),
B48 (grant release), B54 §8.2 (the blocking defect as first measured).

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, shared live tree. No state-modifying git command,
  no worktree, no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, every command foreground. This wave does NOT close
  or reopen the ticket and deletes nothing under `🗑️generated`.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- `RUST_MIN_STACK=134217728` on every law run.
- Peers are live in the plugin `🦀️.rs` files and in the taxonomy; every file was re-read before each
  edit and no peer hunk was reverted.

## 1 The sizing model, with numbers

### 1.1 What one reservation cost, and what the body costs

| quantity | site | value |
| --- | --- | --- |
| per-surface ceiling | `UI_RESIDENT_SURFACE_BYTES`, `🖱️ui/🧬️contract/🎟️resident/🦀️.rs:11` | 8 MiB |
| aggregate | `UI_RESIDENT_AGGREGATE_BYTES`, `:12` | 32 MiB |
| contract fixed backing | `CONTRACT_BACKING_BYTES` + runtime backing | ≈ 0.68 MiB (B56 measured `709 920 B`) |
| what ONE reconcile reservation asked for | `SurfaceReconcileReservation::try_new`, `♻️reconcile/🦀️.rs:2315-2316` | `SurfaceReconcileLimits::default()` → items 4 097, **bytes 8 MiB** |
| concurrent reservations admitted | arithmetic | **3** |
| surfaces a puzzle3d session mounts | B56 §1.3 live census | **13** |


### 1.2 The RED, reproduced natively before any fix

New law `thirteen_mounted_surfaces_at_their_real_sizes_all_hold_a_reconcile_reservation_at_once`
(`…/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs`) mounts the session B56's live census names: three
panes carrying a 54 KB world body (rows of 480-byte `UiText` under groups of thirty, because
`TreeNode.children` holds 32 and the document holds 128 nodes) and ten panels of 1–8 KB, each reserved and
committed in ONE turn set. Against the unfixed tree:

```
running 1 test
test component::reactor::patches::tests::thirteen_mounted_surfaces_at_their_real_sizes_all_hold_a_reconcile_reservation_at_once ... FAILED

panicked at …/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs:1444:33:
surface 3 of a thirteen-surface session must hold a reservation priced by its own 1024-byte body:
 slots=[58:puzzle3d-main#g1:P--:ack0/rev0:outSome(0), 58:puzzle3d-main-top#g2:P--:ack0/rev0:outSome(1),
        58:puzzle3d-main-perspective#g3:P--:ack0/rev0:outSome(2)]
 ready=[g1:--r-,g2:--r-,g3:--r-] deferred=[] rejected=0 unadmitted=0 closing=0 output_fault=none
 reserve_refusal=unmounted:registry-resident-credit-exhausted
 registry=resident=3s/12291i/25875744B of 33554432B handback=378/384
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 696 filtered out; finished in 0.01s
```

**The FOURTH surface is a 1 KB panel and it is refused**, with the ledger holding 3 slots / 25 875 744 B of
33 554 432 B — three 8 MiB reservations on top of 709 920 B of fixed backing. A fourth would be
34 253 216 B, over the aggregate by 698 784 B. The refusal has nothing to do with the body: it is the
fourth asker, exactly as B56 §1.1 predicted, and the handback registry still has 378 of 384 slots free.

### 1.3 The sizing model after the fix, measured

| quantity | value (measured by the laws) |
| --- | --- |
| `SURFACE_RECONCILE_FLOOR_BYTES` — what a reservation costs before its tree is measured | **132 978 B** = `UiDocumentAssembly::required_open_bytes()` (1 906 B) + 4 × `SURFACE_RECONCILE_PAGE_BYTES` (131 072 B) |
| `SURFACE_RECONCILE_FLOOR_ITEMS` | 256 |
| per-surface MAXIMUM (unchanged) | `SurfaceReconcileLimits::default().max_bytes` = 8 388 608 B |
| ceiling-sized bodies the aggregate still admits | **3** — unchanged, and now only for bodies that genuinely fill the maximum |
| 13 mounted surfaces at reservation time | 13 slots, **1 728 714 B** total, mean 132 978 B — **5.2 %** of the 33 554 432 B aggregate |
| PEAK while all 13 reconcile AND publish in one turn set | **23 836 176 B**, mean 1 833 552 B per surface — 71 % of the aggregate, 9.7 MB spare |
| what 13 ceiling-priced reservations would have cost | 109 051 904 B — **3.25 × the whole aggregate** |

The reservation is 63 × smaller than before at admission time, and the credit a surface ends up holding is
the credit its own body needs. The per-surface maximum is untouched: `SurfaceReconcileLimits::default()`
is still the bound the job enforces on itself (`usage.fits(limits)`), so a body that genuinely fills 8 MiB
still reaches 8 MiB and still faults `Credits` past it.

## 2 Root cause, file:line

`SurfaceReconcileReservation::try_new` — `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs:2385`:

```rust
let limits = SurfaceReconcileLimits::default();
let credit = reserve_surface_reconcile(limits).ok_or(SurfaceReconcileReservationRefusal::ResidentCredit)?;
```

and `reserve_surface_reconcile` (`:2284`), which passed those limits straight through:

```rust
ui_contract::UiResidentPermit::try_reserve(ui_contract::UiResidentLimits { items: limits.max_items, bytes: limits.max_bytes }, …)
```

`SurfaceReconcileLimits::default().max_bytes` **is** `UI_RESIDENT_SURFACE_BYTES` — the per-surface CEILING
(`…/🧬️contract/🎟️resident/🦀️.rs:11`), a quarter of `UI_RESIDENT_AGGREGATE_BYTES` (`:12`). So the price of
reconciling a 1 KB panel equalled the price of reconciling the largest body the contract will ever allow, and
`PatchTracker::reserve_mounted_owned` (`…/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:475`) refused the fourth asker
whatever it was.

The truing-up already existed and ran in the wrong place: `SurfaceCanonicalCandidate::seal_step`
(`♻️reconcile/🦀️.rs:377`) shrinks the permit to `usage.bytes + fixed + allocated` — **after** the whole
reconcile, i.e. after the 8 MiB has been held for the entire job. That is B56 §6 candidate 1's
"`try_shrink` runs AFTER the reservation, which is exactly too late", confirmed at its call site.

## 3 The fix — reserve a floor, climb by what is actually used, fault by name

### 3.1 One ledger primitive instead of a one-way shrink

`UiResidentPermit::try_shrink` → **`UiResidentPermit::try_reprice`** (`…/🎟️resident/🦀️.rs:224`). One live
reservation moves to new limits in either direction under the exact-ownership check it always had; an
increase the aggregate cannot take is `UiResidentFault::Capacity` — the same NAMED refusal a first
reservation answers — and the per-surface ceiling is still validated. The one-way `InvalidLimits` guard that
made growth impossible is gone; nothing else about the ledger changed (no new field, no new static).

`UiDocumentAssembly::shrink_resident` → **`reprice_resident`** (`…/📃️document/🎟️assembly/🦀️.rs:238`), plus
**`resident_limits`** (`:226`) so a caller can see what it holds before repricing. Both callers of the old
name were updated: the seal path and the `🌳️root` law. The `minimum` guard (never below the slot's own
allocation, never fewer items than nodes) and the refusal after an output split are unchanged — that is what
keeps `retained_document_root_permit_seal_transfers_output_without_detaching_root_credit`'s
"shrink-after-split must fail" clause meaningful.

### 3.2 The reconcile reserves a floor and climbs

`♻️reconcile/🦀️.rs`:

- **`SURFACE_RECONCILE_FLOOR_BYTES` / `SURFACE_RECONCILE_FLOOR_ITEMS`** (`:2279`) — the assembly root plus
  four work pages, with `const _: () = assert!(…)` pinning both under the per-surface contract. Four pages is
  more than any single step can charge: `step` already faults `PageBytes` on a `Yield` wider than one
  `SURFACE_RECONCILE_PAGE_BYTES`, so the floor is a genuine reserve-BEFORE-use headroom rather than a guess.
- **`reserve_surface_reconcile`** (`:2284`) reserves that floor (clamped by the caller's own limits, so a
  law that passes deliberately tiny limits still behaves exactly as before).
- **`SurfaceReconcileCursor::admit_credit`** (`:1866`) — the climb. It refuses anything that does not fit
  the per-surface limits (as `Credits`, the existing name), then grows the credit to
  `usage + floor` of headroom, clamped at the per-surface maximum, through `try_reprice` while the permit is
  still the cursor's own (`assembly_credit`) and through `reprice_resident` once the permit has moved into
  the open `UiDocumentAssembly`. A **contended** ledger is not a fault — the floor headroom covers the step
  and the next step retries; an **exhausted** aggregate is the new terminal fault
  `SurfaceReconcileFault::ResidentCredit { required, aggregate }` (`:621`), which
  `PatchTracker::take_render_fault` already reports verbatim to the shell.
- Two call sites, which is every place `usage` grows: the node-commit projection in
  `TraversePresentation` (`:1479`, before the flat node is pushed — the one charge that can exceed a page in
  a single go) and the tail of `SurfaceReconcileCursor::step` (`:1847`, immediately after the `Yield`
  accounting that every other charge funnels through).

### 3.3 A registry refusal is reported by name instead of deferring in silence

`…/🩹️patches/🦀️.rs`: `PatchTrackerState::reserve_refusal` carries a `reported` flag (the shape
`output_fault` already uses), and **`PatchTracker::take_unreported_reserve_refusal`** (`:522`) answers the
reason once while leaving the census field intact for `debug_state`.

`…/🔄️turn/🦀️.rs:1192`: the `Err(surface)` arm still defers — a deferred render is right for the ordinary
"previous reconcile still in flight" race — but a refusal from one of the PROCESS-WIDE tables
(`registry-…`) now also raises `ui.surface-reconcile-unadmitted` naming the surface and the table. That is
the half B52's stall cap could not supply: the loop it caps is now audible from its first turn.

## 4 Laws and outputs — every command foreground, tails quoted

### 4.1 The new law, RED then GREEN

`thirteen_mounted_surfaces_at_their_real_sizes_all_hold_a_reconcile_reservation_at_once`
(`…/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs`). RED is §1.2. GREEN, with the sizing model printed:

```
RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- reconcile_budget patches:: \
  surface_context two_hundred_publications the_refused_reconcile every_mounted_surface \
  thirteen_mounted_surfaces --test-threads=1 --nocapture

test …patches::tests::thirteen_mounted_surfaces_at_their_real_sizes_all_hold_a_reconcile_reservation_at_once
  … [DEBUG] 13 surfaces admitted at floor=132978B each; reserved=1728714B for 13 slots (mean 132978B);
    peak while reconciling and publishing all of them=23836176B (mean 1833552B per surface)
    of 33554432B aggregate against a 8388608B per-surface ceiling; baseline=709920B
test …patches::tests::the_refused_reconcile_reservation_names_the_resident_credit_ledger_not_the_handback_registry
  … [DEBUG] ceiling-sized credit ceiling=3 surfaces; refusal=… output_fault=none
    reserve_refusal=unmounted:registry-resident-credit-exhausted
    registry=resident=4s/4i/33421455B of 33554432B handback=384/384 …
test …patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner
  … [DEBUG] mounted-resident-capacity fixed=709920 floor=132978 ceiling=8388608
    ceiling-sized-accepted=3 full=33421455 cap-plus-one=false exact-refusal=true restored=709920
test …patches::tests::two_hundred_publications_leave_the_reconcile_registries_holding_nothing
  … [DEBUG] 200 publications returned every reservation: SurfaceReconcileRegistryCensus {
    resident_items: 0, resident_bytes: 709920, resident_slots: 0, resident_aggregate_bytes: 33554432,
    handback_free: 384, handback_slots: 384 }

test result: ok. 54 passed; 0 failed; 0 ignored; 0 measured; 643 filtered out; finished in 1.70s
```

**54 of 54**, the whole prescribed filter, including every B2/B8/B48/B52/B56 law it covers.

The law does not merely reserve thirteen surfaces: it commits a rendered body to each, then drives the
tracker with all thirteen jobs live, sampling the ledger every step, and requires all thirteen to PUBLISH
within one turn set while the peak stays under thirteen per-surface ceilings. `reserve_refusal=none` is
asserted on the census string itself.

### 4.2 Two laws adjusted, and why that is not a weakened gate

| law | before | after |
| --- | --- | --- |
| `the_refused_reconcile_reservation_names_the_resident_credit_ledger_not_the_handback_registry` (B56) | reserved surfaces through the tracker until the fourth was refused | holds real ledger credit until the aggregate has **less than one floor price** left (`fill_resident_aggregate`), then requires the tracker refusal to name `registry-resident-credit-exhausted` with the handback registry still free |
| `mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner` | asserted the snapshot after one reservation equals `{items: 4097, bytes: fixed + 8 MiB, used_slots: 1}` | asserts it equals `{items: 256, bytes: fixed + 132 978, used_slots: 1}` — the FLOOR — and reaches cap + 1 the same way, keeping "the exact tree comes back" and "a render cannot materialize before a fixed slot exists" verbatim |

Both keep their subject and their fixture (`residentCapacity.reservationBytes` is still asserted equal to
`limits.max_bytes`, because 8 MiB is still the per-surface maximum — it is no longer the price). Nothing was
deleted, and both now pin the SIZING as well as the refusal.

### 4.3 The wider reactor suite: 7 failures → 4, against a neutralised baseline

The change was neutralised in place (floor → `limits.max_*`, `admit_credit` short-circuited, both marked
`[DEBUG] B58 neutralised baseline` and removed afterwards) and the same filter run:

```
cargo test -p semio-framework-plugin --lib -- reactor --skip a_settled_reactor_turn_retains_nothing --test-threads=1
BASELINE: test result: FAILED. 159 passed; 7 failed …
AFTER:    test result: FAILED. 162 passed; 4 failed …
```

The four remaining failures are identical in both runs and are **pre-existing and unrelated** — an envelope
decode worker poll, an async actor poll, a parked-task cancel, and an M1/M2 revision-guard tolerance. The
three that the fix clears are this wave's two laws plus
`turn::turn_execution_tests::guest_turn_execution_resets_for_every_turn`, whose baseline failure is a
wall-clock artifact (`a settled turn keeps exactly the microseconds it executed — left: Some(2), right:
Some(1)`), so it is recorded as **flaky under load, not as a fix**.

`reactor` as a whole cannot be run unfiltered: `plugin_builder_contract_tests::a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford`
aborts the PROCESS in `NativeLifecycleRegistry::drop` (`…/⚛️reactor/🚪️lifetime/🦀️.rs:489`, a panic in a
destructor → SIGABRT) when earlier tests in the same binary have polluted the process-global registries. It
**passes alone** (`[DEBUG] settled reactor turn retention: settled=29951769 after=29951753 per_turn=0 B`,
`1 passed`), so it is cross-test pollution, not this wave — recorded as residual 3.

### 4.4 `semio-framework-ui-runtime`: 75 passed, 2 pre-existing failures, names identical

```
RUST_MIN_STACK=134217728 cargo test -p semio-framework-ui-runtime --lib -- reconcile ownership surface --test-threads=1
AFTER:    test result: FAILED. 75 passed; 2 failed; 2 ignored …
BASELINE: test result: FAILED. 75 passed; 2 failed; 2 ignored …
diff of failure names → IDENTICAL RUNTIME FAILURE NAMES
```

Both are pre-existing: `reconcile::tests::canonical_document_tests::surface_canonical_document_completion_transfers_do_not_borrow_the_child_grant`
and `reconcile::tree_retirement::tests::runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads`.

### 4.5 `semio-framework-ui-contract`: 29 pre-existing failures, names identical

```
RUST_MIN_STACK=134217728 cargo test -p semio-framework-ui-contract --lib -- resident document --test-threads=1
AFTER:    test result: FAILED. 11 passed; 29 failed …
BASELINE: test result: FAILED. 11 passed; 29 failed …
diff of failure names → IDENTICAL FAILURE NAMES
```

The `document` and `resident` families of this crate are **already red at HEAD** (`ArenaFull` cascades
behind two root failures at `🎟️assembly/🧪️tests/🎟️assembly/🦀️.rs:43` and `:13`, which fail alone and
assert on close-grant bytes, a path this wave does not touch). Recorded as residual 2 — the rename of
`try_shrink`/`shrink_resident` did not change a single failure name.

### 4.6 puzzle3d and the wasm target

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
warning: `semio-s-artifact-puzzle-3d` (lib) generated 95 warnings …
    Finished `dev` profile [unoptimized] target(s) in 42.85s     → 0 errors, 126 warning lines
```

```
cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
    Checking semio-s-plugin-puzzle v0.1.0 (…/🧩️puzzle/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 45.99s     → 0 errors
```

Both guest halves really were type-checked for the wasm target after the edits: under
`⚡️cache/cargo/build/wasm32-wasip2` both `libsemio_framework_ui_runtime-*.rmeta` and
`libsemio_framework_plugin-*.rmeta` are newer than `♻️reconcile/🦀️.rs` and `🔄️turn/🦀️.rs` respectively
(`find … -newer`). The wasm BUILD was not run — the coordinator builds #62.

A peer's in-flight `semio-framework-deflate` cursor refactor broke `semio-framework-pack`
(`no method named close`, then `unresolved module semio_framework_deflate`) for about ten minutes in the
middle of this wave, which blocks every dependent check. Polled, not worked around; it cleared on its own.

## 5 What rides wasm #62

**Everything.** This wave has NO host half — every edit is Rust compiled into the guest:

| file | what |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️.rs` | `try_reprice` (bidirectional, aggregate-checked) |
| `…/🧬️contract/📃️document/🎟️assembly/🦀️.rs` | `reprice_resident`, `resident_limits` |
| `…/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs` | floor constants, floor pricing, `admit_credit`, `ResidentCredit` fault, seal repricing |
| `…/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs` | refusal `reported` flag, `take_unreported_reserve_refusal` |
| `…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `ui.surface-reconcile-unadmitted` named fault on a registry refusal |
| `…/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` | the new law, two adjusted laws, `sized_body` / `fill_resident_aggregate` / `release_resident_aggregate` |
| `…/🎟️resident/🧪️tests/…`, `…/🎟️resident/🌳️root/🧪️tests/…` | renamed law call sites |

Recipe unchanged from B54 §8.3 / B56 §5: `bun nx run @semio-tech/puzzle-plugin:component-dev` then
`:materialize-dev` with `CARGO_PROFILE_WASM_DEV_DEBUG=false`, then a census probe on `:6013`.

**What #62 must show**, stated before the fact:

1. `registry=resident=13s/…B of 33554432B` — THIRTEEN resident slots for one puzzle3d session (twelve now
   that B56 retired the `1:window` alias), not three.
2. `reserve_refusal=none` in the reactor census, and no `ui.surface-reconcile-unadmitted` fault on the
   shell's fault surface.
3. After a proven pick on Nakagin, `deleteSelection` **lands**: object census 180 → 179, `settled=1`, and
   the `more-work` streak with `sources=["reconcile"]` and `effects=0` gone. B56's tape for the same
   sequence was `deleteSelection landed=false before=180 after=180 waitedMs=135319 … moreWork=1426`.
4. The peak occupancy should read around 24 MB of 33.5 MB while all thirteen surfaces reconcile — which is
   residual 1.

No live browser measurement was taken in this wave, and deliberately so: with zero host-side edits a probe
on `:6013` at wasm #61 can only re-measure B56 §1.3.

## 6 Residuals

1. **A sealed surface document costs ~1.83 MB whatever its body size, so ~17 surfaces fill the aggregate.**
   The law measures a 23 836 176 B peak for thirteen surfaces — 71 % of 33 554 432 B — although their bodies
   are 54 KB and 1–8 KB. Two contributors, both pre-existing and both left alone here:
   (a) `seal_step` prices `usage.bytes + fixed + allocated` where `fixed` includes a 128-slot
   `UiNodeRecord` table (`size_of::<UiNodeRecord>()` is 6 416 B, so ≈ 821 KB per surface regardless of how
   many nodes it holds); (b) the node-commit charge **double-counts** every node's semantic bytes —
   `♻️reconcile/🦀️.rs:1462` adds `size_of::<FlatPresentedNode>() + self.semantic_usage.bytes` to a
   `usage.bytes` that already accumulated exactly those bytes one census page at a time through the `Yield`
   funnel at `:1837`. Fixing (b) alone would roughly halve the peak. Both are accounting, not allocation,
   so they cost headroom rather than memory — but headroom is what ran out.
2. **`semio-framework-ui-contract`'s `document`/`resident` law families are red at HEAD** (29 failures,
   two root causes at `🎟️assembly/🧪️tests/🎟️assembly/🦀️.rs:43` and `:13`, the rest an `ArenaFull` cascade
   behind them). Not this wave's — proven by an identical failure-name diff against a neutralised baseline.
3. **`cargo test -p semio-framework-plugin --lib -- reactor` aborts the process** in
   `NativeLifecycleRegistry::drop` (`🚪️lifetime/🦀️.rs:489`) when
   `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` runs after other reactor tests; it
   passes alone. A panic in a `Drop` is a non-unwinding abort, so ONE polluted registry takes the whole
   suite down and hides every later law. Worth its own wave.
4. **`turn::turn_execution_tests::guest_turn_execution_resets_for_every_turn` is flaky under load**
   (asserts an exact microsecond count; observed `Some(2)` against `Some(1)` while a peer build was
   running).
5. A peer's `semio-framework-deflate` refactor left `semio-framework-pack` uncompilable for ~10 minutes
   (§4.6). Reported, not worked around.
