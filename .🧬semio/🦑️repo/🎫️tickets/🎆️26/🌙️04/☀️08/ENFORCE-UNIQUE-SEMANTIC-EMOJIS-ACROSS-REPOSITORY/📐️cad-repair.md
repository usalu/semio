# CAD Artifact Emoji Repair

## Scope

Owned tree: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad`.

Every rename was selected explicitly from the CAD concept represented by the node. No automatic emoji selector or rename planner was used.

## Repaired Authorities

- Engine artifact: `📄️artifact` → `🗿️artifact`.
- Configuration alternatives: every colliding `🎚️options` sibling → `☑️options`; `🎚️config` remains configuration.
- Contribution controls: `🧪️oracle` → `🔮️oracle`, `🧪️retained-jobs` → `🗄️retained-jobs`, while `🧪️tests` remains the test authority.
- JSON Schema files colliding with JSON instances: `🔣️.schema.json` → `🧬️.schema.json` in the presence, retained-job, and twenty mutation authorities.
- CAD interchange: each colliding OBJ authority `🧊️obj` → `🗿️obj`; glTF remains `🧊️gltf`.
- Classic model definition: `🏛️aec.building.structure.classic` → `🏺️aec.building.structure.classic` to distinguish it from `🏛️aec.building.structure`.
- Model interaction directories: every `🎬️interactions` sibling → `🕹️interactions`; `🎬️actions` remains the action authority.
- Operation assets were individually named by meaning. Examples include `⚓️createAnchor`, `🪞️mirror`, `🌀️sweep1`, `🌪️sweep2`, `🚀️extrudeCrv`, `🧱️extrudeWire`, `📏️length`, `🟨️area`, `✂️split`, and `✏️trim`.
- Building operations use the represented building element: `🏛️Column`, `🚪️Door`, `🪨️Foundation`, `🏠️Roof`, `🧱️Slab`, `🪜️Stair`, `🛡️Wall`, `🪟️Window`, and `🪵️Beam`.
- Variant-heavy constructors distinguish input geometry: `📍️…From2PointsAndHeight`, `〰️…FromCurveAndHeight`, and `🗺️…FromSurface`, with additional element-specific icons where sibling uniqueness requires them.
- Mutation directory presentation was canonicalized for `🏛️create-structure-classic-model`, `🏷️rename-node`, `👁️change-reference-hidden`, `🖇️replace-reference-media`, and `🗑️delete-node`.

## Central Taxonomy Override

The exact entry below is now present in `testContributionDirectoryOverrides`:

```json
"✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any": "🔮️oracle"
```

## Shared Projection Fixture

The centrally owned `📐️cad-draw-path-projection/🔣️.json` fixture now records the repaired CAD source coordinates. Its source-side fields use these exact directory-segment mappings:

```text
🏛️aec.building.structure.classic → 🏺️aec.building.structure.classic
🎬️interactions                  → 🕹️interactions
```

Exactly 155 `sourcePath` values were updated to the handpicked CAD names. All 209 resulting CAD source paths resolve. No `destinationPath` value changed; destination coordinates remain owned by the projection contract.

The fixture's `modelCatalog.categoryRules[].sourceDirectoryName` value for interactions changed from `🎬️interactions` to `🕹️interactions`, and its classic model `directoryName` changed from `🏛️aec.building.structure.classic` to `🏺️aec.building.structure.classic`.

## References Updated

- Rust `#[path]` mounts for the mutation presentation changes, OBJ authorities, and option authority.
- Rust `include_str!` interaction catalogs and interaction-spec tests.
- TypeScript interaction glob, engine imports, renderer documentation, package tests, and `tsconfig.json` source path.
- Mutation manifests/oracle fields and retained-job `$schema` references.

## Verification

Scoped statute audit:

```text
files       694
directories 505
governed    1198
missing     0
generic     0
presentation 0
spacing     0
duplicate   0
multiple    0
reserved-emoji 0
oracle      0
```

`bun nx run @semio-tech/cad-js:test-quick` currently stops before tests because `🧪️tests/🟦️.ts` imports the absent `createWorkspaceViteResolveConfig` export from itself. This is outside the repaired artifact naming slice.

`bun nx run @semio-tech/cad-plugin:test-quick` reaches Rust compilation and fails with 205 unrelated type/serde errors. Representative diagnostics are an unsatisfied `DslValue: From<&JsonValue>` bound, mismatched types, and missing `Serialize`/`Deserialize` implementations for `CadDiff`, `CadSnapshot`, and `CadMutation`. The output contained no missing-file or unresolved renamed-path diagnostic.
