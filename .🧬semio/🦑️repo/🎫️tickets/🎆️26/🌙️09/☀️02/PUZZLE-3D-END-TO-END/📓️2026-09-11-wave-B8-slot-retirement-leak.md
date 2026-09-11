# Wave B8 — the typed-operation slot retirement leak

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-B8, 2026-09-11. Follows wave B0
(`📓️2026-09-11-wave-B0-pre-admit-slot.md`), whose §6 predicted exactly this: *"If the browser battery shows
`every fixed typed-operation and segmented-output slot already owns a live operation`, that is a genuine
retirement leak (operations mounted and never closed), a different defect."* It is. **Fixed, laws green,
no regression in the crate's failure set.**

No git write, ticket not closed, `🗑️generated` not deleted. Shared cargo build-dir, no
`CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, every command in the foreground.

---

## 1 Admission and retirement, as they actually worked

All line numbers are post-wave in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` unless noted.

### 1.1 Admission — synchronous, at ingress, one slot per action

| step | site |
| --- | --- |
| ingress picks a vacant residue class and mints the id for it (wave B0) | `admit_typed_operation_slot` `:23807`, over `typed_operation_slot_is_vacant` `:23786` |
| vacancy spans FIVE fixed direct-mapped 64-entry tables | `tool_operations`, `typed_operation_reservations`, `latest_wins_commands`, `segmented_downloads`, `segmented_closures` |
| latest-wins verbs park in `latest_wins_commands` + hold `typed_operation_reservations[slot]` | `dispatch_typed_command_inner` `:23824` ff. (latest-wins branch) |
| every other verb mounts a `MountedTypedCommandFullOperation` in `tool_operations` | `start_typed_command_operation` |

Every one of puzzle3d's 66 actions takes this path (B0 §4), so **every action consumes one of 64 slots**.

### 1.2 The mounted operation's stage ladder

`MountedTypedCommandFullOperationStage` (`:16285`): `Worker → Publishing → AwaitingAck → Retiring`.
`acknowledge_result_page` `:16649` moves an ACKed `Terminal`/`Fault`/`Download` page to `Retiring`;
`retirement_step` then drains the operation's owners and only a `Complete` from it permits
`tool_operations.remove`, which is what frees the residue class.

### 1.3 Retirement — asynchronous, on a 24-stage rotation inside a pool job

Before this wave the `Retiring` stage had exactly **two** drivers, and neither is the turn driver:

| driver | site | pacing |
| --- | --- | --- |
| cooperative maintenance, stage **0 of `MAINTENANCE_STAGES = 24`** (`:24621`) | `maintenance_step` `:25215` ff. | ONE retirement unit per whole 24-stage rotation |
| the app close ladder (teardown only) | `close_typed_operation_step` `:20434` | only while the app is closing |

`maintenance_step` itself is not called by the reactor turn: it runs inside `RuntimeLiveCleanupJob`
(`:30314`), an `InteractiveJob` stepped once per turn by
`crate::plugin_runtime::plugin_step_live_cleanup(runtime)` (`⚛️reactor/🔄️turn/🦀️.rs:475`) — one *stage*,
not one retirement, per reactor turn.

### 1.4 The hole: the turn driver reports the operation runnable and advances nothing for it

`has_runnable_work` (`:16642`) answers **true** for `Retiring`. `has_runnable_typed_operations`
(`:25654` region) therefore answers true, `pending_typed_operation_instance` selects the instance, and
`⚛️reactor/🔄️turn/🦀️.rs:1057` calls `plugin_continue_typed_operations` →
`advance_typed_operation_output` → `advance_typed_operation_publication` →
`advance_typed_operation_publication_one` (`:23005`), which — pre-wave — read:

```rust
if … stage == Worker  { return self.drive_typed_operation_worker(operation_id); }
if !(… stage == Publishing) { return Ok(()); }   // ← Retiring falls out here, doing NOTHING
```

So a retiring operation was **runnable by the host's own predicate and un-advanceable by the host's own
driver**. That is simultaneously the slot leak AND the shape wave B2 measured from the host side
(`more-work`, `published nothing`, `effects=0`).

## 2 Per-action-class leak table

