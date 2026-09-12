# New Owner Boundary Audit

## Scope and evidence

This is a read-only audit of the composition-policy boundary, the canonical Stdio child-kind correction, Curation's parent contract and codec relocation, and the Process3D preparation record. No source, launch, Cargo, or Git operation was performed. Source locations below refer to the state inspected on 2026-09-12.

A direct read-only invocation of the current root policy returned 87 intentional medium legacy `ArtifactKindSpec` records and zero current child-DAG records. The zero is not evidence that the child-DAG algorithm is sound: its own workspace test creates a two-kind declaration cycle and asserts a high breach. That test proves the invalid type-level rule is still active even though the present source set happens not to trigger it.

## Verified: the static child-DAG policy is at the wrong abstraction level

`📜️script.ts:30222-30325` derives a node from a schema file path, invents `s.<plugin>.<artifact>.<subset>` candidates, maps every kind to the first owner encountered, then performs DFS over kind-declaration edges. The production rule treats a recursive *type declaration* as an owned-document cycle:

- `PolicyChildSlotOwner` and `policyChildSlotOwner` at `📜️script.ts:30222-30245` make the invented subset-qualified candidates and derive an owner from the source path.
- `policyChildSlotKindDagBreaches` at `📜️script.ts:30256-30325` selects the first owner for a kind at `30260-30265`, creates edges from `#[child]` declarations at `30267-30284`, and rejects DFS cycles at `30286-30324`.
- Its test at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts:1368-1387` asserts that two schemas which declare `A -> B` and `B -> A` must produce a high policy breach. It has no concrete member IDs, owners, or document instance edges.

This is contradicted by the actual ownership model. A kind is the first coordinate of an `ArtifactRef` dialect. The forest is over exact member identities and owner identities, where the same kind may occur at any finite number of distinct nodes. The neutral owned-document-closure fixture's accepted `nested-unordered-complete` case shows `kit`, `object`, `mesh`, and `properties` as distinct artifact IDs while all use artifact kind `s.stdio.semio`; standard and subset distinguish their dialects at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧫️fixtures/🔣️.json:12-129`. The independently implemented Graphlib oracle builds its directed graph from `artifactId` values and owner-parent IDs at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🟦️.ts:13-33`, then verifies acyclicity and root reachability at line 25. The same fixture rejects duplicate ownership, disconnected cycles, and self cycles at `🔣️.json:1437`, `1701`, and `1899`.

The current policy is additionally non-authoritative even on its own terms: it resolves only the first path returned for a kind, includes nested facet/test Rust files as possible path-derived owners, and deliberately skips unresolved kinds. It can neither establish referential integrity nor establish an instance-edge forest.

### Required correction boundary

Retire the type-graph algorithm: remove `PolicyChildSlotOwner`, `policyChildSlotOwner`, the kind-to-owner/edge/DFS implementation, its call from `policyCompositionBreaches`, and the type-cycle expectation/import in the workspace contract. Do not replace it with another kind-to-kind acyclicity check and do not change Store's runtime ownership validation.

Replace that policy with a syntax-only canonical child-kind metadata rule. It should apply the existing `policyIsCanonicalArtifactKind` predicate at `📜️script.ts:30163-30169` to a child declaration value only; it must not resolve that value to an owner or an edge. The complete declaration coverage for this correction is:

| Surface | Declaration form |
| --- | --- |
| Rust | `#[child(kind = "…")]` |
| GraphQL | `@child(kind: "…")` |
| TypeScript schema metadata | `@child kind=…` |
| JSON Schema | `"x-semio-child": "…"` |
| Proto schema metadata | `@child kind=…` |

Each form must accept `s.stdio.semio` and reject `s.stdio.semio.kit`. The replacement test should also contain a same-kind parent/child declaration that returns no policy breach. It must leave the child target's standard/subset contract to typed parsers and schema references; e.g. Curation requires the target dialect to have `standard: v1` and `subset: kit`, not a fabricated fourth kind segment.

`policyCanonicalArtifactKindBreaches` itself correctly checks three-segment declaration grammar, but its doc comment has the stale opposite claim at `📜️script.ts:30182-30193`, specifically lines `30185-30187`. Its declaration-only scope can remain. Its prose and the composition aggregation documentation at `30398-30403` must stop describing child-slot type-DAG acyclicity as a static invariant.

## Verified: Curation preserves the corrected parent/child boundary

