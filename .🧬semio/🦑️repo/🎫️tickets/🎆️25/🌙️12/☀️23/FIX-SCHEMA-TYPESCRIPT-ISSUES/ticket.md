# Ticket

## Todos

# Previously

Schema validation reported 21 errors for missing C# entities. TypeScript compilation reported multiple errors including jsDoc property access, Sketchpad JSX component type, storybook stories with missing args and invalid level props.

# Plan

1. Fix schema.tsx C# parser regex - DONE
2. Verify schema.json shows 0 errors - DONE (21 warnings remain for missing Grasshopper components)
3. Fix code.tsx jsDoc property access - DONE
4. Fix Sketchpad.tsx JSX component type - DONE
5. Fix all storybook stories with missing args and invalid level props - DONE
6. Fix Design.tsx function signature type mismatches - DONE
7. Fix Design.tsx connector vs connectors and coordinate issues - DONE
8. Fix scripts/log.tsx marginTop prop error - DONE
9. Remaining: Systemic issues requiring npm install and dependency updates - BLOCKED

# Changes

## Schema Fixes

- `scripts/schema.tsx`: Changed C# entity regex from `[Model(...)]` to `[Entity(...)]` to match actual C# code

## TypeScript Fixes

- `hooks/code.tsx`: Replaced deprecated `jsDoc` property access with `ts.getJSDocCommentsAndTags(node)`
- `js/compose/sketchpad/Sketchpad.tsx`: Changed return type from `React.ReactElement` to `JSX.Element`
- `js/compose/sketchpad/Design.tsx`:
  - Fixed function signatures to match DesignAppHooks interface (made pieceId/connectionId optional, added guards)
  - Fixed GranularSelectorFactory to allow undefined return type
  - Added `connector` property back to DesignAppSelection for backwards compatibility
  - Fixed x,y to u,v coordinate transformation in useDesignAppDiagramCenter
- All storybook stories: Removed invalid `level` props, added required `args` properties, wrapped level-specific renders with `LevelProvider`
- `scripts/log.tsx`: Fixed marginTop prop by wrapping Text in Box

## Remaining Problems (Require External Actions)

### Requires `npm install` in js/compose directory:

- Three.js JSX elements missing from JSX.IntrinsicElements (`group`, `mesh`, `primitive`, `ambientLight`, etc.)

### Requires React/dependency updates:

- React FC type issues with `bigint` not assignable to `ReactNode` (React 18/19 type definition conflict)
- ReactI18NextChildren type compatibility issues

### Requires port/code refactoring:

- Missing properties on state types (`others` on DesignAppState, `windowLayout` on KitAppState)
- Undefined variable references (`designAppModuleCache`, `homeAppModuleCache`, etc.)
- SidePanelProps missing `position` property
- PanelSections missing `leftSidePanel`, `rightSidePanel`, `hudPanel` properties
- Type.tsx camera type mismatch (`target` vs `forward`/`up`)

## Changes

## Log

## Summary

# Summary
