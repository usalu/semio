# Sol High P6h FEM Numerical Microcursor Implementation

Date: 2026-08-24  
Owner: `/root/p6h_fem_numerical_microcursors`  
Contract: `📓️p6h-fem-numerical-microcursor-repair-contract-2026-08-24.md`  
Source verdict: **SOURCE-AUDIT-READY**

## Outcome

The mounted FEM numerical route now advances LDLT, subspace iteration, constraint recovery, and local element stiffness through persistent bounded cursors. Every live numerical grant checks cancellation, operation/generation, deadline/fuel, consumes fuel before one scalar/entry/cell/page opportunity, and returns without a model-sized helper call.

LDLT and Subspace checkpoint/result publication no longer use contiguous serde or model-sized byte buffers. Their versioned binary schemas write directly into retained 16 KiB pages, reconstruct one tagged owner per restore grant after validating identity/version/field order/count/capacity, and retain partial source/state authority until exact incremental close.

## Changed Files

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗟️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️session/🦀️component.rs`
- `🧰️framework/🔨️modules/🧵️job/🦀️component.rs`
- `📜️script.ts`, exact `🧮️P6hFemNumericalMicrocursors` region

## Numerical Cursor Table

| Area | Retained stages | One admitted opportunity |
|---|---|---|
| LDLT | ReserveColumn, SourceEntry, ContributorLookup, ContributorEntry, PivotRead, DiagonalCommit, EmitRow, PublishColumn, CompleteColumn | one owner admission, source/contributor entry, pivot scalar, row candidate, column entry transfer, or retained page |
| Subspace | ReserveIteration, operator/factor forward/diagonal/backward, orthogonalize, normalize, projected cell, Jacobi search/rotate/converge, incremental mode sort/permutation, residual, convergence, publish | one matrix/vector owner admission or initialization scalar, sparse entry, dense cell, comparison, residual scalar, owner swap, or retained page |
| Constraint recovery | ReserveConstraintWorkspace, IndexTriangleEdge, SearchConstraintEdge, ClassifyIntersection, SelectDeterministicFlip, ValidateFlip, ApplyFlip, RetireFormerEdge, PublishConstraintProgress, ConstraintComplete | one edge index/search/classification/adjacency update/flip transition |
| Element stiffness | reference point, derivative, Jacobian, determinant/inverse, strain, constitutive, stiffness multiply, load, triplet, candidate publication | one fixed-schema scalar/cell/triplet; Bar2, BeamEb2, and Tri3Cst cells match batch kernels |
| Checkpoint restore | DecodePage, CloseDecodedPage, Complete or FaultClose | one 16 KiB field page decode/admission or one exact retained page/owner close |

## Checkpoint Schemas

| Schema | Tag | Identity | Owner fields | Publication |
|---|---|---|---|---|
| LDLT | `FEMLCP1\0`, version 1, kind 11 | operation, revision, generation, seed | Csc colptr/rowind/values, incremental L columns/D/row lists, cursor, accumulator values/marks/generation/candidate | direct retained CheckpointState pages; `FEMLDL2\0` direct CommitOutput pages |
| Subspace | `FEMSCP1\0`, version 1, kind 12 | operation, revision, generation, seed | LDLT factor, Csr indptr/indices/values, basis, theta/residual vectors, complete current and retiring SubspaceWork matrices/vectors/cursors | direct retained CheckpointState pages; `FEMSUB2\0` direct Preview and CommitOutput pages |

Each page carries version, kind, field tag, owner cursor, item cursor, and declared dimensions/count. Restore rejects version, field order, truncation, stale identity, maximum +1, multiplication overflow, or observed-capacity excess before owner admission. A faulted restore retains its payload and partial state for `close_step`; no compatibility or contiguous checkpoint adapter remains for LDLT/Subspace.

## Ownership Ledger

- Numerical child owners are capped at 16 KiB observed capacity. LDLT and Subspace declared order maxima are 40; Subspace basis columns remain capped at 40. Dense 40×40 f64 owners and worst admitted sparse scalar owners fit one credited owner page.
- Retained payload pages remain 16 KiB. The operation envelope is 256 pages/4 MiB so the owner-isolated Subspace schema can simultaneously represent current, candidate/retiring, factor, sparse operator, checkpoint, preview, and terminal owners without an uncensused aggregate buffer.
- LDLT keeps deterministic vector columns and row lists; the live route contains no BTreeMap/HashMap.
- Subspace construction reserves then initializes one owner/scalar per opportunity. Displaced work moves to `retiring_work` and closes incrementally before reuse.
- Checkpoint, preview, final result, rejected page source, partial restore payload, and partial decoded state remain retained authorities. Job and restore close paths release at most one page/semantic owner per grant and expose terminal-empty witnesses.
- Mounted session inventory preserves accepted P6g regions and credits the third mesh edge-index vector root.

## Laws and Faithful Mutations

Focused Rust fixtures cover zero fuel, expired deadline, cancellation, stale generation/revision/seed, maximum +1 admission, singular/refused numerical states, deterministic 1/2/4-worker replay, batch/reference parity, nested-stage cancellation, checkpoint roundtrip, version/identity/truncation rejection, partial checkpoint/result writer close, partial restore close, and exact terminal-empty reclamation.

The isolated verifier has 34 faithful mutations. It rejects restoration of whole LDLT/subspace helpers, missing stages, missing maxima, silent defaulting, identity loss, constructor preallocation, whole preview serialization, recursive displaced-work drop, missing truncation/version tags, missing retained restore cursors, missing direct page backing, missing checkpoint stream/terminal writer/interrupted close, restored publication buffers, mesh edge rebuild/recovery/credit loss, and whole/missing element kernels or laws.

## Exact Residual Census

- Live `LdltJob` contiguous checkpoint API: 0.
- Live `SubspaceIterationJob` contiguous checkpoint API: 0.
- LDLT/Subspace `publication_bytes` or `preview_bytes`: 0.
- Live LDLT/Subspace serde calls: 0.
- Live LDLT whole-column calls: 0.
- Live Subspace whole-iteration calls or standard-library sort: 0.
- Live constraint full edge-map rebuild/recovery calls: 0.
- Mounted element whole stiffness-matrix calls: 0.
- Unclosed partial checkpoint/result/restore writer authority: 0 known paths.
- Contract-level source residual: 0.

The remaining sparse-module contiguous serde checkpoint symbols belong to the out-of-scope PCG job and were not used by either P6h live block.

## Validation

Executed:

- `rustfmt --edition 2021` and `--check` over the six scoped FEM Rust files plus the retained job runtime file: clean.
- `git diff --check` over the exact P6h files: clean.
- `bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test`: `[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=34.`
- Exact source census for forbidden live checkpoint/publication buffers and whole numerical calls: clean.

Deferred by the coordinator collision protocol:

- Cargo check/test, strict-warning Rust gates, Nx, native/Wasm matrices, browser/mounted-product execution, release timing, sanitizer/allocation stress, and third-party numerical oracle execution.
- These executable gates must run serially after overlapping Rust/framework source work quiesces; none is claimed passing here.

