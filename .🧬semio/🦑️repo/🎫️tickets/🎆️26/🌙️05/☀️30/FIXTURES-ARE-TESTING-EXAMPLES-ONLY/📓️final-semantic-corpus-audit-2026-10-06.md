# Final Semantic Corpus Audit — 2026-10-06

Read-only structural audit. A first traversal parsed 5,561 schema JSON files. Candidates below were selected by actual top-level schema properties and opened structures, not title matching. Files may change concurrently. No runtime tests executed by this audit.

## Verified Findings

- The repo events production Rust crate exports ungated `fixture_dir()` and `fixture<T: serde::de::DeserializeOwned>()` at lines 1459–1470 in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/📦️packages/🦀️rust/🦀️.rs`. These load test fixtures from production; move helpers to test code or gate explicitly.
- Titleless wrapper schemas remain in the following table. Fixed case counts, named expected outcomes, source census contracts, before/after pairs, and refusal vectors establish corpus semantics. Replace whole-corpus admission with assertions over plain examples and validate genuine per-value domains individually.
- PDF cos-output `$defs.Cos` and retirement `$defs.color/function/action/outline/form/date` contain genuine recursive wire domains that must survive separately. Compiler generation outputs already references genuine `generated-token-output`; preserve that reference domain. Net-leaves schemas contain genuine mutation leaves within corpus wrappers; preserve those domains individually. WFC bitmap root `edit` is a fixed example (`ordinal=1`, `paletteIndex=255`), not a general edit domain.

## Structural Candidate Paths

### `🧰️framework/🔨️modules/🎒️pack/🌱️value/🧬️schema/🔑️schema-hash/💰️storage/🔣️.json`

cases: {"const": ["flat", "nested", "nested-inner-field-changed", "recursive", "every-shape", "empty", "mutualRecursive", "broadRepeatedEdges"]}

No root definitions.

### `🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🧬️schema/🔣️.json`

cases: {"type": "array", "minItems": 8, "maxItems": 8, "items": {"type": "object", "additionalProperties": false, "required": ["name", "coordinate", "sources", "expected"], "properties": {"name": {"type": "string", "minLength": 1}, "coordinate": {"type": "object", "additionalProperties": false, "required": ["artifact", "standard", "subset", "surface", "owner", "excludedOwnerSegments"], "properties": {"artifact": {"type": "s

No root definitions.

### `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/📤️generation/🧬️schema/🎯️outputs/🔣️.json`

cases: {"type": "array", "minItems": 17, "maxItems": 17, "items": {"type": "object", "additionalProperties": false, "required": ["id", "outputs", "refusal"], "properties": {"id": {"type": "string", "minLength": 1}, "outputs": {"type": "array", "items": {"$ref": "#/$defs/output"}}, "refusal": {"type": ["string", "null"], "enum": [null, "unresolved-generator-origin"]}}}}

Definitions to inspect and preserve: `output`.

### `🧰️framework/🔨️modules/🌱️value/📋️list/🧬️schema/🌳️actual-height.json`

cases: {"const": ["empty-root-no-allocation", "first-leaf-and-payload-only", "growth-retains-first-pointer", "growth-grant-refusal-preserves-root", "exact-parent-handoff-zero-child-disposal"]}

No root definitions.

### `🧰️framework/🔨️modules/🧬️schema/🧬️schema/🧱️neutrality/🔣️.json`

laws: {"const": ["artifact_composition_fields_derive_emits_expected_slot_tables", "artifact_composition_fields_default_to_empty_for_leaf_artifacts", "artifact_composition_projection_walks_aliases_nested_options_and_cancels", "artifact_composition_projection_real_child_alias_has_fixed_admission_bounds", "registry_descriptors_carry_valid_snapshot_state_and_match_field_states", "graphql_state_preamble_matches_normative_sdl", 

No root definitions.

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧬️schema/🧵️production.json`

cases: {"type": "array", "minItems": 5, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["id", "exact", "live", "capacity", "accepted"], "properties": {"id": {"type": "string"}, "exact": {"type": "boolean"}, "live": {"type": "boolean"}, "capacity": {"type": "boolean"}, "accepted": {"type": "boolean"}}}}

No root definitions.

### `✏️s/🔌️plugins/📐️cad/⚙️engine/🏗️construction/🧬️schema/🧱️linear-prism/🔣️.json`

