//! 🏛️ Architect program artifact — the typed register entities for all 65 feature areas (the
//! document model's row types; the `ProgramSnapshot` document that holds them lives in `🦀️.rs`).

//! 🏛️ Architectural programming register entities — typed domain model for all 65 feature areas.

use crate::standards::v1::subsets::any::schema::kernel::*;
use protocol::list_delta::RowPatch;
use protocol::Identified;

// #region 🔖️PatchHelpers
/// 🩹️ Per-field patch algebra of one register row: `commit_row` writes a patched value and `capture_row` reads the current value so the
/// patch that undoes a patch can be built (an absent `Option` field captures nothing — a patch field `Option<T>` cannot express a clear,
/// so a clear travels as a row replacement, never as a patch).
trait PatchRow<T: Clone> {
    fn commit_row(&mut self, patch: &Option<T>);
    fn capture_row(&self) -> Option<T>;
}

impl<T: Clone> PatchRow<T> for T {
    fn commit_row(&mut self, patch: &Option<T>) {
        if let Some(value) = patch {
            *self = value.clone();
        }
    }

    fn capture_row(&self) -> Option<T> {
        Some(self.clone())
    }
}

impl<T: Clone> PatchRow<T> for Option<T> {
    fn commit_row(&mut self, patch: &Option<T>) {
        if let Some(value) = patch {
            *self = Some(value.clone());
        }
    }

    fn capture_row(&self) -> Option<T> {
        self.clone()
    }
}

macro_rules! impl_identified_header {
    ($ty:ty) => {
        impl Identified<EntityId> for $ty {
            fn id(&self) -> &EntityId {
                &self.header.id
            }
        }
    };
}

/// 🧬️ Implements the framework's [`RowPatch`] for one register row: every listed field is an absolute setter, `inverse` captures the
/// current value of exactly the fields the patch carries, and `absorb` lets the later patch's fields win.
macro_rules! impl_patchable {
    ($entity:ty, $patch:ty, { $( [ $($path:ident).+ ] => $f:ident ),+ $(,)? }) => {
        impl RowPatch<$entity> for $patch {
            fn commit_into(&self, row: &mut $entity, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $( PatchRow::commit_row(&mut row$(.$path)+, &self.$f); )+
                Ok(())
            }

            fn absorb(&mut self, later: Self) {
                $( if later.$f.is_some() { self.$f = later.$f; } )+
            }

            fn inverse(&self, row: &$entity) -> Self {
                Self { $( $f: if self.$f.is_some() { PatchRow::capture_row(&row$(.$path)+) } else { None } ),+, ..Self::default() }
            }

            fn is_empty(&self) -> bool {
                true $( && self.$f.is_none() )+
            }
        }
    };
}
// #endregion

impl_patchable!(
    TraceLink,
    TraceLinkPatch,
    {
        [from_id] => from_id,
        [to_id] => to_id,
        [kind] => kind,
        [label] => label,
    }
);

#[path = "🧱️kernel/🦀️.rs"]
mod kernel;
pub use kernel::*;
#[path = "🧭️program-structure/🦀️.rs"]
mod program_structure;
pub use program_structure::*;
#[path = "🔁️operations-requirements/🦀️.rs"]
mod operations_requirements;
pub use operations_requirements::*;
#[path = "🌐️context-requirements/🦀️.rs"]
mod context_requirements;
pub use context_requirements::*;
#[path = "📋️decisions/🦀️.rs"]
mod decisions;
pub use decisions::*;
#[path = "🗃️records/🦀️.rs"]
mod records;
pub use records::*;
