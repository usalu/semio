# Child Slot Kind and Identity Fixture Audit

Read-only comparison of native snapshot #[child(kind)] metadata with direct child handles in committed mutation snapshots. Array children are visited; absent/null/unrecognized values do not establish evidence. This is a diagnostic scan, not a parser/compiler or runtime proof. No production files were changed.

Scanned 114 snapshot schemas and 43 declared child fields with fixture roots; 32 fields have observed kind or child/target identity disagreements. 6 declared fields had no directly readable child handle evidence.

## Observed Disagreements

| Native Snapshot | Field | Declared Kind | Observed Target Kinds | Unequal IDs |
| --- | --- | --- | --- | --- |
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `document` | `s.stdio.semio.document` | `{"s.stdio.semio": 8}` | 8 |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `mesh` | `s.stdio.semio.mesh` | `{"s.stdio.semio": 4}` | 4 |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `drawing` | `s.stdio.semio.drawing` | `{"s.stdio.semio": 24}` | 0 |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `value` | `s.stdio.semio.value` | `{"s.stdio.semio": 24}` | 0 |
| `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `content` | `s.stdio.semio.graph` | `{"s.stdio.semio": 20}` | 20 |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `catalog` | `s.stdio.semio.kit` | `{"s.stdio.semio": 74}` | 0 |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `shapeModel` | `s.stdio.semio.model` | `{"s.stdio.semio": 39}` | 39 |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `buildingModel` | `s.stdio.semio.model` | `{"s.stdio.semio": 39}` | 39 |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `energyModel` | `s.stdio.semio.model` | `{"s.stdio.semio": 39}` | 39 |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `structureClassicModel` | `s.stdio.semio.model` | `{"s.stdio.semio": 39}` | 39 |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `drawings` | `s.stdio.semio.drawing` | `{"s.stdio.semio": 40}` | 40 |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `modules` | `s.stdio.semio.kit` | `{"s.stdio.semio": 36}` | 36 |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `knowledge` | `s.stdio.semio.table` | `{"s.stdio.semio": 532}` | 532 |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `benchmarks` | `s.stdio.semio.table` | `{"s.stdio.semio": 532}` | 532 |
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `document` | `s.stdio.semio.document` | `{"s.stdio.semio": 18}` | 18 |
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `flow` | `s.stdio.semio.flow` | `{"s.stdio.semio": 18}` | 18 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `content` | `s.stdio.semio.graph` | `{"s.stdio.semio": 16}` | 16 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `climate` | `s.stdio.semio.table` | `{"s.stdio.semio": 26}` | 26 |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `structure` | `s.stdio.semio.value` | `{"s.stdio.semio": 20}` | 20 |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `results` | `s.stdio.semio.table` | `{"s.stdio.semio": 20}` | 20 |
| `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `catalog` | `s.stdio.semio.kit` | `{"s.stdio.semio": 6}` | 0 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `qK` | `s.stdio.semio.table` | `{"s.stdio.semio": 20}` | 20 |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `stockSolid` | `s.stdio.semio.brep` | `{"s.stdio.semio": 32}` | 32 |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `steps` | `s.stdio.semio.flow` | `{"s.stdio.semio": 32}` | 32 |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `toolSolids` | `s.stdio.semio.brep` | `{"s.stdio.semio": 15}` | 15 |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | `content` | `s.stdio.semio.graph` | `{"s.stdio.semio": 28}` | 28 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/🦀️.rs` | `brep` | `s.stdio.semio.brep` | `{"s.stdio.semio": 4}` | 4 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/🦀️.rs` | `mesh` | `s.stdio.semio.mesh` | `{"s.stdio.semio": 6}` | 6 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/🦀️.rs` | `properties` | `s.stdio.semio.value` | `{"s.stdio.semio": 2}` | 2 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🦀️.rs` | `objects` | `s.stdio.semio.object` | `{"s.stdio.semio": 30}` | 30 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🦀️.rs` | `models` | `s.stdio.semio.model` | `{"s.stdio.semio": 30}` | 30 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🦀️.rs` | `properties` | `s.stdio.semio.value` | `{"s.stdio.semio": 2}` | 2 |

