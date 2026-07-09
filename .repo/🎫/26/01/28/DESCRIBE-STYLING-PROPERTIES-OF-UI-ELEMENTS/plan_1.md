1. Locate the source of each element’s styling (Tailwind utilities, component props, library CSS).
2. Resolve CSS variables used by the classes (e.g. `--active-base`, `--active-foreground`) by finding their definitions in repo CSS.
3. Produce a property-level map for each element: `class → CSS properties`, including transitions, sizing, alignment, typography, colors.
4. Write the findings into `ticket.md` (and only reference what can be derived from the repo; note what requires browser computed styles).
