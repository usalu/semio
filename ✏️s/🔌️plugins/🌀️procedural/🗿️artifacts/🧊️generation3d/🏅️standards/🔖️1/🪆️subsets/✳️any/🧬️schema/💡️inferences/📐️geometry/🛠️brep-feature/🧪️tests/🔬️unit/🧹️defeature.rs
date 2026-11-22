//! 🧹️ Shape-value ingress for exact rounded-box fillet and rectangular through-bore removal.

use super::*;
use semio_framework_3d_brep_engine::SolidExchangeBoundary;
use semio_framework_3d_brep_representation::contract::RoundedBox;
use semio_framework_3d_brep_representation::topology::EntityRef;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;

const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1,
    maximum_copy_bytes: 4096,
    maximum_capacity_bytes: 16 << 20,
    maximum_release_bytes: 16 << 20,
    maximum_depth: 512,
};

fn rounded_value(origin: [f64; 3], size: [f64; 3], radius: f64) -> (Arc<ShapeValue>, SelectionValue) {
    let mut session = KernelSession::new();
    let kernel = session.brep();
    let recipe = RoundedBox::new(Pnt3::from_array(origin), size, radius, 1e-6).unwrap();
    let mut plan = kernel.plan_exchange_boundary(SolidExchangeBoundary::RoundedBox(recipe)).unwrap();
    let stock = RetainedCloneGrant { maximum_items: 65536, maximum_copy_bytes: 128 << 20, maximum_capacity_bytes: 256 << 20, maximum_release_bytes: 512 << 20, ..GRANT };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(9271), Generation(1), stock, || Some(0), 1000, 65536);
    for _ in 0..2000 {
        if plan.topology_ready() { break; }
        caller.turn(GRANT, |cx| {
            let (progress, work) = cx.with_original_effect(GRANT, |_, effect| kernel.step_solid_primitive_topology(&mut plan, effect))?.unwrap();
            cx.consume_fuel(work as u64);
            cx.consume_retained(progress?)?;
            Ok(())
        }).unwrap();
    }
    assert!(plan.topology_ready());
    for _ in 0..2000 {
        if kernel.solid_primitive_demand(&plan, GRANT.maximum_copy_bytes).unwrap().capacity_bytes == 0 { break; }
        kernel.reserve_solid_primitive(&plan, GRANT).unwrap();
    }
    let handle = semio_framework_3d_brep_engine::GeometryHandle(plan.handle().to_owned());
    assert!(kernel.publish_solid_primitive(&mut plan, handle.0.clone(), GRANT).is_ok());
    caller.retire(stock);
    let solid = kernel.geometry_solid(&handle).unwrap();
    let body = kernel.representation();
    let shell = body.shells.get(body.solids.get(solid).unwrap().outer).unwrap();
    let selection = SelectionValue { component: SelectionKind::Face,
        ids: shell.faces.iter().skip(6).map(|&face| body.faces.get(face).unwrap().label.0).collect(),
    };
    (Arc::new(session.export(&handle).unwrap()), selection)
}

