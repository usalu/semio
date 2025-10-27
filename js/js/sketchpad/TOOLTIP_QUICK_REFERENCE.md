# Quick Reference: Semio Tooltip System

## Import
```typescript
import { SemioTooltipWrapper, TooltipConfig } from "@semio/js";
```

## Basic Usage
```tsx
<SemioTooltipWrapper config={{ labelKey: "navbar.home" }}>
  <Button>Home</Button>
</SemioTooltipWrapper>
```

## Full Configuration
```tsx
<SemioTooltipWrapper
  config={{
    labelKey: "navbar.home",
    manualPath: "/docs/manuals/sketchpad#navigation",
    tutorialPath: "/docs/tutorials/hello-semio",
    hotkey: "⌘H"
  }}
>
  <Button>Home</Button>
</SemioTooltipWrapper>
```

## App Config
```typescript
export const config: AppConfig = {
  getPanels: (t) => [
    {
      key: "workbench",
      icon: BookOpen,
      tooltip: {
        labelKey: "panels.workbench",
        manualPath: "/docs/manuals/sketchpad#workbench",
        tutorialPath: "/docs/tutorials/hello-semio",
        hotkey: "⌘1"
      },
      hotkey: "⌘1"
    }
  ]
};
```

## Translation Pattern
```json
{
  "navbar": {
    "home": "Go to home",
    "home.extensive": "Click to go to home. Shows all kits organized by type."
  }
}
```

## User Modes

| Mode      | Behavior                                    |
|-----------|---------------------------------------------|
| Expert    | No tooltips shown                           |
| Normal    | Basic tooltip + manual link                 |
| Beginner  | Extensive tooltip + manual + tutorial links |

## Config Properties

| Property      | Type     | Required | Description                    |
|---------------|----------|----------|--------------------------------|
| labelKey      | string   | No       | i18n translation key           |
| manualPath    | string   | No       | Path to manual page            |
| tutorialPath  | string   | No       | Path to tutorial page          |
| hotkey        | string   | No       | Hotkey display string          |

## Path Conventions

**Manuals:**
- `/docs/manuals/{manual-name}`
- `/docs/manuals/{manual-name}#{section}`

**Tutorials:**
- `/docs/tutorials/{tutorial-name}`
- `/docs/tutorials/{tutorial-name}/{step}`

## Backward Compatibility

Old string tooltips still work:
```typescript
tooltip: t("panels.workbench")  // ✅ Still works
```

New config object:
```typescript
tooltip: { labelKey: "panels.workbench" }  // ✅ Enhanced
```
