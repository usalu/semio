## Root cause

Engagement suggestion popover called `preventDefault()` on every `pointerDown`, which suppressed `click` on `cmdk` items. Possibles were also unsorted, so `Extr` always favored `ExtrudeWire` (catalog order) over `ExtrudeCrv`.

## Fix

- `ui/react/index.tsx`: rank `filterEngagementPossibles`; skip `preventDefault` for command rows (including text-node targets); `stopPropagation` on item pointer down capture; sync hover index; tests.
