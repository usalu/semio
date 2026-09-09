# Native Library Recovery Audit

Read-only audit of the root-owned 39-entry default-library recovery session (`native-library-recovery-plan.json`, `native-library-recovery.tsv`, and `native-library-recovery-*.txt`). This is a live matrix, not an acceptance result.

## Current Matrix State

- The plan contains 19 leaf package checks and 20 parent package checks.
- At the audit snapshot, `native-library-recovery.tsv` is empty and the only current receipt is `native-library-recovery-semio-s-artifact-mathematical-equation.txt`.
- That receipt has reached `@semio-tech/mathematical-equation-rs:check --lib`, but Cargo reports `Blocking waiting for file lock on build directory`; its running marker reached 180 seconds. It contains no Rust diagnostic, failed test, or terminal Nx status.
- The shared ticket cargo target is active under the root Flow compile (`semio-s-artifact-flow-flow --lib --no-run`, compiling the WebGPU dependency); the root PDF build is also live. The matrix has therefore not yet compiled its first selected library. No implementation repair is justified from the current matrix output.

## Old Diagnostics That Must Not Be Reclassified as Current

- `E0382` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:811` belongs to the earlier default matrices. Current source aggregates `lifecycle_work` and no longer borrows the moved `events` value. It requires a fresh consumer result, not another edit based on the old receipt.
- The older 22-error OS/store fan-out in `owned-24-parent-composition-current.txt` (`E0425` for `ArtifactStoreOneItemPublication` at durable-group lines 1159 onward and `E0422` for `ArtifactStoreOneItemAdmissionRejected` at store line 18215) is historical. Current durable-group source has the batch publication/phase form at `🧩️composition/🗄️durable-group/🦀️.rs:1184-1189`; do not restore removed names to satisfy that stale output.
- The old required-nullable `E0369` is repaired separately and has a new root-owned runtime pass; it is unrelated to this default-library session.

## Next Evidence Boundary

Wait for the first TSV row. For a nonzero row, take the first compiler error from that row's matching `native-library-recovery-<package>.txt`, verify the cited current source line before proposing a minimal repair, and keep later cascades separate. Until then, the only actionable condition is target-directory serialization, not a compiler failure.

## Process3D Default Feature Review

While its ordinary library check was active, the coordinator read the Process3D artifact manifest. It has no default feature group that selects the stdio parent plugin; it directly declares ten required format packages (DWG, glTF, IFC, JSON, OBJ, PNG, Semio, STEP, STL, TXT) plus framework dependencies. Its library source is `../../🦀️.rs`. This is declaration evidence of format selection, not a completed native check. The earlier Shooting check exited130 because jco-package-adapter inputPatterns were unsorted; that specific taxonomy list was corrected before later graph runs.

## Current Default Library Matrix Interruption Classification

The ordinary Nx matrix remains live. VCS and Process3D have accepted exit-0 receipts. Lowpoly, Wires, Forms, Layout and CAD exited during project-graph construction, before Cargo checked their package: Lowpoly encountered the relocated Note document-contract schema import; Wires, Forms, Layout and CAD encountered the relocated shared-map-delta source-law import in the root router. Playbook reached its framework-graph generator prerequisite and failed on the undefined `artifactProjection` identifier. These source paths and the identifier are repaired, and the fresh repository contract invocation has since cleared graph construction. They need targeted retries after the current matrix ends; these receipts do not establish native package regressions. The earlier Equation and Flow native compiler failures were in the shared IO/helper layer and are separately recorded. Shooting was interrupted by the generator input-pattern ordering error.
