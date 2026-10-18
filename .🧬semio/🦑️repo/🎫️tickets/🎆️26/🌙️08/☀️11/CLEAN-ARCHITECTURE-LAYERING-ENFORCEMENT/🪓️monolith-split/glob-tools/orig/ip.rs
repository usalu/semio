//! 🧠️ Repository-owned WebAssembly core interpreter for the Semio plugin sandbox.
//!
//! The interpreter is deliberately execution-engine neutral: component canonical-ABI lowering is
//! an adapter above this module, while every guest core instruction is decoded, fuelled, cancelled,
//! and checkpointed here. Browser hosts keep using the browser's WebAssembly engine.

use std::collections::{BTreeMap, BTreeSet};

/// 🪪️ The interpreter's authored source: the half of the owned engine identity that names every instruction a guest executes.
pub const INTERPRETER_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("🧱️core-types-and-module-model/🦀️.rs"),
    include_str!("🧩️component-artifact/🦀️.rs"),
    include_str!("🔌️owned-semio-boundary/🦀️.rs"),
    include_str!("🔎️binary-decoder-and-module-parser/🦀️.rs"),
    include_str!("🧭️instruction-shape/🦀️.rs"),
    include_str!("🏛️instance-state/🦀️.rs"),
    include_str!("⚙️instruction-execution/🦀️.rs"),
    include_str!("💾️memory-and-bulk-execution/🦀️.rs"),
    include_str!("🧮️numeric-execution/🦀️.rs"),
    include_str!("🔢️numeric-semantics/🦀️.rs"),
    include_str!("📸️checkpoint-codec/🦀️.rs"),
);

/// 🧮️ The heap witness the crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.
#[cfg(test)]
#[global_allocator]
static INTERPRETER_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[path = "🧱️core-types-and-module-model/🦀️.rs"]
mod core_types_and_module_model;
pub use core_types_and_module_model::*;

#[path = "🧩️component-artifact/🦀️.rs"]
mod component_artifact;
pub use component_artifact::*;

#[path = "🔌️owned-semio-boundary/🦀️.rs"]
mod owned_semio_boundary;
pub use owned_semio_boundary::*;

#[path = "🔎️binary-decoder-and-module-parser/🦀️.rs"]
mod binary_decoder_and_module_parser;
use binary_decoder_and_module_parser::*;

#[path = "🧭️instruction-shape/🦀️.rs"]
mod instruction_shape;
use instruction_shape::*;

#[path = "🏛️instance-state/🦀️.rs"]
mod instance_state;
pub use instance_state::*;

#[path = "⚙️instruction-execution/🦀️.rs"]
mod instruction_execution;
use instruction_execution::*;

#[path = "💾️memory-and-bulk-execution/🦀️.rs"]
mod memory_and_bulk_execution;

#[path = "🧮️numeric-execution/🦀️.rs"]
mod numeric_execution;
use numeric_execution::*;

#[path = "🔢️numeric-semantics/🦀️.rs"]
mod numeric_semantics;
use numeric_semantics::*;

#[path = "📸️checkpoint-codec/🦀️.rs"]
mod checkpoint_codec;

#[path="🏗️instance/🦀️.rs"]
mod original_instance_opening;
pub use original_instance_opening::{CoreInstanceOpening,CoreOriginalInvocationCall,CoreOriginalInvocationStatus};

//#region 💾️CheckpointCodec

#[cfg(test)]
#[path="🚪️io/📤️checkpoint/🧪️tests/👓️borrowed/🦀️.rs"]
mod borrowed_actor_checkpoint_tests;

//#endregion 💾️CheckpointCodec

//#region 🧰️Utilities

fn pages_to_bytes(pages: u64) -> Result<usize, CoreError> {
    let bytes = pages.checked_mul(WASM_PAGE_BYTES as u64).ok_or_else(|| CoreError::Validation("memory page count overflows bytes".into()))?;
    usize::try_from(bytes).map_err(|_| CoreError::Validation("memory size exceeds host usize".into()))
}

fn checked_end(start: usize, length: usize, bound: usize, subject: &str) -> Result<usize, CoreError> {
    let end = start as u128 + length as u128;
    if end > bound as u128 {
        Err(CoreError::Trap(format!("{subject} is out of bounds: start={start} length={length} end={end} bound={bound}")))
    } else {
        Ok(end as usize)
    }
}

fn stable_fingerprint(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

//#endregion 🧰️Utilities

//#region 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
