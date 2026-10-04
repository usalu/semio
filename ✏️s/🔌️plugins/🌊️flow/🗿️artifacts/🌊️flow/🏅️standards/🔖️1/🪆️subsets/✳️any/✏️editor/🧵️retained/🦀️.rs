//! 🧵️ Flow-owned byte frontiers for retained command work: scanned id lists close through exact byte grants.

use std::collections::LinkedList;
use std::mem::ManuallyDrop;

//#region 🧹️Retirement
pub(super) enum Owner {
    Bytes(Vec<u8>),
    Strings(Vec<String>),
}

#[derive(Default)]
pub(super) struct Retirement {
    owners: ManuallyDrop<LinkedList<Owner>>,
}

impl Drop for Retirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.is_empty(), "Flow app retirement must reach terminal-empty before drop");
        }
    }
}

impl Retirement {
    pub(super) fn push(&mut self, owner: Owner) {
        self.owners.push_front(owner);
    }

    pub(super) fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    pub(super) fn step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep as Step;
        if self.is_empty() {
            return Step::Complete;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Step::Blocked;
        }
        let mut released_bytes = 0;
        match self.owners.pop_front().expect("nonempty retirement") {
            Owner::Bytes(mut bytes) => {
                released_bytes = maximum_bytes.min(bytes.len());
                bytes.truncate(bytes.len() - released_bytes);
                if !bytes.is_empty() {
                    self.push(Owner::Bytes(bytes));
                }
            }
            Owner::Strings(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(Owner::Strings(values));
                }
                if let Some(value) = next {
                    self.push(Owner::Bytes(value.into_bytes()));
                }
            }
        }
        Step::Pending { released_items: 1, released_bytes }
    }
}
//#endregion 🧹️Retirement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
