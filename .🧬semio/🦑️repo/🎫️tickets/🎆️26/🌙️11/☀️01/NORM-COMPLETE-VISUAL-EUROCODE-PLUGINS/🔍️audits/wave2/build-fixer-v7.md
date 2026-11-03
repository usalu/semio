# build-fixer v7: app-surface-panels-inspection (en1991 run 16)

Root causes: (1) `structural_scene` lib exposes `projection_kit` as a module, `base_id`/`GLOBAL_BADGE` are not at the crate root (defined in `projection-kit/identity` and `projection-kit/highlight`, re-exported via `projection_kit::*`). Not a warnings-cleanup deletion. (2) The inspection lib mounted `viewport_panel_tests` un-gated; that test file belongs to the panels crate (which owns `toolbar`, `lane_overlays` and declares the same module under `#[cfg(test)]`).

| # | file:line | error | cause | fix |
|---|---|---|---|---|
| 1,2 | app-surface/🔗️check-entity-link/🦀️.rs:11 | E0432 `base_id`, `GLOBAL_BADGE` not in root | items live in `projection_kit` | import from `semio_s_plugin_norm_structural_scene::projection_kit::{base_id, GLOBAL_BADGE}`, `StructuralScene` separately |
| 3 | app-surface/🪟️visual-windows/✅️checks/🦀️.rs:208 | E0425 `base_id` | same | path `...::projection_kit::base_id` |
| 4,5 | app-surface/🪄️selection-mirror/🦀️.rs:32 (x2) | E0425 `base_id` | same | path `...::projection_kit::base_id` |
| 6 | app-surface/🪄️selection-mirror/🦀️.rs:45 | E0425 `base_id` | same | same |
| 7 | app-surface/🪄️selection-mirror/🦀️.rs:46 | E0425 `GLOBAL_BADGE` | same | path `...::projection_kit::GLOBAL_BADGE` |
| 8-12 | tests/🔬️viewport-panels/🦀️.rs:3,4,183-186 (E0432/E0433 `crate::lane_overlays`, `crate::toolbar`, `artifact_app_laws`) | test file compiled in lib build of inspection crate | `mod viewport_panel_tests;` at end of panels/🔎️inspection/🦀️.rs was un-gated and in the wrong crate (`artifact_app_laws` needs `cfg(test)`/`artifact-app-testing`) | removed the mount from the inspection lib; panels/🦀️.rs already mounts it under `#[cfg(test)]` |

Also fixed proactively: tests/🔬️live-acceptance/🦀️.rs:230 `use ...::{base_id, ...}` -> `projection_kit::base_id` (same cause, would fail en1990/1992/1995 test builds).

Scan of other app-surface crates: no other `structural_scene::base_id|GLOBAL_BADGE` root uses; no other `crate::toolbar|lane_overlays` users outside panels; all other `#[path = ".../tests/..."]` mounts are `#[cfg(test)]`; remaining `structural_scene::X` root symbols (EntityRef, Handle, Interval, ChoiceToken, QuantityKind, canonical_json, colour_maps) resolve through foundation/schema re-exports. `artifact_app_laws` lives behind `cfg(any(test, feature = "artifact-app-testing"))` in the framework plugin; all app-surface crates dev-depend with that feature.
Not run: cargo/rustc (forbidden).
