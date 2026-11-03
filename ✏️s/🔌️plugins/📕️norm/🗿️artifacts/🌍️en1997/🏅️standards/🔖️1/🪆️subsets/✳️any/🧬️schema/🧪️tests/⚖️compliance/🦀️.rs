//! ⚖️ EN 1997 compliance numeric tests + Python oracle parity + remedy laws.

use semio_s_plugin_norm_app_surface::{get_value_at_path, parse_path};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus};
use semio_framework_value::ToValue;
use semio_s_artifact_norm_en1997_checks::{check_project, part_1, part_2, parse_design_approach, resolve_params, DesignApproach};
use semio_s_artifact_norm_en1997_inferences::outline::En1997Outline;
use semio_s_artifact_norm_en1997_model::{compliant_demo, noncompliant_demo};
use semio_s_artifact_norm_en1997_io::text::snapshot::decode_en1997_dsl;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use std::path::PathBuf;
use std::process::Command;

include!("🧮️design-approach-and-resistance-laws/🦀️.rs");

include!("🔮️oracle-and-remedy-laws/🦀️.rs");

include!("🌐️entity-check-laws/🦀️.rs");

#[test]
fn compliant_dsl_asset_complies() {
    let text = include_str!("../../../🖼️assets/🏗️compliant/🗣️.dsl.semio");
    let doc = decode_en1997_dsl(text).expect("decode compliant DSL");
    assert!(check_project(&doc).complies(), "compliant DSL must Pass");
}

#[test]
fn noncompliant_dsl_asset_has_multiple_fails() {
    let text = include_str!("../../../🖼️assets/🚨noncompliant/🗣️.dsl.semio");
    let doc = decode_en1997_dsl(text).expect("decode noncompliant DSL");
    let n = check_project(&doc).failing().count();
    assert!(n >= 2, "noncompliant DSL fail_count={n}");
}

include!("🏷️field-meta-and-perturbation-laws/🦀️.rs");

include!("📇️catalogue-laws/🦀️.rs");

#[test]
fn annex_switch_reads_the_registry_and_changes_only_the_expected_values() {
    use semio_s_artifact_norm_en1997_checks::resolve_upl_params;
    let (en, de) = (resolve_upl_params(AnnexChoice::En, "persistent"), resolve_upl_params(AnnexChoice::De, "persistent"));
    assert_eq!((en.gamma_hyd, de.gamma_hyd), (1.35, 1.8));
    assert_eq!((en.gamma_g_stb, en.gamma_g_dst), (de.gamma_g_stb, de.gamma_g_dst));
    let (g_en, g_de) = (resolve_params(DesignApproach::Da2, AnnexChoice::En, "persistent"), resolve_params(DesignApproach::Da2, AnnexChoice::De, "persistent"));
    assert_eq!((g_en.gamma_g, g_en.gamma_q), (g_de.gamma_g, g_de.gamma_q));
    assert_eq!(resolve_params(DesignApproach::Da2, AnnexChoice::De, "transient").gamma_g, 1.2);
}
