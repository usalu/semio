---
goal: sketchpad/toolbar
---

# Ticket

## Summary

Bulk close

## Changes

No code changes — research only.

## Log

- Read shared.ts, Sketchpad.tsx, elements.tsx, Home.tsx, Kit.tsx, Type.tsx, Quality.tsx, Feedback.tsx, Docs.tsx, README.md
- Read globals.css design token definitions
- Mapped all toolbar-related files, types, components, and styling

## Todos

- [x] Find all toolbar-related files
- [x] Read main toolbar component code
- [x] Read shared types/interfaces
- [x] Read styling/CSS
- [x] Read design tokens
- [x] Document toolbar system structure

## Plan

Research only — no implementation.

---

# Research Findings

## 1. Toolbar-Related Files

| File                                 | Role                                                                                                                                                                                                               |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `compose/js/sketchpad/shared.ts`     | Types: `PanelKind.TOOLBAR`, `PanelSection.toolbarGroup`, `PanelVisibility.toolbar`, `PanelSizes.toolbarHeight`, `PanelSections.toolbar`                                                                            |
| `compose/js/sketchpad/elements.tsx`  | UI components: `Toggle`, `ToggleGroup`, `ToggleGroupItem`, `Action`, `ActionGroup`, `ActionGroupItem`, `ActionDropdown`, `Button`, `ButtonGroup`, `ButtonGroupItem`, `ButtonCycle`, `Layout` (with `toolbar` prop) |
| `compose/js/sketchpad/Sketchpad.tsx` | Toolbar rendering logic: `ToolbarScopeWrapper`, `LayoutWrapper` toolbar zone composition, group toggling, subtool selection, `getToolbarGroupIcon()`                                                               |
| `compose/js/sketchpad/Home.tsx`      | `HomeToolbarFilters` (filter toggles), `HomeToolbarCreate` (create actions), registers toolbar sections                                                                                                            |
| `compose/js/sketchpad/Kit.tsx`       | `KitToolbarFilters` (artifact kind toggles), `KitToolbarCreate` (artifact create buttons), registers toolbar sections                                                                                              |
| `compose/js/sketchpad/Type.tsx`      | Registers selection + connector toolbar sections                                                                                                                                                                   |
| `compose/js/sketchpad/Quality.tsx`   | Registers view + actions toolbar sections                                                                                                                                                                          |
| `compose/js/sketchpad/Feedback.tsx`  | `FeedbackToolbar` (send button), registers toolbar section                                                                                                                                                         |
| `compose/js/sketchpad/Docs.tsx`      | Registers toolbar sections (if any)                                                                                                                                                                                |
| `compose/js/sketchpad/README.md`     | Specs documentation for toolbar                                                                                                                                                                                    |
| `compose/js/globals.css`             | Design tokens and custom utilities                                                                                                                                                                                 |
| `compose/js/theme.css`               | Color scheme tokens                                                                                                                                                                                                |

## 2. Shared Types/Interfaces (shared.ts)

### PanelKind Enum

```typescript
enum PanelKind {
 TOOLBAR = "toolbar",
 // ... other panel kinds
}
```

### PanelSection (toolbar item shape)

```typescript
interface PanelSection {
 id: string;
 content: ReactNode | (() => ReactNode);
 specificity?: number;
 defaultOpen?: boolean;
 order?: number;
 toolbarGroup?: {
  id: string; // "selection", "filter", "create", "view", "actions", "hand"
  labelId?: string;
  order?: number;
  subToolId?: string;
  subToolLabelId?: string;
  subToolIcon?: ReactNode;
  onActivate?: () => void;
 };
 toolbarPlaceholder?: boolean;
}
```

### PanelVisibility

```typescript
interface PanelVisibility {
 toolbar?: boolean;
 // ... other panels
}
```

### PanelSizes

```typescript
interface PanelSizes {
 toolbarHeight: number; // always set to 52
 // ... other sizes
}
```

### PanelSections

```typescript
interface PanelSections {
 toolbar: PanelSection[];
 // ... other panel section arrays
}
```

### PanelKindConfig for Toolbar

```typescript
[PanelKind.TOOLBAR]: {
  icon: ToolbarIcon,
  position: PanelPosition.BOTTOM,
}
```

## 3. Toolbar Component Architecture

### Layout Component (elements.tsx)

The `Layout` component accepts a `toolbar` ReactNode prop. The toolbar is positioned:

```tsx
<div className="absolute bottom-[calc(100%+var(--spacing-double))] left-1/2 -translate-x-1/2 z-panel pointer-events-none">{toolbar}</div>
```

It floats above the footer, centered horizontally.

### Toolbar Rendering (Sketchpad.tsx LayoutWrapper)

The toolbar has two zones arranged around a vertical seam at 50% width:

1. **Tools Zone** (right of seam): Group toggle buttons
2. **Settings Zone** (left of seam): Content of the active group

```
┌️─️┐️
│️                    CANVAS                                │️
│️                                                          │️
│️    [Settings Zone] ║️ [Tools Zone]                       │️
│️    ← left of seam  ║️ right of seam →                    │️
└️─️┘️
│️                    FOOTER                                │️
```

