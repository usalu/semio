# 💯 Specs

## [👤semio](semiorepo://project/semio)

## Kit

A kit is a collection of types, designs, authors, qualities, attributes, and concepts.

A kit is either _static_ (a special .zip file) or _dynamic_ (bound to a runtime).

A _static_ kit contains a reserved .semio folder that contains a kit.db sqlite file.

The SQL-schema of kit.db MUST follow the schema definition.

For Inter-Process-Communication (IPC) the JSON-schema MUST be used.

## Design

A design is an undirected graph of pieces (nodes) and connections (edges) with organizational layers, groups, stats, attributes, and concepts.

A design is _proto_ (a _protodesign_) when it has no _parent_ design.

The _children_ of a _parent_ design are _subdesigns_.

A _flat_ design has no connections and all pieces are _fixed_.

The pieces are _placed_ _hierarchically_ (breadth-first) for every _component_.

Additional connections which where not used in the _placement_ can be used to validate the computed planes.

## Type

A type is a reusable component with different models, connectors, attributes, concepts, and authors.

The type is _proto_ (a _prototype_) when it has no _parent_.

The _children_ of a _parent_ type are _subtypes_.

A type can be **virtual** (intermediate type requiring other virtual types to form a physical type), **scalable**, and **mirrorable** with **stock** quantity, **unit**, and optional **location**.

## Connection

A connection is a 3D-Link between two pieces with the _translation_ parameters **gap** (offset in y-direction), **shift** (offset in x-direction) and **rise** (offset in z-direction), and the _rotation_ parameters **rotation** (rotation around y-axis), **turn** (rotation around z-axis) and **tilt** (rotation around x-axis).

The _translation_ is applied first, then the _rotation_.

The two pieces are called **_connected_** and **_connecting_** but there is no difference between them.

The _direction_ of a connection goes from the lower _hierarchy_ to the higher _hierarchy_ of the pieces.

A connection can have attributes and diagram positioning with **u** and **v** offsets.

## Piece

A piece is an instance of either a type or a design with **id**, optional **name**, optional **description**, optional **plane**, **center** position, **scale**, optional **mirror plane**, **hidden** and **locked** states, **color**, and attributes.

A piece is either _fixed_ (with a plane) or _linked_ (with a connection).

A group of _connected_ pieces is called a _component_.

The _hierarchy_ of a piece is the length of the shortest path to the next _fixed_ piece.

## Connector

A connector is a conceptual connection **point** with an outwards **direction**, **id**, optional **name**, optional **description**, and **t** value for diagram ring positioning.

A connector can be marked as **mandatory** in which case it is required to be connected to a piece.

A connector can reference a **port** for explicit compatibility control. The port defines which other ports it is compatible with.

No **port** means the _default_ port which is compatible with all other connectors.

Connector compatibility is determined by the port definitions at the kit level.

A connector can have props that define measurable characteristics and attributes for additional metadata.

## Model

A model is a **guid**, optional **name**, **file** reference, optional **tags** references, optional **description**, and attributes.

The **file** is a required reference to a kit-level file entity.

The **tags** are optional references to kit-level tag entities. No **tags** means the _default_ model.

The similarity of models is determined by the jaccard index of their tag guids.

### Supported 3D File Extensions

Model files SHOULD use supported 3D formats including: gltf, glb, fbx, obj, dae, 3ds, stl, ply, usdz, vrm, ifc, 3mf, and more.

### Model Tag Selection

The footer displays all tag names from the type's/design's models. Clicking a tag toggles its selection. The model with the highest Jaccard index matching the selected tags is displayed in the scene.

## Attribute

An attribute is metadata with a unique **name**, an optional **value**, an optional **unit** and an optional **definition** (url or text).

The **name** is kebab-cased and with .-separated string similar to toml keys.

No **value** is equivalent to the boolean _true_ where the **name** is the category of the attribute.

The **unit** is a unit identifier.

- mm for millimeter, cm for centimeter, dm for decimeter, m for meter, km for kilometer
- m² for square meter, m³ for cubic meter, m⁴ for quartic meter
- ° for degree, rad for radian
- N for newton, kN for kilonewton, MN for meganewton
- °C for degree Celsius, °F for degree Fahrenheit
- W for watt, kW for kilowatt, MW for megawatt, GW for gigawatt
- Wh for watt-hour, kWh for kilowatt-hour, MWh for megawatt-hour, GWh for gigawatt-hour
- J for joule, kJ for kilojoule, kcal for kilocalorie
- kWh/m²a for kilowatt-hour per square meter per year
- m/s for meter per second, m²/s for square meter per second, m³/s for cubic meter per second
- Pa for pascal, kPa for kilopascal, MPa for megapascal

A list of attributes is semantically equivalent to nested dictionaries where the key is the **name** and the value is the **value**.

## Tag

A tag is a kit-level entity with a unique **guid**, **name**, optional **description**, optional **icon**, and attributes.

Tags are used to categorize and filter models within a type. A model references tags via guid reference.

## Concept

A concept is a kit-level entity with a unique **guid**, **name**, optional **description**, optional **icon**, and attributes.

Concepts provide semantic grouping for types and designs. Types and designs reference concepts via guid reference.

## Plane

A plane is a location (**origin**) and orientation (**x-axis**, **y-axis** and derived z-axis) in 3D space.

The coordinate system is left-handed where the thumb points up into the direction of the z-axis, the index-finger forwards into the direction of the y-axis and the middle-finger points to the right into the direction of the x-axis.

## Url

A url is either _relative_ (to the root of the .zip file) or _remote_ (http, https, ftp, ...) string.

A _relative_ url is a /-normalized path to a file in the .zip file and is not prefixed with ., ./, /, ....

## Quality

A quality is a measurement definition with a **key**, **name**, **description**, **kind** (General, Design, Type, Piece, Connection, Connector), **unit information** (SI and Imperial), **range constraints** (min/max with exclusion flags), **default value**, and optional **formula**.

A quality can be **scalable** (adjusts with piece scaling) and have multiple **benchmarks** for performance evaluation.

The **kind** determines which entities the quality can be applied to using a bitwise enum system.

## Benchmark

A benchmark is a performance standard within a quality with a **name**, optional **icon**, and **range** (min/max with exclusion flags).

Benchmarks provide reference points for evaluating quality measurements against industry or design standards.

## Port

A port is a connector compatibility definition with **name**, optional **description**, optional **icon**, optional list of **compatible ports** references, and attributes.

The port is defined at the kit level and referenced by connectors.

An empty **compatible ports** list means the port is compatible with all other ports.

Two connectors are compatible if:

- Both have no port specified (default compatibility)
- They reference the same port
- One port's compatible list includes the other port's guid
- Either port has an empty compatible list and the other explicitly allows it

## Author

An author has a **name** and **email** and can be associated with kits, types, or designs with a **rank** indicating contribution level.

Authors provide attribution and contact information for design ownership and collaboration.

## Layer

A layer is an organizational grouping within a design with a **name**, optional **description**, and **color** for visual organization.

Layers provide a way to group and manage pieces logically within complex designs.

## Group

A group is a collection of pieces within a design with optional **name**, **description**, **color**, and attributes.

Groups enable semantic clustering of pieces that belong together functionally or conceptually.

## Prop

A prop is a **key-value** pair on a connector that references a quality with a specific **value** and optional **unit**.

Props define measurable characteristics of connectors using the quality system for standardized measurement.

## Stat

A stat is a statistical measurement on a design that references a quality with **range** (min/max) and optional **unit**.

Stats provide computed or measured performance data for entire designs using the quality framework.

## [👤semio📚engine](semiorepo://bundle/semio/engine)

## Engine

Engine startup MUST support a dev/debug mode flag that waits for debugger attachment before runtime begins.

Engine startup MUST support a pure stdio MCP server mode.

## [👤semio📚js🗃️sketchpad](semiorepo://folder/semio/js/sketchpad)

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

## [👤semio🏪assets🗃️grasshopper💻buildpy🔖build](semiorepo://section/Build)

Grasshopper XML parsing and JSON export MUST extract components and groups.

## [👤semio🏪assets💻iconsts🔖exports](semiorepo://section/Exports)

Exports MUST map each Lucide icon to a domain-specific alias name.

## [👤semio🏪assets💻indexts🔖exports](semiorepo://section/Exports)

Re-exports and data constants MUST come from the Metabolism kit assets.

## [👤semio🏪assets💻indexts🛠️buildlookup](semiorepo://definition/semio/assets/index.ts/buildLookup)

Callers MUST provide an array of objects with optional guid and name fields

## [👤semio🏪assets🛅logo💻logots🔖imports](semiorepo://section/Imports)

MUST import Node.js file system, DOM parsing, and path resolution modules.

## [👤semio🏪assets🛅logo💻logots🔖parsesvg](semiorepo://section/Parse%20SVG)

MUST read SVG content and extract all group transforms and path attributes.

## [👤semio🏪assets🛅logo💻logots🔖generatekeyframesequence](semiorepo://section/Generate%20Keyframe%20Sequence)

MUST produce forward and reverse sequence for smooth animation looping.

## [👤semio🏪assets🛅logo💻logots🔖createanimatedsvg](semiorepo://section/Create%20Animated%20SVG)

MUST generate translate, rotate, scale, fill, stroke, and stroke-width animations.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixabletsx🔖missingend](semiorepo://section/MissingEnd)

MissingEnd MUST provide the missingend functionality.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixabletsx🔖alpha](semiorepo://section/Alpha)

Alpha MUST provide the alpha functionality.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixableexpectedtsx🔖missingend](semiorepo://section/MissingEnd)

MissingEnd MUST provide the missingend functionality.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixableexpectedtsx🔖alpha](semiorepo://section/Alpha)

Alpha MUST provide the alpha functionality.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedcs🛠️fixedclass](semiorepo://definition/semio/assets/repo/some/folder/file_fixed.cs/FixedClass)

/ FixedClass MUST have a Value property.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedgo🔖package](semiorepo://section/Package)

Package MUST be named fixed.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedgo🔖functions](semiorepo://section/Functions)

Functions MUST return valid integers.
FixedValue MUST return 2.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedgo🛠️fixedvalue](semiorepo://definition/semio/assets/repo/some/folder/file_fixed.go/FixedValue)

FixedValue MUST return 2.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedpy🔖functions](semiorepo://section/Functions)

Functions MUST accept typed parameters.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedtsx🔖types](semiorepo://section/Types)

Types MUST be exported when used externally.
FixedType MUST have a name and value.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedtsx🔖components](semiorepo://section/Components)

Components MUST accept FixedType props.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedtsx✂️fixedtype](semiorepo://definition/semio/assets/repo/some/folder/file_fixed.tsx/FixedType)

FixedType MUST have a name and value.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedtsx✂️fixedkind](semiorepo://definition/semio/assets/repo/some/folder/file_fixed.tsx/FixedKind)

FixedKind MUST be one of alpha or beta.

## [👤semio🖱️desktop💻forgeenvdts🔖electronfuses](semiorepo://section/Electron%20Fuses)

Consumers MUST use these enums for configuring fuse settings.

## [👤semio🖱️desktop💻maints🔖mainprocess](semiorepo://section/Main%20Process)

MUST quit on all windows closed except on macOS.

## [👤semio🖱️desktop💻maints🛠️createwindow](semiorepo://definition/semio/desktop/main.ts/createWindow)

MUST load the vite dev server URL in development and the built file in production.

## [👤semio🖱️desktop💻preloadts🔖preload](semiorepo://section/Preload)

Preload MUST use contextBridge to safely expose IPC methods.

## [👤semio🖱️desktop💻renderertsx🔖renderer](semiorepo://section/Renderer)

MUST resolve the user identity before rendering the sketchpad.

## [👤semio🖱️desktop💻renderertsx🛠️invokewindowcontrol](semiorepo://definition/semio/desktop/renderer.tsx/invokeWindowControl)

MUST fall back gracefully when window controls are unavailable.

## [👤semio🖱️desktop💻renderertsx🪨windowevents](semiorepo://definition/semio/desktop/renderer.tsx/windowEvents)

MUST delegate to invokeWindowControl for each action.

## [👤semio🖱️desktop💻renderertsx🪨os](semiorepo://definition/semio/desktop/renderer.tsx/os)

MUST use the preload-exposed getUserId API.

## [👤semio🖱️desktop💻renderertsx🛠️app](semiorepo://definition/semio/desktop/renderer.tsx/App)

MUST show a loading state until the user ID is resolved.

## [👤semio🌐docs💻indextsx🔖entrypoint](semiorepo://section/Entrypoint)

Entrypoint MUST render into the root element defined in the docs index.html.

## [👤semio📚engine💻buildts🔖build](semiorepo://section/Build)

Build script for the engine binary. MUST bundle the engine via PyInstaller.

## [👤semio📚engine💻buildts🪨cwd](semiorepo://definition/semio/engine/build.ts/cwd)

MUST resolve to the engine folder.

## [👤semio📚engine💻buildts🪨args](semiorepo://definition/semio/engine/build.ts/args)

MUST include all required metadata and hidden imports.

## [👤semio📚engine💻enginepy🔖imports](semiorepo://section/Imports)

Imports MUST include all dependencies for store, assistant, GraphQL, REST, MCP, and engine modules.

## [👤semio📚engine💻enginepy🔖store](semiorepo://section/Store)

Store MUST provide the data access layer for kit operations via code-based routing.

## [👤semio📚engine💻enginepy🔖assistant](semiorepo://section/Assistant)

Assistant MUST provide AI-powered design prediction using OpenAI structured outputs.

## [👤semio📚engine💻enginepy🔖graphql](semiorepo://section/Graphql)

Graphql MUST map semio domain types to Graphene schema nodes for query and mutation.

## [👤semio📚engine💻enginepy🔖rest](semiorepo://section/Rest)

Rest MUST expose kit, type, design, and assistant endpoints via FastAPI.

## [👤semio📚engine💻enginepy🔖mcp](semiorepo://section/Mcp)

Mcp MUST expose kit, type, design, validation, and diff tools via Model Context Protocol.

## [👤semio📚engine💻enginepy🔖engine](semiorepo://section/Engine)

Engine MUST mount REST, GraphQL, and MCP sub-applications and manage the server lifecycle.

## [👤semio📚engine💻generateschemasts🔖schemageneration](semiorepo://section/Schema%20Generation)

Schema generation script. MUST invoke the Python engine schema generator.

## [👤semio📚engine💻postbuildts🔖postbuild](semiorepo://section/Post%20Build)

Post-build script. MUST relocate the PyInstaller output to the Grasshopper bin folder.

## [👤semio📚engine💻postbuildts🪨cwd](semiorepo://definition/semio/engine/post-build.ts/cwd)

MUST resolve to the engine folder.

## [👤semio📚engine💻postbuildts🪨exepath](semiorepo://definition/semio/engine/post-build.ts/exePath)

MUST match the PyInstaller output name.

## [👤semio📚engine💻postbuildts🪨internalpath](semiorepo://definition/semio/engine/post-build.ts/internalPath)

MUST be co-located with the executable.

## [👤semio📚engine💻postbuildts🪨grasshopperbinpath](semiorepo://definition/semio/engine/post-build.ts/grasshopperBinPath)

MUST match the .NET build output path.

## [👤semio📚engine💻postbuildts🪨grasshopperexepath](semiorepo://definition/semio/engine/post-build.ts/grasshopperExePath)

MUST use the same executable name as the PyInstaller output.

## [👤semio📚engine💻postbuildts🪨grasshopperinternalpath](semiorepo://definition/semio/engine/post-build.ts/grasshopperInternalPath)

MUST mirror the PyInstaller _internal directory structure.

## [👤semio📚engine💻sqliteschemats🔖schemaexport](semiorepo://section/Schema%20Export)

SQLite schema export script. MUST dump the database schema to a SQL file.

## [👤semio📚engine💻sqliteschemats🪨dbpath](semiorepo://definition/semio/engine/sqliteschema.ts/dbPath)

MUST point to the engine debug build output.

## [👤semio📚engine💻sqliteschemats🪨outputpath](semiorepo://definition/semio/engine/sqliteschema.ts/outputPath)

MUST resolve to the monorepo sqlite schema location.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️goo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Goo)

