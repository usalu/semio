# R15 Mutation and Inference Law Exploration

Read-only static audit, 2026-10-10. No tests executed; findings distinguish source evidence from runtime proof.

Let S be `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`.

## Remaining Confirmed R13 Gaps

- `S/🧬️mutations/🔏️set-classification-system/🦠️mutation/🦀️.rs:17` still authors `entries: Option<Vec<ClassificationItem>>`; line 23 copies the entire vector into the patch. R13 R-L6 explicitly requires keyed entries and individual entry mutations. No `set-classification-entry` or `remove-classification-entry` paths were found.
- `S/🧬️mutations/🛠️set-property-template/🦠️mutation/🦀️.rs:13,15,21` still authors whole vectors for `applies_to` and `properties`. R13 R-L6 requires a reference set for applies_to and keyed property definitions with individual mutations. No `set-template-property` path was found.
- `S/📸️snapshot/💠️values/🦀️.rs:5,779` still declares `PropertyKind` and `PropertyDef.kind`. `S/📸️snapshot/🏷️property-kit/🦀️.rs:9,35` retains its duplicate kind vocabulary. This is explicitly unfinished R13 removal work, not a new rule invented by this audit.
- `S/💡️inferences/🕸️model-graph/🦀️.rs:61` contains 34 NodeKinds but no Finish or Bodies node kinds. R13 required both projections to be registered; inspect their use before implementing nodes because some work may have been renamed into Surface or Solid.

## R13 Gaps Already Addressed in Source

- Production mutation sources no longer import inference modules in the scanned results; remaining matches belong to tests. The authored helper relocation appears landed. Preserve the ownership boundary.
- The old graph registry path is gone. `S/💡️inferences/🕸️model-graph/🧳️instance/🦀️.rs:30` lends sessions through framework `ArtifactInstanceOperationOwnerHandle::lend_inference`; no BIM global session registry remains in that module.
- Its `shown` function at line 39 preserves the last complete inference and adds InferenceFault diagnostics instead of replacing the model with an empty inference.
- `S/💡️inferences/🕸️model-graph/🧮️compute/🦀️.rs:460,470` passes annotation indexes into plan/view inputs. The prior annotations=None defect is fixed in source.
- `S/💡️inferences/🕸️model-graph/🧪️tests/🔬️unit/🦀️.rs:90` now contains `every_projection_is_cache_transparent_warm_equals_cold_equals_uncached`. Its presence is not evidence of a passing run.

## Verification Targets

- Existing mutation fixture cases `🧫️fixtures/🧬️mutations/🔏️set-classification-system/🚫️replaces-the-entries` and `.../🛠️set-property-template/🚫️replaces-the-definitions` must be replaced with keyed-entry fixtures alongside snapshot, diff, inverse and facet schemas.
- Property template third-party parity must continue to witness resulting effective properties; `S/💡️inferences/🏷️effective-properties/🧪️tests/🔬️unit/🦀️.rs:99` already targets cache transparency and a property edit avoiding geometry recomputation.
- Run graph cache transparency and instance ownership tests after any changes, including `.../🕸️model-graph/🧳️instance/🧪️tests/🔬️unit/🦀️.rs` and `.../🕸️model-graph/📡️session/🧪️tests/🔬️unit/🦀️.rs`.
- Counter thread_locals remain in compute at lines 44 and 57; these are instrumentation, not inferred model storage, and are lower priority than domain data defects.

## Architecture Guardrails

The existing protocol already defines pure `Inference<P>::infer`, `InferenceSpec` read paths and `DiffRegions::touches`; cache correctness requires sound read/write intersections. Native Mutation leaves emit handcrafted sparse diffs and concrete semantic inverse mutations from prestate. The framework owns mounted session lifetimes. Keep BIM state, mutation and inference owners within the model artifact taxonomy.
