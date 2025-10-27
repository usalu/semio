# Semio Tooltip System

## Overview

The Semio tooltip system provides a generalized, mode-aware tooltip component that adapts based on user expertise level. It supports i18n translation keys, links to manuals and tutorials, and hotkey display.

## User Modes

There are three user modes that control tooltip behavior:

- **Expert Mode**: No tooltips displayed
- **Normal Mode**: Shows basic tooltips with manual links
- **Beginner Mode**: Shows extensive tooltips with manual and tutorial links

## TooltipConfig Interface

```typescript
interface TooltipConfig {
  labelKey?: string;        // i18n translation key for the tooltip label
  manualPath?: string;      // Path to relevant manual page (/docs/manuals/...)
  tutorialPath?: string;    // Path to relevant tutorial (/docs/tutorials/...)
  hotkey?: string;          // Hotkey string (e.g., "⌘L", "Ctrl+S")
}
```

All properties are optional:
- If `labelKey` is not provided, no label text is shown
- If `manualPath` is not provided, no manual link is shown
- If `tutorialPath` is not provided, no tutorial link is shown
- If `hotkey` is not provided, no hotkey badge is shown

## Usage

### Basic Usage with SemioTooltipWrapper

```tsx
import { SemioTooltipWrapper } from "@semio/js";

<SemioTooltipWrapper
  config={{
    labelKey: "navbar.home",
    manualPath: "/docs/manuals/sketchpad",
    tutorialPath: "/docs/tutorials/hello-semio",
    hotkey: "⌘H"
  }}
>
  <Button>Home</Button>
</SemioTooltipWrapper>
```

### Translation Keys

The system supports extensive tooltips for beginner mode. For any `labelKey`, you can define:

- `labelKey`: Normal mode text
- `labelKey.extensive`: Beginner mode text (with more detail)

Example in `locales/en.json`:
```json
{
  "navbar": {
    "home": "Go to home",
    "home.extensive": "Click to go to home. This will show all your kits organized by type (temporary, local, remote)."
  },
  "panels": {
    "workbench": "Workbench",
    "workbench.extensive": "The workbench shows your active workspace with tools and options for the current task.",
    "details": "Details",
    "details.extensive": "View and edit detailed properties of the selected item.",
    "settings": "Settings",
    "settings.extensive": "Adjust application preferences and configure your workspace."
  }
}
```

And in `locales/de.json`:
```json
{
  "navbar": {
    "home": "Zur Startseite",
    "home.extensive": "Klicken Sie hier, um zur Startseite zu gelangen. Dort sehen Sie alle Ihre Kits nach Typ organisiert (temporär, lokal, remote)."
  },
  "panels": {
    "workbench": "Arbeitsbereich",
    "workbench.extensive": "Der Arbeitsbereich zeigt Ihren aktiven Workspace mit Werkzeugen und Optionen für die aktuelle Aufgabe.",
    "details": "Details",
    "details.extensive": "Zeigen Sie detaillierte Eigenschaften des ausgewählten Elements an und bearbeiten Sie diese.",
    "settings": "Einstellungen",
    "settings.extensive": "Passen Sie Anwendungseinstellungen an und konfigurieren Sie Ihren Workspace."
  }
}
```

If `labelKey.extensive` is not defined, the system falls back to `labelKey`.

### App Panel Configuration

App configs can now use `TooltipConfig` instead of plain strings:

```typescript
// Old approach (still supported)
export const config: AppConfig = {
  // ...
  getPanels: (t) => [
    { 
      key: "workbench", 
      icon: BookOpen, 
      tooltip: t("panels.workbench"), 
      hotkey: "⌘1" 
    }
  ]
};

// New approach with enhanced tooltips
export const config: AppConfig = {
  // ...
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

Note: The `hotkey` property is still required at the panel level for keyboard shortcut functionality. The `hotkey` in `TooltipConfig` is for display purposes in the tooltip.

### Backward Compatibility

The system maintains backward compatibility. Existing code using string tooltips continues to work:

```tsx
// This still works
<ToggleGroupItem tooltip={tooltip("navbar.home")} hotkey="⌘H">
  <Home />
