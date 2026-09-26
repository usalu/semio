# Playbook Scene Showcase

## Result

The registered Playbook editor app now exposes one tab stack containing real authored routes for the renderer scene hosts requested for paired journeys:

| Window | Body key | Surface |
| --- | --- | --- |
| `playbook-steps` | `playbook.play.steps` | Table |
| `playbook-changes` | `playbook.play.changes` | DiffView |
| `playbook-activity` | `playbook.play.activity` | EventFeed |
| `playbook-source` | `playbook.play.source` | TextEditor |

The existing BlockList builder remains the first tab. The new scenes reuse `PlaybookSnapshot`: the Table lists step identity, title, and block count; DiffView compares the empty authored baseline with the current snapshot; EventFeed presents the authored step order; TextEditor publishes the snapshot wire as read-only JSON. Each window has English and German manifest labels and a normal `ArtifactEditor::render` body-key route.

## Contract and validation

- Neutral schema: `builder/🧬️schema/🎬️scene-showcase/🔣️.json`
- Neutral fixture: `builder/🧫️fixtures/🎬️scene-showcase/🔣️.json`
- Third-party Ajv oracle: PASS, 1 file / 1 test.
- New leaf sources, the mode layout, the artifact module registry, and editor routing all parse and pass `rustfmt --check` through scoped Nx. The artifact/editor root check used `skip_children=true` to avoid reformatting or traversing unrelated concurrent modules.

## Coordinated native laws

- `steps_window_is_a_registered_table_surface`
- `definition_declares_the_diff_view_surface_and_body_key`
- `authored_playbook_projects_to_a_real_diff_view_scene`
- `activity_window_is_a_registered_event_feed_surface`
- `source_window_is_a_registered_text_editor_surface`
- `the_default_tab_stack_lists_every_authored_scene_window`
- `playbook_play_app_declares_the_authored_scene_showcase`

