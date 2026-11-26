use semio_s_artifact_puzzle_5d_testkit::{app_with_registry, close_app, dispatch};
use semio_plugin::app::PluginApp;
use semio_plugin::artifact_app_laws::{OriginalFixtureCaller,fixture_renew_turn};
#[semio_framework_async_macros::async_test]
async fn window_load() {
    let mut app = Box::new(app_with_registry());
    let mut reopened = Box::new(app_with_registry());
    let args = semio_framework_pack_json::json!({"camera":{"x":12.5,"y":-6.5,"zoom":3.5}});
    dispatch(&mut app,"setCamera2d",Some(&args),Some("puzzle5d-board-a")).unwrap();
    let pack = app.window_config_packs().await.unwrap().pop().unwrap();
    let (document, history) = pack.source.pair().unwrap();
    let bytes = semio_os_kernel::os_store::serialization::encode_document_pack_bytes(document, history).await;
    let mut entry = semio_os_kernel::WindowConfigPackEntry {window_id:pack.window_id,window_kind_id:pack.window_kind_id,envelope_pack:bytes};
    let mut load = Default::default();
    let grant = OriginalFixtureCaller::event_stock();
    let cx=reopened.1.cx();
    semio_framework_job::testkit::issue_initiating_event(cx,grant).unwrap();
    for n in 0..OriginalFixtureCaller::maximum_turns() {
        fixture_renew_turn(cx,OriginalFixtureCaller::turn_fuel(),grant).unwrap();
        let done = reopened.0.advance_window_config_load(&mut load,&mut entry,cx).unwrap();
        if n < 16 || n.is_power_of_two() {
            eprintln!("[DEBUG] {n} {} {} {} {:?} {:?} {:?}",load.admitted,load.closing,load.done,load.load.as_ref().map(|p|p.phase()),load.failure,load.close_failure);
        }
        if done {break;}
    }
    assert!(load.terminal_is_empty());
    close_app(&mut reopened);
    close_app(&mut app);
}
