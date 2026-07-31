---
goal: SKETCHPADFEATURES
---

# Ticket: Detail Panel Gap Analysis

## Summary

Complete feature-by-feature gap analysis between old and new detail panel builds. Found 3 MISSING features, 1 PARTIAL feature, and 15 NEW features in the new build.

## Plan

1. Read OLD build completely (all 1352 lines) ✅
2. Read NEW build sections: DesignSectionForm, PiecesSectionForm, ConnectionsSectionForm, ConnectorSectionForm, panel routing ✅
3. Compare feature-by-feature ✅
4. Document gap analysis ✅

## Gap Analysis

### DESIGN SECTION

| FEATURE                         | OLD BUILD                                                    | NEW BUILD                                            | STATUS      |
| ------------------------------- | ------------------------------------------------------------ | ---------------------------------------------------- | ----------- |
| Name (editable)                 | `Input label="Name"` with onChange/onFocus/onBlur            | `Input lazy id="...design.name"` with onLazyChange   | MIGRATED    |
| Description (editable)          | `Textarea label="Description"`                               | `Textarea lazy id="...design.description"`           | MIGRATED    |
| Icon (editable)                 | `Input label="Icon"`                                         | `Input lazy id="...design.icon"`                     | MIGRATED    |
| Image (editable)                | `Input label="Image URL"`                                    | `Input lazy id="...design.image"`                    | MIGRATED    |
| Variant (editable)              | `Input label="Variant"`                                      | `Input lazy id="...design.variant"`                  | MIGRATED    |
| View (editable)                 | `Input label="View"`                                         | `Input lazy id="...design.view"`                     | MIGRATED    |
| Unit (editable)                 | `Input label="Unit"`                                         | `Input lazy id="...design.unit"`                     | MIGRATED    |
| Location add/remove             | Add/Remove with Plus/Minus icons, lon/lat steppers           | Add/Remove with AddIcon/RemoveIcon, lon/lat steppers | MIGRATED    |
| Location longitude stepper      | `Stepper label="Longitude" step={0.000001}`                  | `Stepper id="...longitude" step={0.000001}`          | MIGRATED    |
| Location latitude stepper       | `Stepper label="Latitude" step={0.000001}`                   | `Stepper id="...latitude" step={0.000001}`           | MIGRATED    |
| Authors add/remove/reorder      | SortableTreeItems with Plus/Trash2 buttons                   | SortableTreeItems with AddIcon/RemoveIcon actions    | MIGRATED    |
| Author name (editable)          | `Input label="Name"` per author                              | `Input id="...authors.name"` per author              | MIGRATED    |
| Author email (editable)         | `Input label="Email"` per author                             | `Input id="...authors.email"` per author             | MIGRATED    |
| Attributes add/remove/reorder   | SortableTreeItems with key/value/unit/definition             | SortableTreeItems with key/value/unit/definition     | MIGRATED    |
| Attribute key (editable)        | `Input label="Name"`                                         | `Input id="...attributes.name"`                      | MIGRATED    |
| Attribute value (editable)      | `Input label="Value"`                                        | `Input id="...attributes.value"`                     | MIGRATED    |
| Attribute unit (editable)       | `Input label="Unit"`                                         | `Input id="...attributes.unit"`                      | MIGRATED    |
| Attribute definition (editable) | `Input label="Definition"`                                   | `Input id="...attributes.definition"`                | MIGRATED    |
| Metadata: Created date          | `Input label="Created" disabled`                             | `Input id="...createdAt" disabled`                   | MIGRATED    |
| Metadata: Updated date          | `Input label="Updated" disabled`                             | `Input id="...updatedAt" disabled`                   | MIGRATED    |
| Metadata: Piece count           | `Input label="Pieces" disabled` showing `N pieces`           | NOT PRESENT                                          | **MISSING** |
| Metadata: Connection count      | `Input label="Connections" disabled` showing `N connections` | NOT PRESENT                                          | **MISSING** |

### PIECES SECTION

