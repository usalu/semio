# 📓 Executor stdio-mid report

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{📐️step,📜️docx,🎞️gif,🏗️ifc,🎨️svg,📽️pptx,📕️xlsx}` (219 audited kinds, 150 violating).

## ⚠️ Verification status (read first)

- Every edited Rust file (404) parses (`rustfmt --check` reports no syntax error) and every edited JSON file (93) parses. Nothing else is verified.
- NOTHING IN THIS SCOPE HAS BEEN COMPILED OR TESTED. Earlier gated `cargo check --target wasm32-wasip2` runs were killed (exit 137 / OOM / lock contention); the one that reached rustc (docx) stopped on 9 errors inside `semio-framework-plugin` (`RetirementStep::ProcessedBytes/Failure` non-exhaustive match, `SharedOwner<String>` vs `Arc<String>`), i.e. a peer's `🌱️value/🗂️ordered` change, not in this scope. The final xlsx check was still compiling framework crates when free disk fell to 1.2 GB (peers' fleet builds), so I terminated my own check chain (gate, `check.sh`, cargo) to protect the shared disk; its partial log is `🗑️generated/stdio-mid/check-📕️xlsx.log`.
- Per the wave-2 ruling no test build was started. Every test listed below is authored and NOT RUN.
- Expect compile errors on first contact: the volume is ~400 Rust files of uncompiled edits.

## 🧭 Result per subset

| Artifact / subset | Kinds after | Shape of the change |
|---|---|---|
| step ap214 base | 9 | `set-snapshot` and the snapshot codec deleted; `StepDiff::inverse` is concrete (entities restored at base index); header slots read from base; `net_mutations` (name/arg edits, dependency-ordered insert/remove) feeds the editor; middle-row test |
| step cc1-cc6 | 4-5 each | shared `class_diff`/`class_inverse`/`invert_class_edit_restoring`/`edited()` gone; ladder rewritten as snapshot-level reads producing `entity_diff`/`restore_diff`; new atomic `restore-entities` kind; `set-shape-representation` gained `index` |
| docx base | 17 | `agg_diff`/`agg_inverse`/`apply_to_snapshot`/set+patch removed; addressed kinds go through `*_plan` builders (revision-bound addresses for inverses); `set-part` gained `index`; part-level `net_mutations`; `apply_addressed_xml_mutation_in_place` kept as the documented retained-execution seam |
| docx strict / transitional | 8 / 4 | namespace, relationship-base, conformance-attribute, VML and alternate-content diffs built from `XmlNodeDiff`s; `stamp_conformance_class_mutations` replaces the snapshot stamp |
| gif 87a / 89a | 10 / 19 | `set-snapshot` removed, `move-*` built without cloning the base, restore inverses replaced by concrete ones, `apply_*` uses the central applier |
| ifc 4 | 9 | `set-snapshot` removed, residual doc touch-ups |
| ifc 2x3 base / cobie / sav / cv20 | 3 / 5 / 4 / 4 | `mvd` helpers rewritten as diff builders (`entity_diff`, `argument_diff`, `view_definition_diff`, `upsert_diff`, `remove_diff`); entity kinds carry `index`; set-snapshot leaves gone; oracle `upsert_simple_at` |
| svg base / basic / tiny | base leaves + 9 / 9 | base `Restore(SvgDiff)` variants deleted, inverses concrete per leaf, `strip-non-tiny`/`restore-non-tiny` no longer clone the base |
| pptx base | 8 | `PptxDiff` regenerated as the docx named-triple engine for plain types; `*_plan` builders in `xml-address`; new `replace-xml-node`; editor uses `net_mutations`; `whole_document_operation` removed |
| pptx strict / transitional | 9 / 5 | declarative `diff_retarget_namespace`/`diff_conformance_attribute`/`insert_xml_part_diff`/`remove_xml_part_diff`; `insert-vml-part` gained `index` |
| xlsx base | 9 | `canonical_edit` rewritten as plan builders (`set_cell_plan`, `insert_cell_plan`, `remove_cell_plan`, shared-string and sheet plans); set/patch removed; `insert-sheet` and `insert-shared-string` gained `index`; `remove-cell` drops a row it empties so insert/remove are exact inverses; `net_mutations` for the editor |
| xlsx strict / transitional | 8 / 6 | same builders as pptx on `XlsxDiff`; new `set-relationship-base`; `insert-vml-part` gained `index` |

Cross-cutting: every `🚪️io` builder `mutate` now calls `store::apply_outcome(&snapshot, Mutation::diff(..))` instead of an `apply_*_mutation(&mut Snapshot)` helper; the `apply_*_mutation` helpers remain only as thin `Mutation::diff` + `protocol::apply_diff` test conveniences. All `.apply(&base)` calls on diffs were rewritten to `protocol::apply_diff(&diff, &base)`.

## 🧾 Per-kind classification (before codes from the audit, after)

Codes: V1 = diff not declarative / whole snapshot, V2 = inverse derived or whole-snapshot restore, V3 = leaf applies or clones. The 69 kinds that audited clean keep their leaf logic; their aggregator and apply path changed with the rest of the subset.

| Subset | Kind | Before | After |
|---|---|---|---|
| 📐️step/ap214/base | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📐️step/ap214/base | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📐️step/ap214/cc1 | set-file-schema | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc1 | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📐️step/ap214/cc1 | remove-shape-representation | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc1 | set-product-identity | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc2 | demote-shape-representation | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc2 | set-file-schema | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc2 | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📐️step/ap214/cc2 | set-shape-representation | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc2 | set-product-identity | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc3 | demote-shape-representation | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc3 | set-file-schema | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc3 | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📐️step/ap214/cc3 | set-shape-representation | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc3 | set-product-identity | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc4 | demote-shape-representation | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc4 | set-file-schema | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc4 | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📐️step/ap214/cc4 | set-shape-representation | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc4 | set-product-identity | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc5 | demote-shape-representation | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc5 | set-file-schema | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc5 | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📐️step/ap214/cc5 | set-shape-representation | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc5 | set-product-identity | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc6 | set-file-schema | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc6 | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📐️step/ap214/cc6 | set-shape-representation | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📐️step/ap214/cc6 | set-product-identity | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎞️gif/87a/any | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🎞️gif/87a/any | move-image | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎞️gif/87a/any | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🎞️gif/89a/base | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🎞️gif/89a/base | move-frame | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎞️gif/89a/base | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🏗️ifc/2x3/base | set-header | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/base | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🏗️ifc/2x3/base | remove-instance | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/base | upsert-instance | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/base | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🏗️ifc/2x3/cobie | set-facility-name | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/cobie | set-view-definition | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/cobie | set-floor-elevation | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/cobie | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🏗️ifc/2x3/cobie | set-space | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/cobie | set-type-assignment | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/cv20 | set-structural-entity | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/cv20 | set-view-definition | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/cv20 | set-product-placement | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/cv20 | set-project-units | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/cv20 | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🏗️ifc/2x3/sav | set-load-group | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/sav | set-view-definition | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/sav | set-group-assignment | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/2x3/sav | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🏗️ifc/2x3/sav | set-analysis-model | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🏗️ifc/4/any | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🏗️ifc/4/any | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🎨️svg/1.1/base | set-text | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎨️svg/1.1/base | set-attribute | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎨️svg/1.1/base | set-doctype | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎨️svg/1.1/base | set-declaration | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎨️svg/1.1/base | insert-element | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎨️svg/1.1/base | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🎨️svg/1.1/base | set-transform | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎨️svg/1.1/base | set-element-name | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎨️svg/1.1/base | set-view-box | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎨️svg/1.1/base | remove-element | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎨️svg/1.1/base | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🎨️svg/1.1/basic | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🎨️svg/1.1/basic | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🎨️svg/1.1/tiny | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 🎨️svg/1.1/tiny | strip-non-tiny | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 🎨️svg/1.1/tiny | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📜️docx/ecma-376/base | set-block-content | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | insert-block | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | insert-table-row | V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | remove-block | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | remove-table-row | V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | set-style-based-on | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | set-style-name | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | set-part | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📜️docx/ecma-376/base | insert-style | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | remove-part | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | insert-xml-node | V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | remove-style | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | remove-xml-node | V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/base | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📜️docx/ecma-376/strict | set-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/strict | insert-vml-part | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/strict | set-main-namespace | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/strict | remove-alternate-content | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/strict | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📜️docx/ecma-376/strict | remove-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/strict | remove-vml-part | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/strict | insert-alternate-content | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/transitional | set-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/transitional | set-main-namespace | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📜️docx/ecma-376/transitional | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📜️docx/ecma-376/transitional | remove-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/base | set-shape-text | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/base | insert-slide | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/base | remove-slide | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/base | set-shape-position | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/base | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📽️pptx/ecma-376/base | move-slide | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/base | remove-shape | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/base | insert-shape | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/base | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📽️pptx/ecma-376/strict | set-drawing-namespace | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/strict | set-main-namespace | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/strict | remove-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/strict | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📽️pptx/ecma-376/strict | insert-alternate-content | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/strict | set-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/strict | insert-vml-part | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/strict | remove-vml-part | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/strict | remove-alternate-content | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/transitional | set-drawing-namespace | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/transitional | set-main-namespace | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/transitional | remove-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📽️pptx/ecma-376/transitional | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📽️pptx/ecma-376/transitional | set-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/base | set-cell | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/base | insert-sheet | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/base | insert-cell | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/base | remove-sheet | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/base | rename-sheet | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/base | remove-shared-string | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/base | insert-shared-string | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/base | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📕️xlsx/ecma-376/base | set-shared-string | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/base | remove-cell | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/base | patch-snapshot | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📕️xlsx/ecma-376/strict | set-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/strict | insert-vml-part | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/strict | set-main-namespace | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/strict | set-worksheet-content-type | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/strict | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📕️xlsx/ecma-376/strict | set-relationships-namespace | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/strict | remove-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/strict | remove-vml-part | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/transitional | set-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/transitional | set-main-namespace | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/transitional | set-worksheet-content-type | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/transitional | set-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | REMOVED (kind deleted; callers moved to concrete kinds or genesis/load) |
| 📕️xlsx/ecma-376/transitional | set-relationships-namespace | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |
| 📕️xlsx/ecma-376/transitional | remove-conformance-attribute | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | declarative sparse diff + concrete absolute-value inverse; central apply |

### Added kinds

| Artifact | Subset | Kind | Status | Reason |
|---|---|---|---|---|
| 📐️step | ap214/cc1-cc6 | restore-entities | NEW | atomic restore of exact rows (id, entity, base index); inverse of set-product-identity / set-shape-representation / demote / remove-shape-representation |
| 📽️pptx | ecma-376/base | replace-xml-node | NEW | addressed node replace; carries the old node as the inverse and is the target of the editor's net mutations |
| 📕️xlsx | ecma-376/strict + transitional | set-relationship-base | NEW | the xlsx stamp needs the relationship-type base to move (oracle projection reads `relationshipTypes`), as docx and pptx already have |

## 🧪 Tests authored (all NOT RUN)

Run from the repo root through the gate, one crate at a time, e.g. `"$T/🚦️gate.sh" stdio-mid -- cargo test --locked --lib -p semio-s-artifact-stdio-<crate>` (crates: step, docx, gif, ifc, svg, pptx, xlsx). Not run because of the wave-2 ruling and the peer compile break above.

- `assert_mutation_inverse_sum_law` over every demo/representative kind in: step base and cc1-cc6 (`inverse_restores_before`, middle-row `restore-entities` test), ifc 2x3 cobie/sav/cv20/base (`removing_a_middle_row_is_restored_at_its_original_index`), pptx base (`every_demo_kind_satisfies_the_inverse_sum_law`, `removing_a_middle_slide_or_shape_is_restored_at_its_original_index`), pptx strict/transitional (`every_stamp_kind_satisfies_the_inverse_sum_law_and_stamping_round_trips`, strict: VML/alternate-content at a middle position), xlsx base (`every_demo_kind_satisfies_the_inverse_sum_law`, `removing_a_middle_member_is_restored_at_its_original_position`, `removing_a_middle_shared_string_renumbers_the_references_behind_it`, set-cell `widens` fixture), xlsx strict/transitional (stamp sum law and VML/content-type at a middle position).
- docx base: `every_demo_kind_satisfies_the_inverse_sum_law`, `removing_a_middle_member_is_restored_at_its_original_position` (middle block, table row, style). gif 87a/89a and svg base/basic/tiny: sum-law tests from the earlier passes (`set-frame-delay` `slows`, `set-image-pixels` `repaints`, per-subset unit files).
- Language-agnostic fixtures: step `set-entity-arg/restamps`, `restore-entities` history inputs, ifc `history-snapshots/rename-file/snapshot`, xlsx `set-cell/widens` before/after quintet halves, pptx `history-snapshots/retitles/snapshot`. Feature files lost every set/patch row; `mutate-stamp-strict-class` / `inverse-stamp-strict-class` replace the set-snapshot stamp scenarios in docx/pptx/xlsx strict+transitional.
- Third-party validation: the existing independent oracles (calamine, openpyxl, python-pptx, python-docx, ifcopenshell-style reference, quick-xml) are untouched in role; their JSON vector catalogs lost the removed kinds and gained `replace-xml-node` (pptx) and `set-relationship-base` (xlsx).

## ❗ Open issues

1. docx `insert-vml-part`/`remove-vml-part` have no position-carrying payload (pptx and xlsx gained `index`); a VML part removed from the middle of the part list is re-inserted at the end.
2. Nothing compiled. First `cargo check` per crate will surface typo-level errors in the generated leaves (`use super::*` paths, `index` fields added to `InsertSheet`, `InsertSharedString`, `InsertVmlPart`, `SetPart`, `SetShapeRepresentation`, entity kinds in ifc).
3. V5 (absorb soundness): id-keyed diffs (step entities, ifc instances) still absorb positionally; the sum law will expose it where a middle row is deleted and re-inserted in one sequence.
4. Lossy inverses where the payload vocabulary is closed: docx `set-main-namespace` on mixed-namespace packages, `remove-alternate-content` (inverse appends only the canonical node), `remove-vml-part` (content-type override position), `set-part` override handling; pptx `remove-alternate-content` and VML override position; xlsx `remove-cell` (cell attributes such as style `s`), `remove-sheet` (inverse re-inserts typed cells only, not `cols`/views/merge data), `set-shared-string` and `remove-shared-string` (rich-text runs collapse to text), `set-cell` formula attributes when a formula cell is overwritten by a non-formula value, xlsx `set-worksheet-content-type` when no override existed; cobie/sav/cv20 row builders when base instances are not shaped like the row builders.
5. step ladder `entity_diff`/`restore_diff` still call `StepEntityDiff::between` over two supplied entities (entity-granular, never snapshot-granular).
6. Natural-file import (`whole_document_operation`) was removed in pptx, xlsx (base, strict, transitional) and docx/ifc editors; it needs a genesis/load seam, tracked by the design's `stdio-small`/spine ruling.
7. `patch-snapshot` removal drops the EDM-preamble editing path in ifc and the details JSON-pointer editing in xlsx/pptx/docx editors: editors now emit `net_mutations` and refuse an edit the leaves cannot express.
8. Fixtures deleted rather than re-homed: docx `bolds` readme-afters, pptx `patch-snapshot/wire-witness`, xlsx `patch-snapshot/wire-witness`, pptx/xlsx/docx `set-snapshot` wire witnesses, `📸️set-snapshot-applied` evidence for strict/transitional pptx and xlsx (their `testEvidence` entries were removed).
9. The new xlsx `set-relationship-base` and pptx `replace-xml-node` have schema/feature/manifest entries but no python-generated `testEvidence` fixture bytes.
10. The mass signature patch of 22 diff files in other stdio artifacts (`DiffAlgebra::inverse`/`MutationDiff::apply` signature) was applied outside this scope by the spine; not re-verified here.
11. Disk is the blocker for compile verification: 1.2 GB free at the end. Re-run `"$T/🗑️generated/stdio-mid/check.sh" <artifact>` per crate once the disk has headroom and the framework plugin compiles.

## 📂 Files touched

Everything under the seven artifact directories reported by `git status` (404 Rust, 93 JSON, plus features, grammars, protocol files, `.proto`/`.graphql`/`.ts` payload mirrors, fixtures and oracle scripts). Scratch: `🗑️generated/stdio-mid/` (`pconf.py`, `xconf.py`-style generators, check logs) — to be removed by the coordinator at close. Shared framework files were not edited by this executor.

## 🌊 Wave 3 (no feature loss, exact inverses)

Status: WRITTEN, NOT COMPILED, NOT TESTED. Foundation was RED (framework value/replication: `owned_retirement`, `next_close_byte_demand`) every time I tried; before the new rule my gated `cargo check` chains (xlsx, docx, pptx, step, ifc, gif, svg) either queued behind peers or stopped inside `semio-framework-*`. The one artifact-level result I got (xlsx, name resolution only) was fixed (`remove_vml_part_inverse` lost by a script overwrite). Check from each artifact's workspace dir: `"$T/🗑️generated/stdio-mid/check.sh" <artifact>` (cd into `<artifact>/📦️packages/🦀️rust`, `cargo check --offline --target wasm32-wasip2`; `--locked` fails because the standalone `Cargo.lock` is stale).

### Natural-file / document import (load, not a mutation)
- All 26 editors (step base+cc1-6, docx x3, gif x2, ifc x5, svg x3, pptx x3, xlsx x3) override `ArtifactEditor::import_media` with `semio_s_artifact_stdio_contract::import_media_as_load::<Self>` (natural-file port through the editor's own codec, `artifact:in` through the pack): an `Effect::LoadDocument`, no history row.
- Framework gap (not fixable in this scope): the reserved natural-file import JOB (`🔌️plugin` `decode_synchronous` / controlled decoder) still calls `E::whole_document_operation` and fails `NotImplemented` when it is `None`. docx/pptx/xlsx/svg (the editors that declare a natural-file codec) need that job to take the same load route; the media path works.

### Details-pane editing
- Every editor already routes a snapshot edit through `apply_snapshot_edit` + the artifact's `net_mutations` (concrete kinds, replay-verified). Extended: xlsx `net_mutations` now also dispatches `rename-sheet`, `insert-sheet`, `remove-sheet`, `insert/remove/set-shared-string` (strings first, sheets, cells, string removals last).
- Still refused (no concrete kind exists for the field, the old `patch-snapshot` was the only route): ifc EDM preamble and document schema, svg prolog/epilog (and declaration/doctype in the basic and tiny profiles), pptx/xlsx OPC-level fields and part presence, step document schema. Adding `set-edm-preamble` / `set-prolog` / `set-epilog` kinds is the follow-up; each costs a leaf, schema, codecs, oracle and feature rows.

### Lossy inverses closed (optional payload fields carry the missing data)
| Kind | New field(s) | Effect |
|---|---|---|
| docx/pptx/xlsx `insert-vml-part` | `index`, `override_index` | `remove-vml-part` inverse restores the part and its content-type override at their exact positions (docx: `xml_part_positions`, `insert_xml_part_diff`) |
| docx/pptx `insert-alternate-content` | `node`, `index` | inserts the given fallback at an index; inverse of `remove-alternate-content` is one insert per removed child, listed last-to-first (replay reverses) |
| docx/pptx `remove-alternate-content` | `index` | removes only the child at that index (every fallback when absent); inverse of an insert removes exactly the child it landed on |
| docx `set-part` / `remove-part` | `override_index` (and `index` now also positions XML parts) | `remove-part` drops the override with the part; `set-part` inverse restores part, override presence and both positions; an override the extension default already yields is dropped |
| xlsx `set-worksheet-content-type` (strict, transitional) | `override_index` | an explicit override is written at that position; without it a default-yielded type needs no override, so the inverse of adding one removes it |
| xlsx `set-cell`, `insert-cell` | `node` | verbatim cell element (style `s`, formula attributes, extra children); `remove-cell` / `set-cell` inverses carry it only when rebuilding from the typed value would differ |
| xlsx `set-shared-string`, `insert-shared-string` | `node` | verbatim `si` (rich-text runs); inverses carry it only when the typed text would flatten it |
| xlsx `insert-sheet` | `slot` (`XlsxSheetSlot`: entry attrs, workbook relationship, part path/type/document, part relationships) | `remove-sheet` inverse returns the sheet verbatim (cols, views, merges, state, own relationships) at its position |
| ifc 2x3 cobie/sav/cv20 `set-space`, `set-type-assignment`, `set-analysis-model`, `set-load-group`, `set-group-assignment`, `set-structural-entity` | `instance` | the base instance is carried verbatim when the typed row cannot rebuild it |

Text, binary, grammar, proto/graphql, TS and JSON-schema mirrors were updated for every new field (docx `set-part` TS parser also now accepts `index`).

### Not closable with a payload field
- docx/pptx `set-main-namespace` / `set-drawing-namespace` on a package that declares BOTH members of a pair: the forward retarget is not injective, so no single setter can restore which attribute held which member. Uniform packages (the stamp case) invert exactly.
- A redundant explicit content-type override equal to the extension default is dropped by `set-worksheet-content-type` without `override_index` (restored by the inverse, which carries the index).
- Owner order of the removed worksheet part's own relationships (xlsx `remove-sheet` inverse appends the owner).

### New tests (all NOT RUN)
docx: `every_demo_kind_satisfies_the_inverse_sum_law`, `removing_a_middle_member_is_restored_at_its_original_position`, strict `every_strict_kind_satisfies_the_inverse_sum_law` (VML at a middle position, alternate content by index), transitional `every_transitional_kind_satisfies_the_inverse_sum_law`; pptx strict VML/alternate-content law with `index`/`override_index`; xlsx `inverses_restore_what_the_typed_value_cannot_express` (styled cell, rich-text string, sheet entry with `state`), `net_mutations_reach_the_next_snapshot_for_strings_sheets_and_cells`.
Run: `cd <artifact>/📦️packages/🦀️rust && "$T/🚦️gate.sh" stdio-mid -- cargo test --offline --lib mutations` once foundation is GREEN.

## 🚦 Gate run 4 and foundation status (stdio-mid)

- svg tiny R8/R14 breaches fixed: `RestoreNonTiny`/`RestoredElement`/`RestoredAttribute` renamed to `ReinstateNonTiny`/`ReinstatedElement`/`ReinstatedAttribute` (kind string `restore-non-tiny` unchanged); `restore_node_diff` and `stripped_rows` now take paths by reference and recurse functionally (no `&mut`).
- `bun ./📜️script.ts verify mutation-outcome-law` re-run (log `🗑️generated/stdio-mid/gate-run-4.log`): 132 findings in total, 0 of them under `🗄️stdio` (all seven artifacts clean; remaining findings belong to remodel/note/puzzle/norm).
- Build status: BLOCKED ON FOUNDATION. `foundation.status` stayed `RED 18:58:20 native=101 wasm=101` for the whole 60+ minute foreground wait, so no cargo call was made. Everything in this report remains WRITTEN, NOT COMPILED, NOT TESTED (rustfmt syntax check and JSON parse only).
- Wave 3: details-pane editing via concrete kinds is done through `net_mutations`; natural-file import override delegates to `import_media_as_load` (load path); the framework's reserved natural-file import job (`decode_synchronous`) still calls `E::whole_document_operation` and needs a framework change.

## 🌊 Wave 4 (translators deleted: details-pane edits resolve to concrete kinds)

Status: WRITTEN, NOT COMPILED, NOT TESTED (foundation `RED 22:30:46 native=101 wasm=101`, no cargo call made; `rustfmt --check` syntax only).
This section SUPERSEDES the Wave 3 statement that details-pane editing is "done through `net_mutations`": every `net_mutations`, `snapshot_edit_mutations`
and `snapshot_edit_expected` in step/docx/gif/ifc/svg/pptx/xlsx is deleted (rg: no hit under the seven artifacts), and `xml_replace_document` is gone from docx.

### Mechanism (stdio-small "Edit-rules API")
Each editor implements `snapshot_edit_rules() -> &'static EditRules` (a `const` table in the new `🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs` of each aggregator) and,
where the kind's payload is computed from the gesture, `snapshot_edit_special(event, snapshot)` (`edit_rules::special`). `special` never rebuilds or compares a document:
it reads the pointer, reads the addressed entity of `base` and builds the kind of the gesture (`edited_subtree` patches only that one entity).
Whole-source replace is the contract's load (`Effect::LoadDocument`). Each `🧭️edit-rules` has a `🧪️tests` module (rule resolution + replay through the artifact's own apply).

