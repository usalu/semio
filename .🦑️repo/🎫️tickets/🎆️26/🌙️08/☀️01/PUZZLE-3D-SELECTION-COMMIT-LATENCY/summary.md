# Puzzle 3D Selection Commit Latency

Eliminated post-marquee selection stall by skipping full-document serde/diff on view-only actions, shrinking inspector WASM payloads, and making 3D instance chrome subscribe per-row instead of re-rendering the whole layer.

## Root cause

`worldSelect` paid `projection.0.clone()` plus `puzzle3d_operations_from_fixture_change` (full fixture round-trip) even though the document never changed. Inspector refresh duplicated the full selected-id array into every field action and serialized twice for hashing.

## Mechanisms

1. **Action intent** — `puzzle3d_action_document_intent` skips `before` clone and fixture delta for view actions (`worldSelect`, etc.).
2. **SelectionSet** — framework `SelectionSet` with O(1) `contains` and serde-transparent JSON arrays.
3. **Implicit inspector targets** — `patchInspector` resolves ids from `runtime.selection`; field actions carry only `entity` + `field`.
4. **Serialize once** — `ui_refresh_section` hashes a single `to_string` pass.
5. **WorldInstanceChromeStore** — per-instance `useSyncExternalStore` chrome; shared per-mesh pick/edge geometry memos; memoized `WorldInstanceNode`.
6. **Tick yield** — `fillBuildTick` / `suggestionsTick` skip while interactive plugin actions are in flight.
7. **View actions** — `finish_recorded` no longer widens scope to the history panel for `ActionKind::View`.

## Verification

- `cargo test -p semio-s-app-puzzle-3d-ui world_select_emits`
- `cargo test -p semio-s-app-puzzle-3d-ui inspector_field_actions`
- `cargo test -p semio-framework-plugin selection_set_membership`
