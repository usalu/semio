# Fix Action-Filter Synchronization Bug and Artifact Creation System

## Problem Statement

The Kit app's toolbar has two critical bugs that affect the user experience when managing kit artifacts:

### Bug #1: Filter-Action Desynchronization
When a user clicks the **Add** action button (e.g., "Add Type", "Add Design") connected to a filter toggle, the filter toggle is automatically deactivated. This breaks the expected behavior where:
- Creating a new artifact should not hide the artifact kind's table view
- The filter state should remain unchanged when using the "Add" action
- Users expect the newly created artifact to be immediately visible in the table

**Current Impact**: After creating a new type or design, the filter deactivates and the table hides that artifact kind, making it invisible until the user manually reactivates the filter.

### Bug #2: Limited Artifact Creation Support
Only `designs` and `types` have functional `Add` actions. Other artifact kinds (`authors`, `folders`, `qualities`, `ports`, `tags`, `concepts`, `files`) are defined in the UI but their creation handlers are incomplete or missing.

**Current Impact**: Users cannot create new artifacts for most kinds through the toolbar, forcing them to find alternative (non-existent) creation methods.

## Root Cause Analysis

### Issue 1: Toggle State Management in `handleCreateArtifact`
In `Kit.tsx`, the `KitFilterBar` component's `handleCreateArtifact` function currently only implements cases for `"designs"` and `"types"`. When clicking the add action, the default case does nothing, but the toggle's `onPressedChange` still fires, deactivating the filter.

**Location**: `js/semio/sketchpad/Kit.tsx` - `KitFilterBar` component, lines ~3465-3500

**Problem Pattern**:
```typescript
const handleCreateArtifact = (kind: ArtifactKind) => {
  if (!kit || !kitCommands) return;
  switch (kind) {
    case "designs": { /* implementation */ break; }
    case "types": { /* implementation */ break; }
    default:
      break; // <-- Incomplete implementations
  }
};
```

### Issue 2: Click Handler Propagation
The `Toggle` component's `onActionClick` triggers independent of `onPressedChange`, but there's no event propagation control. The filter toggle is receiving change events when it shouldn't.

**Solution Approach**: Ensure `onActionClick` fires WITHOUT triggering the toggle's pressed state change.

### Issue 3: Missing Artifact Creation Commands
The `kitCommands` object (from `KitDiffAppStore`) lacks command implementations for creating:
- Folders
- Authors  
- Qualities
- Ports
- Tags
- Concepts
- Files

**Location**: `js/semio/sketchpad/Kit.tsx` - Command registration region and `kitCommands` object

## Acceptance Criteria

### For Filter-Action Synchronization
- ✅ Clicking the "Add" action button does NOT change the filter toggle's pressed state
- ✅ After creating an artifact, the filter remains in its previous state (on or off)
- ✅ If a filter is OFF and user clicks "Add", the filter stays OFF (newly created artifact is not shown until user manually toggles)
- ✅ If a filter is ON and user clicks "Add", the filter stays ON and the new artifact appears immediately

### For Artifact Creation System
- ✅ All 9 artifact kinds can be created: designs, types, qualities, ports, tags, concepts, files, folders, authors
- ✅ Each creation action generates unique default names (e.g., "New Design", "New Folder", "New Author")
- ✅ Newly created artifacts are added to the kit store and visible in tables
- ✅ Creation actions navigate appropriately (designs/types navigate to their editors; metadata items stay in kit view)
- ✅ All creation handlers follow the same pattern: validate inputs → generate unique name → create entity → update store → navigate (if applicable)

## Technical Implementation Requirements

### 1. Decouple Toggle Pressed Change from Action Click
In `KitFilterBar` component:
- Modify `Toggle` component integration to ensure `onActionClick` callback does NOT propagate to `onPressedChange`
- The `onActionClick` should be self-contained and not affect filter state
- Example: Use `event.stopPropagation()` if needed, or separate the handlers completely

