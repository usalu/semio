This document MUST ALWAYS be followed unless explicitly asked to do otherwise.

# Specs

## Kit

A `kit` is a collection of `types`, `designs`, `authors`, `qualities`, `attributes`, and `concepts`.

A `kit` is either _static_ (a special `.zip` file) or _dynamic_ (bound to a runtime).

A _static_ `kit` contains a reserved `.semio` folder that contains a `kit.db` sqlite file.

The SQL-schema of `kit.db` is found in `./sqlite/schema.sql`.

For Inter-Process-Communication (IPC) the JSON-schema in `./jsonschema/kit.json` is used.

## Design

A `design` is an undirected graph of `pieces` (nodes) and `connections` (edges) with organizational `layers`, `groups`, `stats`, `attributes`, and `concepts`.

A _flat_ `design` has no `connections` and all `pieces` are _fixed_.

The `pieces` are _placed_ _hierarchically_ (breadth-first) for every _component_.

Additional `connections` which where not used in the _placement_ can be used to validate the computed `planes`.

## Type

A `type` is a reusable component with different `representations`, `ports`, `attributes`, `concepts`, and `authors`.

A `type` can be **virtual** (intermediate type requiring other virtual types to form a physical type), **scalable**, and **mirrorable** with **stock** quantity, **unit**, and optional **location**.

## Connection

A `connection` is a 3D-Link between two `pieces` with the _translation_ parameters **gap** (offset in y-direction), **shift** (offset in x-direction) and **rise** (offset in z-direction), and the _rotation_ parameters **rotation** (rotation around y-axis), **turn** (rotation around z-axis) and **tilt** (rotation around x-axis).

The _translation_ is applied first, then the _rotation_.

The two `pieces` are called **_connected_** and **_connecting_** but there is no difference between them.

The _direction_ of a `connection` goes from the lower _hierarchy_ to the higher _hierarchy_ of the `pieces`.

A `connection` can have `attributes` and diagram positioning with **x** and **y** offsets.

## Piece

A `piece` is an instance of either a `type` or a `design` with **id**, optional **description**, optional **plane**, **center** position, **scale**, optional **mirror plane**, **hidden** and **locked** states, **color**, and `attributes`.

A `piece` is either _fixed_ (with a `plane`) or _linked_ (with a `connection`).

A group of _connected_ `pieces` is called a _component_.

The _hierarchy_ of a `piece` is the length of the shortest path to the next _fixed_ `piece`.

## Port

A `port` is a conceptual connection **point** with an outwards **direction**, **id**, optional **description**, and **t** value for diagram ring positioning.

A `port` can be marked as **mandatory** in which case it is required to be connected to a `piece`.

A `port` can have a port **family** and a list of **compatible families** for explicit compatibility control.

No **family** means the _default_ family and no **compatible families** means the port is compatible with all other ports.

It is enough for one `port` to be compatible with another `port` to be compatible with each other.

A `port` can have `props` that define measurable characteristics and `attributes` for additional metadata.

## Representation

A `representation` is a `tagged` `url` to a resource with an optional **description**.

No **`tags`** means the _default_ representation.

The similarity of `representations` is determined by the jaccard index of their **`tags`**.

## Attribute

A `attribute` is metadata with a unique **name**, an optional **value**, an optional **unit** and an optional **definition** (`url` or text).

The **name** is kebab-cased and with `.`-separated string similar to toml keys.

No **value** is equivalent to the boolean _true_ where the **name** is the category of the attribute.

The **unit** is a unit identifier.

- `mm` for millimeter, `cm` for centimeter, `dm` for decimeter, `m` for meter, `km` for kilometer
- `m²` for square meter, `m³` for cubic meter, `m⁴` for quartic meter
- `°` for degree, `rad` for radian
- `N` for newton, `kN` for kilonewton, `MN` for meganewton
- `°C` for degree Celsius, `°F` for degree Fahrenheit
- `W` for watt, `kW` for kilowatt, `MW` for megawatt, `GW` for gigawatt
- `Wh` for watt-hour, `kWh` for kilowatt-hour, `MWh` for megawatt-hour, `GWh` for gigawatt-hour
- `J` for joule, `kJ` for kilojoule, `kcal` for kilocalorie
- `kWh/m²a` for kilowatt-hour per square meter per year
- `m/s` for meter per second, `m²/s` for square meter per second, `m³/s` for cubic meter per second
- `Pa` for pascal, `kPa` for kilopascal, `MPa` for megapascal
- ...

A list of attributes is semantically equivalent to nested dictionaries where the key is the **name** and the value is the **value**.

## Tag

A `tag` is a kebab-cased **name**.

## Plane

A `plane` is a location (**origin**) and orientation (**x-axis**, **y-axis** and derived z-axis) in 3D space.

The coordinate system is left-handed where the thumb points up into the direction of the z-axis, the index-finger forwards into the direction of the y-axis and the middle-finger points to the right into the direction of the x-axis.

## Url

A `url` is either _relative_ (to the root of the `.zip` file) or _remote_ (http, https, ftp, ...) string.

