//! 🎭️ Publication hostiles: the hostile lease kinds the test lanes arm and observe. See [`super`].

use super::*;

#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Generation2dPublicationHostile {
    Missing,
    WrongOperation,
    WrongGeneration,
    WrongBase,
    WrongParent,
}

#[cfg(any(test, feature = "testkit"))]
pub(super) fn generation2d_publication_hostiles() -> &'static std::sync::Mutex<[Option<Generation2dPublicationHostileLease>; GENERATION2D_PUBLICATION_SLOTS]> {
    static HOSTILES: std::sync::OnceLock<std::sync::Mutex<[Option<Generation2dPublicationHostileLease>; GENERATION2D_PUBLICATION_SLOTS]>> = std::sync::OnceLock::new();
    HOSTILES.get_or_init(|| std::sync::Mutex::new([None; GENERATION2D_PUBLICATION_SLOTS]))
}

#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Copy)]
struct Generation2dPublicationHostileLease {
    pub(super) operation: u64,
    pub(super) hostile: Generation2dPublicationHostile,
    pub(super) observed: Option<&'static str>,
}

#[cfg(any(test, feature = "testkit"))]
pub fn generation2d_arm_publication_hostile(operation: semio_framework_job::OperationId, hostile: Generation2dPublicationHostile) {
    let mut hostiles = generation2d_publication_hostiles().try_lock().expect("Generation2d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_none()).expect("Generation2d hostile publication authority has a fixed slot");
    *slot = Some(Generation2dPublicationHostileLease { operation: operation.0, hostile, observed: None });
}

#[cfg(any(test, feature = "testkit"))]
pub fn generation2d_take_publication_hostile_observed(operation: semio_framework_job::OperationId) -> Option<&'static str> {
    let mut hostiles = generation2d_publication_hostiles().try_lock().expect("Generation2d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_some_and(|value| value.operation == operation.0))?;
    slot.take()?.observed
}
