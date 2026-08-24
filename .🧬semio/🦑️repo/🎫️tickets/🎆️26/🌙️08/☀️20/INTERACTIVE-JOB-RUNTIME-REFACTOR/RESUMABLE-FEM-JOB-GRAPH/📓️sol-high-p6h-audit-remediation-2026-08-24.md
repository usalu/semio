# Sol High P6h Acceptance-Audit Remediation

Date: 2026-08-24  
Owner: `/root/p6h_audit_remediation`  
Audits: `📓️terra-phase-6-p6h-fresh-acceptance-audit-2026-08-24.md`,
`📓️terra-p6h-post-remediation-fresh-source-reaudit-2026-08-24.md`
Source verdict: **SOURCE-AUDIT-READY**

## Outcome

The first five and the post-remediation six bounded Terra counterexamples are repaired in production
source. Checkpoint, preview, terminal, and restore paths retain admitted pages across worker turns
and advance one fixed header, scalar, pair, matrix cell, comparison, partition, page transfer, or
close owner after fuel is consumed. Mounted mesh work pre-consumes fuel and retains preparation,
initialization, insertion-finalization, constraint-adjacency, and publication cursors. Owned assembly
retains DOF, node, partition-minimum, and chosen-triplet transfer state. The production-path laws
exercise interruption, replay, numerical parity, timing, refusal, and exact close seams. The isolated
verifier extracts every cited live helper and rejects 70 faithful mutations of the source.

## Counterexample Mapping

| Terra blocker | Production repair | Discriminating evidence |
| --- | --- | --- |
| Checkpoint/output/preview page-fill loops | `RetainedJobPayloadWriter` owns an admitted staged page; LDLT/Subspace writers append one header/scalar/pair/cell and return, then commit the page in a separate retained opportunity | Structural extraction rejects loops in every owner writer and publication body; mutations restore scalar and pair page loops |
| Whole-page restore copy and entry loops | LDLT/Subspace restore cursors retain `page_entry` plus fixed control storage, read directly from `RetainedJobPayload::page`, admit/append one owner or entry, and close the source page only after completion | No 16 KiB stack copy remains; mutations reintroduce a contiguous page copy and scalar restore loop |
| Mesh batching and post-work fuel | `MeshJob::step` performs stale/cancel/deadline checks, consumes one fuel unit, then enters the stage; preparation and insertion each call one retained transition with no batch loop | Ordered verifier predicate requires `should_yield -> consume_fuel -> match`; mutations restore post-work fuel, prepare batching, and insertion batching |
| Hidden input/hole/point scans | `CountInput` accounts one hole per turn; `FaceClassificationCursor` advances one outer/hole polygon edge, point-index comparison, point insertion, or triangle publication | Structural verifier extracts the loop-free face cursor; mutation restores whole point-index lookup |
| Decorative mesh/element laws | Mesh law calls `MeshJob::step` with one-fuel, expired, stale, and cancelled contexts at every constraint stage. Element law constructs owned mounted jobs and exercises Bar2, BeamEb2, and Tri3Cst through every stiffness stage, numerical tolerance, timing, and exact interrupted close. Maximum+1 stiffness backing uses the production admission helper and preserves pointer identity across refused close | Mutations replace the mesh law with direct recovery, owned construction with borrowed construction, remove each family/numerical identity, bypass production maximum+1 admission, or remove rejected-backing identity evidence |
| Substring-only synthetic verifier | The verifier uses balanced Rust block extraction, ordered predicates, loop census, helper/law body evidence, exact identity body evidence, and retained-writer admission/commit/close ordering | All mutations operate on live source passed to the isolated gate; prior 34 categories remain and 18 first-audit mutations were added |

## Post-Remediation Re-Audit Mapping

