# Example: Adding a New Editor to Semio Sketchpad

This example demonstrates how the Open-Closed Principle makes it trivial to add new editors to the Semio Sketchpad application.

## Scenario

We want to add a new "Analytics" editor that shows statistics and visualizations for a design or kit.

## Steps

### 1. Create the Editor Folder

```
js/js/sketchpad/editors/analytics/
```

### 2. Create `config.ts`

```typescript
// js/js/sketchpad/editors/analytics/config.ts

import { BarChart, Info, MessageCircle, Settings } from "lucide-react";
import { EditorConfig } from "../registry";
import { KitScopeProvider, DesignScopeProvider } from "../../kits/store";
import AnalyticsEditor from "./Editor";

export const config: EditorConfig = {
  id: "analytics",
  component: AnalyticsEditor,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
    {
      path: "designs/:design",
      paramName: "design",
      scopeProvider: DesignScopeProvider,
    },
    {
      path: "analytics",
    },
  ],
  getPanels: (t) => [
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  matchesPath: (pathParts) => {
    const isUuidPattern = (str: string) => 
      /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return (
      pathParts.length === 5 &&
      pathParts[0] === "kits" &&
      isUuidPattern(pathParts[1]) &&
      pathParts[2] === "designs" &&
      isUuidPattern(pathParts[3]) &&
      pathParts[4] === "analytics"
    );
  },
  order: 25,
};
```

### 3. Create `Editor.tsx`

```typescript
// js/js/sketchpad/editors/analytics/Editor.tsx

import { FC, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Canvas, Window } from "../../Canvas";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { useEditorType } from "../../store";
import AnalyticsCanvas from "./canvas/Analytics";
import AnalyticsDetails from "./panels/Details";

const AnalyticsEditor: FC = () => {
  const { t } = useTranslation();
  const editorType = useEditorType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useEffect(() => {
    if (editorType !== "analytics") return;

    addSection("details", {
      id: "analytics-details",
      label: t("analytics.details", "Analytics Details"),
      order: 1,
      defaultOpen: true,
      content: () => <AnalyticsDetails />,
    });

    return () => {
      removeSection("details", "analytics-details");
    };
  }, [editorType, addSection, removeSection, t]);

  return (
    <Canvas>
      <Window id="analytics">
        <AnalyticsCanvas />
      </Window>
    </Canvas>
  );
};

export default AnalyticsEditor;
```

### 4. Create `store.tsx`

```typescript
// js/js/sketchpad/editors/analytics/store.tsx

import { useEffect, useState } from "react";
import { EditorStore } from "../../store";

export interface AnalyticsSelection {
  selectedMetrics: string[];
}

export interface AnalyticsState {
  metrics: Record<string, number>;
  selection: AnalyticsSelection;
}

export interface AnalyticsSelectionDiff {
  selectedMetrics?: string[];
}

export interface AnalyticsDiff {
  metrics?: Partial<Record<string, number>>;
  selection?: AnalyticsSelectionDiff;
}

export interface AnalyticsEdit {
  do: AnalyticsSelectionDiff;
  undo: AnalyticsSelectionDiff;
}

class AnalyticsEditorStore extends EditorStore<
  AnalyticsState,
  AnalyticsDiff,
  AnalyticsSelectionDiff,
  AnalyticsEdit,
  any,
  any
> {
  constructor(sketchpad: any, yMap: any) {
    super(sketchpad, yMap);
  }

  protected hash(state: AnalyticsState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): AnalyticsState {
    return {
      metrics: {},
      selection: { selectedMetrics: [] },
    };
  }

  protected applySelectionDiff(diff: AnalyticsSelectionDiff): void {
    // Apply selection changes to Y.js
  }

  protected inverseSelectionDiff(
    selection: AnalyticsSelection,
    diff: AnalyticsSelectionDiff
  ): AnalyticsSelectionDiff {
    return { selectedMetrics: selection.selectedMetrics };
  }

  protected getSelection(): AnalyticsSelection {
    const state = this.get();
    return state.selection;
  }
}

export function useAnalyticsEditor() {
  const [store, setStore] = useState<AnalyticsEditorStore | null>(null);

  useEffect(() => {
    // Initialize store
  }, []);

  return store;
}
```

### 5. Create `commands.ts` (Optional)

```typescript
// js/js/sketchpad/editors/analytics/commands.ts

export const analyticsCommands = {
  selectMetric: "analytics.selectMetric",
  clearSelection: "analytics.clearSelection",
  exportData: "analytics.exportData",
};
```

### 6. Create Canvas Component

```typescript
// js/js/sketchpad/editors/analytics/canvas/Analytics.tsx

import { FC } from "react";

const AnalyticsCanvas: FC = () => {
  return (
    <div className="flex h-full w-full items-center justify-center">
      <div className="text-center">
        <h1 className="text-2xl font-bold">Analytics Dashboard</h1>
        <p className="text-muted-foreground">Visualizations will appear here</p>
      </div>
    </div>
  );
};

export default AnalyticsCanvas;
```

### 7. Create Panel Components

```typescript
// js/js/sketchpad/editors/analytics/panels/Details.tsx

import { FC } from "react";

const AnalyticsDetails: FC = () => {
  return (
    <div className="p-4">
      <h3 className="text-sm font-semibold mb-2">Metrics</h3>
      <ul className="space-y-1 text-xs">
        <li>Total Pieces: 42</li>
        <li>Total Connections: 38</li>
        <li>Components: 3</li>
      </ul>
    </div>
  );
};

export default AnalyticsDetails;
```

## Result

That's it! The new Analytics editor is now:

✅ **Automatically discovered** by the editor registry  
✅ **Accessible** via the route `/kits/:kit/designs/:design/analytics`  
✅ **Integrated** with the panel system  
✅ **Type-safe** - TypeScript checks everything  

## What We Didn't Need to Modify

- ❌ No changes to `editors/index.tsx`
- ❌ No changes to `editors/registry.tsx`
- ❌ No changes to `Sketchpad.tsx`
- ❌ No changes to any other editor
- ❌ No registration files needed

## Adding Tools (Bonus)

To add analytics tools, just create files in `tools_registry/`:

```typescript
// js/js/sketchpad/editors/analytics/tools_registry/SelectionTool.tsx

import { MousePointer2 } from "lucide-react";
import { Tool } from "../../../Tool";
import { ToolType } from "../../../store";
import { AnalyticsState } from "../store";

export const AnalyticsSelectionTool: Tool<AnalyticsState> = {
  id: ToolType.ANALYTICS_SELECTION,
  label: "Select Metrics",
  icon: <MousePointer2 className="h-4 w-4" />,
  tooltip: "Select metrics to analyze",
  render: (context) => ({
    scene: null,
    diagram: null,
    table: <div>Metrics table here</div>,
  }),
};
```

The tool is automatically included - no index.tsx modification needed!

## Summary

The Open-Closed Principle refactoring means:

1. **Create a folder** with your editor name
2. **Add 3-4 files** (config, Editor, store, optionally commands)
3. **Write your logic** - no boilerplate registration code
4. **Done!** The editor appears in the application automatically

This is the power of convention over configuration and the Open-Closed Principle working together!
