//! 🧱️ Runs the BRep topology owner used by the funded primitive producers.

use semio_framework_3d_brep_engine::{plan_owned_analytic_solid, ShapeRoot, ShapeValue, SolidPrimitive};
use semio_framework_3d_brep_vector::Vec3;
use semio_framework_job::{close_original_effect_step, Generation, OperationId, OriginalJobFixtureCaller, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneGrant, ValueError, retirement::{controlled::ControlledRetirement, RetireOwned}};

fn clock() -> Option<u64> { Some(1) }

fn vector(value: &serde_json::Value) -> [f64; 3] {
    std::array::from_fn(|i| value[i].as_f64().unwrap())
}

fn grant(value: &serde_json::Value) -> RetainedCloneGrant {
    let get = |name: &str| value[name].as_u64().unwrap() as usize;
    RetainedCloneGrant {
        maximum_items: get("items"), maximum_copy_bytes: get("copyBytes"),
        maximum_capacity_bytes: get("capacityBytes"), maximum_release_bytes: get("releaseBytes"),
        maximum_depth: get("depth"),
    }
}

fn accounted<T>(caller: &mut OriginalJobFixtureCaller, grant: RetainedCloneGrant, born: &mut usize, released: &mut usize, turn: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> T {
    let before = caller.cumulative_receipt();
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn(grant, turn));
    let after = caller.cumulative_receipt();
    assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
    *born += heap.requested_bytes;
    *released += heap.released_bytes;
    result.unwrap()
}

fn close<T: RetireOwned>(original: T, caller: &mut OriginalJobFixtureCaller, grant: RetainedCloneGrant, born: &mut usize, released: &mut usize) {
    let mut owner = ControlledRetirement::new(original).ok().unwrap();
    for _ in 0..100000 {
        if owner.terminal_is_empty() { break; }
        accounted(caller, grant, born, released, |cx| {
            let body = cx.retained_grant().maximum_copy_bytes;
            let demand = owner.selected_demand(body)?;
            close_original_effect_step(cx, demand, |effect| owner.step(body, effect))?;
            Ok(())
        });
    }
    assert!(owner.terminal_is_empty());
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
}

#[test]
fn original_geometry_solids_publish_checked_topology_and_close_cancelled_backing() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️.json")).unwrap();
    let policy: serde_json::Value = serde_json::from_str(include_str!("../../../🔢️math-values/🧪️tests/🎟️original/🧫️.json")).unwrap();
    let c = &policy["originalCaller"];
    let stock = grant(c);
    let mut caller = OriginalJobFixtureCaller::admit(
        OperationId(c["operation"].as_u64().unwrap()), Generation(c["generation"].as_u64().unwrap()),
        stock, clock, c["deadlineMicros"].as_u64().unwrap(), c["fuel"].as_u64().unwrap(),
    );
    let (mut born, mut released) = (0, 0);
    for row in law["solids"].as_array().unwrap() {
        let number = |port: &str| row["inputs"][port].as_f64().unwrap();
        let primitive = match row["kind"].as_str().unwrap() {
            "brep.primitive.box" => SolidPrimitive::axis_box(number("width"), number("depth"), number("height")),
            "brep.primitive.sphere" => SolidPrimitive::sphere(number("radius")),
            other => panic!("unknown primitive {other}"),
        };
        for cancel in [false, true] {
            let (owner, mut plan) = accounted(&mut caller, stock, &mut born, &mut released, |cx| {
                let owner = cx.original_source_parent()?.source_identity()?;
                Ok((owner, plan_owned_analytic_solid(owner, primitive)?))
            });
            let mut output = None;
            let mut advance = |cx: &mut StepContext<'_>| -> Result<(f32, bool), OriginalTurnFailure> {
                let grant = cx.retained_grant();
                if let Some((step, work)) = cx.with_original_source_effect(grant, |_, effect| -> Result<_, ValueError> {
                    let (shape, receipt) = plan.step(owner, effect)?;
                    if let Some((body, root)) = shape { output = Some(ShapeValue { root: ShapeRoot::Solid(root), body }); }
                    Ok(receipt)
                })? {
                    cx.consume_work(work);
                    cx.consume_retained(step?)?;
                }
                Ok((plan.progress(), output.is_some()))
            };
            let before = caller.cumulative_receipt();
            let denied = RetainedCloneGrant { maximum_capacity_bytes: 0, ..stock };
            accounted(&mut caller, denied, &mut born, &mut released, &mut advance);
            assert_eq!(caller.cumulative_receipt(), before);
            let topology_born = born;
            let mut cancelled = false;
            for _ in 0..10000 {
                let (progress, ready) = accounted(&mut caller, stock, &mut born, &mut released, &mut advance);
                if cancel && progress > 0.0 {
                    assert!(born > topology_born && !ready);
                    cancelled = true;
                    break;
                }
                if ready { break; }
            }
            if cancelled {
                assert!(output.is_none());
                close(plan, &mut caller, stock, &mut born, &mut released);
                continue;
            }
            let shape = output.expect("original primitive must finish under the supported grant");
            let ShapeRoot::Solid(root) = shape.root else { panic!("actual Solid root"); };
            shape.check().unwrap();
            assert_eq!(shape.body.solid_faces(root).len(), row["faces"].as_u64().unwrap() as usize);
            let mesh = shape.tessellate(0.005).unwrap();
            assert!(!mesh.index.is_empty());
            let bounds = [vector(&row["bounds"][0]), vector(&row["bounds"][1])];
            for axis in 0..3 {
                let coordinates = mesh.position.chunks_exact(3).map(|point| f64::from(point[axis]));
                let lo = coordinates.clone().fold(f64::INFINITY, f64::min);
                let hi = coordinates.fold(f64::NEG_INFINITY, f64::max);
                assert!((lo - bounds[0][axis]).abs() < 0.02 && (hi - bounds[1][axis]).abs() < 0.02, "axis {axis} bounds {lo}..{hi} differ from {bounds:?}");
            }
            let point = |i: u32| {
                let i = i as usize * 3;
                Vec3::new(mesh.position[i].into(), mesh.position[i + 1].into(), mesh.position[i + 2].into())
            };
            let volume = mesh.index.chunks_exact(3).map(|tri| point(tri[0]).dot(point(tri[1]).cross(point(tri[2]))) / 6.0).sum::<f64>().abs();
            let expected = row["volume"].as_f64().unwrap();
            assert!((volume - expected).abs() / expected < 0.025, "tessellated volume {volume} differs from {expected}");
            close((plan, shape), &mut caller, stock, &mut born, &mut released);
        }
    }
    assert_eq!(born, released);
    caller.retire(stock);
}
