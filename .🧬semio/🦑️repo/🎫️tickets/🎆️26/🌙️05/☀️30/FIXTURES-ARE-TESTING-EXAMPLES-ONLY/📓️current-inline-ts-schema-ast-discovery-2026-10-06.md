# Current Inline TypeScript Schema AST Discovery

Observed 2026-10-06T18:19:21.910Z. Authored TS/TSX inventory 10119files; 1018compile calls, 43literal schema-named object declarations. This is discovery, not a passing test or a complete dataflow proof: identifier aliases/imported schemas/builders and anonymous compile literals need separate review. Nearest preceding declaration association can overassociate nested scopes; each candidate needs semantic review.

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/💥️generic-stem-collision-resolution/🟦️.ts:71

Declaration line20 schema

```ts
validator.compile(schema)
{
  $id: "https://semio.local/generic-stem-collision-resolution-vectors",
  type: "object",
  required: ["siblingCases", "gluePurityCases", "packageBoundaryHoistCases"],
  additionalProperties: false,
  properties: {
    packageBoundaryHoistCases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        required: ["id", "packageDir", "ownerTestPath"],
        additionalProperties: false,
        properties: {
          id: { type: "string", minLength: 1 },
          packageDir: { type: "string", minLength: 1 },
          ownerTestPath: { type: "string", minLength: 1 },
        },
      },
    },
    siblingCases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        required: ["id", "dir", "implName", "roleName"],
        additionalProperties: false,
        properties: {
          id: { type: "string", minLength: 1 },
          dir: { type: "string", minLength: 1 },
          implName: { type: "string", minLength: 1 },
          roleName: { type: "string", minLength: 1 },
        },
      },
    },
    gluePurityCases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        required: ["id", "path"],
        additionalProperties: false,
        properties: {
          id: { type: "
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔀️surface-switch/🟦️.ts:501

Declaration line143 FIXTURE_SCHEMA

```ts
new Ajv({ strict: true, allErrors: true }).compile(FIXTURE_SCHEMA)
{
  type: "object",
  additionalProperties: false,
  required: ["note", "dialects", "manifests", "boot", "group", "roleTargets", "switch", "gate", "work", "quiesce", "sealed", "busyLabel", "modeSteps", "keybindings", "keybindingOverride"],
  properties: {
    note: { type: "string", minLength: 1 },
    dialects: { type: "object", additionalProperties: { type: "object", additionalProperties: false, required: ["artifactKind", "standard", "subset"], properties: { artifactKind: { type: "string" }, standard: { type: "string" }, subset: { type: "string" } } } },
    manifests: {
      type: "object",
      additionalProperties: { type: "array", minItems: 1, items: { type: "object", additionalProperties: false, required: ["id", "role"], properties: { id: { type: "string" }, role: { enum: ["editor", "viewer"] }, dialect: { type: "string" } } } },
    },
    boot: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["id", "manifest", "search", "envRole", "defaultAppId", "pinnedAppId", "expectedRole", "expectedAppId"],
        properties: { id: { type: "string" }, manifest: { type: "string" }, search: { type: "string" }, envRole: { enum: ["editor", "viewer"] }, defaultAppId: { type: ["string", "null"] }, pi
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⌨️window-scope/🟦️.ts:162

Declaration line64 FIXTURE_SCHEMA

```ts
new Ajv({ strict: true, allErrors: true }).compile(FIXTURE_SCHEMA)
{
  type: "object",
  additionalProperties: false,
  required: ["note", "apps", "stacks", "dockSeed", "tabPresentation", "chords", "unownedHint", "reservedChords"],
  properties: {
    note: { type: "string", minLength: 1 },
    apps: {
      type: "object",
      minProperties: 2,
      additionalProperties: {
        type: "object",
        additionalProperties: false,
        required: ["kinds", "modes"],
        properties: {
          kinds: { type: "array", minItems: 1, items: { type: "object", additionalProperties: false, required: ["id", "actionIds"], properties: { id: { type: "string", minLength: 1 }, actionIds: { type: "array", items: { type: "string", minLength: 1 } } } } },
          modes: { type: "object", minProperties: 1, additionalProperties: { $ref: "#/$defs/node" } },
        },
      },
    },
    stacks: { type: "array", minItems: 3, items: { type: "object", additionalProperties: false, required: ["id", "app", "mode", "expected"], properties: { id: { type: "string" }, app: { type: "string" }, mode: { type: "string" }, expected: { type: "array", items: { $ref: "#/$defs/stack" } } } } },
    dockSeed: { type: "array", minItems: 6, items: { type: "object", additionalProperties: false, required: ["id", "app", "mode", "activeWindowId", "expected"], properties: { i
```


## Semantic verdicts

All3 discovered literal declarations are genuine wholecorpus authorities, not real domain output contracts: generic-stem collision resolution schema siblingCases/gluePurityCases/packageBoundaryHoistCases, declaration20/compile71; renderer surface-switch FIXTURE_SCHEMA dialects/manifests/boot/group/roleTargets/switch/gate/work/quiesce/sealed/busyLabel/modeSteps/keybindings/keybindingOverride, declaration143/compile501/wholefixture502; renderer window-scope FIXTURE_SCHEMA note/apps/stacks/dockSeed/tabPresentation/chords/unownedHint/reservedChords, declaration64/compile162/wholefixture163. Retire these local schema declarations and wholefixture admission, retain actual source/component factory/action/keyboard behavior and independent library oracles. This closes currently discovered local identifier-literal family only; anonymous compile arguments/builders/aliases are next separate sweep.


## Anonymous and arbitrary-name literal follow-up

Observed 2026-10-06T18:20:36.009Z. 323 literal compile arguments or preceding literal variable associations, irrespective identifier name. Candidates below require semantic review.

### 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts:171

Declaration undefined undefined

```ts
new (require("ajv").default)({ strict: false }).compile({ const: expected })
{ const: expected }
```

### 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts:976

Declaration undefined undefined

```ts
new (createRequire(import.meta.url)("ajv").default)().compile({ const: expected.slice().sort() })
{ const: expected.slice().sort() }
```

### 🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:901

Declaration undefined undefined

```ts
ajv.compile({const:entry.expected})
{const:entry.expected}
```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️mutation-fixtures/🟦️.ts:75

Declaration undefined undefined

```ts
new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: expected.map(node => node.path), properties: Object.fromEntries(expected.map(node => [node.path, { const: node.type }])) })
{ type: "object", additionalProperties: false, required: expected.map(node => node.path), properties: Object.fromEntries(expected.map(node => [node.path, { const: node.type }])) }
```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️mutation-fixtures/🟦️.ts:116

Declaration undefined undefined

```ts
new Ajv({ strict: true, allowMatchingProperties: true }).compile({ type: "object", additionalProperties: false, required: Object.keys(vectors.files), properties: Object.fromEntries(Object.keys(vectors.files).map(path => [path, { type: "string" }])), patternProperties: { "^🧬️schema/🧬️mutations/[^/]
{ type: "object", additionalProperties: false, required: Object.keys(vectors.files), properties: Object.fromEntries(Object.keys(vectors.files).map(path => [path, { type: "string" }])), patternProperties: { "^🧬️schema/🧬️mutations/[^/]+/🧪️tests/[^/]+/🦀️\\.rs$": { type: "string" } } }
```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/💥️generic-stem-collision-resolution/🟦️.ts:71

Declaration 20 schema

```ts
validator.compile(schema)
{
  $id: "https://semio.local/generic-stem-collision-resolution-vectors",
  type: "object",
  required: ["siblingCases", "gluePurityCases", "packageBoundaryHoistCases"],
  additionalProperties: false,
  properties: {
    packageBoundaryHoistCases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        required: ["id", "packageDir", "ownerTestPath"],
        additionalProperties: false,
        properties: {
          id: { type: "string", minLength: 1 },
          packageDir: { type: "string", minLength: 1 },
          ownerTestPath: { type: "string", minLength: 1 },
        },
      },
    },
    siblingCases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        required: ["id", "dir", "implName", "roleName"],
        additionalProperties: false,
        properties: {
          id: { type: "string", minLength: 1 },
          dir: { type: "string", minLength: 1 },
          implName: { type: "string", minLength: 1 },
          roleName: { type: "string", minLength: 1 },
        },
      },
    },
    gluePurityCases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        required: ["id", "path"],
        additionalProperties: false,
        properties: {
          id: { type: "string", minLength: 1 },
          path: { type: "string", minLength: 1 },
        },
      },
    },
  },
}
```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🧪️tests/physical-codecs/🟦️.ts:26

Declaration undefined undefined

```ts
oracle.compile({const:fixture.forms.value})
{const:fixture.forms.value}
```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🧪️tests/physical-codecs/🟦️.ts:33

Declaration undefined undefined

```ts
oracle.compile({const:fixture.graph})
{const:fixture.graph}
```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🧪️tests/physical-codecs/🟦️.ts:46

Declaration undefined undefined

```ts
oracle.compile({const:fixture.txt.expected})
{const:fixture.txt.expected}
```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🧪️tests/physical-codecs/🟦️.ts:56

Declaration undefined undefined

```ts
oracle.compile({const:{bits:fixture.word.bits}})
{const:{bits:fixture.word.bits}}
```

### ✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/⛽️publication-grant/🟦️.ts:13

Declaration undefined undefined

```ts
new Ajv2020({ strict: true }).compile({ type: "integer", minimum: 0, maximum: fixture.construction.backingCeilingBytes / fixture.construction.scalarBytes })
{ type: "integer", minimum: 0, maximum: fixture.construction.backingCeilingBytes / fixture.construction.scalarBytes }
```

### ✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📦️pcg-wire/🟦️.ts:37

Declaration undefined undefined

```ts
new Ajv2020({ strict: true }).compile({ type: "array", minItems: 4, maxItems: 4, prefixItems: [fixture.operation, fixture.revision, fixture.generation, fixture.seed].map((value) => ({ const: value })), items: false })
{ type: "array", minItems: 4, maxItems: 4, prefixItems: [fixture.operation, fixture.revision, fixture.generation, fixture.seed].map((value) => ({ const: value })), items: false }
```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️kind-only-basename/🟦️.ts:58

Declaration undefined undefined

```ts
ajv.compile({ type: "string", const: expectedBasename })
{ type: "string", const: expectedBasename }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎬️presented-media-slots/🟦️.ts:36

Declaration undefined undefined

```ts
ajv.compile({ const: row.accepted })
{ const: row.accepted }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️window-host-context/🟦️.ts:27

Declaration undefined undefined

```ts
new Ajv({ strict: true, allErrors: true }).compile({
    type: "object",
    additionalProperties: false,
    required: ["instanceId", "view", "windows", "expected", "inspectionRefresh"],
    properties: {
      instanceId: { type: "integer" },
      view: { type: "object" },
      windows: { type: 
{
    type: "object",
    additionalProperties: false,
    required: ["instanceId", "view", "windows", "expected", "inspectionRefresh"],
    properties: {
      instanceId: { type: "integer" },
      view: { type: "object" },
      windows: { type: "array" },
      expected: { type: "array" },
      inspectionRefresh: { type: "array" },
    },
  }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts:240

Declaration undefined undefined

```ts
new Ajv2020().compile({ type: "array", minItems: 1, maxItems: fixture.pageBytes, items: { type: "integer", minimum: 0, maximum: 255 } })
{ type: "array", minItems: 1, maxItems: fixture.pageBytes, items: { type: "integer", minimum: 0, maximum: 255 } }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔀️surface-switch/🟦️.ts:501

Declaration 143 FIXTURE_SCHEMA

```ts
new Ajv({ strict: true, allErrors: true }).compile(FIXTURE_SCHEMA)
{
  type: "object",
  additionalProperties: false,
  required: ["note", "dialects", "manifests", "boot", "group", "roleTargets", "switch", "gate", "work", "quiesce", "sealed", "busyLabel", "modeSteps", "keybindings", "keybindingOverride"],
  properties: {
    note: { type: "string", minLength: 1 },
    dialects: { type: "object", additionalProperties: { type: "object", additionalProperties: false, required: ["artifactKind", "standard", "subset"], properties: { artifactKind: { type: "string" }, standard: { type: "string" }, subset: { type: "string" } } } },
    manifests: {
      type: "object",
      additionalProperties: { type: "array", minItems: 1, items: { type: "object", additionalProperties: false, required: ["id", "role"], properties: { id: { type: "string" }, role: { enum: ["editor", "viewer"] }, dialect: { type: "string" } } } },
    },
    boot: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["id", "manifest", "search", "envRole", "defaultAppId", "pinnedAppId", "expectedRole", "expectedAppId"],
        properties: { id: { type: "string" }, manifest: { type: "string" }, search: { type: "string" }, envRole: { enum: ["editor", "viewer"] }, defaultAppId: { type: ["string", "null"] }, pinnedAppId: { type: ["string", "null"] }, expectedRole: { enum: ["editor", "viewer"] }, expectedAppId: { type: ["string", "null"] } },
      },
    },
    group: {
      type: "array",
      minItems: 1,
      items: { type: "object", additionalProperties: false, required: ["id", "manifest", "dialect
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⌨️window-scope/🟦️.ts:162

Declaration 64 FIXTURE_SCHEMA

```ts
new Ajv({ strict: true, allErrors: true }).compile(FIXTURE_SCHEMA)
{
  type: "object",
  additionalProperties: false,
  required: ["note", "apps", "stacks", "dockSeed", "tabPresentation", "chords", "unownedHint", "reservedChords"],
  properties: {
    note: { type: "string", minLength: 1 },
    apps: {
      type: "object",
      minProperties: 2,
      additionalProperties: {
        type: "object",
        additionalProperties: false,
        required: ["kinds", "modes"],
        properties: {
          kinds: { type: "array", minItems: 1, items: { type: "object", additionalProperties: false, required: ["id", "actionIds"], properties: { id: { type: "string", minLength: 1 }, actionIds: { type: "array", items: { type: "string", minLength: 1 } } } } },
          modes: { type: "object", minProperties: 1, additionalProperties: { $ref: "#/$defs/node" } },
        },
      },
    },
    stacks: { type: "array", minItems: 3, items: { type: "object", additionalProperties: false, required: ["id", "app", "mode", "expected"], properties: { id: { type: "string" }, app: { type: "string" }, mode: { type: "string" }, expected: { type: "array", items: { $ref: "#/$defs/stack" } } } } },
    dockSeed: { type: "array", minItems: 6, items: { type: "object", additionalProperties: false, required: ["id", "app", "mode", "activeWindowId", "expected"], properties: { id: { type: "string" }, app: { type: "string" }, mode: { type: "string" }, activeWindowId: { type: ["string", "null"] }, expected: { type: ["string", "null"] } } } },
    tabPresentation: { type: "array", minItems: 4, items: { type: "object", additionalProperties: false, required: ["id", "kindIconId"
```

### ✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️ownership/🟦️.ts:144

Declaration undefined undefined

```ts
new Ajv({ strict: true }).compile({ const: expected })
{ const: expected }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧪️tests/🪪️envelope-identity/🟦️.ts:10

Declaration undefined undefined

```ts
new Ajv({ strict: true }).compile({
    type: "object", additionalProperties: false, required: ["plugin", "artifact", "component", "version"],
    properties: { plugin: { const: plugin }, artifact: { const: artifact }, component: { const: expected.component }, version: { const: expected.version } },
{
    type: "object", additionalProperties: false, required: ["plugin", "artifact", "component", "version"],
    properties: { plugin: { const: plugin }, artifact: { const: artifact }, component: { const: expected.component }, version: { const: expected.version } },
  }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts:71

Declaration undefined undefined

```ts
ajv.compile({ type: "array", maxItems: 64, items: refSchema, allOf: fixture.projectionSlots.filter((slot: { many: boolean }) => !slot.many).map((slot: { name: string }) => ({ contains: { type: "object", required: ["slot"], properties: { slot: { const: slot.name } } }, minContains: 0, maxContains: 1 
{ type: "array", maxItems: 64, items: refSchema, allOf: fixture.projectionSlots.filter((slot: { many: boolean }) => !slot.many).map((slot: { name: string }) => ({ contains: { type: "object", required: ["slot"], properties: { slot: { const: slot.name } } }, minContains: 0, maxContains: 1 })) }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts:93

Declaration undefined undefined

```ts
ajv.compile({ const: { persistedId: "child-1", persistedOwner: expectedOwner } })
{ const: { persistedId: "child-1", persistedOwner: expectedOwner } }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧪️tests/🔬️contract/🟦️.ts:15

Declaration undefined undefined

```ts
ajv.compile({ const: fixture.processCases.map((row: { name: string; outcome: string; expectedAlive: boolean }) => ({ name: row.name, outcome: row.outcome, expectedAlive: row.expectedAlive })) })
{ const: fixture.processCases.map((row: { name: string; outcome: string; expectedAlive: boolean }) => ({ name: row.name, outcome: row.outcome, expectedAlive: row.expectedAlive })) }
```

### ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts:147

Declaration undefined undefined

```ts
new Ajv({ strict: true }).compile({ const: { ...replacements, capturedValues: replacements.values.slice(0, -1), expectedRetiredWhileOpen: replacements.values.length - 1, expectedFinalSnapshots: replacements.values.length } })
{ const: { ...replacements, capturedValues: replacements.values.slice(0, -1), expectedRetiredWhileOpen: replacements.values.length - 1, expectedFinalSnapshots: replacements.values.length } }
```

### 🧰️framework/🔨️modules/🧬️schema/🧾️record/🧪️tests/🔬️unit/🟦️.ts:16

Declaration undefined undefined

```ts
new Ajv().compile({ type: "object", properties: Object.fromEntries(vectors.keys.map((key) => [key, {}])), additionalProperties: false })
{ type: "object", properties: Object.fromEntries(vectors.keys.map((key) => [key, {}])), additionalProperties: false }
```

### 🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts:104

Declaration undefined undefined

```ts
new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: ["contractId", "nodes", "schemaVersion", "staleRemovals"], properties: { contractId: { const: row.expectedContractId }, nodes: { type: "array", minItems: 1 }, schemaVersion: { const: 1 }, staleRemovals: { type
{ type: "object", additionalProperties: false, required: ["contractId", "nodes", "schemaVersion", "staleRemovals"], properties: { contractId: { const: row.expectedContractId }, nodes: { type: "array", minItems: 1 }, schemaVersion: { const: 1 }, staleRemovals: { type: "array", maxItems: 0 } } }
```

### ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🧪️tests/🟦️.test.ts:227

Declaration undefined undefined

```ts
ajv.compile({ $ref: row.expected })
{ $ref: row.expected }
```

### ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️extension-guest-standalone/🟦️.ts:40

Declaration undefined undefined

```ts
new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: ["operatorId", "nodeHash"], properties: { operatorId: { const: fixture.evaluate.operatorId }, nodeHash: { type: "integer", minimum: 0 } } })
{ type: "object", additionalProperties: false, required: ["operatorId", "nodeHash"], properties: { operatorId: { const: fixture.evaluate.operatorId }, nodeHash: { type: "integer", minimum: 0 } } }
```


## Anonymous compile semantic verdicts

Additional definite corpus authority: renderer window-host-context test27–39 constructs inline required instanceId/view/windows/expected/inspectionRefresh and admits entire fixture. Remove inline schema/wholefixture admission; preserve actual windowHostContextBindings and independent oracle comparison. CAD presence-retirement test147–148 builds AJV const over authored storeFixture.localReplacements including expectedRetiredWhileOpen/expectedFinalSnapshots, then validates replacements itself; frozen expected law descriptor selfvalidation, remove AJV admission and preserve actual peer/captured-owner retirement/source/native laws.

All other26-discovery candidates are independently inspected as genuine per-value or produced-output oracles: Print actual output/defaults; physical codec roundtrips; actual filesystem materialization map; source-map candidate mutation registry input algorithm (116 arbitrary mock files candidate, source-shaped inputs, not whole testcase schema); FEM real wire/publication limits; actual basename classification; presented media computed acceptedboolean; real byte page limits; Semio envelope value matching parameters; real member reference/projection limits; actual local-hub process outcomes at53; graph actual produced build plan; patch located schema actual reference; actual flow guest computed operator result. Existing3 identifier-literal wholecorpora remain above pending root closure. No blanket const/expected removal is valid.


## Wrapped Literal Follow-Up — 2026-10-06 20:24 Local

A fresh TypeScript AST walk traversed 12,446 authored TS/TSX files and 1,011 property-access `.compile` calls, unwrapping parenthesized expressions, `as`, type assertions, `satisfies`, and non-null assertions. It resolved direct identifier initializers to the preceding declaration. Exactly one associated literal required unwrapping; none of those wrapped literal schemas contained fixture/corpus/cases/expected/scenarios/vectors markers. This closes the wrapped-literal gap for this discovery technique, not arbitrary alias builders or interprocedural schema construction. Literal and anonymous candidates classified above remain the actionable semantic frontier.

## Fresh Physical Facet Census — 2026-10-06 20:24:04 Local

116,189 authored files were traversed, excluding Git, ticket metadata, dependencies, target/dist/build/Nx caches and generated/distribution directories. Collection matching included fixture-containing directory names, tests, test-fixtures and examples suffixes. Seven schema facets remain, all previously classified genuine reusable testing mutation command contracts under plugin `🧪️testing/📢️publication-fixtures` or `🧬️mutation-fixtures`; no current fixture collection-owned schema facet was found. This is a physical facet conclusion only and does not certify inline schema absence.


## Direct Runtime Import Frontier — 2026-10-06 20:26 Local

A source text scan for static `from` and literal dynamic `import(...)` fixture paths in authored TS/TSX, excluding test/testing/fixture directories and permanent script owners, found thirteen candidate imports: UIDialog story dialog-choice examples; Canvas2dHost mounted-input oracle; and eleven glTF inference `🧪️contract` programs (flatness, elongation, slenderness, aspect ratios, handles, genus, holes, Euler characteristic, boundary loops, axis-aligned bounds, overall size). Their source roles are story/oracle/conformance test execution, not application runtime fixture implementations. No application runtime direct fixture import was found by this bounded syntax scan. Permanent scripts intentionally remain a separate executable test/proof frontier; this result does not cover computed imports, asset URLs, Rust includes, or generated distribution contents, which retain their separate audit qualifications.


## Fresh Post-ACK Inline AST Witness

A current expanded scan parsed12,453 authored TS/TSX files, considered both property-access compile and validate calls, resolved up to four preceding identifier initializer aliases, and unwrapped as/satisfies/parentheses/type assertions. No associated direct object-literal schemas containing cases/scenarios/vectors/expected/laws/sourceCases property declarations remained. This is zero in the explicitly scanned literal/alias vocabulary, not zero for external schema files, generated builders, $refs, generic wrapper calls or metadata-only schemas. Those frontiers retain their independent reports.


## Runtime Reader and Replay Helper Frontier

Current source literal fixture paths outside tests/testing/fixture/story directories and permanent scripts were separately swept in TS/TSX. Matches have testing or repository tooling roles: nativeCatalogSelectionOracleV1 validates only an actual NativeCatalogSelectionInputV1 projection (packages/profiles/availableProviders); registry RustTaxonomyMountsCheckScript and PluginRootOwnershipCheckScript materialize source input cases and execute actual compiler/SQLite laws, with prior corpus admissions now absent; Canvas2d mounted-input is an actual testing oracle; repository verification/orchestration/normalization/discovery manipulate test examples as infrastructure; Stdio artifact contract admits actual arbitrary package declarations; eleven glTF 🧪️contract modules execute conformance examples. No new application runtime fixture reader was confirmed. MCP stale replay authority was separately reported and has now disappeared from the export-name sweep.

Rust fixture literal includes in authored source files were not treated as runtime code merely from path: in-source cfg(test)/test functions and generated testing inputs remain legitimate. This scan has not independently proved cfg gating for every Rust include and therefore makes no global Rust-runtime zero claim. Distribution assets retain their prior canonical regeneration qualification until actual compiler receipt.


## Native Literal Include Gating Witness

A Rust tree-sitter scan traversed16,395 authored `.rs` paths outside named tests/testing/fixture directories, finding110 fixture include_str/include_bytes macro invocations. Ancestor attributes directly established cfg(test)/test gating for97. All13 residuals were manually classified: BMP canonical-byte oracle9 includes are inside `#[cfg(all(test, feature = "oracles"))]` module425; Puzzle2d/3d/5d3 retained-jobs includes are each `#[cfg(all(test, feature = "component-app-assembly"))]` retained_command_test_catalog functions; plugin typing-run1 include is in `#[cfg(any(test, feature = "artifact-app-testing"))] pub mod artifact_app_laws`7391. The last is explicit reusable testing-feature support, not default runtime fixture catalog. Thus no unclassified native literal include remains in this bounded source scan. External `#[path]` module mounts, computed fs reads and generated distributions are outside this witness.


Rust explicit path mounts containing fixture names were separately scanned: nineteen matches are test-owner modules or imports of genuine reusable `🧪️testing` mutation APIs. All six matches located in non-test authored source (UI WGPU engine3812, Energy mutations1562, DB storage9113, plugin360 and7314, Puzzle5 mutations796) have an immediately preceding cfg(test). No fixture path is mounted into default runtime by these nineteen declarations.


Fresh retired asset route and named wrapper scan finds only styling static-dir dedup test inputs120/121/124 using synthetic `/cad-fixture` routes and Puzzle3d Storybook override aliases99/100 for retired `/infinite-fixture` routes. Story also has current `/infinite-assets/` canonical nested and earlier flat asset override aliases101–104, so current asset mapping works; old aliases are unnecessary legacy support, not active application fixture serving. No current production runtime catalog literal for either retired route or CapabilityDescriptionFixture/validateFixture/validateCorpus/admitFixture/assertFixtureSchema remained in this authored syntax scan. Generated bundle stale4 refs retain separate regeneration issue.


Physical collection schema facet follow-up included all file types, not only JSON. No non-JSON authored file remains beneath a `🧬️schema` segment after a fixture/tests/examples collection segment, with the same generated/dependency/ticket exclusions. Seven JSON facets are the actual reusable testing mutation APIs. Thus no hidden WIT/XSD/TS/Rust authority file was missed by the earlier JSON-only physical facet witness.
