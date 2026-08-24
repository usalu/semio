# Terra Phase 6 P6h Fresh Acceptance Audit

Date: 2026-08-24  
Auditor: Terra `/root/p6h_fresh_acceptance`  
Scope: live working tree; read-only source audit and isolated verifier/format/diff checks.

## Verdict

**RED — do not accept P6h or Phase 6.**

The P6h verifier reports success, but its positive predicate is a presence/substring check and does
not inspect the executed numerical, checkpoint, mesh, or law bodies. Live source still admits
multiple hidden multi-unit loops and post-work fuel accounting. The claimed 34 mutations therefore
cannot establish the governing one-opportunity contract.

## Governing Inputs Read

- Root and FEM `AGENTS.md`.
- Master Phase 6 and residual/status material.
- P6g accepted third-remediation report and the earlier P6g audit.
- `📓️p6h-fem-numerical-microcursor-repair-contract-2026-08-24.md`.
- `📓️sol-high-p6h-fem-numerical-microcursor-implementation-2026-08-24.md`.

The master gate requires stale edits to cancel immediately, bounded coarse/final work, numerical
reference tolerances, and no step at or above 8 ms. P6h further requires fuel before exactly one
semantic scalar/entry/cell/page opportunity, no fall-through, retained page checkpoint/restore,
and substantive hostile laws.

## Confirmed Retained Work

- LDLT has the required explicit column stages and performs the live numerical state-machine step
  after `should_yield` and `consume_fuel(1)`: `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️component.rs:370-381`, `:1303-1416`.
- Subspace has the required named stages, incremental sort cursor, generation/revision/seed fields,
  and retained current/retiring work: sparse `:2252-2308`, `:3226-3246`, `:3780-4014`.
- `FEMLCP1` and `FEMSCP1` use versioned page tags and their restore cursors validate identity and
  page field ordering: sparse `:1034-1203`, `:3574-3682`. Partial payload close is explicit in both
  restore cursors: sparse `:1261-1300`, `:3731-3776`.
- Constraint recovery retains the required stages and updates the fixed edge authority rather than
  calling the old `recover_constraint` route: `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️component.rs:660-721`, `:1102-1221`.
- The mounted assembly route uses `AssemblyJobConstruction::new_owned`, and `Bar2`, `BeamEb2`, and
  `Tri3Cst` implement fixed-schema `mounted_stiffness_cell` rather than invoking whole matrices in
  the owned element path: session `:1209-1224`; analyses `:1546-1608`; elements `:85-96`, `:341-365`,
  `:611-665`.
- No scoped source file is deleted by the live working-tree diff; P6g's retained session path
  remains present. This is only a no-deletion observation, not a replacement for P6g acceptance.

## Blocking Findings

### 1. LDLT and Subspace checkpoint publication hide page-sized loops behind one fuel grant

`advance_u32_owner`, `advance_u64_owner`, `advance_f64_owner`, `advance_pair_owner`, and
`advance_matrix_owner` each loop until a whole 16 KiB page is filled. The live `LdltJob::step` and
`SubspaceIterationJob::step` consume one fuel unit, call `advance_checkpoint_page`, and therefore
write up to thousands of scalars/entries in that single admitted opportunity.

- Loop helpers: sparse `:414-458`.
- LDLT live call: sparse `:1314-1352`.
- Subspace live call: sparse `:3790-3828`.

This violates the contract's no-hidden-loop and one field/page-entry per grant language. A page is
retained, but serializing every entry that fits in it is not one semantic entry. The same defect
exists for terminal/preview publication: `advance_output_publication` and
`advance_preview_publication` loop over page remaining capacity before the caller yields (sparse
`:830-899`, `:3339-3392`).

**Required repair:** retain a page plus field/owner/item cursor and write exactly one scalar/pair
per step; preserve a partially initialized page as an authority until it is committed or closed.
Perform one fuel/deadline/cancel check before that entry and return immediately.

### 2. Restore decodes a whole retained page in one grant

The restore path copies an entire payload page into a contiguous stack buffer, then the owner
restore helpers loop over every scalar/pair in the page. One `LdltRestoreCursor::step` or
`SubspaceRestoreCursor::step` has only one fuel unit.

- Whole-page copy: sparse `:1250-1257`, `:3720-3727`.
- Scalar/pair restore loops: sparse `:1013-1087` and matrix loop `:3466-3489`.

The payload itself is page-retained, but the decoder is still a whole-page contiguous copy plus a
hidden entry loop. This fails the requested checkpoint/restore microcursor and no-contiguous-buffer
gate.

**Required repair:** decode directly from the retained page source with a persistent byte/item
cursor; append/admit one scalar or pair per grant. Retain the source page and partial target owner
until exact close. Reject truncation/identity/version before each affected admission, not only after
the page copy.

### 3. Mounted mesh still batches and charges fuel after work

