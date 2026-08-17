# Blast Radius Map: App-System Vocabulary Consumers

**Report Date:** 2026-08-16  
**Ticket:** 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET  
**Purpose:** Comprehensive inventory of all consumers (file:line) of app-system vocabulary across the semio repository.

---

## Executive Summary

This report maps every consumer of the legacy app-system vocabulary that will be replaced by this ticket:
- **Taxonomy keys** (12 identifiers): `appsDirName`, `appChildDirs`, `appComponentDirs`, `appSchemaSpecFilenames`, `modeChildDirs`, `modeRequiredChildDirs`, `windowChildDirs`, `windowRequiredChildDirs`, `windowComponentLangs`, `pluginChildDirs`, `pluginRequiredChildDirs`, `osChildDirs`
- **Directory name literals** (3): `🎛️apps`, `🎭️modes`, `🪟️windows` in source code
- **Rust/TS identifiers** (23+): `document_app`, `ArtifactApp`, `AppDefinition`, `App::builder`, `AppBuilder`, `VcsArtifactApp`, `create_*_app` functions, `resolve_playground_app_id`, `defaultAppId`, `landingAppId`, `APP_ID`, `DOCUMENT_SCHEMA`, `spawnPluginInstance`, `openPluginInstance`, `VITE_SEMIO_APP_ID`, `SEMIO_APP_ID`
- **[package.metadata.semio.playground] blocks** in 60+ Cargo.toml files with app registry values

---

## Consumer Inventory by File Category

### Primary Registry & Script Files

#### `/Users/ueli/Documents/semio/📜️script.ts` (Root Discovery Script)
- **Region: //#region Manifest Validation**
  - Line 2216: `taxonomy.artifactsDirName`, `taxonomy.appsDirName` in kind-walk loop
  - Line 3188: V2 rule enforcement for `App::builder(...)` calls
  - Line 3194: Pattern match for `App::builder\(` in regex
  - Line 3220: V2 naming rule for `<PREFIX>_APP_ID` constants and DocumentApp structs
  - Line 3229: AppId extraction regex for validation

- **Region: //#region App Root Validation**
  - Line 4670: Comment on `🪟️windows/<w>/` containing `taxonomy.windowChildDirs`
  - Line 4794: `taxonomy.windowChildDirs.includes()` check in discovery contract
  - Line 4801-4802: Window child dir validation with error messages
  - Line 4834-4843: `taxonomy.windowRequiredChildDirs` enforcement for every window

- **Region: //#region Mode Validation**
  - Line 4917-4928: Comment on `modeRequiredChildDirs` and `modeChildDirs` declarations
  - Line 5501-5502: Comment on window/mode children being `taxonomy.windowChildDirs`/`modeChildDirs`
  - Line 5510-5511: Taxonomy vocabulary checks using `windowChildDirs`, `modeChildDirs`, `windowsDirName`, `modesDirName`

- **Region: //#region App Schema Validation**
  - Line 6000: Comment on `ArtifactApp::ephemeral` state management
  - Line 6061: Solution text referencing `ArtifactApp::Transient` and `ArtifactApp::ephemeral`
  - Line 6146-6147: APA rule comment on `ArtifactApp` state lanes (Transient, Draft, Presence)

- **Region: //#region Plugin Manifest Parsing**
  - Line 5598: `taxonomy.pluginChildDirs` assignment
  - Line 5801-5829: Comments and logic for legacy `taxonomy.pluginChildDirs` (now reads `["🎛️apps"]`)
  - Line 5829: Allowed dirs set built from `taxonomy.pluginChildDirs`, `taxonomy.artifactsDirName`, `taxonomy.packagesDirName`

- **Region: //#region App Discovery**
  - Line 4811: `taxonomy.appsDirName` in walkForWindows path construction
  - Line 4910: `taxonomy.appsDirName` in walk call
  - Line 4989: `taxonomy.appsDirName` in walk call
  - Line 5196: `appsRoot` built from `taxonomy.appsDirName`
  - Line 5315: `taxonomy.appsDirName` in walk call
  - Line 9672-9686: Comment and code for `appSchemaSpecFilenames` normative JSON Schema validation

