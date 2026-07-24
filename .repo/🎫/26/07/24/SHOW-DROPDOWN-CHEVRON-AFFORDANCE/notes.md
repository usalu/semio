# Notes

## Problem

Playground example dropdown (`NavbarExampleSelect` → `SelectTrigger`) and other selects looked like plain buttons: no visible trailing chevron.

## Cause

1. `SelectValue` lacked `min-w-0` / `flex-1`, so long labels pushed the chevron outside fixed-width triggers (often clipped by chrome).
2. Chevron used `opacity-50`, which made the affordance easy to miss on chrome backgrounds.

## Fix

- Reserve space for the value (`min-w-0 flex-1 overflow-hidden`) and keep the trigger `min-w-0`.
- Render a dedicated `data-slot="select-chevron"` trailing icon without fade opacity.
- Combobox/`ButtonGroupItem` labels with `justify-between` now `flex-1 truncate` so trailing icons are not clipped by `overflow-hidden`.

## Verify

```bash
cd ui/js/react && bunx vitest run -t "keeps a trailing chevron affordance"
```

Passed (1 test).
