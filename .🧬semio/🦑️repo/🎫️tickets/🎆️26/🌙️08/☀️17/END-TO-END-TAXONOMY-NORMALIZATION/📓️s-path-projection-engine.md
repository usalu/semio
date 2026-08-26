# S-PATH-PROJECTION-ENGINE

## Outcome

The normalization engine now strictly loads the live v7 projection registries and projects catalog-registered mutation scenario bundles from the standard/subset/schema hierarchy to artifact-level profile storage. Projection is forward-only, schema-driven, deterministic, collision checked, path-budget checked, reference-aware, and fail closed.

Implemented in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`:

- strict parsing and cross-reference validation for `semanticProjectedMemberKinds`, profile renderers, descendant contracts, independent `vectors` catalog contracts, projection contracts, and `mutationCatalogProjection`;
- exact source hierarchy capture and artifact-owner registry matching;
- exact catalog parsing with `kinds` deliberately independent from required physical `vectors`;
- exact-first scenario association followed only by the unique one-unmatched-source ↔ one-unmatched-vector remainder;
- exact 13-node/no-symlink bundle validation with one diff alternative;
- registry-canonical NFC/VS16 mutation member rendering and catalog-canonical scenario rendering;
- artifact/profile/member/scenario destination rendering, collision/path-budget gates, 6 physical file moves per accepted scenario, and rationale `artifact-mutation-test-projection-v1`;
- already-projected tree recognition, bundle validation, and preservation for second-inventory convergence;
- Rust/Gherkin/JSON/Python structural reference tokens, `asset://` rewriting, directory-target projection, and stale old-token verification;
- post-apply exact 6-move/13-node/source-absence/symlink verification before source-directory pruning;
- strict generator ownership narrowed to `owned | external`, with mandatory owned `previewTarget` loading and transitional ownership rejection.

No production path outside the exclusive normalization module was changed by this lane. No Git mutation or Compose/temp-Compose access was performed.

## Deterministic evidence

### Focused golden

```text
bun test '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern='mutation projection golden|projects every registered golden bundle'
2 pass
215 filtered out
0 fail
51 expect() calls
```

The golden covers the four canonical cases, including the three source/canonical DIN identifier splits and all six physical leaves per case.

### Build and static checks

```text
bun build '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts' --target=bun --outfile='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️path-projection-engine-build.js'
Bundled 15 modules in 20ms
```

`bunx tsc --noEmit --allowImportingTsExtensions --moduleResolution bundler --module preserve --target es2023 --skipLibCheck <module>` reported no error in the owned normalization module. It remained nonzero because of pre-existing/shared errors in FSM glue, actor shard client, machine, and UI `ImportMeta.env/glob` sources.

```text
git diff --check -- <normalization module>
# no output

rg -n '\[DEBUG\]' <normalization module>
# no output
```

Final recorded digests for this lane boundary:

- engine SHA-256: `d63d28a079408d31266e56d1a18d086d4e564fea302c667887d22e78fc6d6d7b`
- retained bundle SHA-256: `96553299c4c3d115b2f50068a1e20fabc562117f29c3ad0af2fc810c5ec7cd27`

### Bounded full plugins census

Command shape:

```text
inventoryTaxonomy({repoRoot, scope:'✏️s/🔌️plugins', ticketDir:<active-ticket>, workers:1})
```

Observed deterministic boundary:

| Measure | Count |
|---|---:|
| Inventory entries | 74,444 |
| Inventory digest | `d947526acd7c6eaaeba0f27f8055c7cb09cf0710feff2c95a997ab8f46477f73` |
| Physical catalog scenario rows | 1,555 |
| Physical 13-node bundles | 1,555 |
| Physical file moves at full closure | 9,330 |
| Census-accepted scenarios before owner-registry repair | 1,100 |
| Census-accepted nodes before owner-registry repair | 14,300 |
| Census-accepted file moves before owner-registry repair | 6,600 |

The initial 1,100 count exposed two engine defects that are fixed in the final module: artifact ownership is now checked against the exact `members-of-artifacts` registry rather than a shadowable global directory-kind result, and multi-emoji mutation names no longer assume a one-emoji-prefix ↔ mutation-ID relation.

The target 1,555/20,215/9,330 census remains intentionally blocked by live schema/catalog/path data listed below. A direct read-only catalog-to-physical-bundle bijection audit proves `physical=1555`, `coverage=0`, `catalogMissing=0`; after registry and budget gates it proves `assigned=1426`, `registryMissing=126`, `budget=3` at the recorded boundary.

## Fail-closed live-data blockers

### Mutation member registry closure

The current read-only re-audit has **125 unique unmatched `mutationDirectoryName` values spanning 126 scenario rows**. This corrects the earlier shorthand “126 unmatched names”: `🔖add-representation-tag` occurs in two catalogs/two scenario rows; every other listed value occurs in one catalog/one scenario row. Five names are emoji-leading and cover six rows; 120 are unprefixed and cover 120 rows. The discovery helper normatively requires exact canonical membership in `members-of-schema`, so the engine does not union catalog strings or invent emojis.

Exact values:

