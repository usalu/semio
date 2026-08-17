# 🪪️ Fix EditorApp/ViewerApp APP_ID Placeholder — Summary

## Root Cause

`VcsArtifactApp::with_registry` used `A::APP_ID` (the compile-time `"surface"` placeholder)
to construct all four per-instance envelope IDs:

- document envelope → `"surface"`
- config envelope → `"surface-config"`
- draft envelope → `"surface-draft"`
- interaction envelope → `"surface-interaction"`

`EditorApp<E>` and `ViewerApp<V>` already compute the real canonical surface app id at
runtime in `Default::default()` and stored it in `self.id`, exposed via `instance_id()`.
The ownership checks in `handle_action_invocation` and `dispatch_command` (fixed in lane
4-G of ticket `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS`) compared
against `instance_id()`, but the underlying stores still carried `"surface"` as their id.

## Changes Made

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`

1. **`VcsArtifactApp::with_registry`** — replaced four `A::APP_ID` references with
   `app.instance_id()` so every envelope gets the real derived id:
   ```rust
   let app_id = app.instance_id();
   let envelope = create_document_envelope::<...>(A::DOCUMENT_SCHEMA, app_id, ...);
   let config_id = format!("{}-config", app_id);
   let draft_id  = format!("{}-draft",  app_id);
   let interaction_id = format!("{}-interaction", app_id);
   ```

2. **`VcsArtifactApp` struct** — promoted `config_store` and `draft_store` to
   `pub(crate)` (consistent with `store`, `presence_store`, `transient_store`) so
   tests can assert on their envelope IDs directly.

3. **`EditorApp<E>` doc comment** — updated to reflect that `instance_id()` is now used
   by `VcsArtifactApp` for envelope construction (no longer a future TODO).

4. **Two new unit tests** in `surface_testkit_tests`:
   - `editor_app_envelopes_carry_the_real_canonical_surface_app_id`
   - `viewer_app_envelopes_carry_the_real_canonical_surface_app_id`

   Both assert that `store.envelope().id`, `config_store.envelope().id`,
   `draft_store.envelope().id`, and `interaction_store.envelope().id` all equal the
   canonical `surface_app_id(dialect, role)` — not `"surface"`.

## Verification

| Suite | Result |
|---|---|
| `cargo test -p semio-framework-plugin --lib surface_testkit_tests` | **8/8 ok** |
| `cargo test -p semio-s-plugin-space --lib` | **210/210 ok** |

No regressions. Both new tests pass on the first run.