The Curation parent declares the real kind in all concrete metadata inspected:

- Rust artifact, snapshot, and diff: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:17`, `📸️snapshot/🦀️.rs:15`, and `🔺️diff/🦀️.rs:19`.
- TypeScript artifact and diff: `🧬️schema/🟦️.ts:15` and `🔺️diff/🟦️.ts:10`.
- GraphQL artifact, snapshot, and diff: `🔗️.graphql:63`, `📸️snapshot/🔗️.graphql:4`, and `🔺️diff/🔗️.graphql:32`.
- JSON Schema artifact, snapshot, and diff: `🔣️.json:13-16`, `📸️snapshot/🔣️.json:13-16`, and `🔺️diff/🔣️.json:20-30`.
- Proto artifact, snapshot, and diff: `🛰️.proto:69-70`, `📸️snapshot/🛰️.proto:6-7`, and `🔺️diff/🛰️.proto:37-38`.

The subset is enforced where it belongs. TypeScript calls `parseSemioChild(..., "kit", ...)` for the full artifact and diff at `🧬️schema/🟦️.ts:63-65` and `🔺️diff/🟦️.ts:33-39`. Native snapshot and diff validation call `validate_semio_child_identity(..., "kit")` at `📸️snapshot/🦀️.rs:51-55` and `🔺️diff/🦀️.rs:46-52`. Its committed invalid-child vectors explicitly reject an identity mismatch, fourth-segment kind, incorrect standard, and incorrect subset at `🧫️fixtures/🪪️document-contract/🔣️.json:22-68`.

The ownership relocation is also structurally clean. `CurationDiff::apply_to_artifact`, `diff_set_snapshot`, and `MutationDiff<CurationSnapshot>` are co-located in the schema diff owner at `🧬️schema/🔺️diff/🦀️.rs:197-295`; this audit found no duplicate of those symbols outside that subtree. Schema contains no Curation `ArtifactDsl`, `ArtifactPack`, or named JSON/text codec declaration. Those declarations are now only in their IO leaves: text at `🚪️io/📸️snapshot/📝️text/🦀️.rs:39-62`, Pack at `🚪️io/📸️snapshot/💾️binary/🦀️.rs:7-23`, and JSON at `🚪️io/📸️snapshot/🔣️json/🦀️.rs:1-6`. The set-artifact-json command imports the IO JSON decoder at `✏️editor/🎮️commands/🗿️set-artifact-json/🦀️.rs:3-25`.

The stored native validation output is successful, although it was not re-run for this audit: `🗑️generated/curation-document-contract-native-3.log` reports one passing Curation contract test and its `[DEBUG]` check for JSON/text/Pack exact Kit identity and rejected invalid child replacement. No Curation ownership regression was found in the inspected boundary.

## Verified: remaining fourth-segment child metadata outside Curation

The corrected 57-annotation Stdio sweep did not cover these active declarations. They still encode the subset in `kind` and therefore fail the canonical three-segment grammar:

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:17` and `📸️snapshot/🦀️.rs:26` use `s.stdio.semio.text`.
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:34` and `📸️snapshot/🦀️.rs:43` use `s.stdio.semio.image`.
- The corresponding Shooting TypeScript metadata remains inconsistent at `🧬️schema/🟦️.ts:20`, `🔺️diff/🟦️.ts:22`, and `📸️snapshot/🟦️.ts:20`.

These are verified source-level policy breaches, independent of runtime test status. Their target dialects should retain their exact subset (`text` or `image`) through the existing child validators while their declaration metadata changes to `s.stdio.semio`.

`x-semio-child-kind` is a distinct older metadata convention found elsewhere in the repository. It was not part of the corrected Stdio `x-semio-child` contract and should not be silently folded into the new syntax rule without a separately bounded migration and parity audit.

## Untested risks

- This audit did not run Cargo or rerun the Curation suite; the Curation validation statement above is evidence from the committed ticket output plus current source inspection.
- The replacement static policy can prove only that declaration metadata has the canonical grammar. Instance identity, a single owner, acyclicity, reachability, cancellation, and bounded traversal remain runtime/closure-law obligations. The existing neutral Store fixture and Graphlib/Ajv oracle are the correct owner for those proofs.
- Process3D's report records that its moved native diff-law verification was pending when it was written. This audit did not claim a Process3D native result; the active root verification remains the authority.
