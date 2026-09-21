# FP4 — `semio-framework-plugin --lib` as a gate

Slice FP4, 2026-09-21 (session 8). Continues FP3. Every FP1–FP3 capture was wiped by the 02:30 clean,
so the baseline was re-measured from scratch on the current tree.

All runs: `cargo test -p semio-framework-plugin --lib --no-fail-fast`, private
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp4` (preamble rule 25), shared build-dir, one
cargo at a time, foreground, no sub-agents. Per-test runs drive the compiled binary directly with
`RUST_MIN_STACK=67108864` — the repo-wide value `.cargo/config.toml` `[env]` gives `cargo test`; without
it 12 laws abort with a stack overflow that is an artefact of the invocation, not of the law (§2.1).

**The headline is a correction, not a pass count: the suite is not three arena fixes away from green.
Measured law by law, 61 of the 66 reds fail ALONE, in a fresh process, with an empty arena.** The gate
is 62 independent product/fixture questions away from green, and this slice closed the ordering
problem rather than pretending the count was nearly there.

## 1. Round table

| round | worked | passed | failed | capture |
|---|---|---:|---:|---|
| 0 (baseline, this tree) | — | 752 | 66 | `fp4-round0-suite.txt` |
| 0-solo | every red re-run alone in a fresh process | — | — | `fp4-solo-runs.txt` |
| 1 | built-child retirement pool reclaims before it refuses (§3.1) | 751 | 67 | `fp4-round1-suite.txt` |
| 1b | same tree, second draw | **754** | **64** | `fp4-round1b-suite.txt` |
| 2 | (no code change) three orderings of round 1 | 752 / 752 / 755 | 66 / 66 / 63 | `fp4-round2-para/parb/serial.txt` |
| 3 | handback registry isolation no longer stomps live slots (§3.2) | **754** | **64** | `fp4-round3-para.txt` |
| 3′ | round 3 repeated, byte-identical tree | **754** | **64** | `fp4-round3-parb.txt` |
| 3″ | round 3 serial (`--test-threads=1`) | **756** | **62** | `fp4-round3-serial.txt` |
| 4 | after the orphan-suite deletion (§5) and the gate row (§6) | 754 | 64 | `fp4-round4-final.txt` |

**Net: parallel 752/66 → 754/64, serial 752/63 (FP3's figure) → 756/62, and the population of laws
whose verdict depends on ordering fell from 7 (FP3 §6.1) to 3.** The 62 that are red in every ordering
are the same 62 every time — that is the property this slice was asked to produce, and it is the one
that makes the number worth quoting at all.

## 2. Classification: what the reds actually are

### 2.1 Every red re-run alone (`fp4-solo-runs.txt`)

Each of the 66 baseline reds was run on its own, in its own process, against the compiled binary:

| solo verdict | count | meaning |
|---|---:|---|
| **FAILED alone** | **61** | a real defect; nothing to do with arenas, load or ordering |
| passed alone | 5 | order- or load-dependent — the whole of the "flaky population" |

The five order-dependent laws were `a_100kib_measures_section_fits_document_node_cap`,
`fixture_projection_retires_exact_tree_before_return_error_or_panic`,
`every_inbound_request_row_is_answered_on_the_turn_it_arrives`,
`tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks` and
`guest_turn_execution_resets_for_every_turn`.

**This retires the brief's premise.** FP3 reported the UI admission arena as "the single highest-value
item left … without it the suite cannot be a gate at all", with "±3 on every number". The ±3 is real
and it is now fixed, but it was never the reason the suite is red: it accounted for 3 laws of 66. The
other 61 are ordinary unfixed defects, and no arena work of any kind moves them.

A methodological note worth keeping: a first sweep run without `RUST_MIN_STACK` reported 12 laws as
stack-overflow aborts and 2 as order-dependent. Both numbers were artefacts of bypassing
`.cargo/config.toml`'s `[env]`. Any per-test triage of this crate that shells the binary directly must
export `RUST_MIN_STACK=67108864` or it will invent a bucket that does not exist.

### 2.2 Bucket census, before and after

Buckets are assigned by first panic line. The "after" column is the serial run, which is exactly the
62-law order-independent core.

| bucket | before (round 0, 66) | after (round 3 serial, 62) |
|---|---:|---:|
| composed-child `compositeEdit` lane: missing mutation retirement factory | 5 | 1 |
| fixture close never reaches terminal-empty | 4 | 4 |
| `interactive-job.missing-factory` (manifest/editor command routes) | 4 | 4 |
| transaction fixture (proposal / pending / generation) | 3 | 3 |
| envelope decode worker: fixture app has no external owner | 2 | 2 |
| retained composed replacement state machine | 2 | 2 |
| `maintenance_step` close-authority shape | 2 | 2 |
| tool factory proof / catalog authority | 1 | 1 |
| UI intent has no retained raw-page owner | 1 | 1 |
| typed command not on the mounted live instance | 1 | 1 |
| typed operation lost its worker session | 1 | 1 |
| `ui.fixed-capacity` UI admission arena | 1 | **0** |
| bare ``left == right`` (one question per law) | 16 | 15 |
| one-off, each its own question | 23 | 25 |
| **total** | **66** | **62** |

Two cautions about this table, both measured rather than assumed. First, **the panic text of a core red
is not stable across orderings**: the composed-child bucket reads 5 in the parallel run and 1 in the
serial one although the same laws are red in both — which fault surfaces first depends on interleaving.
Bucket *counts* below about 5 are therefore soft; the *membership* of the 62-law core
(`fp4-hard-core.names`) is exact. Second, the largest two rows are not buckets at all, they are the
residue: 40 of 62 are one law each. There is no remaining lever of the kind FP3 §3 and §4 found, where
one line bought six or eight laws. The rest of this suite is retail work.

## 3. The process-global arenas

Two separate process-global pools drove every one of the five order-dependent laws. Neither is a test
concern in the product's own terms — a wasm guest is one process holding one plugin runtime, so
"process-global" and "per-runtime" coincide there. They diverge in exactly one place: a `cargo test`
binary hosts 818 independent runtimes over one pool. Rather than scope the pools per test (which would
have meant threading a guard through 818 laws and would have left the product's own exhaustion
behaviour untouched), both fixes make the pools behave correctly *as pools*, which is what makes the
laws order-independent as a consequence rather than as a special case.

### 3.1 The built-child retirement pool refused against credit it could reclaim

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs` — `BUILT_CHILD_RETIRE_AUTHORITY`, 384 slots.

