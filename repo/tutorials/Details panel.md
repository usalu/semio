# Details Panel — Element Terminologies and Control Parameters

## Architecture Overview

The Details panel is the **right-side property inspector** in `semio/sketchpad`. It displays metadata and editable controls for selected artifacts. Content is driven by `PanelSection` records added dynamically to `PanelSections.details[]` based on app context and selection state.

| Concept                      | Definition                                                                                                                                                        |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PanelSection`               | A collapsible section: `{ id, content, specificity, order, defaultOpen, actions }`. Higher `specificity` wins when sections overlap; lower `order` renders first. |
| `PanelVisibility`            | Boolean map toggling panel visibility. Details: `Ctrl+L`. Default: `{ details: true, rightSidePanel: true }`.                                                     |
| `PanelKind.DETAILS`          | Enum value `"details"`, position `RIGHT`, group `"right"`, icon `DetailsIcon`.                                                                                    |
| `addSection / removeSection` | Imperative helpers that mutate `PanelSections.details[]` during render-effect hooks.                                                                              |

## Tree Primitives Overview

The Details panel has **two layers**:

1. `PanelSection` decides **which high-level sections** exist in the right panel for the current app and selection.
2. `TreeRow` and `TreeItem` decide **how content inside each section is structured**.

> There is **no `TreeSection` component** in the current codebase. If you are looking for “section” at the panel level, that concept is `PanelSection`. If you are looking for grouped content inside one panel section, that concept is usually `TreeItem`.

| Primitive       | What It Does In The Details Panel | Typical Usage |
| ---------------- | --------------------------------- | ------------- |
| `PanelSection`   | Registers one top-level block in the right panel. | App-level section switching such as design properties, selected piece details, or kit metadata. |
| `TreeRow`        | Renders one leaf row. Usually contains one control or one summary line. | `Input`, `Textarea`, `Toggle`, `Slider`, `Stepper`, or plain text status lines. |
| `TreeItem`       | Renders one grouped or nested tree node, optionally collapsible and optionally with actions. | Location groups, authors lists, attributes lists, piece plane blocks, connection endpoint groups, connector groups. |
| `SortableTreeItems` | Renders reorderable collections of nested `TreeItem` children. | Design authors, design attributes, type models, type connectors. |

### Per-App Tree Usage

| App | Top-Level Registration | Internal Tree Structure |
| --- | --- | --- |
| `Home` | Adds one `PanelSection` for selected kits. | Uses `TreeRow` only. |
| `Kit` | Adds artifact-specific `PanelSection`s plus always-on kit metadata. | Uses `TreeRow` only. |
| `Design` | Adds selection-sensitive `PanelSection`s for pieces, connections, connectors, design, and kit. | Uses both `TreeRow` and nested `TreeItem` heavily. |
| `Type` | Adds connector override sections plus one composite `type.properties` section. | Uses both `TreeRow` and nested `TreeItem` heavily. |
| `Quality` | Adds one quality detail `PanelSection`. | Uses `TreeRow` only. |
| `Docs` | Adds one page-outline `PanelSection`. | Does **not** use `TreeRow` or `TreeItem` in the details panel content. |

---

## Apps and Their Detail Sections

There are **six app contexts** that contribute detail sections. Each app manages its own selection model and registers/removes sections on selection change.

### 1. Home App (`Home.tsx`)

Shown when browsing the kit list (no kit or design open).

| Section ID                                      | Trigger                | Content Component     | Mode      |
| ----------------------------------------------- | ---------------------- | --------------------- | --------- |
| `semio.sketchpad.app.home.panel.details.kit.*`  | Single kit selected    | `SingleKitSection`    | read-only |
| `semio.sketchpad.app.home.panel.details.kits.*` | Multiple kits selected | `MultipleKitsSection` | read-only |

**Fields (SingleKitSection):**

| Control ID           | Label       | Widget     | Editable        | Data Source              |
| -------------------- | ----------- | ---------- | --------------- | ------------------------ |
| `...kit.name`        | name        | `Input`    | no (`readOnly`) | `kitShallow.name`        |
| `...kit.version`     | version     | `Input`    | no              | `kitShallow.version`     |
| `...kit.description` | description | `Textarea` | no              | `kitShallow.description` |
| `...kit.icon`        | icon        | `Input`    | no              | `kitShallow.icon`        |
| `...kit.image`       | image       | `Input`    | no              | `kitShallow.image`       |

**MultipleKitsSection** uses `getCommonValue()`: shows shared value or `"(mixed values)"` placeholder.

---

### 2. Kit App (`Kit.tsx`)

Shown when a kit is open but no design is open. Selection covers: types, designs, ports, tags, concepts, files, folders, authors.

**Section Registration Logic:**

- `removeSection` clears all previous detail sections.
- If `totalSelectedKinds > 1` → shows `MultipleArtifactsSection` (summary of counts per kind).
- If exactly 1 kind is selected → shows the matching single/multiple section.
- `KitSection` (kit metadata) is **always** added at `specificity: 10, order: 100` (lowest priority, last position).

#### 2.1 Kit Section (always visible)

| Section ID                           | Specificity | Order |
| ------------------------------------ | ----------- | ----- |
| `semio.sketchpad.app.kit.properties` | 10          | 100   |

| Control ID           | Label       | Widget          | Editable | Data Source       | Change Handler                          |
| -------------------- | ----------- | --------------- | -------- | ----------------- | --------------------------------------- |
| `...kit.name`        | name        | `Input lazy`    | **yes**  | `kit.name`        | `kitDataSource.change({ name })`        |
| `...kit.version`     | version     | `Input lazy`    | **yes**  | `kit.version`     | `kitDataSource.change({ version })`     |
| `...kit.description` | description | `Textarea lazy` | **yes**  | `kit.description` | `kitDataSource.change({ description })` |
| `...kit.icon`        | icon        | `Input lazy`    | **yes**  | `kit.icon`        | `kitDataSource.change({ icon })`        |
| `...kit.image`       | image       | `Input lazy`    | **yes**  | `kit.image`       | `kitDataSource.change({ image })`       |
| `...kit.homepage`    | homepage    | `Input lazy`    | **yes**  | `kit.homepage`    | `kitDataSource.change({ homepage })`    |
| `...kit.license`     | license     | `Input lazy`    | **yes**  | `kit.license`     | `kitDataSource.change({ license })`     |

> **"lazy"** means `onLazyChange` — commits on blur/enter, not per keystroke.

#### 2.2 Type Section

| Section ID (single)                   | Section ID (multi)                            | Specificity | Order |
| ------------------------------------- | --------------------------------------------- | ----------- | ----- |
| `semio.sketchpad.app.type.properties` | `semio.sketchpad.app.kit.types.multipleTitle` | 30          | 20    |

**Single Type (`SingleTypeSection`):**

| Control ID            | Label       | Widget     | Editable        | Data Source               |
| --------------------- | ----------- | ---------- | --------------- | ------------------------- |
| `...type.name`        | name        | `Input`    | no              | `type.name`               |
| `...type.description` | description | `Textarea` | no              | `type.description`        |
| `...type.icon`        | icon        | `Input`    | no              | `type.icon`               |
| `...type.image`       | image       | `Input`    | no              | `type.image`              |
| `...type.parent`      | parent      | `Input`    | no              | `type.parent?.guid`       |
| `...type.abstract`    | abstract    | `Toggle`   | no (`disabled`) | `type.isAbstract`         |
| `...type.unit`        | unit        | `Input`    | no              | `type.unit` (conditional) |

**Multiple Types:** Lists names only.

#### 2.3 Port Section

| Section ID (single)                       | Section ID (multi)                            | Specificity | Order |
| ----------------------------------------- | --------------------------------------------- | ----------- | ----- |
| `semio.sketchpad.app.kit.port.properties` | `semio.sketchpad.app.kit.ports.multipleTitle` | 30          | 25    |

**Single Port (`SinglePortSection`):**

| Control ID            | Label       | Widget     | Editable | Data Source               |
| --------------------- | ----------- | ---------- | -------- | ------------------------- |
| `...port.name`        | name        | `Input`    | no       | `iface.name`              |
| `...port.description` | description | `Textarea` | no       | `iface.description`       |
| `...port.compatible`  | compatible  | `Input`    | no       | count or "all compatible" |

**Multiple Ports:** Lists names only.

#### 2.4 Tag Section

| Section ID (single)                      | Section ID (multi)                           | Specificity | Order |
| ---------------------------------------- | -------------------------------------------- | ----------- | ----- |
| `semio.sketchpad.app.kit.tag.properties` | `semio.sketchpad.app.kit.tags.multipleTitle` | 30          | 26    |

**Single Tag:**

| Control ID           | Label       | Widget     | Editable |
| -------------------- | ----------- | ---------- | -------- |
| `...tag.name`        | name        | `Input`    | no       |
| `...tag.description` | description | `Textarea` | no       |

#### 2.5 Concept Section

| Section ID (single)                          | Section ID (multi)                               | Specificity | Order |
| -------------------------------------------- | ------------------------------------------------ | ----------- | ----- |
| `semio.sketchpad.app.kit.concept.properties` | `semio.sketchpad.app.kit.concepts.multipleTitle` | 30          | 27    |

**Single Concept:**

| Control ID               | Label       | Widget     | Editable |
| ------------------------ | ----------- | ---------- | -------- |
| `...concept.name`        | name        | `Input`    | no       |
| `...concept.description` | description | `Textarea` | no       |

#### 2.6 Design Section (Kit Context)

| Section ID (single)                     | Section ID (multi)                              | Specificity | Order |
| --------------------------------------- | ----------------------------------------------- | ----------- | ----- |
| `semio.sketchpad.app.design.properties` | `semio.sketchpad.app.kit.designs.multipleTitle` | 30          | 10    |

**Single Design (read-only in kit context):**

| Control ID              | Label       | Widget     | Editable        | Data Source                 |
| ----------------------- | ----------- | ---------- | --------------- | --------------------------- |
| `...design.name`        | name        | `Input`    | no              | `design.name`               |
| `...design.description` | description | `Textarea` | no              | `design.description`        |
| `...design.icon`        | icon        | `Input`    | no              | `design.icon`               |
| `...design.image`       | image       | `Input`    | no              | `design.image`              |
| `...design.variant`     | variant     | `Input`    | no              | `design.variant`            |
| `...design.view`        | view        | `Input`    | no              | `design.view`               |
| `...design.unit`        | unit        | `Input`    | no              | `design.unit`               |
| `...location.longitude` | longitude   | `Input`    | no (`disabled`) | `design.location.longitude` |
| `...location.latitude`  | latitude    | `Input`    | no (`disabled`) | `design.location.latitude`  |
| `...design.createdAt`   | created     | `Input`    | no (`disabled`) | `design.createdAt`          |
| `...design.updatedAt`   | updated     | `Input`    | no (`disabled`) | `design.updatedAt`          |

#### 2.7 File Section

| Section ID (single)                       | Section ID (multi)                            | Specificity | Order |
| ----------------------------------------- | --------------------------------------------- | ----------- | ----- |
| `semio.sketchpad.app.kit.file.properties` | `semio.sketchpad.app.kit.files.multipleTitle` | 30          | 30    |

**Fields:** `name`, `size` (formatted KB), `created`, `updated`.

#### 2.8 Folder Section

| Section ID (single)                         | Section ID (multi)                              | Specificity | Order |
| ------------------------------------------- | ----------------------------------------------- | ----------- | ----- |
| `semio.sketchpad.app.kit.folder.properties` | `semio.sketchpad.app.kit.folders.multipleTitle` | 30          | 40    |

| Control ID              | Label       | Widget          | Editable              | Change Handler                             |
| ----------------------- | ----------- | --------------- | --------------------- | ------------------------------------------ |
| `...folder.name`        | name        | `Input lazy`    | **yes**               | `folderDataSource.change({ name })`        |
| `...folder.description` | description | `Textarea lazy` | **yes** (conditional) | `folderDataSource.change({ description })` |
| created                 | created     | label + `<p>`   | no                    | —                                          |
| updated                 | updated     | label + `<p>`   | no                    | —                                          |

#### 2.9 Multiple Artifacts Section

| Section ID                                   | Specificity | Order |
| -------------------------------------------- | ----------- | ----- |
| `semio.sketchpad.app.kit.artifacts.multiple` | 30          | 0     |

Shows a comma-separated summary: `"3 types, 2 designs, 1 port"`. Displayed only when `totalSelectedKinds > 1`.

---

### 3. Design App (`Design.tsx`)

Shown when a specific design is open. Selection covers: pieces, connections, connectors.

**Section Registration Logic:**

- No selection → `DesignSection` only.
- Connector selected → `ConnectorSection` (order 0) + `DesignSection` (order 50).
- Pieces selected → `PiecesSection` (order 0).
- Connections selected → `ConnectionsSection` (order 10).
- Both pieces and connections → both + mixed-selection warning (order 20).
- `DesignSection` is **always** added at `specificity: 20, order: 50`.
- `KitSection` is **always** added at `specificity: 10, order: 100`.

#### 3.1 Design Section (editable)

| Section ID                              | Specificity | Order |
| --------------------------------------- | ----------- | ----- |
| `semio.sketchpad.app.design.properties` | 20          | 50    |

| Control ID                  | Label       | Widget              | Editable        | Change Handler                                           |
| --------------------------- | ----------- | ------------------- | --------------- | -------------------------------------------------------- |
| `...design.name`            | name        | `Input lazy`        | **yes**         | `updateDesignField({ name })`                            |
| `...design.description`     | description | `Textarea lazy`     | **yes**         | `updateDesignField({ description })`                     |
| `...design.icon`            | icon        | `Input lazy`        | **yes**         | `updateDesignField({ icon })`                            |
| `...design.image`           | image       | `Input lazy`        | **yes**         | `updateDesignField({ image })`                           |
| `...design.variant`         | variant     | `Input lazy`        | **yes**         | `updateDesignField({ variant })`                         |
| `...design.view`            | view        | `Input lazy`        | **yes**         | `updateDesignField({ view })`                            |
| `...design.unit`            | unit        | `Input lazy`        | **yes**         | `updateDesignField({ unit })`                            |
| `...location.longitude`     | longitude   | `Stepper`           | **yes**         | `updateDesignField({ location: { … longitude } })`       |
| `...location.latitude`      | latitude    | `Stepper`           | **yes**         | `updateDesignField({ location: { … latitude } })`        |
| location add/remove         | —           | `TreeItem` action   | **yes**         | `addLocation()` / `removeLocation()`                     |
| authors list                | authors     | `SortableTreeItems` | **yes**         | drag-reorder, add/remove, edit name/email                |
| attributes list             | attributes  | `SortableTreeItems` | **yes**         | drag-reorder, add/remove, edit key/value/unit/definition |
| `...design.createdAt`       | created     | `Input`             | no (`disabled`) | —                                                        |
| `...design.updatedAt`       | updated     | `Input`             | no (`disabled`) | —                                                        |
| `...design.pieceCount`      | pieces      | `Input`             | no (`disabled`) | —                                                        |
| `...design.connectionCount` | connections | `Input`             | no (`disabled`) | —                                                        |

#### 3.2 Pieces Section

| Section ID (single)   | Section ID (multi)       |
| --------------------- | ------------------------ |
| `...piece.properties` | `...piece.multipleTitle` |

**Handles two piece kinds: normal piece vs. design piece (nested design).**

##### Normal Piece Fields

| Control ID             | Label       | Widget              | Editable              | Change Handler                                               |
| ---------------------- | ----------- | ------------------- | --------------------- | ------------------------------------------------------------ |
| `...piece.type`        | type        | `Combobox`          | **yes**               | `handleTypeNameChange()` → `updatePiece({ type: { guid } })` |
| `...type.variant`      | variant     | `Combobox`          | **yes** (conditional) | `handleTypeVariantChange()`                                  |
| `...piece.id`          | id          | `Input`             | no (`disabled`)       | —                                                            |
| `...piece.name`        | name        | `Input lazy`        | **yes**               | `handleNameChange()` → `updatePiece({ name })`               |
| `...piece.description` | description | `Textarea lazy`     | **yes**               | `handleDescriptionChange()`                                  |
| `...piece.attributes`  | attributes  | `SortableTreeItems` | **yes**               | add/remove/edit key/value/unit/definition                    |
| `...piece.scale`       | scale       | `Stepper`           | **yes**               | `handleScaleChange()` → `updatePiece({ scale })`             |
| `...piece.color`       | color       | `Input lazy`        | **yes**               | `handleColorChange()` → `updatePiece({ color })`             |
| `...piece.center.x`    | center x    | `Stepper`           | **yes**               | `handleCenterXChange()`                                      |
| `...piece.center.y`    | center y    | `Stepper`           | **yes**               | `handleCenterYChange()`                                      |
| plane origin x/y/z     | origin      | `Stepper` ×3        | **yes**               | `handlePlaneOrigin{X,Y,Z}Change()`                           |
| plane xAxis x/y/z      | x-axis      | `Stepper` ×3        | **yes**               | `handlePlaneXAxis{X,Y,Z}Change()`                            |
| plane yAxis x/y/z      | y-axis      | `Stepper` ×3        | **yes**               | `handlePlaneYAxis{X,Y,Z}Change()`                            |
| fix piece button       | —           | `Button`            | **yes** (conditional) | `fixPieces()` — shown only when `piece.plane` is null        |

##### Design Piece Fields (nested design reference)

| Control ID          | Label          | Widget     | Editable              | Change Handler                |
| ------------------- | -------------- | ---------- | --------------------- | ----------------------------- |
| `...design.name`    | design name    | `Combobox` | **yes**               | `handleDesignNameChange()`    |
| `...design.variant` | design variant | `Combobox` | **yes** (conditional) | `handleDesignVariantChange()` |
| `...design.view`    | design view    | `Combobox` | **yes** (conditional) | `handleDesignViewChange()`    |

> Both piece kinds share name, description, attributes, scale, color, center, and plane fields.

##### Multi-Selection Behavior

- Uses `getCommonValue()` for shared values across selected pieces.
- `handleBulk*` creates `{ id, diff }[]` arrays and calls `updatePieces()`.
- Mixed piece types (normal + design) show warning message.
- Parent connection auto-detected from `metadata.parentPieceId` and shown inline.

#### 3.3 Connection Section

| Section ID (single)        | Section ID (multi)            |
| -------------------------- | ----------------------------- |
| `...connection.properties` | `...connection.multipleTitle` |

**Single Connection (`SingleConnectionInfo` + `SingleConnectionFields`):**

##### Connection Info (read-only)

Uses two endpoint groups labeled `connecting` and `connected`. Within each group, the fields use the concise labels `piece`, `port`, and `design piece`.

| Control ID                              | Label                   | Widget  | Editable         | Data Source                               |
| --------------------------------------- | ----------------------- | ------- | ---------------- | ----------------------------------------- |
| `...connection.connectingPieceId`       | piece                   | `Input` | no (`disabled`)  | `connection.connecting.piece.guid`        |
| `...connection.connectingPortId`        | port                    | `Input` | no (`disabled`)  | `connection.connecting.connector?.guid`   |
| `...connection.connectingDesignPieceId` | design piece            | `Input` | no (conditional) | `connection.connecting.designPiece?.guid` |
| `...connection.connectedPieceId`        | piece                   | `Input` | no (`disabled`)  | `connection.connected.piece.guid`         |
| `...connection.connectedPortId`         | port                    | `Input` | no (`disabled`)  | `connection.connected.connector?.guid`    |
| `...connection.connectedDesignPieceId`  | design piece            | `Input` | no (conditional) | `connection.connected.designPiece?.guid`  |

##### Connection Fields (editable)

| Control ID                  | Label       | Widget          | Editable | Range    | Step | Change Handler     |
| --------------------------- | ----------- | --------------- | -------- | -------- | ---- | ------------------ |
| `...connection.description` | description | `Textarea lazy` | **yes**  | —        | —    | `setDescription()` |
| **Translation**             |             |                 |          |          |      |                    |
| `...connection.gap`         | gap         | `Slider`        | **yes**  | -100–100 | 0.1  | `setGap()`         |
| `...connection.shift`       | shift       | `Slider`        | **yes**  | -100–100 | 0.1  | `setShift()`       |
| `...connection.rise`        | rise        | `Slider`        | **yes**  | -100–100 | 0.1  | `setRise()`        |
| **Orientation**             |             |                 |          |          |      |                    |
| `...connection.rotation`    | rotation    | `Slider`        | **yes**  | -180–180 | 1    | `setRotation()`    |
| `...connection.turn`        | turn        | `Slider`        | **yes**  | -180–180 | 1    | `setTurn()`        |
| `...connection.tilt`        | tilt        | `Slider`        | **yes**  | -180–180 | 1    | `setTilt()`        |
| **Diagram Position**        |             |                 |          |          |      |                    |
| `...connection.x`           | x           | `Stepper`       | **yes**  | —        | 0.1  | `setU()`           |
| `...connection.y`           | y           | `Stepper`       | **yes**  | —        | 0.1  | `setV()`           |

> Hooks: `useConnectionGap`, `useConnectionShift`, `useConnectionRise`, `useConnectionRotation`, `useConnectionTurn`, `useConnectionTilt`, `useConnectionU`, `useConnectionV`, `useConnectionDescription` — all from `ConnectionScopeProvider`.

##### Multi-Connection Behavior

Uses `getCommonValue()` and `handleBulkUpdate()` with identical widget layout.

#### 3.4 Connector Section

| Section ID                                      |
| ----------------------------------------------- |
| `semio.sketchpad.app.type.connector.properties` |

**Shown when a connector (port on a piece) is selected in the diagram.**

| Control ID                    | Label           | Widget     | Editable         | Data Source                       |
| ----------------------------- | --------------- | ---------- | ---------------- | --------------------------------- |
| `...connector.id`             | id              | `Input`    | no (`disabled`)  | `connector.guid` or `"~default~"` |
| `...connector.name`           | name            | `Input`    | no (conditional) | `connector.name`                  |
| `...connector.t`              | t               | `Input`    | no               | `connector.t` (4 decimals)        |
| `...connector.description`    | description     | `Textarea` | no (conditional) | `connector.description`           |
| `...connector.port`           | port            | `Input`    | no (conditional) | `connector.port.guid`             |
| `...connector.mandatory`      | mandatory       | `Input`    | no (conditional) | `"yes"` / `"no"`                  |
| `...connector.position`       | position        | `Input`    | no               | `(x, y, z)` formatted             |
| `...connector.direction`      | direction       | `Input`    | no               | `(x, y, z)` formatted             |
| `...connector.compatiblePort` | compatible port | `Input` ×N | no               | each compatible port guid         |
| `...connector.attribute`      | attribute       | `Input` ×N | no               | `"key: value (unit)"`             |

**Tree structure pattern across the Design app:**

- `DesignSection` mixes flat `TreeRow` fields with grouped `TreeItem` collections for `location`, `authors`, and `attributes`.
- `PiecesSection` uses `TreeItem` heavily for grouped subobjects such as `pieceInfo`, `center`, `plane`, and nested plane axes/origin groups.
- `ConnectionsSection` uses `TreeItem` for endpoint groups and translation/orientation/diagram subtrees, with `TreeRow` leaves for the actual controls.
- `ConnectorSection` is the simplest design-side override and uses `TreeRow` only.
- `SortableTreeItems` is used anywhere design details support add/remove/reorder operations.

---

### 4. Type App (`Type.tsx`)

The Type app has its own detail-panel registration logic and is not covered by the older three-app model in this file.

**Section registration logic:**

- If one connector is selected, the app adds `semio.sketchpad.app.type.connector.properties`.
- If multiple connectors are selected, the app adds `semio.sketchpad.app.type.panel.details.section.connectors.multipleTitle`.
- It always adds `semio.sketchpad.app.type.properties` containing a composite form:
  - `TypeDetails`
  - `ModelsSection`
  - `ConnectorsListSection`
  - `AuthorsSection`
  - `AttributesSection`
- It also always adds the fallback kit metadata section `semio.sketchpad.app.kit.properties`.

**Tree structure pattern in the Type app:**

- `TypeDetails` uses `TreeRow` only for scalar editable fields.
- `ModelsSection` uses a parent `TreeItem` with add action, then `SortableTreeItems`, then one nested `TreeItem` per model with `TreeRow` children for url, description, and tags.
- `ConnectorsListSection` uses a parent `TreeItem` with add action, an optional `TreeRow` for the ring editor, then `SortableTreeItems`, then one nested `TreeItem` per connector.
- Each connector item contains nested `TreeItem` groups for point and direction axes and `TreeRow` leaves for port, description, and compatible ports.
- `AuthorsSection` and `AttributesSection` follow the same repeatable pattern as design authors/attributes: parent `TreeItem` + `SortableTreeItems` + nested `TreeItem` children + `TreeRow` leaves.
- The connector-only override section uses the same idea but focuses on the currently selected connector(s) instead of the whole type form.

**Practical rule:** in the Type app, `TreeItem` means either “repeatable collection” or “vector/grouped subobject”, while `TreeRow` still means “single field row”.

---

### 5. Quality App (`Quality.tsx`)

The Quality app contributes one detail section:

| Section ID                          | Specificity | Order |
| ----------------------------------- | ----------- | ----- |
| `semio.sketchpad.app.quality.title` | 20          | 0     |

**Tree structure pattern in `QualityDetails`:**

- Uses `TreeRow` only.
- Every quality field is rendered as a flat row:
  - key
  - name
  - description
  - formula
  - default SI / imperial units
  - kind
  - canScale
  - default / min / max values
  - min/max exclusion flags
- The editable field is `formula`; the rest are read-only or disabled.

**Practical rule:** the Quality app is the simplest details-panel consumer. It uses panel sections plus flat rows and does not use `TreeItem` for nesting.

---

### 6. Docs App (`Docs.tsx`)

The Docs app contributes one detail section:

| Section ID                      | Specificity | Order |
| ------------------------------- | ----------- | ----- |
| `semio.sketchpad.app.docs.page` | 20          | 1     |

**Tree structure pattern in `Details`:**

- The docs details panel does **not** use `TreeRow` or `TreeItem`.
- It renders a flat list of buttons from extracted page headings.
- Clicking a button scrolls the page window to the matching heading.

**Practical rule:** Docs still participates in the same `PanelSection` registration system, but its detail content is plain custom JSX rather than tree-form property rows.

---

## UI Widget Inventory

| Widget              | Import         | Purpose                       | Key Props                                                                    |
| ------------------- | -------------- | ----------------------------- | ---------------------------------------------------------------------------- |
| `Input`             | `@semio/ui`    | Single-line text              | `lazy`, `readOnly`, `disabled`, `showLabel`, `placeholderId`, `onLazyChange` |
| `Textarea`          | `@semio/ui`    | Multi-line text               | `lazy`, `readOnly`, `disabled`, `showLabel`, `placeholderId`, `onLazyChange` |
| `Stepper`           | `@semio/ui`    | Numeric increment/decrement   | `value`, `onChange`, `step`, `showLabel`                                     |
| `Slider`            | `@semio/ui`    | Range slider                  | `value[]`, `onValueChange`, `min`, `max`, `step`, `showLabel`                |
| `Combobox`          | `@semio/ui`    | Searchable dropdown           | `options`, `value`, `onValueChange`, `allowClear`, `showLabel`               |
| `Toggle`            | `@semio/ui`    | Boolean toggle                | `pressed`, `onPressedChange`, `disabled`, `showLabel`, `icon`                |
| `Button`            | `@semio/ui`    | Action button                 | `onClick`, children                                                          |
| `PanelSection`      | internal panel registry | Top-level details-panel section | `id`, `content`, `specificity`, `order`, `defaultOpen`, `actions[]`      |
| `TreeRow`           | `@semio/ui`    | Single row in tree            | `id`, `className`, `onClick`, `onDoubleClick`, `actions[]`                   |
| `TreeItem`          | `@semio/ui`    | Expandable tree node          | `id`, `label`, `defaultOpen`, `layoutKind`, `sortable`, `actions[]`          |
| `SortableTreeItems` | `@semio/ui`    | Drag-reorderable list wrapper | `items`, `onReorder`, render callback                                        |
| `Label`             | `@semio/ui`    | Localized text label          | `id` (used as i18n key)                                                      |

---

## External Relations and Overwrites

### Data Flow: Outside → Details Panel

| Source                                                                         | Mechanism           | Effect on Detail Panel                                                                                                                                                                                 |
| ------------------------------------------------------------------------------ | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Selection state** (`useKitAppSelection` / `useDesignAppSelection`)           | React context       | Determines which sections are registered. Selection includes `types[]`, `designs[]`, `ports[]`, `tags[]`, `concepts[]`, `files[]`, `folders[]`, `authors[]`, `pieces[]`, `connections[]`, `connector`. |
| **Kit store** (`useKit`, `useKitAppStore`)                                     | Zustand / Y.js sync | Provides read data for all kit-level fields. `kitDataSource.change()` writes back.                                                                                                                     |
| **Design store** (`useDesign`, `useDesignAppTransaction`)                      | Zustand / Y.js sync | Provides read data for design-level fields. `kitCommands.updateDesign()` writes back.                                                                                                                  |
| **Piece update hooks** (`useDesignAppUpdatePiece`, `useDesignAppUpdatePieces`) | React context       | Commands for piece field mutations.                                                                                                                                                                    |
| **Connection scope** (`ConnectionScopeProvider`, `useConnection*`)             | React context       | Provides per-connection field hooks (gap, shift, rise, rotation, turn, tilt, u, v, description).                                                                                                       |
| **Kit commands** (`useKitCommands`)                                            | React context       | `updateDesign()`, `addPiece()`, etc. — imperative mutations.                                                                                                                                           |
| **Transaction** (`useDesignAppTransaction`)                                    | React context       | `transaction.start()` / `transaction.finalize()` wraps undo-able batches.                                                                                                                              |
| **Replacable types/designs** (`useReplacableTypes`, `useReplacableDesigns`)    | Custom hooks        | Filter available types/designs for `Combobox` options based on current selection.                                                                                                                      |
| **Included designs** (`useIncludedDesigns`)                                    | Custom hook         | Maps design-piece GUIDs to their nested design references.                                                                                                                                             |
| **Pieces metadata** (`usePiecesMetadataMap`)                                   | Custom hook         | Provides `parentPieceId` → used to find parent connection.                                                                                                                                             |

### Overwrite / Priority Rules

| Rule                      | Description                                                                                                                                                                            |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Specificity**           | Higher specificity sections override lower ones visually (rendered in order, but specificity determines "importance"). Kit section = 10, selection sections = 30, design section = 20. |
| **Order**                 | Within same specificity, lower `order` renders first (top of panel).                                                                                                                   |
| **Section replacement**   | `removeSection` + `addSection` pattern: every selection change clears previous sections and re-registers fresh ones. No merging.                                                       |
| **Single vs. Multiple**   | Different section IDs for single vs. multiple selection. Single shows full form; multiple shows shared values or summary.                                                              |
| **Mixed selection**       | `MultipleArtifactsSection` shown when `>1` artifact kinds selected in kit app. In design app: warning message when both pieces and connections selected.                               |
| **Lazy vs. Immediate**    | `lazy` widgets defer writes to blur/enter. `Stepper`/`Slider` write immediately on change.                                                                                             |
| **readOnly vs. disabled** | `readOnly`: user can select/copy but not edit. `disabled`: fully non-interactive, grayed out.                                                                                          |
| **Conditional fields**    | Some fields only render when data exists: `type.unit`, `design.location`, `design.createdAt`, `connector.name`, `connector.description`, `piece.plane`, piece `fixPiece` button.       |

---

## Section Tree Hierarchy (Design App — single piece selected)

```
Details Panel
├── [order: 0]  Piece Properties (specificity: 30)
│   └── TreeItem: pieceInfo
│       ├── Combobox: type name  OR  Combobox: design name
│       ├── Combobox: type variant  OR  Combobox: design variant/view
│       ├── Input: piece id (disabled)
│       ├── Input: piece name (lazy)
│       ├── Textarea: piece description (lazy)
│       ├── TreeItem: attributes (sortable, add/remove)
│       │   └── TreeItem per attribute
│       │       ├── Input: key
│       │       ├── Input: value
│       │       ├── Input: unit
│       │       └── Input: definition
│       ├── Stepper: scale
│       ├── Input: color (lazy)
│       ├── TreeItem: center
│       │   ├── Stepper: x
│       │   └── Stepper: y
│       └── TreeItem: plane (conditional)
│           ├── TreeItem: origin  → Stepper: x, y, z
│           ├── TreeItem: xAxis   → Stepper: x, y, z
│           └── TreeItem: yAxis   → Stepper: x, y, z
│   └── TreeItem: parentConnection (conditional — when piece has parent)
│       └── SingleConnectionInfo + SingleConnectionFields
├── [order: 50]  Design Properties (specificity: 20)
│   ├── Input: name (lazy)
│   ├── Textarea: description (lazy)
│   ├── Input: icon, image, variant, view, unit (lazy)
│   ├── TreeItem: location (add/remove action)
│   │   ├── Stepper: longitude
│   │   └── Stepper: latitude
│   ├── TreeItem: authors (sortable, add/remove)
│   │   └── TreeItem per author → Input: name, email
│   ├── TreeItem: attributes (sortable, add/remove)
│   │   └── TreeItem per attribute → Input: key, value, unit, definition
│   ├── Input: createdAt, updatedAt (disabled)
│   └── Input: pieceCount, connectionCount (disabled)
└── [order: 100]  Kit Properties (specificity: 10)
    ├── Input: name, version (lazy)
    ├── Textarea: description (lazy)
    └── Input: icon, image, homepage, license (lazy)