/ Implementations MUST override CastFrom and CastTo for type conversion.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️param](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Param)

/ Implementations MUST provide component exposure and icon metadata.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️enumgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EnumGoo)

/ Implementations MUST convert between string names and enum values.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️enumparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EnumParam)

/ Implementations MUST restrict input to valid enum members.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️passthroughcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/PassthroughComponent)

/ Implementations MUST transform input data and output the result.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️idgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/IdGoo)

/ Implementations MUST wrap entity ID types for Grasshopper data flow.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️idparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/IdParam)

/ Implementations MUST provide type-safe parameter access for IDs.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️idcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/IdComponent)

/ Implementations MUST register input parameters matching ID fields.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️diffgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DiffGoo)

/ Implementations MUST wrap entity diff types for Grasshopper data flow.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️diffparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DiffParam)

/ Implementations MUST provide type-safe parameter access for diffs.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️diffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DiffComponent)

/ Implementations MUST register input parameters matching diff fields.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️serializecomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/SerializeComponent)

/ Implementations MUST convert entities to valid JSON strings.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️deserializecomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DeserializeComponent)

/ Implementations MUST parse JSON strings into entity instances.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️serializediffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/SerializeDiffComponent)

/ Implementations MUST convert diffs to valid JSON strings.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️deserializediffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DeserializeDiffComponent)

/ Implementations MUST parse JSON strings into diff instances.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️serializeidcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/SerializeIdComponent)

/ Implementations MUST convert entity IDs to valid JSON strings.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️deserializeidcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DeserializeIdComponent)

/ Implementations MUST parse JSON strings into entity ID instances.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entitygoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityGoo)

/ Implementations MUST validate entities before exposing them downstream.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entityparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityParam)

/ Implementations MUST enforce entity validation on parameter access.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entitycomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityComponent)

/ Implementations MUST validate constructed entities before output.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entityidgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityIdGoo)

/ Implementations MUST validate entity IDs before exposing them downstream.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entityidparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityIdParam)

/ Implementations MUST enforce entity ID validation on parameter access.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entityidcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityIdComponent)

/ Implementations MUST validate constructed entity IDs before output.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entitydiffgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityDiffGoo)

/ Implementations MUST validate entity diffs before exposing them downstream.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entitydiffparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityDiffParam)

/ Implementations MUST enforce entity diff validation on parameter access.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entitydiffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityDiffComponent)

/ Implementations MUST validate constructed entity diffs before output.

## [👤semio📚gh🛅semiograsshopper💻buildvalueliststs🔖valuelistgeneration](semiorepo://section/Value%20List%20Generation)

Value list generation script. MUST convert CSV data into Grasshopper value list text files.

## [👤semio📚gh🛅semiograsshopper💻buildvalueliststs🪨builddir](semiorepo://definition/semio/gh/Semio.Grasshopper/build-value-lists.ts/buildDir)

MUST be created if it does not exist.

## [👤semio📚gh🛅semiograsshopper💻buildvalueliststs🛠️convertcsvtovaluelist](semiorepo://definition/semio/gh/Semio.Grasshopper/build-value-lists.ts/convertCsvToValueList)

MUST read the CSV, extract key-value pairs, and write the output file.

## [👤semio📚gh🛅semiograsshopper💻buildts🔖build](semiorepo://section/Build)

Grasshopper build script. MUST compile the solution and copy artifacts to the Yak distribution folder.

## [👤semio📚gh🛅semiograsshopper💻buildts🪨cwd](semiorepo://definition/semio/gh/Semio.Grasshopper/build.ts/cwd)

MUST resolve to the Grasshopper project folder.

## [👤semio📚gh🛅semiograsshopper💻buildts🪨msbuild](semiorepo://definition/semio/gh/Semio.Grasshopper/build.ts/msbuild)

MUST point to the installed MSBuild binary.

## [👤semio📚gh🛅semiograsshopper💻buildts🪨yakdistfolder](semiorepo://definition/semio/gh/Semio.Grasshopper/build.ts/yakDistFolder)

MUST be cleaned and recreated before copying build artifacts.

## [👤semio📚gh🛅semiograsshopper💻buildts🪨binfolder](semiorepo://definition/semio/gh/Semio.Grasshopper/build.ts/binFolder)

MUST contain the .NET Framework 4.8 build output.

## [👤semio📚gh🛅semiograsshopper💻buildts🪨files](semiorepo://definition/semio/gh/Semio.Grasshopper/build.ts/files)

MUST be copied to the Yak distribution folder.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻buildts🔖build](semiorepo://section/Build)

Yak package build script. MUST prepare the distribution folder and build the .yak package.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻buildts🪨cwd](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/build.ts/cwd)

MUST resolve to the yak folder.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻buildts🪨distdir](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/build.ts/distDir)

MUST be cleaned and prepared before building.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻buildts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/build.ts/yak)

MUST point to the installed Yak binary.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻logints🔖login](semiorepo://section/Login)

Yak login script. MUST authenticate with the Yak package manager.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻logints🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/login.ts/yak)

MUST point to the installed Yak binary.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🔖publish](semiorepo://section/Publish)

Yak publish script. MUST push the built package to the Yak server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨cwd](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/cwd)

MUST contain the manifest.yml and built .yak file.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨manifestcontent](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/manifestContent)

MUST contain a version field.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨versionmatch](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/versionMatch)

MUST successfully extract the version string.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨version](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/version)

MUST be trimmed of whitespace.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨buildname](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/buildName)

MUST match the built package name pattern.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/yak)

MUST point to the installed Yak binary.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testpushts🔖testpush](semiorepo://section/Test%20Push)

Yak test push script. MUST push the package to the test Yak server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testpushts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/test-push.ts/yak)

MUST point to the installed Yak binary.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testpushts🪨packagefile](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/test-push.ts/packageFile)

MUST resolve to a valid .yak package file.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testsearchts🔖script](semiorepo://section/Script)

Script MUST execute yak search against the test.yak.rhino3d.com server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testsearchts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/test-search.ts/yak)

Yak path MUST point to the Rhino 8 System directory.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻unyankts🔖unyank](semiorepo://section/Unyank)

Yak unyank script. MUST restore a previously yanked package version.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻unyankts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/unyank.ts/yak)

MUST point to the installed Yak binary.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻unyankts🪨version](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/unyank.ts/version)

MUST be a valid semver version string.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻yankts🔖yank](semiorepo://section/Yank)

Yak yank script. MUST remove a package version from the Yak server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻yankts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/yank.ts/yak)

MUST point to the installed Yak binary.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻yankts🪨version](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/yank.ts/version)

MUST be a valid semver version string.

## [👤semio📚go💻kitsqlitego🔖sqlitekitoperations](semiorepo://section/SQLite%20Kit%20Operations)

SQLite kit operations. MUST provide serialization and deserialization of Kit to and from SQLite and zip formats.

## [👤semio📚go💻kitsqlitego🛠️kitfromsqlite](semiorepo://definition/semio/go/kit_sqlite.go/KitFromSqlite)

Callers MUST provide a valid path to an existing SQLite database

## [👤semio📚go💻kitsqlitego🛠️loadtypes](semiorepo://definition/semio/go/kit_sqlite.go/loadTypes)

Callers MUST provide a valid open database connection and kit GUID

## [👤semio📚go💻kitsqlitego🛠️loaddesigns](semiorepo://definition/semio/go/kit_sqlite.go/loadDesigns)

Callers MUST provide a valid open database connection and kit GUID

## [👤semio📚go💻kitsqlitego🛠️loadpieces](semiorepo://definition/semio/go/kit_sqlite.go/loadPieces)

Callers MUST provide a valid open database connection and design GUID

## [👤semio📚go💻kitsqlitego🛠️loadconnections](semiorepo://definition/semio/go/kit_sqlite.go/loadConnections)

Callers MUST provide a valid open database connection and design GUID

## [👤semio📚go💻kitsqlitego🛠️loadconnectors](semiorepo://definition/semio/go/kit_sqlite.go/loadConnectors)

Callers MUST provide a valid open database connection and type GUID

## [👤semio📚go💻kitsqlitego🛠️kittosqlite](semiorepo://definition/semio/go/kit_sqlite.go/KitToSqlite)

Callers MUST provide a valid Kit, writable database path, and schema SQL

## [👤semio📚go💻kitsqlitego🛠️kitfromzip](semiorepo://definition/semio/go/kit_sqlite.go/KitFromZip)

Callers MUST provide a valid path to an existing zip file containing kit.db

## [👤semio📚go💻kitsqlitego🛠️kittozip](semiorepo://definition/semio/go/kit_sqlite.go/KitToZip)

Callers MUST provide a valid Kit, file map, writable zip path, and schema SQL

## [👤semio📚go💻semiogo🔖imports](semiorepo://section/Imports)

Imports MUST include all required packages for the semio domain library.

## [👤semio📚go💻semiogo🔖constants](semiorepo://section/Constants)

Constants MUST define shared constant values for the semio domain.

## [👤semio📚go💻semiogo🔖utils](semiorepo://section/Utils)

Utils MUST provide general-purpose utility functions for the semio domain.
Guid MUST return a cryptographically random 128-bit hex string.

## [👤semio📚go💻semiogo🔖entityids](semiorepo://section/Entity%20IDs)

Entity IDs MUST define identifier types for all semio domain entities.

## [👤semio📚go💻semiogo🔖weakentities](semiorepo://section/Weak%20Entities)

Weak Entities MUST define value types that exist only as part of parent entities.

## [👤semio📚go💻semiogo🔖attribute](semiorepo://section/Attribute)

Attribute MUST define the key-value metadata entity and its diff types.

## [👤semio📚go💻semiogo🔖location](semiorepo://section/Location)

Location MUST define geographic location entities and their diff types.

## [👤semio📚go💻semiogo🔖author](semiorepo://section/Author)

Author MUST define authorship entities and their diff types.

## [👤semio📚go💻semiogo🔖file](semiorepo://section/File)

File MUST define file reference entities and their diff types.

## [👤semio📚go💻semiogo🔖folder](semiorepo://section/Folder)

Folder MUST define folder hierarchy entities and their diff types.

## [👤semio📚go💻semiogo🔖benchmark](semiorepo://section/Benchmark)

Benchmark MUST define benchmark threshold entities and their diff types.

## [👤semio📚go💻semiogo🔖quality](semiorepo://section/Quality)

Quality MUST define measurable quality entities and their diff types.

## [👤semio📚go💻semiogo🔖port](semiorepo://section/Port)

Port MUST define connector port entities and their diff types.

## [👤semio📚go💻semiogo🔖prop](semiorepo://section/Prop)

Prop MUST define property value entities and their diff types.

## [👤semio📚go💻semiogo🔖tag](semiorepo://section/Tag)

Tag MUST define tag classification entities and their diff types.

## [👤semio📚go💻semiogo🔖concept](semiorepo://section/Concept)

Concept MUST define concept categorization entities and their diff types.

## [👤semio📚go💻semiogo🔖model](semiorepo://section/Model)

Model MUST define 3D model reference entities and their diff types.

## [👤semio📚go💻semiogo🔖connector](semiorepo://section/Connector)

Connector MUST define spatial connector entities and their diff types.

## [👤semio📚go💻semiogo🔖type](semiorepo://section/Type)

Type MUST define component type entities and their diff types.

## [👤semio📚go💻semiogo🔖layer](semiorepo://section/Layer)

Layer MUST define layer hierarchy entities and their diff types.

## [👤semio📚go💻semiogo🔖piece](semiorepo://section/Piece)

Piece MUST define placed piece entities and their diff types.

## [👤semio📚go💻semiogo🔖group](semiorepo://section/Group)

Group MUST define piece grouping entities and their diff types.

## [👤semio📚go💻semiogo🔖side](semiorepo://section/Side)

Side MUST define connection side reference entities and their diff types.

## [👤semio📚go💻semiogo🔖connection](semiorepo://section/Connection)

Connection MUST define spatial connection entities and their diff types.

## [👤semio📚go💻semiogo🔖stat](semiorepo://section/Stat)

Stat MUST define statistical measure entities and their diff types.

## [👤semio📚go💻semiogo🔖design](semiorepo://section/Design)

Design MUST define assembly design entities and their diff types.

## [👤semio📚go💻semiogo🔖kit](semiorepo://section/Kit)

Kit MUST define the root kit container entity and its diff types.

## [👤semio📚go💻semiogo🔖serialization](semiorepo://section/Serialization)

Serialization MUST provide JSON marshaling and unmarshaling for kit data.
SerializeKit MUST return valid JSON with two-space indentation.

## [👤semio📚go💻semiogo🔖helpers](semiorepo://section/Helpers)

Helpers MUST provide lookup functions for finding entities within kits.
FindTypeInKit MUST return nil when no type matches the GUID.

## [👤semio📚go💻semiogo🔖factories](semiorepo://section/Factories)

Factories MUST provide constructor functions for creating new domain entities.
NewKit MUST generate a unique GUID and set version to 0.0.1.

## [👤semio📚go💻semiogo🔖kitoperations](semiorepo://section/Kit%20Operations)

Kit Operations MUST provide comparison, diffing, and application of kit changes.
AreKitsEqual MUST compare all entities by GUID and structural fields.

## [👤semio📚go💻semiogo🔖kitdiffhelpers](semiorepo://section/Kit%20Diff%20Helpers)

Kit Diff Helpers MUST provide convenience functions for single-entity kit diffs.
AddTypeToKit MUST return a diff with exactly one added type.

## [👤semio📚go💻semiogo🔖validation](semiorepo://section/Validation)

Validation MUST provide constraint-based validation of kit data integrity.

## [👤semio📚go💻semiogo🔖validationserialization](semiorepo://section/Validation%20Serialization)

Validation Serialization MUST provide serializable representations of validation results.

## [👤semio📚go💻semiogo🔖flattendesign](semiorepo://section/Flatten%20Design)

Flatten Design MUST compute absolute piece planes from relative connections.

## [👤semio📚go💻semiogo🛠️guid](semiorepo://definition/semio/go/semio.go/Guid)

Guid MUST return a cryptographically random 128-bit hex string.

## [👤semio📚go💻semiogo🛠️normalize](semiorepo://definition/semio/go/semio.go/Normalize)

Normalize MUST trim whitespace and convert to lowercase.

## [👤semio📚go💻semiogo🛠️round](semiorepo://definition/semio/go/semio.go/Round)

Round MUST return the value rounded to exactly the given decimal places.

## [👤semio📚go💻semiogo🛠️deepequal](semiorepo://definition/semio/go/semio.go/DeepEqual)

DeepEqual MUST return true only when both values produce identical JSON.

## [👤semio📚go💻semiogo🛠️unmarshaljson](semiorepo://definition/semio/go/semio.go/UnmarshalJSON)

UnmarshalJSON MUST populate the setFields map for all present JSON keys.