</ToggleGroupItem>
```

To use enhanced tooltips with ToggleGroupItem and similar components, pass the tooltip as a string (for now) and use the `hotkey` prop:

```tsx
<ToggleGroupItem 
  tooltip={tooltip("navbar.home")}
  hotkey="⌘H"
>
  <Home />
</ToggleGroupItem>
```

For new code that needs manual/tutorial links, use `SemioTooltipWrapper`:

```tsx
<SemioTooltipWrapper
  config={{
    labelKey: "navbar.home",
    manualPath: "/docs/manuals/sketchpad",
    hotkey: "⌘H"
  }}
>
  <ToggleGroupItem value="home">
    <Home />
  </ToggleGroupItem>
</SemioTooltipWrapper>
```

## Mode-Based Behavior

### Expert Mode
- Returns children without any tooltip wrapper
- No performance overhead

### Normal Mode
- Shows translated label (using `labelKey`)
- Shows hotkey badge if provided
- Shows link to manual if `manualPath` provided
- Does NOT show tutorial link

### Beginner Mode
- Shows extensive translated label (using `labelKey.extensive`, falls back to `labelKey`)
- Shows hotkey badge if provided
- Shows link to manual if `manualPath` provided
- Shows link to tutorial if `tutorialPath` provided

## Translation Structure

Add these keys to your locale files:

```json
{
  "tooltip": {
    "manual": "Manual",
    "tutorial": "Tutorial"
  }
}
```

These are used for the link labels in tooltips.

## Manual and Tutorial Paths

### Manuals
Path format: `/docs/manuals/{manual-name}` or `/docs/manuals/{manual-name}#{section}`

Example:
- `/docs/manuals/sketchpad` - Link to entire Sketchpad manual
- `/docs/manuals/grasshopper#kit` - Link to Kit section in Grasshopper manual

### Tutorials
Path format: `/docs/tutorials/{tutorial-name}` or `/docs/tutorials/{tutorial-name}/{step}`

Example:
- `/docs/tutorials/hello-semio` - Link to entire Hello Semio tutorial
- `/docs/tutorials/hello-semio/model-brick-set` - Link to specific tutorial step

## Best Practices

1. **Always provide labelKey**: Even if manual/tutorial links are the main content
2. **Use semantic paths**: Link to the most relevant section of documentation
3. **Keep extensive text concise**: Beginner tooltips should add clarity, not verbosity
4. **Consider context**: Link to tutorials for actions, manuals for concepts
5. **Hotkey consistency**: Use platform-appropriate symbols (⌘ for Mac, Ctrl for Windows)

## Migration Guide

To migrate existing code to the new system:

1. Identify tooltips that would benefit from manual/tutorial links
2. Replace string tooltips with `TooltipConfig` objects
3. Add `labelKey.extensive` translations for beginner mode
4. Add manual/tutorial paths where helpful
5. Test in all three modes (Expert, Normal, Beginner)

## Implementation Details

- The system uses:
  - `Mode` enum from `store.tsx` to track user expertise level (Beginner, Normal, Expert)
  - `setTooltipModeProvider` and `useTooltipMode()` from `elements/display/Tooltip.tsx` to propagate the current mode into tooltip components
  - `useSemioTooltip()` hook to access current mode
  - `SemioTooltip` base component from `elements/display/Tooltip.tsx`
  - `SemioTooltipWrapper` convenience wrapper from `sketchpad/SemioTooltip.tsx` that connects the store mode to the tooltip provider

The tooltip component automatically:
- Hides in Expert mode (returns children unwrapped)
- Adjusts content based on mode
- Handles missing translations gracefully
- Styles links with appropriate hover states
- Positions manual/tutorial links in a footer section
