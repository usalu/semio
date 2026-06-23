---
goal: SKETCHPADLAYOUT/COMPONENTANALYSIS
---

# Ticket

## Summary

Completed detailed layout analysis of 10 components in elements.tsx: Label, TreeContent, TreeSection, TreeItem, Input, Combobox, Slider, Stepper, Textarea, SidePanel. Documented all data-slots, layout structures, sizing defaults, showLabel wrapping patterns, and right-panel sizing interactions.
## Plan

1. Read each component definition
2. Document data-slots, layout, sizing, and label interaction
3. Compile report

## Changes

- Read-only analysis; no code changes.

## Log

- Analyzed Label (~L836-861)
- Analyzed TreeContent (~L3764)
- Analyzed TreeSection (~L3850-3982)
- Analyzed TreeItem (~L4162-4308)
- Analyzed Input (~L1981-2098)
- Analyzed Combobox (~L1880-1979)
- Analyzed Slider (~L2278-2498)
- Analyzed Stepper (~L2500-2745)
- Analyzed Textarea (~L2750-2842)
- Analyzed SidePanel (~L4994-5135)

## Todos

- [x] Read all 10 components
- [x] Write report

---

# DETAILED COMPONENT LAYOUT REPORT

Source: `compose/js/sketchpad/elements.tsx` (6992 lines)

---

## 1. Label (lines ~836–861)

### Data-slots
| Slot | Element |
|------|---------|
| `property-row` | Outer `<div>` (grid container) |
| `property-label` | `<span>` (label text) |
| `property-control` | `<div>` (child wrapper) |

### Layout structure
```
<div data-slot="property-row">          ← CSS Grid
  <span data-slot="property-label">     ← Column 1 (96px)
  <div data-slot="property-control">    ← Column 2 (1fr)
    {children}
  </div>
</div>
```

**Grid**: `gridTemplateColumns: "96px 1fr"`, `gap: "8px"`, `minHeight: "24px"`
**Classes**: `group grid min-w-0 w-full items-center`

### Label column
- Fixed width: **96px**
- Height: **22px** (`h-[22px]`)
- Text: `text-xs font-medium`, truncated, left-aligned
- Hover: `hover:bg-hover-panel`
- Wrapped in `<Tooltip>` → `<TooltipTrigger asChild>` → `<TooltipContent>` with `<DescriptionTooltipContent>`

### Control column
- `min-w-0` (prevents overflow)
- Receives `{children}` directly

### Label+Input interaction
The `Label` component is **never used standalone**. Input components (Input, Slider, Stepper, Textarea, Combobox, Select) conditionally wrap themselves in `<Label>` when `showLabel` is true:
```tsx
if (showLabel && id) {
  return <Label id={id} labelElementId={`${id}-label`}>{inputElement}</Label>;
}
return inputElement;
```
This means the grid layout (96px | 1fr) only appears when `showLabel` is truthy. Without it, the input element renders raw.

---

## 2. TreeContent (line ~3764)

### Data-slots
| Slot | Element |
|------|---------|
| `tree-content` | Outer `<div>` |

### Layout structure
```
<div data-slot="tree-content" class="relative"
     style="paddingTop: 3px; paddingBottom: 3px; paddingLeft: {level * 0.75}rem">
  <IndentationLines />
  {children}
</div>
```

- **Relative positioning** for absolute `IndentationLines` overlay
- **Padding**: top/bottom `3px`, left = `level * 0.75rem` (indentation based on nesting level from `TreeContext`)
- No flex/grid – children flow normally (block layout)

---

## 3. TreeSection (lines ~3850–3982)

### Data-slots
| Slot | Element |
|------|---------|
| `tree-section-row` | Outer row `<div>` (both empty and collapsible variants) |
| `tree-label` | `<span>` label text |

### Layout structure

**Two variants:**

#### A. No children (empty section)
```
<div data-slot="tree-section-row" class="relative flex items-center gap-[6px] ..."
     style="paddingLeft: {level*0.75}rem; height: 20px; marginBottom: 6px">
  <IndentationLines />
  <div class="w-[14px] flex-shrink-0" />           ← spacer (no chevron)
  {icon}
  <span data-slot="tree-label">...</span>
  {actions}
</div>
```

#### B. Has children (collapsible)
```
<Collapsible>
  <CollapsibleTrigger asChild>
    <div data-slot="tree-section-row" class="relative flex items-center gap-[6px] ..."
         style="paddingLeft: {level*0.75}rem; height: 20px; marginBottom: 6px">
      <IndentationLines />
      <ChevronDown/RightIcon class="size-[14px] flex-shrink-0" />
      {icon}
      <span data-slot="tree-label">...</span>
      {actions}
    </div>
  </CollapsibleTrigger>
  <CollapsibleContent>
    <TreeContext.Provider value={level+1}>
      {children}
    </TreeContext.Provider>
  </CollapsibleContent>
</Collapsible>
```