| Terra finding | Production repair | Discriminating evidence |
| --- | --- | --- |
| 1. LDLT contributor library search | `ContributorLookup` retains lower, upper, midpoint, comparison, and initialized state. One admitted turn performs one comparison, a later turn resolves the factor, and checkpoint/restore preserves the complete lookup cursor | The LDLT law interrupts the active lookup, proves one grant changes exactly one bound, and closes exactly. Structural extraction rejects `binary_search` and mutations restore the library search or remove a bound |
| 2. Subspace constructor factor-owner scan | Construction validates only fixed top-level admission facts. The mounted step validates one retained `l_cols` owner per admitted `fem.subspace.validate-factor-owner` turn before numerical stages can run | The subspace law proves deadline invariance, one-owner progress, maximum+1 refusal, retained fault, and exact close. Mutations restore `.iter().all` or bypass retained validation |
| 3. Mounted mesh preparation, initialization, finish, and publication | Preparation retains point-lookup and polygon/edge cursors; mounted triangulation retains bounds, insertion-order construction and adjacent ordering; finish retains filter/write/order/truncate cursors; preview/checkpoint/output use pre-admitted `RetainedJobPayloadWriter` pages and one scalar or coordinate/index fragment per grant | The mounted mesh law drives `MeshJob::new_bounded`, interrupts all new cursor families with deadline/stale/cancel, observes every initialization/finish/publication stage, proves deterministic replay and the 8 ms seam, and closes a partially staged page exactly. Mutations restore whole scans, standard sorts/retain, contiguous encoding, and point loops |
| 4. Two-slot constraint retirement | `constraint_retire_adjacency_cursor` clears or checks one adjacency slot per grant; only a later retained control turn changes the edge active bit and advances retirement | Constraint law snapshots the adjacency cursor and edge authority around deadline/stale/cancel, then verifies the mounted flip path. The verifier requires cursor-before-active ordering and rejects a restored adjacency loop |
| 5. Owned assembly lookup and partition minimum | Element construction scans one DOF-order or node entry per grant, publishes the chosen index/position in its own turn, scans one partition per grant, and transfers the retained minimum triplet separately | The owned assembly law runs a real four-partition Bar2 graph, interrupts lookup and merge states, proves deterministic matrix identity and the 8 ms seam. Mutations restore `DofMap::get`, `nodes.iter().find`, or iterator/filter/minimum merge |
| 6. Verifier blind spots | The verifier now extracts LDLT column lookup, retained subspace validation, mesh preparation/begin/init/finish/payload/publication/constraint helpers, element lookup, partition merge, and both new production-path laws | Eighteen post-re-audit mutations target these exact helpers and laws; the isolated self-test now reports 70 hostile mutations |

## Ownership and Close

- A staged payload page owns its exact operation ledger credit and backing before the first byte.
- `finish` rejects a partially staged page.
- Writer close retires the staged page before rejected backing and committed payload pages.
- Restore keeps both source page and partial target owner discoverable through cancellation/fault.
- Mesh face and numerical cursors contain only fixed scalar/control state; model owners remain in the
  existing incremental close lanes.
- Mesh publication keeps the admitted staged page and its ledger credit reachable until commit,
  transfer, or one-owner close; the completed mesh is not transferable before terminal publication.
- Assembly minimum selection retains a copied scalar triplet candidate, while every partition owner
  remains in its existing incremental close lane.

## Validation

Executed after scoped formatting:

- `bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test`
  - Result: `[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=70.`
- `rustfmt --edition 2021 --config skip_children=true --check` over the three changed FEM Rust files
  and retained job runtime exited `0`.
- `git diff --check HEAD -- <exact P6h source/verifier/report set>` exited `0`.
- A scoped semantic-equivalent census found no page-fill loop, whole-page restore copy, contributor
  library search, constructor factor-owner scan, mounted preparation/init/finish/publication batch,
  two-adjacency retirement loop, owned lookup scan, partition-wide minimum, mesh post-work fuel,
  borrowed element law, or direct-recovery mesh law in the extracted live P6h bodies.

The requested source-audit scope excludes Cargo/Nx, native/Wasm, browser, sanitizer, and broad
workspace gates. This report does not claim those gates passed.
