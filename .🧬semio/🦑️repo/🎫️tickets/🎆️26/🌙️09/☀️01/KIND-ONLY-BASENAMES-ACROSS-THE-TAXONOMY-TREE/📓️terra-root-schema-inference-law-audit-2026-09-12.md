# Root Artifact Schema Law Extraction — Pre-Extraction Audit

**Scope:** artifact-schema ownership only. This read-only assessment covers current artifact owner discovery, facet loading, five artifact laws, representation parity, their field-parity test, and their shared filesystem primitive dependency. It deliberately excludes app/surface and inference ownership. It records no extraction acceptance and ran no broad or global target.

## Current Boundary

The current root implementation is entirely in [`📜️script.ts`](../../../../../../../📜️script.ts):

| Current behavior | Current location | Intended extracted responsibility |
| --- | ---: | --- |
| Standard/subset owner discovery | 21318 | Artifact source discovery |
| Normative JSON title lookup | 21347 | Facet identity |
| Schema-format leaf projection | 21387 | Facet loading and real field extraction |
| Facet completeness | 21436 | Artifact facet law |
| Field parity | 21497 | Artifact field law |
| Snapshot state parity | 21564 | Artifact state law |
| Diff coverage | 21623 | Artifact diff law |
| Aggregate | 21707 | Artifact schema law composition |
| Document representation parity | 22226 | Artifact representation law |

The behavior has a coherent extraction boundary: artifact discovery; facet projection; five independent laws; aggregation; and the separate representation-parity evaluation. The language-specific parsing must remain direct dependencies of the accepted field-discovery owners under `library/🧬️schema/🔍️field-discovery`; an extracted artifact owner must not reintroduce a root compatibility façade.

The field-parity test currently imports exactly three root APIs at [`🟦️.ts:11`](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts:11): discovery, aggregate artifact breaches, and representation parity. That is a real remaining root coupling. Rebind it directly to the three extracted owners.

## Consumer Closure

The required current consumer graph is:

1. The field-parity test directly imports artifact discovery, aggregate law, and representation parity. It independently uses TypeScript compiler AST/module graphs, Ajv, and fast-glob.
2. The `verify artifact-field-parity` command imports the test oracle, then calls representation parity for its report/enforce path at [`📜️script.ts:7668`](../../../../../../../📜️script.ts:7668).
3. Full root verification calls the aggregate at [`📜️script.ts:8224`](../../../../../../../📜️script.ts:8224).
4. The workspace lint aggregate calls the same aggregate at [`📜️script.ts:26557`](../../../../../../../📜️script.ts:26557).
5. Representation parity reads the existing abstraction-ownership fixture as source data. That dependency must be recorded explicitly, but the abstraction/surface evaluator itself remains outside this slice.
6. Facet loading calls the existing Rust, TypeScript-with-module-resolution, GraphQL, JSON Schema, Protobuf, and comparison owners directly. The TypeScript case must retain resolution from each actual source file directory.
7. The registered `artifact-field-parity-test`, `-report`, and `-enforce` targets in [`📋️project.json:1915`](../../../../../../../📋️project.json:1915) must keep routing through the real owners. The separate abstraction route is not evidence for the artifact extraction.

No code in an extracted owner should import root `📜️script.ts`. Root remains a router and the two registered aggregate consumers above.

## Shared Read and Discovery Boundary

Artifact discovery directly reaches only:

- `POLICY_SKIP_DIRS`;
- `policyReaddirSafe` at [`📜️script.ts:17062`](../../../../../../../📜️script.ts:17062);
- `policyReadFileSafe` at [`📜️script.ts:17083`](../../../../../../../📜️script.ts:17083).

`policyWalkRelFiles` and `policyLineOfIndex` are used elsewhere in root but are not in the artifact-law closure. They should not be pulled into this slice simply because they are presently neighbouring shared primitives.

The two direct primitives currently suppress every filesystem failure: directory reads return an empty array, and file reads return an empty string. That can falsely make a missing or unreadable source look clean. Move truly shared filesystem behavior once to an existing source/discovery concern; do not hide it inside the artifact owner if its other root consumers also need it. The new contract needs typed/evidenced outcomes that distinguish absence, unreadability, and malformed content.

## Required Portable Controls

The extracted slice needs language-independent, fixture-driven controls for all of these cases:

- Missing admitted root versus unreadable admitted root; unreadable directory, unreadable leaf, and malformed JSON must yield evidence or a controlled failure, never an empty clean result.
- No-follow discovery of directory and leaf symlinks, including a symlink to an otherwise valid artifact tree. Dot, generated, node_modules, and target entries remain excluded.
- Exact standard/subset admission only: an artifact nested within an owner is not itself another owner; missing standards or subsets is not admitted. Ordering must be deterministic and agree with fast-glob for a legitimate tree.
- Missing facet directory, missing normative JSON leaf, missing configured format leaf, invalid normative JSON, and absent/invalid title must remain distinguishable.
- Every configured schema format must be projected from taxonomy. WIT or an unrecognised/default branch must not silently pass a format with required semantics.
- Existing real TypeScript resolver cases remain required: aliases, re-exports, imported declarations, cycles, and a missing resolution boundary from the actual source directory.
- Field-law negatives: missing/extra fields, optionality/cardinality mismatch, protocol-map optional exemption, and the fixed-list-to-list exception only for its admitted TypeScript/GraphQL/Protobuf formats.
- State-law negatives: missing or extra non-artifact state, and a shape mismatch. Diff-law negatives: a nontransient missing from diff and a transient incorrectly included in diff.
- Export-identity negatives for a missing JSON title and mismatched declared type in each material language.
- Representation-parity negatives for a missing declaration and missing/extra language fields against a normative JSON document. The fixture-to-owner source-data edge must be asserted.

The existing fixture already provides independent TypeScript compiler, Ajv, and fast-glob oracles. Extend structured input/output descriptors rather than storing root source-body hashes. Do not fold app/window/surface constraints into an artifact control.

## Current Evidence and Debt

The packet reports a prior direct root `verify abstraction-ownership test` success (2.967 seconds) and a separate read-only invocation of current artifact discovery/aggregate APIs (10.175 seconds). Those are prerequisite evidence, not execution by this audit and not extraction acceptance.

The current artifact diagnostics are **192 owners and 2,000 findings**: 377 facet-completeness, 2 normative-leaf, 1,363 field-parity, 83 state-parity, 74 diff-coverage, and 101 type-name-parity. They are existing policy debt. Do not snapshot their counts as expected output, suppress them, or broaden format exceptions to make the extraction appear green.

## Acceptance Criteria

Acceptance requires an acyclic direct-owner import graph; direct rebinding of every consumer listed above; source-as-data registration for both fixtures; registered test/report/enforce and launch provenance; and focused portable/native checks that prove the hostile cases. A successful abstraction-ownership route alone cannot establish artifact-law extraction because it is a separate concern.

**Status:** pre-extraction audit complete. No product files changed and no artifact-law target was executed by this audit.