A _relative_ `url` is a `/`-normalized path to a file in the `.zip` file and is not prefixed with with `.`, `./`, `/`, ....

## Quality

A `quality` is a measurement definition with a **key**, **name**, **description**, **kind** (General, Design, Type, Piece, Connection, Port), **unit information** (SI and Imperial), **range constraints** (min/max with exclusion flags), **default value**, and optional **formula**.

A `quality` can be **scalable** (adjusts with piece scaling) and have multiple **benchmarks** for performance evaluation.

The **kind** determines which entities the quality can be applied to using a bitwise enum system.

## Benchmark

A `benchmark` is a performance standard within a `quality` with a **name**, optional **icon**, and **range** (min/max with exclusion flags).

Benchmarks provide reference points for evaluating quality measurements against industry or design standards.

## Concept

A `concept` is a **name** and **order** pair that provides semantic grouping for `kits`, `types`, or `designs`.

Concepts enable hierarchical organization and categorization of design elements beyond simple naming.

## Author

An `author` has a **name** and **email** and can be associated with `kits`, `types`, or `designs` with a **rank** indicating contribution level.

Authors provide attribution and contact information for design ownership and collaboration.

## Layer

A `layer` is an organizational grouping within a `design` with a **name**, optional **description**, and **color** for visual organization.

Layers provide a way to group and manage pieces logically within complex designs.

## Group

A `group` is a collection of `pieces` within a `design` with optional **name**, **description**, **color**, and **attributes**.

Groups enable semantic clustering of pieces that belong together functionally or conceptually.

## Prop

A `prop` is a **key-value** pair on a `port` that references a `quality` with a specific **value** and optional **unit**.

Props define measurable characteristics of ports using the quality system for standardized measurement.

## Stat

A `stat` is a statistical measurement on a `design` that references a `quality` with **range** (min/max) and optional **unit**.

Stats provide computed or measured performance data for entire designs using the quality framework.

# Monorepo

## Rules

### General

- ALWAYS finish everything without asking in between.
- NEVER interrupt between TODOs or tickets.
- NEVER remove functionality. Not even to get the code to work quickly.
- ALWAYS be thorough.
- NEVER create scripts to automate manual tasks.
- NEVER leave a placeholder.
- NEVER stop halfways and ask if you should continue.
- If a task is too big, ALWAYS start with one small part and ALWAYS finish it and keep on as much as you can.
- ALWAYS finish the task.
- ALWAYS make the choice directly! If you have several options, don't ask in between, be opionionated and just go for it. Try to do as much as you can.
- ALWAYS toolfriendly over intuitive.
- NEVER create new files unless for temporary purposes.
- NEVER create new folders unless for temporary purposes.
- NEVER worry about breaking compatiblity.
- NEVER create additional example files and implement it directly in the dependent parts.
- NEVER remove code that is commented out.
- NEVER add comments to the code. Especially not to communicate to the user.
- NEVER ask to run a command where you are not using the output. All dev servers, debugging and testing processes are running.
- NEVER run modifying `git` commands. Only read-only `git`commands are allowed. If you messed up, ALWAYS fix the file.
- NEVER add comments to the code.
- NEVER create tests unless you are explicitly asked to.
- ALWAYS use inline syntax if possible.
- NEVER add two statements into the same line.
- ALWAYS inline code.
- NEVER create a variable, function, … class, that is only used once and inline it.
- NEVER add extra new lines inside of code.
- Whenever adding ui elements ALWAYS use i18n setups and provide translations for the existing languages.
- ALWAYS add `[ORIGIN] `(replace ORIGIN with a file, function, class, … name to identify the origin) prefix to temporary logs so that `\[*\]*` can be used to filter them out.
- NEVER build or run the code.

### Styling

- NEVER use hardcoded (hex, rgb, …) or standard colors. All theme colors are explicitly defined.
- ALWAYS use colors for light mode. Dark mode is automatically derived. There are scales for the following number of colors: 2 (dark, light), 3 (dark, gray, light), 4 (dark, dark-gray-gray, light-gray-gray, light), 5 (dark, dark-gray, gray, light-gray, light), 6 (dark, dark-gray-gray, gray, light-gray-gray, light), 7 (dark, dark-6-7, dark-5-7, gray, light-5-7, light-6-7, light), 8 (dark, d-d-d-g, dark-gray, d-g-g-g, light-gray, l-g-g-g, l-l-l-g, light), 9 (dark, dark-8-9, dark-7-9, dark-gray, gray, light-gray, light-7-9, light-8-9, light), 10 (gray-100, gray-200, gray-300, gray-400, gray, gray-600, gray-700, gray-800, gray-900, light), 11 (dark, gray-100, dark-gray-gray, dark-gray, gray, light-gray, light-gray-gray, light-light-gray, l-l-l-g, gray-900, light). ALWAYS pick the one with the highest contrast.
- All closed ui elements ALWAYS have a border.
- NEVER use hardcoded pixels. ALWAYS use the existing ui frameworks with relative units.
- NEVER use rounded corners unless a circle.
- NEVER use shadows.
- Whenever a ui element can be interacted (left/right clicked with/without hold or modifier keys, dragged, …) with, ALWAYS make it visible (different hover color, different cursor, tooltip, …).
- The ui ALWAYS consists of three layers: 1. base, 2. panel and 3. temporary. Every layer has a darker background color and is on top of the previous layer. Every ui element ALWAYS has an enum for the layer and hence ALWAYS has three different color sets.
- ALWAYS indicate on the element and the cursor when it is interactive. Clickable elements have a pointer cursor and a hover effect. Dragable elements have a grab cursor. While dragging, the cursor changes to a grabbing cursor.