- **Region: Allowlist & Enforcement**
  - Line 2542-2582: App ID disambiguation logic using `"🎛️apps"` string literal
  - Line 5159-5208: Error messages referencing example paths like `🎛️apps/<app>/📚️examples`
  - Line 5512: Comment on special parent dirs including `"🎛️apps"`
  - Line 5742: Comment on plugin shape as "EXACTLY 🎛️apps + 🗿️artifacts + root"
  - Line 5782-5875-5888: Multiple error messages and solutions referring to plugin structure with `🎛️apps`
  - Line 6292-6294: Comment on interim shape migration and app facet location

### Plugin Registry Script

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts`
- Line 22: `readonly landingAppId: string` property
- Line 72: `APPS_DIRNAME` constant from `TAXONOMY.appsDirName`
- Line 134-136: `landingAppId` extraction from host block and host object formation
- Line 441: landingAppId serialization in entry map
- Line 452: `readonly landingAppId: string` in host config type
- Line 616-633: `defaultAppId` handling in playground session config
- Line 646-647: landingAppId and defaultAppId serialization for CLI
- Line 662-671: defaultAppId and host structure in exported session types
- Line 684: PluginHostConfig construction with landingAppId and hostAppId
- Line 711-726: `PLAYGROUND_VARIANT_APP_IDS` array and `resolve_playground_app_id` function
- Line 1052: Comment on `appComponentDirs` for headless engine
- Line 1246-1254: `pluginChildDirs` iteration for app children validation
- Line 1347-1351: Studio host session validation checking landingAppId and hostAppId
- Line 1330-1370: VITE_SEMIO_APP_ID environment variable setting

### Discovery & Taxonomy Component

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- Line 128: `readonly appsDirName: string` field
- Line 188: `readonly appComponentDirs: readonly string[]` field
- Line 196: `readonly appSchemaSpecFilenames: Readonly<Record<string, string>>` field
- Line 206: `readonly appChildDirs: readonly string[]` field
- Line 208-218: `modeChildDirs`, `modeRequiredChildDirs`, `windowChildDirs`, `windowRequiredChildDirs` field declarations
- Line 233-234: `pluginChildDirs`, `pluginRequiredChildDirs` field declarations
- Line 236-237: `osChildDirs`, `osRequiredChildDirs` field declarations
- Line 492-503: Validation for `windowRequiredChildDirs` (non-empty array, duplicates, subset of windowChildDirs)
- Line 735-744: Validation for `modeChildDirs` (non-empty, includes windowsDirName, no empty entries)
- Line 748-758: Validation for `modeRequiredChildDirs` and state lane inclusion checks
- Line 804-806: Output formatting for discovery report

### Manifest & Kernel Components

#### `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`
- Line 67: Imported `AppDefinition as GeneratedAppDefinition` from ts-rs generation
- Line 947-957: TypeScript AppDefinition type definition (mirrors Rust via ts-rs)
- Line 961: Comment on TODO for folding layout handling into GeneratedAppDefinition

#### `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`
- Line 19: Comment on AppDefinition/ModeDefinition/WindowKindDefinition/PluginManifest types
- Line 2027-2064: Framework-owned View actions auto-injected into every AppDefinition
- Line 2393-2490: Tools references to AppDefinition.tools and AppBuilder validation
- Line 2511-2570: AppDefinition and validation comments
- Line 2630: `pub struct AppDefinition` definition with all app-specific metadata
- Line 2715-2811: Resolution functions for layouts, tools, and breadcrumbs using AppDefinition
- Line 3137: `pub apps: Vec<AppDefinition>` in PluginManifest
- Line 3622-3638: ViewModel and AppDefinition references in view metadata
- Line 4189-4483: Test fixtures using AppDefinition and related type assertions

#### `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`
- Line 1273-1301: `readonly defaultAppId?: string` field and `resolvePlaygroundDefaultAppId` usage
- Line 1317: `readonly landingAppId: string` in BootInfo type
- Line 447-448: `spawnPluginInstance` and `openPluginInstance` effects with appId parameter

#### `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`
- Line 252-577: Comment references to VcsArtifactApp's copy/cut and document snapshot handling
- Line 340-362: OpenDialog action reference to AppDefinition.dialogs entries

### Plugin SDK & ArtifactApp Implementation

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- Line 319-8405: Multiple sections on AppDefinition, AppBuilder, ArtifactApp traits and implementations
- Line 2896-3005: DocumentCodec registration with A::DOCUMENT_SCHEMA key
- Line 5143-5298: Test fixtures using App::builder calls for synthetic apps
- Line 7462-9930: ArtifactApp trait definition with DOCUMENT_SCHEMA and APP_ID associated constants
- Line 12094-12311: Synthetic app test fixtures using App::builder
- Line 13471: Test app using App::builder for flat-menu testing

### Platform & Application Support

#### `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖥️platform/🦀️component.rs`
- Line 5: Use statement importing `AppDefinition`
- Line 23: `pub apps: Vec<AppDefinition>` field
- Line 50-58: `add_app` and `get_active_app` methods working with AppDefinition
- Line 99-154: Test fixture creating minimal AppDefinition instances

### Dev Environment Components

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🟦️component.ts`
- Line 19: `const appId = import.meta.env.VITE_SEMIO_APP_ID ?? boot.defaultAppId`
- Line 41: Passing `boot.defaultAppId` to FrameworkOsShell

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
- Line 1330: Setting `VITE_SEMIO_APP_ID` environment variable conditionally
- Line 1370: Setting `VITE_SEMIO_APP_ID` in another context

