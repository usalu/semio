# DAG, Space Studio, and Sourcing Curate Retained Cohort

Date: 2026-08-27. Existing ticket: INTERACTIVE-JOB-RUNTIME-REFACTOR. Source-only execution; coordinator owns the exclusive compiler queue.

## Result and Scope

All 68 command-enum routes in the three assigned app files now have an explicit disposition. This is not completion of the whole interactive migration.

| App | Bounded Retained | Config | HostOnly | BatchOnlyPendingRewrite | ForbiddenFromUi |
| --- | ---: | ---: | ---: | ---: | ---: |
| DAG | 2 | 2 | 0 | 11 | 0 |
| Space Studio | 10 | 6 | 4 | 30 | 0 |
| Sourcing Curate | 5 | 5 | 0 | 9 | 1 |
| Total | 17 | 13 | 4 | 50 | 1 |

Space's four HostOnly routes were already retained in the shared tree. This packet adds exact Config preparation to 13 route owners, removes Sourcing's nine-route scan-then-monolith mechanism, and makes the remaining routes explicitly batch-only rather than claiming incremental work.

The fresh scoped official source evidence has 68 rows, 17 exact proofs, 17 publication-ready app-owned rows, zero scan-then-monolith rows, and zero proof-catalog failures. Those are source-verifier results, not compiler or runtime results.

## Research and Decisions

The previous Sourcing “resumable” executor advanced a cursor/digest over command/snapshot inputs and then invoked the original monolithic command reducer. Its yields did not split the expensive semantic operation. Removed that work type, its factory, its registration/proofs, and its obsolete SCC1 checkpoint fixture.

DAG graph edits, workflow edits, Sourcing stock scans, bulk import/export, registry work, and the undeveloped no-op routes still require real domain-owned canonical aggregate preparation or host publication mechanisms. Fifty routes remain explicitly BatchOnlyPendingRewrite. No Artifact or Child lane is claimed by this packet.

Retained routes:

- DAG Config: nodeGraphViewport, setLocale.
- Space Config: setActivePanelTab, nodeGraphViewport, presenceHeartbeat, workflowEngagementInput, compiledDagEngagementInput, closeFocusedInstance.
- Space HostOnly: setActiveExample, importSpacePack, goHome, navigateVirtualFileSystemNode.
- Sourcing Config: setFilterQuery, setFilterTypology, setFilterMinAvailability, sortTable, setContributions.
- Sourcing setLocale remains ForbiddenFromUi.

The existing shared Space presence-global removal was preserved. No process-global owner store was added. The scoped static oracle finds no thread_local, OnceLock, or LazyLock in these owners.

## Store Preparation and the 4096-Byte Grant

The coordinator confirmed that production Store advance and close grants are one semantic item and exactly 4096 bytes. Initial larger envelope estimates were corrected before handoff; no implementation depends on a 32/64-KiB Store grant.

Each Config owner now has two actual phases:

1. Admit the exact immutable base and mutation, then construct a bounded candidate and targeted inverse.
2. Seal the candidate through the Store's exact operation/generation/base-revision authority and canonical edit digest.

Cancellation and closing stop further preparation. Cleanup retires one bounded owner at a time, checks byte grants before destruction, returns the exact base read to its registry, and only reports terminal emptiness after base, mutation, candidate, prepared publication, description, and authority ownership are gone.

Current deliberately small admission envelope:

- Config root structural estimate: at most 768 bytes.
- Aggregate Config text per base/post, and text in an individual input mutation: at most 96 UTF-8 bytes.
- Actor and description: at most 64 UTF-8 bytes each.
- Candidate/publication envelope: 4096 bytes including metadata reserve.
- Worst-case JSON text expansion model: 6 × (2 × 96 + 3 × 64) + 1536 = 3840 bytes.
- Raw command limits remain 8192 bytes for DAG/Sourcing and 65536 bytes for Space; Store admission is intentionally narrower.

Collection cardinalities are checked before bounded traversal or cloning. Both base and resulting post roots are checked. Exact-limit and maximum-plus-one tests were added in Rust sources. DAG and Sourcing use targeted inverse variants. Space also uses targeted inverses, except the first camera insertion: the current schema has no remove-camera mutation, so its exact inverse is a bounded pre-edit Config snapshot.

Larger otherwise valid configuration values are unfinished. In particular, normal large contributions JSON, long engagement input, and Config roots with substantial unrelated stored text currently fail closed. The route counts must not be presented as full-value-domain migration or “100% working.”

## Validation Performed

Executed only source/static Bun checks and read-only git validation:

- Ticket-local schema/source oracle: PASS, 68 routes, 17 retained, 50 batch-only, 1 forbidden, 21 hostile source substitutions, 30 total hostile rejections.
- Existing Ajv strict Draft-07 oracle for this packet: PASS.
- Existing Ajv2020 strict oracle for the updated Space fixture: PASS.
- JSON control-character escaping oracle: PASS.
- Root tool-job verifier self-tests: PASS, 498 clean.
- Scoped official toolJobOwnerSourceEvidence over the real three owner files plus real Sourcing/workflow schema-constant files: PASS, 17 publication-ready rows, zero failures.
- Scoped git diff --check: PASS.

A full live root source report was also generated. It exited red for workspace-wide retained-caller, factory/schema, process-global, missing-publication, and remaining-command failures. The early report included DAG proof qualification/schema-resolution errors; these were corrected with qualified concrete factory implementations, the exact registry binding, and the literal authoritative DAG document schema. The later scoped evidence is the authoritative final cohort result. The full workspace report is retained as historical evidence and is not a claim of final global status.

Rust semantic tests use the already-existing serde_json dependency as an independent JSON post-state oracle. They are added but NOT compiled or run in this executor. No Cargo, Nx, rustfmt, compiler, browser, or runtime execution was performed. No native or WASM behavior is claimed.

## Files Changed

- DAG editor component: retained Config factory/proofs/build, exact publication contracts, two-phase Store preparation, classification ledger, tests.
- DAG Rust Cargo.toml: added the existing workspace-owned semio-framework-job dependency required by its new owner.
- Space Studio component: six Config routes, exact Config contracts, bounded Store owners/disposal, two-phase preparation, targeted inverses, all-route classifications, oracle updates, and corrected async ArtifactApp build_tool_job signature.
- Space retained-command-limits JSON and strict JSON Schema: ten bounded routes, thirty batch-only routes, exact Config/HostOnly contracts.
- Sourcing Curate editor component: replaced fake resumable ownership with five exact Config owners, two-phase preparation, targeted inverses, explicit batch dispositions, and tests.
- Removed the obsolete Sourcing retained-command-checkpoint.json SCC1 fixture. It remains recoverable from repository history/staged baseline; ticket evidence was preserved.
- Added this report and the DAG-SPACE-SOURCING-RETAINED packet: strict schema, language-neutral fixture, ticket-local 📜️script.ts, full historical source report, and final scoped source evidence.

## Follow-Up Boundaries

Required next work includes genuine incremental preparation/retirement for larger Config roots, actual Artifact/Child canonical preparation for the fifty batch routes, and compiler/runtime verification of this packet. The coordinator's shared-child publication work may unlock part of that remaining set.

Repo MCP ticket tools/resources were unavailable to this executor. No ticket or goal lifecycle mutation was attempted; the parent coordinator retains the existing ticket.