| artifact | table (one kind per pointer) | special (computed gestures) | refused (no kind) |
|---|---|---|---|
| step base | header slots -> `set-file-*`; `/entities/*/name` -> `set-entity-name`; `/entities/*/args/*` -> `set-entity-arg`; insert/remove entity + arg | none | complex constituents, entity ids, schema, entity order |
| step cc1..cc6 | `/header/fileSchema/schemas` -> `set-file-schema` | any edit of `/entities/<row>` -> the class's single `restore-entities` row (`ladder::edit_restore_rows`, replaces `ladder::net_restore_rows`) | file description/name, schema, entity order |
| ifc 4 | header records -> `set-file-*`; entity name/arg; insert/remove entity + arg | none | complex constituents, ids, schema |
| ifc 2x3 (base + cobie/sav/cv20) | `/document/header` -> `set-header`; `/document/instances/*` -> `upsert-instance`; insert/remove instance | none | schema, EDM preamble, instance-id changes inside an upsert |
| gif 89a | screen size (carries sibling), gct, bg colour, aspect, loop, frame field setters, insert/remove frame/comment/app-extension | frame geometry (4 fields -> `set-frame-geometry`), frame `lct`/`plainText`/whole frame, comment text, app-extension (row replaced = remove + insert), moves (`move-frame`, comment/app-extension remove + insert) | `/schema` |
| gif 87a | screen size, gct, bg colour, aspect, image pixels/interlace, insert/remove image | image geometry, image `lct`/whole image, `move-image` | `/schema` |
| svg base/basic/tiny | base: `/doc/declaration`, `/doc/doctype` | everything below `/doc/root` through the shared `mutation_support::tree_edit` + a per-subset `TreeEditKit` (set-attribute / insert-element / remove-element / set-text / set-element-name): node path = `children/<i>` pairs, attribute by name + position, attribute move/insert/rename | prolog, epilog, root replace; basic/tiny also declaration/doctype/rename-element |
| docx (base, strict, transitional) | none (table empty) | `/xmlParts/<n>/document/root/...`: insert/remove child -> `insert-xml-node`/`remove-xml-node`, replacing a child or any edit inside an element -> `replace-xml-node` on the addressed element; `/xmlParts` insert/remove, `contentType` -> `set-part`/`remove-part`; `/opc/parts` rows and `contentType`/`bytes` -> `set-part`/`remove-part` | part document prolog/declaration/doctype/epilog, part path, OPC relationships/content-types/comment, row moves of parts |
| pptx (base, strict, transitional) | none | node edits below `/xmlParts/<n>/document/root` -> `replace-xml-node` (child replaced, else the enclosing element; text nodes lift to their element) | the whole OPC layer, which parts exist |
| xlsx (base, strict, transitional) | none | worksheet `<c>` (at/below) -> `set-cell` carrying the element verbatim and its typed value; `<c>` child of `<row>` -> `insert-cell` (vacancy at its `r`) / `remove-cell` / `set-cell`; shared strings `<si>` -> `set-/insert-/remove-shared-string`; workbook `<sheet name>` -> `rename-sheet`, removing a `<sheet>` -> `remove-sheet` | rows/cols/styles/merges, other parts, sheet insert, any other `<sheet>` attribute, the OPC layer |

