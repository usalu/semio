---
goal: R26-02/RUNNING-SKETCHPAD
---

# Ticket

## Summary

Phase 3: Added Connecting/Connected TreeItem grouping to SingleConnectionInfo, added i18n keys for group labels, translated remaining German labels. All old build features fully migrated. TypeScript 0 errors, 13/13 tests pass.
## Changes

- `semio/js/sketchpad/Design.tsx`: Added piece name/description/scale/color/attributes fields to PiecesSectionForm. Added connection description to SingleConnectionFields and ConnectionsSectionForm (bulk). Added connector name and t fields to ConnectorSectionForm. Added piece count and connection count metadata to DesignSectionForm. Added "Editing N connections simultaneously" message to bulk connection editing. Restructured connection fields (gap/shift/rise, rotation/turn/tilt, u/v) into nested TreeItem groups (Plane > Translation, Plane > Orientation, Diagram). Wrapped SingleConnectionInfo connecting/connected fields under "Connecting" and "Connected" TreeItem groups.
- `semio/js/sketchpad/Sketchpad.tsx`: Added `useConnectionDescription` hook following existing connection hook pattern.
- `semio/js/sketchpad/locales/en.json`: Added i18n keys for piece (name, namePlaceholder, description, descriptionPlaceholder, scale, color, colorPlaceholder, attributes, attribute), connection (description, descriptionPlaceholder, multipleEditing, plane, translation, orientation, diagram, connecting, connected), connector (name, t), design metadata (pieceCount, connectionCount).
- `semio/js/sketchpad/locales/de.json`: Added German translations for all new keys including connecting/connected group labels and connector field labels.

## Log

Phase 1:
- Analyzed PieceDiffSchema and AttributesDiffSchema to get correct diff format
- Added handler functions (handleNameChange, handleDescriptionChange, handleScaleChange, handleColorChange, handleAttributeAdd, handleAttributeRemove, handleAttributeUpdate)
- Fixed AttributesDiff format: removed uses `{guid}` not raw string, updated uses `{attribute: {guid}, diff: {field: value}}`
- Fixed union type issues with `(piece as any)` casts for fields not on all union members
- Created useConnectionDescription hook in Sketchpad.tsx
- Replaced Stepper with disabled Input for connector t (Stepper has no disabled prop)
- TypeScript compilation passes with zero errors
- All 13 tests pass

Phase 2:
- Read all 1352 lines of Design.Details.tsx.old for comprehensive gap analysis
- Identified 3 missing features: piece count metadata, connection count metadata, "Editing N connections" message
- Identified 1 partial feature: flat connection fields needed grouping
- Added piece count and connection count as disabled Input fields after updatedAt in DesignSectionForm
- Added "Editing N connections simultaneously" message using multipleEditing i18n key with {{count}} template
- Restructured SingleConnectionFields and ConnectionsSectionForm bulk path with nested TreeItem groups
- Added i18n keys for pieceCount, connectionCount, plane, translation, orientation, diagram in en.json and de.json
- TypeScript compilation passes with zero errors
- All 13 tests pass

Phase 3:
- Old build had connecting/connected fields grouped under "Connecting" and "Connected" TreeItem labels
- New build had them flat in SingleConnectionInfo — visual difference from old build
- Wrapped connecting fields (PieceId, ConnectorId, DesignPieceId) under TreeItem with id "semio.sketchpad.app.design.panel.details.section.connection.connecting"
- Wrapped connected fields under TreeItem with id "semio.sketchpad.app.design.panel.details.section.connection.connected"
- Added i18n keys for "connecting" and "connected" group labels in en.json and de.json
- Translated remaining untranslated de.json connector field labels (Connecting Connector, Connecting Design Piece, Connected Connector, Connected Design Piece)
- TypeScript compilation passes with zero errors
- All 13 tests pass

## Todos

- [x] Add piece field handlers (name, description, scale, color, attributes)
- [x] Add piece name JSX field after ID
- [x] Add piece description/scale/color JSX fields after type selection
- [x] Add piece attributes section with SortableTreeItems
- [x] Add useConnectionDescription hook to Sketchpad.tsx
- [x] Add connection description to single and bulk forms
- [x] Add connector name and t read-only fields
- [x] Add i18n translations (en.json + de.json)
- [x] Build verification (tsc --noEmit passes)
- [x] Test verification (13/13 tests pass)
- [x] Add piece count + connection count metadata to DesignSectionForm
- [x] Add "Editing N connections simultaneously" message to ConnectionsSectionForm bulk path
- [x] Add connection field grouping (Plane > Translation/Orientation, Diagram)
- [x] Add i18n for pieceCount and connectionCount keys
- [x] Build + test verification for phase 2
- [x] Add Connecting/Connected TreeItem grouping to SingleConnectionInfo
- [x] Add i18n keys for connecting/connected group labels (en + de)
- [x] Translate remaining untranslated de.json connector field labels
- [x] Build + test verification for phase 3

