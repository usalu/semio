# 💯 Requirements

## [👤semio📚engine](semiorepo://p/u/semio/b/l/engine)

## Engine

Engine startup MUST support a dev/debug mode flag that waits for debugger attachment before runtime begins.

Engine startup MUST support a pure stdio MCP server mode.

## [👤semio📚js🗃️sketchpad](semiorepo://p/u/semio/b/l/js/f/sketchpad)

## State Management

App hover and selection state MUST be managed by the Sketchpad state machine.

## Toolbar

The toolbar is a floating panel positioned at the bottom center of the canvas. Each app registers toolbar sections.

- **Home app**: Filter toggles for kit kinds (temporary, local, remote) with action buttons to create new kits
- **Kit app**: Filter toggles for artifact kinds (designs, types, qualities, ports, tags, concepts, files, folders, authors) with action buttons to create new artifacts
- **Design app**: Selection tools (normal, additive, subtractive) and lasso tools (rectangular, freeform)
- **Type app**: Selection tools and connector creation tool
- **Feedback app**: Send button to submit feedback form

Toolbar panel visibility defaults to true for all apps in default state creation.

## Interaction State

Hover and selection feedback across Home, Kit, Design, Type, Quality, Docs, and Feedback is driven by the app state machine.

Hover and selection highlights MUST be consistent across tables, lists, and diagrams.

## Borders

- Element border kind (hover color)
- Window border kind (normal border color)
- Window spacing: 1-unit gap between windows and 1-unit margin to canvas edge
- Base canvas uses the base background surface; windows, panels, and temporary UI surfaces use their respective background levels
- Exactly one window is active in a multi-window layout; the active window surface uses an active background tint
- Table views use the active window surface background
- Global Sketchpad shell is wrapped in base level so Navbar/Footer resolve base background
- Panels are rendered under panel level so panel surfaces resolve panel background
- Window chrome controls MUST be rendered as Action UI elements
- Window frames use inset overlay strokes so all four edges remain visible with clipped layouts

## Windows

Sketchpad apps MUST render inside a multi-window workspace.

Each app MUST define a set of window kinds and a default window layout.

Window layouts MUST be persisted per app as JSON strings.

The active window MUST be tracked for focus-sensitive UI.

Window chrome MUST expose action controls for open-in-new-window, maximize/minimize, and close.

## [👤semio📚js🗃️sketchpad💻kittsx](semiorepo://section/semio/js/sketchpad/Kit.tsx)

Selection: - Designs - Types - Folders - Files - Ports

Filters: - Designs - Types - Folders - Files - Ports

## [👤semio📚js🗃️sketchpad💻kittsx🔖table](semiorepo://section/semio/js/sketchpad/Kit.tsx/Internal%20State%20Management/Canvas)

- {{design-row}}
- {{type-row}}
- {{port-row}}
- {{folder-row*}} # folder
  - {{file-row}} # files inside the folder

Currently I have

- file1
- file2

But it should be:

- folder
  - file1
  - file2

## [👤semio📚js🗃️sketchpad💻designtsx🔖panels](semiorepo://section/semio/js/sketchpad/Design.tsx/Panels/Details)

Home screen:

Once Piece, Once Connection selected:

- {{piece-details}}
- {{connection-details}}
- {{kit-details}}

Once Piece selected:

- {{piece-details}}
- {{kit-details}}

Piece Details Section:

```yaml
Piece: # section,
 Type: "{{piece-type-select}}" # input tree item, only show types that can replaced the type (e.g. all used connectors must exist)
 Id: "{{piece-id-input}}" # input tree item
 Description: "{{piece-description-text-area}}" # input tree item
 Attributes:
  - name: "{{attribute-name-input}}" # input tree item
    value: "{{attribute-value-input}}" # input tree item
 Plane: # collection tree item, only show section when
  Origin: # collection tree item
   X: "{{origin-x-stepper}}" # input tree item
   Y: "{{origin-y-stepper}}" # input tree item
   Z: "{{origin-z-stepper}}" # input tree item
  X-Axis:
   X: "{{x-axis-x-stepper}}"
   Y: "{{x-axis-y-stepper}}"
   Z: "{{x-axis-z-stepper}}"
  Y-Axis:
   X: "{{y-axis-x-stepper}}"
   Y: "{{y-axis-y-stepper}}"
   Z: "{{y-axis-z-stepper}}"
Parent Connection:
 Translation:
  Gap: "{{gap-slider}}"
  Shift: "{{shift-slider}}"
  Rise: "{{rise-slider}}"
 Orientation:
  Rotation: "{{rotation-slider}}"
  Inversion: "{{inversion-slider}}"
```

Kit Details Section

```yaml
Kit: # section,
 Name: "{{kit-name}}"
```

## [👤semio🏪assets🗃️grasshopper💻build🔖build](semiorepo://p/u/semio/b/a/assets/fd/org/grasshopper/f/build.py/s/Build)

Grasshopper XML parsing and JSON export MUST extract components and groups.

## [👤semio🏪assets💻icons🔖exports](semiorepo://p/u/semio/b/a/assets/f/icons.ts/s/Exports)

Exports MUST map each Lucide icon to a domain-specific alias name.

## [👤semio🏪assets💻index🔖exports](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports)

Re-exports and data constants MUST come from the Metabolism kit assets.

## [👤semio🏪assets🛅logo💻logo🔖imports](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Imports)

MUST import Node.js file system, DOM parsing, and path resolution modules.

## [👤semio🏪assets🛅logo💻logo🔖types](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Types)

Types MUST provide the types functionality.

## [👤semio🏪assets🛅logo💻logo🔖logogeneration](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Logo%20Generation)

Logo Generation MUST provide the logo generation functionality.

## [👤semio🏪assets🛅logo💻logo🔖parsesvg](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Parse%20SVG)

MUST read SVG content and extract all group transforms and path attributes.

## [👤semio🏪assets🛅logo💻logo🔖generatekeyframesequence](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Generate%20Keyframe%20Sequence)

MUST produce forward and reverse sequence for smooth animation looping.

## [👤semio🏪assets🛅logo💻logo🔖createanimatedsvg](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Create%20Animated%20SVG)

MUST generate translate, rotate, scale, fill, stroke, and stroke-width animations.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixable🔖missingend](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixable.tsx/s/MissingEnd)

MissingEnd MUST provide the missingend functionality.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixable🔖alpha](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixable.tsx/s/Alpha)

Alpha MUST provide the alpha functionality.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixableexpected🔖missingend](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixable_expected.tsx/s/MissingEnd)

MissingEnd MUST provide the missingend functionality.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixableexpected🔖alpha](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixable_expected.tsx/s/Alpha)

Alpha MUST provide the alpha functionality.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🛠️fixedclass](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.cs/d/i/FixedClass)

/ FixedClass MUST have a Value property.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖package](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.go/s/Package)

Package MUST be named fixed.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖functions](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.go/s/Functions)

Functions MUST return valid integers.
FixedValue MUST return 2.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🛠️fixedvalue](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.go/d/i/FixedValue)

FixedValue MUST return 2.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖functions](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.py/s/Functions)

Functions MUST accept typed parameters.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖types](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.tsx/s/Types)

Types MUST be exported when used externally.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖components](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.tsx/s/Components)

Components MUST accept FixedType props.

## [👤semio🖱️desktop💻forgeenvd🔖electronfuses](semiorepo://p/u/semio/b/u/desktop/f/forge.env.d.ts/s/Electron%20Fuses)

Consumers MUST use these enums for configuring fuse settings.

## [👤semio🖱️desktop💻main🔖mainprocess](semiorepo://p/u/semio/b/u/desktop/f/main.ts/s/Main%20Process)

MUST quit on all windows closed except on macOS.

## [👤semio🖱️desktop💻preload🔖preload](semiorepo://p/u/semio/b/u/desktop/f/preload.ts/s/Preload)

Preload MUST use contextBridge to safely expose IPC methods.

## [👤semio🖱️desktop💻renderer🔖renderer](semiorepo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer)

MUST resolve the user identity before rendering the sketchpad.

## [👤semio🌐docs💻index🔖entrypoint](semiorepo://p/u/semio/b/w/docs/f/index.tsx/s/Entrypoint)

Entrypoint MUST render into the root element defined in the docs index.html.

## [👤semio📚engine💻build🔖build](semiorepo://p/u/semio/b/l/engine/f/build.ts/s/Build)

Build script for the engine binary. MUST bundle the engine via PyInstaller.

## [👤semio📚engine💻engine🔖imports](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Imports)

Imports MUST include all dependencies for store, assistant, GraphQL, REST, MCP, and engine modules.

## [👤semio📚engine💻engine🔖store](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store)

Store MUST provide the data access layer for kit operations via code-based routing.

## [👤semio📚engine💻engine🔖assistant](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Assistant)

Assistant MUST provide AI-powered design prediction using OpenAI structured outputs.

## [👤semio📚engine💻engine🔖graphql](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Graphql)

Graphql MUST map semio domain types to Graphene schema nodes for query and mutation.

## [👤semio📚engine💻engine🔖rest](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest)

Rest MUST expose kit, type, design, and assistant endpoints via FastAPI.

## [👤semio📚engine💻engine🔖mcp](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp)

Mcp MUST expose kit, type, design, validation, and diff tools via Model Context Protocol.

## [👤semio📚engine💻engine🔖engine](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Engine)

Engine MUST mount REST, GraphQL, and MCP sub-applications and manage the server lifecycle.

## [👤semio📚engine💻generateschemas🔖schemageneration](semiorepo://p/u/semio/b/l/engine/f/generate-schemas.ts/s/Schema%20Generation)

Schema generation script. MUST invoke the Python engine schema generator.

## [👤semio📚engine💻postbuild🔖postbuild](semiorepo://p/u/semio/b/l/engine/f/post-build.ts/s/Post%20Build)

Post-build script. MUST relocate the PyInstaller output to the Grasshopper bin folder.

## [👤semio📚engine💻sqliteschema🔖schemaexport](semiorepo://p/u/semio/b/l/engine/f/sqliteschema.ts/s/Schema%20Export)

SQLite schema export script. MUST dump the database schema to a SQL file.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️goo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/Goo)

/ Implementations MUST override CastFrom and CastTo for type conversion.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️param](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/Param)

/ Implementations MUST provide component exposure and icon metadata.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️enumgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EnumGoo)

/ Implementations MUST convert between string names and enum values.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️enumparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EnumParam)

/ Implementations MUST restrict input to valid enum members.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️passthroughcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/PassthroughComponent)

/ Implementations MUST transform input data and output the result.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️idgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/IdGoo)

/ Implementations MUST wrap entity ID types for Grasshopper data flow.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️idparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/IdParam)

/ Implementations MUST provide type-safe parameter access for IDs.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️idcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/IdComponent)

/ Implementations MUST register input parameters matching ID fields.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️diffgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DiffGoo)

/ Implementations MUST wrap entity diff types for Grasshopper data flow.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️diffparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DiffParam)

/ Implementations MUST provide type-safe parameter access for diffs.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️diffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DiffComponent)

/ Implementations MUST register input parameters matching diff fields.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️serializecomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/SerializeComponent)

/ Implementations MUST convert entities to valid JSON strings.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️deserializecomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DeserializeComponent)

/ Implementations MUST parse JSON strings into entity instances.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️serializediffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/SerializeDiffComponent)

/ Implementations MUST convert diffs to valid JSON strings.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️deserializediffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DeserializeDiffComponent)

/ Implementations MUST parse JSON strings into diff instances.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️serializeidcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/SerializeIdComponent)

/ Implementations MUST convert entity IDs to valid JSON strings.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️deserializeidcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DeserializeIdComponent)

/ Implementations MUST parse JSON strings into entity ID instances.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entitygoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityGoo)

/ Implementations MUST validate entities before exposing them downstream.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entityparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityParam)

/ Implementations MUST enforce entity validation on parameter access.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entitycomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityComponent)

/ Implementations MUST validate constructed entities before output.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entityidgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityIdGoo)

/ Implementations MUST validate entity IDs before exposing them downstream.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entityidparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityIdParam)

/ Implementations MUST enforce entity ID validation on parameter access.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entityidcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityIdComponent)

/ Implementations MUST validate constructed entity IDs before output.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entitydiffgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityDiffGoo)

/ Implementations MUST validate entity diffs before exposing them downstream.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entitydiffparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityDiffParam)

/ Implementations MUST enforce entity diff validation on parameter access.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entitydiffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityDiffComponent)

/ Implementations MUST validate constructed entity diffs before output.

## [👤semio📚gh🛅semiograsshopper💻buildvaluelists🔖valuelistgeneration](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build-value-lists.ts/s/Value%20List%20Generation)

Value list generation script. MUST convert CSV data into Grasshopper value list text files.

## [👤semio📚gh🛅semiograsshopper💻build🔖build](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build.ts/s/Build)

Grasshopper build script. MUST compile the solution and copy artifacts to the Yak distribution folder.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻build🔖build](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/build.ts/s/Build)

Yak package build script. MUST prepare the distribution folder and build the .yak package.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻login🔖login](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/login.ts/s/Login)

Yak login script. MUST authenticate with the Yak package manager.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publish🔖publish](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/publish.ts/s/Publish)

Yak publish script. MUST push the built package to the Yak server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testpush🔖testpush](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/test-push.ts/s/Test%20Push)

Yak test push script. MUST push the package to the test Yak server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testsearch🔖script](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/test-search.ts/s/Script)

Script MUST execute yak search against the test.yak.rhino3d.com server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻unyank🔖unyank](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/unyank.ts/s/Unyank)

Yak unyank script. MUST restore a previously yanked package version.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻yank🔖yank](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/yank.ts/s/Yank)

Yak yank script. MUST remove a package version from the Yak server.

## [👤semio📚go💻kitsqlite🔖sqlitekitoperations](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations)

SQLite kit operations. MUST provide serialization and deserialization of Kit to and from SQLite and zip formats.

## [👤semio📚go💻kitsqlite🛠️kitfromsqlite](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/KitFromSqlite)

Callers MUST provide a valid path to an existing SQLite database

## [👤semio📚go💻kitsqlite🛠️loadtypes](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/loadTypes)

Callers MUST provide a valid open database connection and kit GUID

## [👤semio📚go💻kitsqlite🛠️loaddesigns](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/loadDesigns)

