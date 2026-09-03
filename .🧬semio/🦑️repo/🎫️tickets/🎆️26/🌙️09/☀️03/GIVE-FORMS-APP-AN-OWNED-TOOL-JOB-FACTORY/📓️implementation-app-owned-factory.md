# Forms App-Owned Tool Job Factory

## Association

- Goal: `🎯️r2602🎯️runningsketchpad🎯️runningsketchpadapps`
- Ticket: `26/09/03/GIVE-FORMS-APP-AN-OWNED-TOOL-JOB-FACTORY`
- Reference implementations: generation2d retained factory and the generation3d app-owned Artifact/Config preparation factories from `26/09/03/PROCEDURAL-3D-END-TO-END`.

## Defect

Forms declares bounded-first-step proof rows backed by the framework sentinel factory but registers no concrete `ArtifactOwnedToolJobFactory`. `VcsArtifactApp::qualified_tool_proof` therefore rejects the tools with `interactive-job.missing-owned-reducer` before dispatch. The command catalog has 28 rows, so fixing only the two currently classified rows would leave the remaining Forms actions outside the owned retained pipeline.

## Publication-Lane Audit

| Lane contract | Tools | Handler evidence |
| --- | --- | --- |
| Config | `setTryValue`, `setTryValues`, `resetTry`, `previousStep`, `nextStep`, `setLocale`, `setContributions`, `setTryValueStep` | Handlers emit `config_mutations`, `Emit::config`, or delegate to the bounded try-value continuation that emits Config mutations. |
| Artifact + Config | `addStep`, `patchStep`, `removeStep`, `moveStep`, `addQuestion`, `removeQuestion`, `patchQuestions`, `moveQuestion`, `dropQuestionKind`, `setSpecJson`, `setActiveExample` | Successful handler paths emit both document and config mutations; no-op and Config-only branches do not widen the contract. |
| Artifact | `updateForm`, `patchQuestionOptions`, `addQuestionOption`, `removeQuestionOption`, `patchVectorField`, `addVectorField`, `removeVectorField` | Successful handler paths emit only document mutations through `Emit::mutations`, `Emit::amend`, or the explicit Artifact field. |
| HostOnly | `submit`, `exportFixture` | `submit` is empty; `exportFixture` emits a host download effect and no retained store mutation. |

No Forms handler emits Draft, Presence, Transient, or Child publication.

## Design

- Register one `FormsBoundedCommandJobFactory` with an exact 28-id key set and an exact 28-row `PUBLICATION_CONTRACTS` table.
- Join every proof row to that concrete factory with `factory_type` and classify every generated Forms command as migrated in the manifest.
- Build each retained job with `ArtifactRetainedCommandPayload` and `BoundedArtifactCommandWork`, preserving operation context, history, interaction, and cancellation ownership.
- Install Forms-owned one-item preparation factories for Artifact and Config. Each admits only the Document history lane, validates the live operation/generation/base authority, bounds encoded mutations, computes inverse and post-state with the domain's `protocol::Mutation` implementation, and retires every retained owner incrementally.
- Pin the command/proof/factory/publication bijection in `retained_route_dispositions_are_exact_and_exhaustive`.

## Verification

### Baseline

The requested pre-change `cargo test --package semio-s-plugin-forms --lib` could not acquire the shared target lock. It was restarted with an isolated ticket-local `CARGO_TARGET_DIR`, but another concurrent workspace cleanup removed that target directory while `semio-s-plugin-stdio` was compiling. Cargo stopped before compiling Forms with `failed to create file encoder: No such file or directory`; consequently this run did not reach the expected `interactive-job.missing-owned-reducer` assertions and is not counted as a Forms test result.

### Post-change

- Source-structure gate: passed. The retained inventory, publication table, and proof table each contain the same 28 ids in the same order; the manifest has 28 migrated classifications; exactly one concrete `factory_type` row and one Forms factory registration are present. `git diff --check` also passed for the Forms editor and this report.
- `cargo check --package semio-s-plugin-forms --lib`: blocked before Forms compiled. The shared tree first failed in concurrently modified framework dependencies because non-ASCII emoji had been inserted into byte-string literals and the graph build script could not resolve its generated registry/import.
- `cargo test --package semio-s-plugin-forms --lib`: blocked during workspace manifest loading because concurrently modified dependency manifests pointed at package paths that had not yet been moved.
- `cargo check --target wasm32-wasip2`: blocked by the same workspace-manifest path inconsistency.
- `bun ./📜️script.ts describe`: blocked before the descriptor builder loaded because the concurrently modified Forms script imported a nonexistent taxonomy path.
- `bun nx run @semio-tech/plugin-registry:check`: blocked before Nx loaded because the concurrently modified root script imported a nonexistent taxonomy path.

The descriptor was therefore not regenerated. The existing `🔣️.json` also has unrelated concurrent modifications and was deliberately left untouched by this ticket. None of the failed compiled gates reached the Forms crate, so they neither validate nor invalidate the owned-factory implementation.