cases: {"type": "array", "minItems": 1, "items": {"type": "object", "required": ["id", "pointA", "pointB", "thickness", "height", "volume"], "additionalProperties": false, "properties": {"pointA": {"type": "array", "minItems": 3, "maxItems": 3, "items": {"type": "number"}}, "pointB": {"type": "array", "minItems": 3, "maxItems": 3, "items": {"type": "number"}}, "thickness": {"type": "number"}, "height": {"type": "number"}, "

No root definitions.

### `✏️s/🔌️plugins/🧱️block/🗂️catalog/🧬️schema/🧭️roles/🔣️.json`

cases: {"type": "array", "minItems": 3, "maxItems": 3, "items": {"type": "object", "additionalProperties": false, "required": ["owner", "role", "accepted"], "properties": {"owner": {"type": "string", "minLength": 1}, "role": {"enum": ["plugin", "library"]}, "accepted": {"type": "boolean"}}}}

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/🧬️schema/📦️assembly/🔣️.json`

cases: {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": false, "required": ["id", "selected", "remove", "accepted", "remaining"], "properties": {"selected": {"type": "array", "items": {"type": "string"}, "uniqueItems": true}, "remove": {"type": "array", "items": {"type": "string"}, "uniqueItems": true}, "remaining": {"type": "array", "items": {"type": "string"}, "uniqueItems": true}, "id"

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧬️schema/🪜️definition-hierarchy/🔣️.json`

cases: {"type": "array", "minItems": 9, "maxItems": 9, "items": {"type": "object", "additionalProperties": false, "required": ["identity", "category", "accepted"], "properties": {"identity": {"type": "string"}, "category": {"type": "string"}, "accepted": {"type": "boolean"}}}}

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧩️composition/🔗️conversions/🧬️schema/🔣️.json`

cases: {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": false, "required": ["id", "packages", "exports", "accepted", "dependencies"], "properties": {"id": {"type": "string", "minLength": 1}, "packages": {"type": "array", "items": {"type": "string"}}, "exports": {"type": "array", "items": {"type": "object", "additionalProperties": false, "required": ["package", "identity"], "properties": {

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🔣️net-leaves/🔣️.json`

cases: {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": false, "required": ["id", "before", "after", "leaves"], "properties": {"id": {"type": "string", "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$"}, "before": {"type": "string"}, "after": {"type": "string"}, "leaves": {"type": "array", "items": {"type": "object", "additionalProperties": false, "required": ["kind", "at"], "properties": {"kind": {"

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔣️net-leaves/🔣️.json`

cases: {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": false, "required": ["id", "before", "after", "leaves"], "properties": {"id": {"type": "string", "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$"}, "before": {"type": "string"}, "after": {"type": "string"}, "leaves": {"type": "array", "items": {"type": "object", "additionalProperties": false, "required": ["kind", "path", "index"], "properties": 

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧬️schema/🪶️sqlite/🚦️cohort/🔣️.json`

laws: {"const": ["semantic-file", "exact-domain-row-bound", "native-input-owner", "native-input-controls", "native-output-owner", "native-output-controls"]}

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔣️net-leaves/🔣️.json`

cases: {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": false, "required": ["id", "before", "after", "leaves"], "properties": {"id": {"type": "string", "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$"}, "before": {"type": "string"}, "after": {"type": "string"}, "leaves": {"oneOf": [{"type": "null"}, {"type": "array", "items": {"oneOf": [{"type": "object", "additionalProperties": false, "required": [

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔣️net-leaves/🔣️.json`

cases: {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": false, "required": ["id", "before", "after", "leaf"], "properties": {"id": {"type": "string", "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$"}, "before": {"type": "string", "pattern": "^([0-9a-f]{2})*$"}, "after": {"type": "string", "pattern": "^([0-9a-f]{2})*$"}, "leaf": {"oneOf": [{"type": "null"}, {"type": "object", "additionalProperties": 

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️schema/🎛️metadata/🔣️.json`

cases: {"type": "array", "minItems": 11, "maxItems": 11, "items": {"type": "object", "additionalProperties": false, "required": ["factory", "owner", "keyword", "layout", "fields"], "properties": {"factory": {"enum": ["dwg_xrecord_value_spec", "dwg_table_control_entry_spec", "table_control_body_spec", "dwg_complex_color_value_spec", "table_record_body_spec", "dwg_evaluation_variant_spec", "dwg_evaluation_expression_value_spe

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🧬️schema/🏗️grammar/🔣️.json`

cases: {"type": "array", "minItems": 15, "maxItems": 15, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text", "accepted"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}, "accepted": {"type": "boolean"}}}}

No root definitions.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🌱️value/🧬️octets/🧬️schema/🛫️cos-output.json`

cases: {"type": "array", "minItems": 10, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["wire"], "properties": {"wire": {"$ref": "#/$defs/Cos"}}}}

Definitions to inspect and preserve: `Cos,Entry`.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧬️schema/♻️retirement/🔣️.json`

cases: {"type": "array", "minItems": 10, "maxItems": 10, "items": {"oneOf": [{"type": "object", "additionalProperties": false, "required": ["id", "owner", "wire"], "properties": {"id": {"type": "string", "minLength": 1}, "owner": {"const": "color"}, "wire": {"$ref": "#/$defs/color"}}}, {"type": "object", "additionalProperties": false, "required": ["id", "owner", "wire"], "properties": {"id": {"type": "string", "minLength": 

Definitions to inspect and preserve: `color,function,action,outline,form,date`.

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧬️schema/🧬️semantic-wire/🔣️.json`

cases: {"type": "array", "minItems": 15, "maxItems": 15, "items": {"type": "object", "additionalProperties": false, "required": ["keyword", "tag", "source", "mutation", "expected"], "properties": {"keyword": {"type": "string", "minLength": 1}, "tag": {"type": "integer", "minimum": 14, "maximum": 19}, "source": {"type": "string", "minLength": 1}, "mutation": {"type": "object"}, "expected": {"type": "object", "minProperties":

No root definitions.

### `✏️s/🔌️plugins/🀄️wfc/🪶️sqlite/🎨️bitmap/🧬️schema/🔣️.json`

cases: {"type": "array", "minItems": 14, "maxItems": 14, "items": {"type": "object", "additionalProperties": false, "required": ["id", "mode", "width", "height", "text", "indices", "gridPrefix"], "properties": {"id": {"type": "string", "minLength": 1}, "mode": {"enum": ["indices", "literal"]}, "width": {"type": "integer", "minimum": 0, "maximum": 4294967295}, "height": {"type": "integer", "minimum": 0, "maximum": 4294967295

No root definitions.

### `✏️s/🔌️plugins/📏️layout/🧬️schema/🛍️canvas-catalogue/🔣️.json`

cases: {"type": "array", "minItems": 15, "items": {"type": "object", "additionalProperties": false, "required": ["id", "action", "args", "kind"], "properties": {"id": {"type": "string", "minLength": 1}, "action": {"enum": ["canvasDragOver", "canvasDrop"]}, "kind": {"enum": ["page", "rect", "text", "image", null]}, "args": {"type": "object", "additionalProperties": false, "required": ["surfaceId", "x", "y", "width", "height"

No root definitions.

### `✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🎲️board/🎬️scene/🧬️schema/🔣️.json`

cases: {"type": "array", "minItems": 20, "maxItems": 20, "items": {"type": "object", "additionalProperties": false, "required": ["id", "mode", "input", "accepted", "expected"], "properties": {"id": {"type": "string"}, "mode": {"enum": ["normal", "ported"]}, "input": {"type": "object"}, "accepted": {"type": "boolean"}, "expected": {"type": ["object", "null"]}}}}

No root definitions.

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧬️schema/🔣️board-tools/🔣️.json`

cases: {"type": "array", "minItems": 1, "items": {"$ref": "#/definitions/Case"}}

Definitions to inspect and preserve: `Row,Flush,Rectangle,Case`.

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📷️framing/🧬️schema/🔣️.json`

cases: {"type": "array", "items": {"type": "object", "required": ["name", "nodes", "expected"], "properties": {"name": {"type": "string"}, "artboard": {"type": "object", "required": ["width", "height"], "properties": {"width": {"type": "number"}, "height": {"type": "number"}}}, "nodes": {"type": "array", "items": {"type": "object", "required": ["id", "transform", "segments", "opacity", "blendMode", "visible"], "properties":

No root definitions.

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧮️status/🧬️schema/🔣️.json`

cases: {"type": "array", "minItems": 3, "items": {"type": "object", "required": ["id", "layers", "selections", "expected"], "additionalProperties": false, "properties": {"id": {"type": "string", "minLength": 1}, "layers": {"type": "integer", "minimum": 0, "maximum": 1024}, "selections": {"type": "array", "minItems": 1, "items": {"type": "array", "maxItems": 256, "uniqueItems": true, "items": {"type": "integer", "minimum": 0

No root definitions.

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪛️utilities/🎬️actions/🧬️schema/🛑️interruption/🔣️.json`

cases: {"type": "array", "minItems": 8, "maxItems": 8, "items": {"type": "object", "additionalProperties": false, "required": ["utility", "press", "move", "preview"], "properties": {"utility": {"enum": ["selectDirect", "selectMarquee", "selectLasso", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon"]}, "press": {"$ref": "#/$defs/point"}, "move": {"$ref": "#/$defs/point"}, "preview": {"enum": ["transform", "ove

Definitions to inspect and preserve: `point`.

### `✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/🧬️schema/🔣️.json`

cases: {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": false, "required": ["id", "input", "accepted", "expected"], "properties": {"id": {"type": "string", "minLength": 1}, "input": {"type": "string"}, "accepted": {"type": "boolean"}, "expected": {"anyOf": [{"$ref": "https://json.schemas.assets.semio-tech.com/s/energy/simulation/site/weather/schema.json"}, {"type": "null"}]}}}}

No root definitions.

### `✏️s/🧑‍💻dev/💡️services/🧬️schema/🌉️mcp-composition/🔣️.json`

cases: {"type": "array", "minItems": 9, "maxItems": 9, "items": {"type": "object", "additionalProperties": false, "required": ["target", "source", "removedSource", "command"], "properties": {"target": {"type": "string", "minLength": 1}, "source": {"type": "string", "minLength": 1}, "removedSource": {"type": "string", "minLength": 1}, "command": {"type": "string", "minLength": 1}}}}

No root definitions.

### `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/🧬️schema/🗂️subset-directory-wiring/🔣️.json`

cases: {"type": "array", "minItems": 4, "maxItems": 4, "items": {"type": "object", "additionalProperties": false, "required": ["id", "directories", "reference", "outcome"], "properties": {"id": {"type": "string", "minLength": 1}, "directories": {"type": "array", "minItems": 0, "maxItems": 2, "items": {"type": "string", "minLength": 1}, "uniqueItems": true}, "reference": {"type": "string", "minLength": 1}, "outcome": {"enum"

No root definitions.

### `🧬️schema/🔣️outcome-law-gate/🔣️.json`

cases: {"type": "array", "minItems": 1, "items": {"$ref": "#/definitions/case"}}

Definitions to inspect and preserve: `repositoryPath,segment,level,line,breach,case`.

## Additional Verified Source-and-Expectation Wrappers

`🧰️framework/🔨️modules/🎒️pack/🔤️json/🧬️schema/🫳️read-source/🔣️.json`: source, duplicate, invalidUtf8, fixed largeStringBytes=8194, fixed decodeAllocationBytes=131072, stepUnits. Entire decoder test setup rather than one JSON domain.

`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🧬️schema/🛬️decoding/🔣️.json`: source plus signed/unsigned/floatWord/text/octets/formulaValue and wires with minItems=8, pairing input and decoded expected results. Preserve individual decoded scalar/wire domains where needed.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧬️schema/🚦️test-dispatch/🔣️.json`: test source contents, cancellation marker source, expectedBundle and expectedDefinition are test setup/census assertions.

## Consumer Evidence and Exceptions

Confirmed Ajv whole-corpus consumers: pack schema-hash storage test line 70; OS reactor-contract-oracles line 51; services mcp-composition line 12; block catalog tests line 13; stdio assembly line 10; hierarchy line 10; HTML/MD/TXT/Binary net-leaves test imports lines 9–10; procedural semantic-wire line 43; PDF output line 5 and retirement line 6; WFC bitmap tests line 22; EPW unit tests line 13. Renderer engine-contract test imports drawing interruption corpus schema line 8. CAD spatial-kernel test dynamically loads linear-prism schema line 234.

The puzzle 3D/5D retained-job includes in artifact roots are inside cfg(all(test, feature = "component-app-assembly")); preserve this test gate. ArtifactPackageContractScript is test orchestration and its inventory schema validates genuine computed package inventory as well as positive/negative examples; preserve the inventory domain. Expected preconditions in raster mutation schemas are genuine runtime mutations and must remain. Source fields in affine frame changes, connect-node mutations, document import requests, and presentation snapshots are genuine runtime domains.

Root owns OS kernel script dynamic reads of deleted paged-history-stack and branch-provenance wrapper schemas. Audit did not modify those files.