New shared pieces: `ladder::edit_restore_rows` (step), `mutation_support::{TreeEditKit, tree_edit}` (svg), `canonical_edit::{PartRole, part_role, node_cell_value, shared_string_text, shared_string_position, sheet_element_name}` (xlsx).

### Docx T4
`set_part_diff` no longer replaces an existing XML part's document: an existing XML part accepts only a content-type change (document must equal the part's); node edits use the `xml-address` kinds. `xml_replace_document` is deleted. Stale fixtures `set-snapshot/🧾️wire-witness` under docx strict and transitional were removed (kind long gone).

### Other translators removed
svg `set-pixel-region` (three editors) and the shared-source path are loads (`load_example_effect(parse_dsl(source))`), not differenced edits. `net_mutations` unit tests (step base, xlsx) were replaced by the rule-resolution tests.

### Honest limits
- Not compiled: type/visibility errors are likely in ~15 new files (glob visibility of private parent imports, `PagedList` iteration, `extract_cell_value` plumbing); the first `cargo check` per artifact will tell. Commands once foundation is GREEN: `cd <artifact>/📦️packages/🦀️rust && "$T/🚦️gate.sh" stdio-mid -- cargo check --offline --target wasm32-wasip2`, then `cargo test --offline --lib edit_rules`.
- `bun ./📜️script.ts verify mutation-outcome-law` could not run at 22:33: `📜️script.ts:14961` is mid-edit by a peer (`Unexpected const`). The previous run (gate-run-4, before wave 4) showed 0 findings under `🗄️stdio`.
- xlsx/pptx/docx rely on revision-bound addresses computed against the current base; a multi-row gesture is never emitted (moves of XML children replace the parent), so no address goes stale inside one edit.
- Features that were reachable through the old whole-document diff and now have no kind (refused with `snapshot-edit.unsupported-path`, never dropped): docx/xlsx/pptx OPC relationships, content-type defaults and package comment; xlsx sheet insertion and non-name sheet attributes; docx part document prolog/epilog; ifc2x3 instance-id rename (it upserts a new instance instead); step cc file description/name.