```

---

## Algorithm Details Panel (separate context)

Used inside `semio/ui` for algorithm apps (e.g. inside `AlgorithmApp`). Not part of the sketchpad details panel system.

| Section           | Fields                                      | Widget              | Source                      |
| ----------------- | ------------------------------------------- | ------------------- | --------------------------- |
| Design            | name, pieces count, connections count       | `TreeRow` spans     | `AlgorithmContextValue.kit` |
| Vec (conditional) | u, v                                        | `TreeRow` spans     | `AlgorithmContextValue.vec` |
| Selection         | list of selected pieces with type guids     | `TreeRow` per piece | `selectedPieceGuids`        |
| Output            | status, error, added/removed/updated counts | `TreeRow` spans     | `designDiff`, `error`       |

---

## GraphQL Schema Reference (underlying data model)

### Kit

`name!`, `description!`, `icon!`, `image!`, `preview!`, `version!`, `remote!`, `homepage!`, `license!`, `created!`, `updated!`, `concepts[]`, `types[]`, `designs[]`, `attributes[]`

### Type

`name!`, `description!`, `icon!`, `image!`, `variant!`, `stock!`, `virtual!`, `unit!`, `created!`, `updated!`, `models[]`, `connectors[]`, `authors[]`, `attributes[]`, `kit`, `pieces[]`, `location!`, `concepts[]`

### Design

`name!`, `description!`, `icon!`, `image!`, `variant!`, `view!`, `unit!`, `created!`, `updated!`, `pieces[]`, `connections[]`, `authors[]`, `attributes[]`, `kit`, `location!`, `concepts[]`

### Piece

`description!`, `id_?`, `type`, `plane?`, `attributes[]`, `design`, `center!`, `connections[]`

### Connection

`description!`, `gap!`, `shift!`, `rise!`, `rotation!`, `turn!`, `tilt!`, `x!`, `y!`, `attributes[]`, `design`, `connected!`, `connecting!`

### Connector

`description!`, `mandatory!`, `port!`, `t!`, `id_?`, `attributes[]`, `type`, `compatiblePorts[]`, `point!`, `direction!`, `connections[]`

### Side

`piece!`, `connector!`

### Supporting Types

- `Point { x!, y!, z! }` — 3D position
- `Vector { x!, y!, z! }` — 3D direction
- `Coord { x!, y! }` — 2D diagram position
- `Location { longitude!, latitude! }` — Earth surface
- `Plane { origin: Point!, xAxis: Vector!, yAxis: Vector! }` — Local coordinate system

---

## UI Control Parameters — Sizing, Positioning, Gaps, Rendering

### Design System Foundation

Source: `elements/ui/globals.css` `@layer base :root`

#### Spacing Scale

Two density modes. All sizes derive from `--spacing`.

| Var                | Compact (`.compact` default) | Touch (`.touch`)   | Formula          |
| ------------------ | ---------------------------- | ------------------ | ---------------- |
| `--spacing`        | `0.2rem` (3.2px)             | `0.275rem` (4.4px) | base unit        |
| `--spacing-single` | 3.2px                        | 4.4px              | `1 × --spacing`  |
| `--spacing-double` | 6.4px                        | 8.8px              | `2 × --spacing`  |
| `--size-tiny`      | 9.6px                        | 13.2px             | `3 × --spacing`  |
| `--size-small`     | 16px                         | 22px               | `5 × --spacing`  |
| `--size-medium`    | 22.4px                       | 30.8px             | `7 × --spacing`  |
| `--size-large`     | 28.8px                       | 39.6px             | `9 × --spacing`  |
| `--size-huge`      | 35.2px                       | 48.4px             | `11 × --spacing` |
| `--size-mega`      | 41.6px                       | 57.2px             | `13 × --spacing` |
| `--size-giga`      | 48px                         | 66px               | `15 × --spacing` |

> **Overwrite**: `.touch` class on root overrides `--spacing` to `--spacing-touch`. All derived vars cascade.

#### Typography Scale

| Var           | Compact | Touch    | Line-Height     |
| ------------- | ------- | -------- | --------------- |
| `--text-2xs`  | 0.6rem  | —        | `calc(0.9/0.6)` |
| `--text-xs`   | 0.7rem  | 0.75rem  | `calc(1/0.7)`   |
| `--text-sm`   | 0.8rem  | 0.875rem | `calc(1.2/0.8)` |
| `--text-base` | 0.9rem  | 1rem     | `calc(1.4/0.9)` |
| `--text-lg`   | 1rem    | 1.125rem | `calc(1.6/1)`   |

> **Overwrite**: `.touch` replaces all text vars. Responsive `md:text-sm` on Input/Textarea shrinks on ≥768px.

#### Radii & Shadows

All set to `0rem` / `0 0 rgb(0 0 0 / 0)` — **flat design, no rounding, no shadows**.

---

### Level System (Depth Context)

Source: `elements/ui/index.tsx` — `LevelContext`

Type: `"base" | "window" | "panel" | "overlay" | "temporary"`

| Level       | bg class       | hover class                | CSS bg color (light) | CSS bg color (dark) |
| ----------- | -------------- | -------------------------- | -------------------- | ------------------- |
| `base`      | `bg-base`      | `hover:bg-hover-base`      | `--color-light`      | `--color-dark`      |
| `window`    | `bg-window`    | `hover:bg-hover-window`    | `--color-light-6-7`  | `--color-dark-8-9`  |
| `panel`     | `bg-panel`     | `hover:bg-hover-panel`     | `--color-light-5-7`  | `--color-dark-7-9`  |
| `overlay`   | `bg-overlay`   | `hover:bg-hover-overlay`   | transparent          | transparent         |
| `temporary` | `bg-temporary` | `hover:bg-hover-temporary` | `--color-light-4-7`  | `--color-dark-gray` |

Resolved via `getLevelBgClass(level)` / `getLevelHoverClass(level)`.

> **Overwrite**: `LevelProvider` wraps panel → all children inherit `"panel"` level. Popover/tooltip children inherit `"temporary"`.

---

### Z-Index Layers

| Var             | Value | Usage                            |
| --------------- | ----- | -------------------------------- |
| `--z-base`      | 0     | GoldenLayout root, items         |
| `--z-window`    | 5     | Window content                   |
| `--z-panel`     | 10    | Panel headers, tabs, side panels |
| `--z-overlay`   | 20    | Default Panel `zIndex` prop      |
| `--z-temporary` | 30    | Tooltips, popovers               |
| `--z-navbar`    | 100   | Top navbar                       |
| `--z-modal`     | 1000  | Modal dialogs                    |
| `--z-tutorial`  | 10000 | Tutorial overlays                |

> **Overwrite**: `PanelProps.zIndex` accepts `10|20|30|40`, defaults `20`. `SidePanelProps.zIndex` also defaults `20`.

---

### Panel Container Layout

Source: `Panel` component in `elements/ui/index.tsx`

#### Panel (`PanelProps`)

| Prop             | Default   | Effect                                  |
| ---------------- | --------- | --------------------------------------- |
| `size`           | `250` px  | Width (horizontal) or height (vertical) |
| `minSize`        | `150` px  | Minimum on drag resize                  |
| `maxSize`        | `500` px  | Maximum on drag resize                  |
| `resizeSide`     | `"right"` | Which edge has the drag handle          |
| `opacity`        | `1`       | Container opacity, transition `150ms`   |
| `showBackground` | `true`    | Adds `bg-panel` class                   |
| `visible`        | `true`    | Returns `null` when false               |
| `zIndex`         | `20`      | Inline `z-index` on container           |

Container classes: `absolute text-foreground border min-w-0 overflow-hidden`

Positioning (inline style):

```
left/right: var(--spacing-double)    // inset from window edges
top: var(--spacing-double)
bottom: var(--spacing-double)
width: {size}px
```

Resize handle: `w-single` (3.2px) drag zone, `cursor-ew-resize`

Inner layout:

```
<Scrollable className="h-full">
  <div className="p-single overflow-hidden min-w-0">  // padding: --spacing-single
    <TreeStateProvider>
      <Tree sections={treeSections} />
    </TreeStateProvider>
  </div>
  {footer}