| FEATURE                      | OLD BUILD                                                             | NEW BUILD                                                                   | STATUS                          |
| ---------------------------- | --------------------------------------------------------------------- | --------------------------------------------------------------------------- | ------------------------------- |
| ID (disabled)                | `Input label="ID" disabled`                                           | `Input id="...piece.id" disabled`                                           | MIGRATED                        |
| Type name (combobox)         | `Combobox label="Type"` with availableTypeNames                       | `Combobox id="...piece.type"` with availableTypeNames                       | MIGRATED                        |
| Type variant (combobox)      | `Combobox label="Variant"` with availableVariants                     | `Combobox id="...type.variant"` with availableVariants                      | MIGRATED                        |
| Design name (combobox)       | `Combobox label="Design Name"` for design pieces                      | `Combobox id="...design.name"` for design pieces                            | MIGRATED                        |
| Design variant (combobox)    | `Combobox label="Design Variant"` for design pieces                   | `Combobox id="...design.variant"` for design pieces                         | MIGRATED                        |
| Design view (combobox)       | `Combobox label="Design View"` for design pieces                      | `Combobox id="...design.view"` for design pieces                            | MIGRATED                        |
| Center X stepper             | `Stepper label="X"` with center.x                                     | `Stepper id="...center.x"` with center.u                                    | MIGRATED (renamed x→u)          |
| Center Y stepper             | `Stepper label="Y"` with center.y                                     | `Stepper id="...center.y"` with center.v                                    | MIGRATED (renamed y→v)          |
| Plane origin X/Y/Z steppers  | 3 steppers for origin x,y,z                                           | 3 steppers for origin x,y,z                                                 | MIGRATED                        |
| Plane X-axis X/Y/Z steppers  | NOT PRESENT                                                           | 3 steppers for xAxis x,y,z                                                  | **NEW** (added)                 |
| Plane Y-axis X/Y/Z steppers  | NOT PRESENT                                                           | 3 steppers for yAxis x,y,z                                                  | **NEW** (added)                 |
| Name (editable)              | NOT PRESENT                                                           | `Input lazy id="...piece.name"` with handleNameChange                       | **NEW** (added)                 |
| Description (editable)       | NOT PRESENT in OLD pieces section                                     | `Textarea lazy id="...piece.description"`                                   | **NEW** (added)                 |
| Scale (editable)             | NOT PRESENT                                                           | `Stepper id="...piece.scale"`                                               | **NEW** (added)                 |
| Color (editable)             | NOT PRESENT                                                           | `Input lazy id="...piece.color"`                                            | **NEW** (added)                 |
| Attributes (add/remove/edit) | NOT PRESENT in OLD build                                              | Full add/remove/update with key/value/unit/definition via SortableTreeItems | **NEW** (added)                 |
| Fix piece button             | `Pin` icon action on section header                                   | `Button id="...fixPiece"` with DisconnectIcon shown when `!piece.plane`     | MIGRATED (improved positioning) |
| Parent connection display    | `ConnectionsSection` embedded via `<div style={marginTop}>`           | `ConnectionsSection` embedded via `<div style={marginTop}>`                 | MIGRATED                        |
| Mixed selection message      | "Selection contains both design pieces..." in TreeSection             | `useLabel("...mixedSelectionMessage")` in TreeItem                          | MIGRATED                        |
| Multi-select support         | `setPieces()` for bulk updates                                        | `updatePieces()` for bulk updates                                           | MIGRATED                        |
| "No valid pieces" message    | "No valid pieces found in selection."                                 | Same message via `hasNoValidPieces` check                                   | MIGRATED                        |
| Multiple pieces label        | `Multiple Pieces (N)` / `Multiple Design Pieces (N)` in section label | Section ID switches between single/multiple IDs                             | MIGRATED (different approach)   |

### CONNECTIONS SECTION

