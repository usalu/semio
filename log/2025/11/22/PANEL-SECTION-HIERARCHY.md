---
date: '2025-11-21T23:00:00.000Z'
slug: PANEL-SECTION-HIERARCHY
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Migration from 2025-11-22_PANEL-SECTION-HIERARCHY.md
model: unknown
---
# Panel Section Hierarchy

## Analysis

The panel system should order sections by specificity:
- **Most specific** (top): App-specific sections (e.g., Design App selection details)
- **Less specific**: App-level sections (e.g., Kit App settings)
- **Least specific** (bottom): Sketchpad-level sections (e.g., general settings)

### Current Implementation Issues

1. **No inheritance mechanism**: Child apps don't automatically inherit parent sections
2. **No ordering by specificity**: Sections are ordered by registration order, not specificity
3. **Manual duplication**: Each app manually adds similar sections without hierarchy

### Expected Behavior Examples

#### Details Panel
- **Top (most specific)**: Selection details (pieces, connections, ports selected)
- **Middle**: Design details (current design metadata)
- **Bottom**: Kit details (kit metadata)

#### Settings Panel
- **Top (most specific)**: Design App settings (tool preferences, view options)
- **Middle**: Kit App settings (kit-specific preferences)
- **Bottom**: Sketchpad settings (theme, language, expertise, layout, mode)

#### Workbench Panel
- **Top**: Design-specific workbench (layers, groups)
- **Bottom**: Kit workbench (types, designs, qualities)

## Implementation Plan

### 1. Add Section Hierarchy System

Modify `PanelSection` interface to include:
- `specificity: number` - Higher number = more specific (rendered first)
- `appId?: string` - Which app owns this section (for automatic cleanup)

### 2. Update PanelSectionProvider

- Sort sections by specificity (descending) when rendering
- Track which app registered each section for cleanup
- Auto-remove app sections when app unmounts

### 3. Define Specificity Levels

```typescript
enum SectionSpecificity {
  SKETCHPAD = 0,      // General sketchpad-level
  KIT = 10,           // Kit-level 
  QUALITY = 20,       // Quality app
  TYPE = 20,          // Type app
  DESIGN = 20,        // Design app
  SELECTION = 30,     // Current selection (most specific)
}
```

### 4. Update Each App

#### Sketchpad (App.tsx)
- Register base settings section with `specificity: 0`
- Register base details section with `specificity: 0`

#### Kit App
- Register kit settings with `specificity: 10`
- Register kit details with `specificity: 10`
- Register workbench with `specificity: 10`

#### Design App
- Register design settings with `specificity: 20`
- Register design details with `specificity: 20`
- Register workbench (layers/groups) with `specificity: 20`
- Register selection details with `specificity: 30`

#### Type App
- Register type settings with `specificity: 20`
- Register type details with `specificity: 20`

#### Quality App
- Register quality settings with `specificity: 20`
- Register quality details with `specificity: 20`

### 5. Implementation Steps

1. ✅ Update `PanelSection` interface in `elements.tsx`
2. ✅ Update `PanelSectionProvider` to sort by specificity (descending), then order (ascending)
3. ✅ Add `SectionSpecificity` enum to `elements.tsx`
4. ✅ Export `SectionSpecificity` from `App.tsx`
5. ✅ Update Home App sections (SKETCHPAD = 0)
6. ✅ Update Kit App sections (KIT = 10, SELECTION = 30 for artifacts)
7. ✅ Update Design App sections (DESIGN = 20, SELECTION = 30)
8. ✅ Update Type App sections (TYPE = 20, SELECTION = 30 for ports)
9. ✅ Update Quality App sections (QUALITY = 20)
10. ✅ Update Docs App sections (DOCS = 20)

## Implementation Complete

All sections now include `specificity` field and will be ordered correctly:
- **Specificity 30** (most specific, top): Selected items (ports, pieces, connections, artifacts)
- **Specificity 20**: App-level details (Design, Type, Quality, Docs)
- **Specificity 10**: Kit-level details
- **Specificity 0** (least specific, bottom): Sketchpad-level (settings, general)

## Breaking Changes

- All `addSection` calls must include `specificity` parameter
- Sections will be reordered automatically

## Benefits

1. **Consistent ordering**: Sections always appear in specificity order
2. **Inheritance**: Child apps automatically show parent sections below theirs
3. **Automatic cleanup**: App sections removed when app unmounts
4. **Predictable UX**: Users always find most specific info at top
