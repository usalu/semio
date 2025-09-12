# Monorepo

## Project

An ecosystem for designing kit-of-parts architecture together.

## Rules

The following rules MUST alwaysbe followed unless explicitly asked to do otherwise.

- NEVER remove functionality! Not even to get the code to work quickly.
- ALWAYS be thorough.
- NEVER create scripts to automate manual tasks. If a task is long
- NEVER stop halfways and ask if you should continue.
- ALWAYS finish the task.
- ALWAYS make the choice directly! If you have several options, don't ask in between, be opionionated and just go for it. Try to do as much as you can.
- ALWAYS toolfriendly over intuitive.
- NEVER create new files unless for temporary purposes.
- NEVER create new folders unless for temporary purposes.
- NEVER worry about breaking compatiblity.
- NEVER create additional example files and implement it directly in the dependent parts.
- NEVER remove code that is commented out.
- NEVER add comments to the code.
- NEVER ask to run a command where you are not using the output. All dev servers, debugging and testing processes are running.
- NEVER run modifying `git` commands. Only read-only `git`commands are allowed. If you messed up, ALWAYS fix the file.
- NEVER add comments to the code.
- NEVER create tests unless you are explicitly asked to.
- ALWAYS use inline syntax if possible.
- NEVER add two statements into the same line.
- ALWAYS inline code.
- NEVER create a variable, function, … class, that is only used once and inline it.
- NEVER add extra new lines inside of code.

## Styling

- NEVER use rounded corners unless a circle.
- NEVER use shadows.
- NEVER use hardcoded or standard colors. All theme colors are explicitly defined.

## Glossary

### Nouns

- Kit: A collection of types and designs. Can be either static (a special .zip file) or dynamic (bound to a runtime).
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

├── .claude
│ ├── agents
│ │ ├── reformatter.md # Exclusively to reformat text (code, lists, …)
│ │ └── reorderer.md # Exclusively to reorder text (code, lists, …)
│ │ └── schema-changer.md # Exclusively to change the schema (code, api, database, …)
│ └── settings.json
├── .cursor
│ ├── rules
│ │ ├── js-core.mdc # js/core/**/_._
│ │ ├── js.mdc # js/**/_._
│ │ └── repo.mdc # \*_/_.\*
├── .github
│ ├── chatmodes
│ │ ├── Reformatter.chatmode.md # Exclusively to reformat text (code, lists, …)
│ │ └── Reorderer.chatmode.md # Exclusively to reorder text (code, lists, …)
│ │ └── Schema-Changer.chatmode.md # Exclusively to change the schema (code, api, database, …)
│ └── workflows
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
│ ├── core # @semio/js: all shared js code (ui, domain logic, configs, …)
│ │ ├── components
│ │ │ ├── ui
│ │ │ │ ├── sketchpad
│ │ │ │ │ │ ├── Chat.tsx
│ │ │ │ │ │ ├── Console.tsx
│ │ │ │ │ │ ├── DesignEditor.tsx
│ │ │ │ │ │ ├── Diagram.tsx
│ │ │ │ │ │ ├── Model.tsx
│ │ │ │ │ │ ├── Navbar.tsx
│ │ │ │ │ │ ├── Workbench.tsx
│ │ │ │ │ │ ├── Sketchpad.stories.tsx
│ │ │ │ │ │ ├── Sketchpad.tsx # main component of @semio/js
│ │ │ │ │ │ ├── TypeEditor.tsx
│ │ │ │ │ │ └── Workbench.tsx
│ │ │ │ │ ├── Accordion.tsx
│ │ │ │ │ ├── Avatar.tsx
│ │ │ │ │ ├── Breadcrumb.tsx
│ │ │ │ │ ├── Button.tsx
│ │ │ │ │ ├── Collapsible.tsx
│ │ │ │ │ ├── Commands.tsx
│ │ │ │ │ ├── Combobox.tsx
│ │ │ │ │ ├── Dialog.tsx
│ │ │ │ │ ├── File.tsx
│ │ │ │ │ ├── HoverCard.tsx
│ │ │ │ │ ├── Icons.tsx
│ │ │ │ │ ├── Input.tsx
│ │ │ │ │ ├── Navbar.tsx
│ │ │ │ │ ├── Popover.tsx
│ │ │ │ │ ├── Resizable.tsx
│ │ │ │ │ ├── ScrollArea.tsx
│ │ │ │ │ ├── Select.tsx
│ │ │ │ │ ├── Slider.tsx
│ │ │ │ │ ├── Stepper.tsx
│ │ │ │ │ ├── Tabs.tsx
│ │ │ │ │ ├── Textarea.tsx
│ │ │ │ │ ├── Toggle.tsx
│ │ │ │ │ ├── ToggleGroup.tsx
│ │ │ │ │ ├── Tooltip.tsx
│ │ │ │ │ └── Tree.tsx
│ │ ├── lib
│ │ │ └── utils.ts
│ │ ├── semio.ts # all domain logic
│ │ ├── store.ts # react hooks for state (uses yjs)
│ │ └── package.json
│ ├── docs
│ ├── play
│ ├── playground
│ └── sketchpad
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
├── CLAUDE.md
├── nx.json # Nx targets and plugin configs
├── package-lock.json # All javascript dependencies
├── package.json # Monorepo and workspace setup
├── powershell.ps1 # General Powershell utility
└── README.md # GFM dev docs

# Ecosystems

## js

Javascript code with shared core (@semio/js) that uses storybook and exports a handful of React components (Sketchpad, Diagram, Model) for both web-based and desktop-based environments, a documentation (@semio/docs) that uses astro with starlight and mdx, and sketchpad (@semio/sketchpad) that runs in electron.

### Guidelines

- No relative imports. Use @semio/PROJECT as base instead of `./`.
- No inline styling. Use tailwindcss (v4). v4 uses a `theme.css` (`@semio/js/theme.css`) for theming and not `{theme:{…}}` in `tailwindconfig`.
- Everything light and darkmode compatible.
- Use only colors defined in `@theme inline {…}` from `@semio/js/globals.css`.
- No rounded borders (unless full rounded).
- No shadows.

### Styling

- The ui consists of a navbar and edgeless content. Everything else is displayed as HUD with floating panels that show different colors.

# Packages

## @semio/js

Shared react components. The main component is Sketchpad. Sketchpad is used in three different szenarios:

1. As guest mode (readonly) in a statically generated pages.
2. As user mode in the browser (nextjs).
3. As user mode in a desktop app (electron).
   Sketchpad has a local store in yjs which syncs with indexeddb and the backend provider.

# Guidelines

- All domain logic is in semio.ts and whenever an operation is not ui bound, it should be implemented there.
- All state is stored in the SketchpadStore. All state and cruds are accessed over hooks.
- There are different scopes: SketchpadScope, KitScope, DesignScope, DesignEditorScope.
- There is a transaction mechanism for kits. Every design editor transaction is an extended kit transaction. The undo redo manager is on editor level and stores the diff of the transaction along with the editor state. This way undo redo works even when the kit changes because only the diff is stored.
