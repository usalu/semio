# Notes

## Bug

`DragHandle` paints `text-muted-foreground` at rest. Labels/icons on the same button inherit the parent’s `hover:text-emphasized`, but the grip’s own color blocks inheritance — so hovering a panel/pane/mode-dock button emphasized the label/icon while the trailing grip stayed muted.

## Fix

CSS in `ui.css`:

```css
[data-hover-scope]:hover [data-slot="drag-handle"] {
  color: var(--border-emphasized-color);
}
```

Uses the existing `data-hover-scope` contract (already on panel tabs, pane chrome toggles, mode-dock tabs, tree rows). Nested tree children stay siblings of the header hover-scope, so parent grips do not light up while hovering a child row.
