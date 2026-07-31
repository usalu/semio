---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Comprehensive gap analysis of the detail panel comparing old build (Design.Details.tsx.old) vs new build (Design.tsx) across all entity types: Design, Piece, Connection, Connector/Port.

## Plan

1. Read all Zod schemas in compose.ts for canonical property definitions
2. Read old Design.Details.tsx.old (1352 lines) completely
3. Read new Design.tsx detail panel sections (L4128-5710)
4. Compare old vs new vs schema for every entity type
5. Document migration gaps and new properties

## Todos

- [x] Read all Zod schema properties for Kit, Design, Type, Piece, Connection, Connector, Port
- [x] Read old Design.Details.tsx.old completely (all 4 sections)
- [x] Read new DesignSectionForm (L4144-4577)
- [x] Read new PiecesSectionForm (L4583-5388)
- [x] Read new ConnectionsSectionForm (L5503-5607)
- [x] Read new ConnectorSectionForm (L5627-5704)
- [x] Write comprehensive gap analysis

## Analysis

### 1. DESIGN SECTION

#### Schema Properties (Design in compose.ts L5823-7098)

`guid, name, parent, isAbstract, folder, pieces[], connections[], stats[], props[], layers[], activeLayer, groups[], canScale, canMirror, unit, location, authors[], concepts[], icon, image, description, attributes[], createdAt, updatedAt`

#### Old Build (Design.Details.tsx.old L35-412)

| Property                    | Rendered | Editable                                            |
| --------------------------- | -------- | --------------------------------------------------- |
| name                        | ✅       | ✅ Combobox                                         |
| description                 | ✅       | ✅ Textarea                                         |
| icon                        | ✅       | ✅ Input                                            |
| image                       | ✅       | ✅ Input                                            |
| variant                     | ✅       | ✅ Combobox                                         |
| view                        | ✅       | ✅ Combobox                                         |
| unit                        | ✅       | ✅ Combobox                                         |
| location (longitude)        | ✅       | ✅ Stepper (add/remove)                             |
| location (latitude)         | ✅       | ✅ Stepper (add/remove)                             |
| authors                     | ✅       | ✅ Sortable (name/email, add/remove)                |
| attributes                  | ✅       | ✅ Sortable (key/value/unit/definition, add/remove) |
| metadata (created)          | ✅       | ❌ disabled                                         |
| metadata (updated)          | ✅       | ❌ disabled                                         |
| metadata (piece count)      | ✅       | ❌ disabled                                         |
| metadata (connection count) | ✅       | ❌ disabled                                         |

#### New Build (Design.tsx L4144-4577)

| Property             | Rendered | Editable                                            |
| -------------------- | -------- | --------------------------------------------------- |
| name                 | ✅       | ✅ Combobox                                         |
| description          | ✅       | ✅ Textarea                                         |
| icon                 | ✅       | ✅ Input                                            |
| image                | ✅       | ✅ Input                                            |
| variant              | ✅       | ✅ Combobox                                         |
| view                 | ✅       | ✅ Combobox                                         |
| unit                 | ✅       | ✅ Combobox                                         |
| location (longitude) | ✅       | ✅ Stepper (add/remove)                             |
| location (latitude)  | ✅       | ✅ Stepper (add/remove)                             |
| authors              | ✅       | ✅ Sortable (name/email, add/remove)                |
| attributes           | ✅       | ✅ Sortable (key/value/unit/definition, add/remove) |
| createdAt            | ✅       | ❌ disabled                                         |
| updatedAt            | ✅       | ❌ disabled                                         |

#### Design Gap Status: ✅ FULLY MIGRATED

The new build has feature parity with the old build. Piece count/connection count metadata are computed values not stored in schema. All stored properties are rendered. New schema properties `parent`, `isAbstract`, `folder`, `stats`, `props`, `layers`, `activeLayer`, `groups`, `canScale`, `canMirror`, `concepts` are structural/internal and not shown in either old or new.

---

### 2. PIECE SECTION

#### Schema Properties (Piece in compose.ts L4889-5175)

`guid, name, type (TypeId), design (DesignId), plane (Plane), center (Coord {u,v}), scale, mirrorPlane, isHidden, isLocked, color, description, props[], attributes[]`

