# Insert Payload Binding Audit

Source scan only; no build or test. Census matched227 literal InsertItemRule declarations to existing native leaf JSON payload schemas across all15 families.

| Family | Insert rules |
| --- | ---: |
| EN1990 |14|
| EN1991 |35|
| EN1992 |15|
| EN1993 |43|
| EN1994 |9|
| EN1995 |17|
| EN1996 |15|
| EN1997 |20|
| EN1998 |23|
| EN1999 |20|
| DIN4108 |5|
| DIN16798 |2|
| DIN18599 |0|
| ISO16757 |8|
| VDI3805 |1|

Comparison checked emitted selector/index/item keys against schema properties and required payload members. Root singleton/const discriminators are excluded from missing-member findings because native from_payload_value supplies its tagged aggregate identity. All227 schemas were found. No index-key mismatch was found.

## Concrete Mismatches

Base: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`.

`🧭️edit-rules/🦀️.rs` binds:
- tieBeams / insert-tie-beam / item tie_beam. Native struct in `➕️insert-tie-beam/🦀️.rs` has Rust field tie_beam with value(rename_all="camelCase"); leaf schema `➕️insert-tie-beam/🧬️schema/🔣️.json` requires tieBeam, not tie_beam.
- slopes / insert-seismic-slope / item seismic_slope. Native struct in `➕️insert-seismic-slope/🦀️.rs` has Rust field seismic_slope with camelCase value metadata; leaf schema requires seismicSlope.

The generic payload therefore omits the required native item member in both cases. These are concrete mismatches independent of insertion defaults/form UX. All225 other declarations matched emitted payload member names in this scan.

## ISO/VDI Position Support Is Present

ISO `📇️iso16757/.../🧬️mutations/📦️introduce-product/🦠️mutation/🦀️.rs` currently declares product:Product and index:Option<usize>. Its diff uses Iso16757ProductsRows::insertion(payload.index.unwrap_or(len),payload.product.clone()); an explicit index >len refuses.

VDI `🏭️vdi3805/.../🧬️mutations/📦️add-product/🦀️.rs` currently declares product:CatalogueProduct and index:Option<usize>. Its diff similarly inserts at index.unwrap_or(len) and refuses explicit index >len. Both leaf schemas expose index, so generic binding index:"index" is schema-consistent. An absent position appends. This audit found no reason to change these bindings to an absent-index representation.

Every current literal InsertItemRule binds an index key. If a future native append-only mutation is introduced, represent that binding explicitly as optional typed metadata; an empty string would create an invalid payload property.

## Unknown Members

Leaf JSON schemas inspected here declare additionalProperties:false. Native value derives only emit unknown-field rejection when container.deny_unknown_fields is set; camelCase alone does not establish this native policy. Generator owner is `🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🎯️expand-entries/🦀️.rs`, with deny_unknown_keys conditional on that flag. Schema validation is therefore a distinct admission layer from native decode.

The two EN1998 findings remain defects even if an unknown snake_case key is tolerated: the required camelCase struct member is missing. No unknown-field compatibility handling should be added to conceal them.

This bounded audit verifies payload key shape, not every mutation's domain invariant, positional inverse law or runtime success. It does not establish that an argument-free Add button supplies a valid item; that separate failure is documented in e2e-inspector-list-actions-audit.md.

## Retained Regression Authored

Added norm/🧪️tests/📥️insert-bindings with closed JSON fixture/schema, feature and focused Bun test. It recursively discovers actual literal rules and uses an independent strict Ajv projected key-membership oracle, with negative unknown-key controls. The227 expected canonical bindings retain tieBeam/seismicSlope. No test was launched; production strings remain unchanged pending parent-observed red. All new paths are below256 characters.
