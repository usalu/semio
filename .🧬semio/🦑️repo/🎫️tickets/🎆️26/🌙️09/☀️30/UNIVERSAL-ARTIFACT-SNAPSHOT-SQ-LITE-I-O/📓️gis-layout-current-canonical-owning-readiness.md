# GIS Terrain16 and Layout9 Current Canonical Readiness

Read-only current inspection, 2026-10-04. No Source, compiler, Cargo or Native execution. All existing owning assertions remain unchanged and pending. Paths below are relative to `/Users/ueli/Documents/semio`. Prior semantic reports and capsules remain authoritative; this report concerns named source prerequisites only.

## Dependencies

GIS `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain`, Layout `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout`, and CAD `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad` each currently lack direct Record and Record-derive bindings in their `📦️packages/🦀️rust/Cargo.toml`, despite production named canonical derives. For each manifest, add the actual named owners `semio-framework-dsl-record` and `semio-framework-dsl-record-derive` using its existing prefix `../../../../../../../🧰️framework/🔨️modules/`, followed respectively by `🗣️dsl/🧬️schema/📦️packages/🦀️rust` and `🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust`. GIS/Layout already declare Value and Diagnostic. Jack declares both Record dependencies at its manifest:23–24.

## GIS Exact Narrow Roster

Under the GIS owner, append `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/` to these five paths:

| File | Current reference locations | Named authority |
| --- | --- | --- |
| 🗺️imported-map/🦀️.rs | grouped use:2 DslValue,Number,ToValue,FromValue; derives:4,10 DslRecord; controlled signatures:40,41 NativeDecodeControl | Value for grouped use/control; Record-derive for derives |
| 📸️snapshot/🪶️sqlite/🦀️.rs | grouped use:4 DslValue,Number | Value |
| 📸️snapshot/📦️pack/🦀️.rs | derives:3,5; RecordSpecProducer:9; NativeEncodeControl/RecordValue:10; print/JoinMode:18; parse_exact/ParseOptions/SourceMode and three field_error calls:19; RecordSpec:23; RecordValue/NativeDecodeControl/FieldValue:28 | Record-derive; Value for encode/decode controls; Record for grammar/spec/fields; Diagnostic for removed field_error |
| 📸️snapshot/📦️pack/🛫️encode/🦀️.rs | DslField:3; RecordSpecProducer:5; NativeEncodeControl/RecordValue:6; native_encoding:8,13; FieldValue:17,18 | Value for NativeEncodeControl; Record for remainder |
| 📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs | DslValue:4,19,23,33,37,41,45; ToValue:22; Number:33,37,41,45 | Value; preserve every assertion and literal representation |

Three removed `dsl::__rt::field_error` calls are all in Pack parse_dsl:19. Pair authored canonical Diagnostic TextError/InvalidValue at the text boundary, preserving messages and meaningful positions. Redirecting to Record::__rt::field_error would retain a removed API.

Pack Terrain:6 has obsolete `#[dsl(extension="gisterrain")]`. Canonical Record derive ignores this extension vocabulary. Authored ArtifactDsl explicitly owns EXTENSION/envelope identity:16–17. Remove stale metadata without deriving a second artifact authority on the internal Terrain projection.

## Layout Exact Narrow Roster

After excluding comments, public product Mutations/MutationLeaf/binary helpers, and actual local document_dsl test aliases, no private qualified Value/Record references were found. One removed helper remains: `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs:25`, `store::__rt::field_error` for invalid mutation JSON. Pair canonical Diagnostic TextError/InvalidValue, retaining the current diagnostic. Canonical Record derives throughout Layout require the two manifest bindings above. DslOps matches are comments, not executable derives.

## Quick Jack/CAD Qualification

Jack has direct Record/derive bindings and no current named private Value/Record references found in this narrow scan. This does not establish compilation or the fourteen rich owning laws.

CAD lacks the two direct dependencies. Its `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🦀️.rs:3` still imports `dsl::{DslField,FieldValue,NativeSchemaControl,Shape}`; producer:83 and owned field guard:116 still use private product namespaces. Pair Record names with the canonical Record crate, checking the actual declared owned-field guard rather than adding an alias. Its Value imports:4–5 are already canonical. Other prior CAD measured prerequisites and unmounted semantic capsules are outside this quick scan.

## Runtime Boundary

No test body, budget, full IEEE/literal comparator or child identity assertion should change for these namespace/dependency repairs. GIS16/Layout9 remain pending authentic unchanged owning Native baselines through the registered Bun/Nx packages. This report is bounded source readiness evidence, not a fresh compiler diagnosis or runtime success claim.

## Subsequent GIS Readback and Source Receipt

The GIS prerequisite roster above describes the earlier inspection. Root subsequently mounted both direct Record bindings and canonical Value/Record/Diagnostic references in the five named GIS files. Current GIS readback finds no executable narrow private Value/Record names; the old snapshot module comment is not executable. The ignored internal Terrain extension annotation remains authored obsolete metadata, not a proven compiler failure; actual ArtifactDsl owns envelope identity manually. All mutation and full-row producer defects remain unchanged for the owning baseline.

Registered GIS Source7 now executed successfully: nine tests, zero failures, 179 expectations, Bun3.27s, Nx7.9s. This supersedes this report's earlier no-Source-execution qualifier for GIS only. It is TypeScript Source evidence, with no Rust compiler/runtime/System allocation inference. Root's sixteen-law unchanged GIS Native baseline is active and its result is pending. Current detailed qualifiers, coverage and unmounted concrete Rust repair inputs are retained in `📓️gis-terrain-canonical-semantic-readiness-refresh.md` and `📓️gis-terrain-source-coverage-and-native-selection.md`.

## Subsequent Authentic GIS Red and Semantic Activation

Root's unchanged sixteen-law run `42e94b0e-51ff-4d97-8b27-37e291f1d806` executed sixteen with eight failures; full reconstruction admitted71 versus641728 System requested bytes. The held GIS repair inputs are now mounted coherently, and the ignored Terrain annotation is removed. Exact changes and syntax/Source receipts are retained in `📓️gis-terrain-authentic-red-coherent-repair.md`. Source10 passed nine tests/179 expects, Bun5.54s/Nx39.6s, after retained unchanged Source9 timeout. Root's owning replay must authenticate the repair; no repaired Rust runtime result is claimed here. Other domain findings in this report remain separate historical bounded inspections.