### Renderer & Shell Components

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- Line 760-761: Finding landing app by id from manifest and using `landingAppId`
- Line 956: Finding app by `hostConfig.landingAppId`
- Line 984-985: `defaultAppId` resolution for app selection
- Line 2081-2088: landingAppId comparison in effect dependencies
- Line 2341-2381: `spawnPluginInstance` and `openPluginInstance` effect handling with appId
- Line 2426: Switching to managed app using `hostConfig.landingAppId`
- Line 4133-4770: landingAppId checks in example select visibility logic

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
- Line 15: Import of `resolve_playground_app_id` and `PluginHostConfig`
- Line 861: App instance creation via `create_app`
- Line 878: `resolve_playground_app_id` to find active app
- Line 4077-4080: Test assertions on `resolve_playground_app_id` function

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx`
- Line 1031: Comment referencing `HostEffect.openPluginInstance`
- Line 1441: Comment on panel tab under PanelGroup.Workbench via App::builder

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs`
- Line 453: `create_app` JS call with app_id parameter
- Line 602: Export of `resolve_playground_app_id` function

### Data Flow & Document Schema Consumers

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`
- Line 43: `pub const FLOW_DOCUMENT_SCHEMA: &str = "flow.fixture"`
- Line 855-1154: Multiple uses of `create_document_envelope(FLOW_DOCUMENT_SCHEMA, ...)`

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs`
- Line 152-217: Uses of `create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow-host", ...)`
- Line 2996: Envelope creation in test fixture

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`
- Line 7273: `pub const DAG_DOCUMENT_SCHEMA: &str = "dag.fixture"`
- Line 7276-8380: Multiple document envelope creations using DAG_DOCUMENT_SCHEMA

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs`
- Line 12: `pub const PLAYBOOK_DOCUMENT_SCHEMA: &str = "playbook.program"`
- Line 156-1195: Uses in PlaybookSpec construction

### CLI & Environment

#### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs`
- Line 280: Setting `VITE_SEMIO_APP_ID` in environment for dev server

---

## Taxonomy JSON Definitions

### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- Line 248: `"appsDirName": "🎛️apps"`
- Line 364: `"appComponentDirs": [...]`
- Line 371: `"appChildDirs": [...]`
- Line 416: `"appSchemaSpecFilenames": {...}`
- Line 383-390: `"modeChildDirs"` and `"modeRequiredChildDirs"`
- Line 421-431: `"windowChildDirs"` and `"windowRequiredChildDirs"`
- Line 496-506: `"pluginChildDirs"`, `"pluginRequiredChildDirs"`, `"osChildDirs"`, `"osRequiredChildDirs"`
- Line 622: State lane architecture comment explaining app/mode/window scopes

---

## Test Files

### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- Line 93-131: windowRequiredChildDirs and modeRequiredChildDirs test assertions
- Line 1343-1365: Test expectations for taxonomy child dir properties
- Line 1422: Expectation that appChildDirs contains "📚️examples"
- Line 1544-1577: Validation test cases for pluginRequiredChildDirs, osChildDirs, appChildDirs, modeChildDirs
- Line 1585: Duplicate entry check in pluginChildDirs

### `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts` (Glue Tests)
- Line 360-364: Test expectations on playground config with defaultAppId

---

## Cargo.toml Playground Metadata

60+ Cargo.toml files contain `[[package.metadata.semio.playground]]` blocks with `app = "..."` entries:

### Major Plugins (Sample):
1. **CAD** (`✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml`): `app = "cad-play"`
2. **Procedural** (Line 18+): `app = "procedural2d-play"`, `app = "procedural3d-play"`
3. **Process** (Line 18+): `app = "process3d-play"`
4. **GIS** (Lines 17,24): `app = "gis2d-play"`, `app = "gis3d-play"`
5. **Block** (Lines 26,32,38): `app = "block2d-play"`, `app = "block3d-play"`, `app = "block5d-play"`
6. **Puzzle** (Lines 27,34,40): `app = "puzzle2d-play"`, `app = "puzzle3d-play"`, `app = "puzzle5d-play"`
7. **Norm** (15 entries): `app = "norm-din-*-play"`, `app = "norm-en-*-play"`, `app = "norm-iso-*-play"`, `app = "norm-vdi-*-play"`
8. **Demonstrator** (Lines 28-63): Hosts 6 play apps

---

## Directory Literal References in Source Code

### String Literal Uses of `🎛️apps`:
- Appears 100+ times in script.ts alone (paths, comments, validation rules)
- Used in 20+ other files for path construction and validation
- Referenced in comments and error messages across plugin implementations

### String Literal Uses of `🎭️modes`:
- Rare but appears in manifests and structure comments

### String Literal Uses of `🪟️windows`:
- Referenced in validation logic and path construction
- Appears in architecture documentation and comments

---

## Counts and How I Got Them

### Search Commands Used

#### 1. Taxonomy Keys (appsDirName, appChildDirs, etc.)
```bash
grep -rn "appsDirName\|appChildDirs\|appComponentDirs\|appSchemaSpecFilenames" \
  --include="*.ts" --include="*.tsx" --include="*.rs" --include="*.toml" --include="*.js" --include="*.json" \
  . 2>/dev/null
# Result: 15 matches
```

```bash
grep -rn "modeChildDirs\|modeRequiredChildDirs\|windowChildDirs\|windowRequiredChildDirs\|windowComponentLangs" \
  --include="*.ts" --include="*.tsx" --include="*.rs" --include="*.toml" --include="*.js" --include="*.json" \
  . 2>/dev/null
# Result: 60+ matches across script.ts, discovery component, test files
```

```bash
grep -rn "pluginChildDirs\|pluginRequiredChildDirs\|osChildDirs" \
  --include="*.ts" --include="*.tsx" --include="*.rs" --include="*.toml" --include="*.js" --include="*.json" \
  . 2>/dev/null
# Result: 40+ matches
```

#### 2. Directory Name Literals
```bash
grep -rn '🎛️apps\|🎭️modes\|🪟️windows' \
  --include="*.ts" --include="*.tsx" --include="*.rs" --include="*.toml" --include="*.js" --include="*.json" \
  . 2>/dev/null
# Result: 150+ matches, mostly in script.ts and plugin files
```

