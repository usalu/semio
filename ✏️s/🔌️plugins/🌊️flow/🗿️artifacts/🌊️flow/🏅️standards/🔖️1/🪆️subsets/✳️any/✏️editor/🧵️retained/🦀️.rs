//! 🧵️ Flow-owned retained command frontiers: scanned id lists close through the original controlled retirement under supplied grants.

use semio_framework_job::InteractiveJobCloseStep as Step;
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::controlled::ControlledRetirement, ValueError};

//#region 🧹️Retirement
#[derive(Default)]
pub struct Retirement {
    pending: Vec<Vec<String>>,
    owner: Option<ControlledRetirement<Vec<Vec<String>>>>,
}

impl Drop for Retirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.is_empty(), "Flow app retirement must reach terminal-empty before drop");
        }
    }
}

impl Retirement {
    pub fn push(&mut self, values: Vec<String>) {
        self.pending.push(values);
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.owner.is_none()
    }

    pub fn step(&mut self, grant: RetainedCloneGrant) -> Result<Step, semio_framework_value::ValueError> {
        if let Some(owner) = self.owner.as_mut() {
            let step = owner.step(grant);
            if owner.terminal_is_empty() {
                self.owner = None;
            }
            return Ok(match step {
                Ok(RetainedCloneStep::Complete(progress)) => Step::Complete { progress },
                Ok(RetainedCloneStep::Progress(progress)) => Step::Pending { progress },
                Err(error) => return Err(error),
            });
        }
        if self.pending.is_empty() {
            return Ok(Step::Complete { progress: RetainedCloneProgress::default() });
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(Step::Pending { progress: RetainedCloneProgress::default() });
        }
        Ok(match ControlledRetirement::new(std::mem::take(&mut self.pending)) {
            Ok(owner) => {
                self.owner = Some(owner);
                Step::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }
            }
            Err((error, pending)) => {
                self.pending = pending;
                return Err(error)
            }
        })
    }

    pub fn copy_byte_demand(&self) -> Result<usize, ValueError> {
        self.owner.as_ref().map_or(Ok(0), |owner| owner.next_copy_byte_demand())
    }

    pub fn capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        self.owner.as_ref().map_or(Ok(0), |owner| owner.next_capacity_byte_demand(maximum_copy_bytes))
    }

    pub fn release_byte_demand(&self) -> Result<usize, ValueError> {
        self.owner.as_ref().map_or(Ok(0), |owner| owner.next_release_byte_demand())
    }

    pub fn depth_demand(&self) -> Result<usize, ValueError> {
        self.owner.as_ref().map_or(Ok(usize::from(!self.pending.is_empty())), |owner| owner.next_depth_demand())
    }
}
//#endregion 🧹️Retirement

//#region 🧪️Tests

//#endregion 🧪️Tests
