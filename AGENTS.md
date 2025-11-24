This document MUST ALWAYS BE followed unless explicitly asked to do otherwise.

IMPORTANT: The codebase in under design and development and not used in production yet. ALWAYS use clean mechanisms that might require large refactorings and NEVER care about backwards compatibility.

# Specs

## Kit

A `kit` is a collection of `types`, `designs`, `authors`, `qualities`, `attributes`, and `concepts`.

A `kit` is either _static_ (a special `.zip` file) or _dynamic_ (bound to a runtime).

A _static_ `kit` contains a reserved `.semio` folder that contains a `kit.db` sqlite file.

The SQL-schema of `kit.db` is found in `./sqlite/schema.sql`.

For Inter-Process-Communication (IPC) the JSON-schema in `./jsonschema/kit.json` is used.

## Design

A `design` is an undirected graph of `pieces` (nodes) and `connections` (edges) with organizational `layers`, `groups`, `stats`, `attributes`, and `concepts`.

A `design` is _proto_ (a _protodesign_) when it has no _parent_ `design`.

The _children_ of a _parent_ `design` are _subdesigns_.

A _flat_ `design` has no `connections` and all `pieces` are _fixed_.

The `pieces` are _placed_ _hierarchically_ (breadth-first) for every _component_.

Additional `connections` which where not used in the _placement_ can be used to validate the computed `planes`.

## Type

A `type` is a reusable component with different `models`, `ports`, `attributes`, `concepts`, and `authors`.

The `type` is _proto_ (a _prototype_) when it has no _parent_.

The _childen_ of a _parent_ `type` are _subtypes_.

A `type` can be **virtual** (intermediate type requiring other virtual types to form a physical type), **scalable**, and **mirrorable** with **stock** quantity, **unit**, and optional **location**.

## Connection

A `connection` is a 3D-Link between two `pieces` with the _translation_ parameters **gap** (offset in y-direction), **shift** (offset in x-direction) and **rise** (offset in z-direction), and the _rotation_ parameters **rotation** (rotation around y-axis), **turn** (rotation around z-axis) and **tilt** (rotation around x-axis).

The _translation_ is applied first, then the _rotation_.

The two `pieces` are called **_connected_** and **_connecting_** but there is no difference between them.

The _direction_ of a `connection` goes from the lower _hierarchy_ to the higher _hierarchy_ of the `pieces`.

A `connection` can have `attributes` and diagram positioning with **u** and **v** offsets.

## Piece

A `piece` is an instance of either a `type` or a `design` with **id**, optional **name**, optional **description**, optional **plane**, **center** position, **scale**, optional **mirror plane**, **hidden** and **locked** states, **color**, and `attributes`.

A `piece` is either _fixed_ (with a `plane`) or _linked_ (with a `connection`).

A group of _connected_ `pieces` is called a _component_.

The _hierarchy_ of a `piece` is the length of the shortest path to the next _fixed_ `piece`.

## Port

A `port` is a conceptual connection **point** with an outwards **direction**, **id**, optional **name**, optional **description**, and **t** value for diagram ring positioning.

A `port` can be marked as **mandatory** in which case it is required to be connected to a `piece`.

A `port` can reference an **interface** (InterfaceId) for explicit compatibility control. The interface defines which other interfaces it is compatible with.

No **interface** means the _default_ interface which is compatible with all other ports.

Port compatibility is determined by the `interface` definitions at the kit level.

A `port` can have `props` that define measurable characteristics and `attributes` for additional metadata.

## Model

A `model` is an optional **name**, `tagged` **url** to a resource with an optional **description**.

No **`tags`** means the _default_ model.

The similarity of `models` is determined by the jaccard index of their **`tags`**.

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

## Interface

An `interface` is a port compatibility definition with **name**, optional **description**, optional **icon**, optional list of **compatible interfaces** (InterfaceId references), and `attributes`.

The `interface` is defined at the kit level and referenced by `ports` via InterfaceId.

An empty **compatible interfaces** list means the interface is compatible with all other interfaces.

Two ports are compatible if:

- Both have no interface specified (default compatibility)
- They reference the same interface
- One interface's compatible list includes the other interface's guid
- Either interface has an empty compatible list and the other explicitly allows it

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

# Validation

## Overview

Semio includes a **domain-pure validation system** built entirely in `semio.ts` with **zero JSON dependencies**. All validation logic works with `Kit` objects and produces `KitDiff`-based fixes.

## Architecture

### Layer 1: Domain Logic (`semio.ts`)

- **100% JSON-agnostic** - No JSON paths, parsing, or serialization logic
- **Pure functions** - All validation is deterministic and side-effect free
- **Diff-based fixes** - Every fix is a `KitDiff` that can be applied, inverted, and merged
- **Reusable everywhere** - Works in Sketchpad UI, CLI, backend, VS Code, and any other platform

### Layer 2: Platform Integrations

Each platform provides its own thin wrapper:

- **VS Code Extension** (`js/vscode`) - JSON linter with Quick Fixes
- **Sketchpad UI** - In-app validation panel
- **CLI** - Command-line validation tool
- **Backend** - API validation endpoint

