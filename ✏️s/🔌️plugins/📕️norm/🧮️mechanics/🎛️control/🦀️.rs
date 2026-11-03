//! 🎛️ Cooperative progress and cancellation shared by every long-running kernel loop.

use crate::{MechanicsError, MechanicsResult, Progress};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Copy, Default)]
pub struct Control<'a> {
    progress: Option<&'a dyn Fn(&Progress)>,
    cancel: Option<&'a AtomicBool>,
}

impl<'a> Control<'a> {
    /// 🕊️ A control that reports nothing and never cancels.
    pub fn none() -> Self {
        Self::default()
    }

    /// 📣️ Attach a progress sink invoked with a stage name and a fraction in `0..=1`.
    pub fn with_progress(mut self, sink: &'a dyn Fn(&Progress)) -> Self {
        self.progress = Some(sink);
        self
    }

    /// 🛑️ Attach a flag that cancels the operation at the next checkpoint once set.
    pub fn with_cancel(mut self, flag: &'a AtomicBool) -> Self {
        self.cancel = Some(flag);
        self
    }

    pub fn report(&self, stage: &str, fraction: f64) {
        if let Some(sink) = self.progress {
            sink(&Progress { stage: stage.to_string(), fraction: fraction.clamp(0.0, 1.0) });
        }
    }

    pub fn check(&self) -> MechanicsResult<()> {
        match self.cancel {
            Some(flag) if flag.load(Ordering::Relaxed) => Err(MechanicsError::Cancelled),
            _ => Ok(()),
        }
    }
}
