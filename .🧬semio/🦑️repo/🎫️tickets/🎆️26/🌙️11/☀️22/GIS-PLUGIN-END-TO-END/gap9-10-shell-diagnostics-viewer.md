# Gaps 9 and 10: shell diagnostics and the Terrain Viewer role

Static work only; nothing was built or run.

## Gap 9: `no-operator-graph`

- No source line emits `no-operator-graph` any more. The only hits are ticket notes. The string came from a temporary
  `[DEBUG] contributions document sources` log in the old contributions publisher. That log carried
  `reason: "no-operator-graph"` when the receiver's document resolved no operator graph.
- Ticket `08/28/DEMONSTRATOR-END-TO-END-ALL-APPS/fix-2026-09-16-...` already stated that "no operator graph" is the normal
  state for documents that have no operator kinds, and it removed the early return.
- The publisher (`engine/elements/ShellHelpers/contributions`) now cuts the pack from the receiver's `consumes` row and the
  loaded plugin closure alone. It no longer reads a document operator graph.
- GIS therefore does not need to declare an operator graph. The diagnostic was wrong for contributions that legitimately
  have none, and the code that emitted it is gone.
- Remaining stale artefact: `BENIGN_RE` in `framework/modules/time-travel/testing/console` still whitelisted
  `contributions document sources`, a message nothing emits. It was removed so the console digest does not hide a regression.
- The 08/29 verification note is a historical observation from before that refactor. Gap 9 is closed as stale.
  The clean-console check on port 6040 still needs a live run.

## Gap 10: Terrain Viewer role

How roles reach the shell:

1. `hub/compositions/gis/rs` builds the plugin with `.editor::<Gis3dPlayApp>(create_gis3d_app())` and
   `.viewer::<GisTerrainViewer>(create_gisterrain_viewer())`. The viewer is also in the closed `GisApps` fleet.
2. The plugin manifest lists `apps`, one per `<dialect>#<role>`.
3. The shell navbar (`ShellHost/command-bar/scope` and `surface-switch`) shows the role group only when
   `surfaceRoleAppsV1(manifest.apps, focusedApp.dialect)` finds both an editor and a viewer whose dialect
   coordinates are equal. The role click then calls `switchToPluginApp(pluginId, target.id)`.

Findings:

- Registration, manifest and dialect are correct. The Terrain viewer is built exactly like the working `process3d` and
  `gismap` viewers (`Viewer::builder(DIALECT)`, one mode, one window, a default layout). The editor and viewer share
  `GISTERRAIN_DIALECT`.
- The existing law `gis_component_assembly_declares_exact_package_identity_before_descriptor_emission` already asserts one
  app per `<dialect>#<role>` and four apps in total.
- The dev-preview note says the navbar Viewer button was present and clickable, and that direct `?role=viewer` mounts.
  That shows the role is published. What timed out was the surface mount that `verify-s-shell-terrain.mjs` waits for
  (`[data-surface-app-id=...#viewer]` containing `.semio-world-3d-host`). The same note records that the running
  component on 6083 was a stale published build while the replacement compiled, and that guest body refresh hit a WASM
  trap and a scrub-ledger stall. Those were fixed afterwards.
- I found no static defect that withholds the Viewer role.
- One residual risk, not changed: the viewer window's scene sets `domain_id = "features"` and `domain_granularity_id = "pin"`,
  but the viewer manifest declares no interaction `features`. The editor declares it through `window_kind_interactions`.
  This affects hover and selection dispatch only, not mounting.

Change: added the Rust law `gis_manifest_publishes_one_editor_viewer_pair_per_artifact_dialect`, which pins the exact
condition the navbar uses (one editor and one viewer per native dialect, equal dialect coordinate).

## Files changed

- `framework/modules/time-travel/testing/console/ts` (removed the dead `contributions document sources` alternative from `BENIGN_RE`)
- `hub/compositions/gis/tests/surface/rs` (new test)

## Tests to run

- Rust: `gis_manifest_publishes_one_editor_viewer_pair_per_artifact_dialect`. Related existing tests:
  `gis_component_assembly_declares_exact_package_identity_before_descriptor_emission`,
  `gisterrain_editor_and_viewer_share_dialect`, `create_gisterrain_viewer_builds_a_definition_for_the_viewer_role`.
- Live: `verify-s-shell-terrain.mjs` Editor, Viewer, Editor switch against a freshly rebuilt GIS component.
  The console digest test `console-digest` is also worth running.
