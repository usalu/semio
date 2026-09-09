# Complete WASI 958 Diagnostics

{
  "counts": {
    "clippy::vec_init_then_push": 3,
    "clippy::too_many_arguments": 23,
    "clippy::type_complexity": 3,
    "clippy::unnecessary_wraps": 17,
    "clippy::unnecessary_lazy_evaluations": 1,
    "clippy::manual_is_multiple_of": 3,
    "clippy::derivable_impls": 10,
    "clippy::field_reassign_with_default": 1,
    "clippy::map_unwrap_or": 2,
    "clippy::redundant_closure": 1,
    "clippy::obfuscated_if_else": 1,
    "error": 5,
    "clippy::needless_update": 16,
    "clippy::explicit_auto_deref": 2,
    "clippy::iter_skip_next": 1,
    "clippy::needless_pass_by_value": 17,
    "E0432": 3,
    "unused_imports": 1,
    "clippy::unnecessary_map_or": 2,
    "clippy::map_identity": 6,
    "E0277": 19,
    "clippy::large_enum_variant": 13,
    "failure-note": 4,
    "clippy::question_mark": 1,
    "clippy::needless_range_loop": 6,
    "clippy::collapsible_if": 1,
    "clippy::enum_variant_names": 1,
    "clippy::manual_map": 1,
    "E0425": 2,
    "unused_qualifications": 2,
    "clippy::manual_clamp": 1,
    "clippy::result_large_err": 24,
    "clippy::semicolon_if_nothing_returned": 1,
    "clippy::empty_line_after_doc_comments": 1
  },
  "packages": {
    "semio-s-artifact-norm-en1993": 1,
    "semio-s-artifact-puzzle-3d": 17,
    "semio-s-artifact-block-2d": 2,
    "semio-s-artifact-space-space": 2,
    "semio-s-artifact-vcs-vcs": 3,
    "semio-s-artifact-demonstrator-playground": 6,
    "semio-framework-os-flow": 2,
    "semio-s-artifact-sourcing-curation": 2,
    "semio-s-artifact-trinity-jack": 24,
    "semio-s-artifact-process-process3d": 5,
    "semio-s-artifact-cad-cad": 2,
    "semio-s-artifact-gis-gismap": 5,
    "semio-s-artifact-fem-2d": 16,
    "semio-s-artifact-procedural-assembly": 5,
    "semio-s-artifact-norm-en1991": 1,
    "semio-s-artifact-dag-dag": 2,
    "semio-s-artifact-note-note": 4,
    "semio-s-artifact-imperative-procedure": 1,
    "semio-s-artifact-layout-layout": 6,
    "semio-s-artifact-remodel-remodeling": 7,
    "semio-s-artifact-mathematical-equation": 7,
    "semio-s-artifact-reasoning-wires": 10,
    "semio-s-artifact-shooting-shooting": 4,
    "semio-s-artifact-sequence-sequence": 1,
    "semio-s-artifact-lowpoly-lowpoly": 1,
    "semio-s-artifact-energy-model": 29,
    "semio-s-artifact-raster-raster": 5,
    "semio-s-artifact-norm-din18599": 2,
    "semio-s-artifact-animate-presentation": 7,
    "semio-s-artifact-draw-drawing": 14,
    "semio-s-artifact-architect-program": 2
  }
}

error: calls to `push` immediately after creation
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:109:9
    |
109 | / ...   let mut mutations = Vec::with_capacity(17);
110 | | ...   mutations.push(En1993Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: snapshot.annex }));
111 | | ...   mutations.push(En1993Mutation::UpdatePileInputs(update_pile_inputs::UpdatePileInputs { new_pile_sigma_mpa: snapshot.pile_si...
112 | | ...   mutations.push(En1993Mutation::UpdateWeldInputs(update_weld_inputs::UpdateWeldInputs {
...   |
204 | | ...       new_stainless_f_y_mpa: snapshot.stainless_f_y_mpa,
205 | | ...   }));
    | |__________^ help: consider using the `vec![]` macro: `let mutations = vec![..];`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#vec_init_then_push
    = note: `-D clippy::vec-init-then-push` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::vec_init_then_push)]`


error: this function has too many arguments (8/7)
    --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2740:5
     |
2740 | /     fn handle_action_impl(
2741 | |         &self,
2742 | |         action: &str,
2743 | |         args: Option<&Value>,
...    |
2748 | |         interaction: &Puzzle3dInteractionSnapshot,
2749 | |     ) -> (Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, EphemeralEmit<EditorApp<Puzzle3dPlayApp>>) {
     | |____________________________________________________________________________________________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
     = note: `-D clippy::too-many-arguments` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this function has too many arguments (8/7)
    --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2846:5
     |
2846 | /     pub(crate) fn dispatch_step(
2847 | |         &mut self,
2848 | |         app: &Puzzle3dPlayApp,
2849 | |         action: &str,
...    |
2854 | |         interaction: &Puzzle3dInteractionSnapshot,
2855 | |     ) -> (Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, EphemeralEmit<EditorApp<Puzzle3dPlayApp>>) {
     | |____________________________________________________________________________________________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


error: very complex type used. Consider factoring parts into `type` definitions
    --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2952:46
     |
2952 | ...ion: &str) -> Option<(Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, EphemeralEmit<EditorApp<Puzzle3dPlayApp>>)> {
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity
     = note: `-D clippy::type-complexity` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::type_complexity)]`


error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3140:1
     |
3140 | / fn puzzle3d_retained_reduce_in_session(
3141 | |     session: Option<(u32, Option<String>)>,
3142 | |     command: &Puzzle3dCommand,
3143 | |     snapshot: &Puzzle3dPlaySnapshot,
...    |
3147 | |     view_state: Option<&semio_framework_plugin::ViewModel>,
3148 | | ) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, Fault> {
     | |__________________________________________________________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
     = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
     |
3148 - ) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, Fault> {
3148 + ) -> semio_framework_plugin::Emit<standards::v1::subsets::any::schema::mutations::component::Puzzle3dMutation, editor::puzzle3d::config::component::Puzzle3dConfigMutation> {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
3151 ~         return Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(120), dialog_id: "addObject".into(), args: None });
3152 |     }
3153 |     if command.action_id() == "worldPointerDown" {
3154 ~         return Emit::default();
3155 |     }
3156 |     if command.action_id() == "addTargetVolume" {
3157 ~         let Some(origin) = command.args().and_then(|args| args.get("origin")).and_then(value_as_vec3) else { return Emit::default() };
3158 |         let grid_spacing = runtime.grid_spacing.max(0.1);
 ...
3163 |         let volume = crate::Puzzle3dTargetVolume { id, origin: snapped, orientation: None, scale: Some(scale), hidden: false, locked: false };
3164 ~         return Emit { artifact_mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_target_volume(volume, None)], ui_scope: UiDirtyScope::Full, ..Default::default() };
3165 |     }
3166 |     let snapshot_interaction = Puzzle3dInteractionSnapshot::from_state(interaction, hover);
3167 ~     with_puzzle3d_app_for(session, &runtime, |app| app.handle_action_impl(command.action_id(), command.args(), command.window_id(), snapshot, &runtime, view_state, &snapshot_interaction).0)
     |


error: unnecessary closure used with `bool::then`
    --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:6115:44
     |
6115 | ... = (self.requested_count != config.fill_count).then(|| Puzzle3dConfigMutation::SetFillCount { count: self.requested_count }).in...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_lazy_evaluations
     = note: `-D clippy::unnecessary-lazy-evaluations` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::unnecessary_lazy_evaluations)]`
help: use `then_some` instead
     |
6115 -                     let config_mutations = (self.requested_count != config.fill_count).then(|| Puzzle3dConfigMutation::SetFillCount { count: self.requested_count }).into_iter().collect();
6115 +                     let config_mutations = (self.requested_count != config.fill_count).then_some(Puzzle3dConfigMutation::SetFillCount { count: self.requested_count }).into_iter().collect();
     |


error: this function has too many arguments (9/7)
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs:402:5
    |
402 | /     fn begin_measurement(
403 | |         &mut self,
404 | |         job: u64,
405 | |         operation: Operation,
...   |
411 | |         observation: FillObservation,
412 | |     ) -> Result<FillJobRequest, FillEnvelopeMeasurementOwners> {
    | |______________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


error: manual implementation of `.is_multiple_of()`
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs:940:12
    |
940 |         && positions.len() % 3 == 0
    |            ^^^^^^^^^^^^^^^^^^^^^^^^ help: replace with: `positions.len().is_multiple_of(3)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_is_multiple_of
    = note: `-D clippy::manual-is-multiple-of` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::manual_is_multiple_of)]`


error: manual implementation of `.is_multiple_of()`
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs:941:12
    |
941 |         && indices.len() % 3 == 0
    |            ^^^^^^^^^^^^^^^^^^^^^^ help: replace with: `indices.len().is_multiple_of(3)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_is_multiple_of


error: this `impl` can be derived
    --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs:1010:1
     |
