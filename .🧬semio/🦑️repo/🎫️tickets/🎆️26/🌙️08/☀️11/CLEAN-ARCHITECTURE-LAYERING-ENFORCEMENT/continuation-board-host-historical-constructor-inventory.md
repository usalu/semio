# Historical BoardHost Constructor Allocation Inventory

This read-only inventory binds Engine4 proposed General Board source. Later BoardAPI6 removes the temporary BoardHost construction route; this report does not reinterpret the historical constructor as the later controlled endpoint or claim native allocation measurements.

Beyond the five empty entity maps, BoardHost::default eagerly seeds five built-in edge-tip keys. Each lookup lowercases an owned temporary string, then owns the key and inserts a tree entry. It also owns rectangle/replace selection-option strings and the world-clip mode string. Nested constructors initialize a boxed event-slot array, a boxed icon cache slot array, and two BrushCandidatePage boxes (byte buffer and optional-entry table). Those are separate allocation boundaries beyond node/handle/edge/wire/region maps. CanvasPalette delegates to BoardPalette conversion and constructs inline Color wrappers; the inspected Color::new body stores four floats without an owned heap object. Empty selection/catalog collections and scalar/None fields are distinct from these populated/boxed operations.

Required controlled cuts follow each seeded tip/key, owned string, event/cache/page backing allocation and initialization work; the caller must retain partially initialized owners across refusal/cancellation and retire completed members under item and byte credits. A five-map-only preparation control would not cover these nested costs. Actual allocation counts/bytes, allocator failure and interruption remain unmeasured here. Later constructor avoidance should be assessed through its final source cuts and owning runtime evidence, not this historical default.

## impl Default for BoardHost

Owner: 🧰️framework/🔨️modules/🖱️ui/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs. Full source SHA d7d3cd6159e781bede9d1033b84d022b90c473e99e74b336f95e3f0b57cc715c; exact body SHA 9cd74975a0cce005927b00a512edd6d21e80246e710ef1680fd4ba65f342a02d.

```rust
impl Default for BoardHost {
        fn default() -> Self {
            Self {
                camera: Camera::default(),
                nodes: BTreeMap::new(),
                handles: BTreeMap::new(),
                edges: BTreeMap::new(),
                wires: BTreeMap::new(),
                regions: BTreeMap::new(),
                handle_kinds: BTreeMap::new(),
                wire_kinds: BTreeMap::new(),
                node_kinds: BTreeMap::new(),
                edge_kinds: BTreeMap::new(),
                edge_tips: builtin_edge_tips(),
                link_compat_rules: Vec::new(),
                selection: BTreeSet::new(),
                preselect: BTreeSet::new(),
                preselect_removed: BTreeSet::new(),
                selection_exit_highlight: BTreeSet::new(),
                selection_options: SelectionOptions { method: "rectangle".into(), mode: "replace".into(), select_nodes: true, select_edges: true, select_handles: true },
                hovered_id: None,
                hovered_kind: None,
                highlighted_ids: BTreeSet::new(),
                interaction: Interaction::None,
                width: 1,
                height: 1,
                dpr: 1.0,
                world_raster_tiling: "world-clip".into(),
                events: BoardEventQueue::default(),
                event_overflow: None,
                event_batch_overflow: None,
                event_schema_fault: false,
                selection_screen_preview: None,
                selection_preview_crossing: false,
                link_screen_preview: None,
                canvas_theme: CanvasPalette::default(),
                grid_visible: true,
                grid_factor: GRID_FACTOR_DEFAULT,
                grid_snap_enabled: false,
                preserve_original_element_style: false,
                automatic_lod: true,
                forced_draw_lod: None,
                icon_paint_cache: IconPaintCache::new(),
                link_compat_nodes_emit_key: None,
                link_target_ring_emit_key: None,
                last_select_emit_sig: None,
                last_preselect_emit_sig: None,
                content_scene_generation: 0,
                world_content_cache: RefCell::new(None),
                opaque_scene_retirement: Cell::new(None),
                opaque_scene_fault: Cell::new(false),
                wheel_zoom_active: false,
                wheel_zoom_render_lod: None,
                active_utility: ActiveUtility::Select,
                suggestion_offset: DEFAULT_SUGGESTION_OFFSET,
                brush_node_size: DEFAULT_BRUSH_NODE_SIZE,
                brush_slot_source_id: None,
                brush_candidates: BrushCandidatePage::default(),
                brush_candidate_index: 0,
                brush_preview: None,
                fixture_drop_preview: None,
                brush_candidates_emit_key: None,
                brush_preview_emit_key: None,
                brush_placement_serial: 0,
                brush_node_kind_weights: HashMap::new(),
                brush_handle_kind_weights: HashMap::new(),
                brush_alt_pressed: false,
                brush_slot_suggestions_active: false,
                transform_flags: TransformGumballFlags::default(),
                transform_drag: None,
                region_drag: None,
                region_paint: None,
                area_brush_extent: (REGION_DEFAULT_BRUSH_EXTENT_WORLD, REGION_DEFAULT_BRUSH_EXTENT_WORLD),
                port_mode: GraphPortMode::Ported,
                interaction_revision: 0,
                gesture_serial: 0,
                pending_delete_planning: None,
                pending_delete_operation: None,
                pending_pointer_commit: None,
                queued_pointer_commit: None,
                cancelled_pointer_plan_retirement: None,
                pointer_publication: None,
                close_phase: BoardHostClosePhase::Events,
                close_entity_retirement: None,
                close_strings: std::array::from_fn(|_| None),
                close_string_len: 0,
                close_node_handles: None,
            }
        }
    }

```

