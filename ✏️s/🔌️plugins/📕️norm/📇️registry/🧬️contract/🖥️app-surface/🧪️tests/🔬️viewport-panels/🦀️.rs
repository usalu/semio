//! 🧪️ Viewport panel tests: window definitions, layouts, interaction domain, the toolbar and the lane overlay surface, lifted out of the viewport3d crate because they need the higher crates. Fixtures mirror the viewport3d unit tests.

use crate::lane_overlays::{LaneOverlays, PaletteEntry};
use crate::toolbar::{render_toolbar, render_viewport_panel, ViewportView};
use crate::viewport3d::config::{DeviceClass, Lang, Viewport3dWindowConfig};
use crate::viewport3d::lowering::{lower_dynamic, ViewportInputs};
use crate::viewport3d::measure::MeasureState;
use crate::viewport3d::parts::BasicTessellator;
use crate::viewport3d::picking::granularity;
use crate::viewport3d::playback::PlaybackState;
use crate::viewport3d::window::*;
use semio_s_plugin_norm_structural_scene::*;

fn copy(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn dimensioned(profile: ProfileKind, params: &[(&str, f64)]) -> SectionShape {
    SectionShape::parametric(profile, params)
}

fn heb240() -> SectionShape {
    dimensioned(ProfileKind::IShape, &[("h", 0.24), ("b", 0.24), ("tw", 0.01), ("tf", 0.017)])
}

fn entity(kind: EntityKind, id: &str, geometry: Geometry, role: MaterialRole, util: Option<(f64, CheckStatus)>, layer: &str) -> SceneEntity {
    let mut appearance = Appearance::role(role);
    if util.is_some() { appearance.channel = ColourChannel::Utilisation; }
    SceneEntity {
        entity: EntityRef::new(kind, id, format!("{}[id={id}]", granularity(kind))),
        geometry,
        appearance,
        utilisation: util.map(|(ratio, status)| UtilisationBinding { check_id: format!("check-{id}"), ratio, status, per_station: None, stale: false }),
        label: None,
        accessible_name: copy(&format!("Entity {id}"), &format!("Objekt {id}")),
        accessible_detail: copy("detail", "Detail"),
        layer: layer.into(),
        pick: PickPolicy::Whole,
        handles: Vec::new(),
        content_hash: String::new(),
    }.sealed()
}

fn beam(id: &str, to_x: f64, util: Option<(f64, CheckStatus)>) -> SceneEntity {
    entity(EntityKind::Member, id, Geometry::Member(MemberGeometry {
        axis: Path3::Polyline(Polyline3 { points: vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(to_x, 0.0, 0.0)] }),
        frame: Frame::WORLD,
        section: heb240(),
        section_taper: None,
        start_offset: Vec2::new(0.0, 0.0),
        end_offset: Vec2::new(0.0, 0.0),
        start_release: EndRelease::Rigid,
        end_release: EndRelease::Rigid,
        stations: Vec::new(),
    }), MaterialRole::Steel, util, "members")
}

fn layers() -> Vec<Layer> {
    ["members", "fasteners", "loads", "supports"].iter().map(|id| Layer { id: (*id).into(), name: copy(id, id), visible: true, locked: false }).collect()
}

fn scene_of(entities: Vec<SceneEntity>) -> StructuralScene {
    StructuralScene {
        schema: SCENE_SCHEMA_ID.into(),
        revision: SceneRevision { document_seq: 1, report_seq: 1 },
        bounds: Aabb::new(Vec3::new(0.0, -1.0, -1.0), Vec3::new(4.0, 1.0, 1.0)),
        entities,
        annotations: Vec::new(),
        legends: Vec::new(),
        layers: layers(),
        tracks: Vec::new(),
        owners: Vec::new(),
        description: copy("A steel frame with two beams.", "Ein Stahlrahmen mit zwei Trägern."),
    }
}

fn key(at: f64, field: Option<&str>, layer: Option<&str>) -> Keyframe {
    Keyframe { at, caption: copy(&format!("k{at}"), &format!("s{at}")), field: field.map(Into::into), layer: layer.map(Into::into) }
}

fn track(id: &str, kind: TrackKind, max: f64, step: f64, keyframes: Vec<Keyframe>) -> ParameterTrack {
    ParameterTrack { id: id.into(), kind, label: copy("Fire duration", "Branddauer"), unit: QuantityKind::Time, min: 0.0, max, step, looped: false, keyframes }
}

