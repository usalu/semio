# Staffage React Render Lag

Goal: `🎯updatedtutorials`.

## Changes

- Catalog category chunks load with `Promise.all`. `validateAsset` is cached in a `WeakMap` for the session. `CatalogStore.#run` yields and publishes category progress every 8 assets, calls `progress` on every asset, and rebuilds `rows` only when the loaded set changes.
- `StaffageClock.#tick` keeps `#state` current on every scheduler tick and notifies listeners only when `frame` or `playing` changes. `seek`, `step`, `play`, `pause`, `setRate`, `setLoop`, and `setDuration` still notify immediately. `useClock` documents that contract.
- Scene cache capacity is 512. `sceneOf` memoises `DrawingScene` per `EvaluatedState` and depiction options. `RowPreview` wraps the ambient clock into the default looping clip so card previews repeat a finite frame set. Compare cards pass a static pose. Line and solid `PlacedFigure`s evaluate the wrapped cycle and keep the unwrapped query for `travelledM`. Off-screen placed figures freeze the last frame.
- `StaffageFigure` skips `valuedItems` and `StaffageValueTable` when the legend is off and nothing inspects the figure.
- Overview category cards on a compact layout load only once revealed, on a longer stagger, and preview two figures. Phone root and category levels mount one background page. The viewer depicts `dark-line` plus the active view. Browse cards use `content-visibility: auto`.
