# ViewModel Locale Reaches Guest Measures

## Finding

The WGPU shell computes the correct locale-bearing `ViewModel`, but a Settings `setLocale` action never schedules the guest refresh that consumes it. It only advances a shell-panel maintenance cursor. This leaves existing guest body and Window Measures documents in their prior language.

The checkpoint-20 Window Options dump records that exact split: WGPU presents English `Grid`, `Sun`, and `Intensity` with intensity `1`, while the paired React document presents German `Raster`, `Sonne`, and `Intensität` with `0.85`. The evidence is at [journey-physical.json](./🗑️generated/astra-runtime/checkpoint20-iab/journey-physical.json:1081).

This is a live guest-refresh omission, not an untranslated Puzzle label catalogue or a WGPU measure-projection fallback.

## Causal path

React carries the current host axes into every guest call. [ShellHost](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4809) overwrites the target session's `locale` and `terminology`; its locale/terminology effect then calls `refreshUi(live, { kind: "full" }, undefined, true)` at [5501](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5501). `replaceBodies=true` is intentional because every guest-authored string must be regenerated.

WGPU has the equivalent input construction: [Shell live_view_state](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6852) assigns `active_locale()` and `active_terminology()` before a refresh. A full refresh passes that state to each guest body at [7110](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7110) and to the canonical Measures section at [7202](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7202).

The missing transition is [`setLocale`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10104): after saving the preference it calls `sync_dock_tabs()` and `arm_localized_panel_refresh(generation)`. That cursor explicitly contains only mounted shell-owned leaves ([20377](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:20377)); it cannot reach a guest body or the guest `framework.section.measures` carrier. It also does not call the existing `owe_refresh(UiDirtyScope::Full)` plus `owe_settle()` route used by topology changes.

The stale output persists because `refresh_window_measures` is only run for a scope that requests Measures, and it is the operation that reads `ProgramBridgeEntry::window_measures_section`, retires the transport lease, and republishes each live window overlay ([4590](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4590)). No full scope means no new guest measures exist to project.

Puzzle consumes the supplied axes at the correct place. [puzzle3d_labels](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs:119) admits German and resolves from `ViewModel.locale`; [ArtifactEditor::window_measures_body](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:8310) passes the resolved labels into the main Window Measures producer, which includes Grid and Sun ([main window measures](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:68)).

The present test contract encodes the incorrect behavior:

- [settings-locale-panel-refresh fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️fixtures/🌐️settings-locale-panel-refresh/🔣️.json) says `requiresGuestRefresh: false`.
- Its schema fixes the same value at [schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🌐️settings-locale-panel-refresh/🔣️.json:48).
- The React fixture test asserts false at [test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌐️settings-locale-panel-refresh/🟦️.tsx:39).
- The native `locale_refresh_republishes_one_exact_mounted_shell_owner_per_maintenance_step` law repeats it at [settings-general-layout](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:596). Its wider `host_preference_dispatch_republishes_general_without_a_guest_refresh` loop also includes `setLocale` ([499](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:499)).

## Minimal repair

Use one shared, private shell operation for locale-bearing guest axes:

1. Commit the changed locale or terminology.
2. Retitle the host layout/tabs synchronously.
3. Call the existing `owe_refresh(UiDirtyScope::Full)` and `owe_settle()`.
4. Let the normal bounded settle pass rebuild guest bodies, engagements, measures, and tools from one `live_view_state`.

Apply it to both `setLocale` and `setTerminology`. React's effect has both axes as dependencies, while WGPU's current `setTerminology` only republishes General ([10112](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10112)), so fixing locale alone would preserve the same defect for terminology-specific guest text.

Do not retain `ShellLocalizedPanelRefreshCursor` alongside this full refresh. `refresh_ui` already republishes each shell-owned panel ([7140](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7140)); keeping the old cursor would schedule duplicate replacement of those documents. The correct single owner for a changed locale is the existing full-refresh/retirement lane. This needs no new wire field or app-specific schema.

## Fail-first laws

1. Replace the false locale fixture contract with a neutral guest-refresh scenario. It should contain only:
   - `en → de` ViewModel axes;
   - one guest window body and its Measures section;
   - stable record IDs;
   - English and German authored labels; and
   - scalar `0.85`.

   The test must use the existing test-only `ProgramBridgeEntry::install_fixture_render` seam ([ProgramBridge](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:565)), returning locale-dependent retained documents for both a normal body and `framework.section.measures`. This crosses the same `render_with_document` route as production.

2. Add a native Shell law, for example `locale_change_rebuilds_guest_body_and_window_measures_from_the_live_view_model`:
   - establish accepted English body and Measures documents;
   - dispatch `setLocale(de)`;
   - assert the action itself performs no unbounded render;
   - take one settle refresh and assert exactly one guest refresh observes `ViewModel.locale == de`;
   - require German body/measure labels and the same `0.85` scalar;
   - require the replaced leases to enter and complete the normal retirement lane.

   A matching terminology case must prove the same full-refresh seam is used, rather than only General's shell document changing.

3. Extend the existing real Puzzle guest law [`document_and_kinds_trees_use_german_reuse_section_labels`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:2700) to assert the German Grid/Sun labels in its actual `app.window_measures(&view)` payload and the unchanged default intensity. This isolates guest production localization from the shell bridge.

4. Update the React fixture so it verifies the same guest refresh requirement, then run a mounted React guest oracle that changes Settings language and asserts the re-rendered Measures region includes `Raster` and `Intensität`.

## Intensity default

`0.85` is canonical. [WorldSunConfig::default](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:40599) assigns `intensity: 0.85`; Puzzle's runtime inherits that default, and its Sun producer passes the runtime value directly to `world3d_sun_measures` ([Sun measure](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/☀️sun/🦀️.rs:14)). WGPU's Window Measures projection likewise passes the incoming slider scalar directly to `measure_slider` ([4470](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4470)); it does not synthesize `1`.

Therefore no renderer default should be changed to `1`. The captured WGPU `1` is a second observable mismatch. The native host law above should record the raw decoded `WindowMeasure::Slider` before projection. If it remains `1` after the guest refresh repair, trace the persisted Puzzle window config/guest response; a projection-side default change would mask the producer-state defect.

## Scope

This audit does not prescribe row heights, fixed pitches, or a new visual token. The supplied React measurements use distinct group, select, and checkbox/slider row functions and are outside the locale/guest-refresh causal path.

No source was changed and no test was run.

