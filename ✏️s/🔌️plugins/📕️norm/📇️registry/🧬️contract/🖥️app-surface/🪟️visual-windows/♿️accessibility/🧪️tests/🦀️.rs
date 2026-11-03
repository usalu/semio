//! 🧪️ Accessibility tests; scenarios in `🥒️.feature`. Independent oracles: `std` sorting and a manual reachability walk for focus, `serde_json` for the aria contract.

use super::focus_order::*;
use super::live_region::*;
use super::screen_reader::*;
use super::*;
use crate::viewport3d::config::{DeviceClass, Lang};
use semio_s_plugin_norm_structural_scene::{CheckStatus, LocalizedCopy};

fn u(id: &str, ratio: f64, status: CheckStatus) -> Utilisation {
    Utilisation { id: id.into(), name: LocalizedCopy::new(format!("Member {id}"), format!("Stab {id}")), ratio, status }
}

#[test]
fn evaluation_is_announced_in_both_languages_and_fails_are_assertive() {
    let a = announce_evaluation(&[u("B1", 0.64, CheckStatus::Pass), u("B2", 1.12, CheckStatus::Fail), u("B3", 0.95, CheckStatus::Warning)]);
    assert_eq!(a.politeness, Politeness::Assertive);
    assert_eq!(a.text.en, "Evaluation finished: 3 checks, 1 failing, 1 near the limit. Highest utilisation: Member B2 at 112 %, fails.");
    assert_eq!(a.text.de, "Auswertung abgeschlossen: 3 Nachweise, 1 nicht erfüllt, 1 nahe am Grenzwert. Höchste Ausnutzung: Stab B2 mit 112 %, versagt.");
    let ok = announce_evaluation(&[u("B1", 0.5, CheckStatus::Pass)]);
    assert_eq!(ok.politeness, Politeness::Polite);
    assert_eq!(announce_evaluation(&[]).text.en, "Evaluation finished: 0 checks, 0 failing, 0 near the limit.");
    assert_eq!(text(&a, Lang::De), a.text.de);
    assert_eq!(serde_json::json!({"aria-live": a.politeness.aria()})["aria-live"], "assertive");
}

#[test]
fn only_meaningful_utilisation_changes_are_announced() {
    let before = [u("B1", 0.64, CheckStatus::Pass), u("B2", 0.80, CheckStatus::Pass), u("B3", 0.50, CheckStatus::Pass)];
    let after = [u("B1", 0.66, CheckStatus::Pass), u("B2", 1.05, CheckStatus::Fail), u("B3", 0.56, CheckStatus::Pass), u("B4", 0.2, CheckStatus::Pass)];
    let got = announce_changes(&before, &after);
    assert_eq!(got.iter().map(|a| a.key.as_str()).collect::<Vec<_>>(), ["utilisation:B2", "utilisation:B3", "utilisation:B4"]);
    assert_eq!(got[0].politeness, Politeness::Assertive);
    assert_eq!(got[0].text.en, "Member B2 utilisation from 80 % to 105 %, fails");
    assert_eq!(got[0].text.de, "Stab B2 Ausnutzung von 80 % auf 105 %, versagt");
    assert_eq!(got[1].politeness, Politeness::Polite);
    assert_eq!(got[2].text.en, "Member B4 utilisation to 20 %, passes");
    assert!(announce_changes(&after, &after).is_empty());
}

#[test]
fn live_region_coalesces_orders_and_caps() {
    let mut region = LiveRegion::default();
    region.extend(announce_changes(&[], &[u("B1", 0.1, CheckStatus::Pass), u("B2", 0.2, CheckStatus::Pass), u("B3", 1.3, CheckStatus::Fail), u("B4", 0.4, CheckStatus::Pass)]));
    region.push(announce_changes(&[], &[u("B1", 0.3, CheckStatus::Pass)]).remove(0));
    assert_eq!(region.len(), 4);
    let spoken = region.drain(2);
    assert_eq!(spoken.len(), 3);
    assert_eq!(spoken[0].key, "utilisation:B3");
    assert_eq!(spoken[2].key, "overflow");
    assert_eq!(spoken[2].text.de, "2 weitere Änderungen, siehe Nachweisliste");
    assert!(region.is_empty());
    let mut sorted: Vec<_> = spoken[..2].iter().map(|a| a.politeness).collect();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(sorted, spoken[..2].iter().map(|a| a.politeness).collect::<Vec<_>>());
}