#### Outer container

```tsx
<div role="toolbar" id="compose.sketchpad.toolbar"
     className="absolute bottom-1.5 left-0 right-0 h-[40px] pointer-events-none px-2">
```

- Fixed height: **40px**
- Full width with 8px horizontal padding

#### Seam container

```tsx
<div id="compose.sketchpad.toolbar.seam"
     className="absolute left-1/2 top-0 h-full w-0 -translate-x-1/2">
```

#### Tools Zone (right side)

```tsx
<div id="compose.sketchpad.toolbar.zone.tools" className="absolute right-[4px] top-0 h-full max-w-[calc(50vw-1rem)] pointer-events-auto">
 <LevelProvider level="panel">
  <div className="bg-panel flex h-full shrink-0 items-center gap-single border rounded-md px-single shadow-sm overflow-hidden">{/* Group buttons rendered here */}</div>
 </LevelProvider>
</div>
```

- Max width: `calc(50vw - 1rem)`
- Background: panel level
- Items spaced with `gap-single` (= `var(--spacing-single)` = 1× spacing unit)
- Padding: `px-single`
- Border, rounded-md, shadow-sm

#### Settings Zone (left side) — only when a group is active

```tsx
<div id="compose.sketchpad.toolbar.zone.settings" className="absolute left-[4px] top-0 h-full max-w-[calc(50vw-1rem)] pointer-events-auto">
 <LevelProvider level="panel">
  <div className="bg-panel flex h-full flex-nowrap items-center gap-single border rounded-md px-single shadow-sm overflow-hidden min-w-0">
   <ToolbarScopeWrapper>{/* Active group's section content */}</ToolbarScopeWrapper>
  </div>
 </LevelProvider>
</div>
```

### Group Order

Groups are rendered in this fixed order:

```typescript
["hand", "selection", "filter", "create", "view", "actions"];
```

### Group Toggle Logic

- Each group is a `Toggle` component (kind="single" or kind="dropdown")
- Only one group can be active at a time
- `activeToolbarGroup` state tracks which group is active
- `toggleToolbarGroup(id)` toggles on/off
- Groups auto-select the first non-"hand" group on mount

### Subtool Support

The "selection" group supports subtools via a `Toggle` with `kind="dropdown"`:

- Each toolbar section can declare a `toolbarGroup.subToolId`
- When a group has multiple subtools, a dropdown chevron appears
- `activeSubToolByGroup` tracks which subtool is active per group
- Settings zone filters to show only the active subtool's content

### ToolbarScopeWrapper

Wraps toolbar content in the correct scope providers (KitScopeProvider, DesignScopeProvider, etc.) based on the current navigation path.

## 4. Component Types in Toolbar

### Toggle (elements.tsx)

Three kinds:

- **"single"/"default"/"icon"**: Simple toggle button (uses ToggleGroup internally with a single item)
- **"withAction"**: Toggle with an embedded action button
- **"dropdown"**: Toggle with a chevron dropdown for sub-options

All Toggle variants internally delegate to `ToggleGroup`:

```typescript
function Toggle<T extends string>(props: ToggleProps<T>) {
 // Delegates to ToggleGroup for all variants
}
```

### ToggleGroup (elements.tsx)

```tsx
<ToggleGroupPrimitive.Root
  className="group/toggle-group flex w-fit shrink-0 items-center border overflow-hidden h-medium divide-x"
>
```

- Height: `h-medium` = `var(--size-medium)` = `calc(7 * var(--spacing))`
- Border + divide-x between items

### ToggleGroupItem (elements.tsx)

Uses `toggleVariants` CVA:

```typescript
const toggleVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-single text-sm font-medium ... h-medium aspect-square p-single ...",
  { variants: { level: { base/window/panel/overlay/temporary hover variants } } }
);
```

- Height: `h-medium`
- Aspect: `aspect-square` (square by default)
- Padding: `p-single`
- Active state: `data-[state=on]:bg-active-base`

### Action (elements.tsx)

```typescript
// Height: h-medium, aspect-square, border, p-single
"... aspect-square p-single h-medium border ...";
```

### ActionGroup (elements.tsx)

```tsx
<div className="group/action-group flex h-small items-center border divide-x overflow-hidden">
```

- Height: `h-small` = `var(--size-small)` = `calc(5 * var(--spacing))`

### Button (elements.tsx)

Wraps `ButtonGroup > ButtonGroupItem`. Uses `buttonGroupItemVariants`:

```typescript
"... h-medium aspect-square p-single overflow-hidden ...";
```

- Height: `h-medium`

## 5. Design Tokens / Sizing System (globals.css)

### Spacing Base

```css
--spacing-compact: 0.2rem; /* compact mode */
--spacing-touch: 0.275rem; /* touch mode */
--spacing: var(--spacing-compact); /* default = compact */
--spacing-single: calc(1 * var(--spacing)); /* 0.2rem */
--spacing-double: calc(2 * var(--spacing)); /* 0.4rem */
```

### Size Scale

