//! 🗺️ Patch round-trip tests of the context registers: site, organizational, service, infrastructure, information, communication, wayfinding, schedule, flexibility, growth, sustainability, resilience, cost and delivery records plus risks and conflicts.

#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::kernel::*;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::registers::kernel::*;
#[allow(unused_imports)]
use protocol::Identified;
#[allow(unused_imports)]
use protocol::list_delta::RowPatch;
use crate::standards::v1::subsets::any::schema::registers::context_requirements::*;

macro_rules! patch_through_program {
    ($item:ident, $patch:expr, $field:ident, $delta:ty, $id:expr) => {{
        let mut program = crate::standards::v1::subsets::any::schema::snapshot::empty_plugin();
        program.$field.push($item.clone());
        let diff = crate::standards::v1::subsets::any::schema::diff::ProgramDiff { $field: Some(<$delta>::modification($id, $patch.clone())), ..Default::default() };
        $item = protocol::apply_diff_cold(&diff, &program).expect("the patch commits through the central applier").$field.remove(0);
    }};
}

include!("🏞️site-and-needs/🦀️.rs");
include!("🧭️flow-and-growth/🦀️.rs");
include!("⚖️risks-and-costs/🦀️.rs");