#[test]
fn focus_order_runs_outliner_viewport_properties_checks_diagrams() {
    assert_eq!(focus_order(DeviceClass::Desktop).iter().map(|r| r.role()).collect::<Vec<_>>(), ["outliner", "viewport", "properties", "checks", "diagrams"]);
    assert_eq!(focus_order(DeviceClass::Tablet), focus_order(DeviceClass::Mobile));
    assert_eq!(focus_order(DeviceClass::Tablet).len(), 5);
    assert_eq!(focus_order(DeviceClass::Mobile)[0], Region::Viewport);
}

#[test]
fn focus_is_never_trapped() {
    for device in [DeviceClass::Desktop, DeviceClass::Mobile, DeviceClass::Tablet] {
        let order = focus_order(device);
        for forward in [true, false] {
            let mut at = if forward { order[0] } else { order[order.len() - 1] };
            let mut steps = 0;
            loop {
                match next(device, at, forward) {
                    FocusMove::To(r) => at = r,
                    FocusMove::Leave { forward: f } => { assert_eq!(f, forward); break; }
                }
                steps += 1;
                assert!(steps <= order.len(), "trapped on {device:?}");
            }
            assert_eq!(steps, order.len() - 1);
        }
    }
    assert_eq!(next(DeviceClass::Mobile, Region::Outliner, true), FocusMove::To(Region::Properties), "the outliner is a tab of the touch layouts");
}

#[test]
fn shortcuts_are_unique_per_scope_bilingual_and_cover_every_region() {
    let all = shortcuts();
    for (i, a) in all.iter().enumerate() {
        assert!(!a.label.en.is_empty() && !a.label.de.is_empty() && a.label.en != a.label.de, "{:?}", a.chord);
        for b in all[i + 1..].iter().filter(|b| b.chord == a.chord) {
            assert!(a.scope != b.scope && a.scope != Scope::Global && b.scope != Scope::Global, "duplicate {:?}", a.chord);
        }
        assert!(!(a.chord.ctrl && matches!(a.chord.key, "w" | "t" | "n" | "Tab")), "browser chord {:?}", a.chord);
        assert!(a.chord.key != "Tab" || a.scope == Scope::Handle, "Tab is only the handle walk");
    }
    for r in DESKTOP_ORDER {
        assert!(all.iter().any(|s| s.action == ShortcutAction::Focus(r)), "{r:?}");
        assert!(!r.name().en.is_empty() && !r.name().de.is_empty());
    }
    let plain = |key| Chord { key, ctrl: false, alt: false, shift: false };
    assert_eq!(lookup(Scope::Viewport, plain("Escape")), Some(ShortcutAction::ClearSelection));
    assert_eq!(lookup(Scope::Handle, plain("Escape")), Some(ShortcutAction::CancelHandle));
    assert_eq!(lookup(Scope::Handle, plain("Enter")), Some(ShortcutAction::ConfirmHandle));
    assert_eq!(lookup(Scope::Viewport, Chord { key: "q", ctrl: true, alt: true, shift: true }), None);
    for key in ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown"] {
        assert!(all.iter().any(|s| s.scope == Scope::Viewport && s.chord == plain(key)), "orbit {key}");
        assert!(all.iter().any(|s| s.scope == Scope::Handle && s.chord == plain(key)), "nudge {key}");
    }
}