A `BuiltChildren` claims a slot on its first `try_push`. Its `Drop` does **not** release that slot: it
*publishes* the backing into the slot, because retiring a deep tree inline would blow the bounded stack
the whole authority exists to protect. The slot is released only when a pump — `close_built_node_page_one()`
— drains that owner to its end. The pump has exactly three callers
(`🧠️runtime/🎭️present/🦀️.rs:296`, `🧠️runtime/♻️reconcile/🦀️.rs:3650`, `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:585`),
so the reactor turn and the reconciler drain what they build and **every other owner of built pages
abandons its slots for good**: a panel kit assembling a probe tree, an out-of-turn assembly helper, a
second runtime in the same process.

Once the 384th slot went, `reserve()` returned `None`, `try_push` refused, and `try_children` reported
`ui.fixed-capacity` — *in whatever unrelated builder happened to run next*, at whatever stage it happened
to be in. That is the whole mechanism behind "fixed UI admission failed at section-root" landing on
`a_100kib_measures_section_fits_document_node_cap`.

**The defect stated as a product law: an admission must never refuse against credit it can itself
reclaim.** `reclaim_built_child_retire_slot()` now drains the queue in place when `reserve()` comes back
empty, and only a pool whose every slot backs a *live* tree still says no. Two properties make this
safe and bounded, and both are load-bearing:

- **It is bounded by exactly one page.** A fully drained owner releases its slot, and the nodes it
  releases republish into slots they *already hold* — `reserve()` is the only claimant in the file, so
  the pool cannot grow while it is being drained.
- **Every node is dropped outside the authority lock.** Dropping one republishes its own children through
  the same non-reentrant `Mutex`; draining under the guard would deadlock. The reclaim loop returns the
  node from the closure and drops it after the guard is gone.

Verified by instrumenting the refusal path with a pool census, running the full suite, and observing
**zero** hard refusals (the reclaim always succeeded); the instrumentation was then removed.

