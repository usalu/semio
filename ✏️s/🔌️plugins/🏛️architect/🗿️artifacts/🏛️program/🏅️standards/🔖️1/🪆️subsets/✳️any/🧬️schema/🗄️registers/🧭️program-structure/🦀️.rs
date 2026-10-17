//! 🧭️ Architect program structure registers: meta, project, stakeholders, users, activities, functions, elements, quantities, relationships and adjacencies; the shared enums live in `kernel` and the patch macros in [`super`].

use crate::standards::v1::subsets::any::schema::kernel::*;
use crate::standards::v1::subsets::any::schema::registers::kernel::*;
use protocol::Identified;
use protocol::list_delta::RowPatch;

#[path = "🏗️project-and-stakeholders/🦀️.rs"]
mod project_and_stakeholders;
pub use project_and_stakeholders::*;

#[path = "🧱️elements-and-relationships/🦀️.rs"]
mod elements_and_relationships;
pub use elements_and_relationships::*;