| FEATURE                                 | OLD BUILD                                                  | NEW BUILD                                                                           | STATUS                                      |
| --------------------------------------- | ---------------------------------------------------------- | ----------------------------------------------------------------------------------- | ------------------------------------------- |
| Connecting piece ID (disabled)          | `Input label="Piece ID"` for connecting side               | `Input id="...connectingPieceId"` using `connection.connecting.piece.guid`          | MIGRATED                                    |
| Connecting port/connector ID (disabled) | `Input label="Port ID"` for connecting port                | `Input id="...connectingConnectorId"` using `connection.connecting.connector?.guid` | MIGRATED (renamed port→connector)           |
| Connecting designPiece ID (disabled)    | `Input label="Design Piece ID"` if exists                  | `Input id="...connectingDesignPieceId"` if exists                                   | MIGRATED                                    |
| Connected piece ID (disabled)           | `Input label="Piece ID"` for connected side                | `Input id="...connectedPieceId"` using `connection.connected.piece.guid`            | MIGRATED                                    |
| Connected port/connector ID (disabled)  | `Input label="Port ID"` for connected port                 | `Input id="...connectedConnectorId"` using `connection.connected.connector?.guid`   | MIGRATED (renamed port→connector)           |
| Connected designPiece ID (disabled)     | `Input label="Design Piece ID"` if exists                  | `Input id="...connectedDesignPieceId"` if exists                                    | MIGRATED                                    |
| Gap stepper                             | `Stepper label="Gap" step={0.1}`                           | `Stepper id="...connection.gap" step={0.1}`                                         | MIGRATED                                    |
| Shift stepper                           | `Stepper label="Shift" step={0.1}`                         | `Stepper id="...connection.shift" step={0.1}`                                       | MIGRATED                                    |
| Rise stepper                            | `Stepper label="Rise" step={0.1}`                          | `Stepper id="...connection.rise" step={0.1}`                                        | MIGRATED                                    |
| Rotation slider (-180 to 180)           | `Slider label="Rotation" min={-180} max={180} step={1}`    | `Slider id="...connection.rotation" min={-180} max={180} step={1}`                  | MIGRATED                                    |
| Turn slider (-180 to 180)               | `Slider label="Turn" min={-180} max={180} step={1}`        | `Slider id="...connection.turn" min={-180} max={180} step={1}`                      | MIGRATED                                    |
| Tilt slider (-180 to 180)               | `Slider label="Tilt" min={-180} max={180} step={1}`        | `Slider id="...connection.tilt" min={-180} max={180} step={1}`                      | MIGRATED                                    |
| X offset stepper (diagram)              | `Stepper label="X Offset" step={0.1}`                      | `Stepper id="...connection.u" step={0.1}`                                           | MIGRATED (renamed x→u)                      |
| Y offset stepper (diagram)              | `Stepper label="Y Offset" step={0.1}`                      | `Stepper id="...connection.v" step={0.1}`                                           | MIGRATED (renamed y→v)                      |
| Description (editable)                  | NOT PRESENT                                                | `Textarea lazy id="...connection.description"` for single & bulk                    | **NEW** (added)                             |
| Attributes (editable)                   | NOT PRESENT                                                | NOT PRESENT                                                                         | N/A (neither has it)                        |
| Multi-select support                    | `setConnections()` for bulk updates                        | `handleBulkUpdate()` with `updateConnections()`                                     | MIGRATED                                    |
| "Editing N connections" message         | `Editing {N} connections simultaneously` text              | NOT PRESENT as explicit message (no count display for multi-select)                 | **MISSING**                                 |
| Nested "Plane > Translation" grouping   | `TreeItem label="Plane" > TreeItem label="Translation"`    | Flat list of steppers (no nested grouping)                                          | **PARTIAL** (flattened, no document labels) |
| Nested "Plane > Orientation" grouping   | `TreeItem label="Orientation"` grouping rotation/turn/tilt | Flat list of sliders (no nested grouping)                                           | **PARTIAL** (flattened, no document labels) |
| Nested "Diagram" grouping               | `TreeItem label="Diagram"` grouping x/y offsets            | Flat list of steppers (no grouping)                                                 | **PARTIAL** (flattened, no document label)  |

### PORT/CONNECTOR SECTION

| FEATURE                              | OLD BUILD (PortSection)                      | NEW BUILD (ConnectorSectionForm)                           | STATUS                            |
| ------------------------------------ | -------------------------------------------- | ---------------------------------------------------------- | --------------------------------- |
| ID (disabled)                        | `Input label="ID"` showing port.id\_         | `Input id="...connector.id"` showing connector.guid        | MIGRATED                          |
| Name (disabled)                      | NOT PRESENT                                  | `Input id="...connector.name"` if exists                   | **NEW** (added)                   |
| T parameter (disabled)               | NOT PRESENT                                  | `Input id="...connector.t"` showing connector.t            | **NEW** (added)                   |
| Description (disabled)               | `Textarea label="Description"` if exists     | `Textarea id="...connector.description"` if exists         | MIGRATED                          |
| Family (disabled)                    | `Input label="Family"` if port.family exists | NOT PRESENT (replaced by port reference)                   | **CHANGED**                       |
| Port reference (disabled)            | NOT PRESENT                                  | `Input id="...connector.port"` showing connector.port.guid | **NEW** (replaces family)         |
| Mandatory (disabled)                 | `Input label="Mandatory"` Yes/No             | `Input id="...connector.mandatory"` with i18n Yes/No       | MIGRATED                          |
| Position point (disabled)            | `Input label="Position"` formatted (x,y,z)   | `Input id="...connector.position"` formatted (x,y,z)       | MIGRATED                          |
| Direction vector (disabled)          | `Input label="Direction"` formatted (x,y,z)  | `Input id="...connector.direction"` formatted (x,y,z)      | MIGRATED                          |
| Compatible families/ports (disabled) | Iterated `compatibleFamilies` list           | Iterated `compatiblePorts` list                            | MIGRATED (renamed families→ports) |
| Attributes (disabled list)           | Iterated attributes with key:value(unit)     | Iterated attributes with key:value(unit)                   | MIGRATED                          |
| "Port not found" message             | "Port not found" text                        | i18n `connector.notFound` message                          | MIGRATED                          |

