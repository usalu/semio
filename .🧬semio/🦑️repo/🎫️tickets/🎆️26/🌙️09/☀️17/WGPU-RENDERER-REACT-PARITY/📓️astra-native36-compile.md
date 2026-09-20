# Native36 Compile Boundary

The full renderer census stopped before any test executed. 39 errors are from the integrated pointer ownership/Ink clipboard interfaces: unpublished PointerId paths, duplicate helper, missing macro, stale call sites and type mismatches. Assigned to the two source owners; no test is skipped or accepted from this run.

```text
error[E0428]: the name `find_ink_item` is defined multiple times
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/../../🧪️tests/🧊️wgpu-standalone/🦀️.rs:586:1
     |
 586 | fn find_ink_item<'a>(blocks: &'a [Value], id: &str) -> Option<&'a Value> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `find_ink_item` redefined here
     |
    ::: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:6254:1
     |
6254 | fn find_ink_item<'a>(blocks: &'a [Value], id: &str) -> Option<&'a Value> {
     | ------------------------------------------------------------------------ previous definition of the value `find_ink_item` here
     |
     = note: `find_ink_item` must be defined only once in the value namespace of this module

error: cannot find macro `json` in this scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:418:45
    |
418 |     assert_eq!(ink_case["publishOnCancel"], json!([]));
    |                                             ^^^^
    |
    = note: macro `crate::scenes::text_editor_tests::json` exists but is inaccessible
help: consider importing one of these macros
    |
  2 + use flow::os_pack::json;
    |
  2 + use framework_editor::os_pack::json;
    |
  2 + use framework_surface_node_graph::tiled_map::os_pack::json;
    |
  2 + use infinite_canvas::os_pack::json;
    |
    = and 2 other candidates

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:309:39
    |
309 |     pointer_id: Option<ui_wgpu::wgpu::PointerId>,
    |                                       ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this struct
    |
 11 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
309 -     pointer_id: Option<ui_wgpu::wgpu::PointerId>,
309 +     pointer_id: Option<PointerId>,
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:340:43
    |
340 |         pointer_id: Option<ui_wgpu::wgpu::PointerId>,
    |                                           ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this struct
    |
 11 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
340 -         pointer_id: Option<ui_wgpu::wgpu::PointerId>,
340 +         pointer_id: Option<PointerId>,
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:396:61
    |
396 |     fn cancel_pointer(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, owners: &[ScenePointerOwner]) {
    |                                                             ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this struct
    |
 11 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
396 -     fn cancel_pointer(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, owners: &[ScenePointerOwner]) {
396 +     fn cancel_pointer(&mut self, pointer_id: PointerId, owners: &[ScenePointerOwner]) {
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:427:32
    |
427 |     pointer_id: ui_wgpu::wgpu::PointerId,
    |                                ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this struct
    |
 11 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
427 -     pointer_id: ui_wgpu::wgpu::PointerId,
427 +     pointer_id: PointerId,
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:451:32
    |
451 |     pointer_id: ui_wgpu::wgpu::PointerId,
    |                                ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this struct
    |
 11 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
451 -     pointer_id: ui_wgpu::wgpu::PointerId,
451 +     pointer_id: PointerId,
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:470:91
    |
470 | pub fn claim_scene_pointer_at(window_id: &str, x: f32, y: f32, pointer_id: ui_wgpu::wgpu::PointerId) -> bool {
    |                                                                                           ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this struct
    |
 11 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
470 - pub fn claim_scene_pointer_at(window_id: &str, x: f32, y: f32, pointer_id: ui_wgpu::wgpu::PointerId) -> bool {
470 + pub fn claim_scene_pointer_at(window_id: &str, x: f32, y: f32, pointer_id: PointerId) -> bool {
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:481:57
    |
481 | pub fn release_scene_pointer(pointer_id: ui_wgpu::wgpu::PointerId) {
    |                                                         ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this struct
    |
 11 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
481 - pub fn release_scene_pointer(pointer_id: ui_wgpu::wgpu::PointerId) {
481 + pub fn release_scene_pointer(pointer_id: PointerId) {
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:486:32
    |
486 |     pointer_id: ui_wgpu::wgpu::PointerId,
    |                                ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this struct
    |
 11 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
486 -     pointer_id: ui_wgpu::wgpu::PointerId,
486 +     pointer_id: PointerId,
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:689:32
    |
689 |     pointer_id: ui_wgpu::wgpu::PointerId,
    |                                ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this struct
    |
 11 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
689 -     pointer_id: ui_wgpu::wgpu::PointerId,
689 +     pointer_id: PointerId,
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:749:95
    |
749 | ...iCommand], pointer_id: Option<ui_wgpu::wgpu::PointerId>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    |                                                 ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this struct
    |
 11 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
749 - fn apply_ui_commands(commands: &[ui_wgpu::wgpu::UiCommand], pointer_id: Option<ui_wgpu::wgpu::PointerId>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
749 + fn apply_ui_commands(commands: &[ui_wgpu::wgpu::UiCommand], pointer_id: Option<PointerId>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1143:39
     |
1143 |     pointer_id: Option<ui_wgpu::wgpu::PointerId>,
     |                                       ^^^^^^^^^ not found in `ui_wgpu::wgpu`
     |
help: consider importing this struct
     |
  11 + use ui_render::PointerId;
     |
help: if you import `PointerId`, refer to it directly
     |
1143 -     pointer_id: Option<ui_wgpu::wgpu::PointerId>,
1143 +     pointer_id: Option<PointerId>,
     |

error[E0425]: cannot find function, tuple struct or tuple variant `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:407:29
    |
