# Current Native Source Audit

## Fixture Completion Route

The shared helper at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6328` is current and live. `project_and_retire_fixture_tree` projects the root under `catch_unwind`, then `observe_and_retire_fixture_tree` constructs `BuiltTreeRetirement` from the same owned root and drains it until terminal before returning or resuming the captured panic.

Both GIS test kits use this route exactly once for each fixture render:

- Map: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs:55`.
- Terrain: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs:41`.

Each passes `app.render(body_key, None, &main_window_view())` into the helper. The view binds the real main window instance, and the searches found no direct render-tree projection bypass in either artifact. No stale Map/Terrain completion route was found.

## Stale Raw Route

`owned-23-standalone-default-current.txt` and `owned-24-parent-composition-current.txt` preserve `E0382` at reactor-turn line 811: a borrow of moved `events`. Current source at that line contains no `events` expression; it aggregates `lifecycle_work` into `more_work`. The raw error is historical evidence only. A fresh consumer compile is required to determine the current shared-plugin outcome.