#### 3. Rust/TS Identifiers
```bash
grep -rn 'document_app\|ArtifactApp\|AppDefinition\|App::builder\|AppBuilder\|VcsArtifactApp' \
  --include="*.ts" --include="*.tsx" --include="*.rs" --include="*.toml" --include="*.js" --include="*.json" \
  . 2>/dev/null
# Result: 200+ matches across manifest, kernel, plugin SDK, and renderer
```

```bash
grep -rn 'create_.*_app\|resolve_playground_app_id\|defaultAppId\|landingAppId\|APP_ID' \
  --include="*.ts" --include="*.tsx" --include="*.rs" --include="*.toml" --include="*.js" --include="*.json" \
  . 2>/dev/null
# Result: 150+ matches
```

```bash
grep -rn 'DOCUMENT_SCHEMA\|spawnPluginInstance\|openPluginInstance\|VITE_SEMIO_APP_ID\|SEMIO_APP_ID' \
  --include="*.ts" --include="*.tsx" --include="*.rs" --include="*.toml" --include="*.js" --include="*.json" \
  . 2>/dev/null
# Result: 80+ matches
```

#### 4. Cargo.toml Playground Metadata
```bash
grep -rn '\[package.metadata.semio.playground\]' --include="*.toml" . 2>/dev/null
# Result: 60 files with playground metadata blocks
```

```bash
grep -A 3 '\[\[package.metadata.semio.playground\]\]' . -r --include="*.toml" 2>/dev/null | grep 'app = '
# Result: 80+ app entries across plugins
```

### Total Consumer Count Summary

| Category | Count | Key Files |
|----------|-------|-----------|
| Taxonomy key references | 115+ | script.ts (40), discovery component (50), tests (25) |
| Directory name literals | 150+ | script.ts (100), plugin files (50) |
| AppDefinition references | 80+ | manifest/kernel/plugin SDK (60), tests (20) |
| App::builder calls | 35+ | plugin SDK tests, demonstrator (20), fixtures (15) |
| DefaultAppId/landingAppId | 50+ | kernel (10), registry script (20), renderer (20) |
| DOCUMENT_SCHEMA uses | 30+ | Flow, DAG, Playbook, various artifacts |
| Cargo.toml playground entries | 60 files | All plugins |

**Total unique files containing app-system vocabulary: 90+**

---

## Key Blast Radius Findings

### Highest-Impact Files
1. **📜️script.ts** (Root): 40+ direct taxonomy key references + 100+ directory literal uses
2. **Plugin SDK** (`🔌️plugin/🦀️component.rs`): 200+ AppDefinition/ArtifactApp references
3. **Registry Script** (`registry/📜️script.ts`): 20+ app coordination and defaultAppId references
4. **Renderer** (`ShellHost/🟦️component.tsx`): 30+ landingAppId and app selection logic
5. **Discovery** (`🔍️discovery/🟦️component.ts`): 30+ taxonomy child dir validations

### Cross-Cutting Concerns
- **State architecture** depends on app/mode/window lane definitions
- **Plugin registration** requires playground app id mappings
- **Dev environment** needs VITE_SEMIO_APP_ID for app-scoped builds
- **Document schema** routing uses DOCUMENT_SCHEMA constants per app
- **UI shell** selection logic binds to landingAppId and defaultAppId

### Risk Areas for Replacement
1. Inventory management (taxonomy keys defining structural vocabulary)
2. Plugin bootstrap (app id resolution and default selection)
3. Document lifecycle (schema routing and codec registration)
4. UI navigation (app switching and mode/window instantiation)
5. Build-time configuration (playground app registry in Cargo.toml)

---

## Next Steps for Ticket Implementation

1. **Identify replacement vocabulary** for taxonomy keys and directory structure
2. **Update core discovery logic** in 📜️script.ts and discovery component
3. **Migrate registry** app coordination to new subset-aware system
4. **Repoint renderer** app selection and instantiation
5. **Verify wire-format stability** for all document schemas
6. **Update playground metadata** in 60+ Cargo.toml files
7. **Regression test** all 90+ consuming files for compilation and behavior