### Default sizing
- **Row height**: `20px`
- **Bottom margin**: `6px`
- **Chevron**: `14px × 14px`
- **Gap**: `6px`
- **Left padding**: `level * 0.75rem`

### Label styling
- `text-xs text-muted-foreground font-semibold uppercase tracking-wide truncate`
- Optionally wrapped in `<Tooltip>` when `id` is provided
- `flex-1` takes remaining space

### Actions
- Rendered in `<div class="flex items-center gap-single">` at the end of the row

---

## 4. TreeItem (lines ~4162–4308)

### Data-slots
| Slot | Element |
|------|---------|
| `tree-item-row` | Outer row `<div>` |
| `tree-label` | `<span>` label text |

### Layout structure

**Three variants:**

#### A. Has children + label (expandable)
```
<>
  <div data-slot="tree-item-row" role="treeitem"
       class="relative flex items-center gap-[6px] ..."
       style="paddingLeft: {level*0.75}rem">
    <IndentationLines />
    <button>                              ← fold/unfold toggle
      <ChevronDown/RightIcon class="size-3" />
    </button>
    {icon}
    <span data-slot="tree-label" class="flex-1 text-xs font-normal truncate text-foreground cursor-selectable"
          onClick={onClick}>
      {label}
    </span>
    {actions}
  </div>
  {open && <TreeContext.Provider value={level+1}>{children}</TreeContext.Provider>}
</>
```

#### B. No label (passthrough)
```
<TreeContext.Provider value={level, ...}>{children}</TreeContext.Provider>
```

#### C. Leaf item (no children)
```
<div data-slot="tree-item-row" role="treeitem"
     class="relative flex items-center gap-[6px] ..."
     style="paddingLeft: {level*0.75}rem"
     onClick={onClick}>
  <IndentationLines />
  {icon}
  <span data-slot="tree-label" class="flex-1 text-xs font-normal truncate text-foreground">
    {label}
  </span>
  {actions}
</div>
```

### Default sizing
- **No explicit height** (content-driven, typically ~20px from text-xs)
- **Chevron**: `size-3` (12px)
- **Gap**: `6px`
- **Left padding**: `level * 0.75rem`

### State classes
- Selected: `bg-active-base text-active-foreground`
- Highlighted: `bg-active-base text-active-foreground`
- Hover: `hover:bg-hover-panel`

### SortableTreeItem variant
- Same layout but uses `useSortable` hook for DnD
- Adds `ref={setNodeRef}`, `transform`, `transition`, `opacity` via inline style
- Optional drag handle: `<Action class="cursor-grab" {...attributes} {...listeners} icon={<GripVerticalIcon />} />`

---

## 5. Input (lines ~1981–2098)

### Data-slots
| Slot | Element |
|------|---------|
| `input` | `<input>` element |

### Layout structure
```
<div style="opacity: shouldFade ? 0 : 1; transition: opacity 150ms">
  <input data-slot="input" type={type} ... />
</div>
```
When `showLabel && id`:
```
<Label id={id} labelElementId={`${id}-label`}>
  <div style="opacity ...">
    <input data-slot="input" ... />
  </div>
</Label>
```

### Default sizing
- **Height**: `h-medium` (CSS variable)
- **Width**: `w-full min-w-0`
- **Padding**: `p-single`
- **Text**: `text-base` → `md:text-sm`
- **Border**: standard border, focus: `focus-visible:border-accent`
- Number type: hides spin buttons via WebKit/Moz rules

### Interaction model
- **lazy mode**: local state tracked; committed on blur/Enter, aborted on Escape
- **interactionId**: fades other controls when this one is active (opacity 0 → 1 transition)
- **transaction**: start on focus, finalize on blur/Enter, abort on Escape

---

## 6. Combobox (lines ~1908–1976)

### Data-slots
None directly — uses composite sub-components:
| Slot (from sub-components) | Source component |
|---|---|
| `popover`, `popover-trigger`, `popover-content` | Popover |
| `command`, `command-input`, `command-list`, `command-empty`, `command-group`, `command-item` | Command |

The trigger button gets `role="combobox"` and `aria-expanded={open}`.

### Layout structure
```
<Popover>
  <PopoverTrigger asChild>
    <Button id={id} role="combobox" aria-expanded={open}
            class="w-full justify-between flex-1 min-w-0">
      {selectedLabel || placeholder}
      <ChevronsUpDownIcon class="ml-2 size-tiny shrink-0 opacity-50" />
    </Button>
  </PopoverTrigger>
  <PopoverContent class="w-full" align="start">
    <Command>
      <CommandInput placeholder="Search..." />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        <CommandGroup>
          {clearItem?}
          {options.map → <CommandItem>}
        </CommandGroup>
      </CommandList>
    </Command>
  </PopoverContent>
</Popover>
```