Callers MUST provide a valid open database connection and kit GUID

## [👤semio📚go💻kitsqlite🛠️loadpieces](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/loadPieces)

Callers MUST provide a valid open database connection and design GUID

## [👤semio📚go💻kitsqlite🛠️loadconnections](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/loadConnections)

Callers MUST provide a valid open database connection and design GUID

## [👤semio📚go💻kitsqlite🛠️loadconnectors](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/loadConnectors)

Callers MUST provide a valid open database connection and type GUID

## [👤semio📚go💻kitsqlite🛠️kittosqlite](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/KitToSqlite)

Callers MUST provide a valid Kit, writable database path, and schema SQL

## [👤semio📚go💻kitsqlite🛠️kitfromzip](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/KitFromZip)

Callers MUST provide a valid path to an existing zip file containing kit.db

## [👤semio📚go💻kitsqlite🛠️kittozip](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/KitToZip)

Callers MUST provide a valid Kit, file map, writable zip path, and schema SQL

## [👤semio📚go💻semio🔖imports](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Imports)

Imports MUST include all required packages for the semio domain library.

## [👤semio📚go💻semio🔖constants](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Constants)

Constants MUST define shared constant values for the semio domain.

## [👤semio📚go💻semio🔖utils](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Utils)

Utils MUST provide general-purpose utility functions for the semio domain.
Guid MUST return a cryptographically random 128-bit hex string.

## [👤semio📚go💻semio🔖entityids](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs)

Entity IDs MUST define identifier types for all semio domain entities.

## [👤semio📚go💻semio🔖weakentities](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Weak%20Entities)

Weak Entities MUST define value types that exist only as part of parent entities.

## [👤semio📚go💻semio🔖attribute](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Attribute)

Attribute MUST define the key-value metadata entity and its diff types.

## [👤semio📚go💻semio🔖location](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Location)

Location MUST define geographic location entities and their diff types.

## [👤semio📚go💻semio🔖author](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Author)

Author MUST define authorship entities and their diff types.

## [👤semio📚go💻semio🔖file](semiorepo://p/u/semio/b/l/go/f/semio.go/s/File)

File MUST define file reference entities and their diff types.

## [👤semio📚go💻semio🔖folder](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Folder)

Folder MUST define folder hierarchy entities and their diff types.

## [👤semio📚go💻semio🔖benchmark](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Benchmark)

Benchmark MUST define benchmark threshold entities and their diff types.

## [👤semio📚go💻semio🔖quality](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Quality)

Quality MUST define measurable quality entities and their diff types.

## [👤semio📚go💻semio🔖port](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Port)

Port MUST define connector port entities and their diff types.

## [👤semio📚go💻semio🔖prop](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Prop)

Prop MUST define property value entities and their diff types.

## [👤semio📚go💻semio🔖tag](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Tag)

Tag MUST define tag classification entities and their diff types.

## [👤semio📚go💻semio🔖concept](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Concept)

Concept MUST define concept categorization entities and their diff types.

## [👤semio📚go💻semio🔖model](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Model)

Model MUST define 3D model reference entities and their diff types.

## [👤semio📚go💻semio🔖connector](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Connector)

Connector MUST define spatial connector entities and their diff types.

## [👤semio📚go💻semio🔖type](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Type)

Type MUST define component type entities and their diff types.

## [👤semio📚go💻semio🔖layer](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Layer)

Layer MUST define layer hierarchy entities and their diff types.

## [👤semio📚go💻semio🔖piece](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Piece)

Piece MUST define placed piece entities and their diff types.

## [👤semio📚go💻semio🔖group](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Group)

Group MUST define piece grouping entities and their diff types.

## [👤semio📚go💻semio🔖side](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Side)

Side MUST define connection side reference entities and their diff types.

## [👤semio📚go💻semio🔖connection](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Connection)

Connection MUST define spatial connection entities and their diff types.

## [👤semio📚go💻semio🔖stat](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Stat)

Stat MUST define statistical measure entities and their diff types.

## [👤semio📚go💻semio🔖design](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Design)

Design MUST define assembly design entities and their diff types.

## [👤semio📚go💻semio🔖kit](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Kit)

Kit MUST define the root kit container entity and its diff types.

## [👤semio📚go💻semio🔖serialization](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Serialization)

Serialization MUST provide JSON marshaling and unmarshaling for kit data.
SerializeKit MUST return valid JSON with two-space indentation.

## [👤semio📚go💻semio🔖helpers](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Helpers)

Helpers MUST provide lookup functions for finding entities within kits.
FindTypeInKit MUST return nil when no type matches the GUID.

## [👤semio📚go💻semio🔖factories](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Factories)

Factories MUST provide constructor functions for creating new domain entities.
NewKit MUST generate a unique GUID and set version to 0.0.1.

## [👤semio📚go💻semio🔖kitoperations](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations)

Kit Operations MUST provide comparison, diffing, and application of kit changes.
AreKitsEqual MUST compare all entities by GUID and structural fields.

## [👤semio📚go💻semio🔖kitdiffhelpers](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Diff%20Helpers)

Kit Diff Helpers MUST provide convenience functions for single-entity kit diffs.
AddTypeToKit MUST return a diff with exactly one added type.

## [👤semio📚go💻semio🔖validation](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Validation)

Validation MUST provide constraint-based validation of kit data integrity.

## [👤semio📚go💻semio🔖validationserialization](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Validation%20Serialization)

Validation Serialization MUST provide serializable representations of validation results.

## [👤semio📚go💻semio🔖flattendesign](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design)

Flatten Design MUST compute absolute piece planes from relative connections.
planeToMatrix MUST perform the planeToMatrix operation.

## [👤semio📚go💻semio🛠️guid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/Guid)

Guid MUST return a cryptographically random 128-bit hex string.

## [👤semio📚go💻semio🛠️normalize](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/Normalize)

Normalize MUST trim whitespace and convert to lowercase.

## [👤semio📚go💻semio🛠️round](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/Round)

Round MUST return the value rounded to exactly the given decimal places.

## [👤semio📚go💻semio🛠️deepequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/DeepEqual)

DeepEqual MUST return true only when both values produce identical JSON.

## [👤semio📚go💻semio🛠️unmarshaljson](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/UnmarshalJSON)

UnmarshalJSON MUST populate the setFields map for all present JSON keys.

## [👤semio📚go💻semio🛠️hasfield](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/HasField)

HasField MUST return false when setFields is nil.

## [👤semio📚go💻semio🛠️unmarshaljson](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/UnmarshalJSON)

UnmarshalJSON MUST populate the setFields map for all present JSON keys.

## [👤semio📚go💻semio🛠️hasfield](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/HasField)

HasField MUST return false when setFields is nil.

## [👤semio📚go💻semio🛠️unmarshaljson](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/UnmarshalJSON)

UnmarshalJSON MUST populate the setFields map for all present JSON keys.

## [👤semio📚go💻semio🛠️hasfield](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/HasField)

HasField MUST return false when setFields is nil.

## [👤semio📚go💻semio🛠️unmarshaljson](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/UnmarshalJSON)

UnmarshalJSON MUST populate the setFields map for all present JSON keys.

## [👤semio📚go💻semio🛠️hasfield](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/HasField)

HasField MUST return false when setFields is nil.

## [👤semio📚go💻semio🛠️serializekit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/SerializeKit)

SerializeKit MUST return valid JSON with two-space indentation.

## [👤semio📚go💻semio🛠️deserializekit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/DeserializeKit)

DeserializeKit MUST return an error if the data is not valid kit JSON.

## [👤semio📚go💻semio🛠️serializekitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/SerializeKitDiff)

SerializeKitDiff MUST return valid JSON with two-space indentation.

## [👤semio📚go💻semio🛠️deserializekitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/DeserializeKitDiff)

DeserializeKitDiff MUST return an error if the data is not valid kit diff JSON.

## [👤semio📚go💻semio🛠️findtypeinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindTypeInKit)

FindTypeInKit MUST return nil when no type matches the GUID.

## [👤semio📚go💻semio🛠️finddesigninkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindDesignInKit)

FindDesignInKit MUST return nil when no design matches the GUID.

## [👤semio📚go💻semio🛠️findpieceindesign](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindPieceInDesign)

FindPieceInDesign MUST return nil when no piece matches the GUID.

## [👤semio📚go💻semio🛠️findconnectionindesign](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindConnectionInDesign)

FindConnectionInDesign MUST return nil when no connection matches the GUID.

## [👤semio📚go💻semio🛠️findconnectorintype](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindConnectorInType)

FindConnectorInType MUST return nil when no connector matches the GUID.

## [👤semio📚go💻semio🛠️findfileinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindFileInKit)

FindFileInKit MUST return nil when no file matches the GUID.

## [👤semio📚go💻semio🛠️findfolderinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindFolderInKit)

FindFolderInKit MUST return nil when no folder matches the GUID.

## [👤semio📚go💻semio🛠️findqualityinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindQualityInKit)

FindQualityInKit MUST return nil when no quality matches the GUID.

## [👤semio📚go💻semio🛠️findportinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindPortInKit)

FindPortInKit MUST return nil when no port matches the GUID.

## [👤semio📚go💻semio🛠️findtaginkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindTagInKit)

FindTagInKit MUST return nil when no tag matches the GUID.

## [👤semio📚go💻semio🛠️findconceptinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindConceptInKit)

FindConceptInKit MUST return nil when no concept matches the GUID.

## [👤semio📚go💻semio🛠️findauthorinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindAuthorInKit)

FindAuthorInKit MUST return nil when no author matches the GUID.

## [👤semio📚go💻semio🛠️newkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewKit)

NewKit MUST generate a unique GUID and set version to 0.0.1.

## [👤semio📚go💻semio🛠️newtype](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewType)

NewType MUST generate a unique GUID for the new type.

## [👤semio📚go💻semio🛠️newdesign](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewDesign)

NewDesign MUST generate a unique GUID for the new design.

## [👤semio📚go💻semio🛠️newpiece](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewPiece)

NewPiece MUST generate a unique GUID for the new piece.

## [👤semio📚go💻semio🛠️newconnection](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewConnection)

NewConnection MUST generate a unique GUID and set both connected and connecting sides.

## [👤semio📚go💻semio🛠️newconnector](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewConnector)

NewConnector MUST generate a unique GUID for the new connector.

## [👤semio📚go💻semio🛠️newfile](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewFile)

NewFile MUST generate a unique GUID for the new file.

## [👤semio📚go💻semio🛠️newfolder](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewFolder)

NewFolder MUST generate a unique GUID for the new folder.

## [👤semio📚go💻semio🛠️newquality](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewQuality)

NewQuality MUST generate a unique GUID for the new quality.

## [👤semio📚go💻semio🛠️newport](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewPort)

NewPort MUST generate a unique GUID for the new port.

## [👤semio📚go💻semio🛠️newtag](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewTag)

NewTag MUST generate a unique GUID for the new tag.

## [👤semio📚go💻semio🛠️newconcept](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewConcept)

NewConcept MUST generate a unique GUID for the new concept.

## [👤semio📚go💻semio🛠️newauthor](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewAuthor)

NewAuthor MUST generate a unique GUID for the new author.

## [👤semio📚go💻semio🛠️arekitsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AreKitsEqual)

AreKitsEqual MUST compare all entities by GUID and structural fields.

## [👤semio📚go💻semio🛠️arekitdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AreKitDiffsEqual)

AreKitDiffsEqual MUST compare all diff fields including nested entity diffs.

## [👤semio📚go💻semio🛠️aretypesdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areTypesDiffsEqual)

areTypesDiffsEqual MUST perform the areTypesDiffsEqual operation.

## [👤semio📚go💻semio🛠️aredesignsdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areDesignsDiffsEqual)

areDesignsDiffsEqual MUST perform the areDesignsDiffsEqual operation.

## [👤semio📚go💻semio🛠️aretagsdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areTagsDiffsEqual)

areTagsDiffsEqual MUST perform the areTagsDiffsEqual operation.

## [👤semio📚go💻semio🛠️areconceptsdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areConceptsDiffsEqual)

areConceptsDiffsEqual MUST perform the areConceptsDiffsEqual operation.

## [👤semio📚go💻semio🛠️areportsdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/arePortsDiffsEqual)

arePortsDiffsEqual MUST perform the arePortsDiffsEqual operation.

## [👤semio📚go💻semio🛠️arefilesdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areFilesDiffsEqual)

areFilesDiffsEqual MUST perform the areFilesDiffsEqual operation.

## [👤semio📚go💻semio🛠️arefoldersdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areFoldersDiffsEqual)

areFoldersDiffsEqual MUST perform the areFoldersDiffsEqual operation.

## [👤semio📚go💻semio🛠️areauthorsdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areAuthorsDiffsEqual)

areAuthorsDiffsEqual MUST perform the areAuthorsDiffsEqual operation.

## [👤semio📚go💻semio🛠️getkitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/GetKitDiff)

GetKitDiff MUST return a diff that when applied to before produces after.

## [👤semio📚go💻semio🛠️gettypesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getTypesDiff)

getTypesDiff MUST perform the getTypesDiff operation.

## [👤semio📚go💻semio🛠️gettypediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getTypeDiff)

getTypeDiff MUST perform the getTypeDiff operation.

## [👤semio📚go💻semio🛠️istypediffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isTypeDiffEmpty)

isTypeDiffEmpty MUST perform the isTypeDiffEmpty operation.

## [👤semio📚go💻semio🛠️getdesignsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getDesignsDiff)

getDesignsDiff MUST perform the getDesignsDiff operation.

## [👤semio📚go💻semio🛠️getdesigndiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getDesignDiff)

getDesignDiff MUST perform the getDesignDiff operation.

## [👤semio📚go💻semio🛠️isdesigndiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isDesignDiffEmpty)

isDesignDiffEmpty MUST perform the isDesignDiffEmpty operation.

## [👤semio📚go💻semio🛠️gettagsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getTagsDiff)

getTagsDiff MUST perform the getTagsDiff operation.

## [👤semio📚go💻semio🛠️gettagdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getTagDiff)

getTagDiff MUST perform the getTagDiff operation.

## [👤semio📚go💻semio🛠️istagdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isTagDiffEmpty)

isTagDiffEmpty MUST perform the isTagDiffEmpty operation.

## [👤semio📚go💻semio🛠️getconceptsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getConceptsDiff)

