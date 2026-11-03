//! ⏱️ Lane clock: the process-wide 30 Hz presence gate and the T1/T2 evaluation debounce of the open gesture, so the reducer and the paint agree on when a preview report is due. See [`super`].

use super::{EvalScheduler, EvalTier, NormPresence, PresencePublisher};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// ⏱️ Wall-clock milliseconds; only differences are ever used.
pub fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as u64)
}

/// ⏱️ The gates of one editor process: presence publication and preview-report scheduling.
#[derive(Clone, Debug, Default)]
pub struct LaneClock {
    pub presence: PresencePublisher,
    pub scheduler: EvalScheduler,
    reports: HashMap<&'static str, Arc<semio_s_plugin_norm_compliance::CheckReport>>,
}

impl LaneClock {
    /// 📡️ True when `local` may be published at `now_ms`: an open drag at most every 34 ms and only when changed, a cleared drag at once so peers drop the ghost.
    pub fn admit_presence(&mut self, local: &NormPresence, now_ms: u64) -> bool {
        if local.drag.is_none() {
            self.presence = PresencePublisher::default();
            return true;
        }
        self.presence.offer(local.clone());
        self.presence.poll(now_ms).is_some()
    }

    /// 🎬️ A gesture tick: the preview report becomes due after the debounce.
    pub fn tick(&mut self, now_ms: u64) {
        self.scheduler.on_tick(now_ms);
    }

    /// ✅️ A commit: the report is due at once and the cached preview report is dropped.
    pub fn commit(&mut self) {
        self.scheduler.on_commit();
        self.reports.clear();
    }

    /// 🧮️ True when the fast tier (T1, 60 ms after the last tick, or a commit) is due; consumes it. A slower tier (T2, 250 ms) fires the same way later.
    pub fn report_due(&mut self, now_ms: u64) -> bool {
        self.scheduler.poll(now_ms).contains(&EvalTier::T1)
    }

    /// 🔗️ The cached preview report of `family` as the handle that was stored: a tick between two due evaluations copies nothing.
    pub fn cached_report(&self, family: &'static str) -> Option<Arc<semio_s_plugin_norm_compliance::CheckReport>> {
        self.reports.get(family).cloned()
    }

    pub fn store_report(&mut self, family: &'static str, report: Arc<semio_s_plugin_norm_compliance::CheckReport>) {
        self.reports.insert(family, report);
    }
}

/// ⏱️ Runs `f` on the process clock.
pub fn with_clock<R>(f: impl FnOnce(&mut LaneClock) -> R) -> R {
    static CLOCK: OnceLock<Mutex<LaneClock>> = OnceLock::new();
    let mut guard = CLOCK.get_or_init(|| Mutex::new(LaneClock::default())).lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut guard)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
