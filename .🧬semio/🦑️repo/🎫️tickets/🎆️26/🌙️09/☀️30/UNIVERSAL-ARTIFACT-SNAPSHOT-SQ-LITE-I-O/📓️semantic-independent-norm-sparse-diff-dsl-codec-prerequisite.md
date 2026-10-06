# Norm Sparse Diff DSL Codec Prerequisite

The actual reached diff_text and diff_binary macros require __dsl_spec, __dsl_to_record and __dsl_from_record, not the existing ToValue/FromValue intrinsic tree. [Macro authority](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:846) prints/parses Inline DSL and encodes/decodes Pack Record using that spec. The moved En1990 IO modules invoke those macros on [En1990Diff](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:9), whose derives lack DslRecord. This is an authentic compile prerequisite, not a Snapshot semantic budget failure.

En1990 has20 optional named fields, with primitives, AnnexChoice and seven Option<Vec<already DslRecord entity>> relations. Adding the genuine DslRecord derive is the natural direct owner prerequisite if AnnexChoice actual DslField is present. Option<Vec<T>> classifies as OptionScalar<Vec<T>>; existing generic Vec DslField produces List. Required default sparse behavior stays None; present-empty is Some(empty Vec), distinct from omission. The text parser fills unmentioned fields as Absent (schema decoding line96), and Option binder converts Absent toNone. Value camelCase default attributes do not rename DSL keys; default DSL kebab names are an independently authored physical contract and must be checked against actual grammar/fixtures.

A blanket derive for all twelve Diff roots is not yet justified: other actual Diff roots contain optional named list-wrapper structs lacking DslRecord, Option<Box<Artifact>>, VDI maps/custom IDs and intrinsic owners. Each reached nested type must have genuine DslField metadata/binding authority before its Diff can derive. A top-level derive alone cannot supply those dependencies. Preserve every original default, named sparse field and no-op/Some(empty) behavior; do not use ToValue as a schema adapter or edit shared macros merely to make these owner calls compile.

No Cargo gate was run. The current En1990 compiler-only receipt remains compiler-only.

AnnexChoice prerequisite independently verified at /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs:514: it derives DslScalar with explicit en/de DSL keys. Its display labels EN/DE-NA remain separate. EN1990 nested PermanentAction and peers already derive DslRecord in actual artifact root.

## Optional Box Artifact Eligibility

Actual12 root Artifact declarations were read directly and retained in inputs/norm-twelve-artifact-diff-dsl-eligibility-readonly.json. Every root derive lacks DslRecord; the existing generic Box<T> DslField requires T:DslField, so Option<Box<Artifact>> cannot acquire a Record shape from ArtifactSchema/ToValue/FromValue alone. A genuine direct Artifact DslRecord derive is a distinct nested owner addition and requires verifying each declared field dependency; Snapshot conversion would clone and is not a borrowed projection substitute. The root declaration row counts/types are exact retained source rather than projected Snapshot guesses. No broad trait adapter or shared macro change suggested.

## Direct Artifact Field Eligibility Compared With Actual Snapshot

Fresh exact12 root-versus-Snapshot field readback is retained in inputs/norm-twelve-artifact-snapshot-field-parity-readonly.json. All twelve actual Snapshot roots derive DslRecord and every Artifact field name/type matches its Snapshot counterpart. The only textual mismatch is EN1996 design_situation: Artifact uses crate::document::DesignSituation while Snapshot imports that exact type at line3. VDI maps/EditionId/catalog/index/security and DIN18599 climate child match exactly. Thus direct Artifact DslRecord derives reuse actual existing field trait eligibility, with no Snapshot shadow; the previous broad nested-field warning is now narrowed for these exact root Artifact fields.

Preserve owner-authored physical attrs deliberately: Snapshot table/block/lines refinements are separate descriptor metadata, not a prerequisite for trait eligibility. Diff wrapper structs and extra Diff-only fields (e.g. VDI manufacturer_file) still need independent direct eligibility. This is source-level proof, not fresh compiled12-family proof.

## Held Twenty Direct Producer Pairs

All20 held BEFORE guards match current source. Every changed line is a derive attribute only:11Diff owners and9 actual reached optionalBox Artifact roots, plus existing named Vec wrappers in those Diff files (EN1993 local wrapper macro included). Fields, defaults, sparse conversion/apply semantics and physical attrs are untouched. The9 root additions match actual optional Box dependencies; roots without that dependency are not gratuitously changed. VDI ManufacturerFile separately already derives DslRecord at actual root214. No definite static field-trait blocker remains from the bounded prior field parity and wrapper audit. This does not qualify actual20-path compilation or Diff text/binary runtime semantics; preserve genuine Before and require owning replay.

[Held20 authority](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/physical-current-norm-eleven-sparse-diff-direct-producer-held-pairs.json).
