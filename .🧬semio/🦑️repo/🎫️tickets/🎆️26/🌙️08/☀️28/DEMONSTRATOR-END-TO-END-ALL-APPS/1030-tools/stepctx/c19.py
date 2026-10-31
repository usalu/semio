from rep import rep
R='/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/'
rep(R+'🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📐️geometry/💾️brep-interchange/🦀️.rs',[
("use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, spend};","use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, settle};"),
("spend(cx, if step.is_some() { 1 } else { fuel });","settle(cx, fuel, step.is_none());"),
])
G=R+'🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry'
rep(G+'/⏱️phased-job/🧪️tests/🔬️unit/🦀️.rs',[("#[test]\nfn unbounded_fuel_finishes_in_one_slice","""#[test]
fn an_unfunded_turn_does_no_work_and_a_funded_turn_does_at_most_its_fuel() {
    let ran = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let build = |ran: &Arc<std::sync::atomic::AtomicUsize>| {
        let units = (0..6).fold(Pipeline::new(kind()).import(&cube(2.0)), |pipeline, _| {
            let ran = Arc::clone(ran);
            pipeline.once(move |_| {
                ran.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Ok(())
            })
        });
        units.exported_after_import()
    };
    let mut job = build(&ran);
    assert!(matches!(step_with_fuel(&mut *job, 0), WidgetStep::Working { progress } if progress == 0.0));
    assert_eq!(ran.load(std::sync::atomic::Ordering::SeqCst), 0, "zero fuel runs no unit");
    for fuel in [1usize, 2, 3] {
        let before = ran.load(std::sync::atomic::Ordering::SeqCst);
        step_with_fuel(&mut *job, fuel);
        assert!(ran.load(std::sync::atomic::Ordering::SeqCst) - before <= fuel, "{fuel} fuel runs at most {fuel} units");
    }
}

#[test]
fn unbounded_fuel_finishes_in_one_slice""")])