```text
🔖add-representation-tag (2 catalogs, 2 scenarios)
🔖️change-handle-kind-label
🔖️rename-block
🔖rename-machine
🔖️rename-paint-layer
bind-default-scene
bind-morph-target-attribute
bind-node-camera
bind-node-child
bind-node-mesh
bind-node-skin
bind-primitive-attribute
bind-primitive-indices
bind-primitive-material
bind-scene-root-node
change-asset-descriptive-metadata
change-asset-extension-data
change-asset-extra-data
change-asset-version
change-document-extension-data
change-document-extra-data
change-material-alpha-mode
change-material-double-sided
change-mesh-extension-data
change-mesh-extra-data
change-mesh-morph-weights
change-mesh-name
change-node-extension-data
change-node-extra-data
change-node-morph-weights
change-node-name
change-primitive-extension-data
change-primitive-extra-data
change-primitive-topology-mode
change-scene-extension-data
change-scene-extra-data
change-scene-name
create-accessor
create-animation
create-buffer
create-buffer-view
create-camera
create-image
create-material
create-mesh
create-morph-target
create-node
create-primitive
create-sampler
create-scene
create-skin
create-texture
declare-used-extension
delete-accessor
delete-animation
delete-buffer
delete-buffer-view
delete-camera
delete-image
delete-material
delete-mesh
delete-morph-target
delete-node
delete-primitive
delete-sampler
delete-scene
delete-skin
delete-texture
move-accessor
move-animation
move-buffer
move-buffer-view
move-camera
move-image
move-material
move-mesh
move-morph-target
move-morph-target-attribute
move-node
move-node-child
move-primitive
move-primitive-attribute
move-required-extension
move-sampler
move-scene
move-scene-root-node
move-skin
move-texture
move-used-extension
reorder-accessors
reorder-animations
reorder-buffer-views
reorder-buffers
reorder-cameras
reorder-images
reorder-materials
reorder-meshs
reorder-morph-target-attributes
reorder-morph-targets
reorder-node-children
reorder-nodes
reorder-primitive-attributes
reorder-primitives
reorder-required-extensions
reorder-samplers
reorder-scene-root-nodes
reorder-scenes
reorder-skins
reorder-textures
reorder-used-extensions
reparent-node
require-extension
transform-node
unbind-default-scene
unbind-morph-target-attribute
unbind-node-camera
unbind-node-child
unbind-node-mesh
unbind-node-skin
unbind-primitive-attribute
unbind-primitive-indices
unbind-primitive-material
unbind-scene-root-node
unrequire-extension
withdraw-used-extension
```

One artifact owner is also absent from `members-of-artifacts`: `💾️binary` (one physical scenario).

### Former four catalog-invalid diagnostics

The census originally reported four `projection-catalog-invalid` diagnostics:

1. `🏛️architect/.../🏛️program/.../🔣️component.json`, vector 0: `connect-adjacency` / `🔗🧲connect-adjacency`.
2. `🏗️fem/.../◻2d/.../🔣️component.json`, vector 2: `create-combination` / `🌱🔗️create-combination`.
3. `🏗️fem/.../🧊️3d/.../🔣️component.json`, vector 2: `create-combination` / `🌱🔗️create-combination`.
4. `🌊️flow/.../🌊️flow/.../🔣️component.json`, vector 7: `reorder-widgets` / `🔀️🪟️reorder-widgets`.

These were an engine defect, not catalog defects: the catalog contract does not require `mutationId` to equal the suffix after exactly one emoji. The overconstraint was removed. Exact multi-emoji physical names now resolve through registry identity.

### 129 unrealized diagnostics

At the recorded census boundary the 129 `projection-catalog-unrealized` rows decompose exactly as:

- 126 scenario rows blocked by missing `members-of-schema` identity;
- 3 scenario rows blocked by the path budget.

There is no catalog/source coverage remainder: exact-first plus unique-remainder association covers all 1,555 physical bundles.

### Three path-budget diagnostics

All three projected scenario roots are 199 bytes; adding the schema-derived 42-byte longest descendant suffix yields **241 bytes**, one byte over the v7 240-byte maximum:

1. `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🧪️tests/🪆️1-any/➖️delete-generation/🧪️removes-the-selected-generation-2-and-falls-back-to-generation-1`
2. `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🧪️tests/🪆️1-any/🍀️change-humidification-provided-kg-h/🧪️drops-provided-humidification-to-1-point-25-kg-per-hour`
3. `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🧪️tests/🪆️1-any/🌾️change-humidification-required-kg-h/🧪️raises-required-humidification-to-3-point-5-kg-per-hour`

The engine correctly leaves these blocking; catalog-owned canonical identifiers must be shortened. It does not truncate or invent aliases.

## Acceptance state

- [x] Projection schema is loaded without fallback/default fields.
- [x] Catalog `vectors` is required and independent from runtime `kinds`.
- [x] Golden first-plan projection is green, including DIN source/canonical splits.
- [x] Exact 13-node bundles and one diff alternative are validated.
- [x] All six physical leaves receive projection rationale and final paths.
- [x] Directory mappings remain available to structured reference rewriting.
- [x] Apply verifies move count, bundle shape, source absence, symlink absence, and stale old tokens before pruning.
- [x] Already-projected catalog-owned trees are recognized and normalized in place for empty-second-plan convergence.
- [x] Transitional generator ownership states are rejected.
- [ ] Full 1,555/20,215/9,330 live census: blocked solely on schema/catalog member closure, one artifact-owner registration, and three catalog path-budget identifiers after the two engine defects above were corrected.