#[test]
fn handles_nudge_with_arrows_and_modifiers_for_the_step_size() {
    let chord = |key, alt, shift| Chord { key, ctrl: false, alt, shift };
    assert_eq!(lookup(Scope::Handle, chord("ArrowRight", false, false)), Some(ShortcutAction::NudgeHandle { dx: 1, dy: 0, step: NudgeStep::Normal }));
    assert_eq!(lookup(Scope::Handle, chord("ArrowLeft", false, true)), Some(ShortcutAction::NudgeHandle { dx: -1, dy: 0, step: NudgeStep::Large }));
    assert_eq!(lookup(Scope::Handle, chord("ArrowUp", true, false)), Some(ShortcutAction::NudgeHandle { dx: 0, dy: 1, step: NudgeStep::Fine }));
    assert_eq!(lookup(Scope::Handle, chord("ArrowDown", false, false)), Some(ShortcutAction::NudgeHandle { dx: 0, dy: -1, step: NudgeStep::Normal }));
    assert_eq!(shortcuts().iter().filter(|s| matches!(s.action, ShortcutAction::NudgeHandle { .. })).count(), 12);
    assert_eq!((NudgeStep::Fine.scale(), NudgeStep::Normal.scale(), NudgeStep::Large.scale()), (0.1, 1.0, 10.0));
    let de = shortcuts().into_iter().find(|s| s.action == ShortcutAction::NudgeHandle { dx: 1, dy: 0, step: NudgeStep::Large }).expect("large right").label.de;
    assert_eq!(de, "Griff um einen großen Schritt nach rechts schieben");
}

#[test]
fn ctrl_arrow_cycles_the_handles_of_one_entity_while_plain_arrows_stay_nudges_and_the_help_lists_it() {
    let ctrl = |key| Chord { key, ctrl: true, alt: false, shift: false };
    for (key, forward) in [("ArrowRight", true), ("ArrowDown", true), ("ArrowLeft", false), ("ArrowUp", false)] {
        assert_eq!(lookup(Scope::Handle, ctrl(key)), Some(ShortcutAction::CycleGripInEntity { forward }), "{key}");
        assert_eq!(lookup(Scope::Viewport, ctrl(key)), None, "the cycle only exists once a handle has the keyboard");
    }
    let plain = |key| Chord { key, ctrl: false, alt: false, shift: false };
    assert!(matches!(lookup(Scope::Handle, plain("ArrowRight")), Some(ShortcutAction::NudgeHandle { dx: 1, dy: 0, step: NudgeStep::Normal })));
    let help: Vec<_> = shortcuts().into_iter().filter(|s| matches!(s.action, ShortcutAction::CycleGripInEntity { .. })).collect();
    assert_eq!(help.len(), 4);
    assert!(help.iter().all(|s| s.label.en.contains("handle of the same entity") && s.label.de.contains("Griff desselben Bauteils")));
    assert_eq!(chord_text(ctrl("ArrowRight"), false), "Ctrl+ArrowRight");
    assert_eq!(chord_text(ctrl("ArrowRight"), true), "⌘ArrowRight");
    assert_eq!(shortcut_action_id(&ShortcutAction::CycleGripInEntity { forward: true }), "norm.handle.cycle-grip.forward");
    assert_eq!(shortcut_action_id(&ShortcutAction::CycleGripInEntity { forward: false }), "norm.handle.cycle-grip.backward");
}

#[test]
fn undo_redo_catalogue_search_and_selection_cycling_are_keyboard_reachable_everywhere() {
    let ctrl = |key, shift| Chord { key, ctrl: true, alt: false, shift };
    for scope in [Scope::Viewport, Scope::Handle, Scope::Global] {
        assert_eq!(lookup(scope, ctrl("z", false)), Some(ShortcutAction::Undo));
        assert_eq!(lookup(scope, ctrl("z", true)), Some(ShortcutAction::Redo));
        assert_eq!(lookup(scope, ctrl("y", false)), Some(ShortcutAction::Redo));
        assert_eq!(lookup(scope, ctrl("k", false)), Some(ShortcutAction::SearchCatalogue));
        assert_eq!(lookup(scope, Chord { key: "/", ctrl: false, alt: false, shift: false }), Some(ShortcutAction::SearchCatalogue));
    }
    let page = |key| Chord { key, ctrl: false, alt: false, shift: false };
    assert_eq!(lookup(Scope::Viewport, page("PageDown")), Some(ShortcutAction::CycleSelection { forward: true }));
    assert_eq!(lookup(Scope::Viewport, page("PageUp")), Some(ShortcutAction::CycleSelection { forward: false }));
    assert_eq!(lookup(Scope::Handle, page("Tab")), Some(ShortcutAction::CycleHandle { forward: true }));
    let ids: Vec<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
    assert_eq!(cycle_selection(&ids, Some("c"), true).as_deref(), Some("a"));
    assert_eq!(cycle_selection(&ids, Some("a"), false).as_deref(), Some("c"));
    assert_eq!(cycle_selection(&ids, Some("b"), true).as_deref(), Some("c"));
    assert_eq!(cycle_selection(&ids, None, true).as_deref(), Some("a"));
    assert_eq!(cycle_selection(&ids, Some("zz"), false).as_deref(), Some("c"));
    assert_eq!(cycle_selection(&[], None, true), None);
}

