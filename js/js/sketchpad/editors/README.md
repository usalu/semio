# Editor Registry System

This directory contains all editors for the Sketchpad application. The architecture follows the **Open/Closed Principle**: the core system is closed for modification but open for extension.

## Architecture Overview

### Registry Pattern

All editors self-register with a central registry (`registry.tsx`). The core components (Navbar, Sketchpad, routes) read from this registry instead of hardcoding editor references.

**Benefits:**
- Add new editors without modifying core code
- Editors are self-contained with their own routes, panels, and commands
- No merge conflicts in core files when multiple editors are developed
- Easy to enable/disable editors by commenting out their registration

### File Structure

```
editors/
├── registry.tsx              # Central registry
├── index.tsx                 # Auto-imports all registrations
├── README.md                 # This file
├── home/
│   ├── Editor.tsx            # Home editor component
│   ├── registration.tsx      # Self-registration
│   ├── store.tsx            # Editor-specific store
│   └── commands.ts          # Editor-specific commands
├── kit/
│   ├── Editor.tsx
│   ├── registration.tsx
│   ├── store.tsx
│   └── commands.ts
└── [other editors...]
```

## Adding a New Editor

### 1. Create Editor Directory

```
editors/
└── my-editor/
    ├── Editor.tsx
    ├── registration.tsx
    ├── store.tsx
    └── commands.ts
```

### 2. Create Editor Component

`Editor.tsx`:
```tsx
import { FC } from "react";

const MyEditor: FC = () => {
  return <div>My Editor Content</div>;
};

export default MyEditor;
```

### 3. Create Registration File

`registration.tsx`:
```tsx
import { Info, MessageCircle, Settings } from "lucide-react";
import { KitScopeProvider } from "../../kits/store";
import { editorRegistry } from "../registry";
import MyEditor from "./Editor";

editorRegistry.register({
  id: "my-editor",
  component: MyEditor,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
    {
      path: "my-items/:myItem",
      paramName: "myItem",
      scopeProvider: MyItemScopeProvider,
    },
  ],
  getPanels: (t) => [
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  matchesPath: (pathParts) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return (
      pathParts.length === 4 &&
      pathParts[0] === "kits" &&
      isUuidPattern(pathParts[1]) &&
      pathParts[2] === "my-items" &&
      isUuidPattern(pathParts[3])
    );
  },
  order: 50,
});
```

### 4. Register in Index

Add your registration import to `editors/index.tsx`:
```tsx
import "./home/registration";
import "./kit/registration";
import "./design/registration";
import "./type/registration";
import "./quality/registration";
import "./my-editor/registration"; // Add this line
```

### 5. Done!

Your editor is now fully integrated:
- Routes are automatically generated
- Panels appear in the navbar
- Navigation breadcrumbs work
- No modifications to core files needed

## Registration Options

### EditorRegistration Interface

```typescript
interface EditorRegistration {
  id: string;                    // Unique identifier (matches EditorType enum)
  component: ComponentType;      // The editor React component
  routeSegments: RouteSegment[]; // Route path segments
  getPanels: (t) => PanelDefinition[]; // Panel configuration
  matchesPath?: (pathParts) => boolean; // URL path matching logic
  order?: number;                // Display order (default 0)
}
```

### RouteSegment

```typescript
interface RouteSegment {
  path: string;              // Route path (e.g., "kits/:kit")
  paramName?: string;        // URL parameter name (e.g., "kit")
  scopeProvider?: ComponentType<{ guid: string; children: ReactNode }>; // Scope provider component
}
```

### PanelDefinition

```typescript
interface PanelDefinition {
  key: string;              // Panel key (e.g., "details", "chat")
  icon: ComponentType<{ size?: number }>; // Lucide icon component
  tooltip: string;          // Tooltip text (i18n key)
  hotkey: string;          // Keyboard shortcut
}
```

## Route Segments Explained

Route segments define the URL structure for your editor. They are nested in order:

**Example:** Design editor route
```
/kits/:kit/designs/:design
```

