#![cfg(test)]
pub use semio_framework_value as value;
#[path="../../../../../../../../🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:test_allocation::RequestedAllocator=test_allocation::RequestedAllocator;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs"]
pub mod identity_control;
pub mod os_vcs{pub mod io{pub mod binary{pub mod entity_identity{pub use crate::identity_control as control;}}}}
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🦀️.rs"]
pub mod operation_authority;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/🦀️.rs"]
mod operation;

mod actor_bindings{wasmtime::component::bindgen!({world:"actor",path:"../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema"});}

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs"]
mod interpreter;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/📤️result/🧪️tests/🦀️.rs"]
mod owned_result_tests;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📤️checkpoint/🧪️tests/🦀️.rs"]
mod controlled_checkpoint_tests;
