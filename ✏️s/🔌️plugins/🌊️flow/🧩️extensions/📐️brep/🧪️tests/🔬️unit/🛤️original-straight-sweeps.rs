//! 🛤️ Funded original Sweep and guided Pipe publication through the authored Flow registry.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;

include!("🧰️original-support.rs");

#[test]
fn original_straight_sweeps_use_directed_displacement_and_keep_profile_placement() {
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-straight-sweeps.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97174, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let bent = retained_boolean_result(&registry, "brep.curve.polyline", polygon_fixture_input(&serde_json::json!({"points": fixture["unsupportedPath"]})), &mut caller, &mut born, &mut free);
    for row in fixture["cases"].as_array().unwrap() {
        let (constructor, channel, profile_input) = if row["kind"] == "circle" {
            let input = Dictionary::new()
                .insert("center", Value::Dictionary(point_dictionary(fixture_xyz(&row["center"]))))
                .insert("normal", Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))))
                .insert("radius", number(row["radius"].as_f64().unwrap()));
            ("brep.curve.circle", "curve", input)
        } else {
            let input = Dictionary::new().insert("width", number(row["size"][0].as_f64().unwrap()))
                .insert("height", number(row["size"][1].as_f64().unwrap()));
            ("brep.curve.rectangle", "wire", input)
        };
        let boundary = retained_boolean_result(&registry, constructor, profile_input, &mut caller, &mut born, &mut free);
        let input = Dictionary::new().insert("wire", boundary.get(channel).unwrap().clone());
        let profile = retained_boolean_result(&registry, "brep.util.faceFromWire", input, &mut caller, &mut born, &mut free);
        let input = polygon_fixture_input(&serde_json::json!({"points": row["path"]}));
        let path = retained_boolean_result(&registry, "brep.curve.polyline", input, &mut caller, &mut born, &mut free);
        let input = |path: &Dictionary| Dictionary::new().insert("profile", profile.get("face").unwrap().clone())
            .insert("path", path.get("wire").unwrap().clone());
        let mut request = input(&path);
        if row["guide"].is_array() {
            let rail = polygon_fixture_input(&serde_json::json!({"points": row["guide"]}));
            let guide = retained_boolean_result(&registry, "brep.curve.polyline", rail, &mut caller, &mut born, &mut free);
            request = request.insert("guide", guide.get("wire").unwrap().clone());
        }
        let operator = row["operator"].as_str().unwrap();
        let copy_bytes = if row["guide"].is_array() { 192 } else { 128 };
        let result = retained_result_with_copy_grant(&registry, operator, request, copy_bytes, &mut caller, &mut born, &mut free);
        let shape = read_geometry(&result, "solid").unwrap();
        session.with_kernel_read(|kernel| {
            let solid = kernel.geometry_solid(&shape).unwrap();
            let body = kernel.representation();
            let reach = body.reachable_from(&[EntityRef::Solid(solid)]);
            assert_eq!(reach.faces.len(), row["faces"].as_u64().unwrap() as usize);
            for &id in &reach.edges {
                let uses = body.edge_coedges(id);
                assert_eq!(uses.len(), 2);
                let orientation = |id| {
                    let use_ = body.coedges.get(id).unwrap();
                    let face = body.loops.get(use_.loop_id).unwrap().face;
                    use_.forward != body.faces.get(face).unwrap().flipped
                };
                assert_ne!(orientation(uses[0]), orientation(uses[1]));
            }
            Ok(())
        }).unwrap();
        let mesh = run_funded_preview(&session, identity, shape.as_str(), &mut caller, &mut born, &mut free);
        mesh.validate_component_references().unwrap();
        let (min, max) = mesh.aabb();
        for (actual, expected) in [(min, &row["bounds"][0]), (max, &row["bounds"][1])] {
            assert!(actual.into_iter().zip(fixture_xyz(expected)).all(|(actual, expected)| (f64::from(actual) - expected).abs() < 0.04));
        }
        let read = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone());
        let volume = retained_boolean_result(&registry, "brep.measure.volume", read, &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&volume, "volume").unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-10);

        let varying = row["varyingGuide"].is_array().then(|| {
            let rail = polygon_fixture_input(&serde_json::json!({"points": row["varyingGuide"]}));
            retained_boolean_result(&registry, "brep.curve.polyline", rail, &mut caller, &mut born, &mut free)
        });
        let before = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
        let guide = path.get("wire").unwrap().clone();
        let mut refusals = vec![(operator, input(&bent)), ("brep.sweep.pipe", input(&path).insert("guide", guide))];
        if let Some(varying) = varying {
            refusals.push(("brep.sweep.pipe", input(&path).insert("guide", varying.get("wire").unwrap().clone())));
        }
        for (operator, refused) in refusals {
            let mut job = Some(admit_live_job(&registry, operator, refused));
            run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 256, ..stock() }, &mut born, &mut free, |cx| {
                assert!(job.as_mut().unwrap().step(cx).is_err());
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
        }
        assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), before);
    }
    caller.retire(stock());
}
