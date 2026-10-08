use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::catalogue::catalogue;
use semio_framework_3d::brep::queries::analysis::{edge_table, face_table, ShapeScope};
use semio_framework_3d::brep::engine::ShapeRoot;

#[path = "../🧰️support/🦀️.rs"]
mod support;

fn kind() -> &'static Kind {
    catalogue().kind("brep.feature.fillet").expect("catalogued")
}

fn cube(size: f64) -> Arc<ShapeValue> {
    let mut session = KernelSession::new();
    let handle = session.brep().box_prim_sync(size, size, size).expect("box");
    Arc::new(session.export(&handle).expect("export"))
}

fn drive_all(job: Box<dyn WidgetJob>, fuel: usize) -> (usize, WidgetEvaluation) {
    support::slices(job, fuel)
}

fn volume_of(value: &GeometryValue) -> f64 {
    let GeometryValue::Shape(shape) = value else { panic!("a shape output") };
    support::kernel_measures(shape).0.expect("a volume")
}

#[test]
fn a_pipeline_imports_runs_and_exports_with_monotone_progress() {
    let shape = cube(2.0);
    let job = Pipeline::new(kind()).import(&shape).once(|work| {
        let handle = work.handle(0)?;
        let moved = work.session.brep().translate_sync(&handle, [1.0, 0.0, 0.0]).map_err(|error| kernel_fault(&error))?;
        work.groups = vec![vec![moved]];
        Ok(())
    }).exported("shape");
    let (working, evaluation) = drive_all(job, 1);
    assert!(working >= 2, "three units of one fuel need at least two Working slices, saw {working}");
    assert!(evaluation.fault.is_none());
    let GeometryValue::Shape(moved) = &evaluation.outputs["shape"] else { panic!("shape output") };
    assert!((support::measures(&support::tessellate(moved, 0.1)).min[0] - 1.0).abs() < 1e-9);
}

#[test]
fn unbounded_fuel_finishes_in_one_slice_and_equals_the_stepped_run() {
    let shape = cube(2.0);
    let build = |shape: &Arc<ShapeValue>| Pipeline::new(kind()).import(shape).operation(|work| Ok(BrepOperation::Fillet { shape: work.handle(0)?, radius: 0.25 })).exported("shape");
    let (stepped_slices, stepped) = drive_all(build(&shape), 1);
    let (whole_slices, whole) = drive_all(build(&shape), usize::MAX);
    assert_eq!(whole_slices, 0);
    assert!(stepped_slices > 3, "a fillet of twelve edges is several units, saw {stepped_slices}");
    assert_eq!(stepped, whole);
    assert!(volume_of(&whole.outputs["shape"]) < 8.0);
}

#[test]
fn cancelling_releases_the_work_and_every_later_step_reports_the_cancellation() {
    let shape = cube(2.0);
    let mut job = Pipeline::new(kind()).import(&shape).operation(|work| Ok(BrepOperation::Fillet { shape: work.handle(0)?, radius: 0.25 })).exported("shape");
    assert!(matches!(job.step(1), WidgetStep::Working { .. }));
    job.cancel();
    job.cancel();
    for _ in 0..2 {
        let WidgetStep::Done(evaluation) = job.step(4) else { panic!("a cancelled job is finished") };
        let fault = evaluation.fault.expect("cancellation is a fault");
        assert_eq!(fault.code, CANCELLED);
        assert!(fault.message.en != fault.message.de && evaluation.outputs.is_empty());
    }
}

#[test]
fn a_finished_job_keeps_answering_with_the_same_evaluation() {
    let shape = cube(2.0);
    let mut job = Pipeline::new(kind()).import(&shape).once(|work| {
        work.groups = vec![vec![work.handle(0)?]];
        Ok(())
    }).exported("shape");
    let first = loop {
        if let WidgetStep::Done(evaluation) = job.step(8) {
            break evaluation;
        }
    };
    let WidgetStep::Done(second) = job.step(1) else { panic!("finished") };
    assert_eq!(first, second);
    job.cancel();
    let WidgetStep::Done(third) = job.step(1) else { panic!("finished") };
    assert_eq!(first, third, "cancelling a finished job changes nothing");
}

#[test]
fn a_panic_inside_the_kernel_becomes_a_localized_kernel_fault() {
    let shape = cube(2.0);
    let job = Pipeline::new(kind()).import(&shape).once(|_| -> Result<(), WidgetFault> { panic!("[DEBUG] simulated kernel panic") }).exported("shape");
    let (_, evaluation) = drive_all(job, usize::MAX);
    let fault = evaluation.fault.expect("fault");
    assert_eq!(fault.code, KERNEL);
    assert!(fault.message.en != fault.message.de && evaluation.outputs.is_empty());
}

