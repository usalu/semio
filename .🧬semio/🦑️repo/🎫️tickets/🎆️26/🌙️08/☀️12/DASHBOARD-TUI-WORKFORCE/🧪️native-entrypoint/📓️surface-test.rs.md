
/// 🪟️ Window handles are captured synchronously before GPU device preparation is delegated.
#[test]
fn native_surface_capture_precedes_worker_device_preparation() {
    fn assert_factory<F: std::future::Future<Output = Result<GpuContext, String>> + Send>(_: impl FnOnce(Arc<winit::window::Window>) -> Result<F, String>) {}
    assert_factory(GpuContext::from_window);
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🪟️surface/🔣️.json")).expect("surface contract");
    let source = include_str!("../../🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs");
    let resumed = &source[source.find("fn resumed(").expect("window callback")..];
    let mut phases = vec![(resumed.find("event_loop.create_window(").expect("create window"), "window"), (resumed.find("GpuContext::from_window(").expect("capture surface"), "surface"), (resumed.find("crate::spawn_app_task(").expect("delegate device preparation"), "worker")];
    phases.sort_by_key(|row| row.0);
    assert_eq!(serde_json::to_value(phases.iter().map(|row| row.1).collect::<Vec<_>>()).expect("phase order"), contract["hostPhases"]);
}
