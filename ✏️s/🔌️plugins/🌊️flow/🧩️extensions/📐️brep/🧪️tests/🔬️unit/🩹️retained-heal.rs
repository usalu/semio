//! 🧭️ Original Flow admission, cancellation, and publication of selected solid healing.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;

include!("🧰️original-support.rs");

#[test]
fn retained_heal_repairs_one_selected_solid_and_cancels_before_publication() {
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{geometry_dict, map_kernel_error, read_geometry};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️retained-heal.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let original = fixture_xyz(&fixture["original"]);
    let shift = fixture_xyz(&fixture["shift"]);
    let moved = [original[0] + shift[0], original[1] + shift[1], original[2] + shift[2]];
    let point = |[x, y, z]| Pnt3::new(x, y, z);
    let (selected, other) = session.with_kernel(|kernel| {
        let [width, depth, height] = fixture_xyz(&fixture["size"]);
        let selected = kernel.box_prim_sync(width, depth, height).map_err(|error| map_kernel_error(&error))?;
        let mut shape = kernel.export_shape(&selected).map_err(|error| map_kernel_error(&error))?;
        let vertex = shape.body.vertices.iter().find(|(_, vertex)| vertex.position == point(original)).unwrap().0;
        shape.body.vertices.get_mut(vertex).unwrap().position = point(moved);
        let selected = kernel.import_shape(&shape).map_err(|error| map_kernel_error(&error))?;
        let other = kernel.import_shape(&shape).map_err(|error| map_kernel_error(&error))?;
        Ok((geometry_dict(kernel, &selected)?, geometry_dict(kernel, &other)?))
    }).unwrap();
    let input = |tolerance| Dictionary::new().insert("geometry", Value::Dictionary(selected.clone())).insert("tolerance", number(tolerance));
    let snapshot = |geometry: &Dictionary| {
        let handle = read_geometry(&Dictionary::new().insert("solid", Value::Dictionary(geometry.clone())), "solid").unwrap();
        session.with_kernel_read(|kernel| {
            let shape = kernel.export_shape(&handle).map_err(|error| map_kernel_error(&error))?;
            Ok(shape.body.vertices.iter().map(|(_, vertex)| vertex.position).collect::<Vec<_>>())
        }).unwrap()
    };
    let before = snapshot(&selected);
    let unrelated = snapshot(&other);
    assert!(before.contains(&point(moved)));
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97779), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut released) = (0, 0);
    let mut job = Some(admit_live_job(&registry, "brep.util.heal", input(tolerance)));
    let mut ready_to_publish = false;
    for _ in 0..10000 {
        run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut released, |cx| {
            let (step, _) = job.as_mut().unwrap().step(cx).unwrap();
            let neural_engine::OperatorJobStep::Working(progress) = step else { panic!("Heal published before cancellation"); };
            ready_to_publish = progress.phase == "healing-publication";
            Ok(())
        });
        if ready_to_publish { break; }
    }
    assert!(ready_to_publish);
    close_live_job(&mut job, &mut caller, &mut born, &mut released);
    assert_eq!(snapshot(&selected), before);
    assert_eq!(snapshot(&other), unrelated);
    let mut refused = Some(admit_live_job(&registry, "brep.util.heal", input(fixture["refusalTolerance"].as_f64().unwrap())));
    let mut rejected = false;
    for _ in 0..10000 {
        run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut released, |cx| {
            match refused.as_mut().unwrap().step(cx) {
                Err(_) => rejected = true,
                Ok((neural_engine::OperatorJobStep::Working(_), _)) => (),
                Ok(_) => panic!("Heal published an out-of-tolerance repair"),
            }
            Ok(())
        });
        if rejected { break; }
    }
    assert!(rejected);
    close_live_job(&mut refused, &mut caller, &mut born, &mut released);
    assert_eq!(snapshot(&selected), before);
    let output = run_live_query(&registry, "brep.util.heal", input(tolerance), &mut caller, &mut born, &mut released);
    let Value::Dictionary(solid) = output.get("solid").unwrap() else { panic!("Heal must publish a solid"); };
    let repaired = snapshot(solid);
    assert!(repaired.contains(&point(original)));
    assert!(!repaired.contains(&point(moved)));
    assert_eq!(snapshot(&other), unrelated);
    caller.retire_exact();
}