#[test]
fn a_step_without_its_import_is_a_pipeline_fault_not_a_panic() {
    let job = Pipeline::new(kind()).once(|work| work.handle(0).map(|_| ())).exported("shape");
    let (_, evaluation) = drive_all(job, usize::MAX);
    assert_eq!(evaluation.fault.expect("fault").code, PIPELINE);
}

fn labels(shape: &ShapeValue, edges: bool) -> Vec<u64> {
    let ShapeRoot::Solid(id) = shape.root else { panic!("solid") };
    if edges {
        edge_table(&shape.body, &ShapeScope::solid(id)).expect("edges").iter().map(|row| row.label).collect()
    } else {
        face_table(&shape.body, &ShapeScope::solid(id), 1e-6).expect("faces").iter().map(|row| row.label).collect()
    }
}

#[test]
fn selections_resolve_through_the_labels_of_the_value_even_in_a_populated_session() {
    let shape = cube(2.0);
    let wanted = labels(&shape, true)[0];
    let selection = SelectionValue { component: SelectionKind::Edge, ids: vec![wanted, wanted] };
    let mut work = Work::new(vec![shape.clone(), shape.clone()]);
    work.import(0).expect("first");
    work.import(1).expect("second");
    let picked = work.pick(1, &selection, SelectionKind::Edge, "edges").expect("resolves");
    assert_eq!(picked.len(), 1, "duplicate labels collapse");
    let first_edge = work.pick(0, &selection, SelectionKind::Edge, "edges").expect("resolves in the first import");
    assert_ne!(first_edge, picked, "the two imports hold distinct copies of the edge");
    let kernel_edge = work.session.export(&picked[0]).expect("exported edge");
    let ShapeRoot::Edge(id) = kernel_edge.root else { panic!("an edge") };
    let own = edge_table(&shape.body, &ShapeScope::solid(match shape.root { ShapeRoot::Solid(id) => id, _ => panic!("solid") })).expect("edges");
    let row = own.iter().find(|row| row.label == wanted).expect("row");
    let moved = edge_table(&kernel_edge.body, &ShapeScope::edge(id)).expect("edge row");
    assert_eq!((moved[0].start, moved[0].end), (row.start, row.end), "the second import's edge is the same edge of the value");
}

#[test]
fn a_stale_label_a_wrong_component_and_a_foreign_kind_are_localized_faults_at_the_port() {
    let shape = cube(2.0);
    let mut work = Work::new(vec![shape.clone()]);
    work.import(0).expect("import");
    let stale = work.pick(0, &SelectionValue { component: SelectionKind::Edge, ids: vec![u64::MAX - 3] }, SelectionKind::Edge, "edges").expect_err("stale");
    assert_eq!((stale.code.as_str(), stale.port.as_deref()), (SELECTION_STALE, Some("edges")));
    assert!(stale.message.en != stale.message.de);
    let face_label = labels(&shape, false)[0];
    let wrong = work.pick(0, &SelectionValue { component: SelectionKind::Face, ids: vec![face_label] }, SelectionKind::Edge, "edges").expect_err("component");
    assert_eq!(wrong.code, SELECTION_COMPONENT);
    let edge_as_face = work.pick(0, &SelectionValue { component: SelectionKind::Face, ids: vec![labels(&shape, true)[0]] }, SelectionKind::Face, "faces").expect_err("an edge label is not a face");
    assert_eq!(edge_as_face.code, SELECTION_STALE);
}

#[test]
fn several_operations_run_in_order_and_collect_their_results_in_group_zero() {
    let shape = cube(2.0);
    let job = Pipeline::new(kind()).import(&shape).operations(|work| {
        let handle = work.handle(0)?;
        Ok(vec![BrepOperation::Fillet { shape: handle.clone(), radius: 0.1 }, BrepOperation::Chamfer { shape: handle, distance: 0.1 }])
    }).export_groups().finish(|work| Ok(outputs([("both", GeometryValue::List(work.values[0].clone()))])));
    let (working, evaluation) = drive_all(job, 1);
    assert!(working > 2);
    let GeometryValue::List(items) = &evaluation.outputs["both"] else { panic!("list") };
    assert_eq!(items.len(), 2);
    assert!(items.iter().all(|item| volume_of(item) < 8.0));
}