getConceptsDiff MUST perform the getConceptsDiff operation.

## [👤semio📚go💻semio🛠️getconceptdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getConceptDiff)

getConceptDiff MUST perform the getConceptDiff operation.

## [👤semio📚go💻semio🛠️isconceptdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isConceptDiffEmpty)

isConceptDiffEmpty MUST perform the isConceptDiffEmpty operation.

## [👤semio📚go💻semio🛠️getportsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getPortsDiff)

getPortsDiff MUST perform the getPortsDiff operation.

## [👤semio📚go💻semio🛠️getportdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getPortDiff)

getPortDiff MUST perform the getPortDiff operation.

## [👤semio📚go💻semio🛠️isportdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isPortDiffEmpty)

isPortDiffEmpty MUST perform the isPortDiffEmpty operation.

## [👤semio📚go💻semio🛠️getfilesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getFilesDiff)

getFilesDiff MUST perform the getFilesDiff operation.

## [👤semio📚go💻semio🛠️getfilediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getFileDiff)

getFileDiff MUST perform the getFileDiff operation.

## [👤semio📚go💻semio🛠️isfilediffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isFileDiffEmpty)

isFileDiffEmpty MUST perform the isFileDiffEmpty operation.

## [👤semio📚go💻semio🛠️getfoldersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getFoldersDiff)

getFoldersDiff MUST perform the getFoldersDiff operation.

## [👤semio📚go💻semio🛠️getfolderdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getFolderDiff)

getFolderDiff MUST perform the getFolderDiff operation.

## [👤semio📚go💻semio🛠️isfolderdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isFolderDiffEmpty)

isFolderDiffEmpty MUST perform the isFolderDiffEmpty operation.

## [👤semio📚go💻semio🛠️getauthorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getAuthorsDiff)

getAuthorsDiff MUST perform the getAuthorsDiff operation.

## [👤semio📚go💻semio🛠️getauthordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getAuthorDiff)

getAuthorDiff MUST perform the getAuthorDiff operation.

## [👤semio📚go💻semio🛠️isauthordiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isAuthorDiffEmpty)

isAuthorDiffEmpty MUST perform the isAuthorDiffEmpty operation.

## [👤semio📚go💻semio🛠️inversekitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/InverseKitDiff)

InverseKitDiff MUST return a diff that when applied restores the original state.

## [👤semio📚go💻semio🛠️inversetypesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseTypesDiff)

inverseTypesDiff MUST perform the inverseTypesDiff operation.

## [👤semio📚go💻semio🛠️inversetypediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseTypeDiff)

inverseTypeDiff MUST perform the inverseTypeDiff operation.

## [👤semio📚go💻semio🛠️inversedesignsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseDesignsDiff)

inverseDesignsDiff MUST perform the inverseDesignsDiff operation.

## [👤semio📚go💻semio🛠️inversedesigndiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseDesignDiff)

inverseDesignDiff MUST perform the inverseDesignDiff operation.

## [👤semio📚go💻semio🛠️inversetagsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseTagsDiff)

inverseTagsDiff MUST perform the inverseTagsDiff operation.

## [👤semio📚go💻semio🛠️inversetagdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseTagDiff)

inverseTagDiff MUST perform the inverseTagDiff operation.

## [👤semio📚go💻semio🛠️inverseconceptsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseConceptsDiff)

inverseConceptsDiff MUST perform the inverseConceptsDiff operation.

## [👤semio📚go💻semio🛠️inverseconceptdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseConceptDiff)

inverseConceptDiff MUST perform the inverseConceptDiff operation.

## [👤semio📚go💻semio🛠️inverseportsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inversePortsDiff)

inversePortsDiff MUST perform the inversePortsDiff operation.

## [👤semio📚go💻semio🛠️inverseportdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inversePortDiff)

inversePortDiff MUST perform the inversePortDiff operation.

## [👤semio📚go💻semio🛠️inversefilesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseFilesDiff)

inverseFilesDiff MUST perform the inverseFilesDiff operation.

## [👤semio📚go💻semio🛠️inversefilediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseFileDiff)

inverseFileDiff MUST perform the inverseFileDiff operation.

## [👤semio📚go💻semio🛠️inversefoldersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseFoldersDiff)

inverseFoldersDiff MUST perform the inverseFoldersDiff operation.

## [👤semio📚go💻semio🛠️inversefolderdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseFolderDiff)

inverseFolderDiff MUST perform the inverseFolderDiff operation.

## [👤semio📚go💻semio🛠️inverseauthorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseAuthorsDiff)

inverseAuthorsDiff MUST perform the inverseAuthorsDiff operation.

## [👤semio📚go💻semio🛠️inverseauthordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseAuthorDiff)

inverseAuthorDiff MUST perform the inverseAuthorDiff operation.

## [👤semio📚go💻semio🛠️normalizestr](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/normalizeStr)

normalizeStr MUST perform the normalizeStr operation.

## [👤semio📚go💻semio🛠️aretypesequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areTypesEqual)

areTypesEqual MUST perform the areTypesEqual operation.

## [👤semio📚go💻semio🛠️areconnectorsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areConnectorsEqual)

areConnectorsEqual MUST perform the areConnectorsEqual operation.

## [👤semio📚go💻semio🛠️aremodelsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areModelsEqual)

areModelsEqual MUST perform the areModelsEqual operation.

## [👤semio📚go💻semio🛠️aredesignsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areDesignsEqual)

areDesignsEqual MUST perform the areDesignsEqual operation.

## [👤semio📚go💻semio🛠️arepiecesequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/arePiecesEqual)

arePiecesEqual MUST perform the arePiecesEqual operation.

## [👤semio📚go💻semio🛠️areconnectionsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areConnectionsEqual)

areConnectionsEqual MUST perform the areConnectionsEqual operation.

## [👤semio📚go💻semio🛠️aretagsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areTagsEqual)

areTagsEqual MUST perform the areTagsEqual operation.

## [👤semio📚go💻semio🛠️areconceptsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areConceptsEqual)

areConceptsEqual MUST perform the areConceptsEqual operation.

## [👤semio📚go💻semio🛠️areportsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/arePortsEqual)

arePortsEqual MUST perform the arePortsEqual operation.

## [👤semio📚go💻semio🛠️arefilesequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areFilesEqual)

areFilesEqual MUST perform the areFilesEqual operation.

## [👤semio📚go💻semio🛠️arefoldersequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areFoldersEqual)

areFoldersEqual MUST perform the areFoldersEqual operation.

## [👤semio📚go💻semio🛠️areauthorsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areAuthorsEqual)

areAuthorsEqual MUST perform the areAuthorsEqual operation.

## [👤semio📚go💻semio🛠️applykitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ApplyKitDiff)

ApplyKitDiff MUST apply all additions, removals and updates from the diff.

## [👤semio📚go💻semio🛠️applytypesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyTypesDiff)

applyTypesDiff MUST perform the applyTypesDiff operation.

## [👤semio📚go💻semio🛠️applytypediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyTypeDiff)

applyTypeDiff MUST perform the applyTypeDiff operation.

## [👤semio📚go💻semio🛠️applyconnectorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConnectorsDiff)

applyConnectorsDiff MUST perform the applyConnectorsDiff operation.

## [👤semio📚go💻semio🛠️applyconnectordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConnectorDiff)

applyConnectorDiff MUST perform the applyConnectorDiff operation.

## [👤semio📚go💻semio🛠️applymodelsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyModelsDiff)

applyModelsDiff MUST perform the applyModelsDiff operation.

## [👤semio📚go💻semio🛠️applymodeldiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyModelDiff)

applyModelDiff MUST perform the applyModelDiff operation.

## [👤semio📚go💻semio🛠️applydesignsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyDesignsDiff)

applyDesignsDiff MUST perform the applyDesignsDiff operation.

## [👤semio📚go💻semio🛠️applydesigndiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyDesignDiff)

applyDesignDiff MUST perform the applyDesignDiff operation.

## [👤semio📚go💻semio🛠️applypiecesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyPiecesDiff)

applyPiecesDiff MUST perform the applyPiecesDiff operation.

## [👤semio📚go💻semio🛠️applypiecediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyPieceDiff)

applyPieceDiff MUST perform the applyPieceDiff operation.

## [👤semio📚go💻semio🛠️applyconnectionsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConnectionsDiff)

applyConnectionsDiff MUST perform the applyConnectionsDiff operation.

## [👤semio📚go💻semio🛠️applyconnectiondiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConnectionDiff)

applyConnectionDiff MUST perform the applyConnectionDiff operation.

## [👤semio📚go💻semio🛠️applytagsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyTagsDiff)

applyTagsDiff MUST perform the applyTagsDiff operation.

## [👤semio📚go💻semio🛠️applytagdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyTagDiff)

applyTagDiff MUST perform the applyTagDiff operation.

## [👤semio📚go💻semio🛠️applyconceptsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConceptsDiff)

applyConceptsDiff MUST perform the applyConceptsDiff operation.

## [👤semio📚go💻semio🛠️applyconceptdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConceptDiff)

applyConceptDiff MUST perform the applyConceptDiff operation.

## [👤semio📚go💻semio🛠️applyportsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyPortsDiff)

applyPortsDiff MUST perform the applyPortsDiff operation.

## [👤semio📚go💻semio🛠️applyportdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyPortDiff)

applyPortDiff MUST perform the applyPortDiff operation.

## [👤semio📚go💻semio🛠️applyfilesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyFilesDiff)

applyFilesDiff MUST perform the applyFilesDiff operation.

## [👤semio📚go💻semio🛠️applyfilediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyFileDiff)

applyFileDiff MUST perform the applyFileDiff operation.

## [👤semio📚go💻semio🛠️applyfoldersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyFoldersDiff)

applyFoldersDiff MUST perform the applyFoldersDiff operation.

## [👤semio📚go💻semio🛠️applyfolderdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyFolderDiff)

applyFolderDiff MUST perform the applyFolderDiff operation.

## [👤semio📚go💻semio🛠️applyauthorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyAuthorsDiff)

applyAuthorsDiff MUST perform the applyAuthorsDiff operation.

## [👤semio📚go💻semio🛠️applyauthordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyAuthorDiff)

applyAuthorDiff MUST perform the applyAuthorDiff operation.

## [👤semio📚go💻semio🛠️filterdesignswithoutparent](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FilterDesignsWithoutParent)

FilterDesignsWithoutParent MUST exclude all designs that have a non-nil parent.

## [👤semio📚go💻semio🛠️addtypetokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddTypeToKit)

AddTypeToKit MUST return a diff with exactly one added type.

## [👤semio📚go💻semio🛠️removetypefromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemoveTypeFromKit)

RemoveTypeFromKit MUST return a diff with exactly one removed type ID.

## [👤semio📚go💻semio🛠️adddesigntokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddDesignToKit)

AddDesignToKit MUST return a diff with exactly one added design.

## [👤semio📚go💻semio🛠️removedesignfromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemoveDesignFromKit)

RemoveDesignFromKit MUST return a diff with exactly one removed design ID.

## [👤semio📚go💻semio🛠️addfiletokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddFileToKit)

AddFileToKit MUST return a diff with exactly one added file.

## [👤semio📚go💻semio🛠️removefilefromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemoveFileFromKit)

RemoveFileFromKit MUST return a diff with exactly one removed file ID.

## [👤semio📚go💻semio🛠️addporttokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddPortToKit)

AddPortToKit MUST return a diff with exactly one added port.

## [👤semio📚go💻semio🛠️removeportfromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemovePortFromKit)

RemovePortFromKit MUST return a diff with exactly one removed port ID.

## [👤semio📚go💻semio🛠️addtagtokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddTagToKit)

AddTagToKit MUST return a diff with exactly one added tag.

## [👤semio📚go💻semio🛠️removetagfromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemoveTagFromKit)

RemoveTagFromKit MUST return a diff with exactly one removed tag ID.

## [👤semio📚go💻semio🛠️addconcepttokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddConceptToKit)

AddConceptToKit MUST return a diff with exactly one added concept.

## [👤semio📚go💻semio🛠️removeconceptfromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemoveConceptFromKit)

RemoveConceptFromKit MUST return a diff with exactly one removed concept ID.

## [👤semio📚go💻semio🛠️buildvalidationcontext](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/buildValidationContext)

buildValidationContext MUST perform the buildValidationContext operation.

## [👤semio📚go💻semio🛠️generateuniquename](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/generateUniqueName)

generateUniqueName MUST perform the generateUniqueName operation.

## [👤semio📚go💻semio🛠️makefix](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/makeFix)

makeFix MUST perform the makeFix operation.

## [👤semio📚go💻semio🛠️guiduniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/GuidUniquenessConstraint)

GuidUniquenessConstraint MUST report each duplicate GUID as a separate problem.

## [👤semio📚go💻semio🛠️updateguideverywhere](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/updateGuidEverywhere)

updateGuidEverywhere MUST perform the updateGuidEverywhere operation.

## [👤semio📚go💻semio🛠️typenameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/TypeNameUniquenessConstraint)

TypeNameUniquenessConstraint MUST report duplicate names among types with the same parent.

## [👤semio📚go💻semio🛠️designnameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/DesignNameUniquenessConstraint)

DesignNameUniquenessConstraint MUST report duplicate names among designs with the same parent.

## [👤semio📚go💻semio🛠️piecenameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/PieceNameUniquenessConstraint)

PieceNameUniquenessConstraint MUST report duplicate piece names within each design.

## [👤semio📚go💻semio🛠️qualitynameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/QualityNameUniquenessConstraint)

QualityNameUniquenessConstraint MUST report each duplicate quality name.

## [👤semio📚go💻semio🛠️portnameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/PortNameUniquenessConstraint)

PortNameUniquenessConstraint MUST report each duplicate port name.

## [👤semio📚go💻semio🛠️filenameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FileNameUniquenessConstraint)

FileNameUniquenessConstraint MUST report each duplicate file name.

## [👤semio📚go💻semio🛠️foldernameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FolderNameUniquenessConstraint)

FolderNameUniquenessConstraint MUST report duplicate names among folders with the same parent.

## [👤semio📚go💻semio🛠️connectornameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ConnectorNameUniquenessConstraint)

ConnectorNameUniquenessConstraint MUST report duplicate connector names within each type.

## [👤semio📚go💻semio🛠️modelnameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ModelNameUniquenessConstraint)

ModelNameUniquenessConstraint MUST report duplicate model names within each type.

