# Shared-Image Optional DATA Recovery Feasibility

Date: 2026-10-21. Read-only source audit. No build dispatch, Native signal, cache payload read/delete, policy change, or production source edit. SourceDemand 9977 remains the only goal Native invocation.

## Decision

**UNSAFE under the current supported owner contract.** Common production inference routes hold the correct namespace writer leases. However, the supported physical/header DATA reader capability accepts image-SH-only custody without requiring namespace enrollment. This missing exclusion is sufficient to reject concurrent DATA retirement. No actual unleased production DATA read was observed, and none is alleged.

Existing authenticated group retirement and its intent catalog independently require a complete global proof and genuine kernel image EX. They cannot be reused under SH. Retaining all Source/lock inventories avoids reclaiming referenced inventories, but does not confer namespace reader custody or authenticate concurrent retirement state. The existing KernelEX path remains the completion route.

## Scope and positive caller coverage

A bounded library scan inspected 2,090 TypeScript/MJS source files, 22,506,764 bytes across 6,769 entries. It excluded node_modules, .git, .cache, .🧬semio, target, dist and generated output. It did not inspect application cache payloads or claim whole-workspace caller completeness. Two earlier recursive listings entered dependency trees and timed out; only the analysis VM reset, with no Native handle or active control client present. The bounded source scan completed in approximately six seconds.

Paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

- `🏗️build/🕸️graph/📥️inference/🚦️invocation/🟨️.mjs:26–40`: nodes/dependencies enter through `withRepositoryBuildGraphFactStoreV1` before optional inference, records and receipts.
- `🏗️build/📥️inputs/💾️storage/🟨️.mjs:141–173`: the genuine fact lease spans the computation; registered record handles close before its release (164–166). Reentrant entry retains selection/dataset namespace (144); a different nested namespace cannot wait (155).
- Graph `🟦️.ts:129–135` enters selected inference through `withRepositoryBuildGraphSelectionV1`; datasets enter at storage 203 onward. Live ALS identities choose physical directories (119–122), rather than granting arbitrary caller namespace authority.
- Actual broad image-SH callbacks found read retained objects: graph portable inputs (graph 201), immutable snapshots (439), exact Source/lock batches (inputs/digest/read 41), inventory reuse (observation/reuse 64), and selected-task Source inventories (`⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🟦️.ts:37–48`). These are not DATA-retirement counterexamples.
- Other direct header-reader calls found in this library were tests. Positive normal-call coverage does not narrow the supported physical reader contract.

## Exact resources

`🏗️build/📥️inputs/💾️storage/🔒️writer/🟨️.mjs` validates 64-lowercase-hex selection/dataset identities. Shard = first SHA-256 byte of kind + ":" + identity, modulo the existing 64 writer shards.

| Domain | Resource |
| --- | --- |
| Isolated optional facts | `repository-graph-facts:<resolved workspace>:shard:<n>` |
| Canonical optional facts | `repository-graph-facts:<resolved workspace>` |
| Selected publication pair | `build-graph-state:<graph root>:selected:<n>` |
| Optional ledger initialization | `repository-graph-optional-ledger:<resolved workspace>` |

Selected publication state and fact namespace are different resources. Normal inference acquires fact leases exclusively for reads and writes. Ledger initialization custody (storage 68–70) is not retained over producer work; accounting uses the separate actual SQLite gate.

## Missing supported reader exclusion

Storage 129 creates `graphReadScopes` under genuine whole-image consumer custody. Storage 133 accepts managed reads inside the canonical store with valid Original/control. It neither requires a fact namespace resource nor checks that the path belongs to that enrolled namespace. A read can use `graphReadScopes` with no writer.

The concrete supported owner chain is exported `inferenceHeaderLine` (`🏗️build/👁️observation/🟨️.mjs:690`) or `readInferenceHeaderV1` (751), accepting an absolute record path, then the physical inference reader and `observationPhysicalReadV1` (703–708). Admission and final fences call only `requireRepositoryBuildGraphObservationPathV1(path,"read")`, which accepts the image-SH context above.

Real ancestry, same-device, single-link regular leaf, descriptor stamp, decompression/hash and delivery rechecks authenticate bytes; they detect a concurrent removal rather than exclude a namespace-only collector. Also `registerBuildGraphOptionalHandleV1` (storage 110) is a no-op outside a writer. Normal observed-record handles are enrolled/drained (observation 547–555; storage 164), but an image-only reader is not thereby a namespace reader.

Consequently the supported reader can access retirement-role DATA under image SH alone. Fact-shard EX would not exclude it. This capability is sufficient to reject the SH proposal, even without a current ordinary production caller exercising it.

## Kernel-only authentication and intent lifetime

Owner: `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts`.

- `captureClosureProof` (61–62) requires current defining Source, genuine kernel image EX, complete census, catalog, pair authentication and worker-validated contents/references. It is not a per-group SH proof.
- `retirementCatalog` (79–81) authenticates hash-addressed plans, original/group identity, complete member role/hash/stamp rosters, renamed directory identity, and prepared/committed states. Pending references and authenticated committed survivors remain authoritative.
- `namespaceWriter` (88) adds exact canonical/dataset/selection EX after the global proof. Storage 224–230 still requires genuine kernel EX. Copied custody/acquired results cannot substitute.
- `publishRetirement`/`invalidateGroup` (84–91) perform durable intent, rename and commit against that global proof. Its global name/stamp fences cannot classify unrelated live namespace/catalog churn under SH as owned mutation. No separate SH intent-catalog writer capability exists here.
- The read worker scans live payload keys/values, packed records, inference blocks and graph/receipt links. References name Source/lock inventories. Retaining all of them is conservative but does not supply reader exclusion or member/intent authentication.

## Accounting and disposition

`🏗️build/📥️inputs/💾️storage/📒️ledger/🟨️.mjs:47–57` uses actual SQLite gate/ledger EX transactions. Durable dirty intent precedes synchronous physical mutation; physical inventory reconciliation precedes clean accounting. Its retire callback cannot be asynchronous. Initialization/measurement authenticates ledger device/inode/journal bounds and inventories known optional leaves. Manifest guesses or logical decrements cannot grant physical headroom.

No new recovery code, Native job, signal, reader/API redesign or guard relaxation was made or authorized by this audit. Keep all current KernelEX APIs unchanged. Prepared unit/catalog/architecture and twenty-case E2E dispatches remain pending their existing release conditions; continue the sole focused ordinary recovery attempt.
