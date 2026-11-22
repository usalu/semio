//! 🧹️ Funded registered Defeature tests over live rounded boxes and rectangular bores.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::Session;
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};

include!("🧰️original-support.rs");

#[test]
fn retained_defeature_restores_original_stock_and_refuses_ambiguous_faces() {
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{read_channel_number, read_geometry, text_dictionary, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️retained-defeature.json")).unwrap();
    let operator = fixture["operator"].as_str().unwrap();
    for row in fixture["cases"].as_array().unwrap().iter().chain([&fixture["throughBore"]]).chain(fixture["blindBores"].as_array().unwrap()) {
        let has_bore = row.get("boreOrigin").is_some();
        let blind = row.get("floorZ").is_some();
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(98152), Generation(17), stock(), || Some(1000), 1500, 131072);
        let (mut born, mut free) = (0, 0);
        let stock_box = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&row["size"]))
            .fold(Dictionary::new(), |input, (axis, size)| input.insert(axis, number(size)));
        let base = run_live_query(&registry, "brep.prim3d.box", stock_box, &mut caller, &mut born, &mut free);
        let placement = Dictionary::new().insert("geometry", base.get("solid").unwrap().clone())
            .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let placed_stock = run_live_query(&registry, "brep.xform.translate", placement, &mut caller, &mut born, &mut free);
        let source = |caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize| {
            let radius = number(row["radius"].as_f64().unwrap());
            let placed = placed_stock.get("geometryOut").unwrap().clone();
            if has_bore {
                let request = Dictionary::new().insert("radius", radius).insert("height", number(row["boreHeight"].as_f64().unwrap()));
                let tool = run_live_query(&registry, "brep.prim3d.cylinder", request, caller, born, free);
                let request = Dictionary::new().insert("geometry", tool.get("solid").unwrap().clone())
                    .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&row["boreOrigin"]))));
                let tool = run_live_query(&registry, "brep.xform.translate", request, caller, born, free);
                let request = Dictionary::new().insert("a", placed).insert("b", tool.get("geometryOut").unwrap().clone());
                return run_live_query(&registry, "brep.bool.cut", request, caller, born, free);
            }
            let request = Dictionary::new().insert("geometry", placed).insert("radius", radius);
            run_live_query(&registry, "brep.solid.fillet", request, caller, born, free)
        };
        let other = source(&mut caller, &mut born, &mut free);
        let source = source(&mut caller, &mut born, &mut free);
        let select = |solid: &Dictionary, feature, caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize| {
            let handle = read_geometry(solid, "solid").unwrap();
            let labels = session.with_kernel_read(|kernel| {
                let body = kernel.representation();
                let labels: Vec<String> = body.solid_faces(kernel.geometry_solid(&handle).unwrap()).into_iter().rev()
                    .filter_map(|id| {
                        let face = body.faces.get(id).unwrap();
                        let plane = matches!(body.surfaces.get(face.surface), Some(Surface::Plane { .. }));
                        let floor = blind && plane && body.face_coedges(id).len() == 1;
                        ((!plane || floor) == feature).then(|| face.label.0.to_string())
                    }).collect();
                Ok(labels)
            }).unwrap();
            let request = Dictionary::new().insert("brep", solid.get("solid").unwrap().clone())
                .insert("faceLabels", Value::Dictionary(text_dictionary(serde_json::to_string(&labels).unwrap())))
                .insert("edgeLabels", Value::Dictionary(text_dictionary("[]")))
                .insert("sourceHandle", Value::Dictionary(text_dictionary("")));
            run_live_query(&registry, "brep.brep", request, caller, born, free)
        };
        let selected = select(&source, true, &mut caller, &mut born, &mut free);
        let foreign = select(&other, true, &mut caller, &mut born, &mut free);
        let planar = select(&source, false, &mut caller, &mut born, &mut free);
        let faces = selected.get("selectedFaces").and_then(Value::as_dictionary).unwrap();
        let foreign = foreign.get("selectedFaces").and_then(Value::as_dictionary).unwrap();
        let planar = planar.get("selectedFaces").and_then(Value::as_dictionary).unwrap();
        let input = |faces: Dictionary| Dictionary::new().insert("geometry", source.get("solid").unwrap().clone())
            .insert("faces", Value::Dictionary(faces));
        let counts = || session.with_kernel_read(|kernel| {
            let body = kernel.representation();
            Ok([kernel.live_handles().len(), body.vertices.len(), body.edges.len(), body.faces.len(), body.solids.len()])
        }).unwrap();
        let before = counts();
        let mut invalid = vec![
            faces.clone().insert("0", foreign.get("0").unwrap().clone()),
            faces.clone().insert("0", planar.get("0").unwrap().clone()),
        ];
        if has_bore {
            invalid.push(Dictionary::with_schema("list"));
        }
        if blind {
            for key in ["0", "1"] {
                invalid.push(Dictionary::with_schema("list").insert("0", faces.get(key).unwrap().clone()));
            }
            invalid.push(faces.clone().insert("0", faces.get("1").unwrap().clone()));
        }
        if !has_bore {
            invalid.push((0..19).fold(Dictionary::with_schema("list"), |list, index| {
                let key = index.to_string();
                let face = faces.get(&key).unwrap().clone();
                list.insert(key, face)
            }));
            invalid.push(faces.clone().insert("0", faces.get("1").unwrap().clone()));
        }
        for invalid in invalid {
            let mut job = Some(admit_live_job(&registry, operator, input(invalid)));
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                assert!(job.as_mut().unwrap().step(cx).is_err(), "invalid selection must fail");
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert_eq!(counts(), before);
        }
        let mut job = Some(admit_live_job(&registry, operator, input(faces.clone())));
        let unfunded = Grant { maximum_items: 0, ..stock() };
        run(&mut caller, unfunded, &mut born, &mut free, |cx| {
            let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
            assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
            assert_eq!(receipt, Progress::default());
            Ok(())
        });
        assert_eq!(counts(), before);
        let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
        for _ in 0..2000 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                assert!(receipt.fits(grant));
                Ok(())
            });
            assert_eq!(counts()[0], before[0]);
            if counts()[3] > before[3] { break; }
        }
        assert!(counts()[3] > before[3]);
        job.as_mut().unwrap().cancel();
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        assert_eq!(counts(), before);

        let repaired = run_live_query(&registry, operator, input(faces.clone()), &mut caller, &mut born, &mut free);
        assert_eq!(counts()[0], before[0] + 1);
        let handle = read_geometry(&repaired, "solid").unwrap();
        session.with_kernel_read(|kernel| {
            let body = kernel.representation();
            let reach = body.reachable_from(&[EntityRef::Solid(kernel.geometry_solid(&handle).unwrap())]);
            for (count, key) in [(reach.faces.len(), "faceCount"), (reach.edges.len(), "edgeCount"), (reach.vertices.len(), "vertexCount")] {
                assert_eq!(count, fixture[key].as_u64().unwrap() as usize);
            }
            for edge in reach.edges {
                let uses = body.edge_coedges(edge);
                assert_eq!(uses.len(), 2);
                assert_ne!(body.coedges.get(uses[0]).unwrap().forward, body.coedges.get(uses[1]).unwrap().forward);
            }
            let mut bounds = [[f64::INFINITY; 3], [f64::NEG_INFINITY; 3]];
            for id in reach.vertices {
                let point = body.vertices.get(id).unwrap().position.to_array();
                for (axis, value) in point.into_iter().enumerate() {
                    bounds[0][axis] = bounds[0][axis].min(value);
                    bounds[1][axis] = bounds[1][axis].max(value);
                }
            }
            assert_eq!(bounds, [fixture_xyz(&row["bounds"][0]), fixture_xyz(&row["bounds"][1])]);
            let source_handle = read_geometry(&source, "solid").unwrap();
            let source_id = kernel.geometry_solid(&source_handle).unwrap();
            let source_faces = body.solid_faces(source_id);
            assert_eq!(source_faces.len(), if blind { 8 } else if has_bore { 7 } else { 26 });
            if blind {
                let (_, bore) = kernel.live_bore_preview(source_handle.as_str()).expect("certified blind bore");
                let top_open = row["openEnd"] == "top";
                assert!(bore.is_open(top_open));
                assert!(!bore.is_open(!top_open));
                let floors: Vec<_> = source_faces.into_iter().filter_map(|id| {
                    let face = body.faces.get(id)?;
                    let Surface::Plane { frame } = body.surfaces.get(face.surface)? else { return None; };
                    (body.face_coedges(id).len() == 1).then_some(frame.origin.z)
                }).collect();
                assert_eq!(floors, [row["floorZ"].as_f64().unwrap()]);
            }
            Ok(())
        }).unwrap();
        for (operator, measure) in [("brep.measure.volume", "volume"), ("brep.measure.area", "area")] {
            let query = Dictionary::new().insert("geometry", repaired.get("solid").unwrap().clone());
            let result = run_live_query(&registry, operator, query, &mut caller, &mut born, &mut free);
            assert!((read_channel_number(&result, measure).unwrap() - row[measure].as_f64().unwrap()).abs() < 1e-9);
        }
        caller.retire(stock());
    }
}