407 |         Some(ui_wgpu::wgpu::PointerId(1)),
    |                             ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this tuple struct
    |
  2 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
407 -         Some(ui_wgpu::wgpu::PointerId(1)),
407 +         Some(PointerId(1)),
    |

error[E0425]: cannot find function, tuple struct or tuple variant `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:427:34
    |
427 |     let pointer = ui_wgpu::wgpu::PointerId(41);
    |                                  ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this tuple struct
    |
  2 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
427 -     let pointer = ui_wgpu::wgpu::PointerId(41);
427 +     let pointer = PointerId(41);
    |

error[E0425]: cannot find function, tuple struct or tuple variant `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:441:49
    |
441 |     assert!(cancel_scene_pointer(ui_wgpu::wgpu::PointerId(42), &mut input).is_empty(), "a sibling pointer cannot retire the Ink own...
    |                                                 ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this tuple struct
    |
  2 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
441 -     assert!(cancel_scene_pointer(ui_wgpu::wgpu::PointerId(42), &mut input).is_empty(), "a sibling pointer cannot retire the Ink owner");
441 +     assert!(cancel_scene_pointer(PointerId(42), &mut input).is_empty(), "a sibling pointer cannot retire the Ink owner");
    |

error[E0425]: cannot find function, tuple struct or tuple variant `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:450:39
    |
450 |     let next_pointer = ui_wgpu::wgpu::PointerId(43);
    |                                       ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this tuple struct
    |
  2 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
450 -     let next_pointer = ui_wgpu::wgpu::PointerId(43);
450 +     let next_pointer = PointerId(43);
    |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:6603:39
     |
6603 |     pointer_id: Option<ui_wgpu::wgpu::PointerId>,
     |                                       ^^^^^^^^^ not found in `ui_wgpu::wgpu`
     |
help: consider importing this struct
     |
   9 + use ui_render::PointerId;
     |
help: if you import `PointerId`, refer to it directly
     |
6603 -     pointer_id: Option<ui_wgpu::wgpu::PointerId>,
6603 +     pointer_id: Option<PointerId>,
     |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:6664:43
     |
6664 |         pointer_id: Option<ui_wgpu::wgpu::PointerId>,
     |                                           ^^^^^^^^^ not found in `ui_wgpu::wgpu`
     |
help: consider importing this struct
     |
   9 + use ui_render::PointerId;
     |
help: if you import `PointerId`, refer to it directly
     |
6664 -         pointer_id: Option<ui_wgpu::wgpu::PointerId>,
6664 +         pointer_id: Option<PointerId>,
     |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:6747:100
     |
6747 |     pub(crate) fn matches_pointer_owner(&self, surface_generation: u64, pointer_id: ui_wgpu::wgpu::PointerId) -> bool {
     |                                                                                                    ^^^^^^^^^ not found in `ui_wgpu::wgpu`
     |
help: consider importing this struct
     |
   9 + use ui_render::PointerId;
     |
help: if you import `PointerId`, refer to it directly
     |
6747 -     pub(crate) fn matches_pointer_owner(&self, surface_generation: u64, pointer_id: ui_wgpu::wgpu::PointerId) -> bool {
6747 +     pub(crate) fn matches_pointer_owner(&self, surface_generation: u64, pointer_id: PointerId) -> bool {
     |

error[E0425]: cannot find function, tuple struct or tuple variant `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs:147:34
    |
147 |     let pointer = ui_wgpu::wgpu::PointerId(7);
    |                                  ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this tuple struct
    |
  1 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
147 -     let pointer = ui_wgpu::wgpu::PointerId(7);
147 +     let pointer = PointerId(7);
    |

error[E0425]: cannot find function, tuple struct or tuple variant `PointerId` in module `ui_wgpu::wgpu`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs:153:59
    |
153 |     assert!(!job.matches_pointer_owner(11, ui_wgpu::wgpu::PointerId(8)), "a sibling pointer cannot retire the job");
    |                                                           ^^^^^^^^^ not found in `ui_wgpu::wgpu`
    |
help: consider importing this tuple struct
    |
  1 + use ui_render::PointerId;
    |
help: if you import `PointerId`, refer to it directly
    |
153 -     assert!(!job.matches_pointer_owner(11, ui_wgpu::wgpu::PointerId(8)), "a sibling pointer cannot retire the job");
153 +     assert!(!job.matches_pointer_owner(11, PointerId(8)), "a sibling pointer cannot retire the job");
    |

error[E0425]: cannot find function, tuple struct or tuple variant `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12296:55
      |
12296 |         self.handle_pointer_cancel_for(ui_wgpu::wgpu::PointerId(1), input);
      |                                                       ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this tuple struct
      |
    9 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
12296 -         self.handle_pointer_cancel_for(ui_wgpu::wgpu::PointerId(1), input);
12296 +         self.handle_pointer_cancel_for(PointerId(1), input);
      |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12299:76
      |
12299 |     pub fn handle_pointer_cancel_for(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, input: &mut InputState<ActionDescriptor>) {
      |                                                                            ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this struct
      |
    9 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
12299 -     pub fn handle_pointer_cancel_for(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, input: &mut InputState<ActionDescriptor>) {
12299 +     pub fn handle_pointer_cancel_for(&mut self, pointer_id: PointerId, input: &mut InputState<ActionDescriptor>) {
      |

error[E0425]: cannot find function, tuple struct or tuple variant `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12344:55
      |
12344 |         self.handle_pointer_button_for(ui_wgpu::wgpu::PointerId(1), x, y, down, button, input, theme).await
      |                                                       ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this tuple struct
      |
    9 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
12344 -         self.handle_pointer_button_for(ui_wgpu::wgpu::PointerId(1), x, y, down, button, input, theme).await
12344 +         self.handle_pointer_button_for(PointerId(1), x, y, down, button, input, theme).await
      |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12347:82
      |
12347 |     pub async fn handle_pointer_button_for(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, x: f32, y: f32, down: bool, button: i...
      |                                                                                  ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this struct
      |
    9 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
12347 -     pub async fn handle_pointer_button_for(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, x: f32, y: f32, down: bool, button: i16, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> Result<(), String> {
12347 +     pub async fn handle_pointer_button_for(&mut self, pointer_id: PointerId, x: f32, y: f32, down: bool, button: i16, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> Result<(), String> {
      |

error[E0425]: cannot find function, tuple struct or tuple variant `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12662:53
      |
12662 |         self.handle_pointer_move_for(ui_wgpu::wgpu::PointerId(1), x, y, down, input, theme);
      |                                                     ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this tuple struct
      |
    9 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
12662 -         self.handle_pointer_move_for(ui_wgpu::wgpu::PointerId(1), x, y, down, input, theme);
12662 +         self.handle_pointer_move_for(PointerId(1), x, y, down, input, theme);
      |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12665:74
      |
12665 |     pub fn handle_pointer_move_for(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, x: f32, y: f32, down: bool, input: &mut Input...
      |                                                                          ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this struct
      |
    9 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
12665 -     pub fn handle_pointer_move_for(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, x: f32, y: f32, down: bool, input: &mut InputState<ActionDescriptor>, theme: &Theme) {
12665 +     pub fn handle_pointer_move_for(&mut self, pointer_id: PointerId, x: f32, y: f32, down: bool, input: &mut InputState<ActionDescriptor>, theme: &Theme) {
      |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12958:74
      |
12958 |     fn route_retained_pointer_move(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, x: f32, y: f32, input: &mut InputState<Action...
      |                                                                          ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this struct
      |
    9 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
12958 -     fn route_retained_pointer_move(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, x: f32, y: f32, input: &mut InputState<ActionDescriptor>) {
12958 +     fn route_retained_pointer_move(&mut self, pointer_id: PointerId, x: f32, y: f32, input: &mut InputState<ActionDescriptor>) {
      |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12990:200
      |
12990 | ...tionDescriptor>, pointer_id: ui_wgpu::wgpu::PointerId, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
      |                                                ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this struct
      |
    9 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
12990 -     async fn route_retained_pointer_press(&mut self, window_id: &str, body: Rect, x: f32, y: f32, down: bool, button: i16, kind: HitKind, action: Option<ActionDescriptor>, pointer_id: ui_wgpu::wgpu::PointerId, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
12990 +     async fn route_retained_pointer_press(&mut self, window_id: &str, body: Rect, x: f32, y: f32, down: bool, button: i16, kind: HitKind, action: Option<ActionDescriptor>, pointer_id: PointerId, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
      |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/🦀️.rs:15524:68
      |
15524 |     fn handle_pointer_cancel(&mut self, pointer_id: ui_wgpu::wgpu::PointerId) {
      |                                                                    ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this struct
      |
    3 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
15524 -     fn handle_pointer_cancel(&mut self, pointer_id: ui_wgpu::wgpu::PointerId) {
15524 +     fn handle_pointer_cancel(&mut self, pointer_id: PointerId) {
      |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/🦀️.rs:15533:74
      |
15533 |     async fn handle_pointer_button(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, x: f32, y: f32, down: bool, button: i16, modi...
      |                                                                          ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this struct
      |
    3 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
15533 -     async fn handle_pointer_button(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
15533 +     async fn handle_pointer_button(&mut self, pointer_id: PointerId, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
      |

error[E0425]: cannot find type `PointerId` in module `ui_wgpu::wgpu`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/🦀️.rs:15736:72
      |
15736 |     async fn handle_pointer_move(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, x: f32, y: f32, down: bool, button: i16, modifi...
      |                                                                        ^^^^^^^^^ not found in `ui_wgpu::wgpu`
      |
help: consider importing this struct
      |
    3 + use ui_render::PointerId;
      |
help: if you import `PointerId`, refer to it directly
      |
15736 -     async fn handle_pointer_move(&mut self, pointer_id: ui_wgpu::wgpu::PointerId, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
15736 +     async fn handle_pointer_move(&mut self, pointer_id: PointerId, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
      |

error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../🪟️winit-app/🦀️.rs:405:53
    |
405 |                 Err(error) => app.text_fault = Some(error),
    |                                                ---- ^^^^^ expected `String`, found `&str`
    |                                                |
    |                                                arguments to this enum variant are incorrect
    |
help: the type constructed contains `&str` due to the type of the argument passed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../🪟️winit-app/🦀️.rs:405:48
    |
405 |                 Err(error) => app.text_fault = Some(error),
    |                                                ^^^^^-----^
    |                                                     |
    |                                                     this argument influences the type of `Some`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
    |
606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
help: try using a conversion method
    |
405 |                 Err(error) => app.text_fault = Some(error.to_string()),
    |                                                          ++++++++++++

error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../🪟️winit-app/🦀️.rs:417:53
    |
417 |                 Err(error) => app.text_fault = Some(error),
    |                                                ---- ^^^^^ expected `String`, found `&str`
    |                                                |
    |                                                arguments to this enum variant are incorrect
    |
help: the type constructed contains `&str` due to the type of the argument passed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../🪟️winit-app/🦀️.rs:417:48
    |
417 |                 Err(error) => app.text_fault = Some(error),
    |                                                ^^^^^-----^
    |                                                     |
    |                                                     this argument influences the type of `Some`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
    |
606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
help: try using a conversion method
    |
417 |                 Err(error) => app.text_fault = Some(error.to_string()),
    |                                                          ++++++++++++

error[E0061]: this function takes 7 arguments but 6 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:552:5
     |
 552 |     apply_scene_ui_command(window_id, node, ui_wgpu::wgpu::SurfaceKind::Canvas2d, rect, &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() }, &mut input);
     |     ^^^^^^^^^^^^^^^^^^^^^^                                                                                                                                                                                                       ---------- argument #6 is missing
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1137:4
     |
1137 | fn apply_scene_ui_command(
     |    ^^^^^^^^^^^^^^^^^^^^^^
...
1143 |     pointer_id: Option<ui_wgpu::wgpu::PointerId>,
     |     --------------------------------------------
help: provide the argument
     |
 552 |     apply_scene_ui_command(window_id, node, ui_wgpu::wgpu::SurfaceKind::Canvas2d, rect, &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() }, /* pointer_id */, &mut input);
     |                                                                                                                                                                                                                                  +++++++++++++++++

error[E0061]: this function takes 7 arguments but 6 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:570:5
     |
 570 |     apply_scene_ui_command(
     |     ^^^^^^^^^^^^^^^^^^^^^^
...
 581 |         &mut input,
     |         ---------- argument #6 is missing
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1137:4
     |
1137 | fn apply_scene_ui_command(
     |    ^^^^^^^^^^^^^^^^^^^^^^
...
1143 |     pointer_id: Option<ui_wgpu::wgpu::PointerId>,
     |     --------------------------------------------
help: provide the argument
     |
 570 |     apply_scene_ui_command(
 ...
 580 |         },
 581 ~         /* pointer_id */,
 582 ~         &mut input,
     |

error[E0061]: this method takes 10 arguments but 9 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-input/🦀️.rs:621:43
      |
  621 | ...k_on(shell.route_retained_pointer_press(pressed, body, 255.0, 90.0, true, 0, HitKind::Input, None, &mut input)).expect("a reta...
      |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------------------------------------------------------------- argument #10 of type `&mut semio_framework_plugin::InputState<semio_framework_plugin::ActionDescriptor>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12990:14
      |
12990 | ...fn route_retained_pointer_press(&mut self, window_id: &str, body: Rect, x: f32, y: f32, down: bool, button: i16, kind: HitKind, action: Option<ActionDescriptor>, pointer_id: ui_wgpu::wgpu::PointerId, input: &mut InputState<ActionDescriptor>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                                                                                                                                         ----------------------------------------
help: provide the argument
      |
  621 |     semio_framework_async::block_on(shell.route_retained_pointer_press(pressed, body, 255.0, 90.0, true, 0, HitKind::Input, None, &mut input, /* &mut semio_framework_plugin::InputState<semio_framework_plugin::ActionDescriptor> */)).expect("a retained body press routes");
      |                                                                                                                                             +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 10 arguments but 9 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-input/🦀️.rs:637:43
      |
  637 | ...ll.route_retained_pointer_press("generation3d-generations", body, 255.0, 90.0, false, 0, HitKind::Input, None, &mut input)).ex...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^------------------------------------------------------------------------------------------- argument #10 of type `&mut semio_framework_plugin::InputState<semio_framework_plugin::ActionDescriptor>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12990:14
      |
12990 | ...fn route_retained_pointer_press(&mut self, window_id: &str, body: Rect, x: f32, y: f32, down: bool, button: i16, kind: HitKind, action: Option<ActionDescriptor>, pointer_id: ui_wgpu::wgpu::PointerId, input: &mut InputState<ActionDescriptor>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                                                                                                                                         ----------------------------------------
help: provide the argument
      |
  637 |     semio_framework_async::block_on(shell.route_retained_pointer_press("generation3d-generations", body, 255.0, 90.0, false, 0, HitKind::Input, None, &mut input, /* &mut semio_framework_plugin::InputState<semio_framework_plugin::ActionDescriptor> */)).expect("a retained body release routes");
      |                                                                                                                                                                 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

error: aborting due to 39 previous errors; 183 warnings emitted


```
