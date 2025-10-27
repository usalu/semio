# Tooltip Usage Guide

This guide shows how to use the new i18n-enforced tooltip system.

## Basic Concept

All tooltips **must** use the `TooltipConfig` interface with i18n keys. String tooltips are no longer supported.

## TooltipConfig Interface

```typescript
interface TooltipConfig {
  labelKey: string;        // Required: i18n key for the tooltip label
  manualPath?: string;     // Optional: path to manual (without /docs/manual/ prefix)
  tutorialPath?: string;   // Optional: path to tutorial (without /docs/tutorials/ prefix)
  hotkey?: string;         // Optional: keyboard shortcut to display
}
```

## Automatic Features

### 1. Extensive Mode (Beginner)
The system automatically appends `.extensive` to the labelKey in beginner mode:
- Beginner mode: Uses `${labelKey}.extensive` (falls back to base key if not found)
- Normal/Expert mode: Uses `${labelKey}` directly

### 2. Path Prefixing
The component automatically adds path prefixes:
- Manual paths: `/docs/manual/` is prepended automatically
- Tutorial paths: `/docs/tutorials/` is prepended automatically

You only provide the path segment (e.g., `"semio#design"` instead of `"/docs/manual/semio#design"`).

## Mode Behavior

### Expert Mode
- **No tooltips shown** at all

### Normal Mode
- Shows: Label + Manual link (if provided)
- Uses base i18n key (e.g., `"navbar.createDesign"`)

### Beginner Mode
- Shows: Label + Manual link + Tutorial link (if provided)
- Uses extensive i18n key (e.g., `"navbar.createDesign.extensive"`)
- Falls back to base key if `.extensive` translation doesn't exist

## Examples

### Simple Tooltip (Label Only)

```tsx
<Button 
  tooltip={{ labelKey: "navbar.createDesign" }}
  onClick={handleCreate}
>
  Create Design
</Button>
```

**i18n Keys:**
```json
{
  "navbar": {
    "createDesign": "Create a new design",
    "createDesign.extensive": "Click to create a new design in your current kit"
  }
}
```

### Tooltip with Manual Link

```tsx
<Button 
  tooltip={{ 
    labelKey: "navbar.createDesign",
    manualPath: "semio#design"  // Component adds /docs/manual/ prefix
  }}
  onClick={handleCreate}
>
  Create Design
</Button>
```

Renders links to: `/docs/manual/semio#design`

### Full Tooltip with Tutorial

```tsx
<Button 
  tooltip={{ 
    labelKey: "navbar.createDesign",
    manualPath: "semio#design",
    tutorialPath: "hello-semio/model-design"  // Component adds /docs/tutorials/ prefix
  }}
  onClick={handleCreate}
>
  Create Design
</Button>
```

Renders links to:
- Manual: `/docs/manual/semio#design`
- Tutorial: `/docs/tutorials/hello-semio/model-design`

### Tooltip with Hotkey

```tsx
<Button 
  tooltip={{ 
    labelKey: "navbar.createDesign",
    manualPath: "semio#design",
    tutorialPath: "hello-semio/model-design",
    hotkey: "⌘D"
  }}
  onClick={handleCreate}
>
  Create Design
</Button>
```

## Component Support

All input components support the tooltip prop:

- `Action`
- `Button`
- `ButtonGroup` (on items)
- `Toggle` (all 4 variants)
- `ToggleGroup` (on items)
- `Input`
- `Select`
- `Slider`
- `Stepper`
- `Textarea`
- `Combobox`

## i18n Translation Structure

**Required Pattern:**

```json
{
  "section": {
    "key": "Short description",
    "key.extensive": "Longer, more detailed explanation for beginners"
  }
}
```

**Example:**

```json
{
  "navbar": {
    "createDesign": "Create Design",
    "createDesign.extensive": "Click to create a new design. Designs are assemblies of types connected together.",
    "createType": "Create Type",
    "createType.extensive": "Click to create a new type. Types are reusable components with ports for connections."
  }
}
```

## Migration from Old String Tooltips

**Old (No longer supported):**
```tsx
<Button tooltip="Create a design" hotkey="⌘D" />
```

**New (Required):**
```tsx
<Button 
  tooltip={{ 
    labelKey: "navbar.createDesign",
    hotkey: "⌘D"
  }}
/>
```

## Common Patterns

### Navigation Actions
```tsx
tooltip={{ 
  labelKey: "navbar.back",
  manualPath: "interface#navigation"
}}
```

### Create Actions
```tsx
tooltip={{ 
  labelKey: "navbar.createDesign",
  manualPath: "semio#design",
  tutorialPath: "hello-semio/model-design",
  hotkey: "⌘D"
}}
```

### Panel Toggles
```tsx
tooltip={{ 
  labelKey: "panels.details",
  manualPath: "interface#panels",
  hotkey: "⌘L"
}}
```

### Tool Selection
```tsx
tooltip={{ 
  labelKey: "tools.select",
  manualPath: "tools#selection",
  tutorialPath: "basics/selecting",
  hotkey: "V"
}}
```

## Best Practices

1. **Always provide .extensive translations** for beginner mode
2. **Keep base tooltips short** (1-3 words ideal)
3. **Make .extensive tooltips educational** (explain what, why, and how)
4. **Use semantic paths** (e.g., "semio#design" not "design-page")
5. **Include hotkeys** when actions have keyboard shortcuts
6. **Link tutorials for complex features** to help beginners
7. **Use consistent i18n key structure** (e.g., "section.action" pattern)

## Debugging

To test different modes, change the mode in Sketchpad:

Import `Mode` from `@semio/js/sketchpad/store`.

```tsx
<Sketchpad 
  mode={Mode.BEGINNER}  // or NORMAL, EXPERT
  {...props}
/>
```

Check browser console for missing translations (i18next will warn).
