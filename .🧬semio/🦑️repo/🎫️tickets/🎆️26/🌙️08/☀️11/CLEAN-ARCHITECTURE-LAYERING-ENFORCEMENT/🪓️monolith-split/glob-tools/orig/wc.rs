//! 🎚️ Exact-window persisted-local configuration with independent event-stream authority.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as pack;

use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use std::any::Any;
use crate::window_mutation::ErasedWindowMutationValue;
use semio_framework_value::{FactoryBoxedValue,FactoryBoxedPublication,retirement::controlled::ControlledRetirement};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[path="🧬️preparation/🔁️replace/🦀️.rs"]
mod replace_edit;
pub use replace_edit::{WindowConfigReplaceCursor,WindowConfigReplaceEdit,WindowConfigReplaceMutation};
#[path="🧬️preparation/✏️apply/🦀️.rs"]
mod apply_edit;
pub use apply_edit::{WindowConfigApplyCursor,WindowConfigApplyEdit,WindowConfigApplyMutation};
#[cfg(test)]
#[path="🧬️preparation/✏️apply/🧪️tests/🦀️.rs"]
mod apply_edit_tests;

#[path = "📥️retained/🦀️.rs"]
pub mod retained;
#[path = "🗂️registry/🦀️.rs"]
pub mod registry;
#[path = "📬️mutation/🦀️.rs"]
pub mod window_mutation;
use registry::WindowRegistry;
pub use retained::{WindowConfigPackLoad, WindowConfigPackLoadDiagnostic, WindowConfigPackLoadPhase, WindowConfigPackLoadProgress, WindowConfigPackLoadStep};

#[path = "📐️owner-contract-and-mutation/🦀️.rs"]
mod owner_contract_and_mutation;
pub use owner_contract_and_mutation::*;

#[path = "🔏️pack-authority-and-publication/🦀️.rs"]
mod pack_authority_and_publication;
pub use pack_authority_and_publication::*;

#[path = "🗄️store-owners/🦀️.rs"]
mod store_owners;
pub use store_owners::*;

#[path = "🧭️owner-registry/🦀️.rs"]
mod owner_registry;
pub use owner_registry::*;