## Validation Types

### Core Types

```typescript
type SemioEntityKind = "Kit" | "Type" | "Design" | "Piece" | "Connection" | "Port" | "Attribute" | "File" | "Folder" | "Quality" | "Interface" | "Prop" | "Model" | "Layer" | "Group" | "Stat";
type SemioValidationSeverity = "error" | "warning";

interface SemioDomainLocation {
  entityKind: SemioEntityKind;
  entityGuid?: Guid;
  field?: string;
}

interface SemioKitFix {
  title: string;
  diff: KitDiff;
}

interface SemioValidationIssue {
  ruleId: string;
  severity: SemioValidationSeverity;
  message: string;
  location: SemioDomainLocation;
  relatedGuids?: Guid[];
  fixes: SemioKitFix[];
}

interface SemioValidationResult {
  issues: SemioValidationIssue[];
}
```

### Validation Context

```typescript
interface SemioValidationContext {
  kit: Kit;
  typesByGuid: Map<Guid, Type>;
  designsByGuid: Map<Guid, Design>;
  piecesByGuid: Map<Guid, { designGuid: Guid; piece: Piece }>;
  portsByTypeGuid: Map<Guid, Port[]>;
  modelsByTypeGuid: Map<Guid, Model[]>;
}
```

## Validation Rules

All validation rules follow the pattern:

```typescript
type SemioValidationRule = (ctx: SemioValidationContext) => SemioValidationIssue[];
```

### Default Rules

#### 1. GUID Uniqueness (`guid-unique`)

**Severity:** Error

All GUIDs must be unique across the entire kit, including:

- Kit
- Types
- Designs
- Pieces
- Connections
- Stats
- Qualities
- Interfaces
- Files
- Folders

**Fix:** Regenerates a new GUID and updates all references throughout the kit.

#### 2. Type Name Uniqueness (`type-name-unique`)

**Severity:** Error

Types with the same parent must have unique names.

**Fix:** Renames the type with a unique suffix (e.g., "Wall 2", "Wall 3").

#### 3. Design Name Uniqueness (`design-name-unique`)

**Severity:** Error

Designs with the same parent must have unique names.

**Fix:** Renames the design with a unique suffix.

#### 4. Piece Name Uniqueness (`piece-name-unique`)

**Severity:** Error

Pieces within a design must have unique names.

**Fix:** Renames the piece with a unique suffix.

#### 5. Quality Name Uniqueness (`quality-name-unique`)

**Severity:** Error

All qualities within a kit must have unique names.

**Fix:** Renames the quality with a unique suffix.

#### 6. Interface Name Uniqueness (`interface-name-unique`)

**Severity:** Error

All interfaces within a kit must have unique names.

**Fix:** Renames the interface with a unique suffix.

#### 7. File Name Uniqueness (`file-name-unique`)

**Severity:** Error

All files within a kit must have unique names.

**Fix:** Renames the file with a unique suffix.

#### 8. Folder Name Uniqueness (`folder-name-unique`)

**Severity:** Error

Folders with the same parent must have unique names.

**Fix:** Renames the folder with a unique suffix.

#### 9. Port Name Uniqueness (`port-name-unique`)

**Severity:** Error

Ports within a type must have unique names.

**Fix:** Renames the port with a unique suffix.

#### 10. Model Name Uniqueness (`model-name-unique`)

**Severity:** Error

Models within a type must have unique names.

**Fix:** Renames the model with a unique suffix.

#### 11. Layer Path Uniqueness (`layer-path-unique`)

**Severity:** Error

Layer paths within a design must be unique.

**Fix:** Renames the layer path with a unique suffix.

## Uniqueness Requirements Summary

| Entity     | Scope                  | Field | Rule ID               |
| ---------- | ---------------------- | ----- | --------------------- |
| Kit        | Global                 | guid  | guid-unique           |
| Type       | Siblings (same parent) | name  | type-name-unique      |
| Type       | Global                 | guid  | guid-unique           |
| Design     | Siblings (same parent) | name  | design-name-unique    |
| Design     | Global                 | guid  | guid-unique           |
| Piece      | Within design          | name  | piece-name-unique     |
| Piece      | Global                 | guid  | guid-unique           |
| Connection | Global                 | guid  | guid-unique           |
| Port       | Within type            | name  | port-name-unique      |
| Model      | Within type            | name  | model-name-unique     |
| Quality    | Global                 | name  | quality-name-unique   |
| Quality    | Global                 | guid  | guid-unique           |
| Interface  | Global                 | name  | interface-name-unique |
| Interface  | Global                 | guid  | guid-unique           |
| File       | Global                 | name  | file-name-unique      |
| File       | Global                 | guid  | guid-unique           |
| Folder     | Siblings (same parent) | name  | folder-name-unique    |
| Folder     | Global                 | guid  | guid-unique           |
| Layer      | Within design          | path  | layer-path-unique     |
| Stat       | Global                 | guid  | guid-unique           |

## Usage

### In Domain Code

```typescript
const result = validateSemioKit(kit);
if (hasSemioErrors(result)) {
  console.error("Validation errors found:", result.issues);
}
```