## [👤semio📚go💻semio🛠️layerpathuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/LayerPathUniquenessConstraint)

LayerPathUniquenessConstraint MUST report duplicate layer paths within each design.

## [👤semio📚go💻semio🛠️validatekit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ValidateKit)

ValidateKit MUST apply all default constraints and return all found problems.

## [👤semio📚go💻semio🛠️validatekitwithconstraints](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ValidateKitWithConstraints)

ValidateKitWithConstraints MUST apply each constraint and aggregate all problems.

## [👤semio📚go💻semio🛠️haserrors](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/HasErrors)

HasErrors MUST return true when any problem has error severity or empty severity.

## [👤semio📚go💻semio🛠️tovalidationresult](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ToValidationResult)

ToValidationResult MUST default empty severity to error.

## [👤semio📚go💻semio🛠️arevalidationresultsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AreValidationResultsEqual)

AreValidationResultsEqual MUST compare problems regardless of their order.

## [👤semio📚go💻semio🛠️planetomatrix](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/planeToMatrix)

planeToMatrix MUST perform the planeToMatrix operation.

## [👤semio📚go💻semio🛠️matrixtoplane](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/matrixToPlane)

matrixToPlane MUST perform the matrixToPlane operation.

## [👤semio📚go💻semio🛠️cross](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/cross)

cross MUST perform the cross operation.

## [👤semio📚go💻semio🛠️normalize](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/normalize)

normalize MUST perform the normalize operation.

## [👤semio📚go💻semio🛠️dot](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/dot)

dot MUST perform the dot operation.

## [👤semio📚go💻semio🛠️veclength](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/vecLength)

vecLength MUST perform the vecLength operation.

## [👤semio📚go💻semio🛠️degtorad](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/degToRad)

degToRad MUST perform the degToRad operation.

## [👤semio📚go💻semio🛠️roundfloat](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/roundFloat)

roundFloat MUST perform the roundFloat operation.

## [👤semio📚go💻semio🛠️roundplane](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/roundPlane)

roundPlane MUST perform the roundPlane operation.

## [👤semio📚go💻semio🛠️makerotationaxis](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/makeRotationAxis)

makeRotationAxis MUST perform the makeRotationAxis operation.

## [👤semio📚go💻semio🛠️maketranslation](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/makeTranslation)

makeTranslation MUST perform the makeTranslation operation.

## [👤semio📚go💻semio🛠️quaternionfromaxisangle](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/quaternionFromAxisAngle)

quaternionFromAxisAngle MUST perform the quaternionFromAxisAngle operation.

## [👤semio📚go💻semio🛠️quaternionfromunitvectors](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/quaternionFromUnitVectors)

quaternionFromUnitVectors MUST perform the quaternionFromUnitVectors operation.

## [👤semio📚go💻semio🛠️quaterniontomatrix](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/quaternionToMatrix)

quaternionToMatrix MUST perform the quaternionToMatrix operation.

## [👤semio📚go💻semio🛠️multiplymatrices](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/multiplyMatrices)

multiplyMatrices MUST perform the multiplyMatrices operation.

## [👤semio📚go💻semio🛠️applymatrix4tovec3](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyMatrix4ToVec3)

applyMatrix4ToVec3 MUST perform the applyMatrix4ToVec3 operation.

## [👤semio📚go💻semio🛠️computechildplane](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/computeChildPlane)

computeChildPlane MUST perform the computeChildPlane operation.

## [👤semio📚go💻semio🛠️getconnector](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getConnector)

getConnector MUST perform the getConnector operation.

## [👤semio📚go💻semio🛠️flattendesign](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FlattenDesign)

FlattenDesign MUST traverse the connection graph via BFS to compute piece transforms.

## [👤semio📚go💻semio🛠️planesequalapprox](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/planesEqualApprox)

planesEqualApprox MUST perform the planesEqualApprox operation.

## [👤semio📚go💻semio🛠️applydesigndiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ApplyDesignDiff)

ApplyDesignDiff MUST apply all piece, connection and property changes from the diff.

## [👤semio📚js💻dev🔖dev](semiorepo://p/u/semio/b/l/js/f/dev.ts/s/Dev)

MUST kill both child processes on SIGINT and SIGTERM.

## [👤semio📚js💻i18n🔖i18n](semiorepo://p/u/semio/b/l/js/f/i18n.ts/s/I18n)

MUST fall back to English when the detected language is unavailable.

## [👤semio📚js💻index🔖exports](semiorepo://p/u/semio/b/l/js/f/index.ts/s/Exports)

MUST re-export all public types alongside their runtime counterparts.

## [👤semio📚js💻semio🔖imports](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Imports)

External dependency imports MUST be declared here.

## [👤semio📚js💻semio🔖constants](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constants)

Global constants MUST define shared numeric parameters.

## [👤semio📚js💻semio🔖utilities](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Utilities)

General-purpose utility functions MUST be defined here.

## [👤semio📚js💻semio🔖entityids](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Entity%20IDs)

Entity identifier types and comparison functions MUST be defined here.

## [👤semio📚js💻semio🔖attribute](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Attribute)

Attribute entity types, schemas, and helper functions MUST be defined here.

## [👤semio📚js💻semio🔖coordweakentity](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Coord%20(weak%20entity))

Coord weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semio🔖vecweakentity](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Vec%20(weak%20entity))

Vec weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semio🔖pointweakentity](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Point%20(weak%20entity))

Point weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semio🔖vectorweakentity](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Vector%20(weak%20entity))

Vector weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semio🔖planeweakentity](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Plane%20(weak%20entity))

Plane weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semio🔖cameraweakentity](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Camera%20(weak%20entity))

Camera weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semio🔖location](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Location)

Location entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖author](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Author)

Author entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖file](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/File)

File entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖folder](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Folder)

Folder entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖benchmark](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Benchmark)

Benchmark entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖quality](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Quality)

Quality entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖port](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Port)

Port entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖prop](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Prop)

Prop entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖tag](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Tag)

Tag entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖concept](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Concept)

Concept entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖model](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Model)

Model entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖connector](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Connector)

Connector entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖type](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Type)

Type entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖layer](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Layer)

Layer entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖piece](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Piece)

Piece entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖group](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Group)

Group entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖side](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Side)

Side entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖connection](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Connection)

Connection entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖stat](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Stat)

Stat entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖design](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Design)

Design entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖kit](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Kit)

Kit entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semio🔖designfamilyhelpers](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Design%20Family%20Helpers)

Design family traversal helpers MUST be defined here.

## [👤semio📚js💻semio🔖typefamilyhelpers](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Type%20Family%20Helpers)

Type family traversal helpers MUST be defined here.

## [👤semio📚js💻semio🔖filetreeutilities](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/File%20Tree%20Utilities)

File tree construction and traversal utilities MUST be defined here.

## [👤semio📚js💻semio🔖kitimportexport](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Kit%20Import/Export)

Kit serialization and deserialization functions MUST be defined here.

## [👤semio📚js💻semio🔖validation](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Validation)

Kit validation engine and constraints MUST be defined here.
Core validation types and interfaces MUST be defined here.

## [👤semio📚js💻semio🔖validationcoretypes](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Validation%20core%20types)

Core validation types and interfaces MUST be defined here.

## [👤semio📚js💻semio🔖validationcontextengine](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Validation%20context%20&%20engine)

Validation context construction and engine MUST be defined here.

## [👤semio📚js💻semio🔖fixhelper](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Fix%20helper)

Validation fix helper functions MUST be defined here.

## [👤semio📚js💻semio🔖guidupdatehelper](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/GUID%20update%20helper)

GUID regeneration helper functions MUST be defined here.

## [👤semio📚js💻semio🔖constraintguiduniqueness](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20GUID%20uniqueness)

GUID uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constrainttypenameuniqueness](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20Type%20name%20uniqueness)

Type name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintdesignnameuniqueness](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20Design%20name%20uniqueness)

Design name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintpiecenameuniqueness](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20Piece%20name%20uniqueness)

Piece name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintqualitynameuniqueness](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20Quality%20name%20uniqueness)

Quality name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintportnameuniqueness](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20Port%20name%20uniqueness)

Port name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintfilenameuniqueness](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20File%20name%20uniqueness)

File name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintfoldernameuniqueness](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20Folder%20name%20uniqueness)

Folder name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintconnectornameuniquenesswithintype](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20Connector%20name%20uniqueness%20within%20type)

Connector name uniqueness within type constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintmodelnameuniquenesswithintype](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20Model%20name%20uniqueness%20within%20type)

Model name uniqueness within type constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintlayerpathuniquenesswithindesign](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20Layer%20path%20uniqueness%20within%20design)

Layer path uniqueness within design constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintdesignpiecesamefamilyconstraint](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint:%20Design%20piece%20same%20family%20constraint)

Design piece same family constraint MUST be enforced here.

## [👤semio📚js💻semio🔖constraintregistration](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Constraint%20registration)

Constraint registration and default configurations MUST be defined here.

## [👤semio📚js💻semio🔖validationserialization](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Validation%20serialization)

Validation result serialization and deserialization MUST be defined here.

## [👤semio📚js💻site🔖entrypoint](semiorepo://p/u/semio/b/l/js/f/site.tsx/s/Entrypoint)

Entrypoint MUST render into the root element defined in index.html.

## [👤semio📚js🗃️sketchpad💻design🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports)

Imports for Design app MUST include all shared sketchpad, React, and UI dependencies.

## [👤semio📚js🗃️sketchpad💻design🔖statemanagement](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/State%20Management)

State management types and interfaces MUST define the Design app selection, presence, hover, diff, and state shape.

## [👤semio📚js🗃️sketchpad💻design🔖commands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Commands)

Commands MUST define all executable Design app actions dispatched by keyboard shortcuts and UI interactions.

## [👤semio📚js🗃️sketchpad💻design🔖store](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store)

Store MUST implement DesignStore extending PlainKitDiffAppStore with undo/redo, selection diff inversion, and state persistence.

## [👤semio📚js🗃️sketchpad💻design🔖designapppluginregistration](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Design%20App%20Plugin%20Registration)

Design app plugin registration MUST register the Design app plugin with machine actions, guards, and default state.

## [👤semio📚js🗃️sketchpad💻design🔖hooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Hooks)

Hooks MUST provide the Design app initialization lifecycle within the React component tree.

## [👤semio📚js🗃️sketchpad💻design🔖components](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Components)

Components MUST provide Design app scope, actor context, and synchronization wrapper components.

## [👤semio📚js🗃️sketchpad💻design🔖actionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Action%20Hooks)

Action hooks MUST provide composable React hooks for Design app selection, hover, focus, panel, and transaction actions.

## [👤semio📚js🗃️sketchpad💻design🔖footer](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Footer)

Footer MUST render dynamic Design app footer items showing selection and transaction state.

## [👤semio📚js🗃️sketchpad💻design🔖tools](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Tools)

Tools MUST define all Design app tool configurations for selection, lasso, and hand modes.

## [👤semio📚js🗃️sketchpad💻design🔖panels](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels)

WindowLibrary MUST provide draggable window templates for adding scene, diagram, and table windows.

## [👤semio📚js🗃️sketchpad💻design🔖windowlibrary](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/WindowLibrary)

WindowLibrary MUST provide draggable window templates for adding scene, diagram, and table windows.

## [👤semio📚js🗃️sketchpad💻design🔖details](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Details)

Details MUST render the Design app detail panels for design, pieces, connections, and connector sections.

## [👤semio📚js🗃️sketchpad💻design🔖canvas](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas)

Hover Intent Context MUST manage debounced hover state to prevent flickering during rapid mouse movement.

## [👤semio📚js🗃️sketchpad💻design🔖hoverintentcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Hover%20Intent%20Context)

Hover Intent Context MUST manage debounced hover state to prevent flickering during rapid mouse movement.

## [👤semio📚js🗃️sketchpad💻design🔖diagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Diagram)

Diagram MUST render the interactive React Flow design diagram with nodes, edges, minimap, and controls.

## [👤semio📚js🗃️sketchpad💻design🔖scene](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Scene)

Scene MUST render the Three.js 3D scene view of design pieces with selection and hover highlighting.

## [👤semio📚js🗃️sketchpad💻design🔖windows](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Windows)

Window components MUST wrap diagram and scene views with hover and transaction providers.

## [👤semio📚js🗃️sketchpad💻design🔖app](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/App)

App MUST compose all Design app panels, canvas, toolbar, and footer into the main Design app layout.

## [👤semio📚js🗃️sketchpad💻design🔖settings](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Settings)

Settings MUST render the Design app settings panel with theme, language, device, expertise, and mode toggles.

## [👤semio📚js🗃️sketchpad💻design🔖config](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Config)

Config MUST export the Design app configuration with route segments, panel definitions, and path matching.

## [👤semio📚js🗃️sketchpad💻docs🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Imports)

External and internal module imports MUST be declared here.

## [👤semio📚js🗃️sketchpad💻docs🔖mdxloader](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/MDX%20Loader)

MDX file loading and section discovery utilities MUST be declared here.

## [👤semio📚js🗃️sketchpad💻docs🔖sectiontree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/SectionTree)

Section tree navigation component MUST render docs file hierarchy.

## [👤semio📚js🗃️sketchpad💻docs🔖registry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Registry)

Docs registry MUST provide page and section lookup for navigation.

## [👤semio📚js🗃️sketchpad💻docs🔖store](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Store)

Docs app section state MUST be declared here.

## [👤semio📚js🗃️sketchpad💻docs🔖types](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Types)

Docs app state, selection, and diff type definitions MUST be declared here.

## [👤semio📚js🗃️sketchpad💻docs🔖docsappstore](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Docs%20App%20Store)

Docs app store MUST extend PlainAppStore with docs-specific state management.

## [👤semio📚js🗃️sketchpad💻docs🔖commands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Commands)

Docs app command handlers MUST modify state through diff objects.

## [👤semio📚js🗃️sketchpad💻docs🔖canvas](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Canvas)

Canvas components MUST render the docs app visual content.
Window components MUST provide windowed views within the canvas.
Page window MUST render MDX content with navigation and heading extraction.

## [👤semio📚js🗃️sketchpad💻docs🔖windows](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Windows)

Window components MUST provide windowed views within the canvas.
Page window MUST render MDX content with navigation and heading extraction.

## [👤semio📚js🗃️sketchpad💻docs🔖page](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Page)