This is not only a test fix. `tree_window_rows` (`🔌️plugin/🦀️.rs:6357`) treats `ui.fixed-capacity` as
"end this window early with a shorter materialised run" and its own docstring names the cause — "the
process-global argument arena taken by another panel mid-build". In the product that meant **a panel
silently rendered fewer rows than it has because an unrelated panel had not been pumped.** The clearest
evidence that this was a live cost, not a hypothetical: `tool_run_overlay_append_per_tick_stays_below_
two_milliseconds_for_nakagin_sized_ticks` — a genuine wall-clock law FP3 §6.3 explicitly declined to
touch, measuring 4 of 771 appends over 2 ms with a 7.7 ms worst case — **passes in every post-fix run.**
The latency spike was the exhausted pool, not the append.

### 3.2 The handback registry's "isolation" stomped slots live in other tests

`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs` — `SURFACE_RECONCILE_HANDBACKS`.

`reclaim_orphaned_handback_slots()` reset **every** slot to free and rebuilt the free list by position,
regardless of who held what. `SurfaceReconcileRegistryTestGuard`'s mutex serialises only the laws that
*take* the guard, so a law running beside one that did had its reservation freed underneath it; its own
later release then found the free list already full and tripped the overflow guard at
`♻️reconcile/🦀️.rs:2548`/`:3557`, which reports `surface handback free list exhausted`. **That message is
an accounting overflow reported as if the pool had run dry** — the pool was in fact completely free —
and it surfaced in whichever law happened to release next. It is the fault
`every_inbound_request_row_is_answered_on_the_turn_it_arrives` died of, as
`plugin.reactor-close-authority: surface handback free list exhausted`.

The reclaim now works from slot **state** instead of position: a `reserved` slot is never reclaimed, the
free list is rebuilt from the slots that are genuinely unowned, and the retirement ring is rebuilt from
the entries whose slot survived rather than blanket-cleared, so a live owner's queued retirement is not
silently dropped either.

### 3.3 What ordering-dependence is left, measured

Three consecutive runs of the byte-identical post-fix tree — two parallel, one serial
(`fp4-round3-para/parb/serial.names`):

| law | in para | in parb | in serial |
|---|---|---|---|
| `tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks` | red | green | green |
| `fixture_projection_retires_exact_tree_before_return_error_or_panic` | red | red | green |
| `guest_turn_execution_resets_for_every_turn` | green | red | green |
| the other 62 | red | red | red |

The two survivors are the honest ones: a 2 ms-per-append ceiling and an exact-microseconds equality,
both asserted on a 10-core machine running a whole fleet. Neither has a deterministic basis that
preserves what it measures, and FP3 §6.3's ruling stands — the cure is to run them off a loaded fleet,
not to weaken them. No ceiling anywhere was changed: not the 2 ms budget, not `UI_TEXT_MAX_BYTES`, not
`UI_BUILT_CHILDREN_MAX`, not `UI_BUILT_CHILD_RETIRE_SLOTS` (still 384), not
`SURFACE_RECONCILE_HANDBACK_SLOTS`.

## 4. Per-bucket root fixes — what was NOT attempted, and why

**Nothing in the 62-law core was fixed in this slice.** That is a scope outcome and it should be read as
one rather than as a result.

The brief asked for the arena at the root, then every bucket at the root, then the orphan suites, then
the gate. The arena work (§3) cost most of the budget because it needed the solo sweep of §2.1 to find
out that the brief's own premise about it was wrong, and a second pool (§3.2) that no prior slice had
identified. The remaining 62 are, by the measurement in §2.2, **40 laws that are one question each** plus
a handful of 2–4 law groups. For calibration: FP1, FP2 and FP3 each landed about 16 laws in a full slice,
and FP3's two productive roots were worth 8 and 6 laws for one line each. No comparable lever remains —
§2.2's table is the evidence, and inventing one would mean guessing.

What a successor should take, in the order the measurement supports:

1. `fixture close never reaches terminal-empty` (4) — untouched by FP1, FP2 and FP3 as well; the oldest
   standing bucket and the only one with four members and a single named symptom.
2. `interactive-job.missing-factory` (4) — the SURFACE packet FP3 §5 scoped and declined
   (`editor_fixture_still_mutates_normally`), plus `addressed_window_action_…` and
   `manifest_mode_command_…`. FP3 costed this at a full fixture packet for the first of them.