The live bounded mesh path processes multiple preparation and insertion units in one call:

- `PrepareInput` loops `0..MESH_JOB_UNIT_BATCH`, calls `advance(...)`, and only then consumes fuel:
  mesh `:1295-1322`.
- `InsertBoundary` loops while `units < MESH_JOB_INSERT_BATCH`, calls `insert_next()`, and only then
  consumes fuel: mesh `:1373-1393`.

`StepContext` explicitly documents `consume_fuel` as post-work and `should_yield` as the only
fuel/deadline predicate: `🧰️framework/🔨️modules/🧵️job/🦀️component.rs:713-725`. Thus these blocks
can perform several semantic units on one opportunity and are contrary to the P6h contract's
pre-consume/no-fall-through rule.

Additional model-sized work remains in the mounted mesh step: `Initialize` sums all hole lengths
in one grant (`mesh :1324-1351`), while `append_face` scans all holes and the complete point index
for one face (`:985-1009`). These are live from `MeshJob::new_bounded` in the mounted session
(`session :1146`).

**Required repair:** remove both batch loops; consume/admit one unit before each retained
preparation/insertion action and return. Add retained cursors for input-size accounting, hole
classification, and point-index lookup, or prove fixed maxima with a tested timing bound.

### 4. P6h mesh and element laws are decorative, not execution-path hostile tests

The mesh law directly invokes `advance_constraint_recovery()` without a `StepContext`; deadline,
stale, and cancellation assertions happen only after the constraint has already completed.

- Direct unbudgeted recovery: mesh `:2337-2343`.
- Deadline/stale/cancel checks after it: mesh `:2361-2375`.

The named element law builds a borrowed `AssemblyJob::new` (`analyses :2697-2699`). The job's live
dispatch chooses `begin_borrowed_element()` for borrowed models, whereas the mounted cell cursor
only executes under `AnalysisModelOwner::Owned`.

- Dispatch split: analyses `:1956-1969`.
- The unbounded borrowed helper calls `element.stiffness_global`: analyses `:1614-1625`.
- The named law merely finds stage-name strings, then steps the borrowed job: analyses `:2687-2719`.

Consequently it neither interrupts every mounted stiffness stage nor proves the claimed fixed-family
timing exception. Its max+1/numerical parity test in `elements2d` validates cell values but not the
mounted job's cancellation/publication/close execution.

**Required repair:** drive `MeshJob::step` under one-fuel contexts and interrupt/cancel/stale it at
every recorded constraint stage. Build `AssemblyJobConstruction::new_owned`, advance it to its
owned `AssemblyJob`, then interrupt/deadline/cancel every stiffness stage for Bar2, BeamEb2, and
Tri3Cst. Add actual timing measurements for their fixed cell kernels and maximum+1 working-set
admission, not source-string ordering assertions.

### 5. The isolated 34-mutation verifier does not test its advertised live invariants

`toolJobFemNumericalMicrocursorExact` uses `includes`, stage-name occurrence counts, and only
extracts the outer LDLT/subspace `InteractiveJob::step` blocks. It does not inspect the helper
bodies where the page loops occur, the mesh `PrepareInput`/`InsertBoundary` batches, restore
decoding, or whether a law exercises the owned route.

- Predicate: `📜️script.ts:3587-3709`.
- The 34 mutations are synthetic strings rather than mutations of parsed live source/AST behavior:
  `📜️script.ts:3713-3791`.

It therefore accepts the current violations and cannot satisfy the contract's faithful-mutation
gate. In particular, no mutation restores an `advance_*_owner` full-page loop, post-work mesh
consume, or the borrowed-only element law.

**Required repair:** make the verifier extract and validate the actual helper and mounted-session
blocks (prefer a parser/span-aware check). Add faithful live-source mutations for each finding,
retain/execute all 34 prior mutations, and add discriminating tests that fail with each mutation.

## Gate Evidence

| Check | Result | Evidence |
| --- | --- | --- |
| Focused P6h verifier/self-test | Pass, insufficient | `bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test` emitted `live-source clean; hostile-mutations=34.` |
| Scoped rustfmt check | Pass | `rustfmt --edition 2021 --check --config skip_children=true` over the six FEM/session sources and retained job runtime exited 0. |
| Scoped diff whitespace check | Pass | `git diff --check` over the P6h source/verifier set exited 0. |
| Exact one-unit source gate | **Fail** | Findings 1–4. |
| Faithful mutation/law gate | **Fail** | Finding 5 and decorative law evidence. |
| Broad compile/test/Wasm/browser/runtime/timing matrix | Not run | Excluded by the requested isolated-audit scope. |

## Acceptance Disposition

P6h must remain RED until all five repairs land and are independently re-audited. The currently
passing formatter/diff/self-test checks do not override the live boundedness failures. No source
files were changed by this audit.
