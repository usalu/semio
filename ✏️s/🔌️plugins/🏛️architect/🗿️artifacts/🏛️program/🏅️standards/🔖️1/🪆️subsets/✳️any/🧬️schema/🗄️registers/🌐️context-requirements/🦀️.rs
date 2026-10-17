//! 🌐️ Architect context registers: site, organizational, service, infrastructure, information, communication, wayfinding, schedule, flexibility, growth, sustainability, resilience, cost and delivery records plus risks and conflicts; the shared enums live in `kernel` and the patch macros in [`super`].

use crate::standards::v1::subsets::any::schema::kernel::*;
use crate::standards::v1::subsets::any::schema::registers::kernel::*;
use protocol::Identified;
use protocol::list_delta::RowPatch;

#[path = "🏢️site-and-organization-requirements/🦀️.rs"]
mod site_and_organization_requirements;
pub use site_and_organization_requirements::*;

#[path = "🗓️schedule-growth-and-cost-requirements/🦀️.rs"]
mod schedule_growth_and_cost_requirements;
pub use schedule_growth_and_cost_requirements::*;

#[path = "⚠️delivery-risk-and-conflict/🦀️.rs"]
mod delivery_risk_and_conflict;
pub use delivery_risk_and_conflict::*;