fn inputs<'a>(config: &'a Viewport3dWindowConfig, selected: &'a [String], measure: &'a MeasureState) -> ViewportInputs<'a> {
    ViewportInputs { config, selected, hovered: None, measure, lang: Lang::En }
}

fn fire_track() -> ParameterTrack {
    track("fire", TrackKind::Time, 3600.0, 60.0, vec![key(0.0, Some("temp@0"), None), key(1800.0, Some("temp@30"), None), key(3600.0, Some("temp@60"), None)])
}

fn field_beam(id: &str, fields: Vec<ScalarField>, active: &str) -> SceneEntity {
    let mut e = beam(id, 4.0, None);
    e.appearance.channel = ColourChannel::Fields { active: active.into(), fields };
    e.sealed()
}

fn vertex_field(id: &str, unit: QuantityKind, values: Vec<f64>) -> ScalarField {
    ScalarField { id: id.into(), unit, samples: FieldSamples::Vertex { values } }
}

#[test]
fn window_definitions_bind_surface_domain_and_actions() {
    use crate::{viewport_window_definition, diagram_window_definition, visual_window_id};
    use semio_plugin::{SurfaceKind, WindowKindDefinition};
    use semio_ui_locale::LocalizedLabel;
    let viewport: WindowKindDefinition = viewport_window_definition("en1993", LocalizedLabel::native("Model", "Modell"), "body-viewport", "box");
    assert_eq!(viewport.id, visual_window_id("en1993", "viewport"));
    assert_eq!(viewport.id, "norm-en1993-viewport");
    assert_eq!(viewport.surface_kind, SurfaceKind::World3d);
    assert_eq!(viewport.interactions.iter().map(|i| i.as_str()).collect::<Vec<_>>(), vec!["norm.en1993"]);
    assert_eq!(viewport.utilities.iter().map(|u| u.as_str()).collect::<Vec<_>>(), vec!["transform"]);
    let ids: Vec<&str> = viewport.actions.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(&ids[..13], &[VIEWPORT_ACTION_SET_CAMERA, VIEWPORT_ACTION_FRAME, VIEWPORT_ACTION_TOGGLE_LAYER, VIEWPORT_ACTION_SET_COLOUR_MAP, VIEWPORT_ACTION_SET_SECTION, VIEWPORT_ACTION_SET_DEFORMATION_SCALE, VIEWPORT_ACTION_SET_LOAD_CASE, playback::ACTION_SET_ACTIVE_FIELD, playback::ACTION_SELECT_TRACK, playback::ACTION_SEEK, playback::ACTION_TOGGLE_PLAY, playback::ACTION_STEP, playback::ACTION_SET_SPEED]);
    for touch in [crate::TOUCH_MENU_OPEN, crate::TOUCH_MENU_CLOSE, crate::TOUCH_FRAME_SELECTION] {
        assert!(ids.contains(&touch), "the viewport window declares the touch action {touch}, so the shell may raise it");
    }
    let diagrams = diagram_window_definition("en1993", LocalizedLabel::native("Diagrams", "Diagramme"), "body-diagrams", "chart-line");
    assert_eq!((diagrams.id.as_str(), diagrams.surface_kind), ("norm-en1993-diagrams", SurfaceKind::Canvas2d));
    assert_eq!(diagrams.interactions.len(), 1);
}

#[test]
fn layouts_are_desktop_first_then_mobile_then_tablet() {
    use crate::visual_layout;
    use semio_plugin::{WindowLayoutChild, WindowLayoutRoot};
    let WindowLayoutRoot::Axis(desktop) = visual_layout("en1993", DeviceClass::Desktop).root else { panic!("row") };
    assert_eq!((desktop.kind.as_str(), desktop.children.len()), ("row", 3));
    assert!(matches!(&desktop.children[1], WindowLayoutChild::Stack(s) if s.children[0].window_kind_id == "norm-en1993-viewport"));
    let WindowLayoutRoot::Stack(mobile) = visual_layout("en1993", DeviceClass::Mobile).root else { panic!("stack") };
    assert_eq!(mobile.children.len(), 5);
    assert_eq!(mobile.active_window_kind_id.as_deref(), Some("norm-en1993-viewport"));
    let WindowLayoutRoot::Axis(tablet) = visual_layout("en1993", DeviceClass::Tablet).root else { panic!("row") };
    assert!(matches!(&tablet.children[1], WindowLayoutChild::Stack(s) if s.children.len() == 4));
}

