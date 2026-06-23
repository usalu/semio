---
goal: R26-02/UPDATED-SKETCHPAD
---

# Ticket

## Summary

Unified toolbar spacing and sizing system for the Sketchpad toolbar — analysis, design strategy, and implementation plan.

## Plan

### 1. Root Cause Analysis

**Why spacing/sizing is inconsistent today:**

The toolbar renders three different component families — `Toggle` (via `ToggleGroup`), `Button` (via `ButtonGroup`), and raw HTML dividers — side by side in a `flex gap-single` container. Each family computes its own dimensions through independent cva variant paths:

| Component | Height source | Width rule | Padding | Gap between items |
|-----------|--------------|-----------|---------|------------------|
| `Toggle` / `ToggleGroupItem` | `h-medium` (7×spacing = 1.4rem) from `toggleVariants` | `aspect-square` when icon-only; `w-auto` when text present | `p-single` | `gap-single` inside ToggleGroup; `gap-single` between sibling Toggles |
| `Button` / `ButtonGroupItem` | `h-medium` from `buttonGroupItemVariants` | `aspect-square` when icon-only; `w-auto` when text present | `p-single` | `divide-x` inside ButtonGroup; `gap-single` between sibling Buttons |
| `ActionGroup` / `ActionGroupItem` | `h-small` (5×spacing = 1.0rem) | `aspect-square` | `p-single` | `divide-x` inside ActionGroup |
| Divider (`<div className="h-small w-px ...">`) | `h-small` | 1px | — | — |

**Specific issues:**

1. **Container-level duplication**: Each `Toggle` without text wraps itself in a `ToggleGroup` of one item. Each `Button` wraps itself in a `ButtonGroup` of one item. Both `ToggleGroup` and `ButtonGroup` add their own `border` + `divide-x`. When these are rendered sequentially in the settings zone, each gets its own border frame, causing visual fragmentation.

2. **Settings zone ad-hoc layout**: App-specific content wrappers (`HomeToolbarFilters`, `KitFilters`, `KitToolbarSelection`, `DesignSelectSettings`, `KitCreateActions`) each wrap their items in their own `<div className="flex shrink-0 items-center gap-single ...">` with inconsistent `px-single`, `h-full`, and `property-panel` classes. These wrappers introduce an extra layout layer with inconsistent horizontal padding.

3. **Divider height mismatch**: Manual dividers use `h-small` (1.0rem) inside a container that's `h-medium` (1.4rem), which is correct for visual proportion but the divider is separately authored in each app, with no shared abstraction.

4. **Tool zone vs Settings zone**: The tools zone (left of seam) uses a single `flex h-full shrink-0 items-center gap-single border rounded-md px-single` container directly wrapping Toggle items. The settings zone repeats the same pattern. But the two containers have different max-width calculations and can independently clip.

5. **No constraint on inner-content height**: App content renders inside `<div className="shrink-0 flex items-center min-w-0">` with no explicit `h-full` or `h-medium` constraint, relying on the parent's `items-center` to vertically center them. But child Toggles have their own `h-medium` while the parent has no explicit height, so the browser resolves height from content.

### 2. Design System Strategy

#### 2.1 Design Tokens (already exist, reuse)

From `globals.css`:
- `--spacing` (0.2rem compact / 0.275rem touch)
- `--spacing-single` = 1×spacing
- `--spacing-double` = 2×spacing
- `--size-tiny` = 3×spacing (icon size in actions)
- `--size-small` = 5×spacing (icon size in toggles)
- `--size-medium` = 7×spacing (interactive element height)

**New tokens to introduce** (in `globals.css`):
- `--toolbar-item-height`: `var(--size-medium)` — canonical height for all toolbar interactive items
- `--toolbar-gap`: `var(--spacing-single)` — gap between toolbar items within a group
- `--toolbar-group-gap`: `var(--spacing-double)` — gap between toolbar groups (across dividers)
- `--toolbar-padding-inline`: `var(--spacing-single)` — horizontal padding inside each toolbar zone
- `--toolbar-divider-height`: `var(--size-small)` — divider visual height