#[test]
fn chords_print_for_the_help_overlay() {
    let redo = Chord { key: "z", ctrl: true, alt: false, shift: true };
    assert_eq!(chord_text(redo, false), "Ctrl+Shift+Z");
    assert_eq!(chord_text(redo, true), "⌘⇧Z");
    assert_eq!(chord_text(Chord { key: "ArrowLeft", ctrl: false, alt: true, shift: false }, false), "Alt+ArrowLeft");
}

#[test]
fn preferences_resolve_to_a_theme_variant() {
    use semio_s_plugin_norm_structural_scene::colour_maps::Variant;
    let p = A11yPrefs::default();
    assert_eq!(resolve_variant(p), Variant::Light);
    assert_eq!(resolve_variant(A11yPrefs { prefers_dark: true, ..p }), Variant::Dark);
    assert_eq!(resolve_variant(A11yPrefs { prefers_dark: true, forced_colors: true, ..p }), Variant::HighContrast);
    assert_eq!(resolve_variant(A11yPrefs { prefers_contrast_more: true, ..p }), Variant::HighContrast);
    assert!(animations_allowed(p) && !animations_allowed(A11yPrefs { prefers_reduced_motion: true, ..p }));
}

mod screen_reader_fixtures {
    use semio_s_plugin_norm_structural_scene::*;

    pub fn entity(id: &str, util: Option<(f64, CheckStatus)>) -> SceneEntity {
        SceneEntity {
            entity: EntityRef::new(EntityKind::Group, id, format!("groups[id={id}]")),
            geometry: Geometry::Group { members: Vec::new() },
            appearance: Appearance::role(MaterialRole::Steel),
            utilisation: util.map(|(ratio, status)| UtilisationBinding { check_id: format!("check-{id}"), ratio, status, per_station: None, stale: false }),
            label: None,
            accessible_name: LocalizedCopy::new(format!("Entity {id}"), format!("Objekt {id}")),
            accessible_detail: LocalizedCopy::new("Steel beam, 6 m", "Stahlträger, 6 m"),
            layer: "groups".into(),
            pick: PickPolicy::Whole,
            handles: Vec::new(),
            content_hash: String::new(),
        }
        .sealed()
    }

    pub fn handle(limits: Option<Interval>) -> Handle {
        Handle {
            id: "h1".into(),
            entity: EntityRef::new(EntityKind::Group, "B1", "groups[id=B1]"),
            kind: HandleKind::DimensionLine,
            anchor: Vec3::new(0.0, 0.0, 0.0),
            grip: GripStyle::Arrow,
            writes: vec![FieldBinding::new("groups[id=B1].lengthM", Affine::IDENTITY, "m")],
            snap: SnapRule::NONE,
            limits,
            accessible_name: LocalizedCopy::new("Beam length", "Trägerlänge"),
        }
    }

    pub fn diagram() -> Diagram2d {
        let axis = |label: &str| Axis { label: LocalizedCopy::new(label, label), quantity: QuantityKind::Moment, unit: "kNm".into(), scale: AxisScale::Linear, domain: Some(Interval::new(0.0, 400.0)), ticks: TickRule::Auto };
        Diagram2d {
            id: "mn".into(),
            title: LocalizedCopy::new("Interaction", "Interaktion"),
            description: LocalizedCopy::new("M-N interaction", "M-N-Interaktion"),
            axes: vec![axis("Moment"), axis("Normal force")],
            series: Vec::new(),
            markers: vec![Marker { id: "load".into(), at: Vec2::new(100.0, 200.0), shape: MarkerShape::Diamond, tone: Tone::Error, label: LocalizedCopy::new("Load", "Last"), utilisation: Some(0.62), bound_check: Some("c1".into()), binding: None }],
            annotations: Vec::new(),
            focus: None,
            legend: true,
            table_twin: DataTable { columns: vec![DataColumn { header: LocalizedCopy::new("M", "M"), unit: "kNm".into(), values: vec![0.0, 200.0, 300.0] }, DataColumn { header: LocalizedCopy::new("N", "N"), unit: "kN".into(), values: vec![0.0, 150.0] }] },
        }
    }
}

