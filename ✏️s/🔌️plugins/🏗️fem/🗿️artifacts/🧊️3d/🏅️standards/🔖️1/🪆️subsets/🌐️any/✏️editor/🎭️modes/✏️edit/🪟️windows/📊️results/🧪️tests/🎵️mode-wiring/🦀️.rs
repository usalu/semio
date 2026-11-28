//! 🎵️ Laws of the mounted results frame for mode shapes: the displayed mode index, never a stale static field, decides the caption.

use super::*;

struct Lent<'a> {
    snapshot: &'a Fem3dSnapshot,
    mode: Option<f64>,
    current: bool,
}

impl Fem3dResultFields for Lent<'_> {
    fn snapshot(&self) -> &Fem3dSnapshot { self.snapshot }
    fn len(&self) -> usize { self.snapshot.nodes.iter().count() }
    fn node_id(&self, index: usize) -> Option<&str> { self.snapshot.nodes.iter().nth(index).map(|node| node.id.as_str()) }
    fn current_revision(&self) -> bool { self.current }
    fn static_values(&self, index: usize) -> Option<[f64; 6]> { Some([index as f64 + 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]) }
    fn mode_values(&self, index: usize) -> Option<([f64; 6], f64)> { self.mode.map(|eigenvalue| ([index as f64 + 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], eigenvalue)) }
    fn stress(&self, _index: usize) -> Option<f64> { None }
}

struct Lender<'a> {
    snapshot: &'a Fem3dSnapshot,
    mode: Option<f64>,
    current: bool,
}

impl Fem3dLiveSession for Lender<'_> {
    fn result_stage(&self, _operation: Option<semio_plugin::AppRenderOperationContext>, _authority: Option<semio_plugin::WindowConfigAuthorityIdentity>) -> Option<&'static str> { Some("fem3d.modes") }
    fn with_result_fields(&self, _operation: Option<semio_plugin::AppRenderOperationContext>, _doc: &Fem3dSnapshot, _authority: Option<semio_plugin::WindowConfigAuthorityIdentity>, _request: Fem3dResultRequest, read: &mut dyn FnMut(Option<&dyn Fem3dResultFields>) -> semio_plugin::UiAssemblyResult<BuiltNode>) -> semio_plugin::UiAssemblyResult<BuiltNode> {
        read(Some(&Lent { snapshot: self.snapshot, mode: self.mode, current: self.current }))
    }
    fn with_live_visual(&self, _operation: Option<semio_plugin::AppRenderOperationContext>, build: &mut dyn FnMut(Option<&Fem3dPageVisualLease>) -> semio_plugin::UiAssemblyResult<BuiltNode>) -> semio_plugin::UiAssemblyResult<BuiltNode> { build(None) }
}

fn caption(doc: &Fem3dSnapshot, mode: semio_s_fem_app_surface::ResultMode, index: u32, lent: Option<f64>) -> String {
    frame(doc, mode, index, lent, true, &Fem3dLabels::NATIVE_EN, semio_plugin::Locale::En)
}

fn frame(doc: &Fem3dSnapshot, mode: semio_s_fem_app_surface::ResultMode, index: u32, lent: Option<f64>, current: bool, labels: &Fem3dLabels, locale: semio_plugin::Locale) -> String {
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 };
    let operation = semio_plugin::AppRenderOperationContext { app_instance_id: 9002, base_revision: semio_framework_job::RevisionId(11), generation: semio_framework_job::Generation(19), canonical_base_revision: [23; 32], mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } };
    let cfg = Fem3dResultsWindowConfig { result_mode: mode, result_mode_index: index, ..Default::default() };
    let node = render(doc, &cfg, &Fem3dInteractionSnapshot::default(), Some(operation), semio_plugin::WindowConfigAuthorityIdentity::issue(), &Lender { snapshot: doc, mode: lent, current }, labels, locale).expect("mounted mode frame");
    semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).expect("fixture projection")
}

