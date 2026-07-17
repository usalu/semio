//! 🔄 Runtime updaters, value trackers, and always-redraw helpers.

use crate::sobject::Sobject;
use std::sync::{Arc, Mutex};

/// 🎚️ Scalar animated parameter with get/set hooks.
#[derive(Clone)]
pub struct ValueTracker {
    pub value: Arc<Mutex<f64>>,
}

impl ValueTracker {
    pub fn new(value: f64) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
        }
    }

    pub fn get(&self) -> f64 {
        *self.value.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn set(&self, value: f64) {
        *self.value.lock().unwrap_or_else(|e| e.into_inner()) = value;
    }

    pub fn increment(&self, delta: f64) {
        let mut v = self.value.lock().unwrap_or_else(|e| e.into_inner());
        *v += delta;
    }
}

/// 🔁 Per-frame mutation callback attached to an Sobject.
#[derive(Clone)]
pub struct Updater {
    pub id: u64,
    pub name: String,
    pub active: bool,
    pub dt_scale: f64,
    callback: Arc<dyn Fn(&mut dyn Sobject, f64) + Send + Sync>,
}

static UPDATER_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

impl Updater {
    pub fn new<F>(name: impl Into<String>, callback: F) -> Self
    where
        F: Fn(&mut dyn Sobject, f64) + Send + Sync + 'static,
    {
        Self {
            id: UPDATER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.into(),
            active: true,
            dt_scale: 1.0,
            callback: Arc::new(callback),
        }
    }

    pub fn invoke(&self, target: &mut dyn Sobject, dt: f64) {
        if self.active {
            (self.callback)(target, dt * self.dt_scale);
        }
    }
}

/// ➕ Attach an updater to an Sobject.
pub fn add_updater(target: &mut dyn Sobject, updater: Updater) {
    target.updaters_mut().push(updater);
}

/// ♾️ Attach an updater that runs every frame.
pub fn always<F>(target: &mut dyn Sobject, name: impl Into<String>, f: F)
where
    F: Fn(&mut dyn Sobject, f64) + Send + Sync + 'static,
{
    add_updater(target, Updater::new(name, f));
}

/// 🎯 Attach an updater driven by a ValueTracker.
pub fn f_always<F>(target: &mut dyn Sobject, tracker: ValueTracker, name: impl Into<String>, f: F)
where
    F: Fn(&mut dyn Sobject, f64) + Send + Sync + 'static,
{
    let t = tracker.clone();
    add_updater(
        target,
        Updater::new(name, move |obj, dt| {
            let _ = t.get();
            f(obj, dt);
        }),
    );
}

/// 🔃 Rebuild an Sobject every frame from a factory closure.
pub fn always_redraw<F>(target: &mut dyn Sobject, name: impl Into<String>, factory: F)
where
    F: Fn() -> Box<dyn Sobject> + Send + Sync + 'static,
{
    let factory = Arc::new(factory);
    add_updater(
        target,
        Updater::new(name, move |obj, _dt| {
            let fresh = factory();
            if let Some(v) = obj.as_any_mut().downcast_mut::<crate::sobject::VSobject>() {
                if let Some(fv) = fresh.as_any().downcast_ref::<crate::sobject::VSobject>() {
                    v.paths = fv.paths.clone();
                    v.style = fv.style.clone();
                    v.transform = fv.transform;
                }
            }
        }),
    );
}

/// 🏃 Run all updaters on a scene object tree.
pub fn run_updaters(target: &mut dyn Sobject, dt: f64) {
    let updaters: Vec<Updater> = target.updaters().to_vec();
    for u in updaters {
        u.invoke(target, dt);
    }
    for child in target.children_mut() {
        run_updaters(child, dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sobject::VSobject;

    #[test]
    fn value_tracker_mutates() {
        let t = ValueTracker::new(1.0);
        t.increment(2.0);
        assert!((t.get() - 3.0).abs() < 1e-9);
    }

    #[test]
    fn updater_runs_on_object() {
        let mut v = VSobject::new();
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
}