### When showLabel && id
```
<Label id={id} labelElementId={`${id}-label`} className="h-medium">
  {comboboxElement}
</Label>
```
Note: extra `h-medium` class on Label to constrain row height.

### Default sizing
- Button: `w-full`, `h-medium` (from Button defaults)
- Popover content: `w-full`, aligned to start
- Command list: `max-h-[300px]` with scroll

---

## 7. Slider (lines ~2278–2498)

### Data-slots
| Slot | Element |
|------|---------|
| `slider` | `<SliderPrimitive.Root>` |
| `slider-track` | `<SliderPrimitive.Track>` |
| `slider-range` | `<SliderPrimitive.Range>` |
| `slider-thumb` | `<SliderPrimitive.Thumb>` (per value) |
| `slider-content` | Outer wrapper `<div>` |
| `slider-row` | Grid row `<div>` |
| `slider-track-cell` | Track cell `<div>` |
| `slider-value` | Value display `<span>` |

### Layout structure
```
<div data-slot="slider-content" class="flex-1 min-w-0"
     style="opacity: shouldFade ? 0 : 1">
  <div data-slot="slider-row"
       class="grid h-[22px] grid-cols-[minmax(0,1fr)_28px] items-center gap-x-[8px]">
    <div data-slot="slider-track-cell" class="min-w-0">
      <Tooltip>
        <TooltipTrigger asChild>
          <SliderPrimitive.Root data-slot="slider" class="relative flex w-full touch-none items-center select-none ...">
            <SliderPrimitive.Track data-slot="slider-track" class="... h-single w-full ...">
              <SliderPrimitive.Range data-slot="slider-range" />
            </SliderPrimitive.Track>
            <SliderPrimitive.Thumb data-slot="slider-thumb" class="size-small ..." />
          </SliderPrimitive.Root>
        </TooltipTrigger>
      </Tooltip>
    </div>
    <span data-slot="slider-value" class="w-[28px] text-right text-xs ...">
      {displayValue}       ← or <Input> when editing
    </span>
  </div>
</div>
```

### When showLabel
```
<Label id={id} labelElementId={`${id}-label`} className={className}>
  {sliderContent}
</Label>
```

### Default sizing
- **slider-row**: `h-[22px]`, grid `[minmax(0,1fr) 28px]`, gap `8px`
- **slider-track**: `h-single` (horizontal), full width
- **slider-thumb**: `size-small`, rounded-full
- **slider-value**: fixed `w-[28px]`, right-aligned, `text-xs`
- **Overall**: fits inside the 1fr column of Label's grid (right side of 96px | 1fr)

---

## 8. Stepper (lines ~2500–2745)

### Data-slots
| Slot | Element |
|------|---------|
| `stepper-group` | Outer `<div>` (flex container) |
| `stepper-minus` | Decrement `<button>` |
| `stepper-plus` | Increment `<button>` |

### Layout structure
```
<Label id={id}>
  <div data-slot="stepper-group"
       class="flex h-[22px] w-[100px] min-w-[100px] items-stretch overflow-hidden rounded-[3px] border ...">
    <button data-slot="stepper-minus"
            class="flex h-[22px] w-[22px] ... items-center justify-center border-r ...">
      <RemoveIcon class="size-tiny" />
    </button>
    <Input type="number" class="h-[22px] w-[56px] min-w-[56px] border-0 px-0 text-center ..." />
    <button data-slot="stepper-plus"
            class="flex h-[22px] w-[22px] ... items-center justify-center border-l ...">
      <AddIcon class="size-tiny" />
    </button>
  </div>
</Label>
```

**ALWAYS wraps in Label** — no `showLabel` conditional. The Stepper always renders inside `<Label id={id}>`.

### Default sizing
- **stepper-group**: `h-[22px]`, `w-[100px]`, `min-w-[100px]`
- **minus/plus buttons**: `22px × 22px` each
- **center input**: `h-[22px]`, `w-[56px]`, `min-w-[56px]`
- **Total**: 22 + 56 + 22 = 100px
- **Border radius**: `rounded-[3px]`
- Focus: `focus-within:border-accent`

---

## 9. Textarea (lines ~2750–2842)

### Data-slots
| Slot | Element |
|------|---------|
| `textarea` | `<textarea>` element |

### Layout structure
```
<textarea data-slot="textarea" class="... min-h-huge w-full ..." />
```
When `showLabel && id`:
```
<Label id={id} labelElementId={`${id}-label`} className="items-start">
  <textarea data-slot="textarea" ... />
</Label>
```