Page window MUST render MDX content with navigation and heading extraction.

## [👤semio📚js🗃️sketchpad💻docs🔖footer](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Footer)

Footer component MUST manage docs app footer items.

## [👤semio📚js🗃️sketchpad💻docs🔖panels](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Panels)

Panel components MUST render sidebar content for the docs app.

## [👤semio📚js🗃️sketchpad💻docs🔖app](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/App)

Docs app root component MUST compose MDX routing, panel sections, and layout.

## [👤semio📚js🗃️sketchpad💻docs🔖config](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Docs.tsx/s/Config)

Docs app route, panel, and path matching configuration MUST be exported.

## [👤semio📚js🗃️sketchpad💻feedback🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Imports)

MUST import external and internal modules for the Feedback app.

## [👤semio📚js🗃️sketchpad💻feedback🔖feedbackapppluginregistration](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Feedback%20App%20Plugin%20Registration)

MUST register the Feedback app plugin with default state and event handlers.

## [👤semio📚js🗃️sketchpad💻feedback🔖triadichooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Triadic%20Hooks)

MUST provide triadic hooks for accessing and mutating Feedback app state.

## [👤semio📚js🗃️sketchpad💻feedback🔖components](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Components)

MUST render feedback form for submitting bug reports and ideas.

## [👤semio📚js🗃️sketchpad💻feedback🔖form](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Form)

MUST render feedback form for submitting bug reports and ideas.

## [👤semio📚js🗃️sketchpad💻feedback🔖app](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/App)

MUST integrate feedback app with toolbar and layout canvas.

## [👤semio📚js🗃️sketchpad💻feedback🔖config](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Config)

MUST define app configuration for the Feedback app.

## [👤semio📚js🗃️sketchpad💻feedback🔖globalfooteritem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Feedback.tsx/s/Global%20Footer%20Item)

MUST re-export the feedback icon for the footer item.

## [👤semio📚js🗃️sketchpad💻home🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Imports)

External and internal module imports MUST be declared here.

## [👤semio📚js🗃️sketchpad💻home🔖types](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Types)

Home app type definitions MUST be declared here.

## [👤semio📚js🗃️sketchpad💻home🔖homeapppluginregistration](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Home%20App%20Plugin%20Registration)

Home app plugin and event handler registration MUST initialize XState context.

## [👤semio📚js🗃️sketchpad💻home🔖hooksxstatebased](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Hooks%20(XState-based))

XState-based hooks MUST re-export state selectors for the Home app.
Table window MUST display kit entries in tabular form.
Details panel MUST show properties of selected kits.

## [👤semio📚js🗃️sketchpad💻home🔖table](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Table)

Table window MUST display kit entries in tabular form.
Details panel MUST show properties of selected kits.

## [👤semio📚js🗃️sketchpad💻home🔖details](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Details)

Details panel MUST show properties of selected kits.

## [👤semio📚js🗃️sketchpad💻home🔖chat](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Chat)

Chat panel MUST show the chat placeholder content.

## [👤semio📚js🗃️sketchpad💻home🔖settings](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Settings)

Settings panel MUST expose theme, language, device, expertise, and mode toggles.

## [👤semio📚js🗃️sketchpad💻home🔖footer](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Footer)

Footer component MUST manage Home app footer items.

## [👤semio📚js🗃️sketchpad💻home🔖dropzone](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/DropZone)

DropZone component MUST handle drag-and-drop kit imports.

## [👤semio📚js🗃️sketchpad💻home🔖app](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/App)

App components MUST compose the Home app toolbar, table, and logic.

## [👤semio📚js🗃️sketchpad💻home🔖multiwindowapp](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Multi-Window%20App)

Multi-window app MUST orchestrate the Home canvas and layout.

## [👤semio📚js🗃️sketchpad💻home🔖config](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Home.tsx/s/Config)

Config MUST define the Home app registration and panel setup.

## [👤semio📚js🗃️sketchpad💻kit🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Imports)

Imports for Kit app MUST include all shared sketchpad, React, DnD, and UI dependencies.

## [👤semio📚js🗃️sketchpad💻kit🔖designfamilyhelpers](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Design%20Family%20Helpers)

Design family helper functions MUST traverse the design hierarchy to collect related design GUIDs.

## [👤semio📚js🗃️sketchpad💻kit🔖internalstatemanagement](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Internal%20State%20Management)

Constants MUST define artifact kinds and toolbar sub-tool configurations for the Kit app.

## [👤semio📚js🗃️sketchpad💻kit🔖constants](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Constants)

Constants MUST define artifact kinds and toolbar sub-tool configurations for the Kit app.

## [👤semio📚js🗃️sketchpad💻kit🔖internalstatemanagement](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Internal%20State%20Management)

Internal state management MUST define all Kit app types, interfaces, store, and Y.js synchronization.

## [👤semio📚js🗃️sketchpad💻kit🔖kitapppluginregistration](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Kit%20App%20Plugin%20Registration)

Kit app plugin registration MUST register the Kit app plugin with machine actions, guards, and default state.

## [👤semio📚js🗃️sketchpad💻kit🔖actionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Action%20Hooks)

Action hooks MUST provide composable React hooks for Kit app selection, hover, sort, filter, and transaction actions.

## [👤semio📚js🗃️sketchpad💻kit🔖selectionhelperhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Selection%20Helper%20Hooks)

Selection helper hooks MUST provide entity-specific add, remove, toggle, select-single, select-all, and clear operations.

## [👤semio📚js🗃️sketchpad💻kit🔖typesselectionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Types%20Selection%20Hooks)

Types selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for type selection.

## [👤semio📚js🗃️sketchpad💻kit🔖designsselectionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Designs%20Selection%20Hooks)

Designs selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for design selection.

## [👤semio📚js🗃️sketchpad💻kit🔖qualitiesselectionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Qualities%20Selection%20Hooks)

Qualities selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for quality selection.

## [👤semio📚js🗃️sketchpad💻kit🔖portsselectionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Ports%20Selection%20Hooks)

Ports selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for port selection.

## [👤semio📚js🗃️sketchpad💻kit🔖tagsselectionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Tags%20Selection%20Hooks)

Tags selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for tag selection.

## [👤semio📚js🗃️sketchpad💻kit🔖conceptsselectionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Concepts%20Selection%20Hooks)

Concepts selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for concept selection.

## [👤semio📚js🗃️sketchpad💻kit🔖filesselectionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Files%20Selection%20Hooks)

Files selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for file selection.

## [👤semio📚js🗃️sketchpad💻kit🔖foldersselectionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Folders%20Selection%20Hooks)

Folders selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for folder selection.

## [👤semio📚js🗃️sketchpad💻kit🔖authorsselectionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Authors%20Selection%20Hooks)

Authors selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for author selection.

## [👤semio📚js🗃️sketchpad💻kit🔖globalselectionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Global%20Selection%20Hooks)

Global selection hooks MUST provide select-all across all artifact kinds.

## [👤semio📚js🗃️sketchpad💻kit🔖types](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Types)

Types MUST provide hover status and color hooks for type visual indication in the Kit app.

## [👤semio📚js🗃️sketchpad💻kit🔖designs](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Designs)

Designs MUST provide hover status and color hooks for design visual indication in the Kit app.

## [👤semio📚js🗃️sketchpad💻kit🔖commands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Commands)

Commands MUST define all executable Kit app actions for artifact CRUD, import, and export.

## [👤semio📚js🗃️sketchpad💻kit🔖canvas](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Canvas)

Table MUST render the interactive data table with sortable columns, expandable rows, and drag-drop reordering.

## [👤semio📚js🗃️sketchpad💻kit🔖windows](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Windows)

Table MUST render the interactive data table with sortable columns, expandable rows, and drag-drop reordering.

## [👤semio📚js🗃️sketchpad💻kit🔖table](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Table)

Table MUST render the interactive data table with sortable columns, expandable rows, and drag-drop reordering.

## [👤semio📚js🗃️sketchpad💻kit🔖diagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Diagram)

Diagram MUST render the interactive force-directed Kit diagram with type and design nodes.

## [👤semio📚js🗃️sketchpad💻kit🔖tools](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Tools)

Tools MUST define Kit app toolbar filter and selection tool components.

## [👤semio📚js🗃️sketchpad💻kit🔖panels](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Panels)

Details MUST render the Kit app detail panels for kit, type, port, tag, concept, design, file, folder, and multi-artifact sections.

## [👤semio📚js🗃️sketchpad💻kit🔖right](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Right)

Details MUST render the Kit app detail panels for kit, type, port, tag, concept, design, file, folder, and multi-artifact sections.

## [👤semio📚js🗃️sketchpad💻kit🔖details](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Details)

Details MUST render the Kit app detail panels for kit, type, port, tag, concept, design, file, folder, and multi-artifact sections.

## [👤semio📚js🗃️sketchpad💻kit🔖settings](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Settings)

Settings MUST render the Kit app settings panel with theme, language, device, expertise, mode, and diagram force controls.

## [👤semio📚js🗃️sketchpad💻kit🔖footer](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Footer)

Footer MUST render the Kit app footer with selection count status.

## [👤semio📚js🗃️sketchpad💻kit🔖config](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Kit.tsx/s/Config)

Config MUST export the Kit app configuration with route segments, panel definitions, and path matching.

## [👤semio📚js🗃️sketchpad💻quality🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Imports)

External and internal module imports MUST be declared here.

## [👤semio📚js🗃️sketchpad💻quality🔖types](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types)

Type definitions MUST declare quality app state, selections, and formula structures.

## [👤semio📚js🗃️sketchpad💻quality🔖functions](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Functions)

Formula function definitions, parsing, and LaTeX conversion utilities MUST be declared here.

## [👤semio📚js🗃️sketchpad💻quality🔖commands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Commands)

Quality app command handlers MUST modify state through diff objects.

## [👤semio📚js🗃️sketchpad💻quality🔖store](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store)

Quality app store, hooks, and reactive state management MUST be declared here.

## [👤semio📚js🗃️sketchpad💻quality🔖qualityapppluginregistration](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Quality%20App%20Plugin%20Registration)

Plugin registration and event handler wiring MUST initialize quality app context.

## [👤semio📚js🗃️sketchpad💻quality🔖actionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Action%20Hooks)

Memoized action hooks MUST provide formula node interaction callbacks.

## [👤semio📚js🗃️sketchpad💻quality🔖components](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components)

React components MUST render the quality app formula diagram, details panel, and workbench.

## [👤semio📚js🗃️sketchpad💻quality🔖app](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/App)

Main quality app component MUST compose window layout, drag-drop, and hotkey handling.

## [👤semio📚js🗃️sketchpad💻quality🔖config](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Config)

Quality app route, panel, and path matching configuration MUST be exported.

## [👤semio📚js🗃️sketchpad💻sketchpad🔖utilities](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Sketchpad.tsx/s/Utilities)

Utilities MUST provide the utilities functionality.

## [👤semio📚js🗃️sketchpad💻tutorials🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Imports)

External and internal module imports MUST be declared here.

## [👤semio📚js🗃️sketchpad💻tutorials🔖components](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components)

Tutorial UI components MUST provide playback and recording controls.
Tutorial playback controls MUST render in the footer during active tutorials.

## [👤semio📚js🗃️sketchpad💻tutorials🔖tutorialcontrols](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Tutorial%20Controls)

Tutorial playback controls MUST render in the footer during active tutorials.

## [👤semio📚js🗃️sketchpad💻tutorials🔖recordingcontrols](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Recording%20Controls)

Recording controls MUST render in the footer during active recording in dev mode.

## [👤semio📚js🗃️sketchpad💻tutorials🔖recordbutton](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Record%20Button)

Record button MUST toggle recording in the footer when in dev mode.

## [👤semio📚js🗃️sketchpad💻tutorials🔖tutorialoverlay](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Tutorial%20Overlay)

Tutorial overlay MUST render focus highlights and cursor animations during playback.

## [👤semio📚js🗃️sketchpad💻tutorials🔖builtintutorials](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Built-in%20Tutorials)

Built-in tutorials MUST define default tutorial content shipped with the app.

## [👤semio📚js🗃️sketchpad💻tutorials🔖commands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Commands)

Tutorial and recording command definitions MUST map command names to store actions.

## [👤semio📚js🗃️sketchpad💻tutorials🔖commandinterceptor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Command%20Interceptor)

Command interceptor MUST record events and check milestone completion during playback.

## [👤semio📚js🗃️sketchpad💻tutorials🔖hooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Hooks)

Tutorial hooks MUST provide reactive access to tutorial and recording state.

## [👤semio📚js🗃️sketchpad💻tutorials🔖context](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Context)

Tutorial context MUST provide the store and state to descendant components.

## [👤semio📚js🗃️sketchpad💻tutorials🔖types](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Types)

Tutorial type definitions MUST be declared here.
Tutorial entity interfaces MUST define milestones, recordings, and playback state.

## [👤semio📚js🗃️sketchpad💻tutorials🔖tutorialentities](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Tutorial%20Entities)

Tutorial entity interfaces MUST define milestones, recordings, and playback state.

## [👤semio📚js🗃️sketchpad💻tutorials🔖store](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Store)

Tutorial store MUST manage playback, recording, and milestone navigation state.

## [👤semio📚js🗃️sketchpad💻type🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports)

External and internal dependency imports. MUST group third-party and local imports.

## [👤semio📚js🗃️sketchpad💻type🔖internalstatemanagement](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Internal%20State%20Management)

TypeApp state interfaces, enums, and diffing types. MUST define all shared state shapes.

## [👤semio📚js🗃️sketchpad💻type🔖typeapppluginregistration](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Type%20App%20Plugin%20Registration)

Plugin registration and XState event handlers for the TypeApp. MUST register all event handlers at module load.

## [👤semio📚js🗃️sketchpad💻type🔖xstatehooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/XState%20Hooks)

React hooks that read and write TypeApp XState machine state. MUST use memoized selectors for performance.

## [👤semio📚js🗃️sketchpad💻type🔖actionhooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Action%20Hooks)

Convenience React hooks wrapping state hooks into single-purpose actions. MUST return action-canAct tuples.

## [👤semio📚js🗃️sketchpad💻type🔖commands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Commands)

Command definitions for the TypeApp producing diffs from context. MUST return TypeAppCommandResult.

## [👤semio📚js🗃️sketchpad💻type🔖scene](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene)

