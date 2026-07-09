---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Successfully added missing selection tools (Hand, Lasso) to Type app and reorganized toolbar to match Design app pattern. All tools now properly registered and translations added for English and German locales.

## Changes

### Type.tsx

- Added HandTool, LassoRectangularTool, and LassoFreeformTool tool definitions
- Updated TypeSelectSettings to match Design app pattern (removed Normal mode, focused on Additive/Subtractive/Intersect toggles)
- Added TypeHandSettings component that activates hand tool on mount
- Added TypeLassoSettings component with rectangular/freeform toggle group
- Updated TypeConnectorSettings to use consistent toggle pattern
- Updated TypeAppTools array to include all tools: SelectionNormalTool, SelectionAdditiveTool, SelectionSubtractiveTool, SelectionIntersectTool, LassoRectangularTool, LassoFreeformTool, HandTool, ConnectorTool
- Registered toolbar sections for selection, lasso, hand, and connector tools with proper order and specificity
- Added DiagramIcon and SceneIcon imports for lasso tools

### locales/en.json

- Added "lasso" to toolbar.parent section
- Added "lasso" to toolbar.subtool section
- Updated app.type.tools.select translations (changed beginner labels from "Create ..." to selection-focused)
- Added app.type.tools.lasso section with rectangular and freeform translations
- Added app.type.tools.hand translation
- Added app.type.tools.connector translation

### locales/de.json

- Added "lasso" to toolbar.parent section with German translation "Lasso"
- Added "lasso" to toolbar.subtool section with German translation "Lasso"
- Updated app.type.tools.select translations (changed from "erstellen" to selection-focused)
- Added app.type.tools.lasso section with German translations (Rechteckig, Freihand)
- Added app.type.tools.hand translation with German translation
- Added app.type.tools.connector translation with German translation

## Log

- Analyzed Design.tsx to understand selection tool implementation pattern
- Analyzed Type.tsx to identify missing functionality
- Identified that Type app was missing Hand and Lasso tools
- Implemented missing tools following Design app architecture
- Added all tool definitions (Hand, LassoRectangular, LassoFreeform)
- Created TypeHandSettings and TypeLassoSettings components
- Updated TypeSelectSettings to remove Normal toggle (now toggled implicitly when others are off)
- Updated TypeConnectorSettings for consistency
- Registered all toolbar sections with proper order: selection (order 10), lasso (order 20), hand (order 30), connector (order 40)
- Added missing icon imports (DiagramIcon, SceneIcon)
- Updated English and German locale files with all missing translations
- Verified no TypeScript compilation errors
- Verified unit tests pass (12/12 tests passing)

## Todos

- [x] Analyze Design.tsx selection implementation
- [x] Analyze Type.tsx selection implementation
- [x] Identify missing selection functionality
- [x] Implement Hand tool in Type app
- [x] Implement Lasso tools in Type app
- [x] Reorganize toolbar settings with dividers
- [x] Test selection functionality
- [x] Close ticket

## Plan

### Completed Implementation

1. **Added missing tools to Type.tsx:**
   - ✅ Added HandTool definition (similar to Design app)
   - ✅ Added LassoRectangularTool definition
   - ✅ Added LassoFreeformTool definition
   - ✅ Updated TypeAppTools array to include all tools

2. **Created settings components:**
   - ✅ Created TypeHandSettings component (similar to DesignHandSettings)
   - ✅ Created TypeLassoSettings component (similar to DesignLassoSettings)

3. **Reorganized TypeSelectSettings:**
   - ✅ Updated TypeSelectSettings to show only selection mode toggles (Additive, Subtractive, Intersect)
   - ✅ Removed Normal toggle (now implicit when others are off, similar to Design app)
   - ✅ Added useEffect to reset from Hand tool to Normal selection

4. **Registered toolbar sections:**
   - ✅ Registered selection tool section (order 10)
   - ✅ Registered lasso tool section (order 20)
   - ✅ Registered hand tool section (order 30)
   - ✅ Registered connector tool section (order 40)
   - ✅ Ensured proper cleanup in return function

5. **Added translations:**
   - ✅ Added missing translations for hand and lasso tools in English locale
   - ✅ Added missing translations for hand and lasso tools in German locale
   - ✅ Added toolbar.parent.lasso and toolbar.subtool.lasso entries
   - ✅ Updated type app tool translations to be consistent