There is **no action class that leaks and none that does not** — the leak is per *operation*, not per verb:
every admitted typed operation parks in `Retiring` and is released only by the maintenance rotation.
Measured natively (§4, first FAIL) the leak appears on **action 0** of the storm, on the plain settled
outcome, with the census `["64:Retiring:presented=true"]`.

What the battery's distribution therefore reflects is *arrival rate*, not class:

| action id | refusals in `probe-2026-09-11T13-15-23.md` | why it dominates |
| --- | --- | --- |
| `engagementAbort` | 20 | fired by every engagement step and every abort in the fill/engagement sections |
| `setCamera` | 19 | one per camera gesture frame — the highest-rate verb in the battery |
| `openVortexSuggestions` | 2 | fired twice in `export-import` after saturation had already set in |
| (every other verb) | 0 refusals, but each one still leaked its slot | they simply arrive between the storms |

Timeline confirms it is cumulative rather than class-specific: the first saturation refusal is at
**t = 390.0 s** (`step clipboard-copy-paste`), ~200 s and several hundred actions into the run, and from
there every high-rate verb fails. `probe-…13-15-23.md:302, 333, 362, 442, 634, 670`.

Arithmetic for why it saturates: one reactor turn advances **one** maintenance stage, so stage 0 is
visited about once per 24 turns and releases **one** retirement unit; a mounted operation owes on the
order of ten such units (publication owner, child publication, captured child content, output chunks,
session, lease, …). That is ~240 reactor turns per released slot against one admitted slot per action.

## 3 Root cause

> **Admission is synchronous and per-action; release was deferred to a cadence that has nothing to do
> with admission, and the driver that reports the operation runnable could not run it.**

Concretely: `Retiring` had no branch in `advance_typed_operation_publication_one`, and the removal from
`tool_operations` was spelled out **three times** (maintenance stage 0, the close ladder, and nowhere on
the turn path) instead of once.

## 4 The fix — one release site, driven by the turn that reports the slot live

| file | change |
| --- | --- |
| `🔌️plugin/🦀️.rs:20386` | **new `VcsArtifactApp::retire_typed_operation_unit(operation_id, items, bytes)`** — the ONE release site. One bounded `retirement_step`, the terminal-empty witness, then the exact `tool_operations.remove`. Nothing else in the crate may remove a mounted typed operation. |
| `:20414` | **new `retire_typed_operation_run(operation_id)`** — a bounded RUN of those units for this turn, under the same `INTERACTIVE_TURN_WORKER_PUMPS` (256) / `INTERACTIVE_TURN_WORKER_WALL_US` (4 000 µs) slice `drive_typed_operation_worker` already gives a Worker-stage operation. Same ladder, same owner, same bound — B2's "bounded work per TURN, not one item per turn" applied to the last ladder that still had one-per-rotation pacing. |
| `:23019` (`advance_typed_operation_publication_one`) | new `Retiring` branch → `retire_typed_operation_run`. The driver that reports the operation runnable is now the driver that releases it. |
| `:20444` (`close_typed_operation_step`) | the `Retiring` arm now early-returns into `retire_typed_operation_unit`; the arm is `unreachable!()` in the stage match. |
| `:25217` (`maintenance_step` stage 0) | same — the rotation keeps its cadence but no longer owns a second copy of the removal. |
| `:23797` | **new `VcsArtifactApp::live_typed_operation_slots()`** — occupancy as the admitting authority counts it: the exact complement of what `admit_typed_operation_slot` can still hand out. |
| `:11624` | **new `PluginApp::live_typed_operation_slots()`** (defaulted `0`, so no other impl changes), forwarded at `:25654`. Diagnostics only — the host may read it, never branch on it. |
| `:32407` | **new `plugin_runtime::trace_typed_operation_slot_occupancy`**, called from `advance_typed_operation_output`. Prints `[DEBUG] typed-operation slots instance=N live=L/64 peak=P` on the same console channel as the existing `[DEBUG] maintenance stage=` line — **no wire, no protocol frame**. Every new peak and its first full drain print unconditionally (that pair IS the leak signal and is self-limiting at ≤ 2 lines per distinct peak, ≤ 128 per session); every intermediate transition prints only under `semio_framework_trace::runtime_diagnostics_enabled()`. |

No constant changed, no table grew, no fault was swallowed. `interactive-job.typed-operation-capacity`
still exists and still means true saturation.

### Every outcome now has exactly one release path

