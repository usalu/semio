# Open-Closed Principle Refactoring

This document describes the refactoring applied to make the codebase follow the Open-Closed Principle (OCP) - closed for modification, open for extension.

## Changes Made

### 1. Editor Registration

**Before:** Each editor required explicit registration via a `registration.tsx` file that was imported in `editors/index.tsx`.

**After:** Editors are automatically discovered based on file structure. Each editor folder exports a `config.ts` file that defines its configuration.

**To add a new editor:**
1. Create a new folder under `js/js/sketchpad/editors/`
2. Add a `config.ts` file exporting `EditorConfig`
3. Add an `Editor.tsx` file with the editor component
4. Add a `store.tsx` file with the editor state management
5. Done! No changes to existing files needed.

**Example structure:**
```
editors/
  myeditor/
    config.ts          # Export: { config: EditorConfig }
    Editor.tsx         # Default export: FC
    store.tsx          # Editor state and hooks
    commands.ts        # Optional: command definitions
    canvas/            # Optional: canvas components
    panels/            # Optional: panel components
    tools_registry/    # Optional: tools
```

### 2. Tool Registration

**Before:** Tools were explicitly imported and listed in `tools_registry/index.tsx`.

**After:** Tools are automatically discovered using Vite's `import.meta.glob`. Any file matching `*Tool.tsx` pattern is automatically included.

**To add a new tool:**
1. Create a file in `editors/{editor}/tools_registry/` ending with `Tool.tsx`
2. Export tool objects (each tool must have `id` and `render` properties)
3. Done! The tool is automatically registered.

**Example:**
```typescript
// MyTool.tsx
export const MyAwesomeTool: Tool<MyEditorState> = {
  id: ToolType.MY_AWESOME,
  label: "My Tool",
  icon: <Icon />,
  render: (context) => ({ /* ... */ }),
};
```

### 3. Panel Management

**Current:** Panels remain dynamically managed by each editor using the `useAddPanelSection` and `useRemovePanelSection` hooks. This is intentional because:
- Panels are lifecycle-bound to editors
- Panels need access to editor state and context
- Panels are conditionally rendered based on editor state

This pattern is already extensible - just add panel sections in your editor's `useEffect`.

## Benefits

1. **No modification of existing files**: Adding features only requires adding new files
2. **Convention over configuration**: File structure determines functionality
3. **Reduced boilerplate**: No explicit registration code needed
4. **Type safety maintained**: TypeScript enforces correct structure
5. **Better scalability**: Easy to add new editors, tools, and features

## Migration Guide

### For Editors

Old registration files (`registration.tsx`) can be deleted. The configuration has been moved to `config.ts` files.

### For Tools

The `index.tsx` files in `tools_registry` folders have been updated to use auto-discovery. Individual tool files remain unchanged.

### For Panels

No changes needed - panels continue to use the hook-based registration system.

## Technical Details

### Auto-Discovery Mechanism

**Editors:** Uses Vite's `import.meta.glob('./*/config.ts', { eager: true })` to find all config files.

**Tools:** Uses Vite's `import.meta.glob('./*Tool.tsx', { eager: true })` to find all tool files.

### File Naming Conventions

- Editor configs: `config.ts` (must export `config: EditorConfig`)
- Editor components: `Editor.tsx` (must export default FC)
- Editor stores: `store.tsx`
- Tools: `*Tool.tsx` (must export Tool objects)
- Panels: `*.tsx` in `panels/` folder

## Future Extensions

Additional auto-discovery systems that could be added:

1. **Commands**: Auto-discover command definitions
2. **Validation rules**: Auto-discover validation logic
3. **Transformers**: Auto-discover data transformers
4. **Shortcuts**: Auto-discover keyboard shortcuts
5. **Context menu items**: Auto-discover context menu contributions