#### Old Build (Design.Details.tsx.old L414-1032)

| Property             | Rendered | Editable    | Notes                    |
| -------------------- | -------- | ----------- | ------------------------ |
| type.name            | ✅       | ✅ Combobox | Regular piece type name  |
| type.variant         | ✅       | ✅ Combobox | Regular piece variant    |
| id (guid)            | ✅       | ❌ disabled | Single select only       |
| design.name          | ✅       | ✅ Combobox | Design piece name        |
| design.variant       | ✅       | ✅ Combobox | Design piece variant     |
| design.view          | ✅       | ✅ Combobox | Design piece view        |
| center.x             | ✅       | ✅ Stepper  |                          |
| center.y             | ✅       | ✅ Stepper  |                          |
| plane.origin (x,y,z) | ✅       | ✅ Stepper  |                          |
| parent connection(s) | ✅       | ✅ inline   | Connection details shown |
| fix pieces action    | ✅       | ✅ Button   | For unconnected pieces   |
| multi-select support | ✅       | ✅          | Common values shown      |
| mixed type warning   | ✅       | ❌          | Design + regular mix     |

#### New Build (Design.tsx L4583-5388)

| Property             | Rendered | Editable    | Notes                     |
| -------------------- | -------- | ----------- | ------------------------- |
| type.name            | ✅       | ✅ Combobox | Regular piece type name   |
| type.variant         | ✅       | ✅ Combobox | Regular piece variant     |
| id (guid)            | ✅       | ❌ disabled | Single select only        |
| design.name          | ✅       | ✅ Combobox | Design piece name         |
| design.variant       | ✅       | ✅ Combobox | Design piece variant      |
| design.view          | ✅       | ✅ Combobox | Design piece view         |
| center.u             | ✅       | ✅ Stepper  | Was center.x in old       |
| center.v             | ✅       | ✅ Stepper  | Was center.y in old       |
| plane.origin (x,y,z) | ✅       | ✅ Stepper  |                           |
| plane.xAxis (x,y,z)  | ✅       | ✅ Stepper  | NEW: not in old build     |
| plane.yAxis (x,y,z)  | ✅       | ✅ Stepper  | NEW: not in old build     |
| parent connection(s) | ✅       | ✅ inline   | Inline ConnectionsSection |
| fix pieces action    | ✅       | ✅ Button   | For unconnected pieces    |
| multi-select support | ✅       | ✅          | Common values shown       |
| mixed type warning   | ✅       | ❌          | Design + regular mix      |

#### Piece Gap Status: ✅ FULLY MIGRATED (plus improvements)

The new build has full parity with the old build and adds plane xAxis/yAxis editing (old only had origin). Schema properties NOT shown in EITHER old or new:

- **name**: piece name (string) — not currently editable in detail panel
- **scale**: numeric scale factor — not shown
- **mirrorPlane**: Plane for mirroring — not shown
- **isHidden**: boolean — not shown (controlled via layers/diagram)
- **isLocked**: boolean — not shown (controlled via layers/diagram)
- **color**: color override — not shown
- **description**: text description — not shown
- **props[]**: quality property values — not shown
- **attributes[]**: key-value attributes — not shown

#### Piece Properties to Consider Adding

| Property     | Priority | Notes                                               |
| ------------ | -------- | --------------------------------------------------- |
| name         | HIGH     | Allow naming pieces for identification              |
| description  | MEDIUM   | Descriptive text                                    |
| props[]      | HIGH     | Quality-based property values (e.g. dimensions)     |
| attributes[] | MEDIUM   | Generic key-value metadata (same pattern as Design) |
| color        | LOW      | Color override (may be better in layer system)      |
| isHidden     | LOW      | Currently via diagram/layer                         |
| isLocked     | LOW      | Currently via diagram/layer                         |
| scale        | MEDIUM   | Scaling factor for the piece                        |
| mirrorPlane  | LOW      | Advanced mirroring geometry                         |

---

### 3. CONNECTION SECTION

#### Schema Properties (Connection in compose.ts L5456-5690)

`guid, connected (Side), connecting (Side), gap, shift, rise, rotation, turn, tilt, u, v, description, attributes[]`

Where Side = `{ piece (PieceId), designPiece (PieceId?), connector (ConnectorId?) }`

