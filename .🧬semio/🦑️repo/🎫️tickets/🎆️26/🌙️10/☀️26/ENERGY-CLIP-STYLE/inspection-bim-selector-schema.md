# BIM Loss Selector Schema Repair

The current room-loss rule legitimately selects IfcSpace, but the loss selector reused the physical-element class enum and rejected the authored model.

## Schema-First RED And GREEN

Eight portable focused cases were saved before the schema edit. Both the repository compileJsonSchemaSubset interface and independent Ajv rejected the valid current IfcSpace loss selector. Physical selectors/types and all negative cases agreed.

The schema now defines loss_selector_class as the union of the existing physical class reference and the existing space.properties.class reference. Only translation.losses.applies_to.class points to this dedicated contract. The physical element enum, physical type enum, 3D point schema, authored data and all stories remain unchanged.

After the edit, all eight focused cases agree with their expected verdict in both validators. The complete current authored model passes Ajv draft2020-12 validation. Unknown/unrelated selectors remain invalid; IfcSpace is still invalid as a physical element or type; IfcSpaceType remains invalid as a physical type.

Schema SHA256: f292973a7c3fc1e15f24a3ca6a55b4b02a1c34a5734df122905b6415e0ca900d.
Unchanged model SHA256: 5289d6bd9591f5843876148650b2b2fd9072e0d91fc87463f45bdd7d55a98164.

## Explicit Validator Scope

The repository TypeScript interface declares an owned Draft07 subset. It does not implement the current point3 draft2020 prefixItems/items:false tuple. Applying it to the full model produces unsupported-geometry errors even when Ajv accepts that geometry. This pre-existing interface limitation is documented; no full-model repository-subset GREEN is claimed.

The feature cases validate the focused selector/physical/type fragments through both implementations, and the full authored model through Ajv. No geometry declaration was weakened or translated to legacy tuple syntax. No shared framework validator was edited or expanded.

## Paths And Registration

- BIM model/🧬️schema/🔣️.schema.json — dedicated selector reference/definition only.
- BIM model/🧪️tests/🧬️loss-selector/🔣️.json — portable eight-case contract and unchanged physical vocabularies.
- BIM model/🧪️tests/🧬️loss-selector/🟦️.ts — repository/Ajv parity and full-model Ajv validation.
- Authoring/📜️script.ts — normal test registration, preserving concurrent entries.

This BIM-specific schema change is outside the current Python wrapper owner. Ordinary current-source guards own any required six-leaf refresh; no duplicate native producer or global wrapper restart was introduced.

## Existing Schema Regression

The unchanged BIM CJS suite also passes: current authored model accepted,27invalid models rejected,8GlobalId vectors plus138stored GlobalIds independently reproduced, and4invalid manifest structures rejected. The post-fix full six-leaf read-only preflight is stable and passes. No native producer was started for these schema checks.

## Final Current Data Validation

After genuine17/117/38/38 completion, the permanent selector driver passed all8 focused repository/Ajv cases against actual current model b934261683a39bb78743049c744da106ff4f7d9c2b732b6c2d726c80731d3157 and schema f292973a7c3fc1e15f24a3ca6a55b4b02a1c34a5734df122905b6415e0ca900d. Full current-model Ajv validation also passed with no errors (Ajv8.20.0), independently repeated by inspection-bim-schema-current.mjs. The repository-validator claim remains focused selectors/elements/types; no full owned Draft2020 model claim is made. Driver rechecked schema/data hashes after validation and wrote no production files. Checked 1793146923.625602.
