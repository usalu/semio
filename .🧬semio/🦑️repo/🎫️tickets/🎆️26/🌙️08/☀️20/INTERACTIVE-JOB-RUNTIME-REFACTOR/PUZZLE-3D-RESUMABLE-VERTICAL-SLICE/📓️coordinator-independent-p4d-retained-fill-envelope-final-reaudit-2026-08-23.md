# Coordinator Independent P4d Retained Fill-envelope Final Re-audit — 2026-08-23

## Verdict

**RED — rejected for source correctness.** The repaired packet improves the live route substantially,
but the claimed measured admission and terminal retirement guarantees are still false. No Cargo, Nx,
Wasm, browser, or runtime gate was run during this audit.

## Audited Scope

- `✏️editor/⏳️precompute/🦀️component.rs`
- `✏️editor/⏳️precompute/🪣️fill/🦀️component.rs`
- `✏️editor/⏳️precompute/📐️geometry/🦀️component.rs`
- both mounted `fillBuildTick` callers in
  `✏️editor/🎮️commands/🪣️fill-build-tick/🦀️component.rs`
- the updated implementation report and permanent-verifier claims

Read-only evidence used `git diff`, `rg`, numbered source excerpts, and schema/type inspection. The
author's formatting and Bun verifier results were not rerun while other Rust source packets remained
active.

## Blocking Findings

### P4d-R1 — owner admission is heuristic, not a measured exact census

`FillBuilderOwnerCensusCursor::charge` computes `capacity * size_of::<T>()` plus a guessed 16 KiB per
`semantic_owners` count (`fill/🦀️component.rs:291-301`). The field cases then hard-code multipliers such
as eight owners per `FixtureObject` and twelve per `ObjectKind` (`:307-384`). Those values are not the
owned allocation count or byte capacity. For example, one `FixtureObject` can own an arbitrarily long
`vortices` vector and its nested strings; one `ObjectKind` can own arbitrarily many representations,
tags, vortex templates, `DslValue` allocations, and strings. `BTreeMap`, `BTreeSet`, and `HashMap`
cases pass `.len()` as though it were allocation capacity, and `CollisionSpatialIndex` is charged as
one fixed owner despite its entries, cell map, per-cell vectors, and oversized set.

Consequently an owner graph larger than four MiB can receive a sub-four-MiB guessed credit and enter
the registry. This does not meet pre-admission byte/item accounting or the cap/+1 requirement. The
repair must use an exact resumable allocation/owner census, or replace the dynamic graph with a
schema-first fixed/paged owner whose credits are exact by construction. Literal per-type multipliers
are not acceptable evidence.

### P4d-R2 — the retirement cursor still performs unbounded nested and multi-owner drops

`FillBuilderRetirementCursor::retire_one` pops whole `FixtureObject`, `ObjectKind`, catalog, collision,
candidate, preview, and mesh values (`fill/🦀️component.rs:405-443`). Each popped value can recursively
destroy dynamic nested vectors/maps/strings in that single grant. After the visible elements are
removed, the `_` arm calls `self.fill.take()` (`:444-446`), which drops every still-allocated empty
container backing store and every omitted owned field together. `close_step` then removes the cursor
again in the same grant (`precompute/🦀️component.rs:466-469`).

The result is neither one exact owner nor one bounded page per close grant. The repair needs nested
retirement state for every dynamic field and must separately release each container backing/root;
terminal empty must be witnessed before the `FillBuilder` shell is removed.

### P4d-R3 — terminalization is lossy under contention and misses a completed-session drop race

`terminalize_fill_envelope` uses `try_lock` and silently returns when the registry is momentarily
contended (`precompute/🦀️component.rs:292-302`). Both the worker fault guard and session `Drop` rely on
that function (`:319-323`, `:1484-1489`), so either can lose the only terminal/close transition and
leave the fixed slot admitted forever.

It also changes only `Admitted` authorities. If the worker has already set Complete/Cancelled/Fault
and the mounted session is dropped before `poll_fill_job` takes the terminal handle, session `Drop`
does not mark it Closed. The replacement mounted session's orphan path searches only
`Terminal(Closed)`, so the completed authority is unreachable and its credits never return.

Terminal intent must be retained independently of the contended registry and retried by a mounted
wake/pump. Session drop must make every exact live generation orphan-closeable regardless of whether
the worker already published a terminal reason.

### P4d-R4 — pre-admission and rejected owners have no retained close path

During the fourteen-turn census, the exact builder lives only in `fill_admission`. Rejection restores
it to `engine.fill` (`precompute/🦀️component.rs:1301-1323`), and registry contention/cap rejection does
the same (`:1327-1338`). `Puzzle3dPrecomputeSession::drop` handles only `fill_job`; it does not transfer
`fill_admission`, an unadmitted `engine.fill`, or a rejected builder into a resumable retirement owner
(`:1484-1490`). Dropping the mounted session at any of those points can therefore deep-drop the entire
builder graph on that thread.

All admitted, admission-in-progress, rejected, cancelled, and session-close paths need the same exact
retained retirement authority and terminal-empty witness.

### P4d-R5 — malformed input can fault before the exact guard exists

`fill_job` constructs `FillEnvelopeWorkerFaultGuard` only after the full 56-byte input token has been
decoded (`precompute/🦀️component.rs:1494-1505`). Invalid length/header/field decode returns a job fault
before any guard can make the already-reserved registry owner terminal. A fixed transport token copied
from the registry lowers the probability but does not close the required corruption/fault path. The
job must receive or derive an exact registry identity independently of fallible payload decode, or
the transport must retain a separate submission-fault authority keyed by the job identity.

## Verifier Gap

The expanded permanent verifier passes because its new mutations check for the presence of a census,
mounted pump, guard, and retirement cursor. It does not discriminate exact measured ownership from
the hard-coded multipliers, recursively owned element pops from shallow retirement, a lost `try_lock`
terminal intent, the already-terminal session-drop race, admission/rejection close, or pre-guard
decode fault. Each blocker above needs a faithful mutation/fixture before this packet can be
source-accepted.

## Acceptance Required for Re-audit

1. Exact bounded item/byte/page accounting of the complete retained builder graph before transfer.
2. Nested one-owner/one-page retirement, including container backing stores and terminal-empty shell.
3. Contention-safe retained terminal intent and the completed-before-session-drop orphan race.
4. Retained close for admission-in-progress and rejected owners.
5. Exact fault ownership before fallible token decode.
6. Discriminating source fixtures/mutations for all five cases.

Phase 4 and P4d remain open.