## pub fn builtin_edge_tips

Owner: 🧰️framework/🔨️modules/🖱️ui/🎲️board/🔌️ports/➡️directed/🦀️.rs. Full source SHA 37cd7cca67827ac56b7e2d2b617ba7d106a9746df57654d25141255675c6c447; exact body SHA 160fab94d36327c00b13118f150f7b7dac5b62999f4754905c935ab800a63d45.

```rust
pub fn builtin_edge_tips() -> BTreeMap<String, EdgeTipDef> {
        let ids = ["arrow", "filled-arrow", "fine-arrow", "filled-diamond", "open-diamond"];
        let mut m = BTreeMap::new();
        for id in ids {
            if let Some(def) = EdgeTipDef::builtin_for_id(id) {
                m.insert(id.to_string(), def);
            }
        }
        m
    }

```

## impl Default for BoardEventQueue

Owner: 🧰️framework/🔨️modules/🖱️ui/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs. Full source SHA d7d3cd6159e781bede9d1033b84d022b90c473e99e74b336f95e3f0b57cc715c; exact body SHA 5f476a2bd48a6bdfba55918ceec04fd90033c2bd80b8eb8f48b53cc41be4be28.

```rust
impl Default for BoardEventQueue {
        fn default() -> Self {
            Self { slots: semio_framework_async::boxed_fixed_slots(|| None), head: 0, len: 0, bytes: 0, claimed_items: 0, claimed_bytes: 0, closing: false }
        }
    }

```

## impl Default for IconPaintRegistry

Owner: 🧰️framework/🔨️modules/🖱️ui/🎲️board/🔌️ports/➡️directed/🦀️.rs. Full source SHA 37cd7cca67827ac56b7e2d2b617ba7d106a9746df57654d25141255675c6c447; exact body SHA 4711ce80a42b903b1631927e68576598abb721decff1ea434b0324501ab7de94.

```rust
impl Default for IconPaintRegistry {
        fn default() -> Self {
            Self { slots: semio_framework_async::boxed_fixed_slots(|| IconPaintSlot { key: None, epoch: 0, generation: 0, value: None }), epoch: 1, faulted: false }
        }
    }

```

## impl Default for BrushCandidatePage

Owner: 🧰️framework/🔨️modules/🖱️ui/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs. Full source SHA d7d3cd6159e781bede9d1033b84d022b90c473e99e74b336f95e3f0b57cc715c; exact body SHA 4df74d033c552479d9e8f51703fc3e68b5eb7e9772e57aad794ceab60b6e1335.

```rust
impl Default for BrushCandidatePage {
        fn default() -> Self {
            Self { bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]), byte_len: 0, entries: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]), len: 0 }
        }
    }

```

## impl Default for CanvasPalette

Owner: 🧰️framework/🔨️modules/🖱️ui/🎲️board/🔌️ports/➡️directed/🦀️.rs. Full source SHA 37cd7cca67827ac56b7e2d2b617ba7d106a9746df57654d25141255675c6c447; exact body SHA d91228ba66e6008fc23da9fb0333f677a93d694c7567166665407b3a7c36e5c6.

```rust
impl Default for CanvasPalette {
        fn default() -> Self {
            Self::from_board_palette(&ui_styling::BOARD_LIGHT)
        }
    }

```

## fn from_board_palette

Owner: 🧰️framework/🔨️modules/🖱️ui/🎲️board/🔌️ports/➡️directed/🦀️.rs. Full source SHA 37cd7cca67827ac56b7e2d2b617ba7d106a9746df57654d25141255675c6c447; exact body SHA 49651878d4054afa7220626eca23cc367ac2d2e902a0ae0130e3cefef8cc829e.