fn attr(node: &AriaNode, key: &str) -> Option<String> {
    node.attributes().into_iter().find(|(k, _)| *k == key).map(|(_, v)| v)
}

#[test]
fn viewport_entities_are_options_that_speak_utilisation_and_status() {
    use screen_reader_fixtures::entity;
    let b1 = entity("B1", Some((0.64, CheckStatus::Pass)));
    let en = entity_node(&b1, Lang::En, 2, 5, true);
    let de = entity_node(&b1, Lang::De, 2, 5, false);
    assert_eq!((en.role, en.name.as_str()), ("option", "Entity B1, utilisation 64 %, passes"));
    assert_eq!(de.name, "Objekt B1, Ausnutzung 64 %, besteht");
    assert_eq!((attr(&en, "aria-posinset").as_deref(), attr(&en, "aria-setsize").as_deref(), attr(&en, "aria-selected").as_deref()), (Some("2"), Some("5"), Some("true")));
    assert_eq!(attr(&de, "aria-selected").as_deref(), Some("false"));
    assert_eq!(attr(&de, "aria-description").as_deref(), Some("Stahlträger, 6 m"));
    assert!(en.missing_required().is_empty());
    assert_eq!(entity_node(&entity("B2", Some((1.12, CheckStatus::Fail))), Lang::De, 1, 1, false).name, "Objekt B2, Ausnutzung 112 %, versagt");
    assert_eq!(entity_node(&entity("S1", None), Lang::En, 1, 1, false).name, "Entity S1");
    assert_eq!(scene_list_node(Lang::De).name, "Szene als Liste");
    let view = viewport_node("12 objects shown.", Lang::En);
    assert_eq!((view.role, attr(&view, "aria-roledescription").as_deref(), attr(&view, "aria-description").as_deref()), ("application", Some("3D model"), Some("12 objects shown.")));
}

#[test]
fn handles_are_sliders_with_value_text_limits_and_keyboard_help() {
    use screen_reader_fixtures::handle;
    use semio_s_plugin_norm_structural_scene::Interval;
    let h = handle(Some(Interval::new(1.0, 12.0)));
    let en = handle_node(&h, Some(6.5), None, Lang::En);
    let de = handle_node(&h, Some(6.5), None, Lang::De);
    assert_eq!((en.role, en.name.as_str(), attr(&en, "aria-valuetext").as_deref()), ("slider", "Beam length", Some("6.5 m")));
    assert_eq!((de.name.as_str(), attr(&de, "aria-valuetext").as_deref(), attr(&de, "aria-roledescription").as_deref()), ("Trägerlänge", Some("6,5 m"), Some("Griff")));
    assert_eq!((attr(&en, "aria-valuenow").as_deref(), attr(&en, "aria-valuemin").as_deref(), attr(&en, "aria-valuemax").as_deref()), (Some("6.5"), Some("1"), Some("12")));
    assert!(attr(&en, "aria-description").expect("help").contains("Shift") && attr(&de, "aria-description").expect("help").contains("Umschalt"));
    assert!(en.missing_required().is_empty());
    let unlimited = handle_node(&handle(None), Some(2.0), None, Lang::En);
    assert_eq!((attr(&unlimited, "aria-valuemin"), attr(&unlimited, "aria-valuemax")), (None, None));
    assert_eq!(handle_node(&h, None, None, Lang::En).missing_required(), ["aria-valuenow"]);
}

