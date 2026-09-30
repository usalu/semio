# Final Contract Regression Audit

Read-only audit of settled dependency completeness, retained command authority, and canonical contribution metadata. No source edits, builds, native reruns, renderer setup, Git mutations, worktrees, or ticket/goal changes were performed. Existing owner logs are evidence of their executions, not tests independently executed by this auditor.

## Findings

No new actionable regression found in the three settled slices. The final aggregate was still running at inspection; its completion must be reported separately. The standalone Cargo direction checker is distinct from the strict TypeScript/JavaScript gate and must retain its known-red status.

## Dependency Completeness

The production library `🕸️dependencies/🧭️direction/🟦️.ts` independently inventories followed source files, rejects missing inventoried importers, and validates local followed-target closure even when resolver flags claim a local TypeScript target is nonfollowed. Unresolved relative references refuse unless an explicit configured terminal pattern applies. Linked sources and linked directories refuse inventory, and unreadable roots propagate filesystem errors. Scope and policy metadata, totals, and independently reconstructed violation sets are validated.

The owning package script constructs source inventory before resolver execution and derives authored workspace package names, ownership, and exported subpaths from readable manifests. Authored aliases remain ownership-bearing even when unresolved; upward aliases cannot silently disappear from the rule verdict. Declared same-layer unresolved aliases are deliberately accepted by the neutral fixture, preserving scope honesty: this gate proves direction and followed-source completeness, not universal module resolvability. Unknown authored subpaths and traversal subpaths refuse. Deliberate external package terminals and node builtins remain valid. This acceptance is not a missing local-source exemption.

The originally reproduced missing-local-target exploit is covered by `missing-followed-local-source`, `fake-nonfollowed-local-source`, `unresolved-local-source`, and actual dependency-cruiser resolution cases. Self-consistent disconnected importer deletion is covered against the independent inventory. The neutral tests compare actual dependency-cruiser output rather than only hand-constructed reports.

Latest owner evidence `🗑️generated/dependency-completeness-canonical-closed.log` records seven passing tests, 408 assertions, 3067 inventoried framework sources, a passing live direction check, and successful Nx completion. Earlier `...canonical-settled.log` is a superseded failing execution and must not be cited as the final result. `...typecheck-closed.log` is the corresponding latest typecheck evidence.

## Retained Command Authority

In plugin `🦀️.rs:31161`, `capture_typed_command_roots` captures immutable snapshots, O(1) peer roots, revision/generation authorities and exact targeted window authority. `start_typed_command_operation` at line31218 mounts the admitted operation without `A::ephemeral` or a preview producer call. Factory construction remains the app-owned job builder boundary. Presence reads in `ArtifactOwnedToolJobContext` are private fields and exposed through a borrowed read-only view; completion mutation ownership remains separate.

The real retained shell `🧵️retained-command/🦀️.rs:556` executes `work.step` during the Work phase; `CompleteWithEphemeral` stores typed mutation output and advances to Publish. Publication verifies a mounted consumer and transfers output to completion. The live app publisher separately validates declared publication lanes and starts generation-authorized one-item presence/transient publication, keeping captured scratch output distinct from live state. Cancellation, stale-authority and close/retirement coverage remains in the exact native contribution.

`🗑️generated/plugin-command-owner-final.log` ends in successful Nx completion and names all four exact native laws: both ephemeral lanes without history, empty ephemeral output, peer capture/root retirement, and saturation/cancellation/stale/interrupted-close authority. This audit did not rerun these native laws.

## Canonical Metadata and Scope

The regenerated `.vscode/launch.json:20231` picker contains exactly16 owners, including `@semio-tech/framework-plugin` and `@semio-tech/s-fixture-sweep-rs`. Root `📜️script.ts:8959` uses generic Nx `run-many -t canonical-architecture --all --exclude workspace`; the observed `🗑️generated/canonical-integrated-final.log` enumerates the same16 projects. At inspection only the CAD success row had appeared; full aggregate completion was not yet available.

Job canonical selects its exact named reconcile law through `runExactCargoLaws`. Hub canonical likewise selects seven explicit plugin-module laws through that exact runner; neither canonical path relies on a broad warning-tolerant empty filter. `🗑️generated/registry-cargo-owner-final.log` confirms registry generation and launch regeneration completed successfully. These narrow canonical targets do not claim the unrelated complete GIS ledger oracle or full native fleet passed.
