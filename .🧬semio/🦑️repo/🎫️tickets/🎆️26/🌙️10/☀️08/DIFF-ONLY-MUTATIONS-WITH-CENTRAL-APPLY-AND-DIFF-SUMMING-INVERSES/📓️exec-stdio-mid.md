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
