# Demonstrator Apps End-to-End Manifest Exploration

## Document Structure

This report documents all six demonstrator panes and their app definitions, plugin registrations, and runtime configurations.

---

## 1. Generated Catalog Entries (playgrounds.ts & plugins.ts)

### Demonstrator Plugin Registration
**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts:49`

**Entry**:
- `pluginId`: "demonstrator"
- `cratePath`: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust"
- `wasmOut`: "semio_s_plugin_demonstrator.wasm"
- `role`: "plugin"
- `dependsOn`: ["cad", "gis", "procedural", "process", "puzzle", "sourcing", "stdio"]
- `consumes`: ["forms.questionKind", "flow.extension", "process.machines"]

---

## 2. Six Demonstrator Variants & Playground Registrations

### File: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts`

#### 2.1 Aggregator (puzzle3d editor)
**Line 22**:
- **Variant**: "aggregator"
- **PluginId**: "demonstrator"
- **App**: "s.puzzle.puzzle3d@1/*#editor"
- **Brand**: "entwerfen-mit-bestand-aggregator"
- **CratePath**: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust"
- **Assets**: 
  - mesh-collection: /mesh → ["🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation", "♻️mit-bestand/🖼️asset/🏚️abbau-aufbau"]
  - static-dir: /infinite-fixture → "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures"

#### 2.2 Aussuchen (sourcing editor)
**Line 25**:
- **Variant**: "aussuchen"
- **PluginId**: "demonstrator"
- **App**: "s.sourcing.curate@1/*#editor"
- **Brand**: "entwerfen-mit-bestand-aussuchen"
- **CratePath**: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust"

#### 2.3 Bearbeiten (process3d editor)
**Line 26**:
- **Variant**: "bearbeiten"
- **PluginId**: "demonstrator"
- **App**: "s.process.process3d@1/*#editor"
- **Brand**: "entwerfen-mit-bestand-bearbeiten"
- **CratePath**: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust"

#### 2.4 Generator (procedural3d editor)
**Line 52**:
- **Variant**: "generator"
- **PluginId**: "demonstrator"
- **App**: "s.procedural.procedural3d@1/*#editor"
- **Brand**: "entwerfen-mit-bestand-generator"
- **CratePath**: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust"

#### 2.5 Koordinator (cad editor)
**Line 57**:
- **Variant**: "koordinator"
- **PluginId**: "demonstrator"
- **App**: "s.cad.cad@1/*#editor"
- **Brand**: "entwerfen-mit-bestand-koordinator"
- **CratePath**: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust"
- **Assets**: 
  - static-dir: /cad-fixture → "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧫️fixtures"

#### 2.6 Verfolgen (gis2d editor)
**Line 80**:
- **Variant**: "verfolgen"
- **PluginId**: "demonstrator"
- **App**: "s.gis.gismap@1/*#editor"
- **Brand**: "entwerfen-mit-bestand-verfolgen"
- **CratePath**: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust"
- **Assets**: 
  - tile-proxy: /osm → OpenStreetMap (cache: osm-tiles)
  - tile-proxy: /vt → OpenFreeMap (cache: openfreemap-vt)

---

## 3. App Definitions & Window/Panel/Command Mappings

### File: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs`

#### 3.1 Demonstrator Plugin Bundle Structure
**Lines 28-41**: Defines the closed runtime app fleet (`DemonstratorApps` enum):

```
Registered apps in order:
1. PlaygroundEditor + PlaygroundViewer (demonstrator's own surfaces)
2. Procedural3dEditor (procedural3d runtime variant)
3. CadEditor (cad)
4. Puzzle3dEditor (puzzle3d)
5. SourcingEditor + SourcingViewer (sourcing: dual surfaces)
6. ProcessEditor + ProcessViewer (process3d: dual surfaces)
7. GisEditor (gis2d)
```