## Glossary

### Nouns

- Kit: A collection of qualities,types and designs. Can be either static (a special .zip file) or dynamic (bound to a runtime).
- Design: An undirected graph of pieces (nodes) and connections (edges).
- Type: A reusable component with different representations and ports.
- Piece: An instance of either a type or a design.
- Port: A conceptual connection point with an outwards direction.
- Connection: A 3D-Link between two pieces with translation parameters (gap, shift, rise) and rotation parameters (rotation, turn, tilt).
- Representation: A tagged url to a resource with an optional description.
- Attribute: Metadata with a name, an optional value, an optional unit and an optional definition (url or text).
- Tag: A kebab-cased name.
- Plane: A location (origin) and orientation (x-axis, y-axis and derived z-axis) in 3D space.
- Url: Either relative (to the root of the .zip file) or remote (http, https, ftp, …) string.
- Cluster: A group of connected pieces.
- Hierarchy: The length of the shortest path to the next fixed piece.
- Vector: A vector in 3D space.
- Point: A point in 3D space.

### Adjectives

- A `fixed` piece is a piece with a plane.
- A `linked` piece is a piece that is not fixed and is connected with a connection.
- A `connected` piece that is not `fixed` and is connected to at least one other piece.
- A `flat` design has no connections and all pieces are fixed.
- A `mandatory` port is a port that must be connected in a design.
- A `static` kit is a special .zip file.
- A `dynamic` kit is bound to a runtime.
- A `relative` url is relative to the root of the .zip file.
- A `remote` url is http, https, ftp, etc.
- A `default` representation has no tags.
- A `default` port family means the port is compatible with all other ports.
- A `virtual` type is an intermediate type that needs other `virtual` types to form a `physical` type.

## File Structure

The folders and files are listed like this: [PATH] [DISKNAME]? # [NAME | SHORTNAME | …]? [SUMMARY]?