## [👤semio📚go💻semiogo🛠️hasfield](semiorepo://definition/semio/go/semio.go/HasField)

HasField MUST return false when setFields is nil.

## [👤semio📚go💻semiogo🛠️unmarshaljson](semiorepo://definition/semio/go/semio.go/UnmarshalJSON)

UnmarshalJSON MUST populate the setFields map for all present JSON keys.

## [👤semio📚go💻semiogo🛠️hasfield](semiorepo://definition/semio/go/semio.go/HasField)

HasField MUST return false when setFields is nil.

## [👤semio📚go💻semiogo🛠️unmarshaljson](semiorepo://definition/semio/go/semio.go/UnmarshalJSON)

UnmarshalJSON MUST populate the setFields map for all present JSON keys.

## [👤semio📚go💻semiogo🛠️hasfield](semiorepo://definition/semio/go/semio.go/HasField)

HasField MUST return false when setFields is nil.

## [👤semio📚go💻semiogo🛠️unmarshaljson](semiorepo://definition/semio/go/semio.go/UnmarshalJSON)

UnmarshalJSON MUST populate the setFields map for all present JSON keys.

## [👤semio📚go💻semiogo🛠️hasfield](semiorepo://definition/semio/go/semio.go/HasField)

HasField MUST return false when setFields is nil.

## [👤semio📚go💻semiogo🛠️serializekit](semiorepo://definition/semio/go/semio.go/SerializeKit)

SerializeKit MUST return valid JSON with two-space indentation.

## [👤semio📚go💻semiogo🛠️deserializekit](semiorepo://definition/semio/go/semio.go/DeserializeKit)

DeserializeKit MUST return an error if the data is not valid kit JSON.

## [👤semio📚go💻semiogo🛠️serializekitdiff](semiorepo://definition/semio/go/semio.go/SerializeKitDiff)

SerializeKitDiff MUST return valid JSON with two-space indentation.

## [👤semio📚go💻semiogo🛠️deserializekitdiff](semiorepo://definition/semio/go/semio.go/DeserializeKitDiff)

DeserializeKitDiff MUST return an error if the data is not valid kit diff JSON.

## [👤semio📚go💻semiogo🛠️findtypeinkit](semiorepo://definition/semio/go/semio.go/FindTypeInKit)

FindTypeInKit MUST return nil when no type matches the GUID.

## [👤semio📚go💻semiogo🛠️finddesigninkit](semiorepo://definition/semio/go/semio.go/FindDesignInKit)

FindDesignInKit MUST return nil when no design matches the GUID.

## [👤semio📚go💻semiogo🛠️findpieceindesign](semiorepo://definition/semio/go/semio.go/FindPieceInDesign)

FindPieceInDesign MUST return nil when no piece matches the GUID.

## [👤semio📚go💻semiogo🛠️findconnectionindesign](semiorepo://definition/semio/go/semio.go/FindConnectionInDesign)

FindConnectionInDesign MUST return nil when no connection matches the GUID.

## [👤semio📚go💻semiogo🛠️findconnectorintype](semiorepo://definition/semio/go/semio.go/FindConnectorInType)

FindConnectorInType MUST return nil when no connector matches the GUID.

## [👤semio📚go💻semiogo🛠️findfileinkit](semiorepo://definition/semio/go/semio.go/FindFileInKit)

FindFileInKit MUST return nil when no file matches the GUID.

## [👤semio📚go💻semiogo🛠️findfolderinkit](semiorepo://definition/semio/go/semio.go/FindFolderInKit)

FindFolderInKit MUST return nil when no folder matches the GUID.

## [👤semio📚go💻semiogo🛠️findqualityinkit](semiorepo://definition/semio/go/semio.go/FindQualityInKit)

FindQualityInKit MUST return nil when no quality matches the GUID.

## [👤semio📚go💻semiogo🛠️findportinkit](semiorepo://definition/semio/go/semio.go/FindPortInKit)

FindPortInKit MUST return nil when no port matches the GUID.

## [👤semio📚go💻semiogo🛠️findtaginkit](semiorepo://definition/semio/go/semio.go/FindTagInKit)

FindTagInKit MUST return nil when no tag matches the GUID.

## [👤semio📚go💻semiogo🛠️findconceptinkit](semiorepo://definition/semio/go/semio.go/FindConceptInKit)

FindConceptInKit MUST return nil when no concept matches the GUID.

## [👤semio📚go💻semiogo🛠️findauthorinkit](semiorepo://definition/semio/go/semio.go/FindAuthorInKit)

FindAuthorInKit MUST return nil when no author matches the GUID.

## [👤semio📚go💻semiogo🛠️newkit](semiorepo://definition/semio/go/semio.go/NewKit)

NewKit MUST generate a unique GUID and set version to 0.0.1.

## [👤semio📚go💻semiogo🛠️newtype](semiorepo://definition/semio/go/semio.go/NewType)

NewType MUST generate a unique GUID for the new type.

## [👤semio📚go💻semiogo🛠️newdesign](semiorepo://definition/semio/go/semio.go/NewDesign)

NewDesign MUST generate a unique GUID for the new design.

## [👤semio📚go💻semiogo🛠️newpiece](semiorepo://definition/semio/go/semio.go/NewPiece)

NewPiece MUST generate a unique GUID for the new piece.

## [👤semio📚go💻semiogo🛠️newconnection](semiorepo://definition/semio/go/semio.go/NewConnection)

NewConnection MUST generate a unique GUID and set both connected and connecting sides.

## [👤semio📚go💻semiogo🛠️newconnector](semiorepo://definition/semio/go/semio.go/NewConnector)

NewConnector MUST generate a unique GUID for the new connector.

## [👤semio📚go💻semiogo🛠️newfile](semiorepo://definition/semio/go/semio.go/NewFile)

NewFile MUST generate a unique GUID for the new file.

## [👤semio📚go💻semiogo🛠️newfolder](semiorepo://definition/semio/go/semio.go/NewFolder)

NewFolder MUST generate a unique GUID for the new folder.

## [👤semio📚go💻semiogo🛠️newquality](semiorepo://definition/semio/go/semio.go/NewQuality)

NewQuality MUST generate a unique GUID for the new quality.

## [👤semio📚go💻semiogo🛠️newport](semiorepo://definition/semio/go/semio.go/NewPort)

NewPort MUST generate a unique GUID for the new port.

## [👤semio📚go💻semiogo🛠️newtag](semiorepo://definition/semio/go/semio.go/NewTag)

NewTag MUST generate a unique GUID for the new tag.

## [👤semio📚go💻semiogo🛠️newconcept](semiorepo://definition/semio/go/semio.go/NewConcept)

NewConcept MUST generate a unique GUID for the new concept.

## [👤semio📚go💻semiogo🛠️newauthor](semiorepo://definition/semio/go/semio.go/NewAuthor)

NewAuthor MUST generate a unique GUID for the new author.

## [👤semio📚go💻semiogo🛠️arekitsequal](semiorepo://definition/semio/go/semio.go/AreKitsEqual)

AreKitsEqual MUST compare all entities by GUID and structural fields.

## [👤semio📚go💻semiogo🛠️arekitdiffsequal](semiorepo://definition/semio/go/semio.go/AreKitDiffsEqual)

AreKitDiffsEqual MUST compare all diff fields including nested entity diffs.

## [👤semio📚go💻semiogo🛠️getkitdiff](semiorepo://definition/semio/go/semio.go/GetKitDiff)

GetKitDiff MUST return a diff that when applied to before produces after.

## [👤semio📚go💻semiogo🛠️inversekitdiff](semiorepo://definition/semio/go/semio.go/InverseKitDiff)

InverseKitDiff MUST return a diff that when applied restores the original state.

## [👤semio📚go💻semiogo🛠️applykitdiff](semiorepo://definition/semio/go/semio.go/ApplyKitDiff)

ApplyKitDiff MUST apply all additions, removals and updates from the diff.

## [👤semio📚go💻semiogo🛠️filterdesignswithoutparent](semiorepo://definition/semio/go/semio.go/FilterDesignsWithoutParent)

FilterDesignsWithoutParent MUST exclude all designs that have a non-nil parent.

## [👤semio📚go💻semiogo🛠️addtypetokit](semiorepo://definition/semio/go/semio.go/AddTypeToKit)

AddTypeToKit MUST return a diff with exactly one added type.

## [👤semio📚go💻semiogo🛠️removetypefromkit](semiorepo://definition/semio/go/semio.go/RemoveTypeFromKit)

RemoveTypeFromKit MUST return a diff with exactly one removed type ID.

## [👤semio📚go💻semiogo🛠️adddesigntokit](semiorepo://definition/semio/go/semio.go/AddDesignToKit)

AddDesignToKit MUST return a diff with exactly one added design.

## [👤semio📚go💻semiogo🛠️removedesignfromkit](semiorepo://definition/semio/go/semio.go/RemoveDesignFromKit)

RemoveDesignFromKit MUST return a diff with exactly one removed design ID.

## [👤semio📚go💻semiogo🛠️addfiletokit](semiorepo://definition/semio/go/semio.go/AddFileToKit)

AddFileToKit MUST return a diff with exactly one added file.

## [👤semio📚go💻semiogo🛠️removefilefromkit](semiorepo://definition/semio/go/semio.go/RemoveFileFromKit)

RemoveFileFromKit MUST return a diff with exactly one removed file ID.

## [👤semio📚go💻semiogo🛠️addporttokit](semiorepo://definition/semio/go/semio.go/AddPortToKit)

AddPortToKit MUST return a diff with exactly one added port.

## [👤semio📚go💻semiogo🛠️removeportfromkit](semiorepo://definition/semio/go/semio.go/RemovePortFromKit)

RemovePortFromKit MUST return a diff with exactly one removed port ID.

## [👤semio📚go💻semiogo🛠️addtagtokit](semiorepo://definition/semio/go/semio.go/AddTagToKit)

AddTagToKit MUST return a diff with exactly one added tag.

## [👤semio📚go💻semiogo🛠️removetagfromkit](semiorepo://definition/semio/go/semio.go/RemoveTagFromKit)

RemoveTagFromKit MUST return a diff with exactly one removed tag ID.

## [👤semio📚go💻semiogo🛠️addconcepttokit](semiorepo://definition/semio/go/semio.go/AddConceptToKit)

AddConceptToKit MUST return a diff with exactly one added concept.

## [👤semio📚go💻semiogo🛠️removeconceptfromkit](semiorepo://definition/semio/go/semio.go/RemoveConceptFromKit)

RemoveConceptFromKit MUST return a diff with exactly one removed concept ID.

## [👤semio📚go💻semiogo🛠️guiduniquenessconstraint](semiorepo://definition/semio/go/semio.go/GuidUniquenessConstraint)

GuidUniquenessConstraint MUST report each duplicate GUID as a separate problem.

## [👤semio📚go💻semiogo🛠️typenameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/TypeNameUniquenessConstraint)

TypeNameUniquenessConstraint MUST report duplicate names among types with the same parent.

## [👤semio📚go💻semiogo🛠️designnameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/DesignNameUniquenessConstraint)

DesignNameUniquenessConstraint MUST report duplicate names among designs with the same parent.

## [👤semio📚go💻semiogo🛠️piecenameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/PieceNameUniquenessConstraint)

PieceNameUniquenessConstraint MUST report duplicate piece names within each design.

## [👤semio📚go💻semiogo🛠️qualitynameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/QualityNameUniquenessConstraint)

QualityNameUniquenessConstraint MUST report each duplicate quality name.

## [👤semio📚go💻semiogo🛠️portnameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/PortNameUniquenessConstraint)

PortNameUniquenessConstraint MUST report each duplicate port name.

## [👤semio📚go💻semiogo🛠️filenameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/FileNameUniquenessConstraint)

FileNameUniquenessConstraint MUST report each duplicate file name.

## [👤semio📚go💻semiogo🛠️foldernameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/FolderNameUniquenessConstraint)

FolderNameUniquenessConstraint MUST report duplicate names among folders with the same parent.

## [👤semio📚go💻semiogo🛠️connectornameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/ConnectorNameUniquenessConstraint)

ConnectorNameUniquenessConstraint MUST report duplicate connector names within each type.

## [👤semio📚go💻semiogo🛠️modelnameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/ModelNameUniquenessConstraint)

ModelNameUniquenessConstraint MUST report duplicate model names within each type.

## [👤semio📚go💻semiogo🛠️layerpathuniquenessconstraint](semiorepo://definition/semio/go/semio.go/LayerPathUniquenessConstraint)

LayerPathUniquenessConstraint MUST report duplicate layer paths within each design.

## [👤semio📚go💻semiogo🛠️validatekit](semiorepo://definition/semio/go/semio.go/ValidateKit)

ValidateKit MUST apply all default constraints and return all found problems.

## [👤semio📚go💻semiogo🛠️validatekitwithconstraints](semiorepo://definition/semio/go/semio.go/ValidateKitWithConstraints)

ValidateKitWithConstraints MUST apply each constraint and aggregate all problems.

## [👤semio📚go💻semiogo🛠️haserrors](semiorepo://definition/semio/go/semio.go/HasErrors)

HasErrors MUST return true when any problem has error severity or empty severity.

## [👤semio📚go💻semiogo🛠️tovalidationresult](semiorepo://definition/semio/go/semio.go/ToValidationResult)

ToValidationResult MUST default empty severity to error.

## [👤semio📚go💻semiogo🛠️arevalidationresultsequal](semiorepo://definition/semio/go/semio.go/AreValidationResultsEqual)

AreValidationResultsEqual MUST compare problems regardless of their order.

## [👤semio📚go💻semiogo🛠️flattendesign](semiorepo://definition/semio/go/semio.go/FlattenDesign)

FlattenDesign MUST traverse the connection graph via BFS to compute piece transforms.

## [👤semio📚go💻semiogo🛠️applydesigndiff](semiorepo://definition/semio/go/semio.go/ApplyDesignDiff)

ApplyDesignDiff MUST apply all piece, connection and property changes from the diff.

## [👤semio📚js💻devts🔖dev](semiorepo://section/Dev)

MUST kill both child processes on SIGINT and SIGTERM.

## [👤semio📚js💻devts🪨iswindows](semiorepo://definition/semio/js/dev.ts/isWindows)

MUST be checked before spawning npm commands.

## [👤semio📚js💻devts🪨npmcmd](semiorepo://definition/semio/js/dev.ts/npmCmd)

MUST use .cmd extension on Windows.

## [👤semio📚js💻devts🪨vite](semiorepo://definition/semio/js/dev.ts/vite)

MUST inherit stdio for live output.

## [👤semio📚js💻devts🪨storybook](semiorepo://definition/semio/js/dev.ts/storybook)

MUST inherit stdio for live output.

## [👤semio📚js💻i18nts🔖i18n](semiorepo://section/I18n)

MUST fall back to English when the detected language is unavailable.

## [👤semio📚js💻i18nts🪨getexpertisefunction](semiorepo://definition/semio/js/i18n.ts/getExpertiseFunction)

MUST be set via setExpertiseProvider before expertise-dependent labels are resolved.

## [👤semio📚js💻indexts🔖exports](semiorepo://section/Exports)

MUST re-export all public types alongside their runtime counterparts.

## [👤semio📚js💻semiots🔖imports](semiorepo://section/Imports)

External dependency imports MUST be declared here.

## [👤semio📚js💻semiots🔖constants](semiorepo://section/Constants)

Global constants MUST define shared numeric parameters.

## [👤semio📚js💻semiots🔖utilities](semiorepo://section/Utilities)

