mod tests {
    use super::*;
    use crate::editor::animate::engine::scene::sobject::VSobject;
    use geometry::BezPath;

    #[test]
    fn value_tracker_mutates() {
        let t = ValueTracker::new(1.0);
        t.increment(2.0);
        assert!((t.get() - 3.0).abs() < 1e-9);
    }

    #[test]
    fn updater_runs_on_object() {
        let mut v: Sobjects = VSobject::new().into();
        let flag = Arc::new(Mutex::new(false));
        let f = Arc::clone(&flag);
        add_updater(
            &mut v,
            Updater::new("mark", move |_o, _dt| {
                *f.lock().unwrap() = true;
            }),
        );
        run_updaters(&mut v, 1.0 / 60.0);
        assert!(*flag.lock().unwrap());
    }

    #[test]
    fn inactive_updater_does_not_invoke_callback() {
        let mut v: Sobjects = VSobject::new().into();
        let flag = Arc::new(Mutex::new(false));
        let f = Arc::clone(&flag);
        let mut u = Updater::new("mark", move |_o, _dt| {
            *f.lock().unwrap() = true;
        });
        u.active = false;
        u.invoke(&mut v, 1.0 / 60.0);
        assert!(!*flag.lock().unwrap());
    }

    #[test]
    fn always_helper_attaches_updater_that_runs() {
        let mut v: Sobjects = VSobject::new().into();
        let flag = Arc::new(Mutex::new(false));
        let f = Arc::clone(&flag);
        always(&mut v, "always-mark", move |_o, _dt| {
            *f.lock().unwrap() = true;
        });
        assert_eq!(v.updaters().len(), 1);
        run_updaters(&mut v, 1.0 / 60.0);
        assert!(*flag.lock().unwrap());
    }

    #[test]
    fn f_always_helper_reads_tracker_and_runs() {
        let mut v: Sobjects = VSobject::new().into();
        let tracker = ValueTracker::new(2.0);
        let flag: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));
        let f = Arc::clone(&flag);
        f_always(&mut v, &tracker, "f-always-mark", move |_o, _dt| {
            *f.lock().unwrap() = 1.0;
        });
        tracker.set(9.0);
        run_updaters(&mut v, 1.0 / 60.0);
        assert!((*flag.lock().unwrap() - 1.0_f64).abs() < 1e-9);
    }

    #[test]
    fn always_redraw_rebuilds_paths_from_factory() {
        let mut v: Sobjects = VSobject::new().into();
        always_redraw(&mut v, "redraw", || {
            let mut fresh = VSobject::new();
            fresh.paths.push(BezPath::new());
            fresh.into()
        });
        assert!(v.paths().is_empty());
        run_updaters(&mut v, 1.0 / 60.0);
        assert_eq!(v.paths().len(), 1);
    }

    #[test]
    fn run_updaters_recurses_into_group_children() {
        let mut child: Sobjects = VSobject::new().into();
        let flag = Arc::new(Mutex::new(false));
        let f = Arc::clone(&flag);
        add_updater(
            &mut child,
            Updater::new("child-mark", move |_o, _dt| {
                *f.lock().unwrap() = true;
            }),
        );
        let mut group: Sobjects = crate::editor::animate::engine::scene::sobject::Group::new(vec![child]).into();
        run_updaters(&mut group, 1.0 / 60.0);
        assert!(*flag.lock().unwrap());
    }
}
