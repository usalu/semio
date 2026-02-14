# Ticket

## Todos

- Locate repo sources for the provided element classnames.
- Map Tailwind utilities to concrete CSS properties.
- Resolve CSS variable definitions for `--active-base` / `--active-foreground`.
- Write per-element style summaries.

## Changes

- No code changes.

## Log

### Element: `span.flex.size-full.items-center.justify-center.rounded-full.text-xs.bg-[color:var(--active-base)].text-[color:var(--active-foreground)]`

This element matches the Radix Avatar fallback in [js/semio/sketchpad/elements.tsx](js/semio/sketchpad/elements.tsx#L790) (it renders `AvatarPrimitive.Fallback`, which is a `span`), composed as:

- Base classes from `AvatarFallback`: `bg-muted flex size-full items-center justify-center rounded-full`
- Additional classes passed by callers (example: `TableAvatar` adds `text-xs` and may add the active colors) in [js/semio/sketchpad/elements.tsx](js/semio/sketchpad/elements.tsx#L858-L875)

Tailwind/class → CSS properties (effective intent):

- `flex`
	- `display: flex;`
- `size-full`
	- `width: 100%;`
	- `height: 100%;`
- `items-center`
	- `align-items: center;`
- `justify-center`
	- `justify-content: center;`
- `rounded-full`
	- `border-radius: 9999px;`
- `text-xs`
	- `font-size: var(--text-xs);`
	- `line-height: var(--text-xs--line-height);`
	- Note: `--text-xs` / `--text-xs--line-height` are defined in [js/semio/globals.css](js/semio/globals.css#L280-L310).
- `bg-muted`
	- `background-color: var(--muted);`
- `bg-[color:var(--active-base)]`
	- `background-color: var(--active-base);` (overrides `bg-muted` when present)
- `text-[color:var(--active-foreground)]`
	- `color: var(--active-foreground);`

CSS variables used:

- Light theme values in [js/semio/globals.css](js/semio/globals.css#L320-L356)
	- `--active-base: var(--color-primary);`
	- `--active-foreground: var(--color-dark);`
- Dark theme values in [js/semio/globals.css](js/semio/globals.css#L572-L606)
	- `--active-base: var(--color-primary);`
	- `--active-foreground: var(--color-light);`

What I cannot truthfully “show” from repo alone:

- The final computed color numbers for `--color-primary`, `--color-dark`, `--color-light` depend on where those are defined (they’re ultimately derived from the theme layer), and also on whether `.dark` is present on the document.

### Element: `path.react-flow__edge-path.transition-colors.duration-200`

This is a React Flow / XYFlow edge path.

Base library CSS for `.react-flow__edge-path` comes from [node_modules/@xyflow/react/dist/style.css](node_modules/@xyflow/react/dist/style.css#L108-L116):

- `stroke: var(--xy-edge-stroke, var(--xy-edge-stroke-default));`
- `stroke-width: var(--xy-edge-stroke-width, var(--xy-edge-stroke-width-default));`
- `fill: none;`

The `transition-*` classes are added by our custom edges (via `BaseEdge className="transition-colors duration-200"`) in:

- Kit diagram edge component: [js/semio/sketchpad/Kit.tsx](js/semio/sketchpad/Kit.tsx#L5085-L5130)
- Design diagram edge component: [js/semio/sketchpad/Design.tsx](js/semio/sketchpad/Design.tsx#L4915-L4963)

Edge color is primarily driven by inline style props passed to `BaseEdge`:

- Kit: `stroke` is set to `var(--foreground)` / `var(--accent-secondary)` / `var(--active-base)` etc. in [js/semio/sketchpad/Kit.tsx](js/semio/sketchpad/Kit.tsx#L5085-L5125)
- Design: `stroke` is set similarly based on diff/selection in [js/semio/sketchpad/Design.tsx](js/semio/sketchpad/Design.tsx#L4915-L4960)

Tailwind/class → CSS properties (the part contributed by Tailwind):

- `transition-colors`
	- `transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;`
	- `transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);` (Tailwind default)
- `duration-200`
	- `transition-duration: 200ms;`

Repo override (only if an ancestor has class `temp`):

- [js/semio/globals.css](js/semio/globals.css#L541-L551)
	- `.temp .react-flow__edge-path { stroke: #e98787; stroke-dasharray: 5 5; }`

### Element: `div` (no classes provided)

With only `div` and no attributes/className/style, the only guaranteed styling is the UA/defaults:

- `display: block;` (typical)

To “show all styling properties” for that `div`, I need at least one of:

- its `class` list,
- any inline `style="..."`,
- its `id` (if it’s targeted in CSS),
- or the component/source location that renders it.

## Summary

Documented styling-property mappings for the provided elements (AvatarFallback span and ReactFlow edge path), including CSS variable resolution.