#[test]
fn a_handle_slider_speaks_the_field_value_range_and_the_live_utilisation_of_its_bound_check() {
    use screen_reader_fixtures::handle;
    use semio_s_plugin_norm_structural_scene::{Affine, Interval};
    let mut h = handle(Some(Interval::new(0.0, 10.0)));
    h.writes[0].map = Affine { base: 0.0, k: 100.0 };
    h.writes[0].unit = "cm".into();
    let en = handle_node(&h, Some(0.5), Some((0.83, CheckStatus::Pass)), Lang::En);
    let de = handle_node(&h, Some(0.5), Some((1.12, CheckStatus::Fail)), Lang::De);
    assert_eq!(attr(&en, "aria-valuetext").as_deref(), Some("50 cm, utilisation 83 %, passes"));
    assert!(attr(&de, "aria-valuetext").expect("text").starts_with("50 cm, Ausnutzung 112 %"), "{de:?}");
    assert_eq!((attr(&en, "aria-valuenow").as_deref(), attr(&en, "aria-valuemin").as_deref(), attr(&en, "aria-valuemax").as_deref()), (Some("50"), Some("0"), Some("1000")), "value and range are in the unit of the field, like the text");
    assert!(attr(&en, "aria-description").expect("help").contains("Enter") && attr(&de, "aria-description").expect("help").contains("Eingabe"));
}

#[test]
fn check_rows_speak_status_and_utilisation() {
    let row = |ratio: f64, status| CheckRowFacts { title: LocalizedCopy::new("Bending", "Biegung"), subject: LocalizedCopy::new("Beam B2", "Stab B2"), status, utilisation: ratio, quantities: String::new(), clause: "EN 1993-1-1 6.2.5".into() };
    let fail = check_row_node(&row(1.12, CheckStatus::Fail), Lang::En, 3, 8);
    assert_eq!((fail.role, fail.name.as_str()), ("row", "Bending, Beam B2, utilisation 112 %, fails"));
    assert_eq!(check_row_node(&row(0.95, CheckStatus::Warning), Lang::De, 1, 2).name, "Biegung, Stab B2, Ausnutzung 95 %, nahe der Grenze");
    assert_eq!(check_row_node(&row(0.0, CheckStatus::NotApplicable), Lang::En, 1, 1).name, "Bending, Beam B2, not applicable");
    assert_eq!((attr(&fail, "aria-posinset").as_deref(), attr(&fail, "aria-setsize").as_deref(), attr(&fail, "aria-description").as_deref()), (Some("3"), Some("8"), Some("EN 1993-1-1 6.2.5")));
}

#[test]
fn check_rows_append_their_quantities_to_the_name() {
    let facts = CheckRowFacts { title: LocalizedCopy::new("Bending", "Biegung"), subject: LocalizedCopy::new("Beam B2", "Stab B2"), status: CheckStatus::Pass, utilisation: 0.5, quantities: "1,2 kNm of 2,4 kNm limit".into(), clause: "EN 1993-1-1 6.2.5".into() };
    assert_eq!(check_row_node(&facts, Lang::De, 1, 1).name, "Biegung, Stab B2, Ausnutzung 50 %, besteht, 1,2 kNm of 2,4 kNm limit");
}

#[test]
fn the_viewport_label_and_the_screen_reader_option_share_one_entity_name() {
    use screen_reader_fixtures::*;
    for lang in [Lang::En, Lang::De] {
        for utilisation in [None, Some((0.64, CheckStatus::Pass)), Some((1.2, CheckStatus::Fail)), Some((f64::NAN, CheckStatus::Warning))] {
            let e = entity("B1", utilisation);
            assert_eq!(entity_node(&e, lang, 1, 1, false).name, crate::viewport3d::description::entity_spoken_name(&e, lang));
        }
    }
}

#[test]
fn diagrams_are_images_described_by_their_table_twin() {
    let (image, table) = screen_reader::diagram_nodes(&screen_reader_fixtures::diagram(), Lang::En, "mn-table");
    assert_eq!((image.role, image.name.as_str(), attr(&image, "aria-describedby").as_deref()), ("img", "Interaction", Some("mn-table")));
    assert!(attr(&image, "aria-description").expect("description").contains("Horizontal axis"));
    assert_eq!((table.role, table.name.as_str(), attr(&table, "aria-description").as_deref()), ("table", "Interaction", Some("4 rows, 2 columns")));
    let (image_de, table_de) = screen_reader::diagram_nodes(&screen_reader_fixtures::diagram(), Lang::De, "mn-table");
    assert_eq!(image_de.name, "Interaktion");
    assert_eq!(attr(&table_de, "aria-description").as_deref(), Some("4 Zeilen, 2 Spalten"));
    assert!(attr(&image_de, "aria-description").expect("description").contains("Horizontale Achse"));
}

