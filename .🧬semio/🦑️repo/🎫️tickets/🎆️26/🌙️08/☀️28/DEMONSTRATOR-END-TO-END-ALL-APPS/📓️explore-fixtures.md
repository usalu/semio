# Demonstrator Fixture Loading End-to-End

## 1. Fixture Directories & Content

### Framework OS Host Fixtures
**Path**: `🧰️framework/🛍️products/💻️os/🧫️fixtures` (framework-level test fixtures)
- **Content**: Test fixtures for framework components
- **Format**: Mixed (`.dsl`, `.spk`, `.json`, `.proto`, `.rs`, protocol-buffer schemas)
- **Files**:
  - DSL files: `🗣️diamond.dsl`, `🗣️single-edge.dsl`, `🗣️dead-end.dsl`, `🗣️chain.dsl`
  - SPK files: `.spk` binary session/package formats
  - Schema/config: JSON, protobuf, GraphQL for various subsystems (manifest, replication, channel, plugin)

**Modules with fixtures**:
- `🧰️framework/🔨️modules/🌉️abi/🧪️fixtures` - Application Binary Interface test data
- `🧰️framework/🔨️modules/🧵️job/🧪️fixtures` - Job/task test fixtures
- `🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures` - Plugin manifest test schemas
- `🧰️framework/🔨️modules/📡️replication/🧫️fixtures` - Data replication test cases
- `🧰️framework/🔨️modules/🎠️kernel/🧪️fixtures` - Kernel session test fixtures

### Plugin Fixtures (🔌️ plugins)
Framework-wide fixtures (plugins contribute shared data):
- `✏️s/🔌️plugins/📕️norm/🧪️fixtures` - Normative standard test data
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧪️fixtures` - Flow extension tests
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/🧫️fixtures` - File format handlers (JSON, MP4, AVI, IFC, WAV, BMP, BCF)

### Demonstrator-Relevant Example/Fixture Data

The six apps use **example sessions** (not classic "fixtures") that are loaded at boot via `resolvePlaygroundBoot()` → `activeExampleId`:

#### **Aggregator** (puzzle plugin)
- **Default Example**: `concrete-forest`
- **Path**: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/`
- **Component**: `📚️examples/🌲️concrete-forest/🟦️component.ts`
- **Assets**: DSL/OP/PACK/SPR format files (puzzle3d serialization formats)
  - `🗣️forest.dsl.semio` (domain-specific language)
  - `🔧️forest.op.semio` (operations)
  - `🎒️forest.pack.semio` (packed data)
  - `📡️forest.spr.semio` (sparse representation)

#### **Generator** (procedural3d plugin)
- **Default Example**: `hexagonal-mushroom-column`
- **Path**: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️hexagonal-mushroom-column/`
- **Status**: Example component exists; specific asset files not verified

#### **Koordinator** (block plugin / 3D spatial kernel)
- **Default Example**: `hexagonal-cut-concrete-forest-left`
- **Path**: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️hexagonal-cut-concrete-forest-left/`
- **Component**: `🟦️component.ts`
- **Assets**: DSL format files for 3D block representation

#### **Aussuchen** (GIS gismap plugin)
- **Default Example**: `demo-stock` (NOT FOUND)
- **Available Examples**: Only `🎬️demo-session` found in editor examples
- **Status**: ⚠️ `demo-stock` example definition is missing or undefined in codebase

#### **Bearbeiten** (block 3D plugin)
- **Default Example**: `timber-beam-joinery`
- **Path**: Likely in block plugin examples (not yet located in search)
- **Status**: ⚠️ Example definition not yet confirmed

#### **Verfolgen** (GIS gisterrain plugin)
- **Default Example**: `reuse-map` (NOT FOUND)
- **Available Examples**: Need to verify terrain examples
- **Status**: ⚠️ `reuse-map` example definition is missing or undefined in codebase

---

## 2. Loading Mechanism

### Boot Flow: `resolvePlaygroundBoot()` → Example Resolution

**Framework Entry**: `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:2914`

```typescript
export function resolvePlaygroundBoot(
  catalog: PluginCatalog, 
  variant: string, 
  session?: PlaygroundBootSession
): PlaygroundBoot
```

1. **Demonstrator Landing Page** (`♻️mit-bestand/🧺️demonstrator/📦️index.tsx`)
   - Imports `resolvePlaygroundBoot` from `@semio-tech/framework`
   - Creates boot config for each of six panes using `demonstratorPaneBootVariants(pane.variant)`
     - **Runtime variant** (plugin to load): Maps pane variant → actual plugin (e.g., "generator" → "procedural3d")
     - **Manifest variant**: The app's own manifest ID
   - Each pane gets both `runtimeBoot` and `manifestBoot` memoized

2. **Shell State Initialization** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/`)
   - `resolveShellDefaults()` extracts `exampleId` from brand's `defaults` object
   - `resolveBootExampleId()` resolves which example to activate:
     - Respects locked example IDs (highest priority)
     - Falls back to defaults.exampleId (brand-defined default)
     - Falls back to first available example in registry