## Representative Unequal-ID Inputs

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-writer/🏷️renames-the-document-to-mission-brief/📸️snapshot/➡️after/🔣️.json` (document)
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-writer/🏷️renames-the-document-to-mission-brief/📸️snapshot/⬅️before/🔣️.json` (document)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎚️change-exaggeration/⛰️raises-exaggeration-from-1-to-2-5/📸️snapshot/➡️after/🔣️.json` (mesh)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎚️change-exaggeration/⛰️raises-exaggeration-from-1-to-2-5/📸️snapshot/⬅️before/🔣️.json` (mesh)
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔷change-node-shape/🧪️reports-a-no-op-when-the-shape-already-reads-circle/📸️snapshot/➡️after/🔣️.json` (content)
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔷change-node-shape/🧪️reports-a-no-op-when-the-shape-already-reads-circle/📸️snapshot/⬅️before/🔣️.json` (content)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/➡️after/🔣️.json` (shapeModel)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/⬅️before/🔣️.json` (shapeModel)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/➡️after/🔣️.json` (buildingModel)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/⬅️before/🔣️.json` (buildingModel)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/➡️after/🔣️.json` (energyModel)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/⬅️before/🔣️.json` (energyModel)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/➡️after/🔣️.json` (structureClassicModel)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/⬅️before/🔣️.json` (structureClassicModel)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/➡️after/🔣️.json` (drawings)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/⬅️before/🔣️.json` (drawings)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🪶️remove-weight/🪶️drops-the-wall-module-weight-override/📸️snapshot/➡️after/🔣️.json` (modules)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🪶️remove-weight/🪶️drops-the-wall-module-weight-override/📸️snapshot/➡️after/🔣️.json` (modules)
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧑️user-profile/🌱️create/🌱️creates-a/📸️snapshot/➡️after/🔣️.json` (knowledge)
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧑️user-profile/🌱️create/🌱️creates-a/📸️snapshot/⬅️before/🔣️.json` (knowledge)
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧑️user-profile/🌱️create/🌱️creates-a/📸️snapshot/➡️after/🔣️.json` (benchmarks)
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧑️user-profile/🌱️create/🌱️creates-a/📸️snapshot/⬅️before/🔣️.json` (benchmarks)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🩹update-step/🧪️no-ops-when-the-header-is-already-current/📸️snapshot/➡️after/🔣️.json` (document)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🩹update-step/🧪️no-ops-when-the-header-is-already-current/📸️snapshot/⬅️before/🔣️.json` (document)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🩹update-step/🧪️no-ops-when-the-header-is-already-current/📸️snapshot/➡️after/🔣️.json` (flow)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🩹update-step/🧪️no-ops-when-the-header-is-already-current/📸️snapshot/⬅️before/🔣️.json` (flow)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/➕️create-node/🚫️rejects-a-node-id-the-scene-already-holds/📸️snapshot/➡️after/🔣️.json` (content)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/➕️create-node/🚫️rejects-a-node-id-the-scene-already-holds/📸️snapshot/⬅️before/🔣️.json` (content)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏢️change-reference-qp-kwh/📉️lowers-the-reference-building-primary-energy-to-8750-kwh/📸️snapshot/➡️after/🔣️.json` (climate)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏢️change-reference-qp-kwh/📉️lowers-the-reference-building-primary-energy-to-8750-kwh/📸️snapshot/⬅️before/🔣️.json` (climate)
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️change-form-title/🧪️titles-an-untitled-survey/📸️snapshot/➡️after/🔣️.json` (structure)
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️change-form-title/🧪️titles-an-untitled-survey/📸️snapshot/⬅️before/🔣️.json` (structure)
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️change-form-title/🧪️titles-an-untitled-survey/📸️snapshot/➡️after/🔣️.json` (results)
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️change-form-title/🧪️titles-an-untitled-survey/📸️snapshot/⬅️before/🔣️.json` (results)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️change-variable-action-category/🚫️refuses-to-recategorise-a-missing-action-0/📸️snapshot/➡️after/🔣️.json` (qK)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️change-variable-action-category/🚫️refuses-to-recategorise-a-missing-action-0/📸️snapshot/⬅️before/🔣️.json` (qK)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/❌delete-machine/➖️empties-the-workshop-of-the-saw/📸️snapshot/➡️after/🔣️.json` (stockSolid)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/❌delete-machine/➖️empties-the-workshop-of-the-saw/📸️snapshot/⬅️before/🔣️.json` (stockSolid)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/❌delete-machine/➖️empties-the-workshop-of-the-saw/📸️snapshot/➡️after/🔣️.json` (steps)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/❌delete-machine/➖️empties-the-workshop-of-the-saw/📸️snapshot/⬅️before/🔣️.json` (steps)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔀reorder-steps/🔀️accepts-a-target-index-and-reorders-them/📸️snapshot/➡️after/🔣️.json` (toolSolids)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔀reorder-steps/🔀️accepts-a-target-index-and-reorders-them/📸️snapshot/⬅️before/🔣️.json` (toolSolids)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🌱create-node/🧪️rejects-a-duplicate-node-id/📸️snapshot/➡️after/🔣️.json` (content)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🌱create-node/🧪️rejects-a-duplicate-node-id/📸️snapshot/⬅️before/🔣️.json` (content)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧫️fixtures/🧬️mutations/🧱create-brep/🧱️attaches-a-brep-child-to-an-object-that-has-none/📸️snapshot/➡️after/🔣️.json` (brep)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧫️fixtures/🧬️mutations/🧨delete-mesh/🧨️detaches-the-mesh-child-and-leaves-the-brep-child-alone/📸️snapshot/➡️after/🔣️.json` (brep)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧫️fixtures/🧬️mutations/🕸️create-mesh/🕸️attaches-a-mesh-child-to-an-object-that-has-none/📸️snapshot/➡️after/🔣️.json` (mesh)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧫️fixtures/🧬️mutations/🚫delete-properties/🚫️detaches-the-properties-child-and-leaves-the-mesh-child-alone/📸️snapshot/➡️after/🔣️.json` (mesh)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧫️fixtures/🧬️mutations/🏷️create-properties/🏷️attaches-a-properties-child-to-an-object-that-has-none/📸️snapshot/➡️after/🔣️.json` (properties)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧫️fixtures/🧬️mutations/🚫delete-properties/🚫️detaches-the-properties-child-and-leaves-the-mesh-child-alone/📸️snapshot/⬅️before/🔣️.json` (properties)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧫️fixtures/🧬️mutations/🏛️create-model/🏛️attaches-a-second-model-child/📸️snapshot/➡️after/🔣️.json` (objects)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧫️fixtures/🧬️mutations/🏛️create-model/🏛️attaches-a-second-model-child/📸️snapshot/⬅️before/🔣️.json` (objects)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧫️fixtures/🧬️mutations/🏛️create-model/🏛️attaches-a-second-model-child/📸️snapshot/➡️after/🔣️.json` (models)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧫️fixtures/🧬️mutations/🏛️create-model/🏛️attaches-a-second-model-child/📸️snapshot/➡️after/🔣️.json` (models)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧫️fixtures/🧬️mutations/🚫delete-properties/🚫️detaches-the-properties-child-and-leaves-every-other-collection-alone/📸️snapshot/⬅️before/🔣️.json` (properties)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧫️fixtures/🧬️mutations/🏷️create-properties/🏷️attaches-a-properties-child-to-a-kit-that-has-none/📸️snapshot/➡️after/🔣️.json` (properties)

## Fields Without Direct Handle Evidence

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`: `image`, declared `s.stdio.semio.image`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`: `emblem`, declared `s.stdio.semio.image`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`: `structure`, declared `s.stdio.semio.value`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`: `zones`, declared `s.stdio.semio.table`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`: `kindCatalogs`, declared `s.stdio.semio.kit`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`: `backgroundDrawing`, declared `s.stdio.semio.drawing`

## Follow-up

Inspect each disagreement against the actual handle constructor and MemberFactory dialect declaration before changing it. The shared kind gate compares artifactKind only; a subset belongs to the full ArtifactDialect, not a dot-appended pseudo-kind. Equality of childId and target.artifactId is the admitted global owned-child contract. Fix both current native constructors and committed fixture references, then run actual ChildRestoreProjection tests. This audit does not justify inventing child payload, IDs, or factories.
