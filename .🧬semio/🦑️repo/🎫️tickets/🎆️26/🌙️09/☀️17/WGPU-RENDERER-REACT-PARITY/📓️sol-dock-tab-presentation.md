# Dock Instance Tab Presentation Parity

## Evidence

The paired runtime captures show React painting the authored **Top** tab as globally active with the red active fill and using the projection-specific grid/triangle icons. WGPU painted both tabs with the inactive fill and reused the window kind's `puzzle` icon.

The WGPU shell booted with the window kind id (`puzzle3d-main`) as its active id even though the authored dock contains only instance ids (`puzzle3d-main-top` and `puzzle3d-main-perspective`). `DockState::sync_active_window` therefore could not resolve an `active_stack`, and `render_stack` correctly withheld the global active-tab fill.

WGPU already owns the live projection state in `world_projection_template`. It is seeded from each authored layout instance, updated when a projection row is selected, retained only for live dock windows in `sync_dock`, and removed during World3d owner retirement. The tab chrome previously ignored this instance state and read only `WindowKindDefinition.icon_id`.

## Repair

- `ShellState::sync_dock` now applies the existing shared `dock_seed_active_window_id_v1` law after every layout reconciliation. An active kind id absent from the concrete layout becomes the first authored active instance, so `active_window_id` and `active_stack` agree before paint and input publication.
- `dock_tab_icon_id_v1` maps a valid live projection template/spec to its authored `projection-*` icon and falls back to the declared kind icon for ordinary windows or malformed template data.
- `dock_chrome_maps` now resolves the per-instance override from the live projection map. No second icon authority or layout-only compatibility state was introduced.
- The shared language-neutral window-scope fixture now includes orthographic, three-point, ordinary, and malformed tab-presentation rows. The TypeScript oracle validates the corpus with Ajv and computes the React answer through `decodeWorldProjectionTemplateId` plus `worldProjectionSpecIconId`.

## Verification

- PASS: `NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:window-scope-check --skip-nx-cache`
  - 1 file, 1 test
  - 4 tab-presentation rows
- PASS: `rustfmt --edition 2021` through `bun nx exec` on both modified Rust files, providing a syntax parse check.
- Native laws awaiting the coordinator's canonical grouped run:
  - `projection_instance_tab_icons_match_reacts_ephemeral_overrides`
  - `dock_sync_seeds_the_authored_instance_and_publishes_its_projection_icons`

No browser runtime claim is made here; the coordinator owns the rebuilt paired browser journey.