### Default sizing
- **Min height**: `min-h-huge` (CSS variable)
- **Width**: `w-full`
- **Padding**: `px-tiny py-single`
- **Sizing**: `field-sizing-content` (auto-grows with content)
- **Text**: `text-base` → `md:text-sm`
- **Border**: standard, focus: `focus-visible:border-accent`

### Label interaction note
When wrapped in Label, adds `className="items-start"` to the Label grid — this aligns the label text to the top instead of center (since textarea can be multi-line).

---

## 10. SidePanel (lines ~4994–5135)

### Data-slots
| Slot | Element |
|------|---------|
| `side-panel-tabs` | Tab bar `<div>` |
| `side-panel-tab-button` | Individual tab `<button>` |
| `side-panel-content` | Content area `<div>` |

Also uses `data-panel="leftSidePanel"` or `data-panel="rightSidePanel"` on the outer container (not `data-slot`).

### Layout structure
```
<LevelProvider level="panel">
  <div data-panel="leftSidePanel|rightSidePanel"
       class="absolute text-foreground border bg-panel min-w-0 overflow-hidden flex flex-col"
       style="left|right: var(--spacing-double); top: var(--spacing-double); bottom: var(--spacing-double); width: {size}px; zIndex: {zIndex}">

    <!-- Tab bar -->
    <div data-slot="side-panel-tabs" class="flex items-center h-medium border-b shrink-0 overflow-x-auto">
      {tabs.map → <Tooltip>
        <button data-slot="side-panel-tab-button" class="flex items-center justify-center h-full px-small border-r ...">
          <Icon size={16} />
        </button>
      </Tooltip>}
    </div>

    <!-- Content area -->
    <Scrollable class="flex-1 min-h-0">
      <div data-slot="side-panel-content" class="p-[10px]">
        {activeTab.content}
      </div>
    </Scrollable>

    <!-- Resize handle (if onSizeChange provided) -->
    <div class="absolute top-0 bottom-0 left|right-0 w-single cursor-ew-resize" />
  </div>
</LevelProvider>
```

### Default sizing
- **Position**: `absolute`, inset by `var(--spacing-double)` from edges
- **Default width**: `300px` (via `size` prop, default 300)
- **Min width**: `200px`, **Max width**: `600px`
- **Tab bar height**: `h-medium`
- **Content padding**: `10px` all sides
- **Tab button padding**: `px-small`
- **Icon size**: `16px`
- **z-index**: `20` (default), options: 10/20/30/40
- **Resize handle**: `w-single` (1 CSS unit), full height

### Resize behavior
- Manual `mousedown`/`mousemove`/`mouseup` handling
- Accent border on hover/resize: `border-l-accent` or `border-r-accent`
- Constrained to `[minSize, maxSize]` range

---

# CROSS-CUTTING PATTERNS

## showLabel wrapping pattern

Components that support `showLabel`:
| Component | Always uses Label? | showLabel conditional? | Extra Label className |
|-----------|-------------------|----------------------|----------------------|
| **Input** | No | `showLabel && id` | — |
| **Textarea** | No | `showLabel && id` | `items-start` |
| **Slider** | No | `showLabel` (no id check) | `{className}` passthrough |
| **Stepper** | **Yes, always** | N/A | — |
| **Combobox** | No | `showLabel && id` | `h-medium` |
| **Select** | No | `showLabel && id` | — |

When wrapped in Label, the component becomes a child of the 96px|1fr grid, placed in the `property-control` (1fr) column.

## Interaction fading

Input and Slider support `interactionId`. When one control is "active" (being interacted with), other controls fade to `opacity: 0` via:
```tsx
const shouldFade = activeInteraction && !isInteracting;
<div style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}>
```

## Transaction model

All input components use `useTransaction()` for undo/redo support:
- `transaction.start()` — on focus/pointerdown/open
- `transaction.finalize()` — on blur/pointerup/close/Enter
- `transaction.abort()` — on Escape

## Right panel sizing interaction

When components are placed inside a `SidePanel`, the panel's content area has:
- `p-[10px]` padding on `side-panel-content`
- The panel itself is `flex flex-col` with `min-w-0 overflow-hidden`
- Width is controlled by `size` prop (default 300px)

Components inside the panel must fit within `size - 20px` (10px padding each side). The Label grid's `96px | 1fr` accommodates this: with a 300px panel, the control column gets approximately `300 - 20 - 96 - 8 = 176px`.

The `min-w-0` on both `property-row` and `property-control` prevents grid blowout. The Stepper's fixed `100px` width fits comfortably. The Slider's `minmax(0,1fr)` track adapts to available space.