#[test]
fn interaction_domain_is_generated_from_entity_kinds() {
    use crate::{interaction_domain_id, viewport_interaction_definition};
    use semio_ui_locale::LocalizedLabel;
    let def = viewport_interaction_definition("en1993", LocalizedLabel::native("Structure", "Tragwerk"), &[EntityKind::Member, EntityKind::Fastener]);
    assert_eq!(def.id, interaction_domain_id("en1993"));
    assert_eq!(def.granularities.iter().map(|g| g.id.as_str()).collect::<Vec<_>>(), vec!["member", "fastener", "check", "loadCase"]);
    assert_eq!(def.hover.channels, vec!["pointer".to_string(), "list".to_string()]);
}

#[test]
fn the_toolbar_lists_fields_tracks_and_the_readout_in_both_languages() {
    let mut scene = scene_of(vec![field_beam("b1", vec![vertex_field("temp@0", QuantityKind::Temperature, vec![20.0, 20.0])], "temp@0")]);
    scene.tracks = vec![fire_track()];
    let view = ViewportView { playback: PlaybackState { track: Some("fire".into()), value: 1800.0, ..PlaybackState::default() }, ..ViewportView::default() };
    let text = |locale| format!("{:?}", render_toolbar(&scene, &view, locale, "s.norm.test@1/*#editor").expect("toolbar").expect("some"));
    assert!(text(semio_ui_locale::Locale::En).contains("Fire duration: 30 min"));
    assert!(text(semio_ui_locale::Locale::De).contains("Branddauer: 30 min"));
    assert!(render_toolbar(&scene_of(vec![]), &ViewportView::default(), semio_ui_locale::Locale::En, "s.norm.test@1/*#editor").expect("toolbar").is_none());
}

#[test]
fn a_viewport_action_changes_the_rendered_state() {
    use crate::{view_event, NormTransient, VIEW_ACTION_FIELD, VIEW_ACTION_SEEK, VIEW_ACTION_TOGGLE, VIEW_ACTION_TRACK};
    let mut scene = scene_of(vec![field_beam("b1", vec![vertex_field("temp@0", QuantityKind::Temperature, vec![20.0, 20.0]), vertex_field("stress", QuantityKind::Stress, vec![1.0, 2.0])], "temp@0")]);
    scene.tracks = vec![fire_track()];
    let toolbar = |view: &ViewportView| format!("{:?}", render_toolbar(&scene, &view.resolved(&scene), semio_ui_locale::Locale::En, "s.norm.test@1/*#editor").expect("toolbar").expect("some"));
    let fold = |events: &[(&str, Option<&str>, Option<f64>)]| events.iter().filter_map(|(a, t, n)| view_event(a, *t, *n)).fold(NormTransient::default(), NormTransient::apply).view;
    let rest = ViewportView::default();
    assert!(!toolbar(&rest).contains("Fire duration: "));
    let at_30 = ViewportView { lane: fold(&[(VIEW_ACTION_TRACK, Some("fire"), None), (VIEW_ACTION_SEEK, None, Some(1800.0))]), ..ViewportView::default() };
    let resolved = at_30.resolved(&scene);
    assert_eq!((resolved.playback.track.as_deref(), resolved.playback.value), (Some("fire"), 1800.0));
    assert!(toolbar(&at_30).contains("Fire duration: 30 min"));
    let at_45 = ViewportView { lane: fold(&[(VIEW_ACTION_TRACK, Some("fire"), None), (VIEW_ACTION_SEEK, None, Some(2700.0))]), ..ViewportView::default() };
    assert!(toolbar(&at_45).contains("Fire duration: 45 min") && !toolbar(&at_45).contains("30 min"));
    let playing = ViewportView { lane: fold(&[(VIEW_ACTION_TRACK, Some("fire"), None), (VIEW_ACTION_TOGGLE, None, None)]), ..ViewportView::default() };
    assert!(playing.resolved(&scene).playback.playing && !resolved.playback.playing);
    let stressed = ViewportView { lane: fold(&[(VIEW_ACTION_FIELD, Some("stress"), None)]), ..ViewportView::default() };
    assert_eq!(stressed.resolved(&scene).config.active_field.as_deref(), Some("stress"));
    let lit = |view: &ViewportView| {
        let measure = MeasureState::default();
        let view = view.resolved(&scene);
        lower_dynamic(&scene, &inputs(&view.config, &[], &measure), &view.playback, &BasicTessellator::default()).scalar_field.map(|f| format!("{f:?}"))
    };
    assert_ne!(lit(&stressed), lit(&rest));
}

