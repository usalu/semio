# Norm command classification

Cause: `view_action` in the shared norm app surface built every window action (viewport camera/layer/playback/touch actions, all `norm.shortcut.*` and `norm.handle.*` shortcut actions, `diagramSelect`, `diagramToggleTable`) with the default `InteractiveJobClassification::Unclassified`. `validate_interactive_job_classification` checks window-kind actions with the window id as owner (`norm-<family>-viewport`), hence the trap in `describe()` for all 15 families.

Fix (one shared source, all 15 families):
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️manifest-and-media/🦀️.rs`: `view_action` now sets `interactive_job = Migrated`.
- Test added in `.../🔌️shell-hooks/🧪️tests/🦀️.rs`: every viewport and diagram window action is classified.

Classification: `Migrated`, the same as the viewer/editor `viewportAct` and the other editor actions in every family. All of these are ephemeral, per-client view changes (or already route into migrated `viewportAct`/entity actions), so they are interactive-safe and need no batch-only or forbidden class.

Checked: app-level actions in all 15 viewer and editor `🦀️.rs` are already classified (viewer 1/1, editor 16/16); no mode commands or app commands exist in norm (`edit_mode_definition` has none); `window_definition` windows (outliner, properties, checks) declare no actions. No fixture records classifications.

Not compiled or run (build pipeline busy).