#[test]
fn every_emitted_role_is_a_wai_aria_role_and_meets_its_required_attributes() {
    const WAI_ARIA_1_2: [&str; 14] = ["application", "listbox", "option", "slider", "row", "img", "table", "button", "status", "alert", "tab", "tablist", "menu", "menuitem"];
    assert!(ROLES.iter().all(|r| WAI_ARIA_1_2.contains(r)));
    use screen_reader_fixtures::*;
    let nodes = vec![
        viewport_node("x", Lang::En),
        scene_list_node(Lang::De),
        entity_node(&entity("B1", Some((0.5, CheckStatus::Pass))), Lang::En, 1, 1, false),
        handle_node(&handle(None), Some(1.0), None, Lang::De),
        check_row_node(&CheckRowFacts { title: LocalizedCopy::new("t", "t"), subject: LocalizedCopy::new("s", "s"), status: CheckStatus::Pass, utilisation: 0.5, quantities: String::new(), clause: String::new() }, Lang::En, 1, 1),
        screen_reader::diagram_nodes(&diagram(), Lang::En, "t").0,
        screen_reader::diagram_nodes(&diagram(), Lang::En, "t").1,
    ];
    for node in &nodes {
        assert!(ROLES.contains(&node.role) && !node.name.is_empty() && node.missing_required().is_empty(), "{node:?}");
        assert!(node.attributes().iter().all(|(k, v)| (*k == "role" || k.starts_with("aria-")) && !v.is_empty()), "{node:?}");
    }
}

#[test]
fn combination_and_governing_check_stepping_are_bound_and_wrap() {
    let key = |key, alt, shift| Chord { key, ctrl: false, alt, shift };
    assert_eq!(lookup(Scope::Viewport, key(".", false, false)), Some(ShortcutAction::NextCombination));
    assert_eq!(lookup(Scope::Viewport, key(",", false, false)), Some(ShortcutAction::PreviousCombination));
    assert_eq!(lookup(Scope::Global, key(".", false, false)), None);
    assert_eq!(lookup(Scope::Handle, key("g", true, false)), Some(ShortcutAction::NextGoverningCheck));
    assert_eq!(lookup(Scope::Viewport, key("g", true, true)), Some(ShortcutAction::PreviousGoverningCheck));
    assert_eq!(step_index(3, Some(2), true), Some(0));
    assert_eq!(step_index(3, Some(0), false), Some(2));
    assert_eq!(step_index(3, None, true), Some(0));
    assert_eq!(step_index(3, None, false), Some(2));
    assert_eq!(step_index(3, Some(9), true), Some(0));
    assert_eq!(step_index(0, None, true), None);
    let utilisations = [Some(0.4), None, Some(1.2), Some(0.9)];
    assert_eq!(step_governing(&utilisations, None, true), Some(2));
    assert_eq!(step_governing(&utilisations, Some(2), true), Some(3));
    assert_eq!(step_governing(&utilisations, Some(3), true), Some(0));
    assert_eq!(step_governing(&utilisations, Some(0), true), Some(1));
    assert_eq!(step_governing(&utilisations, Some(1), true), Some(2));
    assert_eq!(step_governing(&utilisations, Some(2), false), Some(1));
    assert_eq!(step_governing(&[], None, true), None);
    for action in [ShortcutAction::NextCombination, ShortcutAction::PreviousGoverningCheck] {
        assert!(shortcut_action_id(&action).starts_with("norm.shortcut."));
    }
}

