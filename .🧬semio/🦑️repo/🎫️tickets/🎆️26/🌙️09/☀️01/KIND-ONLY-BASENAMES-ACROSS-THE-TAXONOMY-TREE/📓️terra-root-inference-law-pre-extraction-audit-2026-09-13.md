# Root Inference Law Pre-Extraction Audit

Date: 2026-09-13

This is read-only preparation for the root inference-law slice. No broad root, native, or Nx command was run.

## Current Source Map

All inference behavior remains in /Users/ueli/Documents/semio/📜️script.ts.

| Concern | Current declarations | Required boundary |
| --- | --- | --- |
| Discovery and identity | policyListInferenceDirs 20969; policyFindAllInferencesDirs 20982; policyArtifactRootOfInferencesDir 21000 | inference discovery |
| Family structure | policyInferenceFamilyRootCompletenessBreaches 21010; policyInferenceSlugLeafPresenceBreaches | inference family laws |
| Derivation | policyInferenceImplPresenceBreaches; policyInferenceEmojiUniquenessBreaches | derivation laws |
| Assembly | policyInferenceNormalizeToken; policyInferenceAssemblyCoverageBreaches | assembly law |
| State separation | POLICY_DERIVED_MARKER; policyDerivedMarkerLeakBreaches | state-separation law |
| Composition | policyInferenceFamilyBreaches 21284 | aggregate only |

The aggregate has two live root consumers. VerifyScript.runGate includes it in a collection filtered to high priority, while the lint/report aggregate includes all findings. Current inference findings are medium or low. Extraction must retain reporting without manufacturing a gate or hiding findings.

## Reuse And Source Admission

The existing source-access owner at /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts lines 29-68 distinguishes directory/file from missing, unreadable, symlink, and wrong-kind using lstat. Its convenience wrappers at lines 70 and 75 collapse every unavailable result to an empty list/string. Current inference bodies use those wrappers and existsSync, so unreadable or symlinked input can look like ordinary absence. The new laws must consume structured admission results and report unreadable/symlink evidence rather than certify them clean.

Artifact-root derivation already belongs to /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts line 48. Rust field parsing belongs to /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts line 22. Assembly coverage must import those owners directly. Root-only policyWalkRelFiles 19725 and policyLineOfIndex 15295 need suitable shared owners before state-separation moves; no reverse import to root.

## Portable Controls Required

- Existing family only: an absent inference facet yields no completeness breach, while a present family missing canonical root leaves does.
- Every slug needs real Rust and TypeScript leaves. Both an InferredField implementation and a public whole-snapshot function are valid. Empty and export-empty TypeScript are not.
- Emoji reuse across families is valid; reuse within a family is rejected. Missing emoji is medium and U+FE0F remains low severity.
- Assembly accepts normalized kebab, snake, camel, Pascal, and scalar-name equivalence, then rejects orphaned slugs and uncovered fields.
- A derived marker in a snapshot component is rejected; one in an inference concern is allowed.
- Missing, unreadable, symlinked, wrong-kind, and ancestor-symlink candidates stay distinguishable. Traversal must not follow any symlink and unreadable input cannot collapse to zero findings.

No body-hash or current live finding total belongs in the fixture.

## Native Opportunity And Consumer Closure

Real Rust unit fixtures exist under inference concerns, for example /Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🧪️tests/🔬️unit/🦀️.rs. A later focused native invocation can prove a selected derivation but cannot prove the TypeScript source scanner or admission model. None was run here.

The extracted aggregate must be a direct import for root report/enforce consumers. Fixture/schema/test, package route, project inputs, launch seed and generated launch must cover every external owner and source-data leaf. I found no current dedicated registered inference target in the inspected repo-lib command/project routes; extraction needs a focused route rather than full-root policy evidence.

## Limits

This report records current sources and fixture opportunities only. The 192-owner/2,000-finding artifact diagnostic is a separate artifact-law observation and is not an inference expected-value snapshot.