## 🌊 Wave 5 (no feature loss, stricter gate, wave-5 rulings)

Status: WRITTEN, NOT COMPILED, NOT TESTED (foundation still RED; `rustfmt --check` syntax only on every file touched). Gate: `bun ./📜️script.ts verify mutation-outcome-law` (log `🗑️generated/stdio-mid/gate-run-9.log`) reports **0 findings under step, docx, gif, ifc, svg, pptx, xlsx** (run 7 had the coordinator's 48 = 34 R12 + 14 R8; run 6 on the previous script revision had 358, R9/R10/R11 included).

### Gate breaches closed
- R9 `apply_diff`/`.apply(` under `🧬️schema`/`🧬️mutations`: the 28 `apply_*_mutation` wrappers (and their `pub use` re-exports in cc1-6, cobie/sav/cv20, svg basic/tiny) are `#[cfg(test)]`; no production caller existed. The four diff-absorb sites went through `apply_*_to_added` helpers (an applier by name, as the gate defines). docx `apply_addressed_xml_mutation_in_place` / `replace_addressed_node` (the retained-execution seam) moved out of `🧬️schema` into `✏️editor/📬️preparation`; the docx unit tests replay through `protocol::apply_diff` (`apply_addressed` test helper).
- R10/R11 (named like the heuristics, behaviourally already base-reading): the OPC/XML-part `diff_*` helpers of the docx/pptx/xlsx diff types are now `between_*` (sync machinery), the inverse helpers `inverse_*` / `diff_inverse` / `inverse_item` are `rewind_*` (they read base rows and never apply or simulate); step `ladder::entity_diff`/`restore_diff` use the positional `rewritten_entity_diff` instead of `StepEntityDiff::between`.
- R12/R8: `with_diff_applied` simulators renamed `apply_*_diff_to_copy` (appliers); ifc4/step/gif `removed.to_vec()` + `sort` and `prefix.to_vec()` + `extend` became iterator constructions; ifc2x3 `mvd::view_definition_diff`/`argument_diff` build the new header/instance functionally; docx xml-address `let mut bindings = x.clone(); apply_bindings(..)` became `scoped_bindings`, path copies `[parent, &[i]].concat()`, and `run_with_text`/`run_with_formatting`/`paragraph_with_style` hand an OWNED copy to `edited_run_text`/`edited_run_formatting`/`edited_paragraph_style`; pptx `child_path` and the moved slide address are struct-built; the oracle reference undo of gif/svg/xlsx edits through one owned-value seam `applied(value, step)`.

### (1) svg `set-pixel-region`
No longer a load. `edit_rules::region` (base, basic, tiny) turns the described drawing into the concrete kinds of replacing the root element's content (`mutation_support::region_edit`): attributes set/removed (new ones at their position), root children removed last to first and the described ones inserted in order; declaration, doctype and root rename only where the subset has a kind. Each row is a concrete kind, so the gesture undoes row by row; a region the profile refuses (tiny/basic gates) is refused by the row's own leaf. Test: `a_drawing_region_replaces_the_root_content_row_by_row`.

### (2) Previously refused paths, now concrete kinds
| path | artifact | kind(s) |
|---|---|---|
| `/opc/relationships/<owner>/<i>[/field]` insert / remove / change | docx, pptx, xlsx | NEW `set-relationship{owner,id,relType,target,external,index?}`, NEW `remove-relationship{owner,id}`; a changed id is `remove-relationship` + `set-relationship` at the same position; the last relationship of an owner removes the owner (owner groups are sorted by path, so their position is determined) |
| `/opc/contentTypes/defaults\|overrides/<i>` insert / remove / change | docx, pptx, xlsx | NEW `set-content-type{isOverride,name,contentType,index?}`, NEW `remove-content-type{isOverride,name}` |
| `<sheet>` insert in the workbook XML | xlsx | existing `insert-sheet` (empty sheet of that name at that position, no slot) |
| `/document/instances/<i>/id` | ifc 2x3 | existing `upsert-instance` at the instance's index + `remove-instance` of the old id (position kept, undoes row by row; an id in use is refused) instead of a new `rename-instance-id` kind |
Per kind (12 new): leaf (`📎set-relationship`, `🧷remove-relationship`, `📇set-content-type`, `🧺remove-content-type` under each `🧬️mutations`), `🔣️.json` descriptor, `🧬️schema/🔣️.json` payload schema, sum-law test in the leaf's own `🧪️tests` (first/middle/last insert, in-place change, new owner, last-of-owner removal, override and default), shared builder module `🔗opc-layer` (`with_package`, `relationship_write_diff`, `relationship_removal_diff`, `content_type_write_diff`, `content_type_removal_diff`, `*_at`), enum variant + `KINDS` + `demo_cases` (the fixtures gained an external root relationship `rIdDemoExternal` and an unused default `zzdemo`), binary tags (docx 18-21, pptx 9-12, xlsx 12-15), docx/xlsx text + binary codecs and grammar, xlsx `.proto`, mutation union `🔣️.json` and `🟦️.ts` (pptx also its `parsePptxMutation`), oracle `KINDS` + manifest `kinds`, edit-rule dispatch (`relationship_edit`, `content_type_edit`) with tests.

### Not done (stated plainly)
- **AMB-1 positional rows.** The docx/pptx/xlsx `NamedTripleDiff` engine (`between_named`, `apply_named`, `rewind_named`, `absorb_named`, and the xml-part twin) and every leaf builder (`override_diff`, `set_part_diff`, `insert_xml_part_diff`, the xlsx sheet/relationship plans, the 12 new OPC kinds) still carry a whole `order: Vec<K>` key list built from the base list. Replacing it is a diff-type redesign (sparse `{key,index}` rows do not compose in `absorb` without base-side index transport; the sound form is the index-keyed `IndexedTripleDiff` those files already use for blocks/runs/rows, with named collections addressed by base index), touching the three engines, ~150 literal sites, the diff `🔣️.json`/`🟦️.ts` mirrors and the shared `XmlAttributesDiff.order` of stdio-xml. I did not start it blind; it needs the compile loop.
- **AMB-2 derived data.** xlsx `shared_strings_root_diff` writes the derived `uniqueCount`, and `insert-sheet` mints `sheetId`/relationship id/part path inside its diff; pptx slide ids and docx override entries are minted likewise. Moving them into `XlsxDiff::apply` re-derivation changes round-trip exactness for packages with a stale `uniqueCount`; left as is.
- **AMB-3** is met in behaviour (every `DiffAlgebra::inverse` here reads base rows; no simulator), the gate is clean after the renames above.
- **Oracles for the 12 new kinds.** Only `KINDS`/manifest `kinds` were extended; the third-party forward/inverse implementations, generator recipes and fixtures were not authored (they need the generator run with the reference libraries).
- **Compile status.** The new code was written without a compiler: expect visibility/import fixes (leaf test imports, `fixture`/`demo_fixture` privacy from the leaf tests, `edit_package` closure, `xml_parts` iteration on docx's `PagedList`), then run per artifact `"$T/🚦️gate.sh" stdio-mid -- cargo check --offline --target wasm32-wasip2` and `cargo test --offline --lib` (names: `edit_rules`, `mutation_inverse_sum_law`, `set_relationship_inverts_to_the_exact_previous_state`, `remove_relationship_restores_the_relationship_at_its_position`, `set_content_type_inverts_to_the_exact_previous_state`, `remove_content_type_restores_the_entry_at_its_position`).

## 🌊 Wave 6 (positional rows on `protocol::list_delta`, AMB-2, oracles for the 12 plumbing kinds)

Status: WRITTEN, NOT COMPILED, NOT TESTED (foundation RED; `rustfmt --check` syntax only, `tsc --strict` and `bun` on the TS/probe/generator files, the generator self-checks and the fixture re-derivation were RUN). Scripts: `🗑️generated/stdio-mid/w6*.py`; the third-party generator is `🧰️opc-plumbing-oracle/📜️script.ts`.

### (1) AMB-1: no `order` key list in docx / pptx / xlsx, on the framework's list delta
- The algebra exists once, in `protocol::list_delta` (`Parts`, `Keyed`, `RowPatch`, `ItemList`/`BuildList`). My first local copy (`stdio contract list_delta`: `ListDelta`, `commit_onto`, `absorb`, `inverse`, its randomized tests) is DELETED. What remains in `📇️registry/🧬️contract/🪡️list-delta` is only the wire macro `stdio_list_delta!` (+ `Composable`, `compose_optional`, `insertion_index`): the kernel's `list_delta!` shape (`removed [{id,index}]`, `inserted [{index,row}]`, `moved [{id,from,to}]`, `modified [{id,patch}]`, same methods, every one forwarding to `Parts`) minus the `DslRecord` derive, because the rows (an OPC part with its bytes, an XML part whose patch holds an `XmlDiff` tree) are no `DslField`. Swap it for `protocol::list_delta!` once `DslField` admits them.
- The OPC diff is no longer written three times: `🎒️zip/📦️opc/🔺️diff` defines `OpcDiff { comment, contentTypes{defaults,overrides}, parts, relationships }` ONCE (rows `OpcContentTypeRow`, `OpcPart`, `OpcRelationship`, `OpcOwnerRow`; deltas `OpcContentTypeEntriesDelta`, `OpcPartsDelta`, `OpcRelationshipsDelta`, `OpcOwnersDelta`; patches `OpcContentTypePatch`, `OpcPartPatch`, `OpcRelationshipPatch`, `OpcOwnerPatch` nesting the relationship delta) with `commit_into(&mut OpcPackage, ApplyCapability)`, `rewind(&base)` and `absorb`; relationship owners are a sorted set, so an owner row's index is its sorted rank and a delta that breaks the order is refused (`mutation.apply.invalid-order`). docx/pptx/xlsx diffs hold `Option<OpcDiff>` plus their own `<X>XmlPartsDelta` (key = part path; docx lists through the local `DocxXmlPartRows`/`DocxXmlPartsView`, so the inverse never clones a retained document it does not reinsert). Deleted: `NamedTripleDiff`, `NamedModified`, `GenericNamedEngine`, `reorder_*`, every `*Opc*Diff` type and `apply_/rewind_/absorb_*` helper, the dead document-diff types (`IndexedTripleDiff`, block/run/row/cell/style diffs).
- Builders became positional: every insertion states its AFTER index (`insertion_index(len, index)`; XML parts without an index land at their sorted rank instead of the old "sort everything" reorder), every removal its BASE index (`removal_by_id`), moves are `{id,from,to}`. Converted: the three `🔗opc-layer` files, docx/pptx/xlsx aggregators (`override_diff`, `set_part_diff`, `insert_xml_part_diff`, `remove_part_diff`, `remove_xml_part_diff`, `retype_xml_part_diff`, `root_edits_diff`), pptx `xml-address`, xlsx `canonical-edit` (`insert_sheet_plan`, `remove_sheet_plan`), the six strict/transitional aggregators; the diff mirrors (`🔣️.json`, `🟦️.ts` incl. a new pptx pair, `🛰️.proto`, `🔗️.graphql`; pptx's were still the pre-NamedTriple "field replacement" shape and now match the real diff).
- Tests: docx demo cases are declared positional rows (`snapshot_a`/`snapshot_b`/`demo_forward_diff`: parts replaced in place, relationship and default appended, two XML parts swapped, main-document declaration set); result-apply tests for a reorder, forward+inverse+absorb identity, a removal at the wrong base index; `🎒️zip/📦️opc/🔺️diff/🧪️tests` (positions exact, rewind restores, diff absorbed with its rewind is the identity, owner order refused, wrong base index refused). The randomized sequence laws live in the framework module.
- Fixture conversion: `w6_convert_fixtures.py` converts old `NamedTripleDiff` JSON (`removed`/`modified[{key,diff}]`/`added`/`order`) to positional rows and verifies EVERY conversion by replay (old semantics vs new semantics on the same base; `selftest` runs 3000 random conversions, all replay identically). The only committed diff fixture in the three artifacts is `docx/…/🔺️diff/🧫️fixtures/🧹️clear-main-declaration` (`modified[{id,patch}]` now); the pptx `placeholder-kind` and the other fixtures are snapshots, unaffected.
- Not mine, still `order`: `XmlAttributesDiff.order` (the stdio-xml crate, stdio-small); the docx/pptx/xlsx root-attribute builders still pass it.

### (2) AMB-2 derived data
- `uniqueCount`: no leaf writes it. `XlsxDiff::apply` re-derives it in the central applier (`refresh_unique_counts`): for every shared-strings part a diff modifies, if its advertised count equalled its `si` entries BEFORE, it equals them AFTER; a table that already disagreed with itself is left alone (so unrelated edits never normalize a stale package and the sum law holds for it too).
- `insert-sheet`: the gesture mints. `canonical_edit::mint_sheet_slot` (next free `sheetId`, relationship id, part path, content type, document from the typed cells) is called once by `InsertSheet::minted(base, sheet, index)`, which the edit-rules special handler uses; `insert_sheet_plan` only writes the payload's `slot` and refuses a payload without one. The unit/demo cases build their inserts with `minted`.
- docx/pptx: no minted ids exist in their leaf diffs (explicit paths/indices only); nothing to change.

### (3) Third-party oracles and fixtures for the 12 new kinds (docx, pptx, xlsx × set/remove relationship, set/remove content type)
- Independent implementations: `zip` + `quick-xml` handlers in each oracle (`apply`, computed inverse specs from the base package, a `plumbing` projection of the ordered defaults/overrides and every owner's ordered relationships) with unit tests on the real packages (observable, positions exact, inverse restores, missing target refused); docx adds `plumbing` to its projection, pptx/xlsx expose `project_pptx_plumbing`/`project_xlsx_plumbing` (their slide/grid projections and round-trip laws stay untouched). The oracles follow the production rule that a default extension in another letter case is refused.
- Third-party fixtures: `🧰️opc-plumbing-oracle/📜️script.ts` (bun, `jszip` 3.10.1 + `fast-xml-parser` 5.11.1) builds 13 recipes per artifact (applied/no-op/rejected for the four kinds) from each artifact's committed real package; every pair states its effect twice (XML edit and model-level expectation) and the generator refuses to write one where they differ. 39 recipes, 69 files written under each artifact's `🧫️fixtures/📎…/🧷…/📇…/🧺…` dirs, registered as `testEvidence`; `verify` re-derives all of them byte-identically (RUN: passes).
- Manifests: 4 `mutationManifests` entries per artifact (`oracleRequirements` → third-party-library), new third-party oracles `jszip-{pptx,xlsx}-ecma-376-opc-plumbing-reader`; docx gets the judge that can see the plumbing: probe `docx-compare-opc` (ordered `[Content_Types].xml` + every owner's ordered rels; RUN on a pair: detects the inserted relationship, the old `docx-compare` stays blind to it), pipeline `docx-ecma-376-jszip-opc-compare-v1`, profile `semantic-docx-ecma-376-opc-jszip-v1`. Reason for a separate probe: python-docx's afters of the old kinds rewrite the plumbing in their own order (`remove-part` even drops the root relationship), so a global plumbing comparison would turn existing rows red.
- Oracle unit tests: the docx/pptx "one Examples row per kind" counts exclude the four plumbing kinds (proven by the explicit plumbing tests); docx `KINDS` count 16, pptx 12.

### Verification
- Gate (`verify mutation-outcome-law`): see the final line of this section.
- Residuals: (a) no README `.feature` Examples rows for the 12 kinds: the harness judges for pptx/xlsx are python-pptx/openpyxl readers that do not project the plumbing; (b) pptx/xlsx comparison profiles still compare slides/grid only — their plumbing evidence is judged by the ordered `plumbing` projections of the Rust oracles; (c) `stdio_list_delta!` instead of `protocol::list_delta!` (see (1)); (d) everything above is uncompiled: first compile errors to expect are macro hygiene in `stdio_list_delta!` (`$item`/`$key`), `ApplyCapability` import paths in the zip diff module, the `DocxXmlPartRows` `FromIterator` capacity assumption, and the zip diff test harness imports.