#[test]
fn published_nodes_are_kept_per_window_and_domain_and_found_by_the_shell_window_id() {
    use super::published_nodes::*;
    assert_eq!(window_scope("norm-en1993-checks"), Some(("checks", "norm.en1993".to_owned())));
    assert_eq!(window_scope("norm-en1993-viewport"), Some(("viewport", "norm.en1993".to_owned())));
    assert_eq!(window_scope("norm-viewport"), Some(("viewport", String::new())));
    assert_eq!(window_scope("norm-en1993-handle"), Some(("viewport", "norm.en1993".to_owned())));
    assert_eq!((window_scope("norm-en1993-properties"), window_scope("norm-global")), (None, None));
    assert_eq!(domain_of_body("norm.en1993.play.diagrams"), "norm.en1993");
    let nodes = diagram_window_nodes(&[screen_reader_fixtures::diagram()], Lang::De);
    publish("diagrams", "norm.published-test", nodes.clone());
    assert_eq!(published("diagrams", "norm.published-test"), nodes);
    assert!(published("diagrams", "norm.published-other").is_empty() && published("checks", "norm.published-test").is_empty());
    assert_eq!(nodes.iter().map(|n| (n.key.as_str(), n.node.role)).collect::<Vec<_>>(), [("diagram:mn", "img"), ("diagram:mn:table", "table")]);
    assert_eq!(nodes[0].node.described_by.as_deref(), Some("diagram:mn:table"));
    assert!(nodes.iter().all(|n| n.node.missing_required().is_empty()) && nodes[0].node.name == "Interaktion");
}

#[test]
fn the_viewport_publishes_the_scene_list_an_option_per_entity_and_a_slider_per_handle() {
    use super::published_nodes::*;
    use semio_s_plugin_norm_structural_scene::*;
    let scene = StructuralScene {
        schema: SCENE_SCHEMA_ID.into(),
        revision: SceneRevision { document_seq: 1, report_seq: 1 },
        bounds: Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
        entities: vec![screen_reader_fixtures::entity("B1", Some((0.64, CheckStatus::Pass))), screen_reader_fixtures::entity("B2", None)],
        annotations: Vec::new(),
        legends: Vec::new(),
        layers: Vec::new(),
        tracks: Vec::new(),
        owners: Vec::new(),
        description: LocalizedCopy::new("scene", "Szene"),
    };
    let along = Handle { kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: Vec3::new(3.0, 0.0, 0.0), ..screen_reader_fixtures::handle(Some(Interval::new(1.0, 12.0))) };
    assert_eq!(rest_parameter(&along), 3.0);
    assert_eq!(rest_parameter(&screen_reader_fixtures::handle(Some(Interval::new(1.0, 12.0)))), 1.0);
    assert_eq!(rest_parameter(&screen_reader_fixtures::handle(None)), 0.0);
    let nodes = viewport_nodes(&scene, &["B2".to_owned()], &[along], Lang::De);
    assert_eq!(nodes.iter().map(|n| (n.key.as_str(), n.node.role, n.depth)).collect::<Vec<_>>(), [("scene:list", "listbox", 0), ("scene:B1", "option", 1), ("scene:B2", "option", 1), ("handle:h1", "slider", 1)]);
    assert_eq!((nodes[1].node.selected, nodes[2].node.selected), (Some(false), Some(true)));
    let slider = &nodes[3].node;
    assert_eq!(slider.name, "Trägerlänge");
    assert!(slider.value_text.as_deref().is_some_and(|text| text.starts_with('3') && text.ends_with(" m")));
    assert_eq!(slider.range.map(|range| range.now), Some(3.0));
    assert!(nodes.iter().all(|n| n.node.missing_required().is_empty()));
}

#[test]
fn a_grip_change_is_announced_politely_in_both_languages_and_a_run_of_nudges_speaks_only_the_last() {
    use screen_reader_fixtures::handle;
    let h = handle(None);
    let mut region = LiveRegion::default();
    region.push(announce_handle(&h, 6.0, Some((0.9, CheckStatus::Warning))));
    region.push(announce_handle(&h, 6.05, Some((1.12, CheckStatus::Fail))));
    let spoken = region.drain(4);
    assert_eq!(spoken.len(), 1);
    assert_eq!(spoken[0].politeness, Politeness::Polite, "a grip change never interrupts");
    assert_eq!(spoken[0].text.en, "Beam length: 6.05 m, utilisation 112 %, fails");
    assert_eq!(spoken[0].text.de, "Trägerlänge: 6,05 m, Ausnutzung 112 %, versagt");
}