### PANEL ROUTING

| FEATURE                                         | OLD BUILD (Details component)                                     | NEW BUILD (useEffect panel registration)                                              | STATUS                                       |
| ----------------------------------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------- | -------------------------------------------- |
| No selection → Design section                   | `!hasSelection && <DesignSection />`                              | `addSection("details", { id: "...design.properties", specificity: 20 })` always added | MIGRATED (always present at low specificity) |
| Port/Connector selected → Port section          | `hasPortSelected && <PortSection .../>`                           | `addSection("details", { id: "...connector.properties", specificity: 40 })`           | MIGRATED                                     |
| Pieces selected → Pieces section                | `hasPieces && !hasPortSelected && <PiecesSection .../>`           | `addSection("details", { id: piecesSectionId, specificity: 30 })`                     | MIGRATED                                     |
| Connections selected → Connections section      | `hasConnections && !hasPortSelected && <ConnectionsSection .../>` | `addSection("details", { id: connectionsSectionId, specificity: 30 })`                | MIGRATED                                     |
| Mixed pieces+connections → message              | `hasPieces && hasConnections` shows mixed message                 | `hasPieces && hasConnections` adds selectionMultipleId section                        | MIGRATED                                     |
| Resize handle                                   | Manual mouseDown/mouseMove resize with min/max width (150-500px)  | NOT PRESENT in detail sections (handled by panel infrastructure)                      | MIGRATED (external)                          |
| ScrollArea wrapper                              | `<ScrollArea className="h-full">`                                 | Handled by panel infrastructure                                                       | MIGRATED (external)                          |
| KitScopeProvider / DesignScopeProvider wrapping | NOT PRESENT (used hooks directly)                                 | All sections wrapped in `<KitScopeProvider><DesignScopeProvider>`                     | **IMPROVED**                                 |

## Summary of Gaps

### MISSING (must be added to new build):

1. **Design Section - Piece count metadata**: OLD showed `N pieces` disabled input. NEW has no piece/connection count display.
2. **Design Section - Connection count metadata**: OLD showed `N connections` disabled input. NEW has no piece/connection count display.
3. **Connections Section - "Editing N connections" message**: OLD showed `Editing {N} connections simultaneously` when multi-selecting. NEW has no equivalent count message.

### PARTIAL (present but incomplete):

4. **Connections Section - Nested grouping**: OLD had `Plane > Translation`, `Plane > Orientation`, `Diagram` nested TreeItem grouping. NEW has flat list without document labels.

### NEW features in new build (not in old):

5. Piece name field (editable)
6. Piece description field (editable)
7. Piece scale stepper
8. Piece color field
9. Piece attributes (add/remove/update with key/value/unit/definition)
10. Plane X-axis and Y-axis steppers for pieces
11. Connector name field
12. Connector t parameter field
13. Connector port reference field
14. Connection description field
15. KitScopeProvider/DesignScopeProvider wrapping
16. i18n support throughout (useLabel, useTranslation)
17. Lazy input mode for text fields
18. Connection scope provider for single connection fields
19. Fix piece button now shown inline when piece has no plane

## Changes

No code changes - analysis only.

## Log

- Read OLD build completely (1352 lines)
- Read NEW build DesignSectionForm (lines 4145-4650)
- Read NEW build PiecesSectionForm (lines 4660-5670)
- Read NEW build ConnectionsSectionForm (lines 5680-5840)
- Read NEW build ConnectorSectionForm (lines 5930-6020)
- Read NEW build panel routing (lines 9950-10090)
- Completed gap analysis

## Todos

- [x] Read old build
- [x] Read new build sections
- [x] Compare feature-by-feature
- [x] Document findings