## Plan

Phase 1 (completed):
1. Research schemas (PieceDiffSchema, ConnectionDiffSchema, AttributesDiffSchema)
2. Add piece property handlers following existing pattern
3. Add JSX for new piece fields (name, description, scale, color, attributes)
4. Add connection description hook and fields
5. Add connector name/t display fields
6. Add i18n translations

Phase 2 (completed):
1. Add piece count and connection count metadata fields to DesignSectionForm after updatedAt
2. Add "Editing N connections simultaneously" message to ConnectionsSectionForm bulk path
3. Add nested connection field grouping (Plane > Translation, Plane > Orientation, Diagram)
4. Add i18n for new pieceCount/connectionCount keys
5. Build + test verification

Phase 3 (completed):
1. Wrap SingleConnectionInfo connecting fields under "Connecting" TreeItem group
2. Wrap SingleConnectionInfo connected fields under "Connected" TreeItem group
3. Add i18n keys for "connecting" and "connected" group labels (en + de)
4. Translate remaining untranslated German connector field labels
5. Build + test verification

## Feature-by-Feature Gap Analysis (Old vs New Build)

### PIECE Section (PiecesSectionForm)

| # | Feature | Old Build | New Build (Design.tsx) | Status |
|---|---------|-----------|----------------------|--------|
| 1 | **Piece ID** (disabled input) | YES | YES (L5299, `getPieceId(piece)`) | PARITY |
| 2 | **Piece Name** (editable input) | NO | YES (L5304-5312, `handleNameChange`) | NEW in new build |
| 3 | **Type Name** (Combobox) | YES | YES (L5352-5362, `handleTypeNameChange`) | PARITY |
| 4 | **Type Variant** (Combobox) | YES | YES (L5367-5380, `handleTypeVariantChange`) | PARITY |
| 5 | **Design Name** (Combobox) | YES | YES (L5316-5327, `handleDesignNameChange`) | PARITY |
| 6 | **Design Variant** (Combobox) | YES | YES (L5329-5341, `handleDesignVariantChange`) | PARITY |
| 7 | **Design View** (Combobox) | YES | YES (L5343-5351, `handleDesignViewChange`) | PARITY |
| 8 | **Description** (Textarea) | NO | YES (L5385-5394, `handleDescriptionChange`) | NEW in new build |
| 9 | **Scale** (Stepper) | NO | YES (L5398-5404, `handleScaleChange`) | NEW in new build |
| 10 | **Color** (Input) | NO | YES (L5407-5416, `handleColorChange`) | NEW in new build |
| 11 | **Center > X** (Stepper) | YES | YES (L5420, `handleCenterXChange`) | PARITY |
| 12 | **Center > Y** (Stepper) | YES | YES (L5425, `handleCenterYChange`) | PARITY |
| 13 | **Connected piece info msg** | YES | YES (L5457-5465) | PARITY |
| 14 | **Fix Piece button** (DisconnectIcon) | YES (Pin icon) | YES (L5462-5464, `fixPieces`, DisconnectIcon) | PARITY |
| 15 | **Plane > Origin > X/Y/Z** (Steppers) | YES | YES (L5471-5487, handlers L5002-5024) | PARITY |
| 16 | **Plane > X-Axis > X/Y/Z** (Steppers) | NO | YES (L5490-5504, handlers L5026-5048) | NEW in new build |
| 17 | **Plane > Y-Axis > X/Y/Z** (Steppers) | NO | YES (L5507-5521, handlers L5050-5072) | NEW in new build |
| 18 | **Attributes** (CRUD list) | NO | YES (L5525-5608, add/remove/update handlers) | NEW in new build |
| 19 | **Parent Connection** section | YES | YES (L5610-5614, delegates to `ConnectionsSection`) | PARITY |
| 20 | **Mixed Selection** message | YES | YES (L5290-5294, `hasMixedTypes`) | PARITY |
| 21 | **Multi-piece editing** | YES (basic) | YES (all handlers support batch `updatePieces`) | PARITY+ |

### CONNECTION Section (ConnectionsSectionForm / SingleConnectionInfo / SingleConnectionFields)

