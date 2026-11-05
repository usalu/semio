//! 📨️ Final bitmap ticks admit their actual producer effect only after the bounded wire succeeds.
use super::*;
use semio_framework_job::InteractiveJob;

#[test]
fn final_tick_admission_controls_the_actual_completion_port() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🧪️tests/🧫️bitmap-fill-publication/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        for pending in [false, true] {
            let snapshot = BitmapSnapshot::reference_scene();
            let identity = ToolRunIdentity::new(semio_framework_tool_run::ToolRunId { app_instance_id: 1, run: 7 }, [7;32]);
            let port = semio_framework_plugin::ToolRunJobPort::default();
            let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
            let mut job = BitmapFillRunJob::new(identity, port.clone(), snapshot, operation);
            let previous = payload_from_assignment(1, 1, &[(0, 1)], false, false);
            job.last_payload = Some(previous.clone());
            let width = case["width"].as_u64().unwrap() as u32;
            let height = case["height"].as_u64().unwrap() as u32;
            let contradiction = case["contradiction"].as_bool().unwrap();
            let payload = payload_from_assignment(width, height, &[(0, 0)], contradiction, true);
            let mut driver = semio_s_plugin_wfc_engine_oracle::step_context::TestJobDriver::new(operation.operation, operation.generation);
            if case["cancelled"].as_bool().unwrap() { driver.turn(|cx| { cx.original_cancel_token().cancel(); Ok(()) }).unwrap(); }
            driver.turn(|cx| {
                let result = if pending { job.pending_finish = Some((contradiction, payload)); job.run(cx) } else { job.finish_payload(cx, payload, contradiction) };
                if case["cancelled"].as_bool().unwrap() { assert!(matches!(result, FillRun::Cancelled)); }
                else { assert!(matches!(result, FillRun::Staged)); }
                Ok(())
            }).unwrap();
            let admitted = case["admitted"].as_bool().unwrap();
            let commit = case["commit"].as_bool().unwrap();
            assert_eq!(port.has_effects(), commit, "{} pending={pending}", case["id"]);
            assert_eq!(job.committed_solve().is_some(), commit);
            assert_eq!(job.publication.as_ref().is_some_and(|publication| !publication.is_fault()), admitted);
            if !admitted { assert_eq!(job.last_payload(), Some(&previous)); }
            if !admitted && !case["cancelled"].as_bool().unwrap() {
                let mut detail = None;
                for _ in 0..4096 {
                    detail = driver.step(&mut job, 64, |outcome| match outcome {
                        Some(semio_framework_job::JobOutcomeBorrow::Fault { detail, .. }) => Some((0..detail.page_count()).flat_map(|index| detail.page(index).unwrap().to_vec()).collect::<Vec<u8>>()),
                        _ => None,
                    });
                    if detail.is_some() { break; }
                }
                assert_eq!(detail.as_deref(), Some(b"bitmap-fill-payload-capacity".as_slice()), "oversize final publication settles as a named bounded fault");
                assert!(!port.has_effects());
            }
            driver.close(&mut job);
        }
    }
}
