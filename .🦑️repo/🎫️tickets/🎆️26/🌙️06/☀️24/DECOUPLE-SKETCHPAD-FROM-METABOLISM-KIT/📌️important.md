# Decouple Sketchpad From Metabolism Kit

## Summary

Sketchpad no longer hardcodes or auto-seeds the metabolism kit. A generic `COMPOSE_SKETCHPAD_PRELOAD_KITS` env mechanism preloads zero or more kits at init. Metabolism is only referenced in test regions and docs MDX content.

## Launch configs

- `🛠️dev🏘️compose✍️sketchpad🧪️metabolism`
- `🛠️dev🏘️compose✍️sketchpad🎛️play🧪️metabolism`

Both set `COMPOSE_SKETCHPAD_PRELOAD_KITS=/fixture/kit/dev/metabolism/wip/initialKit/kit.compose.json`.

## Verification

- Vitest: 92/92 passed
- Fresh `vite build` (js): main bundle has no metabolism fixture URLs, importFixtureKit, or kit id
- Play `dist`: no `metabolism.zip`