fn panel_json(controller: &'static str, lanes: crate::lane_overlays::LaneOverlays) -> String {
    let view = crate::toolbar::ViewportView { lanes, ..Default::default() };
    let node = crate::toolbar::render_viewport_panel("norm.x.viewport", &scene_of(Vec::new()), &view, &[], "norm.x", controller, semio_ui_locale::Locale::En).expect("panel");
    semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).expect("project the panel")
}

#[test]
fn the_surface_of_a_live_editor_binds_the_drag_phases_and_an_armed_insert_tool_binds_hover_and_drop() {
    use crate::{GhostPreview, GhostSource, LaneFrame, NormTransient};
    let palette = vec![PaletteEntry { id: "member".into(), label_en: "Member".into(), label_de: "Bauteil".into() }];
    let idle = panel_json("s.norm.en1993@1/*#editor", LaneOverlays { palette: palette.clone(), ..Default::default() });
    for needle in ["setFields", "stream", "commit", "abort", "previewRemedy", "clearPreview", "Insert Member"] {
        assert!(idle.contains(needle), "{needle}: {idle}");
    }
    assert!(!idle.contains("placeSpec"), "no tool armed, no placement binding");
    let ghost = GhostPreview { writes: Vec::new(), source: GhostSource::Insert { spec: "member".into(), kind: EntityKind::Member, at: Vec3::new(0.0, 0.0, 0.0) } };
    let armed = panel_json("s.norm.en1993@1/*#editor", LaneOverlays { palette, frame: LaneFrame { transient: NormTransient::default().with_ghost(Some(ghost)), peers: Vec::new() }, ..Default::default() });
    assert!(armed.contains("placeSpec") && armed.contains("insertSpec"), "{armed}");
    let viewer = panel_json("s.norm.en1993@1/*#viewer", LaneOverlays::default());
    assert!(!viewer.contains("setFields"), "{viewer}");
}

#[test]
fn the_viewport_instance_label_is_the_screen_reader_name_of_the_entity() {
    let scene = scene_of(vec![beam("B1", 4.0, Some((0.64, CheckStatus::Pass))), beam("B2", 3.0, None)]);
    let config = Viewport3dWindowConfig::default();
    let measure = MeasureState::default();
    for lang in [Lang::En, Lang::De] {
        let inputs = ViewportInputs { config: &config, selected: &[], hovered: None, measure: &measure, lang };
        let lowered = lower_dynamic(&scene, &inputs, &PlaybackState::default(), &BasicTessellator::default());
        let labels: Vec<&str> = lowered.instances.iter().filter_map(|record| record["label"].as_str()).collect();
        for (index, entity) in scene.entities.iter().enumerate() {
            let want = crate::accessibility::screen_reader::entity_node(entity, lang, index + 1, scene.entities.len(), false).name;
            assert!(labels.contains(&want.as_str()), "{want} in {labels:?}");
        }
    }
    let spoken = crate::viewport3d::description::entity_spoken_name(&scene.entities[0], Lang::De);
    assert_eq!(spoken, "Objekt B1, Ausnutzung 64 %, besteht");
}

#[test]
fn the_scene_reads_as_a_list_with_a_named_option_per_entity_and_projects_to_the_platform_tree() {
    let scene = scene_of(vec![beam("B1", 4.0, Some((0.64, CheckStatus::Pass))), beam("B2", 3.0, None)]);
    let nodes = crate::scene_accessible_nodes(&scene, &["B2".to_owned()], Lang::En);
    assert_eq!(nodes.iter().map(|n| (n.role.as_str(), n.depth)).collect::<Vec<_>>(), [("listbox", 0), ("option", 1), ("option", 1)]);
    assert_eq!((nodes[1].name.as_str(), nodes[1].position, nodes[1].set_size, nodes[1].selected), ("Entity B1, utilisation 64 %, passes", Some(1), Some(2), Some(false)));
    assert_eq!(nodes[2].selected, Some(true));
    let projected = semio_framework::accessible_projection(&nodes, 10);
    assert_eq!(projected.iter().map(|n| (n.node_id, n.role.as_str(), n.label.as_deref())).collect::<Vec<_>>()[1], (11, "option", Some("Entity B1, utilisation 64 %, passes")));
    use semio_framework::PluginShellHooks;
    crate::publish_scene_access(&scene, &[], Lang::De);
    assert_eq!(crate::NormShellHooks.accessible_nodes("norm-viewport", "de").len(), 3);
    assert!(crate::NormShellHooks.accessible_nodes("norm-global", "de").is_empty());
}