├── .claude
│ ├── agents
│ │ ├── reformatter.md # Exclusively to reformat text (code, lists, …)
│ │ └── reorderer.md # Exclusively to reorder text (code, lists, …)
│ │ └── schema-changer.md # Exclusively to change the schema (code, api, database, …)
│ └── settings.json
├── .cursor
│ ├── rules
│ │ └── repo.mdc # \*_/_.\*
├── .github
│ ├── chatmodes
│ │ ├── Reformatter.chatmode.md # Exclusively to reformat text (code, lists, …)
│ │ └── Reorderer.chatmode.md # Exclusively to reorder text (code, lists, …)
│ │ └── Schema-Changer.chatmode.md # Exclusively to change the schema (code, api, database, …)
│ ├── workflows
│ │ └── gh-pages.yml # Deploy user docs togh-pages
│ └── dependabot.yml
├── .vscode
├── antlr
├── assets # @semio/gh: assets for the complete repo
│ ├── badges
│ ├── contributors
│ ├── cursors
│ ├── fonts
│ ├── grasshopper
│ ├── icons
│ ├── images
│ ├── lists
│ ├── logo
│ ├── models
│ └── semio
├── engineering
│ ├── dataarchitecture.pu # blueprint for sql schemas
│ ├── interfacearchitecture.txt # blueprint for json-based (rest api, graphql api, copy&paste) schemas
│ └── softwarearchitecture.txt # blueprint for object-oriented code
├── examples
│ ├── geometry
│ ├── hello-semio
│ ├── metabolism # main example with all features
│ ├── starters
│ ├── urban-patterns
│ └── voxels
├── graphql
├── js
│ ├── ai
│ ├── desktop
│ │ └── package.json
│ ├── docs
│ │ └── package.json
│ ├── js # @semio/js: all shared js code (ui, domain logic, configs, …)
│ │ ├── .storybook
│ │ ├── elements
│ │ │ ├── aggregation
│ │ │ │ ├── Accordion.stories.tsx
│ │ │ │ ├── Accordion.tsx
│ │ │ │ ├── Collapsible.stories.tsx
│ │ │ │ ├── Collapsible.tsx
│ │ │ │ ├── Dialog.stories.tsx
│ │ │ │ ├── Dialog.tsx
│ │ │ │ ├── Resizable.stories.tsx
│ │ │ │ ├── Resizable.tsx
│ │ │ │ ├── ScrollArea.stories.tsx
│ │ │ │ ├── ScrollArea.tsx
│ │ │ │ ├── Tabs.stories.tsx
│ │ │ │ ├── Tabs.tsx
│ │ │ │ ├── Tree.stories.tsx
│ │ │ │ ├── Tree.tsx
│ │ │ │ └── TreeStateProvider.tsx
│ │ │ ├── display
│ │ │ │ ├── Avatar.stories.tsx
│ │ │ │ ├── Avatar.tsx
│ │ │ │ ├── HoverCard.stories.tsx
│ │ │ │ ├── HoverCard.tsx
│ │ │ │ ├── Icons.stories.tsx
│ │ │ │ ├── Icons.tsx
│ │ │ │ ├── Tooltip.stories.tsx
│ │ │ │ └── Tooltip.tsx
│ │ │ ├── docs
│ │ │ │ ├── Aside.tsx
│ │ │ │ ├── Card.tsx
│ │ │ │ ├── FileTree.tsx
│ │ │ │ ├── Page.tsx
│ │ │ │ ├── Section.tsx
│ │ │ │ ├── Steps.tsx
│ │ │ │ ├── Tabs.tsx
│ │ │ │ └── index.ts
│ │ │ ├── input
│ │ │ │ ├── Action.stories.tsx
│ │ │ │ ├── Action.tsx
│ │ │ │ ├── Button.stories.tsx
│ │ │ │ ├── Button.tsx
│ │ │ │ ├── ButtonGroup.stories.tsx
│ │ │ │ ├── ButtonGroup.tsx
│ │ │ │ ├── Combobox.stories.tsx
│ │ │ │ ├── Combobox.tsx
│ │ │ │ ├── Input.stories.tsx
│ │ │ │ ├── Input.tsx
│ │ │ │ ├── Select.stories.tsx
│ │ │ │ ├── Select.tsx
│ │ │ │ ├── Slider.stories.tsx
│ │ │ │ ├── Slider.tsx
│ │ │ │ ├── Stepper.stories.tsx
│ │ │ │ ├── Stepper.tsx
│ │ │ │ ├── Textarea.stories.tsx
│ │ │ │ ├── Textarea.tsx
│ │ │ │ ├── Toggle.stories.tsx
│ │ │ │ ├── Toggle.tsx
│ │ │ │ ├── ToggleGroup.stories.tsx
│ │ │ │ └── ToggleGroup.tsx
│ │ │ ├── navigation
│ │ │ │ ├── Breadcrumb.stories.tsx
│ │ │ │ └── Breadcrumb.tsx
│ │ │ ├── panels
│ │ │ │ ├── BottomPanel.tsx
│ │ │ │ ├── LeftPanel.tsx
│ │ │ │ ├── MiddlePanel.tsx
│ │ │ │ ├── Panel.tsx
│ │ │ │ ├── PanelGroup.tsx
│ │ │ │ └── RightPanel.tsx
│ │ │ ├── windows
│ │ │ │ ├── Diagram.tsx
│ │ │ │ ├── Scene.tsx
│ │ │ │ ├── Table.tsx
│ │ │ │ └── Window.tsx
│ │ │ ├── Canvas.stories.tsx
│ │ │ ├── Canvas.tsx
│ │ │ ├── Command.stories.tsx
│ │ │ ├── Command.tsx
│ │ │ ├── Footer.stories.tsx
│ │ │ ├── Footer.tsx
│ │ │ ├── Layout.stories.tsx
│ │ │ ├── Layout.tsx
│ │ │ ├── Navbar.stories.tsx
│ │ │ ├── Navbar.tsx
│ │ │ ├── Popover.stories.tsx
│ │ │ ├── Popover.tsx
│ │ │ └── index.ts
│ │ ├── locales
│ │ │ ├── de.json
│ │ │ └── en.json
│ │ ├── sketchpad
│ │ │ ├── apps
│ │ │ │ ├── index.tsx
│ │ │ │ ├── registry.tsx
│ │ │ │ ├── design
│ │ │ │ │ ├── canvas
│ │ │ │ │ │ ├── Diagram.tsx
│ │ │ │ │ │ └── Scene.tsx
│ │ │ │ │ ├── panels
│ │ │ │ │ │ ├── Details.tsx
│ │ │ │ │ │ ├── Settings.tsx
│ │ │ │ │ │ └── Workbench.tsx
│ │ │ │ │ ├── tools
│ │ │ │ │ │ ├── index.tsx
│ │ │ │ │ │ ├── LassoTool.tsx
│ │ │ │ │ │ └── SelectionTool.tsx
│ │ │ │ │ ├── commands.ts
│ │ │ │ │ ├── config.ts # Auto-discovered app config
│ │ │ │ │ ├── App.tsx
│ │ │ │ │ ├── store.tsx
│ │ │ │ │ └── Tools.tsx
│ │ │ │ ├── docs
│ │ │ │ │ ├── canvas
│ │ │ │ │ │ └── Page.tsx
│ │ │ │ │ ├── panels
│ │ │ │ │ │ ├── Details.tsx
│ │ │ │ │ │ ├── Settings.tsx
│ │ │ │ │ │ └── Workbench.tsx
│ │ │ │ │ ├── commands.ts
│ │ │ │ │ ├── config.ts # Auto-discovered app config
│ │ │ │ │ ├── App.tsx
│ │ │ │ │ ├── mdx-loader.ts
│ │ │ │ │ ├── mdx-provider.tsx
│ │ │ │ │ ├── registry.ts
│ │ │ │ │ └── store.tsx
│ │ │ │ ├── home
│ │ │ │ │ ├── canvas
│ │ │ │ │ │ └── Table.tsx
│ │ │ │ │ ├── commands.ts
│ │ │ │ │ ├── config.ts # Auto-discovered app config
│ │ │ │ │ ├── App.tsx
│ │ │ │ │ └── store.tsx
│ │ │ │ ├── kit
│ │ │ │ │ ├── canvas
│ │ │ │ │ │ └── Table.tsx
│ │ │ │ │ ├── panels
│ │ │ │ │ │ ├── Details.tsx
│ │ │ │ │ │ └── Settings.tsx
│ │ │ │ │ ├── commands.ts
│ │ │ │ │ ├── config.ts # Auto-discovered app config
│ │ │ │ │ ├── App.tsx
│ │ │ │ │ └── store.tsx
│ │ │ │ ├── quality
│ │ │ │ │ ├── canvas
│ │ │ │ │ │ ├── Diagram.tsx
│ │ │ │ │ │ └── Formula.tsx
│ │ │ │ │ ├── panels
│ │ │ │ │ │ ├── Details.tsx
│ │ │ │ │ │ ├── Settings.tsx
│ │ │ │ │ │ └── Workbench.tsx
│ │ │ │ │ ├── tools
│ │ │ │ │ │ └── index.tsx
│ │ │ │ │ ├── commands.ts
│ │ │ │ │ ├── config.ts # Auto-discovered app config
│ │ │ │ │ ├── App.tsx
│ │ │ │ │ ├── functions.ts
│ │ │ │ │ ├── store.tsx
│ │ │ │ │ └── Tools.tsx
│ │ │ │ └── type
│ │ │ │ ├── canvas
│ │ │ │ │ └── Scene.tsx
│ │ │ │ ├── panels
│ │ │ │ │ ├── Details.tsx
│ │ │ │ │ ├── Settings.tsx
│ │ │ │ │ └── Workbench.tsx
│ │ │ │ ├── tools
│ │ │ │ │ ├── index.tsx
│ │ │ │ │ ├── PortTool.tsx
│ │ │ │ │ └── SelectionTool.tsx
│ │ │ │ ├── commands.ts
│ │ │ │ │ ├── config.ts # Auto-discovered app config
│ │ │ │ ├── App.tsx
│ │ │ │ ├── store.tsx
│ │ │ │ └── Tools.tsx
│ │ │ ├── docs
│ │ │ │ ├── getting-started
│ │ │ │ │ ├── installation.mdx
│ │ │ │ │ ├── intro
│ │ │ │ │ │ ├── think-in-semio.mdx
│ │ │ │ │ │ └── why-semio.mdx
│ │ │ │ │ └── starter.mdx
│ │ │ │ ├── integrations
│ │ │ │ ├── manuals
│ │ │ │ ├── showcases
│ │ │ │ │ └── metabolism.mdx
│ │ │ │ ├── theory
│ │ │ │ │ ├── design-information-modeling.mdx
│ │ │ │ │ ├── graphs.mdx
│ │ │ │ │ └── kit-of-parts-architecture.mdx
│ │ │ │ ├── tutorials
│ │ │ │ │ └── hello-semio
│ │ │ │ │ │ ├── model-brick-set.mdx
│ │ │ │ │ │ ├── model-design.mdx
│ │ │ │ │ │ ├── save-kit.mdx
│ │ │ │ │ │ ├── show-design.mdx
│ │ │ │ │ │ └── sketch-setup.mdx
│ │ │ │ └── index.mdx
│ │ │ ├── kits
│ │ │ │ ├── commands.ts
│ │ │ │ └── store.tsx
│ │ │ ├── panels
│ │ │ │ ├── Chat.tsx
│ │ │ │ ├── Details.tsx
│ │ │ │ ├── Hud.tsx
│ │ │ │ ├── Settings.tsx
│ │ │ │ ├── Stats.tsx
│ │ │ │ ├── Toolbar.tsx
│ │ │ │ ├── Tools.tsx
│ │ │ │ └── Workbench.tsx
│ │ │ ├── Canvas.tsx
│ │ │ ├── commands.ts
│ │ │ ├── App.tsx
│ │ │ ├── Footer.tsx
│ │ │ ├── Navbar.tsx
│ │ │ ├── Panel.tsx
│ │ │ ├── Sketchpad.stories.tsx
│ │ │ ├── Sketchpad.tsx # main component of @semio/js
│ │ │ └── store.tsx
│ │ ├── components.json
│ │ ├── constants.json
│ │ ├── eslint.config.ts
│ │ ├── globals.css
│ │ ├── i18n.ts
│ │ ├── index.ts
│ │ ├── package.json
│ │ ├── postcss.config.ts
│ │ ├── semio.ts # all domain logic
│ │ ├── tailwind.config.ts
│ │ ├── theme.css
│ │ ├── tsconfig.json
│ │ ├── vite.config.ts
│ │ └── vitest.workspace.ts
│ └── play
├── jsonschema
├── liveblocks
├── meta
├── net
│ ├── Semio
│ │ ├── Semio.cs # @semio/net: all .NET code
│ │ ├── UserObjects
│ │ │ ├── github
│ │ │ ├── gitlab
│ │ │ ├── monoceros
│ │ │ ├── semio
│ │ │ └── wasp
│ ├── Semio.Grasshopper
│ │ └── Semio.Grasshopper.cs # @semio/gh: all grasshopper code
│ ├── Semio.Grasshopper.Tests
│ └── Semio.Tests
├── rb
├── rdf
├── scripts
├── sqlite
├── yak
├── .gitignore
├── .gitmodules
├── .prettierignore
├── .prettierrc.json
├── AGENTS.md # All general ai information
├── CITATION.cff
├── CLAUDE.md # Claude specific
├── nx.json # Nx targets and plugin configs
├── package-lock.json # All javascript dependencies
├── package.json # Monorepo and workspace setup
├── powershell.ps1 # General Powershell utility
└── README.md # GFM dev docs