3. The transaction fixture (3) and the retained composed-replacement state machine (2).
4. `local_interaction_cold_transaction_receipts_and_encoded_route_rejection` remains red **on purpose**;
   FP3 §5 established it is a peer's landed feature retiring the clause's premise, and the decision
   belongs to the route owner. Nothing in this slice changes that reading.

No law was deleted, `#[ignore]`d or loosened anywhere in this slice.

## 5. Orphan suites

**K2's 115 orphans are bun/vitest suites, not Rust `[[test]]` targets.** `📓️k2-test-infrastructure.md`
§12 gives the census as `candidates=1113 vitest=195 direct=861 transitive=137 orphans=115` over the
whole repo's TypeScript test owners, and §12's own table already assigns every area an owner (47 to T4c,
11 to dev infrastructure, 11 unowned UI, and so on). None of them is a `semio-framework-plugin` Rust
suite, and none is reachable by adding an ASCII `[[test]] name` + `path`, which is the shape the brief
anticipated. Taking them would have been taking another slice's work.

The question that *is* this slice's — "does the gate have orphan suites?" — was answered by auditing the
crate directly. 174 `🧪️tests/**/🦀️.rs` files exist under `🔌️plugin`. Reachability must count **both**
`#[path = …]` and `include!(concat!(env!("CARGO_MANIFEST_DIR"), …))`; this crate mounts most of its
suites with the latter, and a `#[path]`-only audit reports 51 false orphans, including
`🔬️plugin-runtime-plugin-builder-contract` — the largest suite in the crate.

| # | suite | verdict |
|---|---|---|
| 1 | `⚛️reactor/📨️pending/🧪️tests/🧪️authority/🦀️.rs` | **genuine orphan — deleted, tests a deleted API** |
| 2 | `📇️registry/dist/rust-taxonomy-mounts-check/…/🔬️orphan/🦀️.rs` | not an orphan: a fixture *named* `🔬️orphan`, an input to the taxonomy-mounts check |
| 3 | 4 suites under `🖥️host/🧪️tests/` | belong to `semio-framework-plugin-host`, a different crate |
| 4 | 7 others (`📏️future-size`, `🧵️runtime`, `🧪️aggregate-admission`, `⏳️completion`, `🧩️composition`, `🧾️document-archive-load-legs`, `📨️dispatch`) | reached by `include!(concat!(env!("CARGO_MANIFEST_DIR"), …))` — each call site verified individually |
| | **remaining orphans in `semio-framework-plugin`** | **0** |

**The one real orphan, and why it was deleted rather than wired.** It was mounted
(`#[cfg(test)] #[path = "🧪️tests/🧪️authority/🦀️.rs"] mod authority_tests;`, beside the sibling
`🔬️instance-lifetime-patch-close` that *is* mounted in the same file) and run. It does not compile: 16
errors, all naming an API that no longer exists on `PendingPatchAuthority`
(`fp4-orphan-authority.txt`).

The deleted symbols, each confirmed absent from the whole repo by grep:
**`PendingPatchAuthority::reserve_external`**, **`PendingPatchAuthority::place_external`**,
**`PendingPatchAuthority::reserved_receipt`**, and **`PendingPatchAuthority::required_reservation_bytes`**
(the name survives on `SurfaceReconcileOutputs` and `UiResidentPermit`, never on this type). A fifth
signature moved: `close_instance_step` now takes `instance: u32`, not a `NativeCloseKey`.

The whole external-*reservation* concept is gone from the type: an external patch is now pushed straight
in with `push_external`, refused by handing the patch back (`Result<(), UiPatch>`), and drained with
`take_one` / `hand_back_turn`. Both orphan laws survive under the new API in the mounted sibling suite —
`guest_instance_lifecycle_pending_patch_handback_preserves_rejected_owner_and_exact_bytes` is the
"preserves refused owner" law, and `guest_instance_lifecycle_pending_patch_unwind_keeps_the_exact_typed_
cursor_mounted` the foreign-close one. The file tests an API that was replaced, and its laws are already
covered, so it was deleted (directory `⚛️reactor/📨️pending/🧪️tests/🧪️authority/` removed) and the mount
reverted.

## 6. Gate registration

