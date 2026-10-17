//! 🔁️ Architect operations and requirement registers: processes, flows, access rules, equipment, resources and the operational, storage, environmental, human factor, accessibility, privacy, safety, security and regulatory requirements; the shared enums live in `kernel` and the patch macros in [`super`].

use crate::standards::v1::subsets::any::schema::kernel::*;
use crate::standards::v1::subsets::any::schema::registers::kernel::*;
use protocol::Identified;
use protocol::list_delta::RowPatch;

#[path = "⚙️process-flow-and-resources/🦀️.rs"]
mod process_flow_and_resources;
pub use process_flow_and_resources::*;

#[path = "🛡️human-factor-and-compliance/🦀️.rs"]
mod human_factor_and_compliance;
pub use human_factor_and_compliance::*;


