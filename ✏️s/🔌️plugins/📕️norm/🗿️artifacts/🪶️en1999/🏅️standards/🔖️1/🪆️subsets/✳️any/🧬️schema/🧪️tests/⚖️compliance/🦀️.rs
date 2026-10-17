//! EN 1999 numeric compliance tests — hand-derived worked examples + DoD gates.

use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_io::text::snapshot::encode_en1999_dsl;
use semio_s_artifact_norm_en1999_io::text::snapshot::decode_en1999_dsl;
use semio_s_artifact_norm_en1999_checks::{evaluate_structure, na_de, part_1_1, part_1_2, part_1_3};
use std::path::PathBuf;
use std::process::Command;

#[path = "🔬️resistance-worked-examples/🦀️.rs"]
mod resistance_worked_examples;

#[path = "✅️compliance-gates/🦀️.rs"]
mod compliance_gates;

#[path = "🧬️facet-and-asset-parity/🦀️.rs"]
mod facet_and_asset_parity;

#[path = "🔎️leaf-influence-and-material-laws/🦀️.rs"]
mod leaf_influence_and_material_laws;
use leaf_influence_and_material_laws::*;

#[path = "🧭️path-helpers/🦀️.rs"]
mod path_helpers;
use path_helpers::*;

#[path = "🚦️gate-parity/🦀️.rs"]
mod gate_parity;