3. **Shell Host Runtime** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/`)
   - `activeExampleId` state tracks currently selected example
   - Example dropdown selector allows user to change active example (if not locked)
   - `setActiveExample` action triggers app-level fixture load

4. **Example-to-Session Binding**
   - App manifest declares available examples via registry entry `examples` field
   - Example component exports: `id`, `label` (localized), `icon`, path to fixture files
   - Renderer loads DSL/manifest/session file at the example's `cmdPath`/`dslPath`
   - Fixture file is parsed and mounted into window session state

### Key Source Files
- **Boot resolution**: `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:2891-2950`
- **Example locking/defaulting**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️component.tsx:221-298`
- **Shell state management**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx:5670-6278`
- **Demonstrator pane spec**: `♻️mit-bestand/🧺️demonstrator/🟦️brand.ts:789-796`

---

## 3. App Fixture Status: Real vs. Placeholder

| App | Plugin | Default Example | Fixture Files Exist? | Semantic Content | Status |
|---|---|---|---|---|---|
| **Aggregator** | puzzle/puzzle3d | `concrete-forest` | ✅ Yes | Complex 3D puzzle scene with objects, planes | ✅ Real |
| **Generator** | procedural/procedural3d | `hexagonal-mushroom-column` | 🟡 Partial | Example component exists; asset content unverified | 🟡 Uncertain |
| **Koordinator** | block/block3d | `hexagonal-cut-concrete-forest-left` | ✅ Yes | Block spatial scene (3D objects) | ✅ Real |
| **Aussuchen** | gis/gismap | `demo-stock` | ❌ Missing | Undefined example ID | ❌ Placeholder/Broken |
| **Bearbeiten** | block/block3d | `timber-beam-joinery` | 🟡 Partial | Example component path not yet verified | 🟡 Uncertain |
| **Verfolgen** | gis/gisterrain | `reuse-map` | ❌ Missing | Undefined example ID | ❌ Placeholder/Broken |

---

## 4. Entwerfen-mit-Bestand Domain Data (♻️mit-bestand/)

### Existing Content Structure
- **`recherche/`**: Neo4j research graph database exports and scripts
  - JSON actor network graphs: `actors_graph.json`, `actors_network.json`
  - Python/PowerShell migration/transformation scripts
  - Neo4j schema and relationship queries
  - **Status**: Structured metadata for building components, not directly wired to apps

- **`🖼️asset/`**: Logos and presentation assets
- **`🎤️präsentation/`**: Presentation materials
- **`📋️bericht/`**: Report/documentation
- **`🧺️demonstrator/`**: The six-app demonstrator itself

### Wiring to Apps
- **Not wired**: No direct imports from `recherche/` into demonstrator apps
- **Hardcoded fixtures**: Each app uses example sessions (puzzle, procedural, block) loaded from plugin examples
- **No bestand inventory integration**: The demonstrator does not currently consume `recherche/` data as live inventory

**Implication**: The "Entwerfen-mit-Bestand" reuse-inventory domain data exists separately; demonstrator apps use generic/test examples, not domain-sourced components.

---

## 5. Fixture-Related Tests

### Demonstrator Plugin Tests
**Path**: `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/`
- **File**: `🎯️retained-command-limits.json`
- **Purpose**: Tests command buffer/retention limits in demonstrator context
- **No window-rendering assertions**: Does not test full app-window fixture loading

### Puzzle App Fixture Tests
**Path**: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/`
- Component tests for demo-session example
- No explicit "fixture loads and renders" end-to-end test found