fn demo() -> Retiring<Fem3dSnapshot> {
    let mut doc = Retiring::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("demo"));
    doc.analysis.modal_count = 4;
    doc
}

/// 🧪️ LAW: a mounted frame whose lent fields carry no mode values (the static-only generation still held while the mode job runs) never claims `0.000 Hz`: it names the mode stage instead.
#[test]
fn a_static_only_generation_never_draws_a_zero_hertz_mode() {
    let doc = demo();
    for index in 0..4 {
        let json = caption(&doc, semio_s_fem_app_surface::ResultMode::Modal, index, None);
        assert!(!json.contains("0.000 Hz"), "mode {index} must not show a fake zero frequency: {json}");
        assert!(json.contains("Computing modes"), "mode {index} names the running stage: {json}");
    }
}

/// 🧪️ LAW: the mode index shows in the caption (`Mode n`) and the lent eigenvalue shows as its frequency, in hertz, for every index.
#[test]
fn the_mode_index_and_its_frequency_reach_the_caption() {
    let doc = demo();
    let mut seen = Vec::new();
    for index in 0..4u32 {
        let hertz = 5.0 * f64::from(index + 1);
        let json = caption(&doc, semio_s_fem_app_surface::ResultMode::Modal, index, Some((std::f64::consts::TAU * hertz).powi(2)));
        assert!(json.contains(&format!(": {hertz:.3} Hz")), "mode {index} shows {hertz} Hz: {json}");
        assert!(json.contains(&format!(" {}: ", index + 1)), "mode {index} shows its ordinal: {json}");
        seen.push(json);
    }
    seen.dedup();
    assert_eq!(seen.len(), 4, "every index draws a different caption");
}

/// 🧪️ LAW: a buckling frame shows the lent load factor, and a lent static-only generation stays pending instead of showing zero.
#[test]
fn the_buckling_factor_reaches_the_caption_and_a_static_generation_stays_pending() {
    let doc = demo();
    let json = caption(&doc, semio_s_fem_app_surface::ResultMode::Buckling, 0, Some(12.5));
    assert!(json.contains("12.500"), "the load factor shows: {json}");
    let json = caption(&doc, semio_s_fem_app_surface::ResultMode::Buckling, 0, None);
    assert!(json.contains("Computing modes"), "a static-only generation stays pending: {json}");
}

/// 🧪️ LAW: a document edit that only changes display settings (the deformation scale) never yields a frame without results: the retained generation is drawn at the new scale, captioned as updating in both languages, and once the new generation binds the caption is unmarked.
#[test]
fn a_display_setting_edit_keeps_the_previous_generation_drawn_until_the_new_one_binds() {
    let mut doc = demo();
    doc.analysis.deformation_scale = 1.0;
    let before = frame(&doc, semio_s_fem_app_surface::ResultMode::Static, 0, None, true, &Fem3dLabels::NATIVE_EN, semio_plugin::Locale::En);
    doc.analysis.deformation_scale = 40.0;
    let en = frame(&doc, semio_s_fem_app_surface::ResultMode::Static, 0, None, false, &Fem3dLabels::NATIVE_EN, semio_plugin::Locale::En);
    let de = frame(&doc, semio_s_fem_app_surface::ResultMode::Static, 0, None, false, &Fem3dLabels::NATIVE_DE, semio_plugin::Locale::De);
    let bound = frame(&doc, semio_s_fem_app_surface::ResultMode::Static, 0, None, true, &Fem3dLabels::NATIVE_EN, semio_plugin::Locale::En);
    assert!(en.contains("Updating results…") && en.contains("Case:"), "the retained frame keeps its results and says it is updating: {en}");
    assert!(de.contains("Ergebnisse werden aktualisiert…"), "the German caption names the update: {de}");
    assert_ne!(before, bound, "the new scale redraws the lent fields without a new solve");
    assert!(!bound.contains("Updating results…") && bound.contains("Case:"), "the bound generation is unmarked: {bound}");
}
