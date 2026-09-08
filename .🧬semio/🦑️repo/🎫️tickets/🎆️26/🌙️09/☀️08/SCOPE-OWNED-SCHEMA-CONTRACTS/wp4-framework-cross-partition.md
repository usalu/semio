# WP4 framework-modules — cross-partition rewrite requests

Every line: replace the first string with the second in the named file. `$id` fragments
`#/definitions/<name>` keep `<name>`; a bare old id becomes `<new id>#/$defs/<Export>`.

#### `📜️script.ts`

- path  "🧰️framework/🔨️modules/🧵️job/🧪️fixtures/📡️shared-framework-action-routes.schema.json"  →  "🧰️framework/🔨️modules/🧵️job/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🧵️job/🧪️fixtures/🧬️fixed-operation-registry.schema.json"  →  "🧰️framework/🔨️modules/🧵️job/🧬️schema/🔣️.json"
- id    "semio://framework/plugin/shared-framework-action-routes/v1"  ->  "https://semio.tech/schema/framework/job/schema.json#/$defs/SharedFrameworkActionRoutesFixture"
- id    "semio://framework/job/fixed-operation-registry-law/v1"  ->  "https://semio.tech/schema/framework/job/schema.json#/$defs/FixedOperationRegistryFixture"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts`

- path  "../../../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/📇️descriptor-load/🧬️.schema.json"  →  "../../../../../../../../../🔨️modules/🎠️kernel/🧬️schema/🔣️.json"
- path  "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧪️fixtures/🔣️.schema.json"  →  "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/📜️action-semantics.schema.json"  →  "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🛤️tutorial-document-track.schema.json"  →  "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧯️router-plugin-faults.test.ts`

- path  "../../../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🧬️.schema.json"  →  "../../../../../../../../../🔨️modules/🎠️kernel/🧬️schema/🔣️.json"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx`

- path  "../../../../../../../🔨️modules/🌱️value/💾️resident/🧬️schema.json"  →  "../../../../../../../🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🎭️actor/📃️page/🧬️schema.json"  →  "../../../../../../../🔨️modules/🎭️actor/📃️page/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/⚙️owned-operations/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌱️root-source/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌳️owned-nodes/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎟️read-lease/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎬️owned-scene/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/👶️native-child/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/💾️resident/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📃️scene-json-document/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📤️read-publication/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📥️intake/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📦️scene-generic-pack/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📨️wire-operations/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔏️owned-hash/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔔️intake-notification/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔗️scene-binding/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔢️scene-numeric/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔤️scene-text-bytes/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🗺️owned-surface/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚨️intake-close-fault/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚪️instance-close/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛠️instance-maintenance/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛡️owned-validation/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧳️scene-pack-field/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧵️scene-json-string/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧾️typed-scene/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🩹️patch/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪆️surface-child/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪪️instance-owner/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🔣️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🏷️fields/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/📤️decode/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🗺️surface-bytes/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🧾️typed/🧬️.schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/📐️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema/🔣️.json"
- path  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧪️tests/🧬️schema/🔣️.json"  →  "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`

- path  "../../../../../../../🔨️modules/🌱️value/💾️resident/🧬️schema.json"  →  "../../../../../../../🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json"

#### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`

- path  "🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧬️outputs.schema.json"  →  "🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧬️catalog.schema.json"  →  "🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧬️schema/🔣️.json"

#### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json`

- path  "🧰️framework/🔨️modules/🌱️value/💾️resident/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema.json"  →  "🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json"

#### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/🧬️schema/🔣️.json`

- path  "🧰️framework/🔨️modules/🌱️value/💾️resident/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema.json"  →  "🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🟦️.ts`

- id    "semio.actor.retained-return.v1"  ->  "https://semio.tech/schema/framework/actor/return/schema.json#/$defs/Return"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition/🟦️.ts`

- id    "semio.kernel.poll.composition.v1"  ->  "https://semio.tech/schema/framework/kernel/poll/composition/schema.json#/$defs/Composition"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`