| # | Feature | Old Build | New Build (Design.tsx) | Status |
|---|---------|-----------|----------------------|--------|
| 1 | **Connecting > Piece ID** (disabled) | YES | YES (L5641, `connection.connecting.piece.guid`) | PARITY |
| 2 | **Connecting > Connector ID** (disabled) | YES (Port ID) | YES (L5646, `connection.connecting.connector?.guid`) | PARITY |
| 3 | **Connecting > Design Piece ID** (disabled, conditional) | YES | YES (L5650-5654, conditional on `connection.connecting.designPiece`) | PARITY |
| 4 | **Connected > Piece ID** (disabled) | YES | YES (L5657, `connection.connected.piece.guid`) | PARITY |
| 5 | **Connected > Connector ID** (disabled) | YES (Port ID) | YES (L5662, `connection.connected.connector?.guid`) | PARITY |
| 6 | **Connected > Design Piece ID** (disabled, conditional) | YES | YES (L5666-5670, conditional on `connection.connected.designPiece`) | PARITY |
| 7 | **Description** (Textarea) | NO | YES (L5688-5696, `useConnectionDescription`) | NEW in new build |
| 8 | **Gap** (Stepper) | YES | YES (L5700) | PARITY |
| 9 | **Shift** (Stepper) | YES | YES (L5705) | PARITY |
| 10 | **Rise** (Stepper) | YES | YES (L5710) | PARITY |
| 11 | **Rotation** (Slider) | YES | YES (L5718) | PARITY |
| 12 | **Turn** (Slider) | YES | YES (L5725) | PARITY |
| 13 | **Tilt** (Slider) | YES | YES (L5732) | PARITY |
| 14 | **X Offset / U** (Stepper) | YES | YES (L5741) | PARITY |
| 15 | **Y Offset / V** (Stepper) | YES | YES (L5746) | PARITY |
| 16 | **Plane > Translation group** (TreeItem) | flat in old | YES (L5698, nested TreeItem) | IMPROVED |
| 17 | **Plane > Orientation group** (TreeItem) | flat in old | YES (L5714, nested TreeItem) | IMPROVED |
| 18 | **Diagram group** (TreeItem) | flat in old | YES (L5738, nested TreeItem) | IMPROVED |
| 19 | **"Editing N connections"** msg | YES | YES (L5805-5809, `multipleEditing` i18n with `{{count}}`) | PARITY |
| 20 | **Bulk connection editing** | YES | YES (L5813+, `handleBulkUpdate`) | PARITY |
| 21 | **Bulk description editing** | NO | YES (L5813-5822) | NEW in new build |

### CONNECTOR Section (ConnectorSectionForm)

| # | Feature | Old Build (Port) | New Build (Design.tsx) | Status |
|---|---------|------------------|----------------------|--------|
| 1 | **ID** (disabled) | YES | YES (L5929, `connector.guid`) | PARITY |
| 2 | **Name** (disabled, conditional) | NO (old had "Family") | YES (L5934, `connector.name`) | NEW in new build |
| 3 | **t** (disabled) | NO | YES (L5940, `connector.t.toFixed(4)`) | NEW in new build |
| 4 | **Description** (disabled Textarea) | YES | YES (L5944-5949, conditional) | PARITY |
| 5 | **Port** (disabled) | YES (as "Family") | YES (L5953, `connector.port.guid`) | PARITY (renamed) |
| 6 | **Mandatory** (Yes/No) | YES | YES (L5959, conditional) | PARITY |
| 7 | **Position** (x,y,z) | YES | YES (L5965, formatted tuple) | PARITY |
| 8 | **Direction** (x,y,z) | YES | YES (L5969, formatted tuple) | PARITY |
| 9 | **Compatible Ports** | YES (as "Compatible Families") | YES (L5973-5979, `compatiblePorts`) | PARITY (renamed) |
| 10 | **Attributes** | YES | YES (L5980-5987) | PARITY |

### DESIGN Section (DesignSectionForm)

| # | Feature | Old Build | New Build | Status |
|---|---------|-----------|----------|--------|
| 1 | **Piece Count** | NO | YES (L4572) | NEW in new build |
| 2 | **Connection Count** | NO | YES (L4584) | NEW in new build |

### OVERALL VERDICT

**MISSING from new build vs old: NONE**

All old build features are present in the new build. The new build additionally has:
- Piece: name, description, scale, color, attributes, plane xAxis/yAxis editing
- Connection: description (single + bulk)
- Connector: name, t parameter
- Design: pieceCount, connectionCount metadata
- Improved: Connection fields grouped into Plane>Translation, Plane>Orientation, Diagram sections