#### 3.2 Test Evidence: Window Registry
**Lines 187-193** demonstrate window declarations for each foreign app:

**Procedural3d (Generator)**:
- App: "s.procedural.procedural3d@1/*#editor"
- Windows: 
  - procedural.play.main
  - procedural.play.preview
  - procedural.play.generations
  - procedural.play.generate-form
  - procedural.play.generate-preview

**Cad (Koordinator)**:
- App: "s.cad.cad@1/*#editor"
- Windows:
  - cad.play.shape
  - cad.play.building
  - cad.play.energy
  - cad.play.structure-classic

**Puzzle3d (Aggregator)**:
- App: "s.puzzle.puzzle3d@1/*#editor"
- Windows:
  - puzzle3d.play.composite

**Sourcing (Aussuchen)**:
- App: "s.sourcing.curate@1/*#editor"
- Windows:
  - sourcing.pool
  - sourcing.curated
  - sourcing.preview
  - sourcing.grid

**Process3d (Bearbeiten)**:
- App: "s.process.process3d@1/*#editor"
- Windows:
  - process.play.main

**Gis2d (Verfolgen)**:
- App: "s.gis.gismap@1/*#editor"
- Windows:
  - gis2d.play.composite

---

## 4. Puzzle3d App Manifest Details (Example)

### File: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`

#### 4.1 App Definition Builder (Lines 6766-7025)

**Document Type**: ["semio", "puzzle", "3d"]

**Artifact Kind**: puzzle3d

**Mode**: 
- Default mode: "puzzle3d-play-edit"
- Definition: `edit::definition()`
- Default layout: `edit::layout()`

**Window Kind**:
- Window kind definition: `main::definition()` (line 6777)
- Window ID: "puzzle3d-main"
- Interactions: vortex domain (line 6779)

**Panel Tabs** (lines 6781-6784):
- document::definition()
- catalogue::definition()
- inspection::definition()
- settings::definition()

**Utilities** (lines 6866-6875):
- transform
- brush
- volume_brush
- world_relocate

**Tools** (lines 6878-6879):
- fill (mode-level tool)

**Key Bindings** (lines 6785-6791):
- escape → engagementAbort
- delete → deleteSelection
- backspace → deleteSelection
- mod+d → duplicateSelection
- tab → cycleBrushCandidate
- shift+tab → cycleBrushCandidateBack
- f → focusSelection

**Commands** (sample from lines 6793-6858):
- **Mutations** (document mutations):
  - setFixtureJson
  - setActiveExample
  - addObjectKind
  - deleteSelection
  - duplicateSelection
  - translateSelection
  - rotateSelection
  - scaleSelection
  - transformEnd
  - worldRelocate
  - setSelectionFlag
  - patchInspector
  - engagementSubmit
  - engagementRepeatLast
  - createAttraction
  - deleteAttraction
  - addTargetVolume
  - deleteTargetVolume
  - setTargetVolumeFlag
  - addBrushObject
  - setFillCount
  - acceptSuggestion

- **View Actions** (ephemeral state):
  - setCamera
  - setLocale
  - setTerminology
  - setProjection
  - focusSelection
  - selectSameKindSelection
  - setVortexShow
  - setVortexDirection
  - toggleSun
  - setSunAzimuth
  - setSunElevation
  - setSunIntensity
  - setLodAutomatic
  - setLodDepthVariable
  - setGridVisible
  - setLodManual
  - setGridSnapEnabled
  - setGridSpacing
  - setProximityRadius
  - setChunkSize
  - setSelectableKind
  - engagementInput
  - engagementAbort
  - engagementControlSelect
  - setTransformGumballFlag
  - transformBegin
  - setVoxelDims
  - relocateTargetVolume
  - setBrushPlacementOverlapBudget
  - setObjectKindWeight
  - setVortexKindWeight
  - cycleBrushCandidate
  - cycleBrushCandidateBack
  - openVortexSuggestions
  - closeVortexSuggestions
  - hoverSuggestion
  - suggestionsTick
  - fillBuildTick
  - registerBrushMesh
  - worldPointerDown