1010 | / impl Default for BrushIndexSyncStage {
1011 | |     fn default() -> Self {
1012 | |         Self::Objects
1013 | |     }
1014 | | }
     | |_^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
     = note: `-D clippy::derivable-impls` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute and mark the default variant
     |
 991 + #[derive(Default)]
 992 | enum BrushIndexSyncStage {
 993 ~     #[default]
 994 ~     Objects,
     |


error: this function's return value is unnecessary
    --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs:1357:5
     |
1357 |     pub(crate) fn set_scene_config(&mut self, scene: SceneConfig) -> Result<(), Puzzle3dError> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove the return type...
     |
1357 -     pub(crate) fn set_scene_config(&mut self, scene: SceneConfig) -> Result<(), Puzzle3dError> {
1357 +     pub(crate) fn set_scene_config(&mut self, scene: SceneConfig) -> () {
     |
help: ...and then remove returned values
     |
1378 ~             return ;
1379 |         }
1380 |         if self.scene_synced.as_deref() == Some(&scene) {
1381 ~             return ;
1382 |         }
 ...
1386 |         self.rebuild_queue();
1387 ~         
     |


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs:313:5
    |
313 |     fn quoted(&mut self, preview: &FillBuildPreview, color: &str, status_label: &str, prefix: &'static [u8], source: FillPreviewString, optional: bool, advance: bool) -> Result<FillPreviewJsonUnit, ()> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


error: very complex type used. Consider factoring parts into `type` definitions
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs:156:11
    |
156 |     page: Option<Box<[Option<(K, V)>; N]>>,
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity


error: this function's return value is unnecessarily wrapped by `Option`
    --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs:1066:1
     |
1066 | fn collision_index_backing_credit(credit: Option<(usize, usize)>) -> Option<(usize, usize)> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Option` from the return type...
     |
1066 - fn collision_index_backing_credit(credit: Option<(usize, usize)>) -> Option<(usize, usize)> {
1066 + fn collision_index_backing_credit(credit: Option<(usize, usize)>) -> (usize, usize) {
     |
help: ...and then remove the surrounding `Some()` from returning expressions
     |
1067 -     Some(credit.unwrap_or((0, 0)))
1067 +     credit.unwrap_or((0, 0))
     |


error: field assignment outside of initializer for an instance created with Default::default()
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:161:5
    |
161 |     runtime.fill_count = shared.fill_count;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: consider initializing the variable with `editor::puzzle3d::config::component::Puzzle3dRuntime { fill_count: shared.fill_count, overlap_budget: shared.overlap_budget, object_kind_weights: shared.object_kind_weights.clone(), vortex_kind_weights: shared.vortex_kind_weights.clone(), lod_automatic: window.lod_automatic, lod_depth_variable: window.lod_depth_variable, grid_visible: window.grid_visible, lod_manual: window.lod_manual, grid_snap_enabled: window.grid_snap_enabled, grid_spacing: window.grid_spacing, selectable_kinds: window.selectable_kinds.clone(), proximity_radius: window.proximity_radius, chunk_size: window.chunk_size, voxel_dims: window.voxel_dims, transform_move: window.transform_move, transform_rotate: window.transform_rotate, vortex_show: window.vortex_show.clone(), vortex_direction: window.vortex_direction.clone(), sun: window.sun.clone(), camera: window.camera.clone(), suggestion_menu: transient.suggestion_menu.clone(), engagement_input: transient.engagement_input.clone(), brush_candidate_index: transient.brush_candidate_index, active_tool_id: view.and_then(|value| value.active_tool_id.clone()), window_ids: view.map(|value| value.window_instances.iter().map(|window| window.id.clone()).collect()).unwrap_or_else(|| vec![main::WINDOW_KIND_ID.into()]) }` and removing relevant reassignments
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:160:5
    |
160 |     let mut runtime = Puzzle3dRuntime::default();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#field_reassign_with_default
    = note: `-D clippy::field-reassign-with-default` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::field_reassign_with_default)]`


error: called `map(<f>).unwrap_or_else(<g>)` on an `Option` value
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:185:26
    |
185 | ... = view.map(|value| value.window_instances.iter().map(|window| window.id.clone()).collect()).unwrap_or_else(|| vec![main::WINDOW_KIND_ID.into()]);
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `view.map_or_else(|| vec![main::WINDOW_KIND_ID.into()], |value| value.window_instances.iter().map(|window| window.id.clone()).collect())`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
    = note: `-D clippy::map-unwrap-or` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::map_unwrap_or)]`


error: redundant closure
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:135:124
    |
135 | ...hare, |entry, share| object_kind_item(entry, share)))?;
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace the closure with the function itself: `object_kind_item`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure
    = note: `-D clippy::redundant-closure` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::redundant_closure)]`


error: this method chain can be written more clearly with `if .. else ..`
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:323:37
    |
323 |           let (pending, candidates) = (!menu.vortex_full_id.is_empty())
    |  _____________________________________^
324 | |             .then(|| {
325 | |                 let result = session.brush_candidates(&menu.vortex_full_id);
326 | |                 let candidates: Vec<Value> = result
...   |
346 | |             })
347 | |             .unwrap_or((false, Vec::new()));
    | |___________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#obfuscated_if_else
    = note: `-D clippy::obfuscated-if-else` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::obfuscated_if_else)]`
help: try
    |
323 ~         let (pending, candidates) = if !menu.vortex_full_id.is_empty() { {
324 +                 let result = session.brush_candidates(&menu.vortex_full_id);
325 +                 let candidates: Vec<Value> = result
326 +                     .free
327 +                     .iter()
328 +                     .enumerate()
329 +                     .map(|(index, candidate)| {
330 +                         let object_kind = Some(candidate.object_kind_id.as_str());
331 +                         let object_label = candidate.object_kind_id.as_str();
332 +                         let source_vortex_index = candidate.source_vortex_index;
333 +                         let color = object_kind_color(&envelope.fixture.meta, object_kind);
334 +                         let icon = object_kind_icon(&envelope.fixture.meta, object_kind);
335 +                         json!({
336 +                             "index": index,
337 +                             "objectLabel": object_label,
338 +                             "vortexLabel": format!("vortex {source_vortex_index}"),
339 +                             "icon": icon,
340 +                             "color": color,
341 +                         })
342 +                     })
343 +                     .collect();
344 +                 (result.unknown_pending, candidates)
345 ~             } } else { (false, Vec::new()) };
    |


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:206:1
    |
206 | / fn block2d_retained_reduce(
207 | |     command: &Block2dCommand,
208 | |     snapshot: &Block2dSnapshot,
209 | |     config: &Block2dConfig,
...   |
214 | |     operation: &AppOperationContext,
215 | | ) -> Result<Emit<Block2dMutation, Block2dConfigMutation, NoDraftMutation>, Fault> {
    | |_________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:64:1
   |
64 | / impl Default for Block2dConfig {
65 | |     fn default() -> Self {
66 | |         Self {}
67 | |     }
68 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
18 + #[derive(Default)]
19 | pub struct Block2dConfig {}
   |


error: couldn't read `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/../../../../🫀️core/../../🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`: No such file or directory (os error 2)
  --> ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/../../../../🫀️core/🦀️.rs:59:55
   |
59 | ...son", include_str!("../../🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"));
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: couldn't read `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/../../../../🫀️core/../../✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`: No such file or directory (os error 2)
  --> ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/../../../../🫀️core/🦀️.rs:60:56
   |
60 | ...on", include_str!("../../✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:53:162
   |
53 | ...t.notes, status: snapshot.status, tags: snapshot.tags, ..Self::default() }
   |                                                             ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:189:1
    |
189 | / fn vcs_bounded_reduce(
190 | |     command: &VcsCommand,
191 | |     snapshot: &VcsSnapshot,
192 | |     config: &VcsDemoConfig,
...   |
197 | |     operation: &AppOperationContext,
198 | | ) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation, NoDraftMutation>, Fault> {
    | |_________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:67:1
   |
67 | / impl Default for VcsDemoConfig {
68 | |     fn default() -> Self {
69 | |         Self {}
70 | |     }
71 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
21 + #[derive(Default)]
22 | pub struct VcsDemoConfig {}
   |


error: very complex type used. Consider factoring parts into `type` definitions
  --> ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/../../🦀️.rs:57:15
   |
57 |     let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity
   = note: `-D clippy::type-complexity` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::type_complexity)]`


error: this function's return value is unnecessarily wrapped by `Result`
  --> ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️change-schema/🦀️.rs:52:1
   |
52 | fn bridge_render(snapshot: &PlaygroundSnapshot, messages: Vec<String>) -> Result<String, String> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
   = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
   |
52 - fn bridge_render(snapshot: &PlaygroundSnapshot, messages: Vec<String>) -> Result<String, String> {
52 + fn bridge_render(snapshot: &PlaygroundSnapshot, messages: Vec<String>) -> std::string::String {
   |
help: ...and then remove the surrounding `Ok()` from returning expressions
   |
54 -     Ok(to_string(&value))
54 +     to_string(&value)
   |


error: deref which would be done by auto-deref
  --> ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:29:71
   |
29 |                         AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
   |                                                                       ^^ help: try: `t`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#explicit_auto_deref
   = note: `-D clippy::explicit-auto-deref` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::explicit_auto_deref)]`


error: deref which would be done by auto-deref
  --> ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:30:75
   |
30 |                         AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
   |                                                                           ^^ help: try: `b`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#explicit_auto_deref


error: called `skip(..).next()` on an iterator
  --> ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs:13:48
   |
13 |     let first_data_record = from.records.iter().skip(usize::from(from.has_header)).next();
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(usize::from(from.has_header))`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#iter_skip_next
   = note: `-D clippy::iter-skip-next` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::iter_skip_next)]`


error: this function has too many arguments (8/7)
  --> ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:58:1
   |
58 | / fn playground_retained_reduce(
59 | |     command: &PlaygroundCommand,
60 | |     snapshot: &PlaygroundSnapshot,
61 | |     config: &NoConfig,
...  |
66 | |     operation: &AppOperationContext,
67 | | ) -> Result<Emit<PlaygroundMutation, NoConfigMutation, NoDraftMutation>, Fault> {
   | |_______________________________________________________________________________^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
   = note: `-D clippy::too-many-arguments` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this argument is passed by value, but not consumed in the function body
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️.rs:2705:34
     |
2705 | fn node_eval_status_json(status: NodeEvalStatus) -> crate::os_pack::json::Value {
     |                                  ^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
     = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
     |
2705 | fn node_eval_status_json(status: &NodeEvalStatus) -> crate::os_pack::json::Value {
     |                                  +


error: this argument is passed by value, but not consumed in the function body
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../📐️brep-geometry/🦀️.rs:236:32
    |
236 | pub fn map_kernel_error(error: semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::BrepError) -> EvalError {
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
236 | pub fn map_kernel_error(error: &semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::BrepError) -> EvalError {
    |                                +


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:41:107
   |
41 |         Self { catalog: snapshot.catalog, stock_extra: snapshot.stock_extra, curated: snapshot.curated, ..Self::default() }
   |                                                                                                           ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:254:1
    |
254 | / fn sourcing_curation_retained_reduce(
255 | |     command: &SourcingCurationCommand,
256 | |     snapshot: &CurationSnapshot,
257 | |     config: &SourcingCurationConfig,
...   |
262 | |     operation: &AppOperationContext,
263 | | ) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation, NoDraftMutation>, Fault> {
    | |___________________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: MutationLeaf source authority failed: mutation descriptor must contain exactly the fourteen schema fields
  --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:6:1
   |
 6 | / #[dsl(keyword = "set-query")]
 7 | | #[mutation_leaf(contract = ::protocol)]
 8 | | pub struct SetQuery {
 9 | |     pub value: String,
10 | | }
   | |_^


error: MutationLeaf source authority failed: mutation descriptor must contain exactly the fourteen schema fields
  --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:7:1
   |
 7 | / #[dsl(keyword = "replace-query-result")]
 8 | | #[mutation_leaf(contract = ::protocol)]
 9 | | pub struct ReplaceQueryResult {
10 | |     pub execution_id: Option<String>,
11 | |     pub result: Option<QueryResult>,
12 | |     pub error: Option<String>,
13 | | }
   | |_^


error[E0432]: unresolved import `framework_editor`
 --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:7:9
  |
7 | pub use framework_editor::*;
  |         ^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `framework_editor`
  |
  = help: if you wanted to use a crate named `framework_editor`, use `cargo add framework_editor` to add it to your `Cargo.toml`


error: unused import: `JackEditorWindowConfigMutation`
  --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:10:48
   |
10 | use crate::editor::jack::query_window_config::{JackEditorWindowConfigMutation, JackEditorWindowConfigOwner};
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unused_imports)]`


error: this `map_or` can be simplified
   --> ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../../🦀️.rs:766:19
    |
