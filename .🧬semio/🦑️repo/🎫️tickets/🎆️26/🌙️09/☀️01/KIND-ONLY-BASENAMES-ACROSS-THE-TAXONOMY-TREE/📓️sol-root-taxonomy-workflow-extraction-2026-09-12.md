# Root Taxonomy Inventory And Mutation Workflow Extraction

**Status: complete and independently accepted.** The root taxonomy inventory, mutation workflow, structural reachability, and repository JSON Schema subset now live in 11 semantic owner directories. Every implementation basename is the anonymous TypeScript leaf `🟦️.ts`; domain identity is carried by the directory taxonomy and can remain stable if a leaf is reimplemented in another language. Root `📜️script.ts` remains the mandatory executable router.

## Final owner inventory

All paths are relative to the repository root.

| Concern | Parent kind | Owner kind | Anonymous implementation leaf | Lines | Final SHA-256 |
| --- | --- | --- | --- | ---: | --- |
| command contract | `repo-normalization` | `repo-normalization-command-contract` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🎮️command-contract/🟦️.ts` | 236 | `3722f12ddf928c466d66bc22164ba1553730199080dcd7324cb11b412adba2dc` |
| inventory serialization | `repo-normalization-inventory` | `repo-inventory-serialization` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/📇️inventory/🧾️serialization/🟦️.ts` | 130 | `ed3892baf0fcb30a6c8bef91172604a767f6121fd5a9a6f11567e40069964242` |
| inventory sharding | `repo-normalization-inventory` | `repo-inventory-shards` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/📇️inventory/🧩️shards/🟦️.ts` | 203 | `0819290fd36c2577bc61a00c11a59ff2722431af1c86b2209757d430a29cd452` |
| inventory publication | `repo-normalization-inventory` | `repo-inventory-publication` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/📇️inventory/📦️publication/🟦️.ts` | 109 | `877b8b566c1ec2697d8f8964deb67de73e97c4f6deb3f6302c6644f6f9d35db5` |
| mutation identity | `repo-normalization-mutation` | `repo-mutation-identity` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts` | 52 | `42f9c955960194c580d5dccb41091f35becadf61ba436ffd1c2ab9b7eb6a250d` |
| captured mutation source | `repo-normalization-mutation` | `repo-mutation-captured-source` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📸️captured-source/🟦️.ts` | 159 | `280c44e356c29f73bfd86efc5c993215e1f75c9663bdba29cd66d6cecbfe06ea` |
| mutation source index | `repo-normalization-mutation` | `repo-mutation-source-index` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📇️index/🟦️.ts` | 98 | `eebee2a2f1dc447d15e7efe7246ea9dfcd28a6fc6eadb010bd3a019ecd0b506e` |
| mutation evidence | `repo-normalization-mutation` | `evidence` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts` | 286 | `dd6ca9219f15533296ca1d5eaea3c046a2838bd1c711c7ae7f3fd828e221db9c` |
| mutation workflow | `repo-normalization-mutation` | `repo-mutation-workflow` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🔁️workflow/🟦️.ts` | 118 | `fd8cd7079ab76a95acb2b58cc936806a107291548d5d734f751cda2a232cc265` |
| structural reachability | `repo-normalization-mutation` | `repo-mutation-structural-reachability` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts` | 520 | `184d06b51577810f49dcbc4ddf1fc6e20be47128657c8184fde96798bda4656f` |
| JSON Schema subset validation | `schema` | `repo-schema-subset-validation` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/✅️validation/🟦️.ts` | 67 | `12d2cdaa1490ba16c73ade6e8e09602bb007a59275f8876263827d84703149e8` |

The final path-level AST graph contains 19 owner-to-owner edges and is acyclic. The independent audit classified them as 18 runtime edges plus one type-only `workflow → captured-source` edge. There are zero owner-to-root edges. A lower structural-view interface removed the former type-only `captured-source → index` back edge.

At the final snapshot, root `📜️script.ts` had 28,300 lines and imported six public owners at lines 196–201: command contract, inventory publication, mutation identity, captured source, structural reachability, and workflow. The portable AST contract confirms that the moved implementation declarations are absent from root. The other owners are reached through the acyclic owner graph or by direct consumers.

## Behavioral closure

The source index obtains one canonical admission per snapshot. It captures admitted bytes, structural directories, schemas, and the assignment ledger into that snapshot; facts derive from the captured view. `inventoryMutationTaxonomy` intentionally compares a before and after snapshot for freshness. This is two epochs for change detection, with one collector in each epoch, rather than a post-capture source traversal.

The canonical admission now includes physical structural directories named `🧬️mutations` in the same observation set and membership digest. Directory enumeration is no-follow and preserves raw path bytes through fatal UTF-8 decoding. Repository fences, nested repositories, `compose`, exclusions, cancellation, unreadable paths, and symlinks remain fail-closed. A language-neutral fixture compares the captured directory set with independent `fast-glob` enumeration and proves that removing an empty physical mutation directory changes the digest.

The source index excludes repository ticket-management files and explicitly supplied workflow output paths from source evidence. A plan cannot invalidate itself merely by writing its output in the ticket, while authored assignment evidence is captured separately. Apply and terminal verification use the same output exclusion. Empty physical mutation-root removal invalidates a captured plan; this is recorded as observed current behavior because no pre-change execution evidence exists for the earlier implementation.

Structural reachability validates lexical locators before source capture, projects failures as unresolved proofs, and reads only the captured structural view. Exact direct Rust mounts, wrapped declaration origins, module paths, source aliases, and cancellation continue to come from parsed evidence. It does not use path-substring inference or a second graph read.

Actual source-as-data consumers bind directly to their owners:

| Consumer | Bound owner |
| --- | --- |
| `🧪️tests/🧾️source-file-facts/🟦️.ts` | `🧬️mutation/📸️captured-source/🟦️.ts` |
| `🧪️tests/🎭️source-roster-roles/🟦️.ts` | `🧬️mutation/📸️captured-source/🟦️.ts` |
| `🧪️tests/🎫️ticket-role-routing/🟦️.ts` | `🧬️mutation/📇️index/🟦️.ts` and `🧬️mutation/🔁️workflow/🟦️.ts` |

No root compatibility facade was restored. The source-file fact vectors were also closed over the registered `.d.cts`, `.d.mts`, and `.story.tsx` source chains.

## Exact resumed-pass mutation attribution

The resumed Sol pass modified these exact files:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`: same-epoch physical structural-directory admission and its option. This file also contains separate concurrent print-lane changes; those are not attributed here.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📸️captured-source/🟦️.ts`: structural directory request, excluded-source option, and lower structural-view ownership.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📇️index/🟦️.ts`: ticket/output exclusion projection and retained membership digest.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts`: admitted mutation-root symlink rejection.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🔁️workflow/🟦️.ts`: plan-output exclusion during apply and terminal verification.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts`: lexical validation and unresolved proof projection for absent, unsafe, and symlinked roots.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`: canonical ticket artifact path, empty-directory parity test, and an explicit 30-second budget for the expensive assignment/source-index case.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️source-file-facts/🟦️.ts`: captured-source binding and a 15-second TypeScript-oracle case budget.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎭️source-roster-roles/🟦️.ts`: captured-source AST binding.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts`: separate index and workflow dynamic bindings.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🧾️source-file-facts/🔣️.json`: three missing registered source-chain vectors.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-taxonomy-workflow-source/🔣️.json`: the three repaired source-as-data consumers.

The root router split, semantic owner registration, portable owner/schema/test assets, package target, package script, and seed/derived launch registrations were already present in the handed-off checkpoint and were verified rather than reauthored in this resumed pass.

## Verification evidence

All mutable test artifacts in this pass used `🗑️generated/sol-root-taxonomy-workflow-extraction` through `TMPDIR`, `SEMIO_TEST_ARTIFACT_DIR`, `NX_WORKSPACE_DATA_DIRECTORY`, and `NX_CACHE_DIRECTORY`.

| Check | Result | Evidence |
| --- | --- | --- |
| in-memory Bun build and runtime import | green | root plus 11 owners produced 12/12 build outputs; all 11 owners and root imported |
| owner AST import graph | green | 11 owners, 19 internal edges, zero cycles, zero owner-to-root edges |
| direct portable and repaired-consumer bundle | 18/18, 1,367 assertions | exact owner contexts, root declaration removal, direct source-as-data bindings, role schema, and source-chain closure |
| direct inventory artifact shards | 8/8, 156 assertions | canonical stable JSON, WebCrypto hash parity, 5 MiB sharding, corruption rejection, reconstruction, metadata, and private publication |
| Bun package dispatcher routes | green | ticket routing 2/2 and 200 assertions; source index 1/1 and 19; source facts 9/9 and 56; roster roles 3/3 and 6 |
| isolated Nx four-target composite | green | all four registered targets succeeded serially with private Nx data/cache roots |
| isolated Nx `test-root-taxonomy-workflow-source` | 4/4, 1,105 assertions | target succeeded with cache skipped |
| focused current-source workflow core | 9/9, 407 assertions, 76.29 seconds | admission projection, actual-directory parity, empty structural directories, unsafe scopes, stable inventory, assignment evidence, native direct reachability, wrapped origin, and terminal apply |
| live scoped taxonomy inventory | green | mutation owner subtree: 19 entries, 0 violations, 0 unresolved; digest `66c3e2ad0bc90cec5a62da7f29727cc827f697af6ebdc789527dfe58673bd87` |
| independent Terra acceptance | accepted | 11 owners, 18 runtime plus one type-only internal edge, zero cycles/backedges, direct consumers, actual Nx 4/4 and 1,105 assertions |

The final focused native cases passed on macOS ARM64: exact public canonical Rust reachability in 17.70 seconds and wrapped-origin projection in 29.25 seconds. The terminal fixture exercised symlink rejection, plan-output exclusion, empty-root digest invalidation, baseline mismatch, cancellation, fresh clean verification, and a zero-move committed result.

The assignment/source-index case first completed in 7.2 seconds while competing native jobs were active and exceeded Bun's default 5-second per-case timeout. Its explicit budget is now 30 seconds; the final run completed in 4.09 seconds. This is distinct from an earlier wrong package route, `test source-index-capture`, which selected unrelated broad work and reached a 15-second wrapper budget. Correct registered invocations passed, so there is no observed registered-route wrapper defect.

## Limits and exclusions

- Runtime and native execution was performed on macOS ARM64 with Bun 1.3.14 and the installed Rust toolchain. The portable path/context contracts passed, but native Windows and Linux execution was not performed in this pass.
- No live repository cleanup, scaffold publication, taxonomy mutation apply, or shared transaction replay was executed. Apply behavior was verified only in isolated ticket-owned fixtures and committed zero moves.
- No Git mutation, worktree, dependency addition, or `AGENTS.md` change was made.
- Root cleanup/scaffold extraction and schema-field extraction remain separate packets.