#[test]
fn imported_rounded_box_is_restored_through_the_registered_widget() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️retained-defeature.json")).unwrap();
    let xyz = |value: &serde_json::Value| serde_json::from_value::<[f64; 3]>(value.clone()).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let (input, selection) = rounded_value(xyz(&row["origin"]), xyz(&row["size"]), row["radius"].as_f64().unwrap());
        let before = input.content_hash();
        let mut changed = (*input).clone();
        let moved = changed.body.vertices.iter().next().unwrap().0;
        changed.body.vertices.get_mut(moved).unwrap().position.x += 0.03125;
        assert!(changed.check().is_ok());
        let (_, refused) = run("brep.feature.defeature", vec![
            ("shape", GeometryValue::Shape(Arc::new(changed))),
            ("faces", GeometryValue::Selection(selection.clone())),
        ], 1);
        assert!(refused.outputs.is_empty());
        assert!(refused.fault.is_some());

        let (steps, result) = run("brep.feature.defeature", vec![
            ("shape", GeometryValue::Shape(input.clone())),
            ("faces", GeometryValue::Selection(selection)),
        ], 1);
        assert!(steps >= 2);
        assert!(result.fault.is_none(), "{:?}", result.fault);
        assert_eq!(input.content_hash(), before);
        let GeometryValue::Shape(shape) = result.outputs.get("shape").unwrap() else { panic!("solid shape") };
        let mut session = KernelSession::new();
        let handle = session.import(shape).unwrap().handle;
        let kernel = session.brep();
        let volume = kernel.volume_sync(&handle).unwrap();
        let area = kernel.area_sync(&handle).unwrap();
        assert!((volume - row["volume"].as_f64().unwrap()).abs() < 1e-9);
        assert!((area - row["area"].as_f64().unwrap()).abs() < 1e-9);
        let body = kernel.representation();
        let reach = body.reachable_from(&[EntityRef::Solid(kernel.geometry_solid(&handle).unwrap())]);
        assert_eq!(reach.faces.len(), fixture["faceCount"].as_u64().unwrap() as usize);
        assert_eq!(reach.edges.len(), fixture["edgeCount"].as_u64().unwrap() as usize);
        assert_eq!(reach.vertices.len(), fixture["vertexCount"].as_u64().unwrap() as usize);
        for edge in reach.edges {
            let uses = body.edge_coedges(edge);
            assert_eq!(uses.len(), 2);
            assert_ne!(body.coedges.get(uses[0]).unwrap().forward, body.coedges.get(uses[1]).unwrap().forward);
        }
    }
}

#[test]
fn imported_through_bore_is_restored_through_the_registered_widget() {
    let mut fixture: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
    fixture["cases"].as_array_mut().unwrap().retain(|case| matches!(case["name"].as_str(),
        Some("defeature fills the through hole" | "defeature fills two distinct through holes")));
    assert_eq!(support::run_fixture(catalogue(), &fixture.to_string(), COMPUTES), 2);
}


#[test]
fn imported_blind_bore_restores_stock_through_the_registered_widget() {
    use semio_framework_3d_brep_analysis::{ShapeScope, face_table};
    use semio_framework_3d_brep_engine::ShapeRoot;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️retained-defeature.json")).unwrap();
    for row in fixture["blindBores"].as_array().unwrap() {
        let shape = Arc::new(support::build(&serde_json::json!({
            "recipe":"cut",
            "target":{"recipe":"box","size":&row["size"]},
            "tool":{"recipe":"translate","by":&row["boreOrigin"],
                "of":{"recipe":"cylinder","radius":&row["radius"],"height":&row["boreHeight"]}}
        })));
        let ShapeRoot::Solid(root) = shape.root else { panic!("blind solid") };
        let wall = face_table(&shape.body, &ShapeScope::solid(root), 1e-6).unwrap().into_iter()
            .find(|face| face.normal.is_none()).unwrap().label;
        let floor = shape.body.solid_faces(root).into_iter().find(|&face| shape.body.face_coedges(face).len() == 1).unwrap();
        let floor = shape.body.faces.get(floor).unwrap().label.0;
        let evaluate = |ids| run("brep.feature.defeature", vec![
            ("shape", GeometryValue::Shape(shape.clone())),
            ("faces", GeometryValue::Selection(SelectionValue { component: SelectionKind::Face, ids })),
        ], 1);
        for lone in [wall, floor] {
            let (_, rejected) = evaluate(vec![lone]);
            assert!(rejected.fault.is_some());
            assert!(rejected.outputs.is_empty());
        }
        let (turns, evaluation) = evaluate(vec![floor, wall]);
        assert!(turns >= 2);
        assert!(evaluation.fault.is_none(), "{:?}", evaluation.fault);
        let GeometryValue::Shape(restored) = evaluation.outputs.get("shape").unwrap() else { panic!("restored stock") };
        let mut session = KernelSession::new();
        let handle = session.import(restored).unwrap().handle;
        let kernel = session.brep();
        assert!((kernel.volume_sync(&handle).unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-9);
        assert!((kernel.area_sync(&handle).unwrap() - row["area"].as_f64().unwrap()).abs() < 1e-9);
    }
}
