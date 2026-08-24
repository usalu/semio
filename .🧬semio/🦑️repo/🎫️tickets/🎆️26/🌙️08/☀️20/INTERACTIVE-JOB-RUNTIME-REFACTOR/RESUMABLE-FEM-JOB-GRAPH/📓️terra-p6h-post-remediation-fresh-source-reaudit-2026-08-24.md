# Terra P6h Post-Remediation Fresh Source Re-Audit

Date: 2026-08-24  
Auditor: Terra `/root/p6h_fresh_acceptance`  
Scope: live working tree, read-only source audit. The only written artifact is this report.

## Result

**RED — P6h and therefore Phase 6 remain unaccepted.**

The remediation fixes the page-fill and whole-page-decode counterexamples from the preceding
Terra RED, but live mounted numerical/mesh/assembly paths still contain model-sized hidden work.
The focused verifier's successful 52 synthetic mutations do not cover those executed helpers.

## Inputs And Preserved Baseline

Read root, `✏️s`, FEM-plugin, and module `AGENTS.md`; the Phase-6 master/status/residual
material; accepted P6g third-remediation audit; P6h contract; prior Terra P6h RED; P6h
implementation report; and the new remediation report.

P6g's retained production hand-offs remain present: the mounted session creates
`MeshJob::new_bounded` at
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️session/🦀️component.rs:1146`,
steps it at `:1156`, creates `AssemblyJobConstruction::new_owned` at `:1210`, and steps the
owned assembly at `:1234`. Scoped unstaged/staged deletion census found no deleted FEM, retained
job-runtime, or verifier test source. The only live scoped diff was an unrelated P1q addition in
`📜️script.ts`; it was not attributed to P6h.

## Confirmed Repairs

- LDLT and subspace writers use a retained staged page. Each `advance_*_owner` writes one
  length/scalar/pair/cell and returns; a later empty-item turn commits the page:
  `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️component.rs:451-539`, `:792-857`, and `:2533-2632`.
- `FEMLCP1` and `FEMSCP1` restore directly from `RetainedJobPayload::page`, retain a
  `page_entry`, validate tag/version/field/identity/length before owner admission, and close the
  consumed page separately: sparse `:973-1117`, `:1138-1317`, and `:3650-3843`.
  No whole-page stack buffer or source-page copy is on either restore route.
- The actual LDLT/subspace steps check cancel/stale/deadline/fuel before their bounded-stage work;
  their page writers and partial close owners are retained. Fixed-schema Bar2, BeamEb2, and Tri3
  cell kernels are present, and the Rust law names their owned-route tolerance/timing/close seams:
  elements `:85-96`, `:341-365`, `:611-665`; analyses `:2683-2775`.
- No production nested pool or decorative `async fn` was found in the scoped FEM/session sources.
  The `block_on` hits are test-only cancellation setup.

## Blocking Findings

### 1. LDLT contributor lookup hides an input-sized search in one grant

`LdltColumnStage::ContributorLookup` calls `binary_search_by_key` over an admitted prior column
after one fuel unit, rather than retaining a lookup-comparison cursor:
`✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️component.rs:722-732`.
The P6h contract explicitly requires contributor lookup itself to be cursorized. The library
binary search executes an unmetered, fill-dependent comparison loop in one semantic opportunity.

**Repair:** retain lower/upper/mid/comparison state for the active contributor and consume one
grant per comparison; only compute the factor after that cursor resolves.

### 2. Subspace construction still scans all factor owners synchronously

`SubspaceIterationJob::new` determines `factor_pages_valid` with
`k_factor.l_cols.iter().all(...)` before a retained step exists:
`✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️component.rs:2430-2436`.
This is a model-order scan during construction, not an observed per-owner admission turn.

**Repair:** make validation/construction retained, with a factor-column/owner cursor and a
per-owner capacity check before the numerical job becomes runnable.

### 3. The live mounted mesh still performs whole-model preparation, finalization, and publication

The mounted session reaches this code (the live route is cited above), and `MeshJob::step` admits
one fuel unit before each of the following helpers (`mesh :1325-1365`). Those helpers violate the
one-opportunity rule:

- `MeshInputPreparation::insert` uses `point_indices.iter().find`, while refinement uses
  `point_in_polygon` plus `holes.iter().any`; both are input-sized scans hidden in a preparation
  grant: mesh `:494-503` and `:505-577`.
- Mounted `Initialize` calls `OwnedTriangulation::begin_mounted`, which scans every input point,
  creates/reserves owners, fills an insertion order, and standard-library-sorts it in the same
  grant: mesh `:1428-1453` and `:204-240`.
- `InsertBoundary` calls `finish_insertion()` in a single turn; that helper retains every triangle,
  sorts the complete mesh, and truncates points: mesh `:1473-1486` and `:330-334`.
- `Classify`, `Finalize`, and `Complete` call `encode_preview`/`encode_mesh`; the latter allocates
  a contiguous full result vector and loops every point, triangle, and triangle index:
  mesh `:1113-1145` and `:1557-1577`. The session's capacity decision happens only *after* that
  allocation at session `:1185-1189`, so it cannot be an exact pre-admission refusal or retained
  close owner.

**Repair:** retain cursors for preparation's uniqueness/polygon membership, mounted bounds and
incremental order construction, end-of-insertion filter/order, and mesh preview/checkpoint/output
page serialization. Admit/refuse each output page before writing it; keep partial/rejected output
discoverable through cancellation, stale, fault, drop, and one-owner close.

### 4. Constraint retirement mutates two adjacency owners in one opportunity

`RetireFormerEdge` loops both `slot.adjacent` entries in one invocation:
`✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️component.rs:1297-1312`.
P6h permits one adjacency-owner update per grant. This is not a harmless search: it changes both
owners before yielding.

**Repair:** add a retained adjacency-slot cursor, clear/check one slot per grant, then make the
active-bit transition a separate retained control opportunity.

### 5. Owned AssemblyJob retains model-/partition-sized lookup loops

The mounted session executes the owned `AssemblyJob`, but the job consumes one fuel before these
unbounded helpers:

- `DofMap::get` is an `iter().position` scan of the complete DOF order (`analyses :349-356`) and
  is invoked for each supposedly one-unit local-to-global index (`:1498-1507`).
- Position capture calls `self.model.nodes.iter().find` in its one-position stage (`:1518-1526`).
- Each merge opportunity calls `next_partition_triplet`, an iterator/filter/minimum scan across all
  partition buffers (`:1683-1706`), from the post-fuel `MergeFull`/`MergeFree` cases
  (`:1954-1980`).

**Repair:** retain DOF/node lookup cursors and a deterministic partition-min scan cursor plus
candidate. Scan one entry/partition per grant, then transfer one chosen triplet in its own grant.

### 6. The verifier accepts every blocker above

`toolJobFemNumericalMicrocursorExact` only extracts the outer mesh impl, one constraint-match
slice, the face cursor, and `advance_element_build`; it never examines `begin_mounted`,
`MeshInputPreparation`, `finish_insertion`, `encode_mesh`, `binary_search_by_key`, `DofMap::get`,
or `next_partition_triplet`: `📜️script.ts:3603-3630` and `:3738-3763`.
Its 52 in-memory string mutations consequently have no mutation for these real paths
(`:3771-3825`). The named Rust laws are not executed by the isolated TypeScript verifier, so the
reported green cannot prove real stale/cancel/deadline/tolerance/<8 ms/replay/partial-close
behaviour for the uncovered helpers.

**Repair:** extract every producer/consumer helper actually reached from the mounted route, reject
the specified iterator/sort/retain/encode paths, and add faithful mutations for each. Add focused
executable laws that interrupt every added cursor stage, test max/max+1 refusal before allocation,
and prove retained partial-output close plus deterministic replay and timing.

## Isolated Evidence

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test` | Passes, but falsely: `live-source clean; hostile-mutations=52` |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` over sparse, mesh, model, elements2d, analyses, mounted session, retained job runtime | Pass |
| Scoped unstaged and staged `git diff --check HEAD -- …` | Pass |
| P6g/P6h scoped deleted-source census | Pass; no deleted source reported |
| Exact P6h source contract | **Fail** — findings 1–5 |
| Cargo/Nx/native/Wasm/browser/runtime timing matrix | Not run; excluded by the isolated-audit scope |

No production source was changed by this audit.