### Applying Fixes

```typescript
const issue = result.issues[0];
const fix = issue.fixes[0];
const fixedKit = applyKitDiff(kit, fix.diff);
```

### Custom Validation

```typescript
const customRule: SemioValidationRule = (ctx) => {
  const issues: SemioValidationIssue[] = [];
  // Custom validation logic
  return issues;
};

const result = validateSemioKit(kit, {
  rules: [...defaultSemioValidationRules, customRule],
});
```

## Creating New Rules

1. Define the rule function following `SemioValidationRule` signature
2. Use `semioMakeFix` helper to generate `KitDiff`-based fixes
3. Add to `defaultSemioValidationRules` array
4. Document in this section

Example:

```typescript
export const semioCustomRule: SemioValidationRule = (ctx) => {
  const issues: SemioValidationIssue[] = [];
  // Validation logic
  // Use semioMakeFix to create fixes
  return issues;
};
```

# Monorepo

## Rules

### General

- For every task you are working on, ALWAYS create or update a markdown document to plan and log changes under `log/DATE_SLUG.md` (replace DATE with YEAR-MONTH-DAY and SLUG with a unique slug for the task). E.g. `log/2025-11-24-PIECE-DRAG-AND-DROP-ISSUE.md`.
- ALWAYS document mechanisms technicallly in `AGENTS.md` and in `README.md`. Those documents NEVER keep a log and ALWAYS show the current state of the codebase.
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
- NEVER create new files. ALWAYS add code to existing files using regions and subregions for structuring. Regions organize code into collapsible sections (e.g., `#region RegionName` / `#endregion` in C#, or `//#region RegionName` / `//#endregion` in JavaScript/TypeScript). Use subregions within regions for hierarchical organization. This keeps related code together and maintains a single source of truth per logical unit.
- NEVER create new folders unless for temporary purposes.
- NEVER create additional example files and implement it directly in the dependent parts.
- NEVER remove code that is commented out.
- NEVER add comments to the code. Especially not to communicate to the user.
- NEVER ask to run a command where you are not using the output. All dev servers, debugging and testing processes are running.
- NEVER run modifying `git` commands such as (`git checkout`, `git branch`, …). Only read-only `git` commands are allowed. If you messed up, ALWAYS fix the file.
- NEVER create tests unless you are explicitly asked to.
- ALWAYS use inline syntax if possible.
- NEVER add two statements into the same line.
- ALWAYS inline code.
- NEVER create a variable, function, … class, that is only used once and inline it.
- NEVER add extra new blank lines/newlines inside of code.
- NEVER add raw text to ui elements. ALWAYS use i18n setups and provide translations for the existing languages.
- ALWAYS add `[DEBUG] ` prefix to temporary logs so that they can be easily removed later.
- NEVER build or run the code.
- NEVER care about backwards compatibility unless explicitly asked to. Even on schema changes ALWAYS refactor to clean code and introduce breaking changes.
- NEVER use `type` for naming enums, interfaces, or types. ALWAYS use `kind` instead to avoid confusion with the native `type` concept in Semio. Examples: `ArtifactType` → `ArtifactKind`, `WindowType` → `WindowKind`, etc.
- When fixing issues, ALWAYS update the existing file and NEVER create new fixed, updated, migrated, etc. files next to the old one.
- NEVER skip any test or simplify/remove functionality to pass or fix an issue. ALWAYS adjust implementation to pass the tests.
- NEVER create additional scripts, tests, fixtures, assets, …
- NEVER create scripts outside the `scripts` folder. Not even when debugging or diagnosing a library problem.
- ALWAYS create temporary scripts, tests, fixtures, assets, … in the `temp` folder.
- ALWAYS run specific tests and NEVER use default interactive test mode that creates a never ending process.

### Keywords

- `DIAGNOSE`: Think about the problem and possible causes. ALWAYS add console logs to the codebase to help understand the problem. NEVER assume to know the solution and ALWAYS use logs to verify your hypothesis. ALWAYS add a `[SLUG] ` (replace SLUG with a unique slug for this diagnosis) after `[DEBUG] ` to the console log in order to identify the logs related to the diagnosis. E.g. `[DEBUG] [PIECE-DRAG-AND-DROP-ISSUE] Mounting Dropzone: …`. Then you will receive the logs from the user and if the logs are enough to verify your hypothesis, ALWAYs directly implement the solution. When the `DIAGNOSE` is not enough, update the document with the new information, add new logs and continue the process.

- `FIX`: Anaylze and fix the problem imediatley in one step (without any approval). When you are not sure about the root cause, pick the most likely one and try to implement the solution directly.

- `CLEAN`: Clean up everything intermediate such as diagnostic console logs, comments, temporary code, …

- `I18N`: Run `scripts/i18n.ps1` to produce a report in `agents/i18n.md`. ALWAYS fix all translation issues from report and rerun the script to produce new reports until all issues are resolved. ALWAYS add all missing keys, update all incomplete keys, remove all unused keys, …

- `AUTOMATE`: Create a script to automate a task. `*.ps1` for non-domain related tasks (use `powershell.ps1` for reusable code). `*.ts` for domain related tasks (use `@semio/js` for reusable code). `*.py` for python related tasks (use `@semio/engine` for reusable code).

### Internationalization (i18n)

All user-facing text must be internationalized using i18next. The system supports English (`en`) and German (`de`) by default.

#### i18n Key Convention

Every UI element with an `id` prop automatically gets i18n keys based on that ID:

- `{id}.label` - Standard label text
- `{id}.beginner` - Beginner-friendly description (optional)
- `{id}.manual` - Manual page path (optional)
- `{id}.tutorial` - Tutorial path (optional)
- `{id}.hotkey` - Hotkey display string (optional)

#### Using i18n in Components

NEVER use `useTranslation` directly or hardcode strings. Instead:

1. Assign an `id` prop to the UI element matching the i18n key path
2. Use `<DescriptionTooltipContent>` or let tooltips automatically resolve content
3. For custom text, use `t(id)` where `id` matches the element's `id` prop

#### Translation Files

Translations live in `js/js/locales/{lang}.json`. Keys follow dot-notation paths matching UI element IDs.

#### Tooltip Integration

The tooltip system automatically resolves i18n content from element IDs, adapting to expertise level (beginner/normal/expert).

### Styling

- NEVER use hardcoded (hex, rgb, …) or standard colors. All theme colors are explicitly defined.
- ALWAYS use colors for light mode. Dark mode is automatically derived. There are scales for the following number of colors: 2 (dark, light), 3 (dark, gray, light), 4 (dark, dark-gray-gray, light-gray-gray, light), 5 (dark, dark-gray, gray, light-gray, light), 6 (dark, dark-gray-gray, gray, light-gray-gray, light), 7 (dark, dark-6-7, dark-5-7, gray, light-5-7, light-6-7, light), 8 (dark, d-d-d-g, dark-gray, d-g-g-g, light-gray, l-g-g-g, l-l-l-g, light), 9 (dark, dark-8-9, dark-7-9, dark-gray, gray, light-gray, light-7-9, light-8-9, light), 10 (gray-100, gray-200, gray-300, gray-400, gray, gray-600, gray-700, gray-800, gray-900, light), 11 (dark, gray-100, dark-gray-gray, dark-gray, gray, light-gray, light-gray-gray, light-light-gray, l-l-l-g, gray-900, light). ALWAYS pick the one with the highest contrast.
- All closed ui elements ALWAYS have a border.
- NEVER use hardcoded pixels. ALWAYS use the standardized unit-based sizing system defined in globals.css (derived from `--spacing`):
  - Single: 1 unit - spacing between elements and between icon and element (e.g. `gap-1`)
  - Tiny: 3 units - height/width of icons within actions, small text size (e.g. `h-3`, `w-3`)
  - Small: 5 units - height/width of actions, avatars, default text size (e.g. `h-5`, `w-5`)
  - Medium: 7 units - height of tree items, height of buttons and simple toggles, height of input (e.g. `h-7`)
  - Large: 9 units - height of navbar, height of table row, height of table header (e.g. `h-9`)
  - Huge: 11 units - height of navigation buttons at bottom of docs pages (e.g. `h-11`)
  - Mega: 13 units - width of toggles with actions (toggles with dropdown or action buttons) (e.g. `w-13`)
  - Giga: 15 units - reserved for future use (e.g. `w-15`)
- NEVER use rounded corners unless a circle.
- NEVER use shadows.
- Whenever a ui element can be interacted (left/right clicked with/without hold or modifier keys, dragged, …) with, ALWAYS make it visible (different hover color, different cursor, tooltip, …).
- The ui ALWAYS consists of three layers: 1. base, 2. panel and 3. temporary. Every layer has a darker background color and is on top of the previous layer. Every ui element ALWAYS has an enum for the layer and hence ALWAYS has three different color sets.
- ALWAYS indicate on the element and the cursor when it is interactive. Clickable elements have a pointer cursor and a hover effect. Dragable elements have a grab cursor. While dragging, the cursor changes to a grabbing cursor.

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
│ │ ├── Reorderer.chatmode.md # Exclusively to reorder text (code, lists, …)
│ │ └── Schema-Changer.chatmode.md # Exclusively to change the schema (code, api, database, …)
│ ├── workflows
│ │ └── gh-pages.yml # Deploy user docs togh-pages
│ └── dependabot.yml
├── .vscode
├── agents # All temporary markdown documents for planning, diagnosing, implementing, … by and for agents.
│ ├── i18n.md # i18n validation report produced by scripts/i18n.ps1
│ └── DATE_SLUG.md # DATE is YEAR-MONTH-DAY and SLUG is a unique slug in CAPS-CASE.
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
│ │ │ │ ├── Scrollable.stories.tsx
│ │ │ │ ├── Scrollable.tsx
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
│ │ │ ├── App.tsx # central barrel (Canvas, Navbar, Footer, store, kits, panels)
│ │ │ ├── apps
│ │ │ │ ├── index.tsx
│ │ │ │ ├── design
│ │ │ │ │ └── App.tsx
│ │ │ │ ├── docs
│ │ │ │ │ ├── App.tsx
│ │ │ │ │ └── pages
│ │ │ │ │ ├── index.mdx
│ │ │ │ │ ├── getting-started
│ │ │ │ │ │ ├── index.mdx
│ │ │ │ │ │ └── installation.mdx
│ │ │ │ │ ├── integrations
│ │ │ │ │ │ └── index.mdx
│ │ │ │ │ ├── manuals
│ │ │ │ │ │ └── kit.mdx
│ │ │ │ │ ├── showcases
│ │ │ │ │ │ └── metabolism.mdx
│ │ │ │ │ └── tutorials
│ │ │ │ │ ├── hello-semio
│ │ │ │ │ │ └── index.mdx
│ │ │ │ │ └── serial-conversion
│ │ │ │ │ └── index.mdx
│ │ │ │ ├── home
│ │ │ │ │ └── App.tsx
│ │ │ │ ├── kit
│ │ │ │ │ └── App.tsx
│ │ │ │ ├── quality
│ │ │ │ │ └── App.tsx
│ │ │ │ └── type
│ │ │ │ └── App.tsx
│ │ │ └── tutorials
│ │ │ ├── commands.ts
│ │ │ ├── exampleTutorial.ts
│ │ │ ├── index.ts
│ │ │ ├── RecordButton.tsx
│ │ │ ├── sketchpadTour.ts
│ │ │ ├── store.tsx
│ │ │ ├── TutorialControls.tsx
│ │ │ ├── TutorialOverlay.tsx
│ │ │ └── types.ts
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
│ │ └── UserObjects
│ │ ├── github
│ │ ├── gitlab
│ │ ├── monoceros
│ │ ├── semio
│ │ └── wasp
│ ├── Semio.Grasshopper
│ │ └── Semio.Grasshopper.cs # @semio/gh: all grasshopper code
│ ├── Semio.Grasshopper.Tests
│ └── Semio.Tests
├── rb
├── rdf
├── scripts
│ └── i18n.ps1
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
- ALWAYS load icons via the semantic icon layer in `@semio/assets` and NEVER import icons directly from external libraries (lucide, heroicons, .). Only reexport placeholder assets from those libraries inside `@semio/assets` and consume them through its semantic exports.

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
- Commands ALWAYS have an origin. ALWAYS add the id of the ui element as origin when calling commands.
- There is a transaction mechanism for kits. Every app transaction is an extended kit transaction. The undo redo manager is on app level and stores the diff of the transaction along with the app state. This way undo redo works even when the kit changes because only the diff is stored. The inverted diff is stored along with the diff to enable relative undo redo.
- NEVER use direct strings or `useTranslation` for displaying text. ALWAYS assign an `id` the ui element and use i18n keys which match the id.
- The code runs in different environments (different browsers, electron, mobile/desktop/tablet). Platform-specific functionality MUST be generalized and provided as props to Sketchpad. NEVER hardcode platform-specific behavior or APIs directly in components.

The former `Canvas`, `Navbar`, `Footer`, `Panel`, and `store` modules now live inside `js/js/sketchpad/App.tsx`. Keep the region order intact when modifying this file so downstream imports continue to work.

#### Architecture - Open-Closed Principle

The codebase follows the Open-Closed Principle (OCP): closed for modification, open for extension. Adding new features ONLY requires adding new files/folders, NEVER modifying existing ones.

##### App Structure Standards

All apps in `js/js/sketchpad/apps/*/App.tsx` MUST follow this structure:

1. **Region Order:** Header → Imports → Types → Store → Commands → Components → App → Config
2. **Store Base Class:** MUST extend either `AppStore` or `KitDiffAppStore` (no custom base classes)
3. **Store Registration:** MUST use inline registration pattern (no wrapper functions)
4. **Component Regions:** MUST nest under Components region (Navbar, Canvas, Panels, Tools, Footer)
5. **Tools:** MUST have Tools region if app has multiple interaction modes
6. **Scope Providers:** MUST be defined in app file (not App.tsx)
7. **Commands:** MUST define all commands in Commands region

See `REFACTOR.md` for detailed rationale and migration guide.

##### Adding a New App

To add a new app:

1. Create a folder under `js/js/sketchpad/apps/{appname}/`.
2. Add a single `App.tsx` that:
   - exports the default React component,
   - declares and exports `config: AppConfig`,
   - wires any local state, commands, or helpers needed by the app.
3. Keep optional helpers (pages, panels, tools) alongside the file and import them from the same module.

The app registry auto-discovers `App.tsx` files via `import.meta.glob('./*/App.tsx')`.

Example section inside `App.tsx`:

```typescript
import { FC } from "react";
import { AppConfig } from "../registry";

const App: FC = () => {
  // ...
};

export const config: AppConfig = {
  id: "myapp",
  component: App,
  routeSegments: [{ path: "my/:id", paramName: "id" }],
  getPanels: (t) => [{ key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" }],
  matchesPath: (pathParts) => pathParts[0] === "my",
  order: 50,
};

export default App;
```

##### Adding a New Tool

To add a new tool to an app:

1. Create a `*Tool.tsx` file directly inside `js/js/sketchpad/apps/{app}/`.
2. Export a `Tool<AppState>` object with a unique `id` and `render` implementation.

Each app loads sibling `*Tool.tsx` modules via `import.meta.glob('./*Tool.tsx', { eager: true })`, so simply dropping the file in place registers it.

Example:

```typescript
export const MyTool: Tool<MyAppState> = {
  id: ToolKind.MY_TOOL,
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

##### Tutorials

The tutorial system lives in `js/js/sketchpad/tutorials` and re-exports everything via `index.ts`. Tutorials are managed by `TutorialStore`, which wraps a Y.js map and keeps playback, milestone ordering, and recording state (`TutorialPlaybackState`, `TutorialRecordingState`). Always create the store with the app transaction handler so tutorial mutations participate in undo/redo.

Wrap consumers in `TutorialProvider` and use the helper hooks (`useTutorialStore`, `useActiveTutorial`, `useTutorialProgress`, `useTutorialCommandInterceptor`, etc.) instead of accessing the store directly. `TutorialControls` and `TutorialOverlay` are the canonical UI integrations for playback, highlighting, and recording.

Tutorial commands live in `commands.ts` under the `semio.tutorial.*` and `semio.recording.*` namespaces; extend the exported `tutorialCommands` object and re-export through `index.ts` when adding behaviors. Bundle reusable walkthroughs or recordings as modules (for example `sketchpadTour.ts`) that return `Tutorial` objects and register them with `addTutorial`.

##### Footer

`FooterItemProvider` wraps `Sketchpad` so apps can register footer entries with `useAddFooterItem` and remove them via `useRemoveFooterItem`; the provider keeps items ordered by the optional `order` field.

Register items inside effects and always call the remove helper in the cleanup; default contributions now live inside each app's `App.tsx`, next to the `config` export.

Providing an `id` shows the translated `DescriptionTooltipContent`, and the base footer auto-hides in fullscreen until the cursor nears the bottom edge, so interactive elements must tolerate that visibility change.

#### Styling

- NEVER use colors and spacing directly. ALWAYS use semantic variables from `global.css`. Only `global.css` uses colors and pixels directly.
- NEVER add semantic values and ALWAYS use hardcoded values in `theme.css`. NEVER use `theme.css` outside of `global.css`.
- ALWAYS use the standardized unit-based sizing system defined in globals.css (derived from `--spacing`):
  - Single: 1 unit - spacing between elements and between icon and element (e.g. `gap-1`)
  - Tiny: 3 units - height/width of icons within actions, small text size (e.g. `h-3`, `w-3`)
  - Small: 5 units - height/width of actions, avatars, default text size (e.g. `h-5`, `w-5`)
  - Medium: 7 units - height of tree items, height of buttons and simple toggles, height of input (e.g. `h-7`)
  - Large: 9 units - height of navbar, height of table row, height of table header (e.g. `h-9`)
  - Huge: 11 units - height of navigation buttons at bottom of docs pages (e.g. `h-11`)
  - Mega: 13 units - width of toggles with actions (toggles with dropdown or action buttons) (e.g. `w-13`)
  - Giga: 15 units - reserved for future use (e.g. `w-15`)

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

##### Hooks and Helpers

- **`useSync` / `useSyncDeep`** (from `js/js/sketchpad/App.tsx`) wrap `useSyncExternalStore` against a store's `onChanged` / `onChangedDeep` events. Pass a selector (defaults to `identitySelector`) to scope renders to the slice you need.
- **`createObserver`** bridges a Y.js map or array into the store by registering either shallow or deep observers; always dispose the returned cleanup in `useEffect` finalizers.
- **`RemoteProviders`** bundles the `yProvider` and `fileProvider` factories needed when constructing `SketchpadStore` so persistence and external file access stay aligned.

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
- **TypeAppStore** - Edit types (ports, models)
- **KitAppStore** - Edit kits (types, designs, qualities, files, authors)

#### Concrete Implementations

##### DesignAppStore

Edits design content:

- Selection: pieces, connections, ports
- Kit diffs: piece changes, connection changes
- Transaction support for complex multi-step operations

##### TypeAppStore

Edits type definitions:

- Selection: ports, models
- Kit diffs: port changes, model changes
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

- `js/js/sketchpad/App.tsx` - Base Store, AppStore, KitDiffAppStore, SketchpadStore, KitStore
- `js/js/sketchpad/apps/design/App.tsx` - DesignAppStore and design app state
- `js/js/sketchpad/apps/type/App.tsx` - TypeAppStore and type toolchain
- `js/js/sketchpad/apps/quality/App.tsx` - QualityAppStore and quality workflows
- `js/js/sketchpad/apps/kit/App.tsx` - KitAppStore and kit command wiring
- `js/js/sketchpad/apps/home/App.tsx` - HomeStore and home experience

### Command System

All state mutations are executed through commands. Commands provide a consistent interface for operations and enable undo/redo, logging, and origin tracking.

#### Command Registry

Each store maintains a `commandRegistry` that maps command strings to handler functions. Commands are registered using `registerCommand` and unregistered using `unregisterCommand`.

#### Command Execution

Commands are executed via `executeCommand(command: string, ...args: any[])`:

1. **Origin Extraction**: If the first argument is a string starting with `semio.sketchpad.`, it's treated as the origin (UI element ID). Otherwise, origin is undefined.
2. **Command Lookup**: The command registry is searched for the handler.
3. **Context Building**: A command context is built with current state snapshot.
4. **Handler Execution**: The handler receives context and remaining arguments.
5. **Diff Application**: Result diffs are applied to the store.
6. **Edit Recording**: For AppStore/KitDiffAppStore, edits are recorded for undo/redo.

#### Command Naming Convention

Commands follow the pattern `semio.{scope}.{action}`:

- `semio.sketchpad.*` - Sketchpad-level commands
- `semio.kitApp.*` - Kit app commands
- `semio.designApp.*` - Design app commands
- `semio.typeApp.*` - Type app commands
- `semio.home.*` - Home app commands

Special commands:

- `semio.{app}.startTransaction` - Start a transaction
- `semio.{app}.finalizeTransaction` - Finalize current transaction
- `semio.{app}.abortTransaction` - Abort current transaction
- `semio.{app}.undo` - Undo last edit
- `semio.{app}.redo` - Redo last undone edit

#### Command Origin

Every command execution should include an origin string identifying the UI element that triggered it. Origins follow the pattern `semio.sketchpad.{path}` matching the element's `id` prop. This enables:

- Debugging and logging
- Tutorial recording
- Analytics tracking

### Diff System

The diff system tracks changes to models for undo/redo, synchronization, and persistence.

#### Diff Types

Every model has an associated `Diff` type that represents partial changes:

- **ModelDiff**: Partial update to a single model instance
- **ModelsDiff**: Collection diffs with `removed`, `updated`, and `added` arrays

#### Diff Operations

Each model type supports four diff operations:

1. **`getDiff(before, after): Diff`** - Calculate diff between two states
2. **`inverseDiff(original, appliedDiff): Diff`** - Calculate inverse diff for undo
3. **`mergeDiff(diff1, diff2): Diff`** - Merge two diffs (later takes precedence)
4. **`applyDiff(base, diff): Model`** - Apply diff to base state

#### Diff Status

Diffs track status using `DiffStatus` enum:

- `Unchanged` - No change
- `Added` - Newly added item
- `Removed` - Deleted item
- `Modified` - Updated item

#### Collection Diffs

Collection diffs (`*sDiff`) track changes to arrays/lists:

```typescript
interface CollectionDiff<T> {
  removed?: TId[]; // IDs of removed items
  updated?: { id: TId; diff: TDiff }[]; // Updated items with their diffs
  added?: T[]; // Newly added items
}
```

#### Inverse Diffs

Inverse diffs enable undo by reversing operations:

- `removed` → `added` (restore removed items)
- `added` → `removed` (remove added items)
- `updated` → inverse of the update diff

### Routing & App Registration

Apps are registered via the `AppRegistry` which auto-discovers apps using `import.meta.glob('./*/App.tsx')`.

#### AppConfig

Each app exports a `config: AppConfig`:

```typescript
interface AppConfig {
  id: string; // Unique app identifier
  component: ComponentType; // React component
  routeSegments: RouteSegment[]; // Route path segments
  getPanels: (t: TFunction) => PanelDefinition[]; // Panel definitions
  matchesPath: (pathParts: string[]) => boolean; // Path matcher
  order?: number; // Display order
}
```

#### Route Segments

Route segments define the app's URL structure:

```typescript
interface RouteSegment {
  path: string; // React Router path pattern
  paramName?: string; // Parameter name (e.g., "id")
  scopeProvider?: ComponentType<{ guid: string; children: ReactNode }>; // Scope wrapper
}
```

#### Path Matching

Apps can match paths using `matchesPath(pathParts: string[])`. The registry searches apps in order and returns the first match.

#### Scope Providers

Scope providers wrap app components to provide context (e.g., kit/design/type GUIDs) via React Router params.

### Hotkeys

Hotkeys are configurable keyboard shortcuts stored in the SketchpadStore with user overrides.

#### Hotkey Paths

Hotkey paths follow the pattern `semio.sketchpad.{element.path}` matching UI element IDs. This enables:

- Automatic tooltip display
- Settings UI integration
- Tutorial highlighting

#### Hotkey Values

Hotkeys use the format from `react-hotkeys-hook`:

- `mod+k` - Meta/Ctrl + K
- `shift+alt+d` - Shift + Alt + D
- `escape` - Escape key

#### Hotkey Overrides

Users can override default hotkeys via `hotkeyOverrides` in SketchpadStore. Overrides take precedence over defaults.

#### Hotkey Hooks

- `useHotkey(path, callback, deps)` - Register hotkey handler
- `useSetHotkey()` - Set hotkey override
- `useResetHotkey()` - Reset hotkey to default
- `useResetAllHotkeys()` - Reset all overrides

### File Providers

File providers abstract file storage for kits, supporting multiple backends.

#### FileProvider Interface

```typescript
interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}
```

#### Provider Types

1. **MemoryFileProvider**: In-memory storage using Map (temporary kits)
2. **LocalFileProvider**: IndexedDB storage (browser persistence)
3. **RemoteFileProvider**: HTTP-based storage (server backend)
4. **CompositeFileProvider**: Combines multiple providers with fallback order

#### File Operations

File operations are handled automatically when kit diffs include file changes:

- **Added files**: Uploaded via provider, `remoteUrl` updated in kit
- **Removed files**: Deleted via provider
- **Updated files**: Re-uploaded if blob changed

### Y.js Integration

Y.js provides CRDT-based state synchronization and persistence.

#### Y.js Types

Stores use Y.js types for reactive state:

- `Y.Map` - Key-value maps (state objects)
- `Y.Array` - Arrays (lists, selections)
- `Y.Text` - Text (rarely used)

#### Persistence

- **IndexeddbPersistence**: Local browser persistence for kits
- **YProvider**: Remote synchronization (WebSocket, HTTP)

#### Observers

Y.js observers bridge Y.js changes to store updates:

- **Shallow observers**: Watch top-level map keys
- **Deep observers**: Watch nested changes

Use `createObserver` helper and dispose in `useEffect` cleanup.

#### Transactions

Y.js transactions batch operations:

- All Y.js mutations happen within transactions
- Store `transact` function wraps Y.js transactions
- Origin strings propagate to Y.js for debugging

### Coordinate System

Semio uses a left-handed coordinate system that differs from Three.js.

#### Semio Coordinate System

- **X-axis**: Right (thumb points right)
- **Y-axis**: Forward (index finger forward)
- **Z-axis**: Up (middle finger up)

#### Three.js Coordinate System

- **X-axis**: Right
- **Y-axis**: Up
- **Z-axis**: Backward (negative)

#### Conversion Functions

- `toThreeRotation()` - Matrix4 for Semio → Three.js rotation
- `toSemioRotation()` - Matrix4 for Three.js → Semio rotation
- `toThreeQuaternion()` - Quaternion for Semio → Three.js
- `toSemioQuaternion()` - Quaternion for Three.js → Semio
- `vectorToThree(v)` - Convert Point/Vector to THREE.Vector3

### Expertise & Tooltips

The UI adapts to user expertise level, showing different tooltip content.

#### Expertise Levels

```typescript
enum Expertise {
  BEGINNER = "beginner", // Full tooltips with tutorials
  NORMAL = "normal", // Standard tooltips
  EXPERT = "expert", // No tooltips
}
```

#### Tooltip Content

Tooltips automatically adapt based on expertise:

- **BEGINNER**: Shows `.beginner` i18n key, tutorials, manuals, hotkeys
- **NORMAL**: Shows standard `.label` i18n key, manuals, hotkeys
- **EXPERT**: No tooltips shown

#### i18n Keys for Tooltips

Each UI element with an `id` prop automatically gets tooltip content from i18n:

- `{id}.label` - Standard label
- `{id}.beginner` - Beginner-friendly description
- `{id}.manual` - Manual page path
- `{id}.tutorial` - Tutorial path
- `{id}.hotkey` - Hotkey display string

#### Tooltip Components

- `<Tooltip>` - Base tooltip wrapper
- `<DescriptionTooltipContent>` - Automatic content from element ID
- `<EnhancedTooltipContent>` - Manual configuration

### Windows

Windows are the primary content areas within the canvas, supporting multiple types.

#### Window Types

```typescript
enum WindowType {
  TABLE = "table", // Tabular data view
  SCENE = "scene", // 3D scene view
  DIAGRAM = "diagram", // 2D diagram view
  CUSTOM = "custom", // Custom app-defined view
}
```

#### Window Configuration

Windows are configured per app via `AppWindowConfig`:

```typescript
interface AppWindowConfig {
  type: WindowType;
  component?: ComponentType<AppWindowProps>;
  defaultVisible?: boolean;
}
```

#### Window Layout

Window layouts are managed per app and stored in app state. Apps can define custom layouts or use defaults.

#### Window Events

Windows can emit events via `onWindowEvents` callback:

- Window creation/destruction
- Window focus changes
- Window resize
- Custom app events

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
14. Interface
15. Prop
16. Model
17. Port
18. Type
19. Layer
20. Piece
21. Group
22. Side
23. Connection
24. Stat
25. Design
26. Kit

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

1. U
2. V

### Vec

1. U
2. V

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

### Interface

1. Name
2. Description
3. Icon
4. CompatibleInterfaces
5. Attributes

### Prop

1. Key
2. Value
3. Unit
4. Attributes

### Model

1. Name
2. Tags
3. Url
4. Description
5. Attributes

### Port

1. Id
2. Name
3. Point
4. Direction
5. T
6. Mandatory
7. Interface
8. Description
9. Attributes

### Type

1. Name
2. Variant
3. Models
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
2. Name
3. Type
4. Design
5. Plane
6. Center
7. Scale
8. MirrorPlane
9. Props
10. IsHidden
11. IsLocked
12. Color
13. Description
14. Attributes

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
