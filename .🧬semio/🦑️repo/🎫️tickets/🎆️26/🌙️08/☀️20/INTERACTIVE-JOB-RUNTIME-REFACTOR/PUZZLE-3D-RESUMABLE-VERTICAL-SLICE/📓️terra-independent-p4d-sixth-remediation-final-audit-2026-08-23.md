# Terra Independent P4d Sixth-remediation Final Audit — 2026-08-23

## Verdict

**RED — rejected for source correctness.** R6 correctly replaces the decorative backing tokens with
the actual fixed slot storage, but two reachable P4d lifecycle paths can still destroy or strand
the credited actual owners outside the retained close cursor. P4e residuals are not being rejected
in isolation; both findings are reachable after a P4d envelope has admitted the same `FillBuilder`.

No Cargo, Nx, Wasm, browser, runtime, or network validation was run while Rust sources were being
edited concurrently. The scoped `git diff --check` for the three audited production files was clean.

## Evidence Read

- `AGENTS.md`
- the attached interactivity-first refactor plan
- `📓️p4d-retained-fill-worker-envelope-implementation-2026-08-23.md`
- `📓️coordinator-independent-p4d-retained-fill-envelope-final-reaudit-2026-08-23.md`
- `📓️coordinator-independent-p4d-second-remediation-final-reaudit-2026-08-23.md`
- current Puzzle 3D precompute, fill, geometry, action reachability, and the permanent verifier

## R6 Storage Findings That Are Now Correct

- All ten retained `FillBuilder` collection authorities and all three `CollisionSpatialIndex`
  authorities use `FixedOwnerMap`/`FixedOwnerSet`; none of these thirteen live fields is an
  ordinary standard map/set.
- `FixedOwnerMap` owns its actual entry/control page as
  `Option<Box<[Option<(K, V)>; N]>>`. Every audited live monomorph uses the default
  `N = FIXED_OWNER_SLOTS = 32`; `page_bytes()` credits the layout of that exact page, rather than a
  decorative byte token or a pair-size estimate.
- `try_insert` checks equality before capacity and returns the untouched distinct input key/value
  through `FixedOwnerMapInsert::Occupied`; capacity + 1 returns the exact `(K, V)` in `Err`.
  `remove_entry` returns the stored `(K, V)` pair. The direct boundary fixtures check pointer
  identity and stored-pair preservation.
- The retained close cursor drains semantic entries before
  `retire_fixed_collection_backing`; its `||` chain retires one actual FillBuilder page per close
  grant. `CollisionSpatialIndex::retire_one_owner` likewise drains semantic owners and then each of
  its three actual pages one at a time. The terminal witness requires all pages to be absent.
- There is no `Clone` implementation on either fixed owner type. The `try_from_btree`,
  `cloned_btree`, checkpoint, constructor, and direct rebuild functions remain identifiable P4e
  residuals. They would not by themselves be a P4d finding if disconnected from an admitted owner.

## Blocking Findings

### P4d-R7 — reachable replanning/rebuild paths directly clear or drop credited actual pages after admission

`Puzzle3dPrecomputeSession::enqueue_fill_job` first transfers the exact builder into the registry
with `begin_measurement`, then assigns the **same** `Arc<Mutex<FillBuilder>>` back to
`self.engine.fill` (`precompute/🦀️component.rs:1446-1455`). Therefore the normal session and the
admitted envelope concurrently retain handles to one builder; this is not a pre-admission P4e-only
object.

The public `UpdateKindWeights` path calls `soft_replan_fill_tail` and then `refresh_fill_job`
(`precompute/🦀️component.rs:803-814`) while that shared handle remains usable. The first locks the
admitted builder and invokes `candidate_cache.clear_for_rebuild_residual()` (`:759-775`). The latter
locks the same builder and calls `restart_search()` and `configure()` (`:777-800`).

Those functions bypass the retained terminal cursor:

- `reset_candidate`/`reset_candidate_preparation` call
  `clear_for_rebuild_residual` on `blocked_vortex_ids`, `candidate_seen`, `candidate_cross`, and
  `candidate_same` (`fill/🦀️component.rs:3152-3193`). The helper performs up to 32 pops and Rust
  `drop(entry)` calls in one synchronous call (`geometry/🦀️component.rs:146-152`), including nested
  values.