Segments:
1. `{ path: "kits/:kit", paramName: "kit", scopeProvider: KitScopeProvider }`
2. `{ path: "designs/:design", paramName: "design", scopeProvider: DesignScopeProvider }`

The registry automatically:
- Generates nested routes
- Wraps each level with its scope provider
- Passes the parameter value as `guid` prop to the scope provider

## Panel Configuration

Panels define which UI panels are available for your editor. The navbar automatically:
- Groups panels (workbench/tools, hud/stats, details/chat/settings)
- Creates toggle buttons with icons and hotkeys
- Shows/hides panels based on user interaction

Common panels:
- `workbench`: Left panel for browsing/selecting items
- `tools`: Left panel for tool options
- `toolbar`: Floating toolbar (e.g., drawing tools)
- `hud`: Center-left overlay
- `stats`: Center-right overlay
- `details`: Right panel for item details
- `chat`: Right panel for AI chat
- `settings`: Right panel for settings

## Path Matching

The `matchesPath` function determines which editor should handle a given URL path.

**Input:** Array of path segments (e.g., `["kits", "abc-123", "designs", "def-456"]`)

**Return:** `true` if this editor should handle the path

**Example:**
```tsx
matchesPath: (pathParts) => {
  const isUuid = (s: string) => /^[0-9a-f-]{36}$/i.test(s);
  return (
    pathParts.length === 4 &&
    pathParts[0] === "kits" &&
    isUuid(pathParts[1]) &&
    pathParts[2] === "designs" &&
    isUuid(pathParts[3])
  );
}
```

## EditorType Enum

The `EditorType` enum still exists in `store.tsx` for backward compatibility. Your editor's `id` should match the enum value (lowercase).

To add a new editor type:
1. Add to `EditorType` enum in `store.tsx`
2. Use the same ID in your registration

## Migration Notes

### Before (Hardcoded)

Core files had hardcoded references:
- `Navbar.tsx`: `getPanelConfigs()` function with all editor panels
- `Sketchpad.tsx`: Hardcoded `<Route>` components for each editor
- `store.tsx`: `EditorType` enum

Adding an editor required modifying all three files.

### After (Registry-Based)

Core files read from registry:
- `Navbar.tsx`: `editorRegistry.getPanelConfigs(t)`
- `Sketchpad.tsx`: `<RouteGenerator />` component
- `store.tsx`: `EditorType` enum (for backward compatibility only)

Adding an editor only requires creating the editor directory and registration file.

## Advanced Topics

### Custom Scope Providers

If your editor needs custom context/state management, create a scope provider:

```tsx
interface MyItemScopeProviderProps {
  guid: string;
  children: ReactNode;
}

export const MyItemScopeProvider: FC<MyItemScopeProviderProps> = ({ guid, children }) => {
  const myItemStore = useMyItemStore(guid);
  return (
    <MyItemContext.Provider value={myItemStore}>
      {children}
    </MyItemContext.Provider>
  );
};
```

### Panel Sections

Editors can dynamically add content to panels using the panel section system:

```tsx
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";

useEffect(() => {
  addSection("details", {
    id: "my-section",
    label: "My Section",
    order: 10,
    content: () => <div>Section content</div>,
  });
  
  return () => removeSection("details", "my-section");
}, []);
```

### Editor Commands

Editors should expose commands for common actions:

```tsx
// In store.tsx
export const useMyEditorCommands = () => {
  const editor = useMyEditor();
  
  return {
    createItem: (item: MyItem) => editor.execute("my.editor.createItem", item),
    deleteItem: (id: string) => editor.execute("my.editor.deleteItem", id),
    togglePanel: (panel: keyof PanelVisibility) => editor.change({ panelVisibility: { [panel]: !editor.snapshot().panelVisibility?.[panel] } }),
  };
};
```

## Examples

See existing editors:
- **Simple:** `home/` - No route segments, basic panels
- **Nested:** `kit/` - Single route segment with scope provider
- **Complex:** `design/` - Multiple route segments, many panels, custom tools