Three.js scene components for connectors, meshes, and the 3D viewport. MUST render inside a React Three Fiber canvas.

## [👤semio📚js🗃️sketchpad💻type🔖panels](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels)

Panel UI sections for the right sidebar including details and settings editors. MUST use the panel section registration API.
Right sidebar panel containing details and settings sub-sections. MUST nest detail and settings regions.
Detail panel sections for editing type properties, connectors, models, authors, and attributes. MUST render within tree items.

## [👤semio📚js🗃️sketchpad💻type🔖right](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Right)

Right sidebar panel containing details and settings sub-sections. MUST nest detail and settings regions.
Detail panel sections for editing type properties, connectors, models, authors, and attributes. MUST render within tree items.

## [👤semio📚js🗃️sketchpad💻type🔖details](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Details)

Detail panel sections for editing type properties, connectors, models, authors, and attributes. MUST render within tree items.

## [👤semio📚js🗃️sketchpad💻type🔖settings](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Settings)

Settings panel for theme, language, device, expertise, and mode selection. MUST use toggle groups and select elements.

## [👤semio📚js🗃️sketchpad💻type🔖tools](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Tools)

Tool definitions for selection modes and connector creation. MUST export tool objects and settings components.

## [👤semio📚js🗃️sketchpad💻type🔖app](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/App)

Main TypeApp component orchestrating panels, scene, keyboard shortcuts, and drag-and-drop. MUST register sections on mount.

## [👤semio📚js🗃️sketchpad💻type🔖footer](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Footer)

Footer component displaying model tag toggles. MUST update footer items when tags change.

## [👤semio📚js🗃️sketchpad💻type🔖config](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Config)

App configuration for the TypeApp including route segments, panels, and path matching. MUST define all route segments.

## [👤semio📚js🗃️sketchpad🗃️apps💻index🔖exports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/fd/org/apps/f/index.ts/s/Exports)

Exports MUST expose only the public API surface of the shared module.

## [👤semio📚js🗃️sketchpad💻elements🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Imports)

Consumers MUST NOT add non-tree-shakeable imports.

## [👤semio📚js🗃️sketchpad💻elements🔖sectionspecificity](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Section%20Specificity)

Consumers MUST use these constants for section precedence.

## [👤semio📚js🗃️sketchpad💻elements🔖interactioncontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Interaction%20Context)

Consumers MUST wrap interactive elements with InteractionProvider.

## [👤semio📚js🗃️sketchpad💻elements🔖levelcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Level%20Context)

Consumers MUST wrap components with LevelProvider.

## [👤semio📚js🗃️sketchpad💻elements🔖element](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element)

Consumers MUST use level functions for consistent styling.

## [👤semio📚js🗃️sketchpad💻elements🔖command](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Command)

Consumers MUST use CommandInput for search functionality.

## [👤semio📚js🗃️sketchpad💻elements🔖footer](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Footer)

Consumers MUST provide FooterItem entries for each action.

## [👤semio📚js🗃️sketchpad💻elements🔖layout](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Layout)

Consumers MUST provide a canvas element.

## [👤semio📚js🗃️sketchpad💻elements🔖tooltip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip)

Consumers MUST configure the expertise mode provider.

## [👤semio📚js🗃️sketchpad💻elements🔖basecomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Base%20Components)

Consumers MUST use these as building blocks for inputs.

## [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components)

Consumers MUST pass valid config objects.

## [👤semio📚js🗃️sketchpad💻elements🔖aside](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aside)

Consumers MUST specify a valid kind prop.

## [👤semio📚js🗃️sketchpad💻elements🔖avatar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Avatar)

Consumers MUST provide content for the fallback.

## [👤semio📚js🗃️sketchpad💻elements🔖card](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Card)

Consumers MUST provide a title string.

## [👤semio📚js🗃️sketchpad💻elements🔖spinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Spinner)

Consumers MUST choose an appropriate size for the context.

## [👤semio📚js🗃️sketchpad💻elements🔖notfound](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/NotFound)

Consumers MUST provide a title for the error.

## [👤semio📚js🗃️sketchpad💻elements🔖loadingrow](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/LoadingRow)

Consumers MUST provide a name for the placeholder.

## [👤semio📚js🗃️sketchpad💻elements🔖diagramnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/DiagramNode)

Consumers MUST provide content for the node.

## [👤semio📚js🗃️sketchpad💻elements🔖hovercard](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/HoverCard)

Consumers MUST use HoverCardTrigger to activate.

## [👤semio📚js🗃️sketchpad💻elements🔖section](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Section)

Consumers MUST provide a heading string.

## [👤semio📚js🗃️sketchpad💻elements🔖steps](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Steps)

Consumers MUST provide step children in order.

## [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components)

Consumers MUST provide action items for the group.

## [👤semio📚js🗃️sketchpad💻elements🔖actiongroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/ActionGroup)

Consumers MUST provide action items for the group.

## [👤semio📚js🗃️sketchpad💻elements🔖combobox](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Combobox)

Consumers MUST provide options and onValueChange handler.

## [👤semio📚js🗃️sketchpad💻elements🔖input](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input)

Consumers MUST provide an id for accessibility.

## [👤semio📚js🗃️sketchpad💻elements🔖select](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Select)

Consumers MUST use SelectItem children for options.

## [👤semio📚js🗃️sketchpad💻elements🔖slider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Slider)

Consumers MUST provide min and max values.

## [👤semio📚js🗃️sketchpad💻elements🔖stepper](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Stepper)

Consumers MUST provide min and max bounds.

## [👤semio📚js🗃️sketchpad💻elements🔖textarea](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Textarea)

Consumers MUST provide an id for the field.

## [👤semio📚js🗃️sketchpad💻elements🔖toggle](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Toggle)

Consumers MUST handle onPressedChange events.

## [👤semio📚js🗃️sketchpad💻elements🔖togglegroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/ToggleGroup)

Consumers MUST provide items with distinct values.

## [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components)

Consumers MUST use AccordionItem children.

## [👤semio📚js🗃️sketchpad💻elements🔖accordion](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Accordion)

Consumers MUST use AccordionItem children.

## [👤semio📚js🗃️sketchpad💻elements🔖dialog](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog)

Consumers MUST use DialogTrigger to open.

## [👤semio📚js🗃️sketchpad💻elements🔖scrollable](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Scrollable)

Consumers MUST wrap content in Scrollable.

## [👤semio📚js🗃️sketchpad💻elements🔖band](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Band)

Consumers MUST provide BandItem entries.

## [👤semio📚js🗃️sketchpad💻elements🔖strip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Strip)

Consumers MUST provide StripItem entries.

## [👤semio📚js🗃️sketchpad💻elements🔖navbar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navbar)

Consumers MUST provide NavbarItem entries.

## [👤semio📚js🗃️sketchpad💻elements🔖tabs](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tabs)

Consumers MUST use TabsTrigger and TabsContent.

## [👤semio📚js🗃️sketchpad💻elements🔖tree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree)

Consumers MUST wrap components in TreeStateProvider.

## [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components)

Consumers MUST provide BreadcrumbItemData entries.

## [👤semio📚js🗃️sketchpad💻elements🔖breadcrumb](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Breadcrumb)

Consumers MUST provide BreadcrumbItemData entries.

## [👤semio📚js🗃️sketchpad💻elements🔖pagenavigation](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/PageNavigation)

PageNavigation MUST provide the pagenavigation functionality.

## [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components)

Consumers MUST set resizeSide for the handle.

## [👤semio📚js🗃️sketchpad💻elements🔖panel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel)

Consumers MUST set resizeSide for the handle.

## [👤semio📚js🗃️sketchpad💻elements🔖panelgroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/PanelGroup)

Consumers MUST provide panel children.

## [👤semio📚js🗃️sketchpad💻elements🔖leftpanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/LeftPanel)

Consumers MUST provide visible and children props.

## [👤semio📚js🗃️sketchpad💻elements🔖rightpanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/RightPanel)

Consumers MUST provide visible and children props.

## [👤semio📚js🗃️sketchpad💻elements🔖middlepanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/MiddlePanel)

Consumers MUST provide visible and children props.

## [👤semio📚js🗃️sketchpad💻elements🔖bottompanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/BottomPanel)

Consumers MUST provide visible and children props.

## [👤semio📚js🗃️sketchpad💻elements🔖sidepanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/SidePanel)

Consumers MUST provide SidePanelTabConfig entries.

## [👤semio📚js🗃️sketchpad💻elements🔖hudpanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/HudPanel)

Consumers MUST provide HudPanelTabConfig entries.

## [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components)

Consumers MUST provide a WindowConfig object.

## [👤semio📚js🗃️sketchpad💻elements🔖window](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window)

Consumers MUST provide a WindowConfig object.

## [👤semio📚js🗃️sketchpad💻elements🔖page](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Page)

Consumers MUST provide frontmatter and children.

## [👤semio📚js🗃️sketchpad💻elements🔖diagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Diagram)

Consumers MUST provide nodes and edges arrays.

## [👤semio📚js🗃️sketchpad💻elements🔖scene](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Scene)

Consumers MUST provide SceneGeometry data.

## [👤semio📚js🗃️sketchpad💻elements🔖table](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Table)

Consumers MUST provide columns and data arrays.

## [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Imports)

Imports MUST include icon width constant and kit selection types.

## [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖types](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Types)

Types MUST define selection value extraction for KitAppSelection dimensions.

## [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖genericutilities](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Generic%20Utilities)

Generic Utilities MUST provide immutable selection manipulation functions.

## [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry)

Kit Diagram Geometry MUST provide geometry primitives, shape strategies, and anchor resolution.

## [👤semio📚js🗃️sketchpad💻portcolor🔖portcolor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/portColor.ts/s/Port%20Color)

MUST use a union-find structure to group compatible ports under a single color.

## [👤semio📚js🗃️sketchpad💻shared🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Imports)

MUST import XState, Y.js, and semio core types for shared sketchpad infrastructure.

## [👤semio📚js🗃️sketchpad💻shared🔖types](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Types)

MUST define path segment and path types for navigating Y.js document structures.

## [👤semio📚js🗃️sketchpad💻shared🔖ypathtypes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/YPath%20Types)

MUST define path segment and path types for navigating Y.js document structures.

## [👤semio📚js🗃️sketchpad💻shared🔖granularhooktypes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Granular%20Hook%20Types)

MUST define hook result tuples and field abstractions for granular reactive state access.

## [👤semio📚js🗃️sketchpad💻shared🔖standardemptyconstants](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Standard%20Empty%20Constants)

MUST provide frozen singleton constants for empty collections and default panel visibility.

## [👤semio📚js🗃️sketchpad💻shared🔖genericdifftypes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Generic%20Diff%20Types)

MUST define generic array and selection diff types with apply and inverse operations.

## [👤semio📚js🗃️sketchpad💻shared🔖enums](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Enums)

MUST enumerate theme, expertise, mode, store status, tool, window, and panel kinds.

## [👤semio📚js🗃️sketchpad💻shared🔖ports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Ports)

MUST define file storage provider interfaces for upload, download, and delete operations.

## [👤semio📚js🗃️sketchpad💻shared🔖fileprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/File%20Provider)

MUST define file storage provider interfaces for upload, download, and delete operations.

## [👤semio📚js🗃️sketchpad💻shared🔖appids](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/App%20IDs)

MUST define identifier interfaces for design, kit, type, and quality app scopes.

## [👤semio📚js🗃️sketchpad💻shared🔖panel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Panel)

MUST define panel kind configurations, visibility, sizing, sections, and definition interfaces.

## [👤semio📚js🗃️sketchpad💻shared🔖appregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/App%20Registry)

MUST define route segment and app configuration interfaces for app registration.

## [👤semio📚js🗃️sketchpad💻shared🔖sketchpadstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Sketchpad%20State)

MUST define mutable and immutable sketchpad state interfaces with diff types.

## [👤semio📚js🗃️sketchpad💻shared🔖commands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Commands)

MUST define command context and result interfaces for kit and sketchpad operations.

## [👤semio📚js🗃️sketchpad💻shared🔖store](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Store)

MUST define store state, app step, edit, diff, and command result interfaces.

## [👤semio📚js🗃️sketchpad💻shared🔖completestate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Complete%20State)

MUST define the complete aggregated state interface for the entire sketchpad.

## [👤semio📚js🗃️sketchpad💻shared🔖window](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Window)

MUST define window configuration, control, layout parsing, and default layout creation.

## [👤semio📚js🗃️sketchpad💻shared🔖tool](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Tool)

MUST define tool interfaces for selection, lasso, connector, and hand interactions.

## [👤semio📚js🗃️sketchpad💻shared🔖focus](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Focus)

MUST define the focus item interface for search and navigation targets.

## [👤semio📚js🗃️sketchpad💻shared🔖footer](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Footer)

MUST define the footer item interface for status bar entries.

## [👤semio📚js🗃️sketchpad💻shared🔖panelprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Panel%20Props)

MUST define resizable panel props interface for panel width management.

## [👤semio📚js🗃️sketchpad💻shared🔖xstateintegration](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/XState%20Integration)

MUST define XState machine context and event type interfaces for sketchpad, kit, and app machines.

## [👤semio📚js🗃️sketchpad💻shared🔖xstatetypes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/XState%20Types)

MUST define XState machine context and event type interfaces for sketchpad, kit, and app machines.

## [👤semio📚js🗃️sketchpad💻shared🔖yjsxstatebridge](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Y.js-XState%20Bridge)

MUST bridge Y.js document observation to XState machine events.

## [👤semio📚js🗃️sketchpad💻shared🔖machinefactories](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Machine%20Factories)

MUST define machine input and transaction configuration interfaces for state machine creation.

## [👤semio📚js🗃️sketchpad💻shared🔖ypathhelpers](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/YPath%20Helpers)

MUST provide path segment constructors, value retrieval, and observation functions for Y.js paths.

## [👤semio📚js🗃️sketchpad💻shared🔖derivedstore](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Derived%20Store)

MUST provide reactive derived computation nodes with dependency tracking and caching.

## [👤semio📚js🗃️sketchpad💻shared🔖storefactoryregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Store%20Factory%20Registry)

MUST manage registration and retrieval of app-specific store factory functions.

## [👤semio📚js🗃️sketchpad💻shared🔖apppluginregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/App%20Plugin%20Registry)

MUST manage plugin registration, retrieval, and contribution composition for app extensions.