#### Old Build (Design.Details.tsx.old L1034-1234)

| Property                  | Rendered | Editable                | Notes                   |
| ------------------------- | -------- | ----------------------- | ----------------------- |
| connecting.piece.id       | ✅       | ❌ disabled             |                         |
| connecting.port.id        | ✅       | ❌ disabled             | Was port, now connector |
| connecting.designPiece.id | ✅       | ❌ disabled             |                         |
| connected.piece.id        | ✅       | ❌ disabled             |                         |
| connected.port.id         | ✅       | ❌ disabled             | Was port, now connector |
| connected.designPiece.id  | ✅       | ❌ disabled             |                         |
| gap                       | ✅       | ✅ Stepper              | Translation             |
| shift                     | ✅       | ✅ Stepper              | Translation             |
| rise                      | ✅       | ✅ Stepper              | Translation             |
| rotation                  | ✅       | ✅ Slider (-180 to 180) | Orientation             |
| turn                      | ✅       | ✅ Slider (-180 to 180) | Orientation             |
| tilt                      | ✅       | ✅ Slider (-180 to 180) | Orientation             |
| u (x offset)              | ✅       | ✅ Stepper              | Diagram offset          |
| v (y offset)              | ✅       | ✅ Stepper              | Diagram offset          |
| multi-select support      | ✅       | ✅                      | Common values           |

#### New Build (Design.tsx L5397-5607)

Single connection (via ConnectionScopeProvider + SingleConnectionInfo + SingleConnectionFields):

| Property                    | Rendered | Editable                | Notes               |
| --------------------------- | -------- | ----------------------- | ------------------- |
| connecting.piece.guid       | ✅       | ❌ disabled             |                     |
| connecting.connector.guid   | ✅       | ❌ disabled             | Renamed from port   |
| connecting.designPiece.guid | ✅       | ❌ disabled             | Conditional         |
| connected.piece.guid        | ✅       | ❌ disabled             |                     |
| connected.connector.guid    | ✅       | ❌ disabled             | Renamed from port   |
| connected.designPiece.guid  | ✅       | ❌ disabled             | Conditional         |
| gap                         | ✅       | ✅ Stepper              |                     |
| shift                       | ✅       | ✅ Stepper              |                     |
| rise                        | ✅       | ✅ Stepper              |                     |
| rotation                    | ✅       | ✅ Slider (-180 to 180) |                     |
| turn                        | ✅       | ✅ Slider (-180 to 180) |                     |
| tilt                        | ✅       | ✅ Slider (-180 to 180) |                     |
| u                           | ✅       | ✅ Stepper              |                     |
| v                           | ✅       | ✅ Stepper              |                     |
| multi-select support        | ✅       | ✅                      | Bulk update handler |

#### Connection Gap Status: ✅ FULLY MIGRATED

All old properties are present. Schema properties NOT shown in EITHER old or new:

- **description**: text description — not shown
- **attributes[]**: key-value attributes — not shown

#### Connection Properties to Consider Adding

| Property     | Priority | Notes                                 |
| ------------ | -------- | ------------------------------------- |
| description  | MEDIUM   | Descriptive text about the connection |
| attributes[] | MEDIUM   | Generic key-value metadata            |

---

### 4. CONNECTOR (PORT) SECTION

#### Schema Properties

**Connector** (compose.ts L4265-4484): `guid, name, t, point (Point), direction (Vector), description, port (PortId?), mandatory, props[], attributes[]`
**Port** (compose.ts L2961-3219): `guid, name, description, icon, compatiblePorts[], attributes[]`

#### Old Build (Design.Details.tsx.old L1236-1287) — called "PortSection"

| Property            | Rendered | Editable | Notes                            |
| ------------------- | -------- | -------- | -------------------------------- |
| port id             | ✅       | ❌       | Was "port id"                    |
| description         | ✅       | ❌       |                                  |
| family              | ✅       | ❌       | Port family name                 |
| mandatory           | ✅       | ❌       |                                  |
| position (x,y,z)    | ✅       | ❌       |                                  |
| direction (x,y,z)   | ✅       | ❌       |                                  |
| compatible families | ✅       | ❌       | List of compatible port families |
| attributes          | ✅       | ❌       | Key-value display                |

