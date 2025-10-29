# I18n-Based Tooltip System

This document describes the new i18n-based tooltip system that centralizes all labels, descriptions, and documentation paths in locale JSON files.

## Overview

Previously, tooltips were configured inline in code:

```tsx
// OLD WAY ❌
<Stepper
  label={t("common.x")}
  tooltip={tooltip("type.portPointX", { 
    manualPath: "type#ports", 
    tutorialPath: "hello-semio/model-brick-set" 
  })}
  value={port.point.x}
  onChange={(value) => updatePort(port.guid, { point: { x: value } })}
/>
```

Now, everything is centralized in locale files:

```tsx
// NEW WAY ✅
<Stepper
  i18n="semio.type.panel.details.port.point.x"
  value={port.point.x}
  onChange={(value) => updatePort(port.guid, { point: { x: value } })}
/>
```

## Benefits

1. **Centralized Configuration**: All UI strings, descriptions, and paths in one place
2. **Easier Translation**: Translators see everything together
3. **Consistency**: Same pattern across all components
4. **Maintainability**: Change paths once, affects all usages
5. **Scalability**: Easy to add new fields or languages

## Locale Structure

In `locales/en.json` and `locales/de.json`:

```json
{
  "semio": {
    "type": {
      "panel": {
        "details": {
          "port": {
            "point": {
              "x": {
                "label": "X",
                "description": "X position",
                "description.beginner": "X position of the port in 3D space relative to the type's origin.",
                "manual": "type#ports",
                "tutorial": "hello-semio/model-brick-set"
              }
            }
          }
        }
      }
    }
  }
}
```

## Schema

Each i18n key supports these properties:

| Property | Required | Description |
|----------|----------|-------------|
| `label` | Yes | Short label shown in UI (e.g., "X", "Name") |
| `description` | No | Normal mode tooltip text |
| `description.beginner` | No | Beginner mode tooltip text (more detailed) |
| `manual` | No | Path to manual docs (without `/docs/manual/` prefix) |
| `tutorial` | No | Path to tutorial docs (without `/docs/tutorials/` prefix) |
| `hotkey` | No | Keyboard shortcut to display |

## Component Support

### Input

```tsx
<Input
  i18n="semio.type.panel.details.name"
  value={type.name}
  onChange={(e) => updateField({ name: e.target.value })}
/>
```

### Textarea

```tsx
<Textarea
  i18n="semio.type.panel.details.description"
  value={type.description}
  onChange={(e) => updateField({ description: e.target.value })}
/>
```

### Stepper

```tsx
<Stepper
  i18n="semio.type.panel.details.port.point.x"
  value={port.point.x}
  onChange={(value) => updatePort({ point: { x: value } })}
/>
```

## Mode-Based Behavior

The tooltip system adapts based on user mode:

### Expert Mode
- No tooltips shown
- Clean, minimal UI

### Normal Mode
- Shows `description` (or `label` if no description)
- Shows manual link if `manual` path exists
- Shows hotkey if defined

### Beginner Mode
- Shows `description.beginner` (or `description`, or `label` as fallback)
- Shows manual link if `manual` path exists
- Shows tutorial link if `tutorial` path exists
- Shows hotkey if defined

## Naming Convention

Use dot-separated paths following the UI hierarchy:

```
semio.<app>.<panel>.<section>.<subsection>.<field>
```

Examples:
- `semio.type.panel.details.name`
- `semio.type.panel.details.port.point.x`
- `semio.type.panel.details.port.direction.y`
- `semio.design.panel.details.piece.center.x`
- `semio.kit.panel.details.metadata.version`

## Migration Guide

### Step 1: Identify the field
Find the component using `label={t(...)}` and `tooltip={tooltip(...)}`

### Step 2: Create i18n key
Follow the naming convention: `semio.<app>.panel.details.<field>`

### Step 3: Add to locale files
Add entries to both `en.json` and `de.json` (or other languages)

### Step 4: Update component
Replace `label` and `tooltip` props with single `i18n` prop

### Example Migration

**Before:**
```tsx
const tooltip = useTooltip();
// ...
<Stepper
  label={t("common.y")}
  tooltip={tooltip("type.portDirectionY", { 
    manualPath: "type#ports", 
    tutorialPath: "hello-semio/model-brick-set" 
  })}
  value={port.direction.y}
  onChange={(value) => updatePort(port.guid, { direction: { y: value } })}
/>
```