## [👤semio📚js🗃️sketchpad💻shared🔖dynamiceventdispatchregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Dynamic%20Event%20Dispatch%20Registry)

MUST manage dynamic event handler and guard registration with namespace-based dispatch.

## [👤semio📚js🗃️sketchpad💻shared🔖appeventhandlerfactories](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/App%20Event%20Handler%20Factories)

MUST provide factory functions for creating standard app event handlers for panels, hover, selection, and windows.

## [👤semio📚js🗃️sketchpad💻shared🔖transactionhandlerfactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Transaction%20Handler%20Factory)

MUST provide factory functions for creating undo/redo transaction event handlers.

## [👤semio📚js🗃️sketchpad💻shared🔖selectorfactorypattern](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/Selector%20Factory%20Pattern)

MUST provide factory functions for creating property selectors with app key scoping.

## [👤semio📚js🗃️sketchpad💻shared🔖apphooksregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/App%20Hooks%20Registry)

MUST manage registration and retrieval of design and kit app hook implementations.

## [👤semio📚js🗃️sketchpad💻shared🔖appregistryexports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/shared.ts/s/App%20Registry%20Exports)

MUST provide docs registry port interface and registration for documentation section access.

## [👤semio📚js💻viteenvd🔖declarations](semiorepo://p/u/semio/b/l/js/f/vite-env.d.ts/s/Declarations)

Declarations MUST cover all custom asset import suffixes used in the project.

## [👤semio🛂jsonschema💻build🔖schemaexport](semiorepo://p/u/semio/b/s/jsonschema/f/build.ts/s/Schema%20Export)

JSON Schema export script. MUST unescape and write the kit schema file.

## [👤semio📚net🛅semio💻semio🛠️symbol](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs/d/i/Symbol)

/ Implementations MUST be immutable value types within expression trees.

## [👤semio📚net🛅semio💻semio🛠️entity](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs/d/i/Entity)

/ Implementations MUST override equality based on serialized representation.

## [👤semio📚net🛅semio💻semio🛠️entityvalidator](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs/d/i/EntityValidator)

/ Implementations MUST define validation rules in the constructor.

## [👤semio📚net🛅semio💻build🔖build](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/build.ts/s/Build)

.NET build script. MUST compile the Semio C# project via MSBuild.

## [👤semio🌐play💻index🔖entrypoint](semiorepo://p/u/semio/b/w/play/f/index.tsx/s/Entrypoint)

Entrypoint MUST register all app configs before rendering the Sketchpad component.

## [👤semio📚rs💻semio🔖imports](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Imports)

Imports MUST include all required crates and modules for the semio domain library.

## [👤semio📚rs💻semio🔖errortypes](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Error%20Types)

Error Types MUST provide the error types functionality.

## [👤semio📚rs💻semio🔖utilityfunctions](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions)

Utility Functions MUST provide the utility functions functionality.

## [👤semio📚rs💻semio🔖modeltypesattribute](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Attribute)

Model Types - Attribute MUST provide the model types - attribute functionality.

## [👤semio📚rs💻semio🔖modeltypescoord](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Coord)

Model Types - Coord MUST provide the model types - coord functionality.

## [👤semio📚rs💻semio🔖modeltypesvector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Vector)

Model Types - Vector MUST provide the model types - vector functionality.

## [👤semio📚rs💻semio🔖modeltypesplane](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Plane)

Model Types - Plane MUST provide the model types - plane functionality.

## [👤semio📚rs💻semio🔖modeltypescamera](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Camera)

Model Types - Camera MUST provide the model types - camera functionality.

## [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder)

Model Types - Location, Author, File, Folder MUST provide the model types - location, author, file, folder functionality.

## [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept)

Model Types - Quality, Port, Tag, Concept MUST provide the model types - quality, port, tag, concept functionality.

## [👤semio📚rs💻semio🔖modeltypespropmodelconnector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector)

Model Types - Prop, Model, Connector MUST provide the model types - prop, model, connector functionality.

## [👤semio📚rs💻semio🔖modeltypestype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Type)

Model Types - Type MUST provide the model types - type functionality.

## [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat)

Model Types - Layer, Piece, Group, Side, Connection, Stat MUST provide the model types - layer, piece, group, side, connection, stat functionality.

## [👤semio📚rs💻semio🔖modeltypesdesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Design)

Model Types - Design MUST provide the model types - design functionality.

## [👤semio📚rs💻semio🔖modeltypeskit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Kit)

Model Types - Kit MUST provide the model types - kit functionality.

## [👤semio📚rs💻semio🔖finderfunctions](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions)

Finder Functions MUST provide the finder functions functionality.
/ find_type_in_kit MUST perform the find_type_in_kit operation.

## [👤semio📚rs💻semio🔖serialization](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization)

Serialization MUST provide the serialization functionality.
/ serialize_kit MUST perform the serialize_kit operation.

## [👤semio📚rs💻semio🔖difftypes](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types)

Diff Types MUST provide the diff types functionality.

## [👤semio📚rs💻semio🔖hasguidtrait](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait)

HasGuid Trait MUST provide the hasguid trait functionality.
/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🔖applydiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff)

ApplyDiff MUST provide the applydiff functionality.
/ apply_collection_diff MUST perform the apply_collection_diff operation.

## [👤semio📚rs💻semio🔖flattendesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign)

FlattenDesign MUST provide the flattendesign functionality.
/ FlattenedPiece MUST perform the FlattenedPiece operation.

## [👤semio📚rs💻semio🔖validationtypes](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types)

Validation Types MUST provide the validation types functionality.

## [👤semio📚rs💻semio🔖sqliteimportexport](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/SQLite%20Import/Export)

SQLite Import/Export MUST provide the sqlite import/export functionality.

## [👤semio📚rs💻semio🔖zipimportexport](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Zip%20Import/Export)

Zip Import/Export MUST provide the zip import/export functionality.

## [👤semio📚rs💻semio🔖wasmbindings](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/WASM%20Bindings)

WASM Bindings MUST provide the wasm bindings functionality.

## [👤semio📚rs💻semio🔖tests](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Tests)

Tests MUST provide the tests functionality.

## [👤semio📚rs💻semio🔖roundtriptests](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Roundtrip%20Tests)

Roundtrip Tests MUST provide the roundtrip tests functionality.

## [👤semio📚rs💻semio🔖flattentests](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Flatten%20Tests)

Flatten Tests MUST provide the flatten tests functionality.

## [👤semio📚rs💻semio🔖difftests](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Tests)

Diff Tests MUST provide the diff tests functionality.

## [👤semio📚rs💻semio🔖validationtests](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Tests)

Validation Tests MUST provide the validation tests functionality.

## [👤semio📚rs💻semio🛠️guid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/guid)

/ guid MUST perform the guid operation.

## [👤semio📚rs💻semio🛠️normalize](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/normalize)

/ normalize MUST perform the normalize operation.

## [👤semio📚rs💻semio🛠️round](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/round)

/ round MUST perform the round operation.

## [👤semio📚rs💻semio🛠️jaccard](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/jaccard)

/ jaccard MUST perform the jaccard operation.

## [👤semio📚rs💻semio🛠️deepequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/deep_equal)

/ deep_equal MUST perform the deep_equal operation.

## [👤semio📚rs💻semio🛠️generateuniquename](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/generate_unique_name)

/ generate_unique_name MUST perform the generate_unique_name operation.

## [👤semio📚rs💻semio🛠️attribute](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Attribute)

/ Attribute MUST perform the Attribute operation.

## [👤semio📚rs💻semio🛠️attributeid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/AttributeId)

/ AttributeId MUST perform the AttributeId operation.

## [👤semio📚rs💻semio🛠️coord](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Coord)

/ Coord MUST perform the Coord operation.

## [👤semio📚rs💻semio🛠️coord](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Coord)

/ Coord MUST perform the Coord operation.

## [👤semio📚rs💻semio🛠️vector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Vector)

/ Vector MUST perform the Vector operation.

## [👤semio📚rs💻semio🛠️vector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Vector)

/ Vector MUST perform the Vector operation.

## [👤semio📚rs💻semio🛠️plane](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Plane)

/ Plane MUST perform the Plane operation.

## [👤semio📚rs💻semio🛠️default](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Default)

/ Default MUST perform the Default operation.

## [👤semio📚rs💻semio🛠️plane](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Plane)

/ Plane MUST perform the Plane operation.

## [👤semio📚rs💻semio🛠️camera](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Camera)

/ Camera MUST perform the Camera operation.

## [👤semio📚rs💻semio🛠️default](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Default)

/ Default MUST perform the Default operation.

## [👤semio📚rs💻semio🛠️locationid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/LocationId)

/ LocationId MUST perform the LocationId operation.

## [👤semio📚rs💻semio🛠️location](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Location)

/ Location MUST perform the Location operation.

## [👤semio📚rs💻semio🛠️authorid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/AuthorId)

/ AuthorId MUST perform the AuthorId operation.

## [👤semio📚rs💻semio🛠️author](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Author)

/ Author MUST perform the Author operation.

## [👤semio📚rs💻semio🛠️folderid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/FolderId)

/ FolderId MUST perform the FolderId operation.

## [👤semio📚rs💻semio🛠️folder](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Folder)

/ Folder MUST perform the Folder operation.

## [👤semio📚rs💻semio🛠️fileid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/FileId)

/ FileId MUST perform the FileId operation.

## [👤semio📚rs💻semio🛠️file](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/File)

/ File MUST perform the File operation.

## [👤semio📚rs💻semio🛠️qualityid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/QualityId)

/ QualityId MUST perform the QualityId operation.

## [👤semio📚rs💻semio🛠️quality](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Quality)

/ Quality MUST perform the Quality operation.

## [👤semio📚rs💻semio🛠️portid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PortId)

/ PortId MUST perform the PortId operation.

## [👤semio📚rs💻semio🛠️port](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Port)

/ Port MUST perform the Port operation.

## [👤semio📚rs💻semio🛠️tagid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/TagId)

/ TagId MUST perform the TagId operation.

## [👤semio📚rs💻semio🛠️tag](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Tag)

/ Tag MUST perform the Tag operation.

## [👤semio📚rs💻semio🛠️conceptid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConceptId)

/ ConceptId MUST perform the ConceptId operation.

## [👤semio📚rs💻semio🛠️concept](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Concept)

/ Concept MUST perform the Concept operation.
/ Concept MUST perform the Concept operation.

## [👤semio📚rs💻semio🛠️propid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PropId)

/ PropId MUST perform the PropId operation.

## [👤semio📚rs💻semio🛠️prop](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Prop)

/ Prop MUST perform the Prop operation.

## [👤semio📚rs💻semio🛠️modelid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ModelId)

/ ModelId MUST perform the ModelId operation.

## [👤semio📚rs💻semio🛠️model](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Model)

/ Model MUST perform the Model operation.
/ Model MUST perform the Model operation.

## [👤semio📚rs💻semio🛠️connectorid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConnectorId)

/ ConnectorId MUST perform the ConnectorId operation.

## [👤semio📚rs💻semio🛠️connector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Connector)

/ Connector MUST perform the Connector operation.

## [👤semio📚rs💻semio🛠️typeid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/TypeId)

/ TypeId MUST perform the TypeId operation.

## [👤semio📚rs💻semio🛠️type](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Type)

/ Type MUST perform the Type operation.

## [👤semio📚rs💻semio🛠️layerid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/LayerId)

/ LayerId MUST perform the LayerId operation.

## [👤semio📚rs💻semio🛠️layer](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Layer)

/ Layer MUST perform the Layer operation.

## [👤semio📚rs💻semio🛠️pieceid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PieceId)

/ PieceId MUST perform the PieceId operation.

## [👤semio📚rs💻semio🛠️designid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DesignId)

/ DesignId MUST perform the DesignId operation.

## [👤semio📚rs💻semio🛠️piece](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Piece)

/ Piece MUST perform the Piece operation.

## [👤semio📚rs💻semio🛠️groupid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/GroupId)

/ GroupId MUST perform the GroupId operation.

## [👤semio📚rs💻semio🛠️group](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Group)

/ Group MUST perform the Group operation.

## [👤semio📚rs💻semio🛠️side](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Side)

/ Side MUST perform the Side operation.

## [👤semio📚rs💻semio🛠️connectionid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConnectionId)

/ ConnectionId MUST perform the ConnectionId operation.

## [👤semio📚rs💻semio🛠️connection](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Connection)

/ Connection MUST perform the Connection operation.

## [👤semio📚rs💻semio🛠️statid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/StatId)

/ StatId MUST perform the StatId operation.

## [👤semio📚rs💻semio🛠️stat](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Stat)

/ Stat MUST perform the Stat operation.
/ Stat MUST perform the Stat operation.

## [👤semio📚rs💻semio🛠️design](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Design)

/ Design MUST perform the Design operation.

## [👤semio📚rs💻semio🛠️kit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Kit)

/ Kit MUST perform the Kit operation.

## [👤semio📚rs💻semio🛠️findtypeinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_type_in_kit)

/ find_type_in_kit MUST perform the find_type_in_kit operation.

## [👤semio📚rs💻semio🛠️findtypeinkitmut](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_type_in_kit_mut)

/ find_type_in_kit_mut MUST perform the find_type_in_kit_mut operation.

## [👤semio📚rs💻semio🛠️finddesigninkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_design_in_kit)

/ find_design_in_kit MUST perform the find_design_in_kit operation.

## [👤semio📚rs💻semio🛠️finddesigninkitmut](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_design_in_kit_mut)

/ find_design_in_kit_mut MUST perform the find_design_in_kit_mut operation.

## [👤semio📚rs💻semio🛠️findpieceindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_piece_in_design)

/ find_piece_in_design MUST perform the find_piece_in_design operation.

## [👤semio📚rs💻semio🛠️findpieceindesignmut](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_piece_in_design_mut)

/ find_piece_in_design_mut MUST perform the find_piece_in_design_mut operation.

## [👤semio📚rs💻semio🛠️findconnectionindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_connection_in_design)

/ find_connection_in_design MUST perform the find_connection_in_design operation.

## [👤semio📚rs💻semio🛠️findconnectorintype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_connector_in_type)

/ find_connector_in_type MUST perform the find_connector_in_type operation.

## [👤semio📚rs💻semio🛠️findmodelintype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_model_in_type)