- id    "semio.actor.shard-liveness.v1"  ->  "https://semio.tech/schema/framework/actor/shard-client/schema.json#/$defs/ShardClient"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`

- id    "semio.actor.shard-liveness.v1"  ->  "https://semio.tech/schema/framework/actor/shard-client/schema.json#/$defs/ShardClient"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🎥️tutorial-interaction/🧬️schema.json`

- id    "https://semio.tech/schema/framework/interaction/component.json"  ->  "https://semio.tech/schema/framework/interaction/schema.json#/$defs/Interaction"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/💾️resident/🧬️schema.json`

- id    "semio.value.resident.capacity.v1"  ->  "https://semio.tech/schema/framework/value/resident/schema.json#/$defs/Resident"

#### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json`

- path  "🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🏠️local-interaction/🔣️.schema.json"  →  "🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧬️favicon.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧬️schema/🔣️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🚪️handback/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🚪️handback/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🩹️patch/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🩹️patch/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧬️catalog.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️fixtures/🔣️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️components.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📋️list/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📋️list/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📮️handback/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📮️handback/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/📦️whole/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/📦️whole/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/📃️document/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/📃️document/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🌳️root/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🌳️root/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🗃️fixed/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🗃️fixed/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔗️bindings/📋️copy/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔗️bindings/📋️copy/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/⚙️owned-operations/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌱️root-source/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌳️owned-nodes/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎟️read-lease/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎬️owned-scene/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/👶️native-child/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/💾️resident/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📃️scene-json-document/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📤️read-publication/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📥️intake/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📦️scene-generic-pack/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📨️wire-operations/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔏️owned-hash/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔔️intake-notification/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔗️scene-binding/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔢️scene-numeric/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔤️scene-text-bytes/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🗺️owned-surface/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚨️intake-close-fault/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚪️instance-close/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛠️instance-maintenance/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛡️owned-validation/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧳️scene-pack-field/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧵️scene-json-string/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧾️typed-scene/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🩹️patch/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪆️surface-child/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪪️instance-owner/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🔣️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🏷️fields/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/📤️decode/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🗺️surface-bytes/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🧾️typed/🧬️.schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/📐️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪞️copy/🧬️schema.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪞️copy/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🎛️tutorial-local-interaction.schema.json"  →  "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"
- path  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧪️tests/🧬️schema/🔣️.json"  →  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json"

## §job-budget — not applied, needs the root `📜️script.ts` in the same change

`🧰️framework/🔨️modules/🧵️job/⏱️budget/🧬️schema/` still holds three non-canonical filenames
(`⏱️clock.json`, `🪢️binding.json`, `🪫️budget.json`), which contract §B forbids inside a `🧬️schema/`
module. Consolidating them into `⏱️budget/🧬️schema/🔣️.json` with exports `Clock`, `Binding`, `Budget`
requires these root-`📜️script.ts` edits at the same time (line numbers as of this report):

- `:2012` `readFileSync(join(base, "🧬️schema/🪫️budget.json"))`
  → `readFileSync(join(base, "🧬️schema/🔣️.json"))` and validate with
  `addSchema(m).getSchema(\`${m.$id}#/$defs/Budget\`)!`
- `:2029` `…compile(JSON.parse(readFileSync(join(base, "🧬️schema/⏱️clock.json"))))`
  → `…addSchema(m).getSchema(\`${m.$id}#/$defs/Clock\`)!`
- `:2055` `…compile(JSON.parse(readFileSync(join(base, "🧬️schema/🪢️binding.json"))))`
  → `…addSchema(m).getSchema(\`${m.$id}#/$defs/Binding\`)!`

and these data-file `$schema` pointers (inside the partition, will be updated by whoever applies it):
`⏱️budget/🕰️clock.json:2`, `⏱️budget/🪢️binding.json:2`, `⏱️budget/🧫️fixture/🔣️.json:2`, plus the
matching `"$schema": {"const": …}` in each of the three schema files. `$id` would become
`https://semio.tech/schema/framework/job/budget/schema.json`.