- **nx target: already present.** `@semio-tech/framework-plugin:test` →
  `bun 📜️script.ts test` → `{ mode: "budgeted", args: ["--lib", …] }`
  (`🔌️plugin/📦️packages/🦀️rust/📜️script.ts:18`). It is the lib gate; no new target was needed.
- **launch row: added.** `⚖️gate🔌️plugin🦀️lib`, `group: "4_gate"`, `order: 408.39` — immediately after
  `⚖️gate🖱️ui🖥️host🌐️wasm` (408.38), continuing that framework-module gate cluster. Added to **both**
  `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (there is no generator between them; the repo
  keeps them in step by hand), command
  `bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache`, matching every neighbouring row's
  shape. Both files re-parse as valid JSON afterwards (391 configurations).
- **`--all-targets` inventory** (`fp4-all-targets.txt`): `cargo test -p semio-framework-plugin
  --all-targets --no-run` builds **exactly one test executable**, the lib unittests binary — the crate
  declares no `[[test]]` integration targets. **`--lib` and `--all-targets` are the same set for this
  crate**, so the gate row covers it completely. **Test count: 818** (754 passed / 64 failed in the
  final parallel run; 756 / 62 serial).

## 7. Honest gaps

- **The gate is NOT green.** 62 laws are red in every ordering. The brief's end state ("0 failed") was
  not reached and was not reachable in one slice at the measured cost per law (§4). What *was* delivered
  is the precondition it asked for and the reason it asked for it: the number is now the same number
  every run, and the 62 are enumerated by name in `fp4-hard-core.names`.
- **No law in the core was fixed.** §4 says which to take first and why, with the costing.
- **Two wall-clock laws still move with machine load** (§3.3) and are load artefacts, not defects. On a
  quiet machine the serial figure 756/62 should be the honest reading of this tree.
- **Bucket counts under ~5 are soft** because a core red's panic text changes with ordering (§2.2). Core
  *membership* is exact.
- **`--features component-app-assembly` was not exercised.** The gate as registered runs the default
  feature set, which is what `cargo test -p semio-framework-plugin --lib` means and what every FP slice
  has measured. Whether the crate's wasm-gated code needs a second gate row is an open question this
  slice did not answer.
- **No dependent crate was re-checked.** Both product changes are private: `reclaim_built_child_retire_slot`
  is a private fn in `🏗️builder/🦀️.rs` and `reclaim_orphaned_handback_slots` a private fn in
  `♻️reconcile/🦀️.rs`. No public item changed signature, and no ceiling constant changed value. The two
  crates that own them (`semio-framework-ui`, `semio-framework-ui-runtime`) compile clean in every run
  above, but their **own** suites were not run — a peer owning either crate should re-run them, since
  §3.1 and §3.2 change behaviour under exhaustion that their laws may assert directly.
- **The instrumentation used to prove §3.1 was removed**; no `[DEBUG]` output is left in the tree.
- Every number here is read from a capture in `🗑️generated/`. Nothing is claimed that was not executed.

## 8. Files changed

Product code (3 files):
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs` — `reclaim_built_child_retire_slot()` added;
  `BuiltChildren::try_push` falls back to it when `reserve()` is empty (§3.1)
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs` — `reclaim_orphaned_handback_slots()` reclaims
  by slot state instead of by position and rebuilds the retirement ring from the surviving entries (§3.2)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs` — unchanged net (a mount was
  added to run the orphan suite, then reverted with it, §5)

Deleted (1 directory):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🧪️tests/🧪️authority/` — tests
  `reserve_external` / `place_external` / `reserved_receipt` / `required_reservation_bytes`, none of which
  exists anywhere in the repo (§5)

Launch registration (2 files):
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` — `⚖️gate🔌️plugin🦀️lib` row at order 408.39 (§6)

Captures (`🗑️generated/`): `fp4-round0-suite.txt`, `fp4-round0-failnames.txt`, `fp4-round0-panics.txt`,
`fp4-solo-runs.txt`, `fp4-round1-suite.txt`, `fp4-round1b-suite.txt`,
`fp4-round2-para/parb/serial.txt` (+ `.names`), `fp4-round3-para/parb/serial.txt` (+ `.names`),
`fp4-round3-panics.txt`, `fp4-hard-core.names`, `fp4-orphan-authority.txt`, `fp4-all-targets.txt`,
`fp4-round4-final.txt`.