</Scrollable>
```

#### SidePanel (`SidePanelProps`)

| Prop       | Default  | Effect                |
| ---------- | -------- | --------------------- |
| `size`     | `300` px | Panel width           |
| `minSize`  | `200` px | Min drag resize       |
| `maxSize`  | `600` px | Max drag resize       |
| `zIndex`   | `20`     | Z layer               |
| `position` | required | `"left"` or `"right"` |

Tab bar: `h-medium border-b shrink-0 overflow-x-auto`

Tab button: `h-full px-small border-r cursor-pointer` — active: `bg-hover-panel`

Content area: `p-[10px]` (hardcoded)

---

### Tree Layout Constants

Source: `elements/ui/index.tsx`

| Constant                       | Value           | Formula                               |
| ------------------------------ | --------------- | ------------------------------------- |
| `detailPanelIndentPx(level)`   | `level * 10` px | Indent per nesting depth              |
| `detailTreeValueColumnWidthPx` | `160` px        | Fixed right column in property layout |
| `indentationLinePx(i)`         | `i * 10 + 7` px | Vertical guide line x-position        |

---

### Widget Sizing & Positioning

#### Input

| Param       | Value                                      | Source class              |
| ----------- | ------------------------------------------ | ------------------------- |
| Width       | `w-full` + `min-w-0`                       | Fills parent, no overflow |
| Height      | `h-medium` (22.4px compact / 30.8px touch) | Spacing-derived           |
| Padding     | `p-single` (3.2px)                         | Internal                  |
| Border      | `border` (1px solid)                       | Standard                  |
| Text        | `text-base md:text-sm`                     | Responsive                |
| Background  | `bg-transparent`                           | Inherits from level       |
| Focus       | `focus-visible:border-accent`              | Primary accent ring       |
| Disabled    | `opacity-50 cursor-not-allowed`            | Grayed out                |
| Placeholder | `text-muted-foreground`                    | Muted color               |
| File button | `h-medium text-sm`                         | Upload variant            |

#### Textarea

| Param      | Value                                  | Source class   |
| ---------- | -------------------------------------- | -------------- |
| Width      | `w-full`                               | Full container |
| Min Height | `min-h-huge` (35.2px)                  | Minimum size   |
| Height     | `field-sizing-content` (CSS)           | Auto-grow      |
| Padding    | `px-tiny py-single` (9.6px H, 3.2px V) | Asymmetric     |
| Text       | `text-base md:text-sm`                 | Same as Input  |

#### Stepper

| Param     | Value                                       | Source class                 |
| --------- | ------------------------------------------- | ---------------------------- |
| Container | `w-[100px] h-[22px]`                        | **Hardcoded** fixed size     |
| Border    | `rounded-[3px] border`                      | Only component with rounding |
| Overflow  | `overflow-hidden`                           | Clips buttons                |
| − button  | `w-[22px] h-[22px] border-r`                | Left square                  |
| + button  | `w-[22px] h-[22px] border-l`                | Right square                 |
| Input     | `w-[56px] h-[22px] text-center px-0`        | Center area                  |
| Icons     | `size-tiny` (9.6px)                         | − and + icons                |
| Focus     | `focus:outline-none focus-visible:bg-muted` | Subtle highlight             |

#### Slider

| Param         | Value                                   | Source class            |
| ------------- | --------------------------------------- | ----------------------- |
| Container     | `grid h-[22px]`                         | Grid layout             |
| Grid columns  | `grid-cols-[minmax(0,1fr)_28px]`        | Track + value readout   |
| Column gap    | `gap-x-[8px]`                           | Between track and value |
| Track height  | `h-single` (3.2px)                      | Thin rail               |
| Track width   | `w-full`                                | Full column span        |
| Thumb         | `size-small` (16px) `border-foreground` | Draggable circle        |
| Value display | `w-[28px] text-right text-xs`           | Numeric display         |
| Value edit    | Double-click activates input            | Inline edit mode        |

#### Combobox

| Param           | Value                                   | Source class         |
| --------------- | --------------------------------------- | -------------------- |
| Trigger         | Same as Input: `h-medium w-full border` | Matches Input sizing |
| Popover         | `w-auto p-single min-w-[120px]`         | Auto-width dropdown  |
| List max-height | `max-h-[300px]`                         | Scroll containment   |
| Item padding    | `p-single gap-single`                   | Standard spacing     |
| Item text       | `text-sm`                               | Smaller than input   |
| Empty message   | `py-medium text-center text-sm`         | Centered hint        |

#### Toggle / ToggleGroup

| Param               | Value                                   | Source class     |
| ------------------- | --------------------------------------- | ---------------- |
| Height              | `h-medium` (22.4px)                     | Standard         |
| Width (icon-only)   | `aspect-square`                         | Square button    |
| Width (with text)   | `aspect-auto`                           | Elongated        |
| Padding (icon-only) | `p-single`                              | 3.2px all sides  |
| Padding (with text) | `py-single px-double`                   | 3.2px V, 6.4px H |
| Gap (icon+text)     | `gap-single` (3.2px)                    | Internal         |
| Group gap           | `gap-single` (3.2px)                    | Between items    |
| Text                | `text-xs whitespace-nowrap`             | Compact no-wrap  |
| Active              | `bg-active-base text-active-foreground` | Primary red      |
| Disabled            | `opacity-50 cursor-not-allowed`         | Standard         |
| Focus               | `focus-visible:ring-ring/50 ring-[3px]` | Standard ring    |

#### Button

| Param   | Value                                | Source class     |
| ------- | ------------------------------------ | ---------------- |
| Height  | `h-medium` (22.4px)                  | Standard         |
| Padding | `px-double py-single`                | 6.4px H, 3.2px V |
| Border  | `border`                             | 1px solid        |
| Text    | `text-sm`                            | Slightly larger  |
| Hover   | Level-aware via `getLevelHoverClass` | Dynamic          |

---

### Tree Section Header Rendering

This section describes the internal section header rendered by `Tree`. It is not a separate app-facing `TreeSection` component used directly by the detail-panel apps in `semio/sketchpad`.

| Param               | Value                                                    | Source                             |
| ------------------- | -------------------------------------------------------- | ---------------------------------- |
| Row height          | `20px` (inline style)                                    | Hardcoded                          |
| Row margin-bottom   | `6px` (inline style)                                     | Hardcoded                          |
| Indent padding-left | `level * 10px`                                           | `detailPanelIndentPx()`            |
| Content gap         | `gap-[6px]`                                              | Between chevron/icon/label/actions |
| Chevron icon        | 14px                                                     | Collapse/expand                    |
| Label text          | `text-xs font-semibold uppercase tracking-wide truncate` | Header style                       |
| Action buttons gap  | `gap-single` (3.2px)                                     | Between action icons               |
| Hover               | `hover:bg-hover-panel`                                   | Level-driven                       |
| Cursor              | `cursor-foldable` (expandable) / `cursor-selectable`     | Context-aware                      |

Indentation lines: 1px wide, color `border`, drawn at `x = i*10 + 7 - 0.5` px for each ancestor level `i`.

---

### TreeItem Rendering

| Param                | Value                                              | Source                   |
| -------------------- | -------------------------------------------------- | ------------------------ |
| Row height           | `20px` implicit                                    | From `flex items-center` |
| Row vertical gap     | `gap-y-[2px]`                                      | Between tree items       |
| Content gap          | `gap-[6px]`                                        | Between icon/label/value |
| Indent               | `level * 10px` padding-left                        | Same as section          |
| Label                | `text-xs font-normal truncate flex-1 min-w-0`      | Fills remaining space    |
| **Property variant** | `grid grid-cols-[1fr_160px] gap-x-[8px]`           | Name/value 2-column      |
| Property min-height  | `min-h-[24px]`                                     | Taller than standard     |
| Right column         | `min-w-0 flex items-center justify-end gap-single` | Value alignment          |
| Selection bg         | `bg-active-base text-active-foreground`            | Primary highlight        |
| Drag handle          | `cursor-grab active:cursor-grabbing`               | Drag feedback            |
| Drag visual          | `opacity: 0.5` while dragging                      | Reduced visibility       |

---

### Scrollable Container

| Param         | Value                                | Source              |
| ------------- | ------------------------------------ | ------------------- |
| Root          | `relative`                           | Position context    |
| Viewport      | `size-full`                          | Full parent         |
| Scrollbar (V) | `w-2.5 border-l-transparent p-[1px]` | Thin track          |
| Scrollbar (H) | `h-2.5 border-t-transparent p-[1px]` | Horizontal          |
| Thumb         | `bg-border rounded-full flex-1`      | Standard appearance |

---

### Popover & Tooltip

| Param              | Value                                                                                       | Source           |
| ------------------ | ------------------------------------------------------------------------------------------- | ---------------- |
| PopoverContent     | `bg-popover text-popover-foreground z-temporary w-72 border p-1`                            | Default popover  |
| Popover sideOffset | `4px`                                                                                       | Gap from trigger |
| Tooltip            | `bg-temporary border border-accent-foreground z-temporary p-single text-xs w-max max-w-fit` | Compact tooltip  |
| Tooltip animation  | `fade-in-0 zoom-in-95` / `fade-out-0 zoom-out-95`                                           | Enter/exit       |

---

### Color System

#### Light Theme (default `:root`)

| Semantic Var                 | Maps To             | Hex     |
| ---------------------------- | ------------------- | ------- |
| `--base`                     | `--color-light`     | #f7f3e3 |
| `--window`                   | `--color-light-6-7` | #ebe8d9 |
| `--panel`                    | `--color-light-5-7` | #c9c8bd |
| `--temporary`                | `--color-light-4-7` | #979b94 |
| `--foreground`               | `--color-dark`      | #001117 |
| `--muted-foreground`         | `--color-gray`      | #7b827d |
| `--accent` / `--active-base` | `--color-primary`   | #ff344f |
| `--accent-secondary`         | `--color-secondary` | #34d1bf |
| `--accent-tertiary`          | `--color-tertiary`  | #fa9500 |
| `--border-*`                 | `--color-gray`      | #7b827d |
| `--hover-panel`              | `--color-gray-600`  | #a2a59d |

#### Dark Theme (`.dark`)

| Semantic Var          | Maps To             | Hex     |
| --------------------- | ------------------- | ------- |
| `--base`              | `--color-dark`      | #001117 |
| `--window`            | `--color-dark-8-9`  | #07181d |
| `--panel`             | `--color-dark-7-9`  | #1d2b2f |
| `--temporary`         | `--color-dark-gray` | #243235 |
| `--foreground`        | `--color-light`     | #f7f3e3 |
| `--hover-panel`       | `--color-gray-400`  | #555f5d |
| `--active-foreground` | `--color-light`     | #f7f3e3 |

> **Overwrite**: `.dark` class on root replaces all semantic vars. Components don't need to know the theme.

#### Status Colors (both themes)

| Var               | Hex     | Usage                              |
| ----------------- | ------- | ---------------------------------- |
| `--color-danger`  | #a60009 | Destructive actions, invalid state |
| `--color-warning` | #fccf05 | Warning alerts                     |
| `--color-success` | #7eb77f | Success feedback                   |
| `--color-info`    | #dbbea1 | Informational notes                |

---

### Cursor System

Source: `globals.css :root`

All cursors are custom SVGs with fallbacks. Light/dark variants swap via `.dark`.

| Cursor var                    | Usage                                    |
| ----------------------------- | ---------------------------------------- |
| `--cursor-default`            | All `*` elements                         |
| `--cursor-pointer`            | Buttons, `[role="button"]`, links        |
| `--cursor-selectable`         | `.react-flow__node`, tree items          |
| `--cursor-foldable`           | Expandable sections                      |
| `--cursor-grab/grabbing`      | Drag handles                             |
| `--cursor-ew-resize`          | Horizontal resize handles                |
| `--cursor-ns-resize`          | Vertical resize handles                  |
| `--cursor-crosshair-centered` | ReactFlow `.connectionindicator` handles |
| `--cursor-not-allowed`        | Disabled elements                        |
| `--cursor-text`               | Text inputs                              |

> **Overwrite**: CSS specificity cascade: `body.cursor-selectable *` force-overrides all children. `.dark` swaps all SVG URLs to `*_dark.svg` variants.

---

### Toolbar System Variables

| Var                        | Compact                         | Touch                           | Usage                |
| -------------------------- | ------------------------------- | ------------------------------- | -------------------- |
| `--toolbar-item-height`    | `var(--size-medium)` (22.4px)   | `var(--size-large)` (39.6px)    | Button/toggle height |
| `--toolbar-gap`            | `var(--spacing-single)` (3.2px) | `var(--spacing-double)` (8.8px) | Between items        |
| `--toolbar-group-gap`      | `var(--spacing-double)` (6.4px) | —                               | Between groups       |
| `--toolbar-padding-inline` | `var(--spacing-single)`         | `var(--spacing-double)`         | Horizontal padding   |
| `--toolbar-divider-height` | `var(--size-small)` (16px)      | —                               | Separator height     |

Toolbar container: `bg-panel h-[var(--toolbar-item-height)] gap-[var(--toolbar-gap)] border rounded-md px-[var(--toolbar-padding-inline)] shadow-sm overflow-hidden`

---

### GoldenLayout Integration

Source: `globals.css` — `.lm_*` classes

| Element                    | Sizing                                                 | Style                      |
| -------------------------- | ------------------------------------------------------ | -------------------------- |
| `.lm_header`               | `height: 20px` (compact) / `28px` (touch)              | `border-bottom: 1px`       |
| `.lm_tab`                  | `padding: 2px 8px` (compact) / `4px 12px` (touch)      | `bg: var(--window)`        |
| `.lm_title`                | `font-size: var(--text-xs)` / `var(--text-sm)` (touch) | `color: var(--foreground)` |
| `.lm_splitter` (V)         | `width: var(--spacing-single)`                         | `bg: var(--base)`          |
| `.lm_splitter` (H)         | `height: var(--spacing-single)`                        | `bg: var(--base)`          |
| `.lm_content`              | fills parent                                           | `bg: var(--window)`        |
| `.lm_item.lm_stack::after` | `inset: 0`                                             | `box-shadow` borders       |

> **Overwrite**: `.touch` class overrides header/tab/title sizes. All GL styles use `!important`.

---

### Section Spacing Summary (Gap Map)

Context: **Panel → Tree → PanelSection/section header → TreeItem → Widget**

| Between                           | Gap                                   | Source                         |
| --------------------------------- | ------------------------------------- | ------------------------------ |
| Panel edge → content              | `p-single` (3.2px)                    | Panel inner div                |
| SidePanel edge → content          | `p-[10px]` (hardcoded)                | SidePanel content div          |
| Section → section                 | 0px (no gap, sections stack directly) | Tree sections are sequential   |
| Section header margin-bottom      | `6px`                                 | Inline style on section header |
| TreeItem → TreeItem (vertical)    | `gap-y-[2px]`                         | Flex gap between rows          |
| Indent per level                  | `10px`                                | `detailPanelIndentPx()`        |
| Label/value gap (property layout) | `gap-x-[8px]`                         | Grid column gap                |
| Icon/label gap within row         | `gap-[6px]`                           | Flex gap                       |
| Value column width                | `160px`                               | `detailTreeValueColumnWidthPx` |
| Slider track/value gap            | `gap-x-[8px]`                         | Grid gap                       |
| Toggle group item gap             | `gap-single` (3.2px)                  | ToggleGroup                    |
| Action buttons gap                | `gap-single` (3.2px)                  | section header actions         |

---

### Render Conditions & Conditional Fields

| Field                        | Condition                        | Effect                                |
| ---------------------------- | -------------------------------- | ------------------------------------- |
| `type.unit`                  | `type.unit` exists               | Row appears only when truthy          |
| `design.location`            | Has `location` action            | Add/remove toggles entire subtree     |
| `design.createdAt/updatedAt` | Always present                   | Shown as `disabled`                   |
| `piece.plane`                | `piece.plane !== null`           | Plane origin/xAxis/yAxis subtree      |
| `piece.fixPiece` button      | `piece.plane === null`           | Fix button shown only when no plane   |
| `connector.name/description` | Conditional on data              | Only shown when truthy                |
| `connector.port`             | Conditional on data              | Only shown when truthy                |
| Parent connection subtree    | `metadata.parentPieceId` exists  | Inline connection below piece         |
| Design piece fields          | `piece.type` has nested design   | Swaps type combobox → design combobox |
| Multi-edit warning           | Mixed piece+connection selection | Warning message at order 20           |
| Empty panel message          | No sections registered           | Shows `emptyMessage` prop             |

---

### Interaction State Mapping

| State                 | Visual                           | CSS pattern                                           |
| --------------------- | -------------------------------- | ----------------------------------------------------- |
| Default               | Level bg, standard text          | `bg-{level}`                                          |
| Hover                 | Level hover bg                   | `hover:bg-hover-{level}`                              |
| Focus (keyboard)      | Accent ring 3px                  | `focus-visible:ring-ring/50 ring-[3px]`               |
| Focus (input)         | Accent border                    | `focus-visible:border-accent`                         |
| Selected              | Primary red bg + dark/light text | `bg-active-base text-active-foreground`               |
| Disabled              | 50% opacity, no-cursor           | `disabled:opacity-50 cursor-not-allowed`              |
| ReadOnly              | Normal cursor, selectable text   | No visual change, no writing                          |
| Dragging              | 50% opacity                      | `opacity: isDragging ? 0.5 : 1`                       |
| Resizing handle hover | Accent border on panel edge      | `border-{side}-accent`                                |
| Invalid               | Destructive ring + border        | `aria-invalid:ring-destructive/20 border-destructive` |

---

### Custom Tailwind Utilities Summary

Source: `globals.css` `@utility` declarations

| Utility                                     | CSS Value                                         | Category   |
| ------------------------------------------- | ------------------------------------------------- | ---------- |
| `gap-single`                                | `var(--spacing-single)`                           | Spacing    |
| `gap-double`                                | `var(--spacing-double)`                           | Spacing    |
| `p-single` / `p-double`                     | `var(--spacing-single)` / `var(--spacing-double)` | Padding    |
| `px-single` / `px-tiny` / `px-small`        | Horizontal padding                                | Padding    |
| `py-single` / `py-double` / `py-tiny`       | Vertical padding                                  | Padding    |
| `m-single` / `mx-single` / `my-single`      | Margin variants                                   | Margin     |
| `size-dot`                                  | `calc(2 × --spacing)` square                      | Sizing     |
| `size-tiny` through `size-giga`             | `3×` to `15×` spacing                             | Sizing     |
| `h-medium` / `w-medium`                     | `var(--size-medium)`                              | Sizing     |
| `min-w-tiny` through `min-w-large`          | Min width constraints                             | Sizing     |
| `max-w-tiny` through `max-w-large`          | Max width constraints                             | Sizing     |
| `min-h-huge`                                | `var(--size-huge)`                                | Sizing     |
| `z-base` through `z-tutorial`               | Layer z-indexes                                   | Z-index    |
| `cursor-pointer` / `cursor-selectable` etc. | Custom cursor SVGs                                | Cursors    |
| `text-tiny`                                 | `var(--text-2xs)`                                 | Typography |

---

### Overwrite Hierarchy (from most to least specific)

```
1. Inline style (e.g. Panel `width: {size}px`, `zIndex`)
   ↓
