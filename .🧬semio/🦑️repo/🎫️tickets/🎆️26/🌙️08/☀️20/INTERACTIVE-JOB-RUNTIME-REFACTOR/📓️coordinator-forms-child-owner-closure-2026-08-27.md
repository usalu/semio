# Forms Child Owner Closure

## Scope

Removed the Forms artifact's process-global `FORMS_SCRATCH` working-scene cache and transferred each decoded or constructed step tree into the exact `FormsStructureChild` owner.

## Exact changes

- `FormsWorkingScene` is a cloneable typed local owner.
- `materialize_forms_steps` attaches a decoded/test scene to one mutable structure child.
- `forms_children_from_steps` attaches one scoped `Arc<FormsWorkingScene>` to the returned structure/results pair.
- `forms_scene`, `forms_steps`, and `forms_artifact_steps` resolve only the addressed child's local owner.
- The mutation-vector bridge now requires a mutable snapshot and materializes its exact structure child.
- Existing mutation tests were updated to materialize their own snapshots; the removed cache API has no compatibility alias.
- The language-neutral fixture `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🧪️fixtures/🎯️child-owner-isolation.json` is checked by a serde_json oracle: an owned handle has a scene, its wire identity round-trips exactly, and the reconstructed wire-only handle has no scene.

## Validation

- Bun fixture parse and exact booleans: green.
- Removed-symbol/source scan: `FORMS_SCRATCH`, `cache_forms_steps`, `RefCell`, and `HashMap` absent from the artifact component.
- All eight mutation test materialization sites hold mutable snapshots.
- `git diff --check -- '✏️s/🔌️plugins/📋️forms'`: green.
- Rust compiler/rustfmt validation is deliberately queued behind Flow's exclusive compiler lease; this report does not claim it yet.