| outcome | route to the one site |
| --- | --- |
| settled | terminal page → ACK (`acknowledge_result_page:16649`) → `Retiring` → `retire_typed_operation_unit` |
| faulted | `Fault` lane page → ACK → `Retiring` → same site |
| cancelled | `reject_cancelled_publication:16606` mounts the terminal fault page → same |
| refused at ingress | no slot is taken — `admit_typed_operation_slot` only mints; every fallible step before the reservation write leaves the class vacant |
| replaced (latest-wins) | `advance_latest_wins_command_one` mounts the displaced pending as a terminal-fault operation and clears `typed_operation_reservations` → same site |

## 5 Laws

Bodies in `🔌️plugin/🦀️.rs`, entry points in
`🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` (the shape B0 used).

Both drive **only** the host's own continuation — `advance_typed_operation_publication`, the result page
for the bound receiver plus its mandatory ACK, then the effect/event/UI-scope/completion outboxes, unit
for unit as `plugin_runtime::advance_typed_operation_output` does — and deliberately call **no**
`maintenance_step`: a slot the host cannot release on that path alone is a leak whatever the cooperative
rotation would eventually have done about it. Shared helper `drive_host_typed_continuations` `:17449`.

1. `every_admitted_typed_operation_slot_is_released_by_the_host_continuation_that_reports_it_runnable`
   (body `test_typed_operation_slot_retirement_under_storm` `:17377`) — **200 actions**, 512 host
   continuations each, `live_typed_operation_slots() == 0` asserted after every single one.
2. `a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot`
   (body `test_typed_operation_slot_release_on_every_outcome` `:17409`) — one clause per outcome:
   settled; three admissions on ONE latest-wins key with no continuation between them; and an admitted
   command whose own cancellation lease is cancelled (the path a faulted/refused operation also takes).

**Written first, run at HEAD behaviour (the `Retiring` branch neutralised, everything else in place) —
both FAILED, on the FIRST action:**

```
running 2 tests
test …::a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot ... FAILED
test …::every_admitted_typed_operation_slot_is_released_by_the_host_continuation_that_reports_it_runnable ... FAILED

assertion `left == right` failed: a settled operation left 1 of 64 typed-operation slots live after 512
host continuations ["64:Retiring:presented=true"]
  left: 1
 right: 0

assertion `left == right` failed: action 0 left 1 of 64 typed-operation slots live after 512 host
continuations ["128:Retiring:presented=true"]; an admitted slot must have exactly one release path the
same turn driver walks
  left: 1
 right: 0

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 652 filtered out; finished in 0.04s
```

`64:Retiring:presented=true` is the whole defect in one string: the operation reached `Retiring`, the host
ACKed it, and 512 continuations later it still owned its slot.

**After the change:**

```
running 3 tests
test …::a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot ... ok
test …::every_admitted_typed_operation_slot_is_released_by_the_host_continuation_that_reports_it_runnable ... ok
test …::typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 651 filtered out; finished in 0.40s
```

with `[DEBUG] actual 200 storm actions each released their typed-operation slot within 512 host
continuations, peak occupancy 0`.

### 5.1 A process-global teardown the laws owe the binary