```rust
fn from_board_palette(t: &ui_styling::BoardPalette) -> Self {
            Self {
                raster_clear: Color::new(t.raster_clear),
                grid_minor_stroke: Color::new(t.grid_minor_stroke),
                edge_stroke: Color::new(t.edge_stroke),
                edge_stroke_hovered: Color::new(t.edge_stroke_hovered),
                edge_stroke_selected: Color::new(t.edge_stroke_selected),
                edge_stroke_selection_exit: Color::new(t.edge_stroke_selection_exit),
                edge_stroke_disabled: Color::new(t.edge_stroke_disabled),
                node_fill: Color::new(t.node_fill),
                node_stroke: Color::new(t.node_stroke),
                node_fill_hovered: Color::new(t.node_fill_hovered),
                node_stroke_hovered: Color::new(t.node_stroke_hovered),
                node_fill_selected: Color::new(t.node_fill_selected),
                node_stroke_selected: Color::new(t.node_stroke_selected),
                node_fill_selection_exit: Color::new(t.node_fill_selection_exit),
                node_stroke_selection_exit: Color::new(t.node_stroke_selection_exit),
                node_fill_disabled: Color::new(t.node_fill_disabled),
                node_stroke_disabled: Color::new(t.node_stroke_disabled),
                node_stroke_computing: Color::new(t.node_stroke_computing),
                node_stroke_stale: Color::new(t.node_stroke_stale),
                node_stroke_error: Color::new(t.node_stroke_error),
                node_stroke_blocked: Color::new(t.node_stroke_blocked),
                indirect_handle_fill: Color::new(t.indirect_handle_fill),
                indirect_handle_stroke: Color::new(t.indirect_handle_stroke),
                handle_fill: Color::new(t.handle_fill),
                handle_stroke: Color::new(t.handle_stroke),
                handle_fill_hovered: Color::new(t.handle_fill_hovered),
                handle_stroke_hovered: Color::new(t.handle_stroke_hovered),
                handle_fill_selected: Color::new(t.handle_fill_selected),
                handle_stroke_selected: Color::new(t.handle_stroke_selected),
                handle_fill_selection_exit: Color::new(t.handle_fill_selection_exit),
                handle_stroke_selection_exit: Color::new(t.handle_stroke_selection_exit),
                handle_fill_disabled: Color::new(t.handle_fill_disabled),
                handle_stroke_disabled: Color::new(t.handle_stroke_disabled),
                wire_stroke: Color::new(t.wire_stroke),
                wire_stroke_hovered: Color::new(t.wire_stroke_hovered),
                wire_stroke_selected: Color::new(t.wire_stroke_selected),
                wire_stroke_highlighted: Color::new(t.wire_stroke_highlighted),
                wire_stroke_disabled: Color::new(t.wire_stroke_disabled),
                selection_preview_fill: Color::new(t.selection_preview_fill),
                selection_preview_stroke: Color::new(t.selection_preview_stroke),
                label_fill: Color::new(t.label_fill),
                label_fill_hovered: Color::new(t.label_fill_hovered),
                label_halo: Color::new(t.label_halo),
                minimap_widget_panel_fill: Color::new(t.minimap_widget_panel_fill),
                minimap_widget_panel_stroke: Color::new(t.minimap_widget_panel_stroke),
                minimap_widget_viewport_fill: Color::new(t.minimap_widget_viewport_fill),
                minimap_widget_viewport_stroke: Color::new(t.minimap_widget_viewport_stroke),
                minimap_widget_viewport_stroke_hovered: Color::new(t.minimap_widget_viewport_stroke_hovered),
            }
        }

        fn merge_color_field(next: &mut Color, v: &semio_framework_value::DslValue, key: &str) {
            crate::canvas::theme::merge_color_field(next, v, key);
        }

        /// 🎨️ Replaces this palette from the React host UI theme JSON payload.
        pub fn merge_from_json(&mut self, json: &str) -> Result<(), String> {
            let v = semio_framework_value::DslValue::from(serde_json::from_str::<serde_json::Value>(json).map_err(|e| e.to_string())?);
            let mut next = Self::default();
            Self::merge_color_field(&mut next.raster_clear, &v, "rasterClear");
            Self::merge_color_field(&mut next.grid_minor_stroke, &v, "gridMinorStroke");
            Self::merge_color_field(&mut next.edge_stroke, &v, "edgeStroke");
            Self::merge_color_field(&mut next.edge_stroke_hovered, &v, "edgeStrokeHovered");
            Self::merge_color_field(&mut next.edge_stroke_selected, &v, "edgeStrokeSelected");
            Self::merge_color_field(&mut next.edge_stroke_selection_exit, &v, "edgeStrokeSelectionExit");
            Self::merge_color_field(&mut next.edge_stroke_disabled, &v, "edgeStrokeDisabled");
            Self::merge_color_field(&mut next.node_fill, &v, "nodeFill");
            Self::merge_color_field(&mut next.node_stroke, &v, "nodeStroke");
            Self::merge_color_field(&mut next.node_fill_hovered, &v, "nodeFillHovered");
            Self::merge_color_field(&mut next.node_stroke_hovered, &v, "nodeStrokeHovered");
            Self::merge_color_field(&mut next.node_fill_selected, &v, "nodeFillSelected");
            Self::merge_color_field(&mut next.node_stroke_selected, &v, "nodeStrokeSelected");
            Self::merge_color_field(&mut next.node_fill_selection_exit, &v, "nodeFillSelectionExit");
            Self::merge_color_field(&mut next.node_stroke_selection_exit, &v, "nodeStrokeSelectionExit");
            Self::merge_color_field(&mut next.node_fill_disabled, &v, "nodeFillDisabled");
            Self::merge_color_field(&mut next.node_stroke_disabled, &v, "nodeStrokeDisabled");
            Self::merge_color_field(&mut next.indirect_handle_fill, &v, "indirectHandleFill");
            Self::merge_color_field(&mut next.indirect_handle_stroke, &v, "indirectHandleStroke");
            Self::merge_color_field(&mut next.handle_fill, &v, "handleFill");
            Self::merge_color_field(&mut next.handle_stroke, &v, "handleStroke");
            Self::merge_color_field(&mut next.handle_fill_hovered, &v, "handleFillHovered");
            Self::merge_color_field(&mut next.handle_stroke_hovered, &v, "handleStrokeHovered");
            Self::merge_color_field(&mut next.handle_fill_selected, &v, "handleFillSelected");
            Self::merge_color_field(&mut next.handle_stroke_selected, &v, "handleStrokeSelected");
            Self::merge_color_field(&mut next.handle_fill_selection_exit, &v, "handleFillSelectionExit");
            Self::merge_color_field(&mut next.handle_stroke_selection_exit, &v, "handleStrokeSelectionExit");
            Self::merge_color_field(&mut next.handle_fill_disabled, &v, "handleFillDisabled");
            Self::merge_color_field(&mut next.handle_stroke_disabled, &v, "handleStrokeDisabled");
            Self::merge_color_field(&mut next.wire_stroke, &v, "wireStroke");
            Self::merge_color_field(&mut next.wire_stroke_hovered, &v, "wireStrokeHovered");
            Self::merge_color_field(&mut next.wire_stroke_selected, &v, "wireStrokeSelected");
            Self::merge_color_field(&mut next.wire_stroke_highlighted, &v, "wireStrokeHighlighted");
            Self::merge_color_field(&mut next.wire_stroke_disabled, &v, "wireStrokeDisabled");
            Self::merge_color_field(&mut next.selection_preview_fill, &v, "selectionPreviewFill");
            Self::merge_color_field(&mut next.selection_preview_stroke, &v, "selectionPreviewStroke");
            Self::merge_color_field(&mut next.label_fill, &v, "labelFill");
            Self::merge_color_field(&mut next.label_fill_hovered, &v, "labelFillHovered");
            Self::merge_color_field(&mut next.label_halo, &v, "labelHalo");
            Self::merge_color_field(&mut next.minimap_widget_panel_fill, &v, "minimapWidgetPanelFill");
            Self::merge_color_field(&mut next.minimap_widget_panel_stroke, &v, "minimapWidgetPanelStroke");
            Self::merge_color_field(&mut next.minimap_widget_viewport_fill, &v, "minimapWidgetViewportFill");
            Self::merge_color_field(&mut next.minimap_widget_viewport_stroke, &v, "minimapWidgetViewportStroke");
            Self::merge_color_field(&mut next.minimap_widget_viewport_stroke_hovered, &v, "minimapWidgetViewportStrokeHovered");
            *self = next;
            Ok(())
        }
    }

```

## impl Color

Owner: 🧰️framework/🔨️modules/🖱️ui/🖼️canvas/🦀️.rs. Full source SHA cf9325707a57d26a683d0fe0215b20a44f1396ee2f639a418b12fa42f25e1391; exact body SHA dd42cf340b45dda4f23f1c0c3e5c4100ed9834eb0a32316787d6b15e0011acbc.

```rust
impl Color {
        pub fn new(rgba: [f32; 4]) -> Self {
            Self(rgba)
        }
        pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self([r, g, b, a].map(|c| f32::from(c) * (1.0 / 255.0)))
        }
        pub fn to_rgba8(self) -> Rgba8 {
            let [r, g, b, a] = self.0.map(|c| (c * 255.0 + 0.5) as u8);
            Rgba8 { r, g, b, a }
        }
        pub fn components(self) -> [f32; 4] {
            self.0
        }
        pub fn multiply_alpha(self, alpha: f32) -> Self {
            let [r, g, b, a] = self.0;
            Self([r, g, b, a * alpha])
        }
    }

```