#[test]
fn a_lane_check_frames_the_camera_on_the_selected_entities_and_a_free_lane_keeps_the_whole_scene() {
    use crate::viewport3d::camera::{fit_selection, preset_pose};
    use crate::viewport3d::picking::entity_address;
    let scene = scene_of(vec![beam("b1", 4.0, None), beam("b2", 10.0, None)]);
    let selected = vec![entity_address(EntityKind::Member, "b2").expect("address")];
    let free = ViewportView::default();
    assert_eq!(crate::toolbar::lowered_viewport(&scene, &free, &selected, Lang::En).camera, preset_pose(&scene, free.config.preset));
    let stepped = ViewportView { lane: crate::ViewLane { check: Some(1), ..Default::default() }, ..Default::default() };
    let camera = crate::toolbar::lowered_viewport(&scene, &stepped, &selected, Lang::En).camera;
    assert_eq!(camera, fit_selection(&scene, &["b2".to_string()], stepped.config.preset));
    assert_ne!(camera, preset_pose(&scene, stepped.config.preset));
}

#[test]
fn the_keyboard_help_lists_every_shortcut_in_english_and_german() {
    use crate::accessibility::focus_order::{chord_text, shortcuts};
    let help = |locale, mac| format!("{:?}", crate::toolbar::render_shortcut_help(locale, mac).expect("help"));
    let (en, de) = (help(semio_ui_locale::Locale::En, false), help(semio_ui_locale::Locale::De, false));
    for shortcut in shortcuts() {
        assert!(en.contains(&shortcut.label.en) && de.contains(&shortcut.label.de), "{:?}", shortcut.action);
        assert!(en.contains(&chord_text(shortcut.chord, false)));
    }
    for label in ["Next check", "Previous check", "Next governing check", "Previous governing check", "Next combination", "Previous combination"] {
        assert!(en.contains(label), "{label}");
    }
    for label in ["Nächster Nachweis", "Vorheriger Nachweis", "Nächster maßgebender Nachweis", "Vorheriger maßgebender Nachweis", "Nächste Kombination", "Vorherige Kombination"] {
        assert!(de.contains(label), "{label}");
    }
    assert!(help(semio_ui_locale::Locale::En, true).contains("⌥G"));
    let panel = panel_json("s.norm.test@1/*#editor", crate::lane_overlays::LaneOverlays::default());
    assert!(panel.contains("norm-viewport-help"));
}

fn grip(id: &str, rest: f64, limits: (f64, f64), increment: f64) -> Handle {
    Handle {
        id: id.into(),
        entity: EntityRef::new(EntityKind::Member, "b1", "beams[id=b1]"),
        kind: HandleKind::ScalarAlong { axis: Vec3::new(1.0, 0.0, 0.0) },
        anchor: Vec3::new(rest, 0.0, 0.0),
        grip: GripStyle::Arrow,
        writes: vec![FieldBinding::new("beams[id=b1].xM", Affine { base: 0.0, k: 1.0 }, "m")],
        snap: SnapRule { grid: None, increment: Some(increment), targets: vec![], catalogue: vec![], tolerance: 0.1 },
        limits: Some(Interval::new(limits.0, limits.1)),
        accessible_name: copy("Position", "Lage"),
    }
}

const EDITOR_CONTROLLER: &str = "s.norm.en1993@1/*#editor";

fn touch_json(view: &ViewportView, device: Option<DeviceClass>, locale: semio_ui_locale::Locale) -> Option<String> {
    crate::toolbar::render_touch_controls(view, device, "norm.x", locale, EDITOR_CONTROLLER).expect("touch controls").map(|node| semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).expect("project the touch controls"))
}