#### New Build (Design.tsx L5627-5704) — called "ConnectorSectionForm"

| Property            | Rendered | Editable | Notes                    |
| ------------------- | -------- | -------- | ------------------------ |
| connector id (guid) | ✅       | ❌       | Con connector guid       |
| description         | ✅       | ❌       | Conditional              |
| port (guid)         | ✅       | ❌       | Port reference           |
| mandatory           | ✅       | ❌       | Conditional              |
| position (x,y,z)    | ✅       | ❌       | Formatted string         |
| direction (x,y,z)   | ✅       | ❌       | Formatted string         |
| compatible ports    | ✅       | ❌       | Iterates compatiblePorts |
| attributes          | ✅       | ❌       | Key-value display        |

#### Connector Gap Status: ✅ FULLY MIGRATED

All old properties are present with updated naming (port → connector). Schema properties NOT shown in EITHER old or new:

- **name**: connector name — not shown
- **t**: parametric position 0-1 — not shown
- **props[]**: quality property values — not shown

#### Connector Properties to Consider Adding

| Property | Priority | Notes                             |
| -------- | -------- | --------------------------------- |
| name     | MEDIUM   | Connector name for identification |
| t        | LOW      | Parametric position along edge    |
| props[]  | LOW      | Connector-level property values   |

---

### 5. OVERALL MIGRATION SUMMARY

#### Migration Status by Section

| Section    | Status      | Notes                                        |
| ---------- | ----------- | -------------------------------------------- |
| Design     | ✅ Complete | Full parity                                  |
| Piece      | ✅ Complete | Full parity + plane axis editing improvement |
| Connection | ✅ Complete | Full parity                                  |
| Connector  | ✅ Complete | Full parity, renamed from port               |

#### Schema Properties NOT Exposed in Either Build (Enhancement Opportunities)

| Entity     | Property     | Schema Type | Priority |
| ---------- | ------------ | ----------- | -------- |
| Piece      | name         | string      | HIGH     |
| Piece      | description  | string      | MEDIUM   |
| Piece      | props[]      | Prop[]      | HIGH     |
| Piece      | attributes[] | Attribute[] | MEDIUM   |
| Piece      | scale        | number      | MEDIUM   |
| Piece      | color        | string      | LOW      |
| Piece      | isHidden     | boolean     | LOW      |
| Piece      | isLocked     | boolean     | LOW      |
| Piece      | mirrorPlane  | Plane       | LOW      |
| Connection | description  | string      | MEDIUM   |
| Connection | attributes[] | Attribute[] | MEDIUM   |
| Connector  | name         | string      | MEDIUM   |
| Connector  | t            | number      | LOW      |
| Connector  | props[]      | Prop[]      | LOW      |

#### Key Architectural Differences (Old vs New)

1. **Store pattern**: Old used `useDesignEditorCommands` / `useDesignEditorSelection` → New uses `useDesignAppTransaction` / `useDesignAppUpdatePiece` / `useDesignAppUpdatePieces` / `useDesignAppUpdateConnections`
2. **Scope providers**: New uses `ConnectionScopeProvider` with individual field hooks (`useConnectionGap`, `useConnectionShift`, etc.) for single connection editing
3. **Panel registration**: New uses `useAddPanelSection` / `useRemovePanelSection` system for conditional section rendering
4. **i18n**: New uses `useTranslation` / `useLabel` for all strings
5. **Type references**: Old used `type.name`/`type.variant` strings → New uses `type.guid` (TypeId) with kit resolution
6. **Design references**: Old used `type.name === "design"` + `type.variant` → New uses `piece.design.guid` (DesignId)
7. **Coordinate naming**: Old used `center.x/y` → New uses `center.u/v` (Coord schema)
8. **Port → Connector**: Renamed throughout (PortSection → ConnectorSection, port.id → connector.guid)

## Changes

No code changes — this is a research/analysis ticket.

## Log

- Read all Zod schemas in compose.ts for Kit, Design, Type, Piece, Connection, Connector, Port, and supporting types
- Read complete old Design.Details.tsx.old (1352 lines, 4 sections)
- Read complete new Design.tsx detail panel (L4128-5710, 4 sections)
- Performed property-by-property comparison for all 4 entity sections
- Documented migration status, gaps, and enhancement opportunities