```css
--size-tiny: calc(3 * var(--spacing)); /* 0.6rem (9.6px) */
--size-small: calc(5 * var(--spacing)); /* 1.0rem (16px) */
--size-medium: calc(7 * var(--spacing)); /* 1.4rem (22.4px) */
--size-large: calc(9 * var(--spacing)); /* 1.8rem (28.8px) */
--size-huge: calc(11 * var(--spacing)); /* 2.2rem (35.2px) */
--size-mega: calc(13 * var(--spacing)); /* 2.6rem */
--size-giga: calc(15 * var(--spacing)); /* 3.0rem */
```

### Custom Utilities

```css
@utility gap-single {
 gap: var(--spacing-single);
}
@utility gap-double {
 gap: var(--spacing-double);
}
@utility gap-tiny {
 gap: var(--size-tiny);
}
@utility gap-small {
 gap: var(--size-small);
}
@utility gap-medium {
 gap: var(--size-medium);
}

@utility p-single {
 padding: var(--spacing-single);
}
@utility p-double {
 padding: var(--spacing-double);
}

@utility size-tiny {
 width: var(--size-tiny);
 height: var(--size-tiny);
}
@utility size-small {
 width: var(--size-small);
 height: var(--size-small);
}
@utility size-medium {
 width: var(--size-medium);
 height: var(--size-medium);
}
@utility size-large {
 width: var(--size-large);
 height: var(--size-large);
}

@utility h-small {
 height: var(--size-small);
}
@utility h-medium {
 height: var(--size-medium);
}
```

### Touch Mode Override

```css
.touch {
 --spacing: var(--spacing-touch); /* 0.275rem instead of 0.2rem */
}
```

## 6. How Apps Register Toolbar Sections

Each app uses `addSection("toolbar", { ... })` in its setup effect and `removeSection("toolbar", id)` in cleanup. Example from Home.tsx:

```typescript
addSection("toolbar", {
  id: "compose.sketchpad.app.home.toolbar.filters",
  specificity: SectionSpecificity.DESIGN,
  toolbarGroup: {
    id: "filter",
    labelId: "compose.sketchpad.toolbar.parent.filter",
    order: 0,
  },
  content: <HomeToolbarFilters />,
});
```

### Per-App Toolbar Content

| App          | Groups                     | Content Components                                                                                    |
| ------------ | -------------------------- | ----------------------------------------------------------------------------------------------------- |
| **Home**     | filter, create             | `HomeToolbarFilters` (Toggle×3 for temporary/local/remote), `HomeToolbarCreate` (Action×3 for create) |
| **Kit**      | filter, create             | Filter toggles for 9 artifact kinds, Create buttons for 7 artifact kinds                              |
| **Type**     | selection, create          | Selection tools (normal/additive/subtractive), Connector tool                                         |
| **Design**   | selection, create (likely) | Selection + lasso tools, various design tools                                                         |
| **Quality**  | view, actions              | View toggles, action buttons                                                                          |
| **Feedback** | actions                    | `FeedbackToolbar` with send Button                                                                    |
| **Docs**     | (varies)                   | Documentation-specific tools                                                                          |

## 7. Sizing/Spacing Summary

| Element                      | Height                       | Width                             | Gap                   | Padding              |
| ---------------------------- | ---------------------------- | --------------------------------- | --------------------- | -------------------- |
| Toolbar outer container      | 40px fixed                   | full width                        | —                     | px-2 (8px)           |
| Toolbar inner panels         | h-full (40px)                | auto                              | `gap-single` (0.2rem) | `px-single` (0.2rem) |
| Toggle/ToggleGroup           | `h-medium` (1.4rem / 22.4px) | `aspect-square`                   | `gap-single`          | `p-single`           |
| ToggleGroupItem              | `h-medium`                   | `aspect-square` or auto with text | —                     | `p-single`           |
| Action                       | `h-medium`                   | `aspect-square` or auto with text | —                     | `p-single`           |
| ActionGroup                  | `h-small` (1.0rem / 16px)    | auto                              | —                     | —                    |
| Button/ButtonGroupItem       | `h-medium`                   | `aspect-square` or auto with text | `gap-single`          | `p-single`           |
| Icon sizes (SVGs in toggles) | `size-small` (1.0rem)        | `size-small`                      | —                     | —                    |
| Icon sizes (SVGs in actions) | `size-tiny` (0.6rem)         | `size-tiny`                       | —                     | —                    |

## 8. Key Observations

1. **Two-zone seam layout**: Tools zone (right) and settings zone (left) are separated by a center seam
2. **All toolbar items use panel-level styling** (bg-panel, hover-panel)
3. **Consistent h-medium height** for all interactive elements (Toggle, Button, Action)
4. **ActionGroup is shorter** (h-small) — used for footer items, not directly in toolbar
5. **toolbarHeight is always 52** in state but the actual rendered height is 40px via CSS
6. **Groups are fixed-order** and rendered conditionally based on available sections
7. **Subtool dropdown** only actively used for "selection" group with subtools
8. **Touch mode** scales everything up by changing the spacing base from 0.2rem to 0.275rem