#[test]
fn the_nudge_buttons_of_a_tap_to_nudge_device_commit_one_schema_step_in_one_set_fields() {
    use semio_ui_locale::Locale;
    let view = ViewportView { lanes: LaneOverlays { handles: vec![grip("b1/x", 1.0, (-5.0, 5.0), 0.5)], ..Default::default() }, ..Default::default() };
    let phone = touch_json(&view, Some(DeviceClass::Mobile), Locale::En).expect("a phone offers nudge buttons");
    for needle in ["norm-viewport.nudge.b1/x.minus", "norm-viewport.nudge.b1/x.plus", "setFields", "commit", "1.5", "0.5", "Position", "Handles"] {
        assert!(phone.contains(needle), "{needle}: {phone}");
    }
    let german = touch_json(&view, Some(DeviceClass::Tablet), Locale::De).expect("a tablet offers nudge buttons");
    assert!(german.contains("Lage") && german.contains("Griffe"), "{german}");
    assert!(touch_json(&view, Some(DeviceClass::Desktop), Locale::En).is_none(), "the desktop keeps the pointer and the keyboard");
    assert!(touch_json(&view, None, Locale::En).is_none(), "before the shell chose a layout there is no device to adapt to");
    let none = ViewportView::default();
    assert!(touch_json(&none, Some(DeviceClass::Mobile), Locale::En).is_none(), "nothing selected, no grips, no buttons");
    let plus = crate::Gesture::parse(r#"{"handle":"b1/x","s":1.5,"origin":1.0,"phase":"commit","ctrl":true}"#).expect("the flat gesture of the plus button");
    assert_eq!(crate::commit_writes(&[grip("b1/x", 1.0, (-5.0, 5.0), 0.5)], &plus), Some(vec![crate::FieldWrite::number("beams[id=b1].xM", 1.5)]), "one setFields moves the field by one step");
    let edge = ViewportView { lanes: LaneOverlays { handles: vec![grip("b1/x", 5.0, (-5.0, 5.0), 0.5)], ..Default::default() }, ..Default::default() };
    for (locale, text) in [(Locale::En, "At the limit"), (Locale::De, "Am Grenzwert")] {
        let at_limit = touch_json(&edge, Some(DeviceClass::Mobile), locale).expect("buttons");
        assert!(at_limit.contains(text) && at_limit.contains("4.5"), "{locale:?}: the plus row says so and the minus row still steps: {at_limit}");
    }
    let focused = ViewportView { lanes: LaneOverlays { handles: vec![grip("b1/x", 1.0, (-5.0, 5.0), 0.5), grip("b1/y", 1.0, (-5.0, 5.0), 0.5)], focused: Some("b1/y".into()), ..Default::default() }, ..Default::default() };
    let narrowed = touch_json(&focused, Some(DeviceClass::Mobile), Locale::En).expect("buttons");
    assert!(narrowed.contains("b1/y.plus") && !narrowed.contains("b1/x.plus"), "the focused grip narrows the list: {narrowed}");
}

#[test]
fn the_edit_menu_is_part_of_the_viewport_panel_while_the_view_lane_holds_it_open_and_only_for_a_live_editor() {
    use semio_ui_locale::Locale;
    let panel = |menu: bool, controller: &'static str, locale: Locale| {
        let view = ViewportView { lane: crate::ViewLane { menu, ..Default::default() }, ..Default::default() };
        let node = render_viewport_panel("norm.menu-law.viewport", &scene_of(Vec::new()), &view, &[], "norm.menu-law", controller, locale).expect("panel");
        semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).expect("project the panel")
    };
    let open = panel(true, EDITOR_CONTROLLER, Locale::En);
    for needle in ["norm-touch-menu", "Select all", "Focus check", "Close menu", "viewportTouchMenu", "touch"] {
        assert!(open.contains(needle), "{needle}: {open}");
    }
    let german = panel(true, EDITOR_CONTROLLER, Locale::De);
    assert!(german.contains("Bearbeiten") && german.contains("Menü schließen") && german.contains("Alle auswählen") && german.contains("Prüfung fokussieren"), "{german}");
    assert!(!panel(false, EDITOR_CONTROLLER, Locale::En).contains("norm-touch-menu"), "a closed menu is not drawn");
    assert!(!panel(true, "s.norm.en1993@1/*#viewer", Locale::En).contains("norm-touch-menu"), "a viewer edits nothing, so it has no Edit menu");
    let nodes = crate::accessibility::published_nodes::published("menu", "norm.menu-law");
    assert_eq!((nodes.len(), nodes[0].node.role), (7, "menu"), "the open menu publishes a menu with its six entries (focus check first) for the screen reader");
    assert_eq!(nodes[1].key, crate::TOUCH_FOCUS_CHECK_ID);
    panel(true, EDITOR_CONTROLLER, Locale::En);
    assert!(crate::touch_menu_open("norm.menu-law"), "the panel recorded that the menu of norm.menu-law is open, for the shell hook that closes it");
    panel(false, EDITOR_CONTROLLER, Locale::En);
    assert!(!crate::touch_menu_open("norm.menu-law"), "a closed menu is recorded as closed");
}
