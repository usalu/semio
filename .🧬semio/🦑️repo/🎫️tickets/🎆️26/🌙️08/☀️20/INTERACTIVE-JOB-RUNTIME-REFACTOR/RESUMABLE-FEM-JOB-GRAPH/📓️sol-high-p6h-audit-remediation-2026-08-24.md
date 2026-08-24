# Sol High P6h Acceptance-Audit Remediation

Date: 2026-08-24  
Owner: `/root/p6h_audit_remediation`  
Audit: `📓️terra-phase-6-p6h-fresh-acceptance-audit-2026-08-24.md`  
Source verdict: **SOURCE-AUDIT-READY**

## Outcome

The five bounded Terra counterexamples are repaired in production source. Checkpoint, preview,
terminal, and restore paths retain admitted pages across worker turns and advance one fixed header,
scalar, pair, matrix cell, page transfer, or close owner after fuel is consumed. Mounted mesh work
pre-consumes fuel and retains input-count, polygon-edge, hole, point-index, and publication cursors.
The mesh and element laws now drive the mounted `InteractiveJob` paths. The isolated verifier
extracts the executed helper, step, restore, law, and retained-writer bodies and rejects 52 mutations
of the live source.

## Counterexample Mapping

| Terra blocker | Production repair | Discriminating evidence |
| --- | --- | --- |
| Checkpoint/output/preview page-fill loops | `RetainedJobPayloadWriter` owns an admitted staged page; LDLT/Subspace writers append one header/scalar/pair/cell and return, then commit the page in a separate retained opportunity | Structural extraction rejects loops in every owner writer and publication body; mutations restore scalar and pair page loops |
| Whole-page restore copy and entry loops | LDLT/Subspace restore cursors retain `page_entry` plus fixed control storage, read directly from `RetainedJobPayload::page`, admit/append one owner or entry, and close the source page only after completion | No 16 KiB stack copy remains; mutations reintroduce a contiguous page copy and scalar restore loop |
| Mesh batching and post-work fuel | `MeshJob::step` performs stale/cancel/deadline checks, consumes one fuel unit, then enters the stage; preparation and insertion each call one retained transition with no batch loop | Ordered verifier predicate requires `should_yield -> consume_fuel -> match`; mutations restore post-work fuel, prepare batching, and insertion batching |
| Hidden input/hole/point scans | `CountInput` accounts one hole per turn; `FaceClassificationCursor` advances one outer/hole polygon edge, point-index comparison, point insertion, or triangle publication | Structural verifier extracts the loop-free face cursor; mutation restores whole point-index lookup |
| Decorative mesh/element laws | Mesh law calls `MeshJob::step` with one-fuel, expired, stale, and cancelled contexts at every constraint stage. Element law constructs owned mounted jobs and exercises Bar2, BeamEb2, and Tri3Cst through every stiffness stage, numerical tolerance, timing, and exact interrupted close. Maximum+1 stiffness backing uses the production admission helper and preserves pointer identity across refused close | Mutations replace the mesh law with direct recovery, owned construction with borrowed construction, remove each family/numerical identity, bypass production maximum+1 admission, or remove rejected-backing identity evidence |
| Substring-only synthetic verifier | The verifier uses balanced Rust block extraction, ordered predicates, loop census, helper/law body evidence, exact identity body evidence, and retained-writer admission/commit/close ordering | All mutations operate on live source passed to the isolated gate; prior 34 categories remain and 16 audit-specific mutations were added |

## Ownership and Close

- A staged payload page owns its exact operation ledger credit and backing before the first byte.
- `finish` rejects a partially staged page.
- Writer close retires the staged page before rejected backing and committed payload pages.
- Restore keeps both source page and partial target owner discoverable through cancellation/fault.
- Mesh face and numerical cursors contain only fixed scalar/control state; model owners remain in the
  existing incremental close lanes.

## Validation

Executed after scoped formatting:

- `bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test`
  - Result: `[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=52.`
- `rustfmt --edition 2021 --config skip_children=true --check` over the three changed FEM Rust files
  and retained job runtime exited `0`.
- `git diff --check HEAD -- <exact P6h source/verifier/report set>` exited `0`.
- A scoped semantic-equivalent census found no page-fill loop, whole-page restore copy, mesh step
  loop, post-work mesh fuel, borrowed element law, or direct-recovery mesh law in the extracted live
  P6h bodies.

The requested source-audit scope excludes Cargo/Nx, native/Wasm, browser, sanitizer, and broad
workspace gates. This report does not claim those gates passed.