In general, if the user talks about an old file, then probably there is the same file with the suffix `*.old` that is the original state.

# Ecosystems

## js

Javascript code with shared core (@semio/js) that uses storybook and exports a handful of React components (Sketchpad, Diagram, Model) for both web-based and desktop-based environments, a documentation (@semio/docs) that uses astro with starlight and mdx, and desktop (@semio/desktop) that runs in electron.

### Rules

- NEVER use inline styling. Use tailwindcss (v4). v4 uses a `theme.css` (`@semio/js/theme.css`) for theming and not `{theme:{…}}` in `tailwindconfig`.
- ALWAYS use colors defined in `@theme inline {…}` from `js/js/globals.css`. NEVER use direct colors such as light, gray, …, dark, primary, secondary, tertiary outside of `js/js/globals.css` and ALWAYS use semantic colors instead such as active, disabled, hover, …
- ALWAYS add tooltips (normal and extensive) to all ui elements.

### Styling

- The ui consists of a three horizontal strips: navbar, canvas and footer. A canvas consists of windows. On top of the canvas are panels which can toggled on and off.

# Packages

## @semio/js

Shared react components. The main component is Sketchpad. Sketchpad is used in three different szenarios:

1. As guest mode (readonly) in a statically generated pages.
2. As user mode in the browser (nextjs).
3. As user mode in a desktop app (electron).
   Sketchpad has a local store in yjs which syncs with indexeddb and the backend provider.