#### 2.2 Layout Primitives

**`ToolbarZone`**: A flex container with `h-[var(--toolbar-item-height)]`, `items-center`, `gap-[var(--toolbar-gap)]`, `px-[var(--toolbar-padding-inline)]`, `border`, `rounded-md`, `bg-panel`. Shared between Tools zone and Settings zone.

**`ToolbarGroup`**: A flex container within a zone that holds semantically related items. Uses `gap-[var(--toolbar-gap)]`. Groups are separated by `ToolbarDivider`.

**`ToolbarDivider`**: A `<div>` with `w-px`, `h-[var(--toolbar-divider-height)]`, `bg-border`, `my-auto`, `shrink-0`. Replaces all manual divider `<div>`s.

**`ToolbarItem`**: A wrapper `<div>` with `shrink-0 flex items-center h-full` that normalizes any child component (Toggle, Button, Action) into the toolbar's height slot. This removes the need for app-specific wrappers.

#### 2.3 Component Normalization

All toolbar-rendered components already use `h-medium` = `var(--size-medium)`. The strategy is:
1. Keep `h-medium` on all interactive elements (already consistent)
2. Remove per-app wrapper divs and replace with `ToolbarGroup` / `ToolbarDivider`
3. The outer zone container enforces the canonical height

### 3. Technical Implementation Plan

#### Phase 1: Extract shared toolbar layout components into `elements.tsx`

Add a new region `// #region Toolbar` in `elements.tsx` with:
- `ToolbarZone` component
- `ToolbarGroup` component
- `ToolbarDivider` component
- `ToolbarItem` wrapper component

These use the existing tokens, no new CSS variables needed initially — just consistent application of `gap-single`, `h-medium`, `px-single`, and the level-aware border classes.

#### Phase 2: Refactor `Sketchpad.tsx` toolbar rendering

Replace inline toolbar zone markup:
```tsx
<div className="bg-panel flex h-full shrink-0 items-center gap-single border rounded-md px-single shadow-sm overflow-hidden">
```
with `<ToolbarZone>` for both tools and settings zones.

Replace the per-section wrapper:
```tsx
<div key={section.id} className="shrink-0 flex items-center min-w-0">
```
with `<ToolbarItem>`.

#### Phase 3: Migrate app-specific toolbar content

For each app (`Home.tsx`, `Kit.tsx`, `Design.tsx`, `Type.tsx`, `Quality.tsx`, `Feedback.tsx`):
- Replace manual `<div className="flex shrink-0 items-center gap-single h-full px-single ...">` wrappers with `<ToolbarGroup>`.
- Replace manual `<div className="h-small w-px bg-border my-auto" />` with `<ToolbarDivider />`.
- Remove redundant CSS classes that are now handled by the zone container.

#### Phase 4: Introduce `--toolbar-*` tokens

Add the token aliases to `globals.css` so future changes to toolbar spacing can be done in one place.

### 4. Migration Strategy

1. **Additive first**: Add new components without removing old markup
2. **One app at a time**: Migrate Home → Kit → Design → Type → Quality → Feedback
3. **Visual parity check**: Each migration must look identical to before (same pixel sizes) — only the code structure changes
4. **Then tune**: Once all apps use ToolbarZone/ToolbarGroup/ToolbarDivider, adjusting spacing is a single token change

### 5. Edge Cases

