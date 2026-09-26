# DiffView Produced Surface Parity

## Result

DiffView now has an authored Playbook producer and the Settings conflict Tree can relate one selected row to a real DiffView `Surface` direct child. The retained Tree contract carries the relation as `detail`, validates that it targets a direct `Component::Surface`, preserves it through credited copies and TypeScript ownership, and excludes it from disclosure children. WGPU reconciliation rehydrates that Surface as a `UiComponentSceneNode`; layout reserves a full-width 160 px band below row chrome and exposes the scene in the accepted frame.

The Playbook builder mode registers `playbook-changes` / `playbook.play.changes`, renders the current `PlaybookSnapshot` against the empty authored baseline, and includes the window in its default row layout. The shell conflict projection produces one detail only for the selected conflict while preserving the labeled Accept and Discard Button toolbar.

## Shared corpus and oracles

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🆚️diff-view-produced-surface/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🆚️diff-view-produced-surface/🔣️.json`
- The corpus has unified before/after text, the exact equal/remove/add sequence, two conflict states, selected identity, localized action labels, and registered producer identities.
- React validates the schema with Ajv, checks the expected operation sequence against the installed `diff` package, and mounts the actual `DiffViewHost` in unified and split modes.
- Native laws reuse the corpus for the WGPU diff sequence and the accepted conflict detail Surface.

## Focused validation

- PASS: React actual-host and third-party oracle, 1 file / 2 tests.
  - `SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=@semio-tech/framework-renderer-react --excludeTaskDependencies -- bunx vitest run --config <react-config> <diff-view-produced-surface-test>`
- PASS: retained TypeScript Tree toolbar/detail ownership, 1 test / 2 skipped.
  - Same scoped runner against `ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts`, filtered to `requires and owns the explicit Tree inline toolbar and detail relations`.
- Rust source parse completed through scoped Nx `rustfmt --check`; the command exited nonzero only for repository formatting differences and reported no parser diagnostic.
- The first two direct Vitest attempts used a nonexistent config path and then the nested test fragment instead of its owning suite. Both stopped before running tests and were replaced by the passing owning-suite commands above.

## Native laws for the coordinated build

- `tree_detail_relation_validates_and_copies_a_direct_diff_view_surface`
- `selected_conflict_owns_a_real_diff_view_detail_in_the_accepted_frame`
- `neutral_diff_fixture_matches_the_native_operation_sequence`
- `definition_declares_the_diff_view_surface_and_body_key`
- `authored_playbook_projects_to_a_real_diff_view_scene`