### Rules

#### General

- Domain logic is ALWAYS in semio.ts and whenever an operation is not ui bound, it should be implemented there.
- State managment ALWAYS is in the corresponding store.tsx. State is ALWAYS accessed over hooks. There are internal hooks (e.g. store accessors) that are NEVER used directly in components. Mutation ALWAYS are executed via commands. NEVER use useState or other local state in components.
- There is a transaction mechanism for kits. Every app transaction is an extended kit transaction. The undo redo manager is on app level and stores the diff of the transaction along with the app state. This way undo redo works even when the kit changes because only the diff is stored. The inverted diff is stored along with the diff to enable relative undo redo.

#### Architecture - Open-Closed Principle

The codebase follows the Open-Closed Principle (OCP): closed for modification, open for extension. Adding new features ONLY requires adding new files/folders, NEVER modifying existing ones.

##### Adding a New App

To add a new app:

1. Create a folder under `js/js/sketchpad/apps/{appname}/`
2. Add `config.ts` exporting `AppConfig`
3. Add `App.tsx` with default export FC
4. Add `store.tsx` with app state
5. Optionally add `canvas/`, `panels/`, `tools/`, `commands.ts`

The app is automatically discovered via `import.meta.glob('./*/config.ts')`.

Example `config.ts`:

```typescript
import { AppConfig } from "../registry";
import MyApp from "./App";

export const config: AppConfig = {
  id: "myapp",
  component: MyApp,
  routeSegments: [{ path: "my/:id", paramName: "id" }],
  getPanels: (t) => [{ key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" }],
  matchesPath: (pathParts) => pathParts[0] === "my",
  order: 50,
};
```

##### Adding a New Tool

To add a new tool to an app:

1. Create a file `apps/{app}/tools/*Tool.tsx`
2. Export tool objects with `id` and `render` properties

Tools are automatically discovered via `import.meta.glob('./*Tool.tsx')`.

Example:

```typescript
export const MyTool: Tool<MyAppState> = {
  id: ToolType.MY_TOOL,
  label: "My Tool",
  icon: <Icon />,
  render: (context) => ({ scene: <></>, diagram: null, table: null }),
};
```

##### Adding Panel Sections

Panel sections are dynamically added in the app's `useEffect`:

```typescript
useEffect(() => {
  addSection("details", {
    id: "my-section",
    label: t("mySection"),
    content: () => <MyComponent />,
    order: 1,
  });
  return () => removeSection("details", "my-section");
}, [appType, addSection, removeSection]);
```

#### Styling

- NEVER use colors and spacing directly. ALWAYS use semantic variables from global.css. Only global.css uses colors and pixels directly.

### Store Architecture

This document describes the generalized store hierarchy for the Semio application.

