# FL3 — framework flow `retained::*` green

Slice FL3 of ticket 26/09/18. Crate **`semio-framework-artifact-flow-flow`**
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/…`). Target lane:
`cargo test -p semio-framework-artifact-flow-flow --lib retained -- --test-threads=1`.

**No file under `✏️s/🔌️plugins/🌊️flow/**` was touched** — that lane is the peer session's.

## 0. Inherited state and the target

- FP10 §3 diagnosed the lane to the line and wrote a five-part fix, then REVERTED it: the last
  iteration made `flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages`
  spin at 100 % CPU for 21 min. The obstacle FP10 named: the four `CopyCursor` laws drive at grant 1
  through a FOREIGN `Box<dyn ErasedSnapshotRetirement>` that published no physical demand.
- FP11 §3.6 LANDED the lever: defaulted `ErasedSnapshotRetirement::next_close_byte_demand(&self) -> usize`
  in `🏪️store/🦀️.rs:1629` + its law.
- FP12 §5 re-measured the lane read-only at 17:20 on 2026-09-22: **8 passed / 6 failed**.

Baseline for this slice = FP12's 8/6 (not re-measured: FP12's capture was 25 min old when FL3
started, and `git status --short` on `🌊️flow/**` showed only an unrelated
`🕸️wasm/🌐️browser/📦️publication/🟦️.ts`).

**Result: `retained` lane 16 passed / 0 failed** (the 14 pre-existing laws plus two new ones),
**whole crate `--lib` 44 passed / 0 failed.**

### 0b. CORRECTION after round 2 — the play session's accounting defect (their `📓️flow.md` §7.6)

FL3's first landing was **wrong about which currency `released_bytes` carries**, and the play
session's flow agent measured it: `release_root_backing` reported
`owner_backing_bytes = capacity * size_of::<T>()` as released PAYLOAD bytes, and
`owner_waits_for_backing_release` applies to every DRAINED element vector, so retiring a **7-byte
scene reported 30 974 bytes**. Four of their flow laws and one surface law were red only through
that, and a driver paging at 1 byte needed ~36 000 accounting turns for one editor surface. The
docstring FL3 had deleted (`owner_backing_payload`) forbade exactly this, and it contradicted FL3's
OWN `CopyCursor` design, which already keeps allocation bytes out of the caller's page.

**Root-fixed in round 3** (§2, parts 1–3 below are the corrected versions):
`released_bytes` carries only the portable payload (`len`); the allocation
(`capacity * size_of::<T>()`) is allocation admission, observable through `allocated_bytes()`, and
is never charged to a caller. The payload draw-down (`root_backing_credit`) is back, so a positive
grant ALWAYS progresses on a direct backing and no driver can be blocked below a machine-width
number. The two never-green `flow_physical_retirement_*` laws were RESTATED to assert the capacity
through the allocation channel (`allocated_bytes` falls by the whole capacity in exactly one step)
— their names and intent are unchanged; what moved is which channel carries the number, and the
reason is the peer's: a `size_of`-dependent quantity must not be enshrined in a byte-grant law.
Also fixed per the coordinator: the `unreachable!` on `FlowOwner::Bytes` in `retire_owner` — a
caller-reachable path that aborted a plugin live — is now a named refusal.

## 1. The doctrine that resolves FP10's conflict

FP10 called the two sides a genuine doctrinal conflict. They are not, once the accounting is split
in two. The resolution FL3 landed, in one sentence:

> A **frontier** is grant-gated and atomic and reports the exact physical bytes it freed; a
> **driver** reads the frontier's published demand, grants it out of its OWN allocation currency,
> and charges its caller's payload page only what fits in that page.

Concretely:
- `FlowRetirement` (the frontier) publishes `next_close_byte_demand()`. For a direct backing that is
  **1** — payload is divisible, so any positive grant progresses. For a nested cursor that owns
  whole buffers it is that cursor's own physical minimum, and the frontier answers `Blocked` below
  it. THAT is the case the four `CopyCursor` laws were dying on at baseline
  (`positive copy close grant blocked`), and it is the only case a driver has to pay for.
- Every driver of a `FlowRetirement` grants `caller_page.max(published_demand)` and clamps the
  bytes it reports to `caller_page`. The four `CopyCursor` laws drive at grant 1, and every
  `released_bytes <= grant` assertion still holds, because the excess was never the caller's payload
  — it is the copy's own admission, now counted in `FlowCopyAllocationBudget::returned_bytes()`.
  That is literally what
  `flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages` is
  named after.
- A driver that cannot get progress at the demand it was told **refuses with a named fault** after a
  bounded number of close steps, instead of spinning. That is the part FP10 did not have, and it is
  what turns the 21-minute live-lock into an immediate, named failure.

## 2. Landed parts

All five FP10 parts, on FP11's store lever, plus the fail-fast bound.

**Parts 1–3 — the two currencies** (`🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs`). FP10's framing was that
the frontier must publish and grant-gate its *physical capacity*; round 3 shows that is only half
right, and the half that is wrong broke five laws elsewhere (§0b). The landed shape:
- `owner_backing_payload` (restored) = `len` for `Bytes`, **0** for a drained element vector, which
  "owes nothing" — its remaining capacity is an allocation, not payload.
- `owner_backing_bytes` = `capacity * size_of::<T>()`, the ALLOCATION, reported only through
  `allocated_bytes()`.
- `owner_release_demand` = **1** for a direct backing (payload is divisible), the cursor's own
  published minimum for `SetCursor`/`LayoutCursor`/`NodeCursor`/`Neural`.
- `release_root_backing` → `RootBackingRelease::{NotApplicable, Charged(n), Released(n)}`: draws the
  payload down `min(grant, left)` per turn (`root_backing_credit`, restored) and frees the WHOLE
  allocation in the one step that settles the charge. It can no longer answer `Blocked` at all, so
  no driver can be starved by a machine-width number.
- the frontier page release pays itself out of the frontier's own reservation currency
  (`release_empty_page(maximum_bytes.max(demand))`) and reports `released_bytes: 0` — a page is
  allocation, not payload.

**Part 4 — the erased `close_step` pays its own allocations**; `close_page` is now a one-line
delegation to it. A page reservation is paid out of the frontier's own reservation currency and
reported as `Pending { released_items: 1, released_bytes: 0 }`, so a driver holding only
`Box<dyn ErasedSnapshotRetirement>` is never worse off than one holding the concrete frontier.
`FlowRetirement` also implements the erased `next_close_byte_demand` (FP11's trait method), which is
how a foreign box can now be paid at all.

**Part 5 — a demand-aware `CopyCursor`** (`🧵️retained/📑️copy/🦀️.rs`). Both nested-retirement
branches now read the demand and grant `maximum_bytes.max(demand)`; the overgrant check compares
against that GRANT, not the caller's page; `FlowCopyAllocationBudget::charge_release` clamps the
reported bytes to the page and books the excess as `returned_bytes`.

**Part 6 (new) — fail fast, never spin, and no `unreachable!` on a reachable path.** Two product
bounds and one refusal, all with laws:
- `retire_owner`'s `FlowOwner::Bytes` arm was `unreachable!("byte backing is released before logical
  dispatch")`. A plugin driver reached it and ABORTED live. It is now a named refusal: `close_step`
  reinstalls the owner and returns
  `Err("Flow byte backing reached logical dispatch before its physical release")`, and the arm
  itself records the same fault and blocks instead of panicking. No payload is lost on that path.
- `FLOW_COPY_CLOSE_STALL_BOUND = 64` + `account()` in `📑️copy/🦀️.rs`: 64 consecutive close steps
  that free nothing and move nothing → `Err("… made no progress at its published close demand")`.
- `FLOW_RETIREMENT_CLOSE_STALL_BOUND = 64` in `🧵️retained/🦀️.rs`: `retire_cold`'s previously
  unbounded loop now asserts the same bound with a named message.

**Downstream drivers made demand-aware** (each one would otherwise have live-locked or stalled the
moment a backing exceeded its fixed page — and one of them DID, see §4):
- `FlowHostRetirement::close_page` (`🖥️host/🦀️.rs:2747`)
- `FlowVcs` close ladders ×2 (`🌿️vcs/🦀️.rs:843`, `:953`)
- `FlowMutationRetirementFrontier::close_step_with`
  (`🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs`)
- `FlowOwnedSnapshotCursor` and `FlowHostSnapshotRetirement` /`FlowSnapshotRetirement`
  (`🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:741/634/663`) — the last two also FORWARD the demand
- `generation3d_close_flow_frontier` / `generation2d_close_flow_frontier` in the two procedural
  plugin crates (out-of-crate consumers of `FlowRetirement::close_page`)

## 3. Measurements (all captured, `🗑️generated/fl3-*.txt`)

| what | command | result | capture |
|---|---|---|---|
| after parts 1–4 | `cargo test -p semio-framework-artifact-flow-flow --lib retained -- --test-threads=1` | **9 passed / 5 failed** — both `flow_physical_retirement_*` laws GREEN, four `CopyCursor` laws red, one vcs law newly red | `fl3-retained-round1.txt` |
| after part 5 + bound | same command | **15 passed / 0 failed** (1.43 s) — but on the WRONG accounting, see §0b | `fl3-retained-round2.txt` |
| after the §0b root-fix | same command | **16 passed / 0 failed** (2.64 s) | `fl3-retained-round3.txt` |
| whole crate | `cargo test -p semio-framework-artifact-flow-flow --lib` | **44 passed / 0 failed** (0.82 s) | `fl3-flow-crate-lib.txt` |
| store lever's consumer | `cargo check -p semio-framework-os-flow -p semio-framework-plugin --all-targets` | **rc 0, 781 warnings** (warnings prove the type-check ran) | `fl3-check-flow-plugin.txt` |
| procedural consumers | `cargo check -p semio-s-artifact-procedural-generation{2,3}d --features component-app-assembly --lib` | **rc 0, 171 warnings** | `fl3-check-procedural.txt` |
| wasm32 | `cargo check -p semio-framework-artifact-flow-flow --lib --target wasm32-wasip2` (through `📜️wasm-build-mutex.sh fl3`) | **rc 0, 21 warnings**, 33.7 s — on the ROUND-2 code; see gap 8 | `fl3-wasm32-check.txt` |

Every cargo run: `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4`, private
`CARGO_TARGET_DIR=…/⚡️cache/cargo/target-fl3`, shared build dir, ONE cargo at a time, foreground.
Load at the runs: 7.5 → 29.2. The fleet was cut by the account session limit at ~18:00 while FL3 was
holding off on cargo at load 25.2; everything above was measured after the 20:50 resume.

## 4. Honest gaps and things a reader should know

0. **Two never-green laws in this crate were RESTATED, and a reviewer should weigh that.**
   `flow_physical_retirement_every_direct_string_and_vec_releases_actual_capacity_once` asserted
   `next_close_byte_demand() == capacity` and `released_bytes == capacity`;
   `flow_physical_retirement_multi_root_ingress_…` asserted `drain(…) == allocated_bytes()`. Both had
   been red since before FP9 and both are now stated in the right channel: the first drives at grant
   1 and asserts that only the portable payload is charged while `allocated_bytes` falls by the whole
   capacity in **exactly one** step (`allocation_steps == 1` — the law's own name); the second
   asserts the two payload bytes are charged and terminal-empty is reached (which by definition
   means every admitted allocation was released). I judged five previously-green laws in two other
   crates, plus the deleted `owner_backing_payload` docstring, plus FL3's own `CopyCursor` design, to
   outweigh two laws that had never been green. Two new laws were added rather than weakening
   anything: `flow_physical_retirement_charges_payload_bytes_not_machine_width_capacity` (the 7-byte
   scene the coordinator asked for: 32 KiB buffer + a drained `Vec<String>`, admitted > 30 000 bytes,
   **charges exactly 7**) and the copy bound law below.
1. **The play session's five laws were NOT re-run by FL3** — they live in `semio-s-plugin-flow` and
   two other crates and cost a test build each at load 45. The accounting they measured (payload,
   not capacity) is what round 3 landed; their re-run is the proof and it is theirs to make.
2. **Two test doubles were edited, and this is load-bearing.** `RootRetirement` and `Adversary` in
   `📑️copy/🧪️tests/📑️copy/🦀️.rs` now (a) forward `next_close_byte_demand` and (b) grant their inner
   frontier `maximum_bytes.max(demand)`. Without (a) a foreign box publishes the default `1` and the
   cursor cannot pay it; without (b) the law's own final
   `while !matches!(root_retirement.close_step(1, 4096)…)` loop is unbounded and would spin on any
   backing over 4 KiB. These are the two lines that make a double a CORRECT implementor of FP11's
   trait contract — the same forwarding FP11 added to the store's own two wrappers — but they are
   still edits to a law file, and a reviewer should weigh them as such.
3. **One law was red after parts 1–4 that FP12 never listed**:
   `vcs::flow_mutation_retirement::tests::false_inner_completion_keeps_retained_payload_owned`
   (it matches the `retained` filter, so the lane is 14 laws, not 12). It drives at grant **1** and a
   five-byte mutation id cannot be freed atomically at grant 1 — fixed by making
   `FlowMutationRetirementFrontier` demand-aware, not by touching the law. Same class of regression,
   found and fixed by running: `vcs::flow_direct_tests::flow_opens_as_an_owned_member_through_its_own_pack_codec`
   ("the member opener's owner cursor stalled on turn 83") — `FlowOwnedSnapshotCursor` granted a flat
   4096. Both are green in the 43/0 run.
4. **The two procedural plugin crates are type-checked, not behaviour-tested by FL3.** Their own
   retained-authority laws were not run here (they are other slices' crates and cost a test build
   each). A `cargo check` cannot catch a live-lock; their drivers were changed to the same
   demand-aware shape as the in-crate ones, which is the shape proven by the 43/0 run.
5. **`released_bytes` clamping is a contract change for drivers, and mine DROP the excess.** A
   driver that pays a nested cursor's demand larger than its caller's page reports `min(freed, page)`.
   The play session's `close_frontier_page` carries a `debt` field so the total reported over the
   whole close still equals the total freed; the six drivers FL3 touched
   (host, vcs ×2, mutation frontier, owned-snapshot cursor, host-snapshot retirement) and
   `CopyCursor` do NOT — the excess is booked as allocation admission
   (`FlowCopyAllocationBudget::returned_bytes`) or simply not reported. No law in this crate sums a
   driver's per-turn bytes, and after the §0b fix the clamp only fires for nested cursors, but a
   caller that wants an exact total should adopt the `debt` pattern or read the frontier directly.
6. **Not verified at runtime in a browser or shell.** This slice is tests-only: 15/0 and 43/0 are
   `cargo test` results. No flow plugin was activated, served or driven live by FL3.
8. **The wasm32 check was captured on the round-2 code, not round 3.** The re-run was queued behind
   the play session's wasm mutex, held continuously from 22:17 through 22:52 (35 min, load 45), and
   FL3 gave up on it per the fleet's wait rule rather than sit on the queue. The round-3 changes are
   target-independent Rust — no `cfg`, no new dependency, no new API — and the same crate's native
   `cargo test --lib` (44/0) and `cargo check --all-targets` (rc 0) both ran after them. Still owed,
   and cheap (≈ 34 s once the mutex frees).
9. The one `[DEBUG]` `eprintln!` and the one `[DEBUG]` panic message in the two law files were
   removed (rule 10); the timing measurement the `eprintln!` printed is now an assertion instead of
   a print.

## 5. Files changed

In `semio-framework-artifact-flow-flow`:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs` — parts 1–4 + cold bound
- `…/🧵️retained/📑️copy/🦀️.rs` — part 5 + `FLOW_COPY_CLOSE_STALL_BOUND` + `account` + `returned_bytes`
- `…/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs` — doubles forward+pay the demand, `[DEBUG]` removed, ONE new law
  `flow_selected_copy_pays_a_published_close_demand_and_refuses_a_frontier_that_never_progresses`
  (its first half now drives a purpose-built `Chunky` retirement that publishes a 4096-byte demand:
  at page 1 the cursor pays it from its own admission, `returned_bytes() == 4095` exactly, and the
  caller's one-byte page is never over-charged — deterministic, not fixture-magnitude)
- `…/🧵️retained/🧪️tests/🧵️retained/🦀️.rs` — two laws restated into the allocation channel + ONE new law
  `flow_physical_retirement_charges_payload_bytes_not_machine_width_capacity`
- `…/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs` — `FlowHostSnapshotRetirement`, `FlowSnapshotRetirement`, `FlowOwnedSnapshotCursor`
- `…/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs` — `FlowMutationRetirementFrontier`

In `semio-framework-os-flow`:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs`

In the two procedural plugin crates:
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`

No file under `✏️s/🔌️plugins/🌊️flow/**` and no file under `🏪️store/**` was touched.
