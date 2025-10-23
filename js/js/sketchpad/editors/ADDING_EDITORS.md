# Quick Start: Adding a New Editor

This guide shows you how to add a new editor in **3 simple steps** without modifying core code.

## The 3-Step Process

### Step 1: Create Editor Files

Create a new directory in `editors/` with these files:

```
editors/
└── my-editor/
    ├── Editor.tsx          # Your editor component
    ├── registration.tsx    # Self-registration (required)
    ├── store.tsx          # Editor state (optional)
    └── commands.ts        # Editor commands (optional)
```

### Step 2: Implement Your Editor

**Editor.tsx** (minimal example):
```tsx
import { FC } from "react";

const MyEditor: FC = () => {
  return (
    <div className="h-full w-full p-4">
      <h1>My Editor</h1>
      <p>Your editor content here</p>
    </div>
  );
};

export default MyEditor;
```

**registration.tsx** (required):
```tsx
import { Info, MessageCircle, Settings } from "lucide-react";
import { editorRegistry } from "../registry";
import MyEditor from "./Editor";

editorRegistry.register({
  id: "my-editor",
  component: MyEditor,
  
  // Define your routes (empty array for root route)
  routeSegments: [],
  
  // Define available panels
  getPanels: (t) => [
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  
  // Define when this editor should be used
  matchesPath: (pathParts) => {
    return pathParts.length === 1 && pathParts[0] === "my-editor";
  },
  
  order: 100, // Display order
});
```

### Step 3: Register Import

Open `editors/index.tsx` and add one line:

```tsx
import "./home/registration";
import "./kit/registration";
import "./design/registration";
import "./type/registration";
import "./quality/registration";
import "./my-editor/registration"; // ← Add this line

export { editorRegistry } from "./registry";
```

**Done!** Your editor is now available at `/my-editor` 🎉

## Common Patterns

### Nested Routes (with Scope Provider)

If your editor needs nested routes (e.g., `/items/:itemId`):

```tsx
// Create a scope provider
export const MyItemScopeProvider: FC<{ guid: string; children: ReactNode }> = ({ guid, children }) => {
  const itemStore = useMyItemStore(guid);
  return (
    <MyItemContext.Provider value={itemStore}>
      {children}
    </MyItemContext.Provider>
  );
};

// Use in registration
editorRegistry.register({
  id: "my-editor",
  component: MyEditor,
  routeSegments: [
    {
      path: "items/:item",
      paramName: "item",
      scopeProvider: MyItemScopeProvider,
    },
  ],
  // ... rest of config
});
```

URL: `/items/abc-123` → Editor receives item with guid `abc-123`

### Multiple Route Levels

For deeply nested routes (e.g., `/kits/:kit/items/:item`):

```tsx
routeSegments: [
  {
    path: "kits/:kit",
    paramName: "kit",
    scopeProvider: KitScopeProvider,
  },
  {
    path: "items/:item",
    paramName: "item",
    scopeProvider: MyItemScopeProvider,
  },
]
```

URL: `/kits/xyz/items/abc` → Nested scopes for kit and item

### Path Matching with UUID Validation

```tsx
matchesPath: (pathParts) => {
  const isUuid = (s: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(s);
  
  return (
    pathParts.length === 2 &&
    pathParts[0] === "items" &&
    isUuid(pathParts[1])
  );
}
```

Matches: `/items/abc-123-def-456` (with UUID validation)

### Panel Configuration

Available panel keys:
- `workbench` - Left panel (browsing/selection)
- `tools` - Left panel (tool options)
- `toolbar` - Floating toolbar
- `hud` - Center-left overlay
- `stats` - Center-right overlay
- `details` - Right panel (details)
- `chat` - Right panel (AI chat)
- `settings` - Right panel (settings)

Example with many panels:
```tsx
getPanels: (t) => [
  { key: "workbench", icon: Box, tooltip: t("panels.workbench"), hotkey: "⌘J" },
  { key: "tools", icon: Wrench, tooltip: t("panels.tools"), hotkey: "⌘U" },
  { key: "toolbar", icon: Hammer, tooltip: t("panels.toolbar"), hotkey: "⌘K" },
  { key: "hud", icon: Layers, tooltip: t("panels.hud"), hotkey: "⌘H" },
  { key: "stats", icon: BarChart3, tooltip: t("panels.stats"), hotkey: "⌘I" },
  { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
  { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
  { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
]
```

