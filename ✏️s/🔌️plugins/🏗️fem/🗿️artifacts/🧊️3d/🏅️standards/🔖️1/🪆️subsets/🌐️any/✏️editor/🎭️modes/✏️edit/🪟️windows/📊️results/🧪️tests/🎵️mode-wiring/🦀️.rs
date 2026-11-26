//! 🎵️ Laws of the mounted results frame for mode shapes: the displayed mode index, never a stale static field, decides the caption.

use super::*;

struct Lent<'a> {
    snapshot: &'a Fem3dSnapshot,
    mode: Option<f64>,
}

impl Fem3dResultFields for Lent<'_> {
    fn snapshot(&self) -> &Fem3dSnapshot { self.snapshot }
    fn len(&self) -> usize { 2 }
    fn node_id(&self, index: usize) -> Option<&str> { ["a", "b"].get(index).copied() }
    fn current_revision(&self) -> bool { true }
    fn static_values(&self, _index: usize) -> Option<[f64; 6]> { Some([0.0; 6]) }
    fn mode_values(&self, index: usize) -> Option<([f64; 6], f64)> { self.mode.map(|eigenvalue| ([index as f64 + 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], eigenvalue)) }
    fn stress(&self, _index: usize) -> Option<f64> { None }
}

struct Lender<'a> {
    snapshot: &'a Fem3dSnapshot,
    mode: Option<f64>,
}

impl Fem3dLiveSession for Lender<'_> {
    fn result_stage(&self, _operation: Option<semio_plugin::AppRenderOperationContext>, _authority: Option<semio_plugin::WindowConfigAuthorityIdentity>) -> Option<&'static str> { Some("fem3d.modes") }
    fn with_result_fields(&self, _operation: Option<semio_plugin::AppRenderOperationContext>, _doc: &Fem3dSnapshot, _authority: Option<semio_plugin::WindowConfigAuthorityIdentity>, _request: Fem3dResultRequest, read: &mut dyn FnMut(Option<&dyn Fem3dResultFields>) -> semio_plugin::UiAssemblyResult<BuiltNode>) -> semio_plugin::UiAssemblyResult<BuiltNode> {
        read(Some(&Lent { snapshot: self.snapshot, mode: self.mode }))
    }
    fn with_live_visual(&self, _operation: Option<semio_plugin::AppRenderOperationContext>, build: &mut dyn FnMut(Option<&Fem3dPageVisualLease>) -> semio_plugin::UiAssemblyResult<BuiltNode>) -> semio_plugin::UiAssemblyResult<BuiltNode> { build(None) }
}

fn caption(doc: &Fem3dSnapshot, mode: semio_s_fem_app_surface::ResultMode, index: u32, lent: Option<f64>) -> String {
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 };
    let operation = semio_plugin::AppRenderOperationContext { app_instance_id: 9002, base_revision: semio_framework_job::RevisionId(11), generation: semio_framework_job::Generation(19), canonical_base_revision: [23; 32], mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } };
    let cfg = Fem3dResultsWindowConfig { result_mode: mode, result_mode_index: index, ..Default::default() };
    let node = render(doc, &cfg, &Fem3dInteractionSnapshot::default(), Some(operation), semio_plugin::WindowConfigAuthorityIdentity::issue(), &Lender { snapshot: doc, mode: lent }, &Fem3dLabels::NATIVE_EN, semio_plugin::Locale::En).expect("mounted mode frame");
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
