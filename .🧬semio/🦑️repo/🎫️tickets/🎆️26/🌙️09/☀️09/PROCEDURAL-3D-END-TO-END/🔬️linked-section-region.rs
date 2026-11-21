//! 🧵️ Native publication of one exact Section face with a private void ring.

use semio_framework_3d_brep_analysis::{PlanarMeasure, PlanarMeasureSource};
use semio_framework_3d_brep_construction::primitives::make_box;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_native_registry::{BrepRegistry, Entity, PolygonFacePublication, SectionPlan};
use semio_framework_3d_brep_representation::{
    arena::SolidId,
    contract::SectionPolygon,
    topology::{Body, history::OpRecorder},
};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::Vec3;
use semio_framework_value::{
    RetainedAllocationEffect, RetainedCloneGrant as Grant, ValueError,
    retained_clone::{OriginalSourceTurnIdentity, OriginalWork, OriginalWorkAuthority, OriginalWorkPartialCheck,
        OriginalWorkPartialCheckComplete, OriginalWorkQuote, OriginalWorkQuoteComplete, OriginalWorkRecipient},
};

struct Work;

impl OriginalWorkRecipient for Work {
    type Context<'a,'b> = () where 'b: 'a;
    fn validate_context<'a,'b>(&self, _: &()) -> Result<(),ValueError> where 'b: 'a { Ok(()) }
    fn source_turn<'a,'b>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'b: 'a { None }
    fn admits<'r,'a,'b>(&mut self, _: &mut (), quote: OriginalWorkQuote<'r>, _: u64, _: u64) -> OriginalWorkQuoteComplete<'r> where 'b: 'a {
        quote.complete(Ok(true))
    }
    fn settle_partial<'r,'a,'b>(&mut self, _: &mut (), check: OriginalWorkPartialCheck<'r>) -> OriginalWorkPartialCheckComplete<'r> where 'b: 'a {
        check.complete(Ok(()))
    }
    fn recover_partial<'r,'a,'b>(&mut self, _: &mut (), check: OriginalWorkPartialCheck<'r>) -> OriginalWorkPartialCheckComplete<'r> where 'b: 'a {
        check.pending()
    }
}

fn grant() -> Grant {
    Grant { maximum_items: 1, maximum_capacity_bytes: 1024 * 1024,
        maximum_copy_bytes: 128, maximum_depth: 1024, maximum_release_bytes: 1024 * 1024 }
}

fn funded<T>(grant: Grant, f: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> T) -> T {
    let mut authority = OriginalWorkAuthority::new(Work);
    authority.with_loan(&mut (), |loan| {
        let mut loan = loan.reborrow();
        let mut work = OriginalWork::empty();
        work.recipient = Some(&mut loan);
        let (mut action, mut receipt) = (None, None);
        assert_eq!(work.prepare_for_action(1, &mut action).unwrap(), Some(true));
        work.issue_performed_for_action(action, &mut receipt).unwrap();
        let mut marker = None;
        let (result, spent) = {
            let mut effect = RetainedAllocationEffect::new(grant, 1, &mut marker, receipt.as_mut().unwrap());
            let result = f(&mut effect);
            (result, effect.performed_work())
        };
        work.claim_performed_for_action(action, &mut receipt).unwrap();
        if spent == 0 { work.cancel_for_action(&mut action).unwrap(); }
        else {
            let (mut partial, mut proof) = (None, None);
            assert!(work.settle_partial_for_action_into(action, &mut partial).unwrap());
            assert!(work.commit_for_action(spent, action, &mut proof).unwrap());
            work.finish_for_action(&mut proof, &mut action).unwrap();
        }
        result
    }).unwrap()
}

fn offset_box(body: &mut Body, size: f64, offset: Vec3) -> SolidId {
    let mut stock = Body::new();
    let id = make_box(&mut stock, size, size, size, &mut OpRecorder::new()).unwrap();
    for id in stock.vertices.ids().collect::<Vec<_>>() {
        let vertex = stock.vertices.get_mut(id).unwrap();
        vertex.position = vertex.position + offset;
    }
    for id in stock.curves3.ids().collect::<Vec<_>>() {
        let Curve3::Line { origin, .. } = stock.curves3.get_mut(id).unwrap() else { panic!("box edge"); };
        *origin = *origin + offset;
    }
    for id in stock.surfaces.ids().collect::<Vec<_>>() {
        let Surface::Plane { frame } = stock.surfaces.get_mut(id).unwrap() else { panic!("box surface"); };
        frame.origin = frame.origin + offset;
    }
    body.merge(&stock).solids[&id]
}

fn cavity() -> (BrepRegistry, String) {
    let mut body = Body::new();
    let stock = offset_box(&mut body, 6., Vec3::ZERO);
    for (size, origin) in [(1., Vec3::new(1., 1., 0.5)), (2., Vec3::new(2., 2., 2.))] {
        let void = offset_box(&mut body, size, origin);
        let shell = body.solids.get(void).unwrap().outer;
        for id in body.shells.get(shell).unwrap().faces.clone() {
            body.faces.get_mut(id).unwrap().flipped = true;
        }
        body.solids.get_mut(stock).unwrap().inners.push(shell);
    }
    let mut registry = BrepRegistry::new();
    let handles = registry.merge_representation(body);
    let handle = handles.iter().find(|handle| {
        let id = registry.solid_id(handle).unwrap();
        registry.representation().solids.get(id).unwrap().inners.len() == 2
    }).unwrap().as_str().to_owned();
    (registry, handle)
}