- `configure` assigns `self.weights = weights` (`fill/🦀️component.rs:2343-2362`), so replacing a
  populated pair of credited fixed pages invokes ordinary Rust drop rather than the one-page close
  authority.
- the mesh-refresh variant assigns `self.meshes = retained` and calls `rebuild_collision_index`
  (`fill/🦀️component.rs:2365-2383`). Rebuild clears `placed_lookup` and overwrites
  `self.spatial_index` with a new index (`:2573-2597`), so an ordinary `Drop` releases a populated
  spatial index, including its actual three fixed pages and retained cell vectors.

This invalidates P4d's admission credit and its claimed only-close retirement contract: the
registry retains the old item/byte reservation while credited entries/pages can be synchronously
cleared, replaced, or dropped without the terminal witness. The direct `CollisionSpatialIndex::remove`
bulk drops are P4e spatial-operation residuals and are not needed for this finding; the reachable
shared-owner rebuild replacement already proves it.

Required repair: an admitted builder must be exclusively envelope-owned until its resumable close
finishes, or all refresh/replan/rebuild requests must become retained, generation-keyed terminal or
replacement authorities that preserve one-owner/one-page close. A cancellation request alone is
insufficient while the old authority still holds the reservation. Add a fixture that admits a
populated builder, invokes both weight and mesh refresh, and proves that no fixed page or semantic
owner is cleared/dropped before a mounted retained close returns the exact credit.

### P4d-R8 — dropping a mounted session during `Closing` strands the partial retirement forever

`pump_fill_terminal_step` stores a `FillEnvelopeTerminalHandle` in `self.fill_terminal` and then
advances `close_step` (`precompute/🦀️component.rs:1503-1527`). The first close grant changes the
authority to `Closing` and moves the builder into `FillBuilderRetirementCursor`
(`:563-596`). This is deliberately partial, as the source fixture confirms.

If the session is then dropped, `Puzzle3dPrecomputeSession::drop` merely cancels and requests
`Closed` for `fill_job` (`:1623-1629`); dropping its `fill_terminal` invokes
`FillEnvelopeTerminalHandle::drop`, which only clears `checked_out` (`:633-639`).
`apply_fill_envelope_terminal_intent` explicitly refuses to change a `Closing` authority
(`:190-202`), `take_terminal_fill_job` accepts only `Terminal(_)` (`:1530-1540`), and the orphan
pump's `take_closed` accepts only `Terminal(Closed)` (`:385-397`). Thus a new mounted session cannot
reacquire or resume the partial cursor. The authority remains in the fixed registry holding its
semantic owners, actual pages, and reserved credit; its terminal witness can never become true.

Required repair: terminal-handle/session handback must preserve a resumable `Closing` authority and
allow the next mounted pump to reclaim that exact generation, or session drop must atomically hand
the handle to a durable close queue. Add a discriminating fixture/mutation that drops a session
after at least one `close_step` has populated the retirement cursor, mounts a new session, and
requires eventual terminal emptiness plus slot/credit rearm.

## Verifier Fidelity

The permanent predicate correctly denies R6 regressions such as decorative pages, a standard live
collection, 33 slots, missing page credit, occupied-owner erasure, value-only removal, restored
`Clone`, bulk backing release, and missing direct page fixtures. It does not mutate either actual
blocking path above: it does not force an admitted shared builder through weight/mesh refresh, and
it does not drop a session with a `Closing` terminal handle. Consequently the current baseline and
self-test can remain green while both production lifecycle defects remain.

## Scope Boundary

The residual checkpoint serialization/conversion and constructor-spatial work remains P4e in the
abstract. This audit treats it as P4d only where the admitted P4d envelope leaves its exact shared
builder reachable by the live refresh/replan calls described in P4d-R7. The separate direct spatial
`upsert`/`remove` operation work is not used as a P4d blocker here.

P4d and Phase 4 remain open.