2. Component prop (e.g. `disabled`, `readOnly`, `lazy`, `className`)
   ↓
3. Level context (`LevelProvider` → bg/hover classes)
   ↓
4. Density mode (`.touch` class → overrides spacing, text, toolbar vars)
   ↓
5. Theme (`.dark` class → overrides color semantic vars)
   ↓
6. CSS custom properties (`:root` → base spacing, sizes, colors)
   ↓
7. Tailwind utilities (compile-time classes from @utility declarations)
   ↓
8. @theme inline (maps semantic vars → Tailwind color tokens)
```

Key override points for visual changes:

- **Spacing everywhere**: Change `--spacing` in `:root` → cascades to all sizes
- **Individual widget**: Add `className` prop → merges via `cn()` (clsx+twMerge)
- **Panel width**: Change `size` prop or `minSize`/`maxSize` on Panel/SidePanel
- **Tree indentation**: Modify `detailPanelIndentPx()` constant (currently `level * 10`)
- **Tree value column**: Modify `detailTreeValueColumnWidthPx` (currently `160px`)
- **Section row height**: Inline `20px` in the tree section header / `TreeItem` row (not variable-driven)
- **Section margin**: Inline `6px` marginBottom (not variable-driven)
- **Theme colors**: Override semantic vars in `.dark` or `:root`
- **Density**: Toggle `.touch` class on root element