- **Shell Actions** (lines 6817):
  - openAddObjectDialog

**Introduction** (lines 6882-6940):
- Title: "Welcome to Aggregator"
- Steps: welcome, viewport, catalogue, add-object, transform-utility

**Dialogs** (lines 6943-6958):
- "addObject" dialog with objectKind select field

#### 4.2 Window Binding Pattern

From line 6777, window content binding is declared via:
```
.window_kind_def(main::definition(&envelope, &Puzzle3dLabels::NATIVE_EN))
```

This establishes the window kind definition for the main 3D view. Panel binding happens via:
```
.panel_tab_def(document::definition())
.panel_tab_def(catalogue::definition())
```

Window rendering is controlled by the app's mode (line 6775):
```
.default_mode_id(edit::PUZZLE3D_PLAY_MODE_EDIT)
```

---

## 5. Window Content Binding Mechanism

### General Framework Pattern

All app manifests follow this pattern:

1. **Mode Definition** (`edit::definition()`):
   - Defines which windows/panels are visible in this mode
   - Specifies mode-level tools

2. **Window Kind Definition** (`main::definition()`):
   - Declares surfaces and rendering components
   - Specifies window-level utilities and interactions

3. **Panel Tab Definition** (`catalogue::definition()`, etc.):
   - Declares panel content and interactions
   - Panels are attached via `.panel_tab_def()`

4. **Default Layout** (`edit::layout()`):
   - Specifies which windows/panels open by default
   - Controls initial pane arrangement

### Default Windows Opening

The default layout declares which windows open on app startup. For Puzzle3d:
- Main window (puzzle3d-main) opens automatically
- Panel tabs are accessible via tabs but not all open initially
- User can toggle panels via tab clicks

---

## 6. Missing/Stubbed/Placeholder Manifests Check

### Status: All Six Apps Have Complete Manifests