**After:**
```tsx
// No need for useTooltip() or t() anymore!
<Stepper
  i18n="semio.type.panel.details.port.direction.y"
  value={port.direction.y}
  onChange={(value) => updatePort(port.guid, { direction: { y: value } })}
/>
```

**Add to `locales/en.json`:**
```json
{
  "semio": {
    "type": {
      "panel": {
        "details": {
          "port": {
            "direction": {
              "y": {
                "label": "Y",
                "description": "Y coordinate",
                "description.beginner": "Y coordinate of the port direction vector. This defines which direction the port points in 3D space.",
                "manual": "type#ports",
                "tutorial": "hello-semio/model-brick-set"
              }
            }
          }
        }
      }
    }
  }
}
```

**Add to `locales/de.json`:**
```json
{
  "semio": {
    "type": {
      "panel": {
        "details": {
          "port": {
            "direction": {
              "y": {
                "label": "Y",
                "description": "Y-Koordinate",
                "description.beginner": "Y-Koordinate des Port-Richtungsvektors. Dies definiert die Richtung, in die der Port im 3D-Raum zeigt.",
                "manual": "type#ports",
                "tutorial": "hello-semio/model-brick-set"
              }
            }
          }
        }
      }
    }
  }
}
```

## Backward Compatibility

The old `label` and `tooltip` props still work! This allows gradual migration:

```tsx
// This still works ✅
<Input
  label={t("type.name")}
  tooltip={tooltip("type.name", { manualPath: "type#metadata" })}
  value={type.name}
/>

// But this is preferred ✅
<Input
  i18n="semio.type.panel.details.name"
  value={type.name}
/>
```

Components check for `i18n` prop first, then fall back to `label` and `tooltip`.

## Implementation Details

### Tooltip Component

New `I18nTooltipContent` component reads from i18n:

```tsx
function I18nTooltipContent({ i18nKey, mode }: I18nTooltipContentProps) {
  const { t } = useTranslation();
  
  const label = t(`${i18nKey}.label`, { defaultValue: "" });
  const description = mode === Mode.BEGINNER 
    ? t(`${i18nKey}.description.beginner`, { 
        defaultValue: t(`${i18nKey}.description`, { defaultValue: "" }) 
      })
    : t(`${i18nKey}.description`, { defaultValue: "" });
  
  const manualPath = t(`${i18nKey}.manual`, { defaultValue: "" });
  const tutorialPath = t(`${i18nKey}.tutorial`, { defaultValue: "" });
  const hotkey = t(`${i18nKey}.hotkey`, { defaultValue: "" });
  
  // ... render tooltip with links
}
```

### Component Integration

Each form component (Input, Textarea, Stepper) has been updated:

```tsx
interface InputProps {
  // ... existing props
  i18n?: string;  // NEW!
}

function Input({ i18n, label, tooltip, ...props }: InputProps) {
  const { t } = useTranslation();
  const mode = useTooltipMode();
  
  // i18n takes precedence
  const effectiveLabel = i18n ? t(`${i18n}.label`, { defaultValue: label || "" }) : label;
  
  const wrappedInput = i18n ? (
    <Tooltip>
      <TooltipTrigger asChild>{inputElement}</TooltipTrigger>
      <TooltipContent>
        <I18nTooltipContent i18nKey={i18n} mode={mode} />
      </TooltipContent>
    </Tooltip>
  ) : tooltip ? (
    // Old tooltip system still works
    // ...
  ) : inputElement;
  
  // ...
}
```

## Best Practices

1. **Always provide translations**: Add to both `en.json` and `de.json`
2. **Use semantic paths**: Follow UI hierarchy in naming
3. **Write clear descriptions**: Beginner descriptions should be educational
4. **Keep labels short**: 1-2 words max
5. **Test all modes**: Check Expert, Normal, and Beginner modes
6. **Link to docs**: Always provide manual paths, tutorials for complex fields

## Future Enhancements

- [ ] Generate TypeScript types from locale files
- [ ] Validate i18n keys at build time
- [ ] Auto-generate documentation from locale structure
- [ ] Create migration script for bulk conversion
- [ ] Add linting rules to enforce i18n usage