#### Overview

The store architecture consists of three levels of abstraction:

1. **Store** - Base class for any component with data
2. **AppStore** - Base class for apps with transaction support and undo/redo
3. **KitDiffAppStore** - Base class for apps that modify kits and track both app-specific and kit diffs

#### Store Hierarchy

```
Store<TState>
  ↓ extends
AppStore<TState, TDiff, TSelectionDiff, TEdit, TCommandContext, TCommandResult>
  ↓ extends
KitDiffAppStore<TState, TDiff, TSelectionDiff, TEdit, TCommandContext, TCommandResult>
```

#### 1. Store (Base Class)

The `Store` class is the foundation for all components that hold data.

##### Responsibilities

- State management with snapshot caching
- Observable pattern (onChanged, onChangedDeep)
- Access to parent SketchpadStore
- Y.js integration via yMap

##### Abstract Methods

- `hash(state: TState): string` - Generate a hash for cache invalidation
- `buildSnapshot(): TState` - Build the current state snapshot

##### Usage

Use this for simple components that only need state management without editing capabilities (e.g., HomeStore).

#### 2. AppStore (extends Store)

The `AppStore` adds transaction support with undo/redo functionality for any app.

##### Responsibilities

- Transaction management (start, finalize, abort)
- Undo/redo with two stacks:
  - **Current transaction stack**: Edits in the active transaction (merged on finalize)
  - **Past transactions stack**: Finalized transactions
- Selection management with diff-based updates
- Panel visibility and fullscreen management

##### Transaction Model

Every app supports transactions:

1. **Start Transaction**: `startTransaction()`
   - Activates transaction mode
   - New edits go to current transaction stack

2. **During Transaction**: `executeCommand(...)`
   - Each command creates an edit with `do` and `undo` steps
   - Edits accumulate in current transaction stack
   - Undo/redo work within the current transaction

3. **Finalize Transaction**: `finalizeTransaction()`
   - Merges all edits in current transaction into one edit
   - Moves merged edit to past transactions stack
   - Clears redo stack

4. **Abort Transaction**: `abortTransaction()`
   - Undoes all edits in current transaction
   - Clears current transaction stack

##### Edit Structure

```typescript
interface AppEdit<TSelectionDiff> {
  do: AppStep<TSelectionDiff>;
  undo: AppStep<TSelectionDiff>;
}

interface AppStep<TSelectionDiff> {
  selectionDiff?: TSelectionDiff;
}
```

Each edit stores:

- **do**: Forward diff to apply the change
- **undo**: Inverse diff to revert the change

### Abstract Methods (in addition to Store)

- `applySelectionDiff(selectionDiff: TSelectionDiff): void` - Apply selection changes to Y.js
- `inverseSelectionDiff(selection, diff): TSelectionDiff` - Calculate inverse diff for undo
- `getSelection()` - Get current selection state

##### Undo/Redo Behavior

**Within Transaction:**

- Undo: Pops from current transaction stack, stores in temp variable
- Redo: Pushes temp variable back to current transaction stack

**Outside Transaction:**

- Undo: Moves edit from past transactions stack to redo stack
- Redo: Moves edit from redo stack back to past transactions stack

##### Usage

Use this for apps that don't modify kits (e.g., HomeStore for managing the home screen).

#### 3. KitDiffAppStore (extends AppStore)

The `KitDiffAppStore` extends AppStore for apps that modify kits (designs, types).

##### Additional Responsibilities

- Tracks kit diffs alongside app-specific diffs
- Applies kit changes through KitStore
- Records both app and kit changes in edits

##### Edit Structure

```typescript
interface KitDiffAppEdit<TSelectionDiff> {
  do: KitDiffAppStep<TSelectionDiff>;
  undo: KitDiffAppStep<TSelectionDiff>;
}

interface KitDiffAppStep<TSelectionDiff> {
  kitDiff?: KitDiff;
  selectionDiff?: TSelectionDiff;
}
```

Each edit stores:

- **do.kitDiff**: Forward kit diff to apply changes
- **do.selectionDiff**: Forward selection diff
- **undo.kitDiff**: Inverse kit diff to revert changes
- **undo.selectionDiff**: Inverse selection diff

##### Undo/Redo Behavior

Extends AppStore undo/redo to also:

- Apply/revert kit diffs through `kit().change(kitDiff)`
- Handle both kit and selection changes atomically

##### Abstract Methods

- `kit(): KitStore` - Get the associated kit store

##### Usage

Use this for apps that modify kits:

- **DesignAppStore** - Edit designs (pieces, connections)
- **TypeAppStore** - Edit types (ports, representations)
- **KitAppStore** - Edit kits (types, designs, qualities, files, authors)

#### Concrete Implementations

##### DesignAppStore

Edits design content:

- Selection: pieces, connections, ports
- Kit diffs: piece changes, connection changes
- Transaction support for complex multi-step operations

##### TypeAppStore

Edits type definitions:

- Selection: ports, representations
- Kit diffs: port changes, representation changes
- Transaction support for type modifications

##### KitAppStore

Edits kit metadata:

- Selection: types, designs, qualities, files, authors
- Kit diffs: add/remove artifacts
- Transaction support for kit-level operations

##### HomeStore

Manages home screen (extends AppStore, not KitDiffAppStore):

- Selection: kits
- No kit diffs (doesn't modify kit content)
- Sorting and filtering state

#### Command Pattern

All apps use a command pattern:

```typescript
interface CommandContext {
  // Current state
}

interface CommandResult {
  diff?: TDiff;      // App-specific diff
  kitDiff?: KitDiff; // Kit diff (only for KitDiffAppStore)
}

executeCommand<T>(command: string, ...args): Promise<T>
```

##### Command Execution Flow

1. Look up command in registry
2. Build context with current state
3. Execute command function
4. Apply diffs (app diff + kit diff)
5. Record edit for undo/redo
6. Return result

#### Best Practices

1. **Always use transactions** for multi-step operations
2. **Keep edits atomic** - each edit should be independently undoable
3. **Calculate inverse diffs correctly** - critical for undo
4. **Don't nest transactions** - finish one before starting another
5. **Clear redo stack on new edits** - standard undo/redo behavior
6. **Use selection diffs** for all selection changes

#### Files

- `js/js/sketchpad/store.tsx` - Base Store, AppStore, KitDiffAppStore
- `js/js/sketchpad/apps/design/store.tsx` - DesignAppStore
- `js/js/sketchpad/apps/type/store.tsx` - TypeAppStore
- `js/js/sketchpad/apps/kit/store.tsx` - KitAppStore
- `js/js/sketchpad/apps/home/store.tsx` - HomeStore

# Hierarchies

Use this hierarchy for code organization (order of appearance of regions, classes, properties, functions, methods, types, statements, constants, …).

## 1. Models

1. Attribute
2. Coord
3. Vec
4. Point
5. Vector
6. Plane
7. Camera
8. Location
9. Author
10. File
11. Benchmark
12. QualityKind
13. Quality
14. Prop
15. Representation
16. Port
17. Type
18. Layer
19. Piece
20. Group
21. Side
22. Connection
23. Stat
24. Design
25. Kit

## 2. Classes | Types

1. Model
2. Id
3. Shallow
4. Diff
5. Diffs
6. Input
7. Output
8. Context
9. Prediction

## 3. Properties

### Attribute

1. Key
2. Value
3. Definition

### Coord

1. X
2. Y

### Vec

1. X
2. Y

### Point

1. X
2. Y
3. Z

### Vector

1. X
2. Y
3. Z

### Plane

1. Origin
2. XAxis
3. YAxis

### Camera

1. Position
2. Forward
3. Up

### Location

1. Longitude
2. Latitude
3. Altitude
4. Attributes

### Author

1. Name
2. Email
3. Attributes

### File

1. Path
2. RemoteUrl
3. Description
4. Attributes

### Benchmark

1. Name
2. Icon
3. Min
4. MinExcluded
5. Max
6. MaxExcluded
7. Definition
8. Attributes

### QualityKind

1. General
2. Type
3. Design
4. Piece
5. Connection
6. Port

### Quality

1. Key
2. Name
3. Kind
4. Default
5. Formula
6. DefaultSiUnit
7. DefaultImperialUnit
8. Min
9. MinExcluded
10. Max
11. MaxExcluded
12. CanScale
13. Benchmarks
14. Definition
15. Attributes

### Prop

1. Key
2. Value
3. Unit
4. Attributes

### Representation

1. Tags
2. Url
3. Description
4. Attributes

### Port

1. Id
2. Point
3. Direction
4. T
5. Mandatory
6. Family
7. CompatibleFamilies
8. Description
9. Attributes

### Type

1. Name
2. Variant
3. Representations
4. Ports
5. Props
6. IsVirtual
7. CanScale
8. CanMirror
9. Unit
10. AvailableCount
11. Location
12. Authors
13. Concepts
14. Icon
15. Image
16. Description
17. Attributes

### Layer

1. Path
2. IsHidden
3. IsLocked
4. Color
5. Description
6. Attributes

### Group

1. Pieces
2. Color
3. Name
4. Description
5. Attributes

### Piece

1. Id
2. Type
3. Design
4. Plane
5. Center
6. Scale
7. MirrorPlane
8. Props
9. IsHidden
10. IsLocked
11. Color
12. Description
13. Attributes

### Side

1. Piece
2. DesignPiece
3. Port

### Connection

1. Connected
2. Connecting
3. Gap
4. Shift
5. Rise
6. Rotation
7. Turn
8. Tilt
9. X
10. Y
11. Description
12. Attributes

### Design

1. Name
2. Variant
3. View
4. Pieces
5. Connections
6. Stats
7. Props
8. Layers
9. ActiveLayer
10. Groups
11. CanScale
12. CanMirror
13. Unit
14. Location
15. Authors
16. Concepts
17. Icon
18. Image
19. Description
20. Attributes

### Kit

1. Name
2. Version
3. Types
4. Designs
5. Qualities
6. Files
7. Authors
8. RemoteUrl
9. HomepageUrl
10. License
11. Concepts
12. Icon
13. Image
14. Description
15. Attributes