### Shell Rendering Tests
**Path**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts:5961-5983`
- **Test**: `resolveShellDefaults` + `resolveBootExampleId` logic
- **Assertions**: 
  - Verifies default example ID is set correctly from brand
  - Verifies locked example ID overrides defaults
  - ✅ Tests that aggregator defaults to `concrete-forest`
  - ✅ Tests that example ID persists in ephemeral shell state

**Line 5977**: Hardcoded brand for aggregator: `entwerfen-mit-bestand-aggregator` with `defaults: { exampleId: "concrete-forest" }`

**No end-to-end test found** that loads a fixture file and asserts window content renders

---

## 6. Bootstrap Flow Details

### Demonstrator Script (`📜️script.ts:98-112`)
```typescript
async function buildDemonstratorPlugins(): Promise<void> {
  // 1. Build primary variant (koordinator by default, pane[0])
  // 2. Build additional variants (procedural3d for generator)
  // 3. Build engines (WASM per-pane: tiled-map for verfolgen only)
  // 4. Static-serve plugin modules from dist/plugin-modules/
}
```
- **Env**: `SKIP_PLUGIN_BUILD=1` skips plugin rebuild
- **Each pane** independently resolves its boot via `resolvePlaygroundBoot(PLUGIN_CATALOG, bootVariants.runtime)`

### Landing Page Init (`📦️index.tsx:38-41`)
```typescript
const demonstratorStorage = createBrowserStoragePort();
bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance(demonstratorStorage));
initUiLocaleSync(DEMONSTRATOR_LOCALE); // Locks to German
```
- Ephemeral brand (no localStorage persistence per pane)
- Locale locked at boot time to `"de"`

### Pane Component Render (`📦️index.tsx` DemonstratorPane)
```typescript
const runtimeBoot = useMemo(() => 
  resolvePlaygroundBoot(PLUGIN_CATALOG, bootVariants.runtime), 
  [bootVariants.runtime]
);
const manifestBoot = useMemo(() => 
  resolvePlaygroundBoot(PLUGIN_CATALOG, bootVariants.manifest), 
  [bootVariants.manifest]
);
```
- Memoizes both runtime plugin and manifest boot configs
- Passed to `<FrameworkOsShell>` for window rendering

---

## Summary: Fixture Path End-to-End

```
Brand Default (brand.ts)
  ↓
exampleId: "concrete-forest"
  ↓
resolveBootExampleId() [Shell/component.tsx:294]
  ↓
activeExampleId state
  ↓
Plugin Manifest Registry (plugin declares examples[])
  ↓
Example Component (e.g., puzzle/3d/concrete-forest/component.ts)
  ↓
Export: id, label, dslPath/cmdPath/opPath/packPath/sprPath
  ↓
Renderer loads DSL/manifest file from URL
  ↓
Session state initialized with fixture data
  ↓
Window renders with concrete-forest puzzle scene
```

---

## Missing/Broken Fixtures

1. **Aussuchen** (`demo-stock` example) — undefined, falls back to first available or empty state
2. **Verfolgen** (`reuse-map` example) — undefined, falls back to first available or empty state
3. **Bearbeiten** (`timber-beam-joinery` example) — example component path not verified in search

---

## Files Referenced

- Demonstrator app: `♻️mit-bestand/🧺️demonstrator/📦️index.tsx`, `🟦️brand.ts`, `📜️script.ts`
- Framework boot: `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:2891-2950`
- Shell state: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️component.tsx`, `ShellHost/🟦️component.tsx`
- Examples: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/*/📚️examples/*/🟦️component.ts`
- Tests: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts:5961-5983`