### 2. Implement Complete `handleCreateArtifact` Function
Extend the switch statement to handle all 9 artifact kinds:
- **designs**: Navigate to design editor (✅ done)
- **types**: Navigate to type editor (✅ done)
- **qualities**: Create quality, set active filter to "qualities", select the new quality in table
- **ports**: Create port, set active filter to "ports", select the new port in table
- **tags**: Create tag, set active filter to "tags", select the new tag in table
- **concepts**: Create concept, set active filter to "concepts", select the new concept in table
- **files**: Create file, set active filter to "files", select the new file in table
- **folders**: Create folder, set active filter to "folders", select the new folder in table
- **authors**: Create author, set active filter to "authors", select the new author in table

### 3. Create Artifact Creation Commands in `kitCommands`
In the Commands region of `Kit.tsx`, add handlers for each missing artifact type:
- `createFolder(folder: Folder): void`
- `createAuthor(author: Author): void`
- `createQuality(quality: Quality): void`
- `createPort(port: Port): void`
- `createTag(tag: Tag): void`
- `createConcept(concept: Concept): void`
- `createFile(file: SemioFile): void`

Each command should:
1. Validate the kit exists and is writable
2. Generate a `KitDiff` with the new entity added
3. Apply the diff through the store
4. Trigger necessary UI updates (selection, filter activation, etc.)

### 4. Unique Name Generation
Use existing `generateUniqueName()` utility for all artifact types:
- Default names follow pattern: `defaultXName || "New {Type}"`
- Scope for uniqueness:
  - **Global scope**: Qualities, Ports, Files, Tags, Concepts, Authors
  - **Sibling scope**: Folders (unique among siblings), Designs/Types (unique among siblings)

### 5. Post-Creation Behavior

**For editing artifacts (designs, types):**
- Navigate away from kit view to editor
- No need to maintain filter or selection state

**For metadata artifacts (all others):**
- Keep user in kit view
- Set the created artifact's kind filter to active (e.g., if creating a folder, ensure "folders" filter is ON)
- Auto-select the newly created artifact in the table
- Optionally scroll to reveal the new artifact

## Implementation Scope

### Files to Modify
1. **js/semio/sketchpad/Kit.tsx**
   - `KitFilterBar` component: Fix toggle-action click handling
   - `handleCreateArtifact` function: Implement all 9 cases
   - Commands region: Add missing artifact creation commands
   - Selection/Filter logic: Ensure created artifacts are properly selected and filtered

### Files to Reference
1. **js/semio/semio.ts** - Model definitions for all artifact types
2. **js/semio/sketchpad/Sketchpad.tsx** - Store transaction patterns and command registration
3. **js/semio/sketchpad/Design.tsx** - Example of similar create patterns
4. **js/semio/elements.tsx** - Toggle component behavior

## Testing Strategy

### Unit Tests
- Verify each `kitCommands.createX()` produces valid diffs
- Verify unique name generation for all artifact types
- Verify filter state persists across creation actions

### Integration Tests
- Click each of 9 "Add" buttons and verify:
  - No toggle state change occurs
  - New artifact appears in kit store
  - Proper navigation happens (types/designs) or table updates (others)
  - Selection reflects newly created artifact

### Manual Testing
- Create artifacts with all 9 kinds
- Verify filter behavior before/after
- Verify default names and uniqueness
- Test with filters both ON and OFF
- Verify large kit performance (100+ artifacts per kind)

## Success Criteria Summary

✅ All 9 artifact kinds can be created from toolbar  
✅ Filter toggles maintain state when "Add" action is clicked  
✅ Newly created artifacts follow consistent behavior  
✅ No console errors or warnings  
✅ E2E tests pass for all creation paths  
✅ Filters don't auto-deactivate after any creation  

## Constraints

- No breaking changes to existing models or APIs
- Must maintain backward compatibility with current kits
- Should follow existing command and store patterns
- Must work across all supported browsers
- Performance should remain acceptable with large kits