### Dynamic Panel Content

Add content to panels dynamically:

```tsx
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";

const MyEditor: FC = () => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  
  useEffect(() => {
    // Add content to details panel
    addSection("details", {
      id: "my-custom-section",
      label: "My Section",
      order: 10,
      defaultOpen: true,
      content: () => (
        <div>
          <h3>Section Title</h3>
          <p>Section content</p>
        </div>
      ),
    });
    
    // Cleanup on unmount
    return () => removeSection("details", "my-custom-section");
  }, [addSection, removeSection]);
  
  return <div>Editor content</div>;
};
```

## Templates

### Minimal Editor Template

```tsx
// Editor.tsx
import { FC } from "react";

const MyEditor: FC = () => {
  return <div className="h-full w-full">My Editor</div>;
};

export default MyEditor;

// registration.tsx
import { Settings } from "lucide-react";
import { editorRegistry } from "../registry";
import MyEditor from "./Editor";

editorRegistry.register({
  id: "my-editor",
  component: MyEditor,
  routeSegments: [],
  getPanels: (t) => [
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  matchesPath: (pathParts) => pathParts.length === 1 && pathParts[0] === "my-editor",
  order: 100,
});
```

### Full-Featured Editor Template

```tsx
// Editor.tsx
import { FC, useEffect } from "react";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { useMyEditor, useMyEditorCommands } from "./store";

const MyEditor: FC = () => {
  const editor = useMyEditor();
  const commands = useMyEditorCommands();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  
  useEffect(() => {
    addSection("details", {
      id: "my-details",
      label: "Details",
      order: 0,
      content: () => <div>Details content</div>,
    });
    
    return () => removeSection("details", "my-details");
  }, [addSection, removeSection]);
  
  return (
    <div className="h-full w-full">
      <h1>My Editor</h1>
      {/* Your editor UI */}
    </div>
  );
};

export default MyEditor;

// registration.tsx
import { Box, Info, MessageCircle, Settings } from "lucide-react";
import { KitScopeProvider } from "../../kits/store";
import { editorRegistry } from "../registry";
import MyEditor from "./Editor";
import { MyItemScopeProvider } from "./store";

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
    { key: "workbench", icon: Box, tooltip: t("panels.workbench"), hotkey: "⌘J" },
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  matchesPath: (pathParts) => {
    const isUuid = (s: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(s);
    return (
      pathParts.length === 4 &&
      pathParts[0] === "kits" &&
      isUuid(pathParts[1]) &&
      pathParts[2] === "my-items" &&
      isUuid(pathParts[3])
    );
  },
  order: 50,
});

// store.tsx
import { createContext, FC, ReactNode, useContext } from "react";
import { EditorStore } from "../../store";

interface MyEditorState {
  // Your state here
}

export const MyItemScopeProvider: FC<{ guid: string; children: ReactNode }> = ({ guid, children }) => {
  // Your scope provider logic
  return <>{children}</>;
};

export const useMyEditor = () => {
  // Your editor hook
};

export const useMyEditorCommands = () => {
  // Your commands hook
};
```

## Checklist

When adding a new editor, ensure:

- [ ] Created editor directory in `editors/`
- [ ] Implemented `Editor.tsx` component
- [ ] Created `registration.tsx` with proper config
- [ ] Added import to `editors/index.tsx`
- [ ] Defined route segments (if needed)
- [ ] Defined panel configuration
- [ ] Implemented path matching logic
- [ ] Created scope provider (if needed)
- [ ] Tested navigation to editor
- [ ] Tested panel visibility toggles
- [ ] Tested keyboard shortcuts

## Troubleshooting

**Editor not showing up?**
- Check import is added to `editors/index.tsx`
- Verify `matchesPath()` logic is correct
- Check browser console for errors

**Route not working?**
- Verify `routeSegments` path matches URL
- Check `paramName` matches route parameter
- Ensure scope provider is correct

**Panels not appearing?**
- Check panel keys are valid
- Verify icon imports from `lucide-react`
- Check translation keys exist

## Resources

- **Full Documentation:** `editors/README.md`
- **Architecture:** `../ARCHITECTURE.md`
- **Migration Guide:** `../MIGRATION.md`
- **Examples:** See existing editors (home, kit, design, type, quality)

## Need Help?

- Check existing editor implementations for reference
- Read the detailed README.md in this directory
- Review the architecture documentation
- Look at the migration guide for examples
