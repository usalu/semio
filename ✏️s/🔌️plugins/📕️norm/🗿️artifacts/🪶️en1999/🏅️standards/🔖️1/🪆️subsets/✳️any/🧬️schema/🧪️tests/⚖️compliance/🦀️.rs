//! 📏️ EN 1999 numeric compliance tests — hand-derived worked examples + DoD gates.

use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus, ReliabilityClass};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_io::text::snapshot::encode_en1999_dsl;
use semio_s_artifact_norm_en1999_io::text::snapshot::decode_en1999_dsl;
use semio_s_artifact_norm_en1999_checks::{evaluate_structure, na_de, part_en1990, part_1_1, part_1_2, part_1_3};
use std::path::PathBuf;
use std::process::Command;

#[path = "🔬️resistance-worked-examples/🦀️.rs"]
mod resistance_worked_examples;

#[path = "📐️worked-example-vectors/🦀️.rs"]
mod worked_example_vectors;

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

#[path = "🧵️weld-runs-and-outlines/🦀️.rs"]
mod weld_runs_and_outlines;

#[path = "🚦️gate-parity/🦀️.rs"]
mod gate_parity;

#[test]
fn reliability_class_rc3_raises_the_governing_design_effect_by_k_fi() {
    for annex in [AnnexChoice::De, AnnexChoice::En] {
        let member = En1999Snapshot::compliant_roof_purlin().members[0].clone();
        let params = na_de::AnnexParams::for_choice(annex);
        let governing = |class| {
            part_en1990::uls_combinations(&member, &params, class).iter().map(|e| e.n_ed.abs() + e.v_z_ed.abs() + e.m_y_ed.abs()).fold(0.0_f64, f64::max)
        };
        let k_fi = ReliabilityClass::RC3.k_fi(annex);
        assert!((k_fi - 1.1).abs() < 1e-9, "registry K_FI of RC3 (unverified) is {k_fi}");
        let ratio = governing(ReliabilityClass::RC3) / governing(ReliabilityClass::RC2);
        assert!((ratio - k_fi / ReliabilityClass::RC2.k_fi(annex)).abs() < 1e-9, "governing design effect ratio {ratio}");
    }
}

#[test]
fn reliability_class_of_the_document_reaches_the_evaluation() {
    let rc2 = En1999Snapshot::compliant_roof_purlin();
    let rc3 = En1999Snapshot { reliability_class: ReliabilityClass::RC3, ..rc2.clone() };
    let load_effect = |document: &En1999Snapshot| evaluate_structure(document).checks.iter().map(|c| c.utilization).sum::<f64>();
    assert!(load_effect(&rc3) > load_effect(&rc2));
}