766 |     let enabled = param("enabled").map_or(true, |v| v == "true");
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_map_or
    = note: `-D clippy::unnecessary-map-or` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unnecessary_map_or)]`
help: use `is_none_or` instead
    |
766 -     let enabled = param("enabled").map_or(true, |v| v == "true");
766 +     let enabled = param("enabled").is_none_or(|v| v == "true");
    |


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:94:15
   |
94 |             ..Self::default()
   |               ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: unnecessary map of the identity function
   --> ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:660:64
    |
660 |         let target = store::os_io::ArtifactRef::parse_uri(&uri).map_err(|error| error)?;
    |                                                                ^^^^^^^^^^^^^^^^^^^^^^^ help: remove the call to `map_err`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_identity
    = note: `-D clippy::map-identity` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::map_identity)]`


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:306:1
    |
306 | / pub fn process3d_admit_publication_authority(
307 | |     operation: semio_framework_job::OperationId,
308 | |     generation: semio_framework_job::Generation,
309 | |     base_revision: u64,
...   |
314 | |     maximum_controls: usize,
315 | | ) -> Result<(), &'static str> {
    | |_____________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:342:1
    |
342 | / fn process3d_retained_reduce(
343 | |     command: &Process3dCommand,
344 | |     snapshot: &Process3dSnapshot,
345 | |     config: &Process3dConfig,
...   |
350 | |     operation: &AppOperationContext,
351 | | ) -> Result<Emit<Process3dMutation, Process3dConfigMutation, NoDraftMutation>, Fault> {
    | |_____________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


error[E0277]: the trait bound `editor::jack::query_window_config::mutations::set_query::SetQuery: dsl::MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:12:89
    |
 12 | impl protocol::MutationKind<JackEditorWindowConfig, JackEditorWindowConfigMutation> for SetQuery {
    |                                                                                         ^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `editor::jack::query_window_config::mutations::set_query::SetQuery`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:8:1
    |
  8 | pub struct SetQuery {
    | ^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-9175024944399929990.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `ReplaceQueryResult: MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:15:97
    |
 15 | impl protocol::MutationKind<JackResultsWindowTransient, JackResultsWindowTransientMutation> for ReplaceQueryResult {
    |                                                                                                 ^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `ReplaceQueryResult`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:9:1
    |
  9 | pub struct ReplaceQueryResult {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-11766129370207311671.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `editor::jack::query_window_config::mutations::set_query::SetQuery: dsl::MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:13:14
    |
 13 |     SetQuery(SetQuery),
    |              ^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `editor::jack::query_window_config::mutations::set_query::SetQuery`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:8:1
    |
  8 | pub struct SetQuery {
    | ^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::SEMANTICS`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::SEMANTICS`
...
223 |     const SEMANTICS: SemanticDescriptor;
    |           --------- required by a bound in this associated constant
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-9175024944399929990.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `editor::jack::query_window_config::mutations::set_query::SetQuery: dsl::MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:13:14
    |
 13 |     SetQuery(SetQuery),
    |              ^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `editor::jack::query_window_config::mutations::set_query::SetQuery`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:8:1
    |
  8 | pub struct SetQuery {
    | ^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::diff`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::diff`
...
225 |     fn diff(&self, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff>;
    |        ---- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-9175024944399929990.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `editor::jack::query_window_config::mutations::set_query::SetQuery: dsl::MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:13:14
    |
 13 |     SetQuery(SetQuery),
    |              ^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `editor::jack::query_window_config::mutations::set_query::SetQuery`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:8:1
    |
  8 | pub struct SetQuery {
    | ^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::inverse`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::inverse`
...
229 |     fn inverse(&self, base: &P) -> Vec<Op>;
    |        ------- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-9175024944399929990.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `editor::jack::query_window_config::mutations::set_query::SetQuery: dsl::MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:13:14
    |
 13 |     SetQuery(SetQuery),
    |              ^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `editor::jack::query_window_config::mutations::set_query::SetQuery`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:8:1
    |
  8 | pub struct SetQuery {
    | ^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::timestamp`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::timestamp`
...
233 |     fn timestamp(&self) -> Option<protocol::ids::HybridLogicalTimestamp> {
    |        --------- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-9175024944399929990.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `editor::jack::query_window_config::mutations::set_query::SetQuery: dsl::MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:13:14
    |
 13 |     SetQuery(SetQuery),
    |              ^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `editor::jack::query_window_config::mutations::set_query::SetQuery`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:8:1
    |
  8 | pub struct SetQuery {
    | ^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::may_emit_foreign_steps`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::may_emit_foreign_steps`
...
242 |     fn may_emit_foreign_steps(&self) -> bool {
    |        ---------------------- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-9175024944399929990.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `editor::jack::query_window_config::mutations::set_query::SetQuery: dsl::MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:13:14
    |
 13 |     SetQuery(SetQuery),
    |              ^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `editor::jack::query_window_config::mutations::set_query::SetQuery`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:8:1
    |
  8 | pub struct SetQuery {
    | ^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::foreign_steps`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::foreign_steps`
...
248 |     fn foreign_steps(&self, _base: &P) -> Vec<ForeignStep> {
    |        ------------- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-9175024944399929990.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `editor::jack::query_window_config::mutations::set_query::SetQuery: dsl::MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:13:14
    |
 13 |     SetQuery(SetQuery),
    |              ^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `editor::jack::query_window_config::mutations::set_query::SetQuery`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:8:1
    |
  8 | pub struct SetQuery {
    | ^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::label`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::label`
...
231 |     fn label(&self) -> String;
    |        ----- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-9175024944399929990.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `editor::jack::query_window_config::mutations::set_query::SetQuery: dsl::MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:13:14
    |
 13 |     SetQuery(SetQuery),
    |              ^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `editor::jack::query_window_config::mutations::set_query::SetQuery`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs:8:1
    |
  8 | pub struct SetQuery {
    | ^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::target`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::target`
...
238 |     fn target(&self) -> Vec<String> {
    |        ------ required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-9175024944399929990.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `ReplaceQueryResult: MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs:13:24
    |
 13 |     ReplaceQueryResult(ReplaceQueryResult),
    |                        ^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `ReplaceQueryResult`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:9:1
    |
  9 | pub struct ReplaceQueryResult {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::SEMANTICS`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::SEMANTICS`
...
223 |     const SEMANTICS: SemanticDescriptor;
    |           --------- required by a bound in this associated constant
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-11766129370207311671.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `ReplaceQueryResult: MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs:13:24
    |
 13 |     ReplaceQueryResult(ReplaceQueryResult),
    |                        ^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `ReplaceQueryResult`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:9:1
    |
  9 | pub struct ReplaceQueryResult {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::diff`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::diff`
...
225 |     fn diff(&self, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff>;
    |        ---- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-11766129370207311671.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `ReplaceQueryResult: MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs:13:24
    |
 13 |     ReplaceQueryResult(ReplaceQueryResult),
    |                        ^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `ReplaceQueryResult`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:9:1
    |
  9 | pub struct ReplaceQueryResult {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::inverse`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::inverse`
...
229 |     fn inverse(&self, base: &P) -> Vec<Op>;
    |        ------- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-11766129370207311671.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `ReplaceQueryResult: MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs:13:24
    |
 13 |     ReplaceQueryResult(ReplaceQueryResult),
    |                        ^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `ReplaceQueryResult`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:9:1
    |
  9 | pub struct ReplaceQueryResult {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::timestamp`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::timestamp`
...
233 |     fn timestamp(&self) -> Option<protocol::ids::HybridLogicalTimestamp> {
    |        --------- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-11766129370207311671.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `ReplaceQueryResult: MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs:13:24
    |
 13 |     ReplaceQueryResult(ReplaceQueryResult),
    |                        ^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `ReplaceQueryResult`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:9:1
    |
  9 | pub struct ReplaceQueryResult {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::may_emit_foreign_steps`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::may_emit_foreign_steps`
...
242 |     fn may_emit_foreign_steps(&self) -> bool {
    |        ---------------------- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-11766129370207311671.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `ReplaceQueryResult: MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs:13:24
    |
 13 |     ReplaceQueryResult(ReplaceQueryResult),
    |                        ^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `ReplaceQueryResult`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:9:1
    |
  9 | pub struct ReplaceQueryResult {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::foreign_steps`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::foreign_steps`
...
248 |     fn foreign_steps(&self, _base: &P) -> Vec<ForeignStep> {
    |        ------------- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-11766129370207311671.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `ReplaceQueryResult: MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs:13:24
    |
 13 |     ReplaceQueryResult(ReplaceQueryResult),
    |                        ^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `ReplaceQueryResult`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:9:1
    |
  9 | pub struct ReplaceQueryResult {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::label`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::label`
...
231 |     fn label(&self) -> String;
    |        ----- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-11766129370207311671.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `ReplaceQueryResult: MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs:13:24
    |
 13 |     ReplaceQueryResult(ReplaceQueryResult),
    |                        ^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `ReplaceQueryResult`
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🦀️.rs:9:1
    |
  9 | pub struct ReplaceQueryResult {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              SetEditorSelection
              editor::jack::window_config::mutations::set_camera::SetCamera
            and 986 others
note: required by a bound in `dsl::MutationKind::target`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind::target`
...
238 |     fn target(&self) -> Vec<String> {
    |        ------ required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_trinity_jack-25e620277d9d21fa.long-type-11766129370207311671.txt'
    = note: consider using `--verbose` to print the full type name to the console


error: this function has too many arguments (8/7)
    --> ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1105:1
     |
1105 | / fn cad_retained_reduce(
1106 | |     command: &CadCommand,
1107 | |     snapshot: &CadSnapshot,
1108 | |     config: &CadConfig,
...    |
1113 | |     operation: &AppOperationContext,
1114 | | ) -> Result<Emit<CadMutation, CadConfigMutation, NoDraftMutation>, Fault> {
     | |_________________________________________________________________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
     = note: `-D clippy::too-many-arguments` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: large size difference between variants
   --> ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:244:1
    |
244 | /  pub enum CadConfigMutation {
245 | |      #[dsl(key = "snapshot")]
246 | |/     Snapshot {
247 | ||         #[dsl(block)]
248 | ||         config: CadConfig,
249 | ||     },
    | ||_____- the largest variant contains at least 1064 bytes
250 | |      #[dsl(key = "contributions")]
251 | |      SetContributions { json: String },
    | |      --------------------------------- the second-largest variant contains at least 12 bytes
252 | |  }
    | |__^ the entire enum is at least 1064 bytes
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    = note: `-D clippy::large-enum-variant` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
248 -         config: CadConfig,
248 +         config: Box<CadConfig>,
    |


Some errors have detailed explanations: E0277, E0432.

For more information about an error, try `rustc --explain E0277`.

error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:42:1
   |
42 | / impl Default for GisMapArtifact {
43 | |     fn default() -> Self {
44 | |         Self { positions: Vec::new(), routes: Vec::new(), regions: Vec::new(), image: None }
45 | |     }
46 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
22 + #[derive(Default)]
23 | pub struct GisMapArtifact {
   |


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:57:124
   |
57 | ...tes, regions: snapshot.regions, image: snapshot.image, ..Self::default() }
   |                                                             ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: large size difference between variants
   --> ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:341:9
    |
341 | /         enum $state {
342 | |             AwaitToken,
    | |             ---------- the second-largest variant carries no data at all
343 | |             Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
    | |             ----------------------------------------------------------------- the largest variant contains at least 320 bytes
344 | |             Ready,
...   |
347 | |             Complete,
348 | |         }
    | |_________^ the entire enum is at least 320 bytes
...
464 | / gis_map_owned_field_authority!(
465 | |     GisMapSnapshotDecodeState,
466 | |     GisMapSnapshotDecodeAuthority,
467 | |     GisMapSnapshot,
...   |
473 | |     "snapshot"
474 | | );
    | |_- in this macro invocation
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    = note: `-D clippy::large-enum-variant` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
    = note: this error originates in the macro `gis_map_owned_field_authority` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
343 -             Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
343 +             Decode(Box<store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>>),
    |


error: large size difference between variants
   --> ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:341:9
    |
341 | /         enum $state {
342 | |             AwaitToken,
    | |             ---------- the second-largest variant carries no data at all
343 | |             Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
    | |             ----------------------------------------------------------------- the largest variant contains at least 320 bytes
344 | |             Ready,
...   |
347 | |             Complete,
348 | |         }
    | |_________^ the entire enum is at least 320 bytes
...
476 | / gis_map_owned_field_authority!(
477 | |     GisMapMutationDecodeState,
478 | |     GisMapMutationDecodeAuthority,
479 | |     GisMapMutation,
...   |
485 | |     "mutation"
486 | | );
    | |_- in this macro invocation
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    = note: this error originates in the macro `gis_map_owned_field_authority` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
343 -             Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
343 +             Decode(Box<store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>>),
    |


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:275:1
    |
275 | / fn gis2d_retained_reduce(
276 | |     command: &Gis2dCommand,
277 | |     snapshot: &GisMapSnapshot,
278 | |     config: &Gis2dConfig,
...   |
283 | |     operation: &AppOperationContext,
284 | | ) -> Result<Emit<GisMapMutation, Gis2dConfigMutation, NoDraftMutation>, Fault> {
    | |______________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this `let...else` may be rewritten with the `?` operator
    --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs:2124:9
     |
2124 |         let Some((partition_index, entry)) = self.state.merge_candidate.take() else { return None };
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace it with: `let (partition_index, entry) = self.state.merge_candidate.take()?;`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#question_mark
     = note: `-D clippy::question-mark` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::question_mark)]`


error: the loop variable `k` is used to index `reaction_sum`
    --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs:2684:18
     |
2684 |         for k in 0..6 {
     |                  ^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
     = note: `-D clippy::needless-range-loop` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::needless_range_loop)]`
help: consider using an iterator and enumerate()
     |
2684 -         for k in 0..6 {
2684 +         for (k, <item>) in reaction_sum.iter_mut().enumerate() {
     |


error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../⚙️engine/🖥️app-surface/🦀️.rs:24:56
   |
24 | pub fn canvas_2d_surface(id: impl Into<String>, scene: semio_framework_ui_scene::Canvas2dScene) -> semio_framework_plugin::UiAssembl...
   |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
   = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
   |
24 | pub fn canvas_2d_surface(id: impl Into<String>, scene: &semio_framework_ui_scene::Canvas2dScene) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
   |                                                        +


error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../⚙️engine/🖥️app-surface/🦀️.rs:35:55
   |
35 | pub fn world_3d_surface(id: impl Into<String>, scene: semio_framework_ui_scene::World3dScene) -> semio_framework_plugin::UiAssemblyR...
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
   |
35 | pub fn world_3d_surface(id: impl Into<String>, scene: &semio_framework_ui_scene::World3dScene) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
   |                                                       +


error: the loop variable `j` is used to index `grads`
   --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs:514:22
    |
514 |             for j in 0..4 {
    |                      ^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator and enumerate()
    |
514 -             for j in 0..4 {
514 +             for (j, <item>) in grads.iter().enumerate() {
    |


error: the loop variable `a` is used to index `pd`
   --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs:591:22
    |
591 |             for a in 0..3 {
    |                      ^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator and enumerate()
    |
591 -             for a in 0..3 {
591 +             for (a, <item>) in pd.iter().enumerate().take(3) {
    |


error: the loop variable `j` is used to index `grads`
   --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs:684:26
    |
684 |                 for j in 0..8 {
    |                          ^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator and enumerate()
    |
684 -                 for j in 0..8 {
684 +                 for (j, <item>) in grads.iter().enumerate() {
    |


error: large size difference between variants
   --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs:182:1
    |
182 | / enum MeshDomainOwner {
183 | |     Dynamic(PlanarDomain),
    | |     --------------------- the second-largest variant contains at least 24 bytes
184 | |     Mounted(MountedPlanarDomain),
    | |     ---------------------------- the largest variant contains at least 67864 bytes
185 | | }
    | |_^ the entire enum is at least 67872 bytes
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    = note: `-D clippy::large-enum-variant` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
184 -     Mounted(MountedPlanarDomain),
184 +     Mounted(Box<MountedPlanarDomain>),
    |


error: this function's return value is unnecessary
    --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs:3445:5
     |
3445 |     fn advance_orthogonalize(&mut self) -> Result<(), ()> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
     = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove the return type...
     |
3445 -     fn advance_orthogonalize(&mut self) -> Result<(), ()> {
3445 +     fn advance_orthogonalize(&mut self) -> () {
     |
help: ...and then remove returned values
     |
3453 ~             return ;
3454 |         }
 ...
3475 |             }
3476 ~             return ;
3477 |         }
 ...
3481 |         work.scalar = 0.0;
3482 ~         
     |


error: this function's return value is unnecessary
    --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs:3485:5
     |
3485 |     fn advance_normalize(&mut self) -> Result<(), ()> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove the return type...
     |
3485 -     fn advance_normalize(&mut self) -> Result<(), ()> {
3485 +     fn advance_normalize(&mut self) -> () {
     |
help: ...and then remove returned values
     |
3497 ~                     return ;
3498 |                 }
 ...
3502 |             }
3503 ~             return ;
3504 |         }
 ...
3514 |             }
3515 ~             return ;
3516 |         }
 ...
3520 |                 work.phase = 4;
3521 ~                 return ;
3522 |             }
 ...
3532 |             }
3533 ~             return ;
3534 |         }
 ...
3544 |             }
3545 ~             return ;
3546 |         }
 ...
3552 |                 work.phase = 6;
3553 ~                 return ;
3554 |             }
 ...
3564 |             }
3565 ~             return ;
3566 |         }
 ...
3571 |                 work.phase = 8;
3572 ~                 return ;
3573 |             }
 ...
3579 |             }
3580 ~             return ;
3581 |         }
 ...
3592 |             }
3593 ~             return ;
3594 |         }
 ...
3602 |             }
3603 ~             return ;
3604 |         }
 ...
3618 |         }
3619 ~         
     |


error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs:33:1
   |
33 | / impl Default for Fem2dArtifact {
34 | |     fn default() -> Self {
35 | |         Self {
36 | |             nodes: Default::default(),
...  |
47 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
10 + #[derive(Default)]
11 | pub struct Fem2dArtifact {
   |


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs:77:15
   |
77 |             ..Self::default()
   |               ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs:151:1
    |
151 | / fn fem2d_retained_reduce(
152 | |     command: &Fem2dCommand,
153 | |     snapshot: &Fem2dSnapshot,
154 | |     config: &Fem2dConfig,
...   |
159 | |     operation: &AppOperationContext,
160 | | ) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation, NoDraftMutation>, Fault> {
    | |_____________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs:915:21
    |
915 | fn vector_layer(id: String, origin: (f64, f64), vector: [f64; 2], color: &str) -> dsl::json::Value {
    |                     ^^^^^^ help: consider changing the type to: `&str`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value


error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:24:30
   |
24 | fn filled_triangle_layer(id: String, p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), color: &str, alpha: f64) -> Value {
   |                              ^^^^^^ help: consider changing the type to: `&str`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value


error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:42:29
   |
42 | fn filled_polygon_layer(id: String, points: &[(f64, f64)], color: &str, alpha: f64) -> Value {
   |                             ^^^^^^ help: consider changing the type to: `&str`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value


error: this `map_or` can be simplified
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs:415:8
    |
415 |     if bytes.len().checked_add(additional).map_or(true, |length| length > byte_limit) {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_map_or
    = note: `-D clippy::unnecessary-map-or` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unnecessary_map_or)]`
help: use `is_none_or` instead
    |
415 -     if bytes.len().checked_add(additional).map_or(true, |length| length > byte_limit) {
415 +     if bytes.len().checked_add(additional).is_none_or(|length| length > byte_limit) {
    |


error: this function's return value is unnecessary
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs:1477:5
     |
1477 |     fn rebuild_one(&mut self) -> Result<(), String> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
     = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove the return type...
     |
1477 -     fn rebuild_one(&mut self) -> Result<(), String> {
1477 +     fn rebuild_one(&mut self) -> () {
     |
help: ...and then remove returned values
     |
1481 ~             return ;
1482 |         }
 ...
1491 |             }
1492 ~             return ;
1493 |         }
 ...
1509 |         self.weighted_log_sum = 0.0;
1510 ~         
     |


error: this function's return value is unnecessary
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs:1513:5
     |
1513 |     fn finish(&mut self) -> Result<(), String> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove the return type...
     |
1513 -     fn finish(&mut self) -> Result<(), String> {
1513 +     fn finish(&mut self) -> () {
     |
help: ...and then remove returned values
     |
1571 -         Ok(())
     |


error: manual implementation of `.is_multiple_of()`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs:1751:56
     |
1751 |                     if self.state.observations == 1 || self.state.observations % CHECKPOINT_INTERVAL == 0 {
     |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace with: `self.state.observations.is_multiple_of(CHECKPOINT_INTERVAL)`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_is_multiple_of
     = note: `-D clippy::manual-is-multiple-of` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::manual_is_multiple_of)]`


error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:786:66
    |
786 | ...).map(|module| snapshot.weights.iter().find(|w| w.module_id == module.child_id).map(|w| w.weight).unwrap_or(1.0)).collect();
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
    = note: `-D clippy::map-unwrap-or` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::map_unwrap_or)]`
help: use `map_or(<a>, <f>)` instead
    |
786 -     let weights: Vec<f64> = snapshot.modules.iter().map(|module| snapshot.weights.iter().find(|w| w.module_id == module.child_id).map(|w| w.weight).unwrap_or(1.0)).collect();
786 +     let weights: Vec<f64> = snapshot.modules.iter().map(|module| snapshot.weights.iter().find(|w| w.module_id == module.child_id).map_or(1.0, |w| w.weight)).collect();
    |


error: calls to `push` immediately after creation
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:156:9
    |
156 | /         let mut mutations = Vec::with_capacity(32);
157 | |         mutations.push(En1991Mutation::ChangeDeltaTK(change_delta_tk::ChangeDeltaTK { new_delta_t_k: snapshot.delta_t_k }));
158 | |         mutations.push(En1991Mutation::ChangeBridgeLane(change_bridge_lane::ChangeBridgeLane { new_bridge_lane: snapshot.bridge_lane }));
159 | |         mutations.push(En1991Mutation::ChangeSiloBulkDensityKnM3(change_silo_bulk_density_kn_m3::ChangeSiloBulkDensityKnM3 { new_silo_bulk_density_kn_m3: snapshot.silo_bulk_density_kn_m3 }));
...   |
187 | |         mutations.push(En1991Mutation::ChangeFireResistanceMin(change_fire_resistance_min::ChangeFireResistanceMin { new_fire_resistance_min: snapshot.fire_resistance_min }));
188 | |         mutations.push(En1991Mutation::ChangeConstructionActivity(change_construction_activity::ChangeConstructionActivity { new_construction_activity: snapshot.construction_activity.clone() }));
    | |___________________________________________________________________________________________________________________________________________________________________________________________________^ help: consider using the `vec![]` macro: `let mutations = vec![..];`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#vec_init_then_push
    = note: `-D clippy::vec-init-then-push` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::vec_init_then_push)]`


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:162:1
    |
162 | / fn dag_retained_config_reduce(
163 | |     command: &DagCommand,
164 | |     snapshot: &DagSnapshot,
165 | |     config: &DagConfig,
...   |
170 | |     operation: &AppOperationContext,
171 | | ) -> Result<Emit<DagMutation, DagConfigMutation, NoDraftMutation>, Fault> {
    | |_________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️change-camera/🦀️.rs:20:114
   |
20 |         protocol::MutationOutcome::new(DagConfig { camera_x: self.x, camera_y: self.y, camera_zoom: self.zoom, ..base.clone() })
   |                                                                                                                  ^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:93:15
   |
93 |             ..Self::default_ui()
   |               ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:394:47
    |
394 | pub fn note_block_patch_diff(id: &str, block: NoteBlockNode) -> NoteDiff {
    |                                               ^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
    |
394 | pub fn note_block_patch_diff(id: &str, block: &NoteBlockNode) -> NoteDiff {
    |                                               +


error: large size difference between variants
    --> ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:1059:1
     |
1059 | /   enum NoteBlockPayloadMaterialization {
1060 | | /     Text {
1061 | | |         phase: u8,
1062 | | |         content: NoteTextContentMaterializationCursor,
1063 | | |         materialized_content: Option<crate::NoteTextChild>,
...    | |
1067 | | |         materialized_align: Option<String>,
1068 | | |     },
     | | |_____- the largest variant contains at least 393 bytes
...    |
1073 | | /     Table {
1074 | | |         phase: u8,
1075 | | |         column_cursor: usize,
1076 | | |         cell_row_cursor: usize,
...    | |
1080 | | |         rows: Vec<Vec<crate::NoteTableCell>>,
1081 | | |     },
     | | |_____- the second-largest variant contains at least 53 bytes
...    |
1094 | |       },
1095 | |   }
     | |___^ the entire enum is at least 396 bytes
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
     = note: `-D clippy::large-enum-variant` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
     |
1062 -         content: NoteTextContentMaterializationCursor,
1062 +         content: Box<NoteTextContentMaterializationCursor>,
     |


error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:79:1
   |
79 | / impl Default for NoteConfig {
80 | |     fn default() -> Self {
81 | |         Self { engagement_input: String::new(), camera: NoteCamera::default() }
82 | |     }
83 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
26 + #[derive(Default)]
27 | pub struct NoteConfig {
   |


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:39:85
   |
39 |         Self { schema: snapshot.schema, flow: snapshot.flow, text: snapshot.text, ..Self::default() }
   |                                                                                     ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: struct update has no effect, all the fields in the struct have already been specified
   --> ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:107:15
    |
107 |             ..Self::default()
    |               ^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
    = note: `-D clippy::needless-update` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:207:1
    |
207 | / fn layout_retained_reduce(
208 | |     command: &LayoutCommand,
209 | |     snapshot: &LayoutSnapshot,
210 | |     config: &LayoutConfig,
...   |
215 | |     operation: &AppOperationContext,
216 | | ) -> Result<Emit<LayoutMutation, LayoutConfigMutation, NoDraftMutation>, Fault> {
    | |_______________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this `if` statement can be collapsed
   --> ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs:597:13
    |
597 | /             if !matches!(self.token, JsonToken::None) {
598 | |                 if self.advance_token(bytes)? {
599 | |                     continue;
600 | |                 }
601 | |             }
    | |_____________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_if
    = note: `-D clippy::collapsible-if` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::collapsible_if)]`
help: collapse nested if block
    |
597 ~             if !matches!(self.token, JsonToken::None)
598 ~                 && self.advance_token(bytes)? {
599 |                     continue;
600 ~                 }
    |


error: all variants have the same postfix: `Ids`
   --> ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs:721:1
    |
721 | / enum StringArraySource {
722 | |     SpreadPageIds(usize),
723 | |     PageLayerIds(usize),
724 | |     ParentLayerIds(usize),
725 | |     LayerObjectIds { owner: RecordOwner, layer: usize },
726 | | }
    | |_^
    |
    = help: remove the postfixes and use full paths to the variants instead of glob imports
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#enum_variant_names
    = note: `-D clippy::enum-variant-names` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::enum_variant_names)]`


error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs:1114:5
     |
1114 |     fn escaped_string_step(value: &str, cursor: &mut JsonStringWriteCursor) -> Result<(Vec<u8>, bool), String> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
     = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
     |
1114 -     fn escaped_string_step(value: &str, cursor: &mut JsonStringWriteCursor) -> Result<(Vec<u8>, bool), String> {
1114 +     fn escaped_string_step(value: &str, cursor: &mut JsonStringWriteCursor) -> (std::vec::Vec<u8>, bool) {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
1150 -         Ok((output, cursor.closed))
1150 +         (output, cursor.closed)
     |


error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs:2822:5
     |
2822 |     fn close_pending(released_items: usize, released_bytes: usize) -> Result<PluginCloseStep, Fault> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Result` from the return type...
     |
2822 -     fn close_pending(released_items: usize, released_bytes: usize) -> Result<PluginCloseStep, Fault> {
2822 +     fn close_pending(released_items: usize, released_bytes: usize) -> semio_framework_plugin::PluginCloseStep {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
2823 -         Ok(PluginCloseStep::Pending { released_items, released_bytes })
2823 +         PluginCloseStep::Pending { released_items, released_bytes }
     |


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:603:1
    |
603 | / fn remodeling_retained_reduce(
604 | |     command: &RemodelingCommand,
605 | |     snapshot: &RemodelingSnapshot,
606 | |     config: &RemodelingConfig,
...   |
611 | |     operation: &AppOperationContext,
612 | | ) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation, NoDraftMutation>, Fault> {
    | |_______________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: the loop variable `axis` is used to index `position`
    --> ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🦀️.rs:3790:33
     |
3790 |                     for axis in 0..3 {
     |                                 ^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
     = note: `-D clippy::needless-range-loop` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::needless_range_loop)]`
help: consider using an iterator and enumerate()
     |
3790 -                     for axis in 0..3 {
3790 +                     for (axis, <item>) in position.iter().enumerate().take(3) {
     |


error: the loop variable `axis` is used to index `point`
    --> ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️.rs:1588:29
     |
1588 |                 for axis in 0..3 {
     |                             ^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator and enumerate()
     |
1588 -                 for axis in 0..3 {
1588 +                 for (axis, <item>) in point.iter().enumerate() {
     |


error: large size difference between variants
   --> ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs:125:1
    |
125 | / enum RasterAssetProgress {
126 | |     Working,
127 | |     Mutation(RemodelingMutation),
    | |     ---------------------------- the largest variant contains at least 296 bytes
128 | |     Complete(ReconstructionAssetCommit),
    | |     ----------------------------------- the second-largest variant contains at least 44 bytes
129 | |     Failed,
130 | | }
    | |_^ the entire enum is at least 296 bytes
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    = note: `-D clippy::large-enum-variant` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
127 -     Mutation(RemodelingMutation),
127 +     Mutation(Box<RemodelingMutation>),
    |


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs:560:61
    |
560 | fn emit_step(job: ReconstructionJob, generation: u64, next: Option<AdvanceReconstruction>) -> Emit<RemodelingMutation, RemodelingCo...
    |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
    |
560 | fn emit_step(job: ReconstructionJob, generation: u64, next: Option<&AdvanceReconstruction>) -> Emit<RemodelingMutation, RemodelingConfigMutation> {
    |                                                                    +


error: this function's return value is unnecessarily wrapped by `Result`
   --> ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs:643:1
    |
643 | fn begin_requested_reconstruction(doc: &ArtifactView<'_, RemodelingSnapshot>, requested_stage: RequestedStage) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
    = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
    |
643 - fn begin_requested_reconstruction(doc: &ArtifactView<'_, RemodelingSnapshot>, requested_stage: RequestedStage) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
643 + fn begin_requested_reconstruction(doc: &ArtifactView<'_, RemodelingSnapshot>, requested_stage: RequestedStage) -> semio_framework_plugin::Emit<standards::v1::subsets::any::schema::mutations::component::RemodelingMutation, editor::remodeling::config::component::mutations::RemodelingConfigMutation> {
    |
help: ...and then remove the surrounding `Ok()` from returning expressions
    |
646 ~         return Emit::default();
647 |     }
...
680 |         job.error = Some(format!("Interactive reconstruction capacity is {MAX_LIVE_SESSIONS} active jobs; cancel one before retrying."));
681 ~         return emit_step(job, generation, None);
682 |     }
683 |     put_session(generation, session);
684 ~     emit_step(job, generation, Some(next))
    |


error: manual implementation of `Option::map`
   --> ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs:767:39
    |
767 |                   terminal.watertight = match session.engine.terminal_watertight_report().as_ref() {
    |  _______________________________________^
768 | |                     Some(report) => Some(watertight_snapshot(report)),
769 | |                     None => None,
770 | |                 };
    | |_________________^ help: try: `session.engine.terminal_watertight_report().as_ref().map(|report| watertight_snapshot(report))`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_map
    = note: `-D clippy::manual-map` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::manual_map)]`


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/../../🦀️.rs:392:44
    |
392 | pub fn equation_snapshot_with_state(graph: EquationGraph, geometry: EquationGeometry) -> EquationSnapshot {
    |                                            ^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
    |
392 | pub fn equation_snapshot_with_state(graph: &EquationGraph, geometry: EquationGeometry) -> EquationSnapshot {
    |                                            +


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/../../🦀️.rs:392:69
    |
392 | pub fn equation_snapshot_with_state(graph: EquationGraph, geometry: EquationGeometry) -> EquationSnapshot {
    |                                                                     ^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
392 | pub fn equation_snapshot_with_state(graph: EquationGraph, geometry: &EquationGeometry) -> EquationSnapshot {
    |                                                                     +


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:72:132
   |
72 | ...ted: snapshot.computed, equation: snapshot.equation, ..Self::default_ui() }
   |                                                           ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:179:67
    |
179 |     pub fn replace(&mut self, label: EquationNodeLabel, new_kind: EquationNodeKind) -> bool {
    |                                                                   ^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
179 |     pub fn replace(&mut self, label: EquationNodeLabel, new_kind: &EquationNodeKind) -> bool {
    |                                                                   +


error: unnecessary map of the identity function
   --> ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:127:115
    |
127 | ...pack::json::from_dsl_value(&value)).map_err(|error| error)?;
    |                                       ^^^^^^^^^^^^^^^^^^^^^^^ help: remove the call to `map_err`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_identity
    = note: `-D clippy::map-identity` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::map_identity)]`


error: unnecessary map of the identity function
   --> ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:128:122
    |
128 | ...pack::json::from_dsl_value(&value)).map_err(|error| error)?;
    |                                       ^^^^^^^^^^^^^^^^^^^^^^^ help: remove the call to `map_err`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_identity


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🦀️.rs:20:99
   |
20 |         protocol::MutationOutcome::new(EquationGraphWindowConfig { camera: self.camera.clone(), ..base.clone() })
   |                                                                                                   ^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update


error: MutationLeaf source authority failed: No such file or directory (os error 2)
  --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🦀️.rs:7:1
   |
 7 | / #[dsl(keyword = "set-camera")]
 8 | | #[mutation_leaf(contract = ::protocol)]
 9 | | pub struct SetCamera {
10 | |     #[dsl(block)]
11 | |     pub camera: WiresCanvasCamera,
12 | | }
   | |_^


error[E0432]: unresolved import `protocol::value::bounded_clone`
 --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:5:22
  |
5 | use protocol::value::bounded_clone::{
  |                      ^^^^^^^^^^^^^ could not find `bounded_clone` in `value`


error[E0432]: unresolved import `protocol::value::DslValueSource`
 --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:8:5
  |
8 | use protocol::value::DslValueSource;
  |     ^^^^^^^^^^^^^^^^^--------------
  |                      |
  |                      no `DslValueSource` in `os_pack::value`


error[E0425]: cannot find function `set_node_field` in module `crate::schema`
   --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:607:44
    |
607 | ...                   crate::schema::set_node_field(&mut board, node_id, "x", dsl::DslValue::float(preview_x));
    |                                      ^^^^^^^^^^^^^^ not found in `crate::schema`
    |
help: consider importing this function through its public re-export
    |
  8 + use crate::mutations::set_node_field;
    |
help: if you import `set_node_field`, refer to it directly
    |
607 -                             crate::schema::set_node_field(&mut board, node_id, "x", dsl::DslValue::float(preview_x));
607 +                             set_node_field(&mut board, node_id, "x", dsl::DslValue::float(preview_x));
    |


error[E0425]: cannot find function `set_node_field` in module `crate::schema`
   --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:608:44
    |
608 | ...                   crate::schema::set_node_field(&mut board, node_id, "y", dsl::DslValue::float(preview_y));
    |                                      ^^^^^^^^^^^^^^ not found in `crate::schema`
    |
help: consider importing this function through its public re-export
    |
  8 + use crate::mutations::set_node_field;
    |
help: if you import `set_node_field`, refer to it directly
    |
608 -                             crate::schema::set_node_field(&mut board, node_id, "y", dsl::DslValue::float(preview_y));
608 +                             set_node_field(&mut board, node_id, "y", dsl::DslValue::float(preview_y));
    |


error: unnecessary qualification
   --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:436:44
    |
436 |         let requested = length.checked_mul(std::mem::size_of::<T>()).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
    |                                            ^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D unused-qualifications` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(unused_qualifications)]`
help: remove the unnecessary path segments
    |
436 -         let requested = length.checked_mul(std::mem::size_of::<T>()).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
436 +         let requested = length.checked_mul(size_of::<T>()).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
    |


error: unnecessary qualification
   --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:441:52
    |
441 | ...   let actual = values.capacity().checked_mul(std::mem::size_of::<T>()).ok_or_else(|| "wires-publication.retained-limit".to_stri...
    |                                                  ^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
441 -         let actual = values.capacity().checked_mul(std::mem::size_of::<T>()).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
441 +         let actual = values.capacity().checked_mul(size_of::<T>()).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
    |


error[E0277]: the trait bound `SetCamera: MutationLeaf` is not satisfied
   --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🦀️.rs:14:91
    |
 14 | impl protocol::MutationKind<WiresCanvasWindowConfig, WiresCanvasWindowConfigMutation> for SetCamera {
    |                                                                                           ^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `dsl::MutationLeaf` is not implemented for `editor::wires::modes::edit::windows::canvas::config::mutations::set_camera::SetCamera`
   --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🦀️.rs:9:1
    |
  9 | pub struct SetCamera {
    | ^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::MutationLeaf`:
              dsl::CommitSpaceCheckpoint
              dsl::CreateSpaceAlternative
              dsl::RemoveSpaceAlternative
              dsl::RemoveSpaceCheckpoint
              dsl::RestoreActiveSpaceAlternative
              dsl::SwitchSpaceAlternative
              editor::wires::component::window_transient::mutations::set_drag::SetDrag
              ChangeNodeAbbreviation
            and 976 others
note: required by a bound in `dsl::MutationKind`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️.rs:219:32
    |
219 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
    |                                ^^^^^^^^^^^^ required by this bound in `MutationKind`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_reasoning_wires-7c1f0c8bbd35eca2.long-type-10793747334563922514.txt'
    = note: consider using `--verbose` to print the full type name to the console


Some errors have detailed explanations: E0277, E0425, E0432.

For more information about an error, try `rustc --explain E0277`.

error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:72:15
   |
72 |             ..Self::default()
   |               ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: unnecessary map of the identity function
   --> ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:137:67
    |
137 |     let target = store::os_io::ArtifactRef::parse_uri(&target_uri).map_err(|e| e)?;
    |                                                                   ^^^^^^^^^^^^^^^ help: remove the call to `map_err`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_identity
    = note: `-D clippy::map-identity` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::map_identity)]`


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:306:1
    |
306 | / fn shooting_bounded_reduce(
307 | |     command: &ShootingCommand,
308 | |     snapshot: &ShootingSnapshot,
309 | |     config: &ShootingConfig,
...   |
314 | |     operation: &AppOperationContext,
315 | | ) -> Result<Emit<ShootingMutation, ShootingConfigMutation, NoDraftMutation>, Fault> {
    | |___________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs:22:1
   |
22 | / impl Default for ShootingPresence {
23 | |     fn default() -> Self {
24 | |         Self { selected_shot_ids: Vec::new(), camera: ShootingCamera::default() }
25 | |     }
26 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
12 + #[derive(Default)]
13 | pub struct ShootingPresence {
   |


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:385:36
    |
385 |     pub fn from_snapshot(snapshot: SequenceSnapshot) -> Self {
    |                                    ^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
    |
385 |     pub fn from_snapshot(snapshot: &SequenceSnapshot) -> Self {
    |                                    +


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:36:70
   |
36 |         Self { schema: snapshot.schema, objects: snapshot.objects, ..Self::default() }
   |                                                                      ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: clamp-like pattern without using clamp function
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs:1215:32
     |
1215 |                     let load = work.remaining_load_w.min(100_000.0).max(0.0);
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace with clamp: `work.remaining_load_w.clamp(0.0, 100_000.0)`
     |
     = note: clamp will panic if max < min, min.is_nan(), or max.is_nan()
     = note: clamp returns NaN if the input is NaN
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_clamp
     = note: `-D clippy::manual-clamp` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::manual_clamp)]`


error: this function's return value is unnecessarily wrapped by `Option`
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs:1446:1
     |
1446 | fn finish_schedule_lookup(work: &mut Option<ScheduleLookupWork>, value: f64) -> Option<f64> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
     = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Option` from the return type...
     |
1446 - fn finish_schedule_lookup(work: &mut Option<ScheduleLookupWork>, value: f64) -> Option<f64> {
1446 + fn finish_schedule_lookup(work: &mut Option<ScheduleLookupWork>, value: f64) -> f64 {
     |
help: ...and then remove the surrounding `Some()` from returning expressions
     |
1448 -     Some(value)
1448 +     value
     |


error: large size difference between variants
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:97:1
    |
 97 | / enum EnergyWireLeaseRecoverySlot {
 98 | |     Vacant,
 99 | |     Reserved(u64),
    | |     ------------- the second-largest variant contains at least 8 bytes
100 | |     Abandoned(u64, EnergyWirePacket),
    | |     -------------------------------- the largest variant contains at least 3192 bytes
101 | | }
    | |_^ the entire enum is at least 3192 bytes
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    = note: `-D clippy::large-enum-variant` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
100 -     Abandoned(u64, EnergyWirePacket),
100 +     Abandoned(u64, Box<EnergyWirePacket>),
    |


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:162:53
    |
162 |     fn push(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3184 bytes
    |
    = help: try reducing the size of `sim::EnergyWirePacket`, for example by boxing large elements or replacing it with `Box<sim::EnergyWirePacket>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err
    = note: `-D clippy::result-large-err` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::result_large_err)]`


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:180:62
    |
180 |     fn push_reserved(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
    |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3184 bytes
    |
    = help: try reducing the size of `sim::EnergyWirePacket`, for example by boxing large elements or replacing it with `Box<sim::EnergyWirePacket>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:209:56
    |
209 |     fn retry(&mut self, mut lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
    |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3200 bytes
    |
    = help: try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:225:54
    |
225 |     fn ack(&mut self, mut lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3200 bytes
    |
    = help: try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:244:63
    |
244 |     fn ack_transfer(&mut self, mut lease: EnergyWireLease) -> Result<EnergyWirePacket, EnergyWireLease> {
    |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3200 bytes
    |
    = help: try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:545:58
    |
545 |     pub fn retry(self, bounds: EnergyNumericalBounds) -> Result<EnergyJob, Self> {
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 1144 bytes
    |
    = help: try reducing the size of `sim::EnergyAdmissionRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyAdmissionRejected>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:588:58
    |
588 |     pub fn retry(self, bounds: EnergyNumericalBounds) -> Result<EnergyRestoreJob, Self> {
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 4328 bytes
    |
    = help: try reducing the size of `sim::EnergyCheckpointRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyCheckpointRejected>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:698:140
    |
698 | ...ergyNumericalBounds) -> Result<Self, EnergyCheckpointRejected> {
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 4328 bytes
    |
    = help: try reducing the size of `sim::EnergyCheckpointRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyCheckpointRejected>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:994:59
    |
994 |     pub fn finish(mut self, context: &StepContext<'_>) -> Result<EnergyJob, Self> {
    |                                                           ^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 10928 bytes
    |
    = help: try reducing the size of `sim::EnergyRestoreJob`, for example by boxing large elements or replacing it with `Box<sim::EnergyRestoreJob>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:1897:81
     |
1897 |     pub fn new(operation: Operation, model: Model, config: SimulationConfig) -> Result<Self, EnergyAdmissionRejected> {
     |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 1144 bytes
     |
     = help: try reducing the size of `sim::EnergyAdmissionRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyAdmissionRejected>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:1901:114
     |
1901 | ...ergyNumericalBounds) -> Result<Self, EnergyAdmissionRejected> {
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 1144 bytes
     |
     = help: try reducing the size of `sim::EnergyAdmissionRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyAdmissionRejected>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2015:73
     |
2015 |     pub fn retry_preview_packet(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
     |                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3184 bytes
     |
     = help: try reducing the size of `sim::EnergyWirePacket`, for example by boxing large elements or replacing it with `Box<sim::EnergyWirePacket>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2030:74
     |
2030 |     pub fn retry_checkpoint_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
     |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3200 bytes
     |
     = help: try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2034:72
     |
2034 |     pub fn ack_checkpoint_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
     |                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3200 bytes
     |
     = help: try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2038:77
     |
2038 |     pub fn ack_checkpoint_for_restore(&mut self, lease: EnergyWireLease) -> Result<EnergyWirePacket, EnergyWireLease> {
     |                                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3200 bytes
     |
     = help: try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2049:70
     |
2049 |     pub fn retry_commit_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
     |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3200 bytes
     |
     = help: try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2053:68
     |
2053 |     pub fn ack_commit_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
     |                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3200 bytes
     |
     = help: try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2070:69
     |
2070 |     pub fn retry_fault_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
     |                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3200 bytes
     |
     = help: try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2074:67
     |
2074 |     pub fn ack_fault_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
     |                                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 3200 bytes
     |
     = help: try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:4611:59
     |
4611 |     pub fn job(model: Model, config: SimulationConfig) -> Result<EnergyJob, EnergyAdmissionRejected> {
     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 1144 bytes
     |
     = help: try reducing the size of `sim::EnergyAdmissionRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyAdmissionRejected>`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: large size difference between variants
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs:937:1
    |
937 | / enum MountedAdmissionError {
938 | |     Stale { checkpoint: Option<EnergyWirePacket> },
    | |     ---------------------------------------------- the largest variant contains at least 3184 bytes
939 | |     Rejected(&'static str),
    | |     ---------------------- the second-largest variant contains at least 8 bytes
940 | | }
    | |_^ the entire enum is at least 3184 bytes
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
938 -     Stale { checkpoint: Option<EnergyWirePacket> },
938 +     Stale { checkpoint: Box<Option<EnergyWirePacket>> },
    |


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs:989:151
    |
938 |     Stale { checkpoint: Option<EnergyWirePacket> },
    |     ---------------------------------------------- the largest variant contains at least 3184 bytes
...
989 |     fn admit_job(&mut self, render: AppRenderOperationContext, live_request: u64, expected: MountedIdentity, checkpoint: Option<EnergyWirePacket>) -> Result<(), MountedAdmissionError> {
    |                                                                                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: try reducing the size of `energy_simulation_session::MountedAdmissionError`, for example by boxing large elements or replacing it with `Box<energy_simulation_session::MountedAdmissionError>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: this argument is passed by value, but not consumed in the function body
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs:1069:44
     |
1069 |     fn install_preview(&mut self, preview: EnergyJobPreview) -> bool {
     |                                            ^^^^^^^^^^^^^^^^
     |
help: or consider marking this type as `Copy`
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:396:1
     |
 396 | pub struct EnergyJobPreview {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
     = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
     |
1069 |     fn install_preview(&mut self, preview: &EnergyJobPreview) -> bool {
     |                                            +


error: unnecessary map of the identity function
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:1170:115
     |
1170 | ...ack::json::from_dsl_value(&value)).map_err(|error| error)?;
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^ help: remove the call to `map_err`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_identity
     = note: `-D clippy::map-identity` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::map_identity)]`


error: unnecessary map of the identity function
    --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:1171:122
     |
1171 | ...ack::json::from_dsl_value(&value)).map_err(|error| error)?;
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^ help: remove the call to `map_err`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_identity


error: consider adding a `;` to the last statement for consistent formatting
   --> ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🦀️.rs:326:21
    |
326 | ...   diagnostics.push(EpJsonDiagnostic::new("epjson.construction.unknown-layer", name, format!("layer {index} names {layer:?}, which this document does not define as a Material, Material:NoMass or WindowMaterial:SimpleGlazingSystem")))
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add a `;` here: `diagnostics.push(EpJsonDiagnostic::new("epjson.construction.unknown-layer", name, format!("layer {index} names {layer:?}, which this document does not define as a Material, Material:NoMass or WindowMaterial:SimpleGlazingSystem")));`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#semicolon_if_nothing_returned
    = note: `-D clippy::semicolon-if-nothing-returned` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::semicolon_if_nothing_returned)]`


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:40:133
   |
40 | ...tle, layers: snapshot.layers, assets: snapshot.assets, ..Self::default() }
   |                                                             ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: large size difference between variants
    --> ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:867:9
     |
 867 | /         enum $state {
 868 | |             AwaitToken,
     | |             ---------- the second-largest variant carries no data at all
 869 | |             Decode(store::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>),
     | |             ---------------------------------------------------------------- the largest variant contains at least 320 bytes
 870 | |             Ready,
...    |
 873 | |             Complete,
 874 | |         }
     | |_________^ the entire enum is at least 320 bytes
...
1011 | / raster_owned_field_authority!(
1012 | |     RasterSnapshotDecodeState,
1013 | |     RasterSnapshotDecodeAuthority,
1014 | |     RasterSnapshot,
...    |
1020 | |     "snapshot"
1021 | | );
     | |_- in this macro invocation
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
     = note: `-D clippy::large-enum-variant` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
     = note: this error originates in the macro `raster_owned_field_authority` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
     |
 869 -             Decode(store::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>),
 869 +             Decode(Box<store::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>>),
     |


error: large size difference between variants
    --> ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:867:9
     |
 867 | /         enum $state {
 868 | |             AwaitToken,
     | |             ---------- the second-largest variant carries no data at all
 869 | |             Decode(store::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>),
     | |             ---------------------------------------------------------------- the largest variant contains at least 320 bytes
 870 | |             Ready,
...    |
 873 | |             Complete,
 874 | |         }
     | |_________^ the entire enum is at least 320 bytes
...
1023 | / raster_owned_field_authority!(
1024 | |     RasterMutationDecodeState,
1025 | |     RasterMutationDecodeAuthority,
1026 | |     RasterMutation,
...    |
1032 | |     "mutation"
1033 | | );
     | |_- in this macro invocation
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
     = note: this error originates in the macro `raster_owned_field_authority` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
     |
 869 -             Decode(store::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>),
 869 +             Decode(Box<store::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>>),
     |


error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1245:5
     |
1245 |     fn next<'a, T>(&self, values: &'a RasterOwnedMap<T>) -> Result<Option<(&'a String, &'a T)>, &'static str> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
     = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
     |
1245 -     fn next<'a, T>(&self, values: &'a RasterOwnedMap<T>) -> Result<Option<(&'a String, &'a T)>, &'static str> {
1245 +     fn next<'a, T>(&self, values: &'a RasterOwnedMap<T>) -> std::option::Option<(&std::string::String, &T)> {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
1246 -         Ok(values.entry_at(self.index))
1246 +         values.entry_at(self.index)
     |


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:284:1
    |
284 | / fn raster_retained_reduce(
285 | |     command: &RasterCommand,
286 | |     snapshot: &RasterSnapshot,
287 | |     config: &RasterConfig,
...   |
292 | |     operation: &AppOperationContext,
293 | | ) -> Result<Emit<RasterMutation, RasterConfigMutation, NoDraftMutation>, Fault> {
    | |_______________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: empty lines after doc comment
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust/../../🦀️.rs:148:1
    |
148 | / /// 📋️ Inputs for annual energy balancing.
...   |
165 | | /// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
166 | |
    | |_^
167 |   pub type BalancingInputs = Din18599Snapshot;
    |   ------------------------ the comment documents this type alias
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_doc_comments
    = note: `-D clippy::empty-line-after-doc-comments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::empty_line_after_doc_comments)]`
    = help: if the empty lines are unintentional, remove them
help: if the doc comment should not document type alias `BalancingInputs` then comment it out
    |
148 ~ // /// 📋️ Inputs for annual energy balancing.
149 | // BalancingInputs remains the nested persistent payload type; snapshot is Din18599Snapshot.
...
164 |
165 ~ // /// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
    |


error: calls to `push` immediately after creation
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:95:9
    |
 95 | /         let mut mutations = Vec::with_capacity(13);
 96 | |         mutations.push(Din18599Mutation::ChangeUseClass(change_use_class::ChangeUseClass { new_use_class: snapshot.use_class }));
 97 | |         mutations.push(Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::ChangeHeatedAreaM2 { new_heated_area_m2: snapshot.heated_a...
 98 | |         mutations.push(Din18599Mutation::ChangeOccupants(change_occupants::ChangeOccupants { new_occupants: snapshot.occupants }));
...   |
107 | |         mutations.push(Din18599Mutation::ChangeReferenceQPKwh(change_reference_q_p_kwh::ChangeReferenceQPKwh { new_reference_q_p_kwh: snapsho...
108 | |         mutations.push(Din18599Mutation::UpdateClimate(update_climate::UpdateClimate { new_climate: crate::din18599_climate(snapshot) }));
    | |__________________________________________________________________________________________________________________________________________^ help: consider using the `vec![]` macro: `let mutations = vec![..];`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#vec_init_then_push
    = note: `-D clippy::vec-init-then-push` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::vec_init_then_push)]`


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:38:111
   |
38 |         Self { schema: snapshot.schema, presentation: snapshot.presentation, animation: snapshot.animation, ..Self::default() }
   |                                                                                                               ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: large size difference between variants
   --> ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs:99:1
    |
 99 | / enum PresentationPackSnapshotState {
100 | |     AwaitToken,
    | |     ---------- the second-largest variant carries no data at all
101 | |     Decode(store::OwnedSchemaHexAuthority<PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES>),
    | |     --------------------------------------------------------------------------------- the largest variant contains at least 320 bytes
102 | |     Ready,
...   |
105 | |     Complete,
106 | | }
    | |_^ the entire enum is at least 320 bytes
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    = note: `-D clippy::large-enum-variant` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
101 -     Decode(store::OwnedSchemaHexAuthority<PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES>),
101 +     Decode(Box<store::OwnedSchemaHexAuthority<PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES>>),
    |


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs:338:61
    |
338 |     fn try_adopt(&mut self, value: PresentationSnapshot) -> Result<(), PresentationSnapshot>;
    |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 148 bytes
    |
    = help: try reducing the size of `standards::v1::subsets::any::schema::snapshot::component::PresentationSnapshot`, for example by boxing large elements or replacing it with `Box<standards::v1::subsets::any::schema::snapshot::component::PresentationSnapshot>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err
    = note: `-D clippy::result-large-err` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::result_large_err)]`


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs:452:40
    |
452 |     fn pump_field_return(&mut self) -> Result<bool, store::OwnedSchemaDecodeDiagnostic> {
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 288 bytes
    |
    = help: try reducing the size of `dsl::OwnedSchemaDecodeDiagnostic`, for example by boxing large elements or replacing it with `Box<dsl::OwnedSchemaDecodeDiagnostic>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs:480:44
    |
480 |     fn begin_completed_close(&mut self) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 288 bytes
    |
    = help: try reducing the size of `dsl::OwnedSchemaDecodeDiagnostic`, for example by boxing large elements or replacing it with `Box<dsl::OwnedSchemaDecodeDiagnostic>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:287:1
    |
287 | / fn animate_presentation_retained_reduce(
288 | |     command: &PresentationCommand,
289 | |     snapshot: &PresentationSnapshot,
290 | |     config: &PresentationConfig,
...   |
295 | |     operation: &AppOperationContext,
296 | | ) -> Result<Emit<PresentationMutation, PresentationConfigMutation, NoDraftMutation>, Fault> {
    | |___________________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:70:1
   |
70 | / impl Default for PresentationConfig {
71 | |     fn default() -> Self {
72 | |         Self { engagement_input: String::new() }
73 | |     }
74 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
21 + #[derive(Default)]
22 | pub struct PresentationConfig {
   |


error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:49:162
   |
49 | ... assets: snapshot.assets, artboard: snapshot.artboard, ..Self::default() }
   |                                                             ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


error: large size difference between variants
   --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:453:9
    |
453 | /         enum $state {
454 | |             AwaitToken,
    | |             ---------- the second-largest variant carries no data at all
455 | |             Decode(store::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>),
    | |             ----------------------------------------------------------------- the largest variant contains at least 320 bytes
456 | |             Ready,
...   |
459 | |             Complete,
460 | |         }
    | |_________^ the entire enum is at least 320 bytes
...
576 | / drawing_owned_field_authority!(
577 | |     DrawingSnapshotDecodeState,
578 | |     DrawingSnapshotDecodeAuthority,
579 | |     DrawingSnapshot,
...   |
585 | |     "snapshot"
586 | | );
    | |_- in this macro invocation
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    = note: `-D clippy::large-enum-variant` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
    = note: this error originates in the macro `drawing_owned_field_authority` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
455 -             Decode(store::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>),
455 +             Decode(Box<store::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>>),
    |


error: large size difference between variants
   --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:453:9
    |
453 | /         enum $state {
454 | |             AwaitToken,
    | |             ---------- the second-largest variant carries no data at all
455 | |             Decode(store::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>),
    | |             ----------------------------------------------------------------- the largest variant contains at least 320 bytes
456 | |             Ready,
...   |
459 | |             Complete,
460 | |         }
    | |_________^ the entire enum is at least 320 bytes
...
588 | / drawing_owned_field_authority!(
589 | |     DrawingMutationDecodeState,
590 | |     DrawingMutationDecodeAuthority,
591 | |     DrawingMutation,
...   |
597 | |     "mutation"
598 | | );
    | |_- in this macro invocation
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    = note: this error originates in the macro `drawing_owned_field_authority` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
455 -             Decode(store::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>),
455 +             Decode(Box<store::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>>),
    |


error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:1676:5
     |
1676 |     fn skeleton(source: &DrawingLayerNode) -> Result<DrawingLayerNode, &'static str> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
     = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
     |
1676 -     fn skeleton(source: &DrawingLayerNode) -> Result<DrawingLayerNode, &'static str> {
1676 +     fn skeleton(source: &DrawingLayerNode) -> DrawingLayerNode {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
1677 ~         match source {
1678 +             DrawingLayerNode::Shape(value) => DrawingLayerNode::Shape(crate::DrawingShapeBody {
1679 +                 base: Self::base_skeleton(&value.base),
1680 +                 shape_kind: String::new(),
1681 +                 rect: value.rect.as_ref().map(Self::rect),
1682 +                 ellipse: value.ellipse.as_ref().map(|source| crate::DrawingEllipse { cx: source.cx, cy: source.cy, rx: source.rx, ry: source.ry }),
1683 +                 circle: value.circle.as_ref().map(|source| crate::DrawingCircle { cx: source.cx, cy: source.cy, r: source.r }),
1684 +                 line: value.line.as_ref().map(|source| crate::DrawingLine { x1: source.x1, y1: source.y1, x2: source.x2, y2: source.y2 }),
1685 +                 polygon: value.polygon.as_ref().map(|source| crate::DrawingPolygon { points: Vec::with_capacity(source.points.len()) }),
1686 +             }),
1687 +             DrawingLayerNode::Path(value) => DrawingLayerNode::Path(crate::DrawingPathBody { base: Self::base_skeleton(&value.base), segments: Vec::with_capacity(value.segments.len()) }),
1688 +             DrawingLayerNode::Text(value) => DrawingLayerNode::Text(crate::DrawingTextBody { base: Self::base_skeleton(&value.base), x: value.x, y: value.y, content: String::new(), size: value.size }),
1689 +             DrawingLayerNode::Image(value) => DrawingLayerNode::Image(crate::DrawingImageBody { base: Self::base_skeleton(&value.base), image_key: String::new(), width: value.width, height: value.height }),
1690 +             DrawingLayerNode::Group(value) => DrawingLayerNode::Group(crate::DrawingGroupBody { base: Self::base_skeleton(&value.base), children: Vec::with_capacity(value.children.len()) }),
1691 +             DrawingLayerNode::Boolean(value) => DrawingLayerNode::Boolean(crate::DrawingBooleanBody { base: Self::base_skeleton(&value.base), operation: String::new(), children: Vec::with_capacity(value.children.len()) }),
1692 +             DrawingLayerNode::Trace(value) => DrawingLayerNode::Trace(crate::DrawingTraceBody {
1693 +                 base: Self::base_skeleton(&value.base),
1694 +                 source_key: String::new(),
1695 +                 params: crate::DrawingTraceParams { threshold: value.params.threshold, simplify_epsilon: value.params.simplify_epsilon },
1696 +             }),
1697 +         }
     |


error: this function has too many arguments (8/7)
    --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:2907:5
     |
2907 |     fn option(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, tag: u16, present: bool, next: u8, absent: u8, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
     = note: `-D clippy::too-many-arguments` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:3397:5
     |
3397 |     fn close_step(&mut self, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Result` from the return type...
     |
3397 -     fn close_step(&mut self, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
3397 +     fn close_step(&mut self, _maximum_bytes: usize) -> dsl::SnapshotRetirementStep {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
3405 -         Ok(store::SnapshotRetirementStep::Complete)
3405 +         store::SnapshotRetirementStep::Complete
     |


error: this function has too many arguments (8/7)
    --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:3446:5
     |
3446 | /     fn admit(
3447 | |         source: DrawingSnapshotOwnerTotals,
3448 | |         mutation: DrawingSemanticDigestTotals,
3449 | |         operation: &DrawingMutation,
...    |
3454 | |         duplicate_id_bytes: usize,
3455 | |     ) -> Result<Self, &'static str> {
     | |___________________________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:3712:5
     |
3712 |     fn close_step(&mut self, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Result` from the return type...
     |
3712 -     fn close_step(&mut self, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
3712 +     fn close_step(&mut self, _maximum_bytes: usize) -> dsl::SnapshotRetirementStep {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
3716 -         Ok(store::SnapshotRetirementStep::Complete)
3716 +         store::SnapshotRetirementStep::Complete
     |


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:498:37
    |
498 | pub fn diff_from_snapshot(snapshot: DrawingSnapshot) -> DrawingDiff {
    |                                     ^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
    |
498 | pub fn diff_from_snapshot(snapshot: &DrawingSnapshot) -> DrawingDiff {
    |                                     +


error: this function has too many arguments (9/7)
   --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:221:5
    |
221 | /     fn dispatch(
222 | |         &mut self,
223 | |         key: semio_framework_job::FixedOperationKey,
224 | |         base_revision: [u8; 32],
...   |
230 | |         operation: semio_framework_plugin::AppOperationContext,
231 | |     ) -> Result<Option<Emit<DrawingMutation, DrawingConfigMutation, NoDraftMutation>>, Fault> {
    | |_____________________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:824:1
    |
824 | / fn drawing_bounded_reduce(
825 | |     command: &DrawingCommand,
826 | |     snapshot: &DrawingSnapshot,
827 | |     config: &DrawingConfig,
...   |
832 | |     operation: &semio_framework_plugin::AppOperationContext,
833 | | ) -> Result<Emit<DrawingMutation, DrawingConfigMutation, NoDraftMutation>, Fault> {
    | |_________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:77:1
   |
77 | / impl Default for DrawingConfig {
78 | |     fn default() -> Self {
79 | |         Self { engagement_input: String::new(), camera: DrawingCamera::default(), trace_pointer_generation: 0, trace_pointer_compl...
80 | |     }
81 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
18 + #[derive(Default)]
19 | pub struct DrawingConfig {
   |


error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs:22:1
   |
22 | / impl Default for DrawingPresence {
23 | |     fn default() -> Self {
24 | |         Self { engagement_input: String::new(), camera: DrawingCamera::default() }
25 | |     }
26 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
help: replace the manual implementation with a derive attribute
   |
16 + #[derive(Default)]
17 | pub struct DrawingPresence {
   |


error: this function's return value is unnecessarily wrapped by `Option`
    --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:1016:1
     |
1016 | fn queue_trace_pointer(payload: &CanvasPointerDown, job: &TracePointerJob) -> Option<Effect> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Option` from the return type...
     |
1016 - fn queue_trace_pointer(payload: &CanvasPointerDown, job: &TracePointerJob) -> Option<Effect> {
1016 + fn queue_trace_pointer(payload: &CanvasPointerDown, job: &TracePointerJob) -> semio_framework_plugin::Effect {
     |
help: ...and then remove the surrounding `Some()` from returning expressions
     |
1030 -     Some(Effect::DispatchAction { req: RequestId(NEXT_TRACE_POINTER_REQUEST.fetch_add(1, Ordering::Relaxed)), action: "canvasPointerDown".into(), args, delay_ms: 0 })
1030 +     Effect::DispatchAction { req: RequestId(NEXT_TRACE_POINTER_REQUEST.fetch_add(1, Ordering::Relaxed)), action: "canvasPointerDown".into(), args, delay_ms: 0 }
     |


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:760:92
    |
760 | pub fn patch_register_item_operation(program: &ProgramSnapshot, register: &str, entity_id: EntityId, patch: Value) -> Option<Progra...
    |                                                                                            ^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
    |
760 | pub fn patch_register_item_operation(program: &ProgramSnapshot, register: &str, entity_id: &EntityId, patch: Value) -> Option<ProgramMutation> {
    |                                                                                            +


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:760:109
    |
760 | pub fn patch_register_item_operation(program: &ProgramSnapshot, register: &str, entity_id: EntityId, patch: Value) -> Option<Progra...
    |                                                                                                             ^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
760 | pub fn patch_register_item_operation(program: &ProgramSnapshot, register: &str, entity_id: EntityId, patch: &Value) -> Option<ProgramMutation> {
    |                                                                                                             +

