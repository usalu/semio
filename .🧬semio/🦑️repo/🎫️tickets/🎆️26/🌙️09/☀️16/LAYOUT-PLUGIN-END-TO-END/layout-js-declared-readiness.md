# Declared LayoutJS Gate Readiness

The exact declared frontdoor is repo --service=off run @semio-tech/layout-js:test, from the actual repository root in prototype mode. This is not an independent early Native-style route: the ordinary runtime applies buildOperationRequiresV1 and ensureBuildEnvironmentV1 before task graph/executor dispatch. Current run/test operation requirements are dependencies. A declared attempt therefore cannot be assumed to execute independently of JavaScript readiness. The ensure owner can invoke its canonical installer/publication/readmission path when material is missing, so launching it now is not a guaranteed read-only Bun-only action. No command was executed.

The package project target uses repo:script, cwd ✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript, command repo ./📜️script.ts test, forwardAllArgs true. Its router receives a genuine Script invocation, registers TestScript, and delegates to runTestScript. That program has two awaited owned child commands: the current Bun process runs its explicit 16 test entries, then the same Bun process runs the resolved TypeScript compiler with noEmit/strict on behavior.test.ts, the test program, and structure law. It does not dispatch Cargo, setup, cache maintenance or native-storage in its own command body.

The tests are JavaScript/TypeScript with independent test-library oracles. They are not literally free of native test libraries: portable-oracles imports @napi-rs/canvas and pdfjs-dist alongside three/lodash/fast-xml-parser/fast-json-patch. Those are existing installed prerequisites, not a Cargo launcher here. The behavior aggregator invokes drawing, flow, export, canonical JSON, interchange, schema birth and retained-diff laws. The PDF oracle configures canvas globals and a fileURL pdfjs worker source; this is distinct from native cache-image effects. This bounded program review does not manufacture a complete transitive no-effects proof for every import.

## Current declared input roots

The project runtimeSourceEntries enumerates 51 entries, including imported laws and portable-oracles. The actual first Bun child explicitly selects these 16 roots:

- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🖼️frame-planning/🧪️tests/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🎨️fidelity/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/📄️large-sheet/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️canvas/🧵️paragraph-work/🧪️tests/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/👁️layer-order/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔣️transport/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/✅️validation/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🏗️structure/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🕸️graph/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️frame-selection/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/behavior.test.ts

## Inspected identities and route evidence

- ✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/📋️project.json: SHA256 0f5088e2d52eb62299563ab91a4fd83cca299a6c7550dfdf60bbd4d20c61cb4d
- ✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/📜️script.ts: SHA256 e6397c338968118fa627a7fba335c4b7051a62115d69d9c2ed5bd8a2b3a4ee90
- ✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/🟦️.ts: SHA256 7f2fa59ec37c3bcc7c6871e9d069c3000788df12d359f2a00493369876a8e7b5
- ✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/behavior.test.ts: SHA256 779be9bd6091208b6e416c2937716889e8a150dfbc8a4acde02869f6b1fae83a
- ✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/portable-oracles.mjs: SHA256 43b631fa48d9b94b1d82cb0527d986c9ca573f3914665741c4141febb16088e4

The defining readiness branch is build/command/runtime lines53–54, before ordinary graph-backed execution. build/operations reads the neutral catalog and defaults unknown commands to dependencies; no environment bypass was proposed. Source review alone cannot assert the present installed receipt is qualified. Existing setup7 terminal busy and unproved full rematerialization remain relevant operational history; only a genuine current ready observation could discharge that prerequisite. Direct script execution or tools exec substitution would not prove this declared gate and is not recommended as a bypass.

Native8 actual-log postprocessors remain separately conditional, zero attempts, and were not affected by this read-only review.