- **Dropdown overflow**: Toggle dropdowns (Popover) must still be able to overflow the toolbar zone. The zone uses `overflow-hidden` today — keep the popover portalled (it already is via Radix).
- **Empty groups**: Some apps register toolbar sections conditionally. `ToolbarZone` must handle zero children gracefully (collapse to zero width).
- **Touch mode**: `--spacing` changes from compact to touch. All toolbar tokens derive from it, so sizes scale automatically.
- **Long text labels**: Toggles/Buttons with `text` prop need `whitespace-nowrap` (already set). The zone needs `overflow-hidden` to clip.
- **Responsive width**: Both zones have `max-w-[calc(50vw-1rem)]`. This stays.
- **DnD overlay**: The drag overlay sits above the toolbar at `z-modal` level — no conflict.
- **Accessibility**: `role="toolbar"` is already on the outer container. Group semantics can be enhanced with `role="group"` on `ToolbarGroup`.

## Todos

- [x] Root cause analysis
- [x] Design system strategy
- [x] Technical implementation plan
- [x] Migration strategy
- [x] Edge cases
- [x] Implement ToolbarZone/ToolbarGroup/ToolbarDivider/ToolbarItem in elements.tsx
- [x] Add --toolbar-* tokens to globals.css
- [x] Refactor Sketchpad.tsx toolbar rendering to use new primitives
- [x] Migrate Home.tsx toolbar content
- [x] Migrate Kit.tsx toolbar content
- [x] Migrate Design.tsx toolbar content
- [x] Migrate Type/Quality/Feedback toolbar content
- [x] Run tests
- [x] CSS normalization: strip inner group borders and force height inheritance in toolbar context
- [x] Document toolbar sizing normalization in README specs

## Changes

- `compose/js/globals.css`: Added 5 toolbar design tokens (`--toolbar-item-height`, `--toolbar-gap`, `--toolbar-group-gap`, `--toolbar-padding-inline`, `--toolbar-divider-height`). Added CSS normalization rules for `[data-slot="toolbar-zone"]` descendants that strip `border-width` and set `height: 100%` on `toggle-group`, `button-group`, `toggle-group-item`, and `button-group-item` so all toolbar elements derive their height from the zone rather than hardcoded `h-medium`.
- `compose/js/sketchpad/elements.tsx`: Added `ToolbarZone`, `ToolbarGroup`, `ToolbarDivider`, `ToolbarItem` components in new `Toolbar Components` region
- `compose/js/sketchpad/Sketchpad.tsx`: Replaced inline toolbar zone divs with `ToolbarZone`, per-section wrappers with `ToolbarItem`
- `compose/js/sketchpad/Home.tsx`: Migrated `HomeToolbarFilters` and `HomeToolbarCreate` wrappers to `ToolbarGroup`
- `compose/js/sketchpad/Kit.tsx`: Migrated `KitKindToggles`, `KitFilters`, `KitToolbarSelection`, `KitCreateActions` to `ToolbarGroup`/`ToolbarDivider`
- `compose/js/sketchpad/Design.tsx`: Migrated `DesignSelectSettings` 3-category layout to nested `ToolbarGroup`/`ToolbarDivider`
- `compose/js/sketchpad/Type.tsx`: Migrated `TypeSelectSettings` and `TypeConnectorSettings` wrappers to `ToolbarGroup`
- `compose/js/sketchpad/Feedback.tsx`: Migrated `FeedbackToolbar` wrapper to `ToolbarGroup`
- `compose/js/sketchpad/README.md`: Documented toolbar sizing normalization mechanism in Specs

## Log

- Analyzed current toolbar implementation across elements.tsx, Sketchpad.tsx, Home.tsx, Kit.tsx, Design.tsx
- Identified 5 root causes of spacing inconsistency
- Designed token-driven layout primitive strategy
- Created phased implementation plan with migration strategy
- Implemented tokens in globals.css and components in elements.tsx
- Refactored Sketchpad.tsx zones to use ToolbarZone/ToolbarItem
- Migrated all 6 app files (Home, Kit, Design, Type, Quality, Feedback)
- Verified zero remaining manual toolbar patterns
- All 11 JS tests pass
- Reopened: Added CSS normalization rules to strip inner group borders and force height inheritance for toolbar descendants
- Documented toolbar sizing normalization in sketchpad README specs