fn counts(registry: &BrepRegistry) -> [usize;9] {
    let b = registry.representation();
    [b.vertices.len(), b.edges.len(), b.loops.len(), b.coedges.len(), b.faces.len(),
        b.surfaces.len(), b.curves3.len(), b.curves2.len(), registry.live_handles().len()]
}

fn section(registry: &mut BrepRegistry, source: &str, normal: f64) -> (PolygonFacePublication, SectionPolygon) {
    let SectionPlan::Polygon(face, outer) = registry.plan_section(source, [0.,0.,3.], [0.,0.,normal]).unwrap()
        else { panic!("section cross material and one void"); };
    (face, outer)
}

fn step(registry: &mut BrepRegistry, plan: &mut PolygonFacePublication, outer: &SectionPolygon, grant: Grant) {
    let point = plan.next_point_index().and_then(|index| outer.point(index));
    let progress = funded(grant, |effect| registry.step_polygon_face_topology(plan, None, point, effect)).unwrap();
    assert!(progress.fits(grant));
}

fn close(registry: &mut BrepRegistry, plan: &mut PolygonFacePublication) {
    for _ in 0..512 {
        if plan.terminal_is_empty() { return; }
        funded(grant(), |effect| registry.close_polygon_face(plan, effect)).unwrap();
    }
    panic!("polygon face retirement did not terminate");
}

#[test]
fn cancelling_after_the_void_loop_is_reserved_removes_private_topology() {
    let (mut registry, source) = cavity();
    let before = counts(&registry);
    let (mut plan, outer) = section(&mut registry, &source, 1.);
    step(&mut registry, &mut plan, &outer, Grant { maximum_items: 0, ..grant() });
    assert_eq!(counts(&registry), before);
    for _ in 0..512 {
        step(&mut registry, &mut plan, &outer, grant());
        if counts(&registry)[2] == before[2] + 2 { break; }
    }
    assert_eq!(counts(&registry)[2], before[2] + 2);
    assert_eq!(counts(&registry)[8], before[8]);
    close(&mut registry, &mut plan);
    assert_eq!(counts(&registry), before);
}

#[test]
fn section_publishes_the_clockwise_inner_loop_with_the_exact_remaining_face_area() {
    let (mut registry, source) = cavity();
    for sign in [-1., 1.] {
        let (mut plan, outer) = section(&mut registry, &source, sign);
        for _ in 0..512 {
            if plan.topology_ready() { break; }
            step(&mut registry, &mut plan, &outer, grant());
        }
        assert!(plan.topology_ready());
        while registry.polygon_face_demand(&plan, 0).unwrap().capacity_bytes != 0 {
            registry.reserve_polygon_face(&plan, grant()).unwrap();
        }
        let key = plan.handle().to_owned();
        registry.publish_polygon_face(&mut plan, key.clone(), grant()).unwrap();
        close(&mut registry, &mut plan);
        let Entity::Face(id) = *registry.original_query_entity(&key).unwrap() else { panic!("section face"); };
        let body = registry.representation();
        let face = body.faces.get(id).unwrap();
        let Surface::Plane { frame } = body.surfaces.get(face.surface).unwrap() else { panic!("section plane"); };
        assert!((frame.z.z - sign).abs() < 1e-12);
        assert_eq!(face.inners.len(), 1);
        let loop_id = face.inners[0];
        let first = body.loops.get(loop_id).unwrap().first;
        let (mut current, mut twice_area) = (first, 0.);
        for i in 0..4 {
            let edge_use = body.coedges.get(current).unwrap();
            let edge = body.edges.get(edge_use.edge).unwrap();
            let a = body.vertices.get(edge.v0).unwrap().position;
            let b = body.vertices.get(edge.v1).unwrap().position;
            assert!(edge_use.forward);
            assert_eq!(edge_use.loop_id, loop_id);
            assert_eq!(body.coedges.get(edge_use.next).unwrap().prev, current);
            let pcurve = body.curves2.get(edge_use.pcurve.unwrap()).unwrap();
            let domain = edge_use.prange;
            for (point, parameter) in [(a, domain.0), (b, domain.1)] {
                let uv = pcurve.eval(parameter);
                let local = frame.to_local(point);
                assert!((uv.x - local.x).abs() < 1e-12 && (uv.y - local.y).abs() < 1e-12);
            }
            twice_area += a.x * b.y - a.y * b.x;
            current = edge_use.next;
            assert_eq!(current == first, i == 3);
        }
        assert!((twice_area * sign + 8.).abs() < 1e-12);
        let mut measure = PlanarMeasure::new(PlanarMeasureSource::Face(id), true);
        let area = (0..128).find_map(|_| measure.step(body).unwrap()).unwrap();
        assert!((area - 32.).abs() < 1e-12, "{area}");
    }
}
