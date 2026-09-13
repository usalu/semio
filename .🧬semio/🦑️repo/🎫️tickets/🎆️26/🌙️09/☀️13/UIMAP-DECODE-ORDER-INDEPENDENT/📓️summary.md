# UiMap Decode Order Independence

## Problem

`UiMapBuilder::push` required strictly ascending keys. `UiMap`'s `Deserialize::visit_map` surfaced a single opaque error (`UiMap requires at most N ascending unique entries`), so JSON objects emitted by `JSON.stringify` in the wgpu bridge (numeric string keys first, then `"01".."09"`) failed to parse while React readers sorted keys client-side.

## Fix

- `UiValueArena::search_map_key`, `map_prev_at_position`, and `try_insert_map_page` splice a new arena page into the map's sorted singly-linked chain without buffering into a `Vec`.
- `UiMapBuilder::try_insert`, `contains`, and `refusal` mirror `UiFixedMap`'s decoder contract.
- `Deserialize for UiMap` admits entries in any wire order; duplicate keys and capacity/page-grant refusals keep distinct messages.
- `push` delegates to `try_insert`.

## Tests

Extended `catalogue_carrier_map` integration tests with `UiMap` laws over `🛍️catalogue-carrier-map.json`.

## Verification

Run: `cargo test -p semio-framework-ui-contract`
