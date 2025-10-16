# Monorepo

This document MUST ALWAYS be followed unless explicitly asked to do otherwise.

## Rules

### General

- NEVER remove functionality! Not even to get the code to work quickly.
- ALWAYS be thorough.
- NEVER create scripts to automate manual tasks.
- NEVER leave a placeholder.
- NEVER stop halfways and ask if you should continue.
- If a task is to big, ALWAYS start with one small part and ALWAYS finish it and keep on as much as you can.
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
- ALWAYS add `[ORIGIN] ` prefix to temporary logs so that `\[*\]*` can be used to filter them out.

### Styling

- NEVER use hardcoded (hex, rgb, …) or standard colors. All theme colors are explicitly defined.
- ALWAYS use colors for light mode. Dark mode is automatically derived. There are scales for the following number of colors: 2 (dark, light), 3 (dark, gray, light), 4 (dark, dark-gray-gray, light-gray-gray, light), 5 (dark, dark-gray, gray, light-gray, light), 6 (dark, dark-gray-gray, gray, light-gray-gray, light), 7 (dark, dark-6-7, dark-5-7, gray, light-5-7, light-6-7, light), 8 (dark, d-d-d-g, dark-gray, d-g-g-g, light-gray, l-g-g-g, l-l-l-g, light), 9 (dark, dark-8-9, dark-7-9, dark-gray, gray, light-gray, light-7-9, light-8-9, light), 10 (gray-100, gray-200, gray-300, gray-400, gray, gray-600, gray-700, gray-800, gray-900, light), 11 (dark, gray-100, dark-gray-gray, dark-gray, gray, light-gray, light-gray-gray, light-light-gray, l-l-l-g, gray-900, light). ALWAYS pick the one with the highest contrast.
- All closed ui elements ALWAYS have a border.
- NEVER use hardcoded pixels. ALWAYS use the existing ui frameworks with relative units.
- NEVER use rounded corners unless a circle.
- NEVER use shadows.
- Whenever a ui element can be interacted (left/right clicked with/without hold or modifier keys, dragged, …) with, ALWAYS make it visible (different hover color, different cursor, tooltip, …).
- The ui ALWAYS consists of three layers: 1. base, 2. panel and 3. temporary. Every layer has a darker background color and is on top of the previous layer. Every ui element ALWAYS has an enum for the layer and hence ALWAYS has three different color sets.

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
│ │ │ │ └── Tree.tsx
│ │ │ ├── display
│ │ │ │ ├── Avatar.stories.tsx
│ │ │ │ ├── Avatar.tsx
│ │ │ │ ├── HoverCard.stories.tsx
│ │ │ │ ├── HoverCard.tsx
│ │ │ │ ├── Icons.stories.tsx
│ │ │ │ ├── Icons.tsx
│ │ │ │ ├── Tooltip.stories.tsx
│ │ │ │ └── Tooltip.tsx
│ │ │ ├── input
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
│ │ │ │ ├── ToggleCycle.stories.tsx
│ │ │ │ ├── ToggleCycle.tsx
│ │ │ │ ├── ToggleGroup.stories.tsx
│ │ │ │ └── ToggleGroup.tsx
│ │ │ ├── navigation
│ │ │ │ ├── Breadcrumb.stories.tsx
│ │ │ │ └── Breadcrumb.tsx
│ │ │ ├── Popover.stories.tsx
│ │ │ ├── Popover.tsx
│ │ │ ├── Scene.stories.tsx
│ │ │ └── Scene.tsx
│ │ ├── locales
│ │ │ ├── de.json
│ │ │ └── en.json
│ │ ├── sketchpad
│ │ │ ├── editors
│ │ │ │ ├── design
│ │ │ │ │ ├── canvas
│ │ │ │ │ │ ├── Diagram.tsx
│ │ │ │ │ │ └── Scene.tsx
│ │ │ │ │ ├── panels
│ │ │ │ │ │ ├── Details.tsx
│ │ │ │ │ │ ├── Settings.tsx
│ │ │ │ │ │ ├── Toolbar.tsx
│ │ │ │ │ │ └── Workbench.tsx
│ │ │ │ │ ├── Editor.tsx
│ │ │ │ │ └── store.tsx
│ │ │ │ ├── home
│ │ │ │ │ ├── canvas
│ │ │ │ │ │ └── Table.tsx
│ │ │ │ │ ├── Editor.tsx
│ │ │ │ │ └── store.tsx
│ │ │ │ ├── kit
│ │ │ │ │ ├── canvas
│ │ │ │ │ │ └── Table.tsx
│ │ │ │ │ ├── panels
│ │ │ │ │ │ └── Details.tsx
│ │ │ │ │ ├── Editor.tsx
│ │ │ │ │ └── store.tsx
│ │ │ │ └── type
│ │ │ │ ├── canvas
│ │ │ │ │ └── Scene.tsx
│ │ │ │ ├── panels
│ │ │ │ │ ├── Details.tsx
│ │ │ │ │ ├── Settings.tsx
│ │ │ │ │ └── Toolbar.tsx
│ │ │ │ ├── Editor.tsx
│ │ │ │ └── store.tsx
│ │ │ ├── panels
│ │ │ │ ├── Chat.tsx
│ │ │ │ ├── Details.tsx
│ │ │ │ ├── Settings.tsx
│ │ │ │ ├── Toolbar.tsx
│ │ │ │ └── Workbench.tsx
│ │ │ ├── Canvas.tsx
│ │ │ ├── Editor.tsx
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

- The ui consists of a navbar and edgeless content. Everything else is displayed as HUD with floating panels that show different colors.
-

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
- There is a transaction mechanism for kits. Every editor transaction is an extended kit transaction. The undo redo manager is on editor level and stores the diff of the transaction along with the editor state. This way undo redo works even when the kit changes because only the diff is stored. The inverted diff is stored along with the diff to enable relative undo redo.

#### Styling

- NEVER use colors and spacing directly. ALWAYS use semantic variables from global.css. Only global.css uses colors and pixels directly.

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