`close_fixture_app_to_terminal_emptiness` `:17483` walks the cooperative rotation once (bounded by
`worker_job_retirements_are_parked()`) **before** the close ladder. The worker-session retirements a
200-action storm parks are PROCESS-global; leaving them behind made
`a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` measure 82 B/turn against its 64 B
ceiling and abort the whole binary (that test panics before `reactor_native_lifecycle_finish`, so
`NativeLifecycleRegistry::drop`'s assert fires during unwinding → `SIGABRT`). With the teardown the trio
passes; see §6 for the residual flake.

## 6 Verification (foreground, tails quoted)

| command | result |
| --- | --- |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1 typed_operation_slot release_their_exact_slot typed_operation_ingress_pre_admits` | **3 passed / 0 failed** (tail in §5) — includes B0's law |
| the same with the fix neutralised | **0 passed / 2 failed** (tail in §5) |
| `… --test-threads=1 typed_operation_ingress_pre_admits reserved_undo artifact_fixed_registry` | `test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 639 filtered out; finished in 0.06s` — B0's law and the whole `reserved_undo` set |
| `… --test-threads=1` (whole crate, with the wave) | `test result: FAILED. 508 passed; 146 failed; 0 ignored; 0 measured; 0 filtered out; finished in 36.76s` |
| the same with the fix neutralised (baseline) | `test result: FAILED. 506 passed; 148 failed` |
| the same skipping only this wave's two laws | `test result: FAILED. 506 passed; 146 failed; 0 ignored; 0 measured; 2 filtered out; finished in 159.39s` |
| **failure-name diff, wave vs baseline** | `regressions: (empty)` / `fixed: the two new laws` — **zero regressions** |
| **failure-name diff, wave vs skip-mine** | **identical sets** — the new laws cause no collateral |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: semio-s-artifact-puzzle-3d (lib) generated 88 warnings …` / `Finished dev profile [unoptimized] target(s) in 13.68s` — **0 errors** |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Checking semio-s-plugin-puzzle v0.1.0 …` / `Finished dev profile [unoptimized] target(s) in 23.62s` — **0 errors**, guest target |

The 146 pre-existing failures are the peer refactor B0 §5 and B2 §4 both recorded, unchanged in
membership: dominated by `interactive-job.missing-factory: typed command '…' has no exact
controller/owner/factory/tool/schema proof` (≈ 30), `app-definition.invalid: app id testkit-txn must be a
canonical surface id` (13) and the `artifact store reached Drop without its exact terminal-empty
shallow-shell witness` (18) those two cause downstream. The wasm component build was NOT run (coordinator
owns it).

Raw logs: `🗑️generated/wave-B8-plugin-lib-final2.txt` (with wave),
`wave-B8-plugin-lib-baseline.txt` (fix neutralised), `wave-B8-plugin-lib-skipmine.txt` (laws skipped),
`wave-B8-plugin-lib-full.txt` and `wave-B8-plugin-lib-final.txt` (earlier passes).

## 7 `AliasCapacity` and `no exact pending spawn slot`

### 7.1 `1:framework.panel.history: AliasCapacity` — **the same family, a different table; NOT fixed here**

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📃️document.rs:642` `alias()` caps a
`UiDocumentLease` at `UI_DOCUMENT_LEASE_ALIASES = 8` (`:447`) simultaneous aliases per document slot.
Acquisition is synchronous. **Release is deferred**: `UiDocumentLease::drop` (`:783`) does not decrement —
it records a `UiArenaHandback::ReleaseAlias` obligation
(`🖱️ui/🧬️contract/♻️retirement/🌳️typed/📃️document.rs:179`), and the decrement happens later in
`close_ui_document_page_with_grant` (`:190`, `consume_handback:10` in the same file), driven from the reactor turn's close
ladder (`⚛️reactor/🔄️turn/🦀️.rs:453`). That is structurally the same defect shape as this wave's — a
fixed capacity whose acquire is immediate and whose release rides a per-turn retirement ladder.

Two differences matter, and they are why it is out of B8's scope and not fixed by this change:
- the obligation registry is a **counter**, not a flag (`UiArenaHandbacks::record` `fetch_add`,
  `♻️retirement/📮️handback/🦀️.rs:25`), so no release is ever lost — this is a rate/concurrency problem,
  not a permanent leak;
- the cap is **8, not 64**, and it fired at t = 192.7 s on `translateSelection`, the battery's FIRST hard
  fault, long before any typed-operation saturation. Eight simultaneous live aliases of one document slot
  is plausibly reached *within a single turn* (the history panel re-reading its document), which no
  retirement pacing can fix.

**Recommendation:** a separate wave should count the concurrent `try_alias` holders of
`framework.panel.history` in one turn. Predicted independent of this wave's browser result.

### 7.2 `framework route 'interactionHover'/'interactionSelect' has no exact pending spawn slot` — **a different family, ALREADY fixed at HEAD**

That message no longer exists in the tree. `pending_reserved` is a fourth direct-mapped 64-entry table for
framework-reserved **Isolated** jobs; it had B0's *mint-then-test* defect, and a peer already converted it
to probe-then-admit at `:22527` ff. with a latest-wins pre-retirement
(`retire_pending_reserved_latest_wins:22475`) and a new message,
`framework route '…' found no vacant pending spawn slot in 64 residue classes` (`:22541`). The comment
there names this ticket and "battery #44-pre", i.e. that fix landed **after** the wasm the 13-15-23 probe
ran. Its release path is the host completing the Isolated job (`complete_reserved_spawned_job`), which is
not the typed-operation ladder, so B8 changes nothing about it.

## 8 What the probe should assert on #45

1. **No `every fixed typed-operation and segmented-output slot already owns a live operation` at all.**
   39 occurrences → 0 is the primary verdict. If even one appears, the leak is not closed.
2. **`[DEBUG] typed-operation slots … live=0/64 peak=P` after each settle.** The new console line prints
   unconditionally on each new peak and on the first full drain after it, so the battery should show a
   small `peak` (single digits under the hover/camera storms) and a `live=0` drain line after it — never a
   monotonically rising `peak` with no drain. Assert the LAST such line reads `live=0`.
   With `SEMIO_RUNTIME_DIAGNOSTICS=1` armed in the guest the same line prints on every transition, which
   turns the assertion into a full occupancy trace; the puzzle plugin does not arm it today (procedural
   does, `✏️s/🔌️plugins/🌀️procedural/🦀️.rs:82`), so the unconditional peak/drain pair is what the probe
   gets for free.
3. **The ~19 collateral `setCamera` / 20 `engagementAbort` step FAILs should disappear with it**, since
   B0 §6 and the timeline both say the post-390 s failures are downstream of saturation. Expect
   `hard=45` to drop by roughly 39 + collateral.
4. **`AliasCapacity` is expected to REMAIN** (§7.1) — do not read it as this wave failing. It should still
   be the first hard fault, at ~190 s, on `translateSelection`.
5. **`… has no exact pending spawn slot` is expected to disappear** (§7.2) purely because #45 is the first
   wasm containing the peer's probe-then-admit fix; that is not B8's doing.

## 9 Remaining risks

- **The browser has not been re-measured.** This is guest Rust only; nothing changes until
  `component-release` is rebuilt AND materialised. The coordinator owns that run.
- **A retiring operation may now hold up to 4 ms of a turn.** `retire_typed_operation_run` takes the same
  slice a Worker-stage operation already takes, and only one instance is advanced per turn, so the turn's
  worst case is unchanged in shape — but a document whose retirement ladder is genuinely deep will now
  spend that slice retiring instead of returning to the host early. That is the intent (B2's pricing), and
  it is what removes the ~240-turn-per-slot pacing.
- **`a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` is flaky under a full-crate run.**
  It asserts ≤ 64 B retained per settled reactor turn, measured against the heap of the whole preceding
  shared test process; it was observed at 82 B and 128 B in 2 of 5 full runs and at "ok" in the other 3,
  including a run with this wave's laws skipped and a run with them present. When it fails it takes the
  whole binary down with `SIGABRT` (§5.1). This wave's laws raise the process's allocator pressure and so
  make the flake more likely; the ceiling itself is a pre-existing order-dependent measurement (cf.
  `📓️2026-09-10-order-dependent-tests-audit.md`). Worth its own wave.
- **`live_typed_operation_slots` is defaulted to `0` on `PluginApp`.** Only `VcsArtifactApp` reports a real
  value; any future app type that owns its own typed-operation table must override it or the diagnostic
  will under-report for that app.
- **The trace statics are process-global**, so `peak` is the maximum across every instance in the guest,
  not per instance. Deliberate: the table it reports is per app, but the leak signal is a process-level
  property and one line per peak is the point.

## 10 Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `retire_typed_operation_unit`,
  `retire_typed_operation_run`, the `Retiring` branch in `advance_typed_operation_publication_one`, the two
  rewired removal sites (`close_typed_operation_step`, `maintenance_step` stage 0),
  `VcsArtifactApp::live_typed_operation_slots`, `PluginApp::live_typed_operation_slots` + its impl,
  `plugin_runtime::trace_typed_operation_slot_occupancy` and its two statics, and the two law bodies with
  their helpers (`admit_fixture_typed_action`, `drive_host_typed_continuations`, `released_slot_census`,
  `close_fixture_app_to_terminal_emptiness`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
  — the two law entry points.
- this report.

No temporary `[DEBUG]` logging was left in production code. The `[DEBUG] ` prefixes that remain are the two
laws' own measurement lines and the permanent occupancy diagnostic, which matches the prefix the existing
`[DEBUG] maintenance stage=` diagnostic in the same module already uses. `🗑️generated` was written to and
never deleted.
