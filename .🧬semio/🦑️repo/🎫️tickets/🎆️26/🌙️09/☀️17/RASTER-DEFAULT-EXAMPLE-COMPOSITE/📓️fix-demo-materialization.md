# Raster default example composite fix

## Symptom

Booting raster with the default `demo` example showed an empty composite: layer tree listed Backdrop/Brighten but no pixels rendered.

## Cause

1. `setActiveExample` plants assets only when `raster_asset` finds materialized `local_owner` content. The committed `.dsl.semio` carrier stores wire handles only, so `add-layer-asset` never ran and `semio-emblem` dangled.
2. The demo DSL carried an empty adjustment `params` map, so brightness/contrast never reached the paint host.

## Fix

- Sidecar `🖼️semio-emblem.png` + `art_raster_demo::emblem_image_asset()`.
- `set-active-example::example_media_operations` (remodel frame pattern) emits `add-layer-asset` for the emblem when the store has no materialized pixels yet.
- Demo DSL updated with committed `brightness`/`contrast` parameters.

## Tests

- `default_document_boots_on_the_semio_demo_carrier` — emblem materialized + params present.
- `mounted_boot_replays_the_demo_example_through_the_retained_route` — emblem materialized after boot replay.