✅ **Aggregator** (puzzle3d):
- App ID: "s.puzzle.puzzle3d@1/*#editor"
- Status: Fully defined with manifest
- File: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:6766`

✅ **Generator** (procedural3d):
- App ID: "s.procedural.procedural3d@1/*#editor"
- Status: Registered in demonstrator bundle, fully defined

✅ **Koordinator** (cad):
- App ID: "s.cad.cad@1/*#editor"
- Status: Registered in demonstrator bundle, fully defined

✅ **Aussuchen** (sourcing):
- App ID: "s.sourcing.curate@1/*#editor"
- Status: Registered in demonstrator bundle with dual editor+viewer surfaces

✅ **Bearbeiten** (process3d):
- App ID: "s.process.process3d@1/*#editor"
- Status: Registered in demonstrator bundle with dual editor+viewer surfaces

✅ **Verfolgen** (gis2d):
- App ID: "s.gis.gismap@1/*#editor"
- Status: Registered in demonstrator bundle, fully defined

---

## 7. Plugin Crate Backing Each App

All six demonstrator panes use the same demonstrator plugin crate but surface different apps from foreign plugins:

### Demonstrator Plugin Crate
- **Package Name**: semio-s-plugin-demonstrator
- **Cargo.toml Path**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml`
- **Source**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust`

### Foreign Plugin Dependencies (transitive via demonstrator)

1. **Procedural (Generator)**
   - Package: semio-s-plugin-procedural
   - Path: `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust`

2. **Cad (Koordinator)**
   - Package: semio-s-plugin-cad
   - Path: `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust`

3. **Puzzle (Aggregator)**
   - Package: semio-s-plugin-puzzle
   - Path: `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust`

4. **Sourcing (Aussuchen)**
   - Package: semio-s-plugin-sourcing
   - Path: `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust`

5. **Process (Bearbeiten)**
   - Package: semio-s-plugin-process
   - Path: `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust`

6. **Gis (Verfolgen)**
   - Package: semio-s-plugin-gis
   - Path: `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust`

### Demonstrator Plugin Dependencies (plugins.ts:49)

Direct dependencies declared:
```
dependsOn: ["cad", "gis", "procedural", "process", "puzzle", "sourcing", "stdio"]
```

---

## 8. Brand to App Mapping

### Shell Brands (brand.ts:30-37)

All six demonstrator apps have corresponding shell brands defined in:
`/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🟦️brand.ts`

**Brand IDs** (lines 30-36):
1. "entwerfen-mit-bestand-aggregator" → Aggregator brand
2. "entwerfen-mit-bestand-aussuchen" → Aussuchen brand
3. "entwerfen-mit-bestand-bearbeiten" → Bearbeiten brand
4. "entwerfen-mit-bestand-generator" → Generator brand
5. "entwerfen-mit-bestand-koordinator" → Koordinator brand
6. "entwerfen-mit-bestand-verfolgen" → Verfolgen brand

Each brand includes:
- Logo (shared: ENTWERFEN_MIT_BESTAND_LOGO_SVG)
- Locale lock (de)
- Terminology lock (reuse)
- Theme ID (semio)
- Default example ID
- Ephemeral flag (true)
- Asset directory
- Introduction steps
- Optional tutorial (Aggregator only: ENTWERFEN_MIT_BESTAND_TUTORIAL)

---

## 9. Pane Configuration (brand.ts:789-796)

### DEMONSTRATOR_PANES Array

Each pane specification includes:
- `id`: pane identifier (e.g., "generator")
- `variant`: runtime variant (e.g., "generator" → runtime "procedural3d")
- `brand`: corresponding ShellBrand
- `label`: display label (e.g., "Generator")
- `tagline`: short description (e.g., "Parametrische Abläufe")
- `icon`: icon name (e.g., "workflow")

**Configuration**:
- Grid order: row-major (0-2 top row, 3-5 bottom row)
- Boot scheduler: `scheduleDemonstratorIdle()` paces warm boots

---

## 10. Summary Table

| Variant | Brand ID | App ID | Plugin | Cargo Crate | Runtime App | Windows |
|---------|----------|--------|--------|-------------|-------------|---------|
| aggregator | entwerfen-mit-bestand-aggregator | s.puzzle.puzzle3d@1/*#editor | demonstrator | semio-s-plugin-puzzle | puzzle3d | puzzle3d.play.composite |
| aussuchen | entwerfen-mit-bestand-aussuchen | s.sourcing.curate@1/*#editor | demonstrator | semio-s-plugin-sourcing | sourcing | sourcing.pool, sourcing.curated, sourcing.preview, sourcing.grid |
| bearbeiten | entwerfen-mit-bestand-bearbeiten | s.process.process3d@1/*#editor | demonstrator | semio-s-plugin-process | process3d | process.play.main |
| generator | entwerfen-mit-bestand-generator | s.procedural.procedural3d@1/*#editor | demonstrator | semio-s-plugin-procedural | procedural3d | procedural.play.main, procedural.play.preview, procedural.play.generations, procedural.play.generate-form, procedural.play.generate-preview |
| koordinator | entwerfen-mit-bestand-koordinator | s.cad.cad@1/*#editor | demonstrator | semio-s-plugin-cad | cad | cad.play.shape, cad.play.building, cad.play.energy, cad.play.structure-classic |
| verfolgen | entwerfen-mit-bestand-verfolgen | s.gis.gismap@1/*#editor | demonstrator | semio-s-plugin-gis | gis2d | gis2d.play.composite |

---

## Key File References

1. **Brand definitions**: `/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🟦️brand.ts`
2. **Playgrounds registry**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts`
3. **Plugins registry**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts`
4. **Demonstrator manifest**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs`
5. **Puzzle3d app definition**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`

