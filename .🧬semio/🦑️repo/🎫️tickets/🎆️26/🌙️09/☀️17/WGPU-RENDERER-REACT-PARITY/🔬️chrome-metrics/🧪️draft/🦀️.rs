//! 📏️ Registration-ready Chrome width and footer parity laws, held outside the native test module
//! until checkpoint 17 captures the repaired settings/window binary.
//!
//! The production seam this draft expects is a closed optional `ShellNavbarWidthPolicy` on
//! `ShellNavbarControl`, populated only by `shell_example_control`. `Theme::root_rem_pixels` owns the
//! logical root-rem size and starts from the generated `ui_styling::dom::ROOT_REM_PX` token. Both
//! `navbar_control_band_width` and `render_navbar_cluster_step` resolve the retained available width
//! through that policy. Mode, role, and overlay controls keep `width_policy: None`.

use super::*;

fn chrome_metrics_fixture() -> Value {
    let path = repo_root().join("🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🔝️navbar-centered-band/🔣️.json");
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))).expect("navbar centered-band fixture")
}

fn example_control(label: String, minimum_rem: f32, maximum_rem: f32) -> ShellNavbarControl {
    ShellNavbarControl {
        control_id: "playground.navbar.fixture".into(),
        icon_id: Some("file"),
        label,
        active: false,
        width_policy: Some(ShellNavbarWidthPolicy { minimum_rem, maximum_rem }),
    }
}

fn painted_example_hit_width(shell: &mut ShellState, theme: &Theme, control: ShellNavbarControl, available_width: f32) -> f32 {
    let mut cursor = ShellChromeChildCursor { x: 40.0, right: 40.0 + available_width, ..Default::default() };
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    for _ in 0..64 {
        if shell.render_navbar_cluster_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, theme, std::slice::from_ref(&control), 3.2, theme.control_height) {
            break;
        }
    }
    input
        .staged_hits()
        .iter()
        .find(|hit| hit.control_id.as_deref() == Some("playground.navbar.fixture"))
        .expect("the painted example control registers its real hit rectangle")
        .rect
        .w
}

/// 📐️ Exact neutral rows first prove the resolver. The same rows then drive the real navbar
/// reservation and retained painter, whose registered hit rectangle is the painted geometry.
#[test]
fn example_control_root_rem_bounds_feed_reservation_and_painted_hit_geometry() {
    let fixture = chrome_metrics_fixture();
    let metrics = &fixture["exampleControlMetrics"];
    let minimum_rem = metrics["minimumRem"].as_f64().expect("minimum rem") as f32;
    let maximum_rem = metrics["maximumRem"].as_f64().expect("maximum rem") as f32;
    let mut shell = ShellState::new(Vec::new(), "chrome-width-law".into());

    for row in metrics["cases"].as_array().expect("width cases") {
        let root_rem = row["rootRemPixels"].as_f64().expect("root rem") as f32;
        let available = row["availablePixels"].as_f64().expect("available width") as f32;
        let expected = row["expectedPixels"].as_f64().expect("expected width") as f32;
        let expected_reservation = row["expectedReservationPixels"].as_f64().expect("expected reservation") as f32;
        let expected_paint = row["expectedPaintPixels"].as_f64().expect("expected paint") as f32;
        let policy = ShellNavbarWidthPolicy { minimum_rem, maximum_rem };
        assert_eq!(resolve_shell_navbar_control_width(available, Some(policy), root_rem), expected, "{}", row["name"].as_str().expect("case name"));

        let mut theme = Theme::light();
        theme.root_rem_pixels = root_rem;
        let mut atlas = FontAtlas::builtin();
        let control = example_control("Concrete Forest".to_string(), minimum_rem, maximum_rem);
        assert_eq!(shell.navbar_control_band_width(&mut atlas, &theme, std::slice::from_ref(&control), available), expected_reservation);
        assert_eq!(painted_example_hit_width(&mut shell, &theme, control, available), expected_paint);
    }
}

/// 🧵️ The footer row keeps React's exact toggle vocabulary and its position after Settings and
/// Marketplace. This drives `default_dock`, not an invented dock node.
#[test]
fn task_manager_footer_identity_order_icon_and_localized_label_match_react() {
    let fixture = chrome_metrics_fixture();
    let expected = &fixture["taskManagerFooter"];
    for label in expected["labels"].as_array().expect("task manager labels") {
        let mut shell = ShellState::new(Vec::new(), "task-manager-footer-law".into());
        shell.locale_id = label["locale"].as_str().expect("locale").to_string();
        shell.session = Some(ActiveSession {
            plugin_id: "test".into(),
            instance_id: 1,
            app: super::command_registry_tests::test_app(Vec::new(), Vec::new()),
            view_state: ViewModel::default(),
        });
        let dock = shell.default_dock();
        let rows = dock.tabs(PanelAnchor::BottomRight);
        let index = rows.iter().position(|row| row.id == expected["id"].as_str().expect("task manager id")).expect("task manager footer row");
        let row = &rows[index];
        assert_eq!(index as i64, expected["order"].as_i64().expect("task manager order"));
        assert_eq!(row.order as i64, expected["order"].as_i64().expect("task manager order"));
        assert_eq!(row.icon_id, expected["iconId"].as_str().expect("task manager icon"));
        assert_eq!(row.label, label["label"].as_str().expect("task manager label"));
        assert_eq!(rows.get(index.wrapping_sub(1)).map(|row| row.id.as_str()), Some(FRAMEWORK_MARKETPLACE_TAB_ID));
    }
}
