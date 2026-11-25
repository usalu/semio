//! 🗃️ Static NDP rows of din16798 generated from `🔣️.json` by the ticket codegen; sorted by id. Edit the JSON, not this file.

use semio_s_plugin_norm_ndp_model::NdpValue::*;
use semio_s_plugin_norm_ndp_model::{c, NdpEntry};

const S0: &str = "EN 16798-1";
const S1: &str = "m2/person";

/// 📊️ Registered NDPs and table cells of din16798.
pub const ROWS: &[NdpEntry] = &[
    c("ndp.din16798-1.occupancy.classroom", S0, "Annex B", "typical floor area per person, classroom", Number(2.0), S1, S0),
    c("ndp.din16798-1.occupancy.meeting", S0, "Annex B", "typical floor area per person, meeting", Number(2.0), S1, S0),
    c("ndp.din16798-1.occupancy.office", S0, "Annex B", "typical floor area per person, office", Number(10.0), S1, S0),
    c("ndp.din16798-1.occupancy.retail", S0, "Annex B", "typical floor area per person, retail", Number(7.0), S1, S0),
];