/ find_model_in_type MUST perform the find_model_in_type operation.

## [👤semio📚rs💻semio🛠️findfileinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_file_in_kit)

/ find_file_in_kit MUST perform the find_file_in_kit operation.

## [👤semio📚rs💻semio🛠️findfolderinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_folder_in_kit)

/ find_folder_in_kit MUST perform the find_folder_in_kit operation.

## [👤semio📚rs💻semio🛠️findauthorinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_author_in_kit)

/ find_author_in_kit MUST perform the find_author_in_kit operation.
/ find_author_in_kit MUST perform the find_author_in_kit operation.

## [👤semio📚rs💻semio🛠️findtaginkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_tag_in_kit)

/ find_tag_in_kit MUST perform the find_tag_in_kit operation.

## [👤semio📚rs💻semio🛠️findconceptinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_concept_in_kit)

/ find_concept_in_kit MUST perform the find_concept_in_kit operation.

## [👤semio📚rs💻semio🛠️findqualityinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_quality_in_kit)

/ find_quality_in_kit MUST perform the find_quality_in_kit operation.

## [👤semio📚rs💻semio🛠️findinterfaceinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_interface_in_kit)

/ find_interface_in_kit MUST perform the find_interface_in_kit operation.

## [👤semio📚rs💻semio🛠️findlayerindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_layer_in_design)

/ find_layer_in_design MUST perform the find_layer_in_design operation.

## [👤semio📚rs💻semio🛠️findgroupindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_group_in_design)

/ find_group_in_design MUST perform the find_group_in_design operation.

## [👤semio📚rs💻semio🛠️findstatindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_stat_in_design)

/ find_stat_in_design MUST perform the find_stat_in_design operation.

## [👤semio📚rs💻semio🛠️serializekit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/serialize_kit)

/ serialize_kit MUST perform the serialize_kit operation.

## [👤semio📚rs💻semio🛠️deserializekit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/deserialize_kit)

/ deserialize_kit MUST perform the deserialize_kit operation.

## [👤semio📚rs💻semio🛠️serializedesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/serialize_design)

/ serialize_design MUST perform the serialize_design operation.

## [👤semio📚rs💻semio🛠️deserializedesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/deserialize_design)

/ deserialize_design MUST perform the deserialize_design operation.

## [👤semio📚rs💻semio🛠️serializetype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/serialize_type)

/ serialize_type MUST perform the serialize_type operation.

## [👤semio📚rs💻semio🛠️deserializetype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/deserialize_type)

/ deserialize_type MUST perform the deserialize_type operation.

## [👤semio📚rs💻semio🛠️arekitsequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/are_kits_equal)

/ are_kits_equal MUST perform the are_kits_equal operation.
/ are_kits_equal MUST perform the are_kits_equal operation.

## [👤semio📚rs💻semio🛠️aredesignsequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/are_designs_equal)

/ are_designs_equal MUST perform the are_designs_equal operation.

## [👤semio📚rs💻semio🛠️aretypesequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/are_types_equal)

/ are_types_equal MUST perform the are_types_equal operation.

## [👤semio📚rs💻semio🛠️issupportedmodelextension](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/is_supported_model_extension)

/ is_supported_model_extension MUST perform the is_supported_model_extension operation.
/ is_supported_model_extension MUST perform the is_supported_model_extension operation.

## [👤semio📚rs💻semio🛠️removeditem](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/RemovedItem)

/ RemovedItem MUST perform the RemovedItem operation.

## [👤semio📚rs💻semio🛠️diffupdate](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffUpdate)

/ DiffUpdate MUST perform the DiffUpdate operation.

## [👤semio📚rs💻semio🛠️collectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/CollectionDiff)

/ CollectionDiff MUST perform the CollectionDiff operation.

## [👤semio📚rs💻semio🛠️attributediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/AttributeDiff)

/ AttributeDiff MUST perform the AttributeDiff operation.

## [👤semio📚rs💻semio🛠️propdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PropDiff)

/ PropDiff MUST perform the PropDiff operation.

## [👤semio📚rs💻semio🛠️connectordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConnectorDiff)

/ ConnectorDiff MUST perform the ConnectorDiff operation.

## [👤semio📚rs💻semio🛠️modeldiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ModelDiff)

/ ModelDiff MUST perform the ModelDiff operation.

## [👤semio📚rs💻semio🛠️typediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/TypeDiff)

/ TypeDiff MUST perform the TypeDiff operation.

## [👤semio📚rs💻semio🛠️sidediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/SideDiff)

/ SideDiff MUST perform the SideDiff operation.

## [👤semio📚rs💻semio🛠️connectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConnectionDiff)

/ ConnectionDiff MUST perform the ConnectionDiff operation.

## [👤semio📚rs💻semio🛠️piecediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PieceDiff)

/ PieceDiff MUST perform the PieceDiff operation.

## [👤semio📚rs💻semio🛠️layerdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/LayerDiff)

/ LayerDiff MUST perform the LayerDiff operation.

## [👤semio📚rs💻semio🛠️groupdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/GroupDiff)

/ GroupDiff MUST perform the GroupDiff operation.

## [👤semio📚rs💻semio🛠️statdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/StatDiff)

/ StatDiff MUST perform the StatDiff operation.

## [👤semio📚rs💻semio🛠️designdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DesignDiff)

/ DesignDiff MUST perform the DesignDiff operation.

## [👤semio📚rs💻semio🛠️tagdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/TagDiff)

/ TagDiff MUST perform the TagDiff operation.

## [👤semio📚rs💻semio🛠️conceptdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConceptDiff)

/ ConceptDiff MUST perform the ConceptDiff operation.

## [👤semio📚rs💻semio🛠️portdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PortDiff)

/ PortDiff MUST perform the PortDiff operation.

## [👤semio📚rs💻semio🛠️qualitydiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/QualityDiff)

/ QualityDiff MUST perform the QualityDiff operation.

## [👤semio📚rs💻semio🛠️filediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/FileDiff)

/ FileDiff MUST perform the FileDiff operation.

## [👤semio📚rs💻semio🛠️folderdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/FolderDiff)

/ FolderDiff MUST perform the FolderDiff operation.

## [👤semio📚rs💻semio🛠️authordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/AuthorDiff)

/ AuthorDiff MUST perform the AuthorDiff operation.

## [👤semio📚rs💻semio🛠️kitdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/KitDiff)

/ KitDiff MUST perform the KitDiff operation.

## [👤semio📚rs💻semio✂️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semio✂️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.
/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semio🛠️applycollectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_collection_diff)

/ apply_collection_diff MUST perform the apply_collection_diff operation.

## [👤semio📚rs💻semio🛠️applyattributediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_attribute_diff)

/ apply_attribute_diff MUST perform the apply_attribute_diff operation.
/ apply_attribute_diff MUST perform the apply_attribute_diff operation.

## [👤semio📚rs💻semio🛠️applypropdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_prop_diff)

/ apply_prop_diff MUST perform the apply_prop_diff operation.

## [👤semio📚rs💻semio🛠️applyconnectordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_connector_diff)

/ apply_connector_diff MUST perform the apply_connector_diff operation.

## [👤semio📚rs💻semio🛠️applymodeldiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_model_diff)

/ apply_model_diff MUST perform the apply_model_diff operation.

## [👤semio📚rs💻semio🛠️applytypediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_type_diff)

/ apply_type_diff MUST perform the apply_type_diff operation.

## [👤semio📚rs💻semio🛠️applylayerdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_layer_diff)

/ apply_layer_diff MUST perform the apply_layer_diff operation.

## [👤semio📚rs💻semio🛠️applygroupdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_group_diff)

/ apply_group_diff MUST perform the apply_group_diff operation.

## [👤semio📚rs💻semio🛠️applystatdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_stat_diff)

/ apply_stat_diff MUST perform the apply_stat_diff operation.

## [👤semio📚rs💻semio🛠️applypiecediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_piece_diff)

/ apply_piece_diff MUST perform the apply_piece_diff operation.

## [👤semio📚rs💻semio🛠️applyconnectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_connection_diff)

/ apply_connection_diff MUST perform the apply_connection_diff operation.

## [👤semio📚rs💻semio🛠️applydesigndiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_design_diff)

/ apply_design_diff MUST perform the apply_design_diff operation.

## [👤semio📚rs💻semio🛠️applytagdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_tag_diff)

/ apply_tag_diff MUST perform the apply_tag_diff operation.

## [👤semio📚rs💻semio🛠️applyconceptdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_concept_diff)

/ apply_concept_diff MUST perform the apply_concept_diff operation.

## [👤semio📚rs💻semio🛠️applyinterfacediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_interface_diff)

/ apply_interface_diff MUST perform the apply_interface_diff operation.

## [👤semio📚rs💻semio🛠️applyqualitydiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_quality_diff)

/ apply_quality_diff MUST perform the apply_quality_diff operation.

## [👤semio📚rs💻semio🛠️applyfilediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_file_diff)

/ apply_file_diff MUST perform the apply_file_diff operation.

## [👤semio📚rs💻semio🛠️applyfolderdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_folder_diff)

/ apply_folder_diff MUST perform the apply_folder_diff operation.

## [👤semio📚rs💻semio🛠️applyauthordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_author_diff)

/ apply_author_diff MUST perform the apply_author_diff operation.

## [👤semio📚rs💻semio🛠️applykitdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_kit_diff)

/ apply_kit_diff MUST perform the apply_kit_diff operation.

## [👤semio📚rs💻semio🛠️flattenedpiece](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/FlattenedPiece)

/ FlattenedPiece MUST perform the FlattenedPiece operation.

## [👤semio📚rs💻semio🛠️flattendesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/flatten_design)

/ flatten_design MUST perform the flatten_design operation.

## [👤semio📚rs💻semio🛠️planesequalapprox](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/planes_equal_approx)

/ planes_equal_approx MUST perform the planes_equal_approx operation.

## [👤semio📚rs💻semio🛠️computeconnectionmatrixfast](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/compute_connection_matrix_fast)

/ compute_connection_matrix_fast MUST perform the compute_connection_matrix_fast operation.

## [👤semio📚rs💻semio🛠️computechildplanematrix](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/compute_child_plane_matrix)

/ compute_child_plane_matrix MUST perform the compute_child_plane_matrix operation.

## [👤semio📚rs💻semio🛠️quattomatrix4](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/quat_to_matrix4)

/ quat_to_matrix4 MUST perform the quat_to_matrix4 operation.

## [👤semio📚rs💻semio🛠️maketranslation](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/make_translation)

/ make_translation MUST perform the make_translation operation.

## [👤semio📚rs💻semio🛠️applymatrix4tovec3](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_matrix4_to_vec3)

/ apply_matrix4_to_vec3 MUST perform the apply_matrix4_to_vec3 operation.

## [👤semio📚rs💻semio🛠️getconnectorforsidefast](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/get_connector_for_side_fast)

/ get_connector_for_side_fast MUST perform the get_connector_for_side_fast operation.

## [👤semio📚rs💻semio🛠️getconnectorfromtype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/get_connector_from_type)

/ get_connector_from_type MUST perform the get_connector_from_type operation.

## [👤semio📚rs💻semio🛠️connectortoplane](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/connector_to_plane)

/ connector_to_plane MUST perform the connector_to_plane operation.

## [👤semio📚rs💻semio🛠️validationproblem](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ValidationProblem)

/ ValidationProblem MUST perform the ValidationProblem operation.
/ ValidationProblem MUST perform the ValidationProblem operation.

## [👤semio📚rs💻semio🛠️validationfix](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ValidationFix)

/ ValidationFix MUST perform the ValidationFix operation.

## [👤semio📚rs💻semio🛠️validationresult](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ValidationResult)

/ ValidationResult MUST perform the ValidationResult operation.

## [👤semio📚rs💻semio🛠️validatekit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/validate_kit)

/ validate_kit MUST perform the validate_kit operation.

## [👤semio📚rs💻semio🛠️checkguiduniquenessconstraint](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_guid_uniqueness_constraint)

/ check_guid_uniqueness_constraint MUST perform the check_guid_uniqueness_constraint operation.

## [👤semio📚rs💻semio🛠️checkguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_guid)

/ check_guid MUST perform the check_guid operation.

## [👤semio📚rs💻semio🛠️checktypenameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_type_name_uniqueness)

/ check_type_name_uniqueness MUST perform the check_type_name_uniqueness operation.

## [👤semio📚rs💻semio🛠️checkdesignnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_design_name_uniqueness)

/ check_design_name_uniqueness MUST perform the check_design_name_uniqueness operation.

## [👤semio📚rs💻semio🛠️checkpiecenameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_piece_name_uniqueness)

/ check_piece_name_uniqueness MUST perform the check_piece_name_uniqueness operation.

## [👤semio📚rs💻semio🛠️checkconnectionnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_connection_name_uniqueness)

/ check_connection_name_uniqueness MUST perform the check_connection_name_uniqueness operation.

## [👤semio📚rs💻semio🛠️checkconnectornameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_connector_name_uniqueness)

/ check_connector_name_uniqueness MUST perform the check_connector_name_uniqueness operation.

## [👤semio📚rs💻semio🛠️checkmodelnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_model_name_uniqueness)

/ check_model_name_uniqueness MUST perform the check_model_name_uniqueness operation.

## [👤semio📚rs💻semio🛠️checklayerpathuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_layer_path_uniqueness)

/ check_layer_path_uniqueness MUST perform the check_layer_path_uniqueness operation.

## [👤semio📚rs💻semio🛠️checkqualitynameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_quality_name_uniqueness)

/ check_quality_name_uniqueness MUST perform the check_quality_name_uniqueness operation.

## [👤semio📚rs💻semio🛠️checkportnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_port_name_uniqueness)

/ check_port_name_uniqueness MUST perform the check_port_name_uniqueness operation.

## [👤semio📚rs💻semio🛠️checkfilenameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_file_name_uniqueness)

/ check_file_name_uniqueness MUST perform the check_file_name_uniqueness operation.

## [👤semio📚rs💻semio🛠️checkfoldernameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_folder_name_uniqueness)

/ check_folder_name_uniqueness MUST perform the check_folder_name_uniqueness operation.

## [👤semio🖱️sketchpad💻index🔖entrypoint](semiorepo://p/u/semio/b/u/sketchpad/f/index.tsx/s/Entrypoint)

Entrypoint MUST register all app configs before rendering the Sketchpad component.