General-purpose utility functions MUST be defined here.

## [👤semio📚js💻semiots🔖entityids](semiorepo://section/Entity%20IDs)

Entity identifier types and comparison functions MUST be defined here.

## [👤semio📚js💻semiots🔖attribute](semiorepo://section/Attribute)

Attribute entity types, schemas, and helper functions MUST be defined here.

## [👤semio📚js💻semiots🔖coordweakentity](semiorepo://section/Coord%20(weak%20entity))

Coord weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semiots🔖vecweakentity](semiorepo://section/Vec%20(weak%20entity))

Vec weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semiots🔖pointweakentity](semiorepo://section/Point%20(weak%20entity))

Point weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semiots🔖vectorweakentity](semiorepo://section/Vector%20(weak%20entity))

Vector weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semiots🔖planeweakentity](semiorepo://section/Plane%20(weak%20entity))

Plane weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semiots🔖cameraweakentity](semiorepo://section/Camera%20(weak%20entity))

Camera weak entity types and schemas MUST be defined here.

## [👤semio📚js💻semiots🔖location](semiorepo://section/Location)

Location entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖author](semiorepo://section/Author)

Author entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖file](semiorepo://section/File)

File entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖folder](semiorepo://section/Folder)

Folder entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖benchmark](semiorepo://section/Benchmark)

Benchmark entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖quality](semiorepo://section/Quality)

Quality entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖port](semiorepo://section/Port)

Port entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖prop](semiorepo://section/Prop)

Prop entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖tag](semiorepo://section/Tag)

Tag entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖concept](semiorepo://section/Concept)

Concept entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖model](semiorepo://section/Model)

Model entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖connector](semiorepo://section/Connector)

Connector entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖type](semiorepo://section/Type)

Type entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖layer](semiorepo://section/Layer)

Layer entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖piece](semiorepo://section/Piece)

Piece entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖group](semiorepo://section/Group)

Group entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖side](semiorepo://section/Side)

Side entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖connection](semiorepo://section/Connection)

Connection entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖stat](semiorepo://section/Stat)

Stat entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖design](semiorepo://section/Design)

Design entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖kit](semiorepo://section/Kit)

Kit entity types, schemas, and helpers MUST be defined here.

## [👤semio📚js💻semiots🔖designfamilyhelpers](semiorepo://section/Design%20Family%20Helpers)

Design family traversal helpers MUST be defined here.

## [👤semio📚js💻semiots🔖typefamilyhelpers](semiorepo://section/Type%20Family%20Helpers)

Type family traversal helpers MUST be defined here.

## [👤semio📚js💻semiots🔖filetreeutilities](semiorepo://section/File%20Tree%20Utilities)

File tree construction and traversal utilities MUST be defined here.

## [👤semio📚js💻semiots🔖kitimportexport](semiorepo://section/Kit%20Import/Export)

Kit serialization and deserialization functions MUST be defined here.

## [👤semio📚js💻semiots🔖validation](semiorepo://section/Validation)

Kit validation engine and constraints MUST be defined here.
Core validation types and interfaces MUST be defined here.

## [👤semio📚js💻semiots🔖validationcoretypes](semiorepo://section/Validation%20core%20types)

Core validation types and interfaces MUST be defined here.

## [👤semio📚js💻semiots🔖validationcontextengine](semiorepo://section/Validation%20context%20&%20engine)

Validation context construction and engine MUST be defined here.

## [👤semio📚js💻semiots🔖fixhelper](semiorepo://section/Fix%20helper)

Validation fix helper functions MUST be defined here.

## [👤semio📚js💻semiots🔖guidupdatehelper](semiorepo://section/GUID%20update%20helper)

GUID regeneration helper functions MUST be defined here.

## [👤semio📚js💻semiots🔖constraintguiduniqueness](semiorepo://section/Constraint:%20GUID%20uniqueness)

GUID uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constrainttypenameuniqueness](semiorepo://section/Constraint:%20Type%20name%20uniqueness)

Type name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintdesignnameuniqueness](semiorepo://section/Constraint:%20Design%20name%20uniqueness)

Design name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintpiecenameuniqueness](semiorepo://section/Constraint:%20Piece%20name%20uniqueness)

Piece name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintqualitynameuniqueness](semiorepo://section/Constraint:%20Quality%20name%20uniqueness)

Quality name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintportnameuniqueness](semiorepo://section/Constraint:%20Port%20name%20uniqueness)

Port name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintfilenameuniqueness](semiorepo://section/Constraint:%20File%20name%20uniqueness)

File name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintfoldernameuniqueness](semiorepo://section/Constraint:%20Folder%20name%20uniqueness)

Folder name uniqueness constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintconnectornameuniquenesswithintype](semiorepo://section/Constraint:%20Connector%20name%20uniqueness%20within%20type)

Connector name uniqueness within type constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintmodelnameuniquenesswithintype](semiorepo://section/Constraint:%20Model%20name%20uniqueness%20within%20type)

Model name uniqueness within type constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintlayerpathuniquenesswithindesign](semiorepo://section/Constraint:%20Layer%20path%20uniqueness%20within%20design)

Layer path uniqueness within design constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintdesignpiecesamefamilyconstraint](semiorepo://section/Constraint:%20Design%20piece%20same%20family%20constraint)

Design piece same family constraint MUST be enforced here.

## [👤semio📚js💻semiots🔖constraintregistration](semiorepo://section/Constraint%20registration)

Constraint registration and default configurations MUST be defined here.

## [👤semio📚js💻semiots🔖validationserialization](semiorepo://section/Validation%20serialization)

Validation result serialization and deserialization MUST be defined here.

## [👤semio📚js💻sitetsx🔖entrypoint](semiorepo://section/Entrypoint)

Entrypoint MUST render into the root element defined in index.html.

## [👤semio📚js🗃️sketchpad💻designtsx🔖imports](semiorepo://section/Imports)

Imports for Design app MUST include all shared sketchpad, React, and UI dependencies.

## [👤semio📚js🗃️sketchpad💻designtsx🔖statemanagement](semiorepo://section/State%20Management)

State management types and interfaces MUST define the Design app selection, presence, hover, diff, and state shape.

## [👤semio📚js🗃️sketchpad💻designtsx🔖commands](semiorepo://section/Commands)

Commands MUST define all executable Design app actions dispatched by keyboard shortcuts and UI interactions.

## [👤semio📚js🗃️sketchpad💻designtsx🔖store](semiorepo://section/Store)

Store MUST implement DesignStore extending PlainKitDiffAppStore with undo/redo, selection diff inversion, and state persistence.

## [👤semio📚js🗃️sketchpad💻designtsx🔖designapppluginregistration](semiorepo://section/Design%20App%20Plugin%20Registration)

Design app plugin registration MUST register the Design app plugin with machine actions, guards, and default state.

## [👤semio📚js🗃️sketchpad💻designtsx🔖hooks](semiorepo://section/Hooks)

Hooks MUST provide the Design app initialization lifecycle within the React component tree.

## [👤semio📚js🗃️sketchpad💻designtsx🔖components](semiorepo://section/Components)

Components MUST provide Design app scope, actor context, and synchronization wrapper components.

## [👤semio📚js🗃️sketchpad💻designtsx🔖actionhooks](semiorepo://section/Action%20Hooks)

Action hooks MUST provide composable React hooks for Design app selection, hover, focus, panel, and transaction actions.

## [👤semio📚js🗃️sketchpad💻designtsx🔖footer](semiorepo://section/Footer)

Footer MUST render dynamic Design app footer items showing selection and transaction state.

## [👤semio📚js🗃️sketchpad💻designtsx🔖tools](semiorepo://section/Tools)

Tools MUST define all Design app tool configurations for selection, lasso, and hand modes.

## [👤semio📚js🗃️sketchpad💻designtsx🔖panels](semiorepo://section/Panels)

WindowLibrary MUST provide draggable window templates for adding scene, diagram, and table windows.

## [👤semio📚js🗃️sketchpad💻designtsx🔖windowlibrary](semiorepo://section/WindowLibrary)

WindowLibrary MUST provide draggable window templates for adding scene, diagram, and table windows.

## [👤semio📚js🗃️sketchpad💻designtsx🔖details](semiorepo://section/Details)

Details MUST render the Design app detail panels for design, pieces, connections, and connector sections.

## [👤semio📚js🗃️sketchpad💻designtsx🔖canvas](semiorepo://section/Canvas)

Hover Intent Context MUST manage debounced hover state to prevent flickering during rapid mouse movement.

## [👤semio📚js🗃️sketchpad💻designtsx🔖hoverintentcontext](semiorepo://section/Hover%20Intent%20Context)

Hover Intent Context MUST manage debounced hover state to prevent flickering during rapid mouse movement.

## [👤semio📚js🗃️sketchpad💻designtsx🔖diagram](semiorepo://section/Diagram)

Diagram MUST render the interactive React Flow design diagram with nodes, edges, minimap, and controls.

## [👤semio📚js🗃️sketchpad💻designtsx🔖scene](semiorepo://section/Scene)

Scene MUST render the Three.js 3D scene view of design pieces with selection and hover highlighting.

## [👤semio📚js🗃️sketchpad💻designtsx🔖windows](semiorepo://section/Windows)

Window components MUST wrap diagram and scene views with hover and transaction providers.

## [👤semio📚js🗃️sketchpad💻designtsx🔖app](semiorepo://section/App)

App MUST compose all Design app panels, canvas, toolbar, and footer into the main Design app layout.

## [👤semio📚js🗃️sketchpad💻designtsx🔖settings](semiorepo://section/Settings)

Settings MUST render the Design app settings panel with theme, language, device, expertise, and mode toggles.

## [👤semio📚js🗃️sketchpad💻designtsx🔖config](semiorepo://section/Config)

Config MUST export the Design app configuration with route segments, panel definitions, and path matching.

## [👤semio📚js🗃️sketchpad💻docstsx🔖imports](semiorepo://section/Imports)

External and internal module imports MUST be declared here.

## [👤semio📚js🗃️sketchpad💻docstsx🔖mdxloader](semiorepo://section/MDX%20Loader)

MDX file loading and section discovery utilities MUST be declared here.

## [👤semio📚js🗃️sketchpad💻docstsx🔖mdxprovider](semiorepo://section/MDX%20Provider)

MDX rendering context and heading components MUST be declared here.
Section tree navigation component MUST render docs file hierarchy.

## [👤semio📚js🗃️sketchpad💻docstsx🔖sectiontree](semiorepo://section/SectionTree)

Section tree navigation component MUST render docs file hierarchy.

## [👤semio📚js🗃️sketchpad💻docstsx🔖registry](semiorepo://section/Registry)

Docs registry MUST provide page and section lookup for navigation.

## [👤semio📚js🗃️sketchpad💻docstsx🔖store](semiorepo://section/Store)

Docs app section state MUST be declared here.

## [👤semio📚js🗃️sketchpad💻docstsx🔖types](semiorepo://section/Types)

Docs app state, selection, and diff type definitions MUST be declared here.

## [👤semio📚js🗃️sketchpad💻docstsx🔖docsappstore](semiorepo://section/Docs%20App%20Store)

Docs app store MUST extend PlainAppStore with docs-specific state management.

## [👤semio📚js🗃️sketchpad💻docstsx🔖commands](semiorepo://section/Commands)

Docs app command handlers MUST modify state through diff objects.

## [👤semio📚js🗃️sketchpad💻docstsx🔖docsapppluginregistration](semiorepo://section/Docs%20App%20Plugin%20Registration)

Plugin registration MUST initialize docs app context and registry.

## [👤semio📚js🗃️sketchpad💻docstsx🔖canvas](semiorepo://section/Canvas)

Canvas components MUST render the docs app visual content.
Window components MUST provide windowed views within the canvas.
Page window MUST render MDX content with navigation and heading extraction.

## [👤semio📚js🗃️sketchpad💻docstsx🔖windows](semiorepo://section/Windows)

Window components MUST provide windowed views within the canvas.
Page window MUST render MDX content with navigation and heading extraction.

## [👤semio📚js🗃️sketchpad💻docstsx🔖page](semiorepo://section/Page)

Page window MUST render MDX content with navigation and heading extraction.

## [👤semio📚js🗃️sketchpad💻docstsx🔖footer](semiorepo://section/Footer)

Footer component MUST manage docs app footer items.

## [👤semio📚js🗃️sketchpad💻docstsx🔖panels](semiorepo://section/Panels)

Panel components MUST render sidebar content for the docs app.

## [👤semio📚js🗃️sketchpad💻docstsx🔖app](semiorepo://section/App)

Docs app root component MUST compose MDX routing, panel sections, and layout.

## [👤semio📚js🗃️sketchpad💻docstsx🔖config](semiorepo://section/Config)

Docs app route, panel, and path matching configuration MUST be exported.

## [👤semio📚js🗃️sketchpad💻feedbacktsx🔖imports](semiorepo://section/Imports)

MUST import external and internal modules for the Feedback app.

## [👤semio📚js🗃️sketchpad💻feedbacktsx🔖feedbackapppluginregistration](semiorepo://section/Feedback%20App%20Plugin%20Registration)

MUST register the Feedback app plugin with default state and event handlers.

## [👤semio📚js🗃️sketchpad💻feedbacktsx🔖triadichooks](semiorepo://section/Triadic%20Hooks)

MUST provide triadic hooks for accessing and mutating Feedback app state.

## [👤semio📚js🗃️sketchpad💻feedbacktsx🔖components](semiorepo://section/Components)

MUST render feedback form for submitting bug reports and ideas.

## [👤semio📚js🗃️sketchpad💻feedbacktsx🔖form](semiorepo://section/Form)

MUST render feedback form for submitting bug reports and ideas.

## [👤semio📚js🗃️sketchpad💻feedbacktsx🔖app](semiorepo://section/App)

MUST integrate feedback app with toolbar and layout canvas.

## [👤semio📚js🗃️sketchpad💻feedbacktsx🔖config](semiorepo://section/Config)

MUST define app configuration for the Feedback app.

## [👤semio📚js🗃️sketchpad💻feedbacktsx🔖globalfooteritem](semiorepo://section/Global%20Footer%20Item)

MUST re-export the feedback icon for the footer item.

## [👤semio📚js🗃️sketchpad💻hometsx🔖imports](semiorepo://section/Imports)

External and internal module imports MUST be declared here.

## [👤semio📚js🗃️sketchpad💻hometsx🔖types](semiorepo://section/Types)

Home app type definitions MUST be declared here.

## [👤semio📚js🗃️sketchpad💻hometsx🔖homeapppluginregistration](semiorepo://section/Home%20App%20Plugin%20Registration)

Home app plugin and event handler registration MUST initialize XState context.

## [👤semio📚js🗃️sketchpad💻hometsx🔖hooksxstatebased](semiorepo://section/Hooks%20(XState-based))

XState-based hooks MUST re-export state selectors for the Home app.

## [👤semio📚js🗃️sketchpad💻hometsx🔖canvas](semiorepo://section/Canvas)

Canvas components MUST render the Home app visual content.
Window components MUST provide windowed views within the canvas.
Table window MUST display kit entries in tabular form.

## [👤semio📚js🗃️sketchpad💻hometsx🔖windows](semiorepo://section/Windows)

Window components MUST provide windowed views within the canvas.
Table window MUST display kit entries in tabular form.

## [👤semio📚js🗃️sketchpad💻hometsx🔖table](semiorepo://section/Table)

Table window MUST display kit entries in tabular form.

## [👤semio📚js🗃️sketchpad💻hometsx🔖panels](semiorepo://section/Panels)

Panel components MUST render sidebar panel content.
Right panel components MUST render details, chat, and settings.
Details panel MUST show properties of selected kits.

## [👤semio📚js🗃️sketchpad💻hometsx🔖right](semiorepo://section/Right)

Right panel components MUST render details, chat, and settings.
Details panel MUST show properties of selected kits.

## [👤semio📚js🗃️sketchpad💻hometsx🔖details](semiorepo://section/Details)

Details panel MUST show properties of selected kits.

## [👤semio📚js🗃️sketchpad💻hometsx🔖chat](semiorepo://section/Chat)

Chat panel MUST show the chat placeholder content.

## [👤semio📚js🗃️sketchpad💻hometsx🔖settings](semiorepo://section/Settings)

Settings panel MUST expose theme, language, device, expertise, and mode toggles.

## [👤semio📚js🗃️sketchpad💻hometsx🔖footer](semiorepo://section/Footer)

Footer component MUST manage Home app footer items.

## [👤semio📚js🗃️sketchpad💻hometsx🔖dropzone](semiorepo://section/DropZone)

DropZone component MUST handle drag-and-drop kit imports.

## [👤semio📚js🗃️sketchpad💻hometsx🔖app](semiorepo://section/App)

App components MUST compose the Home app toolbar, table, and logic.

## [👤semio📚js🗃️sketchpad💻hometsx🔖multiwindowapp](semiorepo://section/Multi-Window%20App)

Multi-window app MUST orchestrate the Home canvas and layout.

## [👤semio📚js🗃️sketchpad💻hometsx🔖config](semiorepo://section/Config)

Config MUST define the Home app registration and panel setup.

## [👤semio📚js🗃️sketchpad💻kittsx🔖imports](semiorepo://section/Imports)

Imports for Kit app MUST include all shared sketchpad, React, DnD, and UI dependencies.

## [👤semio📚js🗃️sketchpad💻kittsx🔖designfamilyhelpers](semiorepo://section/Design%20Family%20Helpers)

Design family helper functions MUST traverse the design hierarchy to collect related design GUIDs.

## [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement](semiorepo://section/Internal%20State%20Management)

Constants MUST define artifact kinds and toolbar sub-tool configurations for the Kit app.

## [👤semio📚js🗃️sketchpad💻kittsx🔖constants](semiorepo://section/Constants)

Constants MUST define artifact kinds and toolbar sub-tool configurations for the Kit app.

## [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement](semiorepo://section/Internal%20State%20Management)

Internal state management MUST define all Kit app types, interfaces, store, and Y.js synchronization.

## [👤semio📚js🗃️sketchpad💻kittsx🔖kitapppluginregistration](semiorepo://section/Kit%20App%20Plugin%20Registration)

Kit app plugin registration MUST register the Kit app plugin with machine actions, guards, and default state.

## [👤semio📚js🗃️sketchpad💻kittsx🔖actionhooks](semiorepo://section/Action%20Hooks)

Action hooks MUST provide composable React hooks for Kit app selection, hover, sort, filter, and transaction actions.

## [👤semio📚js🗃️sketchpad💻kittsx🔖selectionhelperhooks](semiorepo://section/Selection%20Helper%20Hooks)

Selection helper hooks MUST provide entity-specific add, remove, toggle, select-single, select-all, and clear operations.

## [👤semio📚js🗃️sketchpad💻kittsx🔖typesselectionhooks](semiorepo://section/Types%20Selection%20Hooks)

Types selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for type selection.

## [👤semio📚js🗃️sketchpad💻kittsx🔖designsselectionhooks](semiorepo://section/Designs%20Selection%20Hooks)

Designs selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for design selection.

## [👤semio📚js🗃️sketchpad💻kittsx🔖qualitiesselectionhooks](semiorepo://section/Qualities%20Selection%20Hooks)

Qualities selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for quality selection.

## [👤semio📚js🗃️sketchpad💻kittsx🔖portsselectionhooks](semiorepo://section/Ports%20Selection%20Hooks)

Ports selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for port selection.

## [👤semio📚js🗃️sketchpad💻kittsx🔖tagsselectionhooks](semiorepo://section/Tags%20Selection%20Hooks)

Tags selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for tag selection.

## [👤semio📚js🗃️sketchpad💻kittsx🔖conceptsselectionhooks](semiorepo://section/Concepts%20Selection%20Hooks)

Concepts selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for concept selection.

## [👤semio📚js🗃️sketchpad💻kittsx🔖filesselectionhooks](semiorepo://section/Files%20Selection%20Hooks)

Files selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for file selection.

## [👤semio📚js🗃️sketchpad💻kittsx🔖foldersselectionhooks](semiorepo://section/Folders%20Selection%20Hooks)

Folders selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for folder selection.

## [👤semio📚js🗃️sketchpad💻kittsx🔖authorsselectionhooks](semiorepo://section/Authors%20Selection%20Hooks)

Authors selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for author selection.

## [👤semio📚js🗃️sketchpad💻kittsx🔖globalselectionhooks](semiorepo://section/Global%20Selection%20Hooks)

Global selection hooks MUST provide select-all across all artifact kinds.

## [👤semio📚js🗃️sketchpad💻kittsx🔖types](semiorepo://section/Types)

Types MUST provide hover status and color hooks for type visual indication in the Kit app.

## [👤semio📚js🗃️sketchpad💻kittsx🔖designs](semiorepo://section/Designs)

Designs MUST provide hover status and color hooks for design visual indication in the Kit app.

## [👤semio📚js🗃️sketchpad💻kittsx🔖commands](semiorepo://section/Commands)

Commands MUST define all executable Kit app actions for artifact CRUD, import, and export.

## [👤semio📚js🗃️sketchpad💻kittsx🔖canvas](semiorepo://section/Canvas)

Table MUST render the interactive data table with sortable columns, expandable rows, and drag-drop reordering.

## [👤semio📚js🗃️sketchpad💻kittsx🔖windows](semiorepo://section/Windows)

Table MUST render the interactive data table with sortable columns, expandable rows, and drag-drop reordering.

## [👤semio📚js🗃️sketchpad💻kittsx🔖table](semiorepo://section/Table)

Table MUST render the interactive data table with sortable columns, expandable rows, and drag-drop reordering.

## [👤semio📚js🗃️sketchpad💻kittsx🔖diagram](semiorepo://section/Diagram)

Diagram MUST render the interactive force-directed Kit diagram with type and design nodes.

## [👤semio📚js🗃️sketchpad💻kittsx🔖tools](semiorepo://section/Tools)

Tools MUST define Kit app toolbar filter and selection tool components.

## [👤semio📚js🗃️sketchpad💻kittsx🔖panels](semiorepo://section/Panels)

Details MUST render the Kit app detail panels for kit, type, port, tag, concept, design, file, folder, and multi-artifact sections.

## [👤semio📚js🗃️sketchpad💻kittsx🔖right](semiorepo://section/Right)

Details MUST render the Kit app detail panels for kit, type, port, tag, concept, design, file, folder, and multi-artifact sections.

## [👤semio📚js🗃️sketchpad💻kittsx🔖details](semiorepo://section/Details)

Details MUST render the Kit app detail panels for kit, type, port, tag, concept, design, file, folder, and multi-artifact sections.

## [👤semio📚js🗃️sketchpad💻kittsx🔖settings](semiorepo://section/Settings)

Settings MUST render the Kit app settings panel with theme, language, device, expertise, mode, and diagram force controls.

## [👤semio📚js🗃️sketchpad💻kittsx🔖footer](semiorepo://section/Footer)

Footer MUST render the Kit app footer with selection count status.

## [👤semio📚js🗃️sketchpad💻kittsx🔖config](semiorepo://section/Config)

Config MUST export the Kit app configuration with route segments, panel definitions, and path matching.

## [👤semio📚js🗃️sketchpad💻qualitytsx🔖imports](semiorepo://section/Imports)

External and internal module imports MUST be declared here.

## [👤semio📚js🗃️sketchpad💻qualitytsx🔖types](semiorepo://section/Types)

Type definitions MUST declare quality app state, selections, and formula structures.

## [👤semio📚js🗃️sketchpad💻qualitytsx🔖functions](semiorepo://section/Functions)

Formula function definitions, parsing, and LaTeX conversion utilities MUST be declared here.

## [👤semio📚js🗃️sketchpad💻qualitytsx🔖commands](semiorepo://section/Commands)

Quality app command handlers MUST modify state through diff objects.

## [👤semio📚js🗃️sketchpad💻qualitytsx🔖store](semiorepo://section/Store)

Quality app store, hooks, and reactive state management MUST be declared here.

## [👤semio📚js🗃️sketchpad💻qualitytsx🔖qualityapppluginregistration](semiorepo://section/Quality%20App%20Plugin%20Registration)

Plugin registration and event handler wiring MUST initialize quality app context.

## [👤semio📚js🗃️sketchpad💻qualitytsx🔖actionhooks](semiorepo://section/Action%20Hooks)

Memoized action hooks MUST provide formula node interaction callbacks.

## [👤semio📚js🗃️sketchpad💻qualitytsx🔖components](semiorepo://section/Components)

React components MUST render the quality app formula diagram, details panel, and workbench.

## [👤semio📚js🗃️sketchpad💻qualitytsx🔖app](semiorepo://section/App)

Main quality app component MUST compose window layout, drag-drop, and hotkey handling.

## [👤semio📚js🗃️sketchpad💻qualitytsx🔖config](semiorepo://section/Config)

Quality app route, panel, and path matching configuration MUST be exported.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖imports](semiorepo://section/Imports)

External and internal module imports MUST be declared here.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖components](semiorepo://section/Components)

Tutorial UI components MUST provide playback and recording controls.
Tutorial playback controls MUST render in the footer during active tutorials.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖tutorialcontrols](semiorepo://section/Tutorial%20Controls)

Tutorial playback controls MUST render in the footer during active tutorials.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖recordingcontrols](semiorepo://section/Recording%20Controls)

Recording controls MUST render in the footer during active recording in dev mode.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖recordbutton](semiorepo://section/Record%20Button)

Record button MUST toggle recording in the footer when in dev mode.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖tutorialoverlay](semiorepo://section/Tutorial%20Overlay)

Tutorial overlay MUST render focus highlights and cursor animations during playback.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖builtintutorials](semiorepo://section/Built-in%20Tutorials)

Built-in tutorials MUST define default tutorial content shipped with the app.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖commands](semiorepo://section/Commands)

Tutorial and recording command definitions MUST map command names to store actions.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖commandinterceptor](semiorepo://section/Command%20Interceptor)

Command interceptor MUST record events and check milestone completion during playback.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖hooks](semiorepo://section/Hooks)

Tutorial hooks MUST provide reactive access to tutorial and recording state.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖context](semiorepo://section/Context)

Tutorial context MUST provide the store and state to descendant components.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖types](semiorepo://section/Types)

Tutorial type definitions MUST be declared here.
Tutorial entity interfaces MUST define milestones, recordings, and playback state.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖tutorialentities](semiorepo://section/Tutorial%20Entities)

Tutorial entity interfaces MUST define milestones, recordings, and playback state.

## [👤semio📚js🗃️sketchpad💻tutorialstsx🔖store](semiorepo://section/Store)

Tutorial store MUST manage playback, recording, and milestone navigation state.

## [👤semio📚js🗃️sketchpad💻typetsx🔖imports](semiorepo://section/Imports)

External and internal dependency imports. MUST group third-party and local imports.

## [👤semio📚js🗃️sketchpad💻typetsx🔖internalstatemanagement](semiorepo://section/Internal%20State%20Management)

TypeApp state interfaces, enums, and diffing types. MUST define all shared state shapes.

## [👤semio📚js🗃️sketchpad💻typetsx🔖typeapppluginregistration](semiorepo://section/Type%20App%20Plugin%20Registration)

Plugin registration and XState event handlers for the TypeApp. MUST register all event handlers at module load.

## [👤semio📚js🗃️sketchpad💻typetsx🔖xstatehooks](semiorepo://section/XState%20Hooks)

React hooks that read and write TypeApp XState machine state. MUST use memoized selectors for performance.

## [👤semio📚js🗃️sketchpad💻typetsx🔖actionhooks](semiorepo://section/Action%20Hooks)

Convenience React hooks wrapping state hooks into single-purpose actions. MUST return action-canAct tuples.

## [👤semio📚js🗃️sketchpad💻typetsx🔖commands](semiorepo://section/Commands)

Command definitions for the TypeApp producing diffs from context. MUST return TypeAppCommandResult.

## [👤semio📚js🗃️sketchpad💻typetsx🔖scene](semiorepo://section/Scene)

Three.js scene components for connectors, meshes, and the 3D viewport. MUST render inside a React Three Fiber canvas.

## [👤semio📚js🗃️sketchpad💻typetsx🔖panels](semiorepo://section/Panels)

Panel UI sections for the right sidebar including details and settings editors. MUST use the panel section registration API.
Right sidebar panel containing details and settings sub-sections. MUST nest detail and settings regions.
Detail panel sections for editing type properties, connectors, models, authors, and attributes. MUST render within tree items.

## [👤semio📚js🗃️sketchpad💻typetsx🔖right](semiorepo://section/Right)

Right sidebar panel containing details and settings sub-sections. MUST nest detail and settings regions.
Detail panel sections for editing type properties, connectors, models, authors, and attributes. MUST render within tree items.

## [👤semio📚js🗃️sketchpad💻typetsx🔖details](semiorepo://section/Details)

Detail panel sections for editing type properties, connectors, models, authors, and attributes. MUST render within tree items.

## [👤semio📚js🗃️sketchpad💻typetsx🔖settings](semiorepo://section/Settings)

Settings panel for theme, language, device, expertise, and mode selection. MUST use toggle groups and select elements.

## [👤semio📚js🗃️sketchpad💻typetsx🔖tools](semiorepo://section/Tools)

Tool definitions for selection modes and connector creation. MUST export tool objects and settings components.

## [👤semio📚js🗃️sketchpad💻typetsx🔖app](semiorepo://section/App)

Main TypeApp component orchestrating panels, scene, keyboard shortcuts, and drag-and-drop. MUST register sections on mount.

## [👤semio📚js🗃️sketchpad💻typetsx🔖footer](semiorepo://section/Footer)

Footer component displaying model tag toggles. MUST update footer items when tags change.

## [👤semio📚js🗃️sketchpad💻typetsx🔖config](semiorepo://section/Config)

App configuration for the TypeApp including route segments, panels, and path matching. MUST define all route segments.

## [👤semio📚js🗃️sketchpad🗃️apps💻indexts🔖exports](semiorepo://section/Exports)

Exports MUST expose only the public API surface of the shared module.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖imports](semiorepo://section/Imports)

Consumers MUST NOT add non-tree-shakeable imports.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖sectionspecificity](semiorepo://section/Section%20Specificity)

Consumers MUST use these constants for section precedence.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖interactioncontext](semiorepo://section/Interaction%20Context)

Consumers MUST wrap interactive elements with InteractionProvider.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖levelcontext](semiorepo://section/Level%20Context)

Consumers MUST wrap components with LevelProvider.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖element](semiorepo://section/Element)

Consumers MUST use level functions for consistent styling.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖command](semiorepo://section/Command)

Consumers MUST use CommandInput for search functionality.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖footer](semiorepo://section/Footer)

Consumers MUST provide FooterItem entries for each action.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖layout](semiorepo://section/Layout)

Consumers MUST provide a canvas element.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖popover](semiorepo://section/Popover)

Consumers MUST wrap content in PopoverContent.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖tooltip](semiorepo://section/Tooltip)

Consumers MUST configure the expertise mode provider.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖basecomponents](semiorepo://section/Base%20Components)

Consumers MUST use these as building blocks for inputs.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖displaycomponents](semiorepo://section/Display%20Components)

Consumers MUST pass valid config objects.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖aside](semiorepo://section/Aside)

Consumers MUST specify a valid kind prop.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖avatar](semiorepo://section/Avatar)

Consumers MUST provide content for the fallback.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖card](semiorepo://section/Card)

Consumers MUST provide a title string.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖spinner](semiorepo://section/Spinner)

Consumers MUST choose an appropriate size for the context.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖notfound](semiorepo://section/NotFound)

Consumers MUST provide a title for the error.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖loadingrow](semiorepo://section/LoadingRow)

Consumers MUST provide a name for the placeholder.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖diagramnode](semiorepo://section/DiagramNode)

Consumers MUST provide content for the node.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖hovercard](semiorepo://section/HoverCard)

Consumers MUST use HoverCardTrigger to activate.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖icons](semiorepo://section/Icons)

Consumers MUST provide position data for rendering.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖section](semiorepo://section/Section)

Consumers MUST provide a heading string.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖steps](semiorepo://section/Steps)

Consumers MUST provide step children in order.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖inputcomponents](semiorepo://section/Input%20Components)

Consumers MUST provide action items for the group.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖actiongroup](semiorepo://section/ActionGroup)

Consumers MUST provide action items for the group.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖combobox](semiorepo://section/Combobox)

Consumers MUST provide options and onValueChange handler.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖input](semiorepo://section/Input)

Consumers MUST provide an id for accessibility.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖select](semiorepo://section/Select)

Consumers MUST use SelectItem children for options.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖slider](semiorepo://section/Slider)

Consumers MUST provide min and max values.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖stepper](semiorepo://section/Stepper)

Consumers MUST provide min and max bounds.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖textarea](semiorepo://section/Textarea)

Consumers MUST provide an id for the field.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖toggle](semiorepo://section/Toggle)

Consumers MUST handle onPressedChange events.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖togglegroup](semiorepo://section/ToggleGroup)

Consumers MUST provide items with distinct values.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents](semiorepo://section/Aggregation%20Components)

Consumers MUST use AccordionItem children.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖accordion](semiorepo://section/Accordion)

Consumers MUST use AccordionItem children.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖collapsible](semiorepo://section/Collapsible)

Consumers MUST use CollapsibleTrigger.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖dialog](semiorepo://section/Dialog)

Consumers MUST use DialogTrigger to open.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖resizable](semiorepo://section/Resizable)

Consumers MUST use ResizableHandle between panels.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖scrollable](semiorepo://section/Scrollable)

Consumers MUST wrap content in Scrollable.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖band](semiorepo://section/Band)

Consumers MUST provide BandItem entries.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖strip](semiorepo://section/Strip)

Consumers MUST provide StripItem entries.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖navbar](semiorepo://section/Navbar)

Consumers MUST provide NavbarItem entries.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖tabs](semiorepo://section/Tabs)

Consumers MUST use TabsTrigger and TabsContent.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖tree](semiorepo://section/Tree)

Consumers MUST wrap components in TreeStateProvider.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖navigationcomponents](semiorepo://section/Navigation%20Components)

Consumers MUST provide BreadcrumbItemData entries.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖breadcrumb](semiorepo://section/Breadcrumb)

Consumers MUST provide BreadcrumbItemData entries.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖pagenavigation](semiorepo://section/PageNavigation)

Consumers MUST provide PageNavigationLink data.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖panelcomponents](semiorepo://section/Panel%20Components)

Consumers MUST set resizeSide for the handle.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖panel](semiorepo://section/Panel)

Consumers MUST set resizeSide for the handle.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖panelgroup](semiorepo://section/PanelGroup)

Consumers MUST provide panel children.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖leftpanel](semiorepo://section/LeftPanel)

Consumers MUST provide visible and children props.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖rightpanel](semiorepo://section/RightPanel)

Consumers MUST provide visible and children props.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖middlepanel](semiorepo://section/MiddlePanel)

Consumers MUST provide visible and children props.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖bottompanel](semiorepo://section/BottomPanel)

Consumers MUST provide visible and children props.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖sidepanel](semiorepo://section/SidePanel)

Consumers MUST provide SidePanelTabConfig entries.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖hudpanel](semiorepo://section/HudPanel)

Consumers MUST provide HudPanelTabConfig entries.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖windowcomponents](semiorepo://section/Window%20Components)

Consumers MUST provide a WindowConfig object.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖window](semiorepo://section/Window)

Consumers MUST provide a WindowConfig object.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖page](semiorepo://section/Page)

Consumers MUST provide frontmatter and children.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖diagram](semiorepo://section/Diagram)

Consumers MUST provide nodes and edges arrays.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖scene](semiorepo://section/Scene)

Consumers MUST provide SceneGeometry data.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖table](semiorepo://section/Table)

Consumers MUST provide columns and data arrays.

## [👤semio📚js🗃️sketchpad💻kitselectionhelperts🔖imports](semiorepo://section/Imports)

Imports MUST include icon width constant and kit selection types.

## [👤semio📚js🗃️sketchpad💻kitselectionhelperts🔖types](semiorepo://section/Types)

Types MUST define selection value extraction for KitAppSelection dimensions.

## [👤semio📚js🗃️sketchpad💻kitselectionhelperts🔖genericutilities](semiorepo://section/Generic%20Utilities)

Generic Utilities MUST provide immutable selection manipulation functions.

## [👤semio📚js🗃️sketchpad💻kitselectionhelperts🔖kitdiagramgeometry](semiorepo://section/Kit%20Diagram%20Geometry)

Kit Diagram Geometry MUST provide geometry primitives, shape strategies, and anchor resolution.

## [👤semio📚js🗃️sketchpad💻portcolorts🔖portcolor](semiorepo://section/Port%20Color)

MUST use a union-find structure to group compatible ports under a single color.

## [👤semio📚js🗃️sketchpad💻portcolorts🪨defaultportguid](semiorepo://definition/semio/js/sketchpad/portColor.ts/DEFAULT_PORT_GUID)

MUST be used as the fallback key for tone generation.

## [👤semio📚js🗃️sketchpad💻portcolorts🛠️normalizeguid](semiorepo://definition/semio/js/sketchpad/portColor.ts/normalizeGuid)

MUST return undefined for null, undefined, or whitespace-only input.

## [👤semio📚js🗃️sketchpad💻portcolorts🛠️normalizeportref](semiorepo://definition/semio/js/sketchpad/portColor.ts/normalizePortRef)

MUST handle both direct string GUIDs and port reference objects.

## [👤semio📚js🗃️sketchpad💻portcolorts🛠️hashstring](semiorepo://definition/semio/js/sketchpad/portColor.ts/hashString)

MUST return the absolute value of a 32-bit hash.

## [👤semio📚js🗃️sketchpad💻portcolorts🛠️gettoneforkey](semiorepo://definition/semio/js/sketchpad/portColor.ts/getToneForKey)

MUST return a neutral grey tone for the default port GUID.

## [👤semio📚js🗃️sketchpad💻portcolorts🛠️createportgroupmap](semiorepo://definition/semio/js/sketchpad/portColor.ts/createPortGroupMap)

MUST union ports linked via compatiblePorts relationships.

## [👤semio📚js🗃️sketchpad💻sharedts🔖imports](semiorepo://section/Imports)

MUST import XState, Y.js, and semio core types for shared sketchpad infrastructure.

## [👤semio📚js🗃️sketchpad💻sharedts🔖types](semiorepo://section/Types)

MUST define path segment and path types for navigating Y.js document structures.

## [👤semio📚js🗃️sketchpad💻sharedts🔖ypathtypes](semiorepo://section/YPath%20Types)

MUST define path segment and path types for navigating Y.js document structures.

## [👤semio📚js🗃️sketchpad💻sharedts🔖granularhooktypes](semiorepo://section/Granular%20Hook%20Types)

MUST define hook result tuples and field abstractions for granular reactive state access.

## [👤semio📚js🗃️sketchpad💻sharedts🔖standardemptyconstants](semiorepo://section/Standard%20Empty%20Constants)

MUST provide frozen singleton constants for empty collections and default panel visibility.

## [👤semio📚js🗃️sketchpad💻sharedts🔖genericdifftypes](semiorepo://section/Generic%20Diff%20Types)

MUST define generic array and selection diff types with apply and inverse operations.

## [👤semio📚js🗃️sketchpad💻sharedts🔖enums](semiorepo://section/Enums)

MUST enumerate theme, expertise, mode, store status, tool, window, and panel kinds.

## [👤semio📚js🗃️sketchpad💻sharedts🔖ports](semiorepo://section/Ports)

MUST define file storage provider interfaces for upload, download, and delete operations.

## [👤semio📚js🗃️sketchpad💻sharedts🔖fileprovider](semiorepo://section/File%20Provider)

MUST define file storage provider interfaces for upload, download, and delete operations.

## [👤semio📚js🗃️sketchpad💻sharedts🔖appids](semiorepo://section/App%20IDs)

MUST define identifier interfaces for design, kit, type, and quality app scopes.

## [👤semio📚js🗃️sketchpad💻sharedts🔖panel](semiorepo://section/Panel)

MUST define panel kind configurations, visibility, sizing, sections, and definition interfaces.

## [👤semio📚js🗃️sketchpad💻sharedts🔖appregistry](semiorepo://section/App%20Registry)

MUST define route segment and app configuration interfaces for app registration.

## [👤semio📚js🗃️sketchpad💻sharedts🔖sketchpadstate](semiorepo://section/Sketchpad%20State)

MUST define mutable and immutable sketchpad state interfaces with diff types.

## [👤semio📚js🗃️sketchpad💻sharedts🔖commands](semiorepo://section/Commands)

MUST define command context and result interfaces for kit and sketchpad operations.

## [👤semio📚js🗃️sketchpad💻sharedts🔖store](semiorepo://section/Store)

MUST define store state, app step, edit, diff, and command result interfaces.

## [👤semio📚js🗃️sketchpad💻sharedts🔖completestate](semiorepo://section/Complete%20State)

MUST define the complete aggregated state interface for the entire sketchpad.

## [👤semio📚js🗃️sketchpad💻sharedts🔖window](semiorepo://section/Window)

MUST define window configuration, control, layout parsing, and default layout creation.

## [👤semio📚js🗃️sketchpad💻sharedts🔖tool](semiorepo://section/Tool)

MUST define tool interfaces for selection, lasso, connector, and hand interactions.

## [👤semio📚js🗃️sketchpad💻sharedts🔖focus](semiorepo://section/Focus)

MUST define the focus item interface for search and navigation targets.

## [👤semio📚js🗃️sketchpad💻sharedts🔖footer](semiorepo://section/Footer)

MUST define the footer item interface for status bar entries.

## [👤semio📚js🗃️sketchpad💻sharedts🔖panelprops](semiorepo://section/Panel%20Props)

MUST define resizable panel props interface for panel width management.

## [👤semio📚js🗃️sketchpad💻sharedts🔖xstateintegration](semiorepo://section/XState%20Integration)

MUST define XState machine context and event type interfaces for sketchpad, kit, and app machines.

## [👤semio📚js🗃️sketchpad💻sharedts🔖xstatetypes](semiorepo://section/XState%20Types)

MUST define XState machine context and event type interfaces for sketchpad, kit, and app machines.

## [👤semio📚js🗃️sketchpad💻sharedts🔖yjsxstatebridge](semiorepo://section/Y.js-XState%20Bridge)

MUST bridge Y.js document observation to XState machine events.

## [👤semio📚js🗃️sketchpad💻sharedts🔖machinefactories](semiorepo://section/Machine%20Factories)

MUST define machine input and transaction configuration interfaces for state machine creation.

## [👤semio📚js🗃️sketchpad💻sharedts🔖ypathhelpers](semiorepo://section/YPath%20Helpers)

MUST provide path segment constructors, value retrieval, and observation functions for Y.js paths.

## [👤semio📚js🗃️sketchpad💻sharedts🔖derivedstore](semiorepo://section/Derived%20Store)

MUST provide reactive derived computation nodes with dependency tracking and caching.

## [👤semio📚js🗃️sketchpad💻sharedts🔖storefactoryregistry](semiorepo://section/Store%20Factory%20Registry)

MUST manage registration and retrieval of app-specific store factory functions.

## [👤semio📚js🗃️sketchpad💻sharedts🔖apppluginregistry](semiorepo://section/App%20Plugin%20Registry)

MUST manage plugin registration, retrieval, and contribution composition for app extensions.

## [👤semio📚js🗃️sketchpad💻sharedts🔖dynamiceventdispatchregistry](semiorepo://section/Dynamic%20Event%20Dispatch%20Registry)

MUST manage dynamic event handler and guard registration with namespace-based dispatch.

## [👤semio📚js🗃️sketchpad💻sharedts🔖appeventhandlerfactories](semiorepo://section/App%20Event%20Handler%20Factories)

MUST provide factory functions for creating standard app event handlers for panels, hover, selection, and windows.

## [👤semio📚js🗃️sketchpad💻sharedts🔖transactionhandlerfactory](semiorepo://section/Transaction%20Handler%20Factory)

MUST provide factory functions for creating undo/redo transaction event handlers.

## [👤semio📚js🗃️sketchpad💻sharedts🔖selectorfactorypattern](semiorepo://section/Selector%20Factory%20Pattern)

MUST provide factory functions for creating property selectors with app key scoping.

## [👤semio📚js🗃️sketchpad💻sharedts🔖apphooksregistry](semiorepo://section/App%20Hooks%20Registry)

MUST manage registration and retrieval of design and kit app hook implementations.

## [👤semio📚js🗃️sketchpad💻sharedts🔖appregistryexports](semiorepo://section/App%20Registry%20Exports)

MUST provide docs registry port interface and registration for documentation section access.

## [👤semio📚js💻viteenvdts🔖declarations](semiorepo://section/Declarations)

Declarations MUST cover all custom asset import suffixes used in the project.

## [👤semio🛂jsonschema💻buildts🔖schemaexport](semiorepo://section/Schema%20Export)

JSON Schema export script. MUST unescape and write the kit schema file.

## [👤semio🛂jsonschema💻buildts🪨inputfilepath](semiorepo://definition/semio/jsonschema/build.ts/inputFilePath)

MUST point to the kit.json schema file.

## [👤semio🛂jsonschema💻buildts🪨outputfilepath](semiorepo://definition/semio/jsonschema/build.ts/outputFilePath)

MUST be written next to the input file.

## [👤semio🛂jsonschema💻buildts🪨jsoncontent](semiorepo://definition/semio/jsonschema/build.ts/jsonContent)

MUST be read as UTF-8.

## [👤semio🛂jsonschema💻buildts🪨unescapedcontent](semiorepo://definition/semio/jsonschema/build.ts/unescapedContent)

MUST replace all escaped characters.

## [👤semio📚net🛅semio💻semiocs🛠️symbol](semiorepo://definition/semio/net/Semio/Semio.cs/Symbol)

/ Implementations MUST be immutable value types within expression trees.

## [👤semio📚net🛅semio💻semiocs🛠️entity](semiorepo://definition/semio/net/Semio/Semio.cs/Entity)

/ Implementations MUST override equality based on serialized representation.

## [👤semio📚net🛅semio💻semiocs🛠️entityvalidator](semiorepo://definition/semio/net/Semio/Semio.cs/EntityValidator)

/ Implementations MUST define validation rules in the constructor.

## [👤semio📚net🛅semio💻buildts🔖build](semiorepo://section/Build)

.NET build script. MUST compile the Semio C# project via MSBuild.

## [👤semio📚net🛅semio💻buildts🪨msbuild](semiorepo://definition/semio/net/Semio/build.ts/msbuild)

MUST point to the installed MSBuild binary.

## [👤semio🌐play💻indextsx🔖entrypoint](semiorepo://section/Entrypoint)

Entrypoint MUST register all app configs before rendering the Sketchpad component.

## [👤semio📚rs💻semiors🔖imports](semiorepo://section/Imports)

Imports MUST include all required crates and modules for the semio domain library.

## [👤semio📚rs💻semiors🔖errortypes](semiorepo://section/Error%20Types)

Error Types MUST provide the error types functionality.

## [👤semio📚rs💻semiors🔖utilityfunctions](semiorepo://section/Utility%20Functions)

Utility Functions MUST provide the utility functions functionality.

## [👤semio📚rs💻semiors🔖modeltypesattribute](semiorepo://section/Model%20Types%20-%20Attribute)

Model Types - Attribute MUST provide the model types - attribute functionality.

## [👤semio📚rs💻semiors🔖modeltypescoord](semiorepo://section/Model%20Types%20-%20Coord)

Model Types - Coord MUST provide the model types - coord functionality.

## [👤semio📚rs💻semiors🔖modeltypesvector](semiorepo://section/Model%20Types%20-%20Vector)

Model Types - Vector MUST provide the model types - vector functionality.

## [👤semio📚rs💻semiors🔖modeltypesplane](semiorepo://section/Model%20Types%20-%20Plane)

Model Types - Plane MUST provide the model types - plane functionality.

## [👤semio📚rs💻semiors🔖modeltypescamera](semiorepo://section/Model%20Types%20-%20Camera)

Model Types - Camera MUST provide the model types - camera functionality.

## [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder](semiorepo://section/Model%20Types%20-%20Location,%20Author,%20File,%20Folder)

Model Types - Location, Author, File, Folder MUST provide the model types - location, author, file, folder functionality.

## [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept](semiorepo://section/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept)

Model Types - Quality, Port, Tag, Concept MUST provide the model types - quality, port, tag, concept functionality.

## [👤semio📚rs💻semiors🔖modeltypespropmodelconnector](semiorepo://section/Model%20Types%20-%20Prop,%20Model,%20Connector)

Model Types - Prop, Model, Connector MUST provide the model types - prop, model, connector functionality.

## [👤semio📚rs💻semiors🔖modeltypestype](semiorepo://section/Model%20Types%20-%20Type)

Model Types - Type MUST provide the model types - type functionality.

## [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat](semiorepo://section/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat)

Model Types - Layer, Piece, Group, Side, Connection, Stat MUST provide the model types - layer, piece, group, side, connection, stat functionality.

## [👤semio📚rs💻semiors🔖modeltypesdesign](semiorepo://section/Model%20Types%20-%20Design)

Model Types - Design MUST provide the model types - design functionality.

## [👤semio📚rs💻semiors🔖modeltypeskit](semiorepo://section/Model%20Types%20-%20Kit)

Model Types - Kit MUST provide the model types - kit functionality.

## [👤semio📚rs💻semiors🔖finderfunctions](semiorepo://section/Finder%20Functions)

Finder Functions MUST provide the finder functions functionality.
/ find_type_in_kit MUST perform the find_type_in_kit operation.

## [👤semio📚rs💻semiors🔖serialization](semiorepo://section/Serialization)

Serialization MUST provide the serialization functionality.
/ serialize_kit MUST perform the serialize_kit operation.

## [👤semio📚rs💻semiors🔖difftypes](semiorepo://section/Diff%20Types)

Diff Types MUST provide the diff types functionality.

## [👤semio📚rs💻semiors🔖hasguidtrait](semiorepo://section/HasGuid%20Trait)

HasGuid Trait MUST provide the hasguid trait functionality.
/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semiors🔖applydiff](semiorepo://section/ApplyDiff)

ApplyDiff MUST provide the applydiff functionality.
/ apply_collection_diff MUST perform the apply_collection_diff operation.

## [👤semio📚rs💻semiors🔖flattendesign](semiorepo://section/FlattenDesign)

FlattenDesign MUST provide the flattendesign functionality.
/ FlattenedPiece MUST perform the FlattenedPiece operation.

## [👤semio📚rs💻semiors🔖validationtypes](semiorepo://section/Validation%20Types)

Validation Types MUST provide the validation types functionality.

## [👤semio📚rs💻semiors🔖sqliteimportexport](semiorepo://section/SQLite%20Import/Export)

SQLite Import/Export MUST provide the sqlite import/export functionality.

## [👤semio📚rs💻semiors🔖zipimportexport](semiorepo://section/Zip%20Import/Export)

Zip Import/Export MUST provide the zip import/export functionality.

## [👤semio📚rs💻semiors🔖wasmbindings](semiorepo://section/WASM%20Bindings)

WASM Bindings MUST provide the wasm bindings functionality.

## [👤semio📚rs💻semiors🔖tests](semiorepo://section/Tests)

Tests MUST provide the tests functionality.

## [👤semio📚rs💻semiors🔖roundtriptests](semiorepo://section/Roundtrip%20Tests)

Roundtrip Tests MUST provide the roundtrip tests functionality.

## [👤semio📚rs💻semiors🔖flattentests](semiorepo://section/Flatten%20Tests)

Flatten Tests MUST provide the flatten tests functionality.

## [👤semio📚rs💻semiors🔖difftests](semiorepo://section/Diff%20Tests)

Diff Tests MUST provide the diff tests functionality.

## [👤semio📚rs💻semiors🔖validationtests](semiorepo://section/Validation%20Tests)

Validation Tests MUST provide the validation tests functionality.

## [👤semio📚rs💻semiors🛠️guid](semiorepo://definition/semio/rs/semio.rs/guid)

/ guid MUST perform the guid operation.

## [👤semio📚rs💻semiors🛠️normalize](semiorepo://definition/semio/rs/semio.rs/normalize)

/ normalize MUST perform the normalize operation.

## [👤semio📚rs💻semiors🛠️round](semiorepo://definition/semio/rs/semio.rs/round)

/ round MUST perform the round operation.

## [👤semio📚rs💻semiors🛠️jaccard](semiorepo://definition/semio/rs/semio.rs/jaccard)

/ jaccard MUST perform the jaccard operation.

## [👤semio📚rs💻semiors🛠️deepequal](semiorepo://definition/semio/rs/semio.rs/deep_equal)

/ deep_equal MUST perform the deep_equal operation.

## [👤semio📚rs💻semiors🛠️generateuniquename](semiorepo://definition/semio/rs/semio.rs/generate_unique_name)

/ generate_unique_name MUST perform the generate_unique_name operation.

## [👤semio📚rs💻semiors🛠️attribute](semiorepo://definition/semio/rs/semio.rs/Attribute)

/ Attribute MUST perform the Attribute operation.

## [👤semio📚rs💻semiors🛠️attributeid](semiorepo://definition/semio/rs/semio.rs/AttributeId)

/ AttributeId MUST perform the AttributeId operation.

## [👤semio📚rs💻semiors🛠️coord](semiorepo://definition/semio/rs/semio.rs/Coord)

/ Coord MUST perform the Coord operation.

## [👤semio📚rs💻semiors🛠️vector](semiorepo://definition/semio/rs/semio.rs/Vector)

/ Vector MUST perform the Vector operation.

## [👤semio📚rs💻semiors🛠️plane](semiorepo://definition/semio/rs/semio.rs/Plane)

/ Plane MUST perform the Plane operation.

## [👤semio📚rs💻semiors🛠️camera](semiorepo://definition/semio/rs/semio.rs/Camera)

/ Camera MUST perform the Camera operation.

## [👤semio📚rs💻semiors🛠️locationid](semiorepo://definition/semio/rs/semio.rs/LocationId)

/ LocationId MUST perform the LocationId operation.

## [👤semio📚rs💻semiors🛠️location](semiorepo://definition/semio/rs/semio.rs/Location)

/ Location MUST perform the Location operation.

## [👤semio📚rs💻semiors🛠️authorid](semiorepo://definition/semio/rs/semio.rs/AuthorId)

/ AuthorId MUST perform the AuthorId operation.

## [👤semio📚rs💻semiors🛠️author](semiorepo://definition/semio/rs/semio.rs/Author)

/ Author MUST perform the Author operation.

## [👤semio📚rs💻semiors🛠️folderid](semiorepo://definition/semio/rs/semio.rs/FolderId)

/ FolderId MUST perform the FolderId operation.

## [👤semio📚rs💻semiors🛠️folder](semiorepo://definition/semio/rs/semio.rs/Folder)

/ Folder MUST perform the Folder operation.

## [👤semio📚rs💻semiors🛠️fileid](semiorepo://definition/semio/rs/semio.rs/FileId)

/ FileId MUST perform the FileId operation.

## [👤semio📚rs💻semiors🛠️file](semiorepo://definition/semio/rs/semio.rs/File)

/ File MUST perform the File operation.

## [👤semio📚rs💻semiors🛠️qualityid](semiorepo://definition/semio/rs/semio.rs/QualityId)

/ QualityId MUST perform the QualityId operation.

## [👤semio📚rs💻semiors🛠️quality](semiorepo://definition/semio/rs/semio.rs/Quality)

/ Quality MUST perform the Quality operation.

## [👤semio📚rs💻semiors🛠️portid](semiorepo://definition/semio/rs/semio.rs/PortId)

/ PortId MUST perform the PortId operation.

## [👤semio📚rs💻semiors🛠️port](semiorepo://definition/semio/rs/semio.rs/Port)

/ Port MUST perform the Port operation.

## [👤semio📚rs💻semiors🛠️tagid](semiorepo://definition/semio/rs/semio.rs/TagId)

/ TagId MUST perform the TagId operation.

## [👤semio📚rs💻semiors🛠️tag](semiorepo://definition/semio/rs/semio.rs/Tag)

/ Tag MUST perform the Tag operation.

## [👤semio📚rs💻semiors🛠️conceptid](semiorepo://definition/semio/rs/semio.rs/ConceptId)

/ ConceptId MUST perform the ConceptId operation.

## [👤semio📚rs💻semiors🛠️concept](semiorepo://definition/semio/rs/semio.rs/Concept)

/ Concept MUST perform the Concept operation.
/ Concept MUST perform the Concept operation.

## [👤semio📚rs💻semiors🛠️propid](semiorepo://definition/semio/rs/semio.rs/PropId)

/ PropId MUST perform the PropId operation.

## [👤semio📚rs💻semiors🛠️prop](semiorepo://definition/semio/rs/semio.rs/Prop)

/ Prop MUST perform the Prop operation.

## [👤semio📚rs💻semiors🛠️modelid](semiorepo://definition/semio/rs/semio.rs/ModelId)

/ ModelId MUST perform the ModelId operation.

## [👤semio📚rs💻semiors🛠️model](semiorepo://definition/semio/rs/semio.rs/Model)

/ Model MUST perform the Model operation.
/ Model MUST perform the Model operation.

## [👤semio📚rs💻semiors🛠️connectorid](semiorepo://definition/semio/rs/semio.rs/ConnectorId)

/ ConnectorId MUST perform the ConnectorId operation.

## [👤semio📚rs💻semiors🛠️connector](semiorepo://definition/semio/rs/semio.rs/Connector)

/ Connector MUST perform the Connector operation.

## [👤semio📚rs💻semiors🛠️typeid](semiorepo://definition/semio/rs/semio.rs/TypeId)

/ TypeId MUST perform the TypeId operation.

## [👤semio📚rs💻semiors🛠️type](semiorepo://definition/semio/rs/semio.rs/Type)

/ Type MUST perform the Type operation.

## [👤semio📚rs💻semiors🛠️layerid](semiorepo://definition/semio/rs/semio.rs/LayerId)

/ LayerId MUST perform the LayerId operation.

## [👤semio📚rs💻semiors🛠️layer](semiorepo://definition/semio/rs/semio.rs/Layer)

/ Layer MUST perform the Layer operation.

## [👤semio📚rs💻semiors🛠️pieceid](semiorepo://definition/semio/rs/semio.rs/PieceId)

/ PieceId MUST perform the PieceId operation.

## [👤semio📚rs💻semiors🛠️designid](semiorepo://definition/semio/rs/semio.rs/DesignId)

/ DesignId MUST perform the DesignId operation.

## [👤semio📚rs💻semiors🛠️piece](semiorepo://definition/semio/rs/semio.rs/Piece)

/ Piece MUST perform the Piece operation.

## [👤semio📚rs💻semiors🛠️groupid](semiorepo://definition/semio/rs/semio.rs/GroupId)

/ GroupId MUST perform the GroupId operation.

## [👤semio📚rs💻semiors🛠️group](semiorepo://definition/semio/rs/semio.rs/Group)

/ Group MUST perform the Group operation.

## [👤semio📚rs💻semiors🛠️side](semiorepo://definition/semio/rs/semio.rs/Side)

/ Side MUST perform the Side operation.

## [👤semio📚rs💻semiors🛠️connectionid](semiorepo://definition/semio/rs/semio.rs/ConnectionId)

/ ConnectionId MUST perform the ConnectionId operation.

## [👤semio📚rs💻semiors🛠️connection](semiorepo://definition/semio/rs/semio.rs/Connection)

/ Connection MUST perform the Connection operation.

## [👤semio📚rs💻semiors🛠️statid](semiorepo://definition/semio/rs/semio.rs/StatId)

/ StatId MUST perform the StatId operation.

## [👤semio📚rs💻semiors🛠️stat](semiorepo://definition/semio/rs/semio.rs/Stat)

/ Stat MUST perform the Stat operation.
/ Stat MUST perform the Stat operation.

## [👤semio📚rs💻semiors🛠️design](semiorepo://definition/semio/rs/semio.rs/Design)

/ Design MUST perform the Design operation.

## [👤semio📚rs💻semiors🛠️kit](semiorepo://definition/semio/rs/semio.rs/Kit)

/ Kit MUST perform the Kit operation.

## [👤semio📚rs💻semiors🛠️findtypeinkit](semiorepo://definition/semio/rs/semio.rs/find_type_in_kit)

/ find_type_in_kit MUST perform the find_type_in_kit operation.

## [👤semio📚rs💻semiors🛠️findtypeinkitmut](semiorepo://definition/semio/rs/semio.rs/find_type_in_kit_mut)

/ find_type_in_kit_mut MUST perform the find_type_in_kit_mut operation.

## [👤semio📚rs💻semiors🛠️finddesigninkit](semiorepo://definition/semio/rs/semio.rs/find_design_in_kit)

/ find_design_in_kit MUST perform the find_design_in_kit operation.

## [👤semio📚rs💻semiors🛠️finddesigninkitmut](semiorepo://definition/semio/rs/semio.rs/find_design_in_kit_mut)

/ find_design_in_kit_mut MUST perform the find_design_in_kit_mut operation.

## [👤semio📚rs💻semiors🛠️findpieceindesign](semiorepo://definition/semio/rs/semio.rs/find_piece_in_design)

/ find_piece_in_design MUST perform the find_piece_in_design operation.

## [👤semio📚rs💻semiors🛠️findpieceindesignmut](semiorepo://definition/semio/rs/semio.rs/find_piece_in_design_mut)

/ find_piece_in_design_mut MUST perform the find_piece_in_design_mut operation.

## [👤semio📚rs💻semiors🛠️findconnectionindesign](semiorepo://definition/semio/rs/semio.rs/find_connection_in_design)

/ find_connection_in_design MUST perform the find_connection_in_design operation.

## [👤semio📚rs💻semiors🛠️findconnectorintype](semiorepo://definition/semio/rs/semio.rs/find_connector_in_type)

/ find_connector_in_type MUST perform the find_connector_in_type operation.

## [👤semio📚rs💻semiors🛠️findmodelintype](semiorepo://definition/semio/rs/semio.rs/find_model_in_type)

/ find_model_in_type MUST perform the find_model_in_type operation.

## [👤semio📚rs💻semiors🛠️findfileinkit](semiorepo://definition/semio/rs/semio.rs/find_file_in_kit)

/ find_file_in_kit MUST perform the find_file_in_kit operation.

## [👤semio📚rs💻semiors🛠️findfolderinkit](semiorepo://definition/semio/rs/semio.rs/find_folder_in_kit)

/ find_folder_in_kit MUST perform the find_folder_in_kit operation.

## [👤semio📚rs💻semiors🛠️findauthorinkit](semiorepo://definition/semio/rs/semio.rs/find_author_in_kit)

/ find_author_in_kit MUST perform the find_author_in_kit operation.
/ find_author_in_kit MUST perform the find_author_in_kit operation.

## [👤semio📚rs💻semiors🛠️findtaginkit](semiorepo://definition/semio/rs/semio.rs/find_tag_in_kit)

/ find_tag_in_kit MUST perform the find_tag_in_kit operation.

## [👤semio📚rs💻semiors🛠️findconceptinkit](semiorepo://definition/semio/rs/semio.rs/find_concept_in_kit)

/ find_concept_in_kit MUST perform the find_concept_in_kit operation.

## [👤semio📚rs💻semiors🛠️findqualityinkit](semiorepo://definition/semio/rs/semio.rs/find_quality_in_kit)

/ find_quality_in_kit MUST perform the find_quality_in_kit operation.

## [👤semio📚rs💻semiors🛠️findinterfaceinkit](semiorepo://definition/semio/rs/semio.rs/find_interface_in_kit)

/ find_interface_in_kit MUST perform the find_interface_in_kit operation.

## [👤semio📚rs💻semiors🛠️findlayerindesign](semiorepo://definition/semio/rs/semio.rs/find_layer_in_design)

/ find_layer_in_design MUST perform the find_layer_in_design operation.

## [👤semio📚rs💻semiors🛠️findgroupindesign](semiorepo://definition/semio/rs/semio.rs/find_group_in_design)

/ find_group_in_design MUST perform the find_group_in_design operation.

## [👤semio📚rs💻semiors🛠️findstatindesign](semiorepo://definition/semio/rs/semio.rs/find_stat_in_design)

/ find_stat_in_design MUST perform the find_stat_in_design operation.

## [👤semio📚rs💻semiors🛠️serializekit](semiorepo://definition/semio/rs/semio.rs/serialize_kit)

/ serialize_kit MUST perform the serialize_kit operation.

## [👤semio📚rs💻semiors🛠️deserializekit](semiorepo://definition/semio/rs/semio.rs/deserialize_kit)

/ deserialize_kit MUST perform the deserialize_kit operation.

## [👤semio📚rs💻semiors🛠️serializedesign](semiorepo://definition/semio/rs/semio.rs/serialize_design)

/ serialize_design MUST perform the serialize_design operation.

## [👤semio📚rs💻semiors🛠️deserializedesign](semiorepo://definition/semio/rs/semio.rs/deserialize_design)

/ deserialize_design MUST perform the deserialize_design operation.

## [👤semio📚rs💻semiors🛠️serializetype](semiorepo://definition/semio/rs/semio.rs/serialize_type)

/ serialize_type MUST perform the serialize_type operation.

## [👤semio📚rs💻semiors🛠️deserializetype](semiorepo://definition/semio/rs/semio.rs/deserialize_type)

/ deserialize_type MUST perform the deserialize_type operation.

## [👤semio📚rs💻semiors🛠️arekitsequal](semiorepo://definition/semio/rs/semio.rs/are_kits_equal)

/ are_kits_equal MUST perform the are_kits_equal operation.
/ are_kits_equal MUST perform the are_kits_equal operation.

## [👤semio📚rs💻semiors🛠️aredesignsequal](semiorepo://definition/semio/rs/semio.rs/are_designs_equal)

/ are_designs_equal MUST perform the are_designs_equal operation.

## [👤semio📚rs💻semiors🛠️aretypesequal](semiorepo://definition/semio/rs/semio.rs/are_types_equal)

/ are_types_equal MUST perform the are_types_equal operation.

## [👤semio📚rs💻semiors🛠️issupportedmodelextension](semiorepo://definition/semio/rs/semio.rs/is_supported_model_extension)

/ is_supported_model_extension MUST perform the is_supported_model_extension operation.
/ is_supported_model_extension MUST perform the is_supported_model_extension operation.

## [👤semio📚rs💻semiors🛠️removeditem](semiorepo://definition/semio/rs/semio.rs/RemovedItem)

/ RemovedItem MUST perform the RemovedItem operation.

## [👤semio📚rs💻semiors🛠️diffupdate](semiorepo://definition/semio/rs/semio.rs/DiffUpdate)

/ DiffUpdate MUST perform the DiffUpdate operation.

## [👤semio📚rs💻semiors🛠️collectiondiff](semiorepo://definition/semio/rs/semio.rs/CollectionDiff)

/ CollectionDiff MUST perform the CollectionDiff operation.

## [👤semio📚rs💻semiors🛠️attributediff](semiorepo://definition/semio/rs/semio.rs/AttributeDiff)

/ AttributeDiff MUST perform the AttributeDiff operation.

## [👤semio📚rs💻semiors🛠️propdiff](semiorepo://definition/semio/rs/semio.rs/PropDiff)

/ PropDiff MUST perform the PropDiff operation.

## [👤semio📚rs💻semiors🛠️connectordiff](semiorepo://definition/semio/rs/semio.rs/ConnectorDiff)

/ ConnectorDiff MUST perform the ConnectorDiff operation.

## [👤semio📚rs💻semiors🛠️modeldiff](semiorepo://definition/semio/rs/semio.rs/ModelDiff)

/ ModelDiff MUST perform the ModelDiff operation.

## [👤semio📚rs💻semiors🛠️typediff](semiorepo://definition/semio/rs/semio.rs/TypeDiff)

/ TypeDiff MUST perform the TypeDiff operation.

## [👤semio📚rs💻semiors🛠️sidediff](semiorepo://definition/semio/rs/semio.rs/SideDiff)

/ SideDiff MUST perform the SideDiff operation.

## [👤semio📚rs💻semiors🛠️connectiondiff](semiorepo://definition/semio/rs/semio.rs/ConnectionDiff)

/ ConnectionDiff MUST perform the ConnectionDiff operation.

## [👤semio📚rs💻semiors🛠️piecediff](semiorepo://definition/semio/rs/semio.rs/PieceDiff)

/ PieceDiff MUST perform the PieceDiff operation.

## [👤semio📚rs💻semiors🛠️layerdiff](semiorepo://definition/semio/rs/semio.rs/LayerDiff)

/ LayerDiff MUST perform the LayerDiff operation.

## [👤semio📚rs💻semiors🛠️groupdiff](semiorepo://definition/semio/rs/semio.rs/GroupDiff)

/ GroupDiff MUST perform the GroupDiff operation.

## [👤semio📚rs💻semiors🛠️statdiff](semiorepo://definition/semio/rs/semio.rs/StatDiff)

/ StatDiff MUST perform the StatDiff operation.

## [👤semio📚rs💻semiors🛠️designdiff](semiorepo://definition/semio/rs/semio.rs/DesignDiff)

/ DesignDiff MUST perform the DesignDiff operation.

## [👤semio📚rs💻semiors🛠️tagdiff](semiorepo://definition/semio/rs/semio.rs/TagDiff)

/ TagDiff MUST perform the TagDiff operation.

## [👤semio📚rs💻semiors🛠️conceptdiff](semiorepo://definition/semio/rs/semio.rs/ConceptDiff)

/ ConceptDiff MUST perform the ConceptDiff operation.

## [👤semio📚rs💻semiors🛠️portdiff](semiorepo://definition/semio/rs/semio.rs/PortDiff)

/ PortDiff MUST perform the PortDiff operation.

## [👤semio📚rs💻semiors🛠️qualitydiff](semiorepo://definition/semio/rs/semio.rs/QualityDiff)

/ QualityDiff MUST perform the QualityDiff operation.

## [👤semio📚rs💻semiors🛠️filediff](semiorepo://definition/semio/rs/semio.rs/FileDiff)

/ FileDiff MUST perform the FileDiff operation.

## [👤semio📚rs💻semiors🛠️folderdiff](semiorepo://definition/semio/rs/semio.rs/FolderDiff)

/ FolderDiff MUST perform the FolderDiff operation.

## [👤semio📚rs💻semiors🛠️authordiff](semiorepo://definition/semio/rs/semio.rs/AuthorDiff)

/ AuthorDiff MUST perform the AuthorDiff operation.

## [👤semio📚rs💻semiors🛠️kitdiff](semiorepo://definition/semio/rs/semio.rs/KitDiff)

/ KitDiff MUST perform the KitDiff operation.

## [👤semio📚rs💻semiors✂️hasguid](semiorepo://definition/semio/rs/semio.rs/HasGuid)

/ HasGuid MUST perform the HasGuid operation.

## [👤semio📚rs💻semiors✂️diffhasguid](semiorepo://definition/semio/rs/semio.rs/DiffHasGuid)

/ DiffHasGuid MUST perform the DiffHasGuid operation.

## [👤semio📚rs💻semiors🛠️applycollectiondiff](semiorepo://definition/semio/rs/semio.rs/apply_collection_diff)

/ apply_collection_diff MUST perform the apply_collection_diff operation.

## [👤semio📚rs💻semiors🛠️applyattributediff](semiorepo://definition/semio/rs/semio.rs/apply_attribute_diff)

/ apply_attribute_diff MUST perform the apply_attribute_diff operation.
/ apply_attribute_diff MUST perform the apply_attribute_diff operation.

## [👤semio📚rs💻semiors🛠️applypropdiff](semiorepo://definition/semio/rs/semio.rs/apply_prop_diff)

/ apply_prop_diff MUST perform the apply_prop_diff operation.

## [👤semio📚rs💻semiors🛠️applyconnectordiff](semiorepo://definition/semio/rs/semio.rs/apply_connector_diff)

/ apply_connector_diff MUST perform the apply_connector_diff operation.

## [👤semio📚rs💻semiors🛠️applymodeldiff](semiorepo://definition/semio/rs/semio.rs/apply_model_diff)

/ apply_model_diff MUST perform the apply_model_diff operation.

## [👤semio📚rs💻semiors🛠️applytypediff](semiorepo://definition/semio/rs/semio.rs/apply_type_diff)

/ apply_type_diff MUST perform the apply_type_diff operation.

## [👤semio📚rs💻semiors🛠️applylayerdiff](semiorepo://definition/semio/rs/semio.rs/apply_layer_diff)

/ apply_layer_diff MUST perform the apply_layer_diff operation.

## [👤semio📚rs💻semiors🛠️applygroupdiff](semiorepo://definition/semio/rs/semio.rs/apply_group_diff)

/ apply_group_diff MUST perform the apply_group_diff operation.

## [👤semio📚rs💻semiors🛠️applystatdiff](semiorepo://definition/semio/rs/semio.rs/apply_stat_diff)

/ apply_stat_diff MUST perform the apply_stat_diff operation.

## [👤semio📚rs💻semiors🛠️applypiecediff](semiorepo://definition/semio/rs/semio.rs/apply_piece_diff)

/ apply_piece_diff MUST perform the apply_piece_diff operation.

## [👤semio📚rs💻semiors🛠️applyconnectiondiff](semiorepo://definition/semio/rs/semio.rs/apply_connection_diff)

/ apply_connection_diff MUST perform the apply_connection_diff operation.

## [👤semio📚rs💻semiors🛠️applydesigndiff](semiorepo://definition/semio/rs/semio.rs/apply_design_diff)

/ apply_design_diff MUST perform the apply_design_diff operation.

## [👤semio📚rs💻semiors🛠️applytagdiff](semiorepo://definition/semio/rs/semio.rs/apply_tag_diff)

/ apply_tag_diff MUST perform the apply_tag_diff operation.

## [👤semio📚rs💻semiors🛠️applyconceptdiff](semiorepo://definition/semio/rs/semio.rs/apply_concept_diff)

/ apply_concept_diff MUST perform the apply_concept_diff operation.

## [👤semio📚rs💻semiors🛠️applyinterfacediff](semiorepo://definition/semio/rs/semio.rs/apply_interface_diff)

/ apply_interface_diff MUST perform the apply_interface_diff operation.

## [👤semio📚rs💻semiors🛠️applyqualitydiff](semiorepo://definition/semio/rs/semio.rs/apply_quality_diff)

/ apply_quality_diff MUST perform the apply_quality_diff operation.

## [👤semio📚rs💻semiors🛠️applyfilediff](semiorepo://definition/semio/rs/semio.rs/apply_file_diff)

/ apply_file_diff MUST perform the apply_file_diff operation.

## [👤semio📚rs💻semiors🛠️applyfolderdiff](semiorepo://definition/semio/rs/semio.rs/apply_folder_diff)

/ apply_folder_diff MUST perform the apply_folder_diff operation.

## [👤semio📚rs💻semiors🛠️applyauthordiff](semiorepo://definition/semio/rs/semio.rs/apply_author_diff)

/ apply_author_diff MUST perform the apply_author_diff operation.

## [👤semio📚rs💻semiors🛠️applykitdiff](semiorepo://definition/semio/rs/semio.rs/apply_kit_diff)

/ apply_kit_diff MUST perform the apply_kit_diff operation.

## [👤semio📚rs💻semiors🛠️flattenedpiece](semiorepo://definition/semio/rs/semio.rs/FlattenedPiece)

/ FlattenedPiece MUST perform the FlattenedPiece operation.

## [👤semio📚rs💻semiors🛠️flattendesign](semiorepo://definition/semio/rs/semio.rs/flatten_design)

/ flatten_design MUST perform the flatten_design operation.

## [👤semio📚rs💻semiors🛠️validationproblem](semiorepo://definition/semio/rs/semio.rs/ValidationProblem)

/ ValidationProblem MUST perform the ValidationProblem operation.
/ ValidationProblem MUST perform the ValidationProblem operation.

## [👤semio📚rs💻semiors🛠️validationfix](semiorepo://definition/semio/rs/semio.rs/ValidationFix)

/ ValidationFix MUST perform the ValidationFix operation.

## [👤semio📚rs💻semiors🛠️validationresult](semiorepo://definition/semio/rs/semio.rs/ValidationResult)

/ ValidationResult MUST perform the ValidationResult operation.

## [👤semio📚rs💻semiors🛠️validatekit](semiorepo://definition/semio/rs/semio.rs/validate_kit)

/ validate_kit MUST perform the validate_kit operation.

## [👤semio🖱️sketchpad💻indextsx🔖entrypoint](semiorepo://section/Entrypoint)

Entrypoint MUST register all app configs before rendering the Sketchpad component.
