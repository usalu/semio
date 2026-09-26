//! 🚦️ Cross-family compliance gate — examples, locales, paths, remedies, annex divergence.

#![allow(clippy::type_complexity)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_norm_contract::app_surface::{apply_remedy_edit, get_value_at_path, parse_path, set_value_at_path};
use semio_s_artifact_norm_contract::document::{CheckReport, CheckStatus, NormFamily, RemedyBound};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// 🌍️ Families with no annex axis, or with documented identical EN/DE recommended values under evaluate.
const ANNEX_IDENTICAL_ALLOWLIST: &[&str] = &["din4108", "din18599", "iso16757", "vdi3805"];

/// 🚫 Forbidden evaluate-path gaming tokens (fingerprint / score folds).
const GAMING_TOKENS: &[&str] = &["field_fingerprint", "id_score", "tag_fp", "en1999.en1990.psi"];

struct FamilyReport {
    id: &'static str,
    errors: Vec<String>,
}

impl FamilyReport {
    fn new(id: &'static str) -> Self {
        Self { id, errors: Vec::new() }
    }

    fn fail(&mut self, message: impl Into<String>) {
        self.errors.push(message.into());
    }

    fn ok(&self) -> bool {
        self.errors.is_empty()
    }

    fn summary(&self) -> String {
        if self.ok() {
            format!("{}: PASS", self.id)
        } else {
            format!("{}: FAIL — {}", self.id, self.errors.join("; "))
        }
    }
}

fn decode_dsl<D: store::ArtifactDsl>(label: &str, text: &str, report: &mut FamilyReport) -> Option<D> {
    match D::parse_dsl(text) {
        Ok(doc) => Some(doc),
        Err(error) => {
            report.fail(format!("{label}: decode failed: {error}"));
            None
        }
    }
}

fn assert_localized_copy(report: &mut FamilyReport, check_id: &str, field: &str, en: &str, de: &str) {
    if en.trim().is_empty() {
        report.fail(format!("{check_id}: {field}.en empty"));
    }
    if de.trim().is_empty() {
        report.fail(format!("{check_id}: {field}.de empty"));
    }
    if !en.trim().is_empty() && en == de {
        report.fail(format!("{check_id}: {field}.en identical to {field}.de"));
    }
}

fn assert_report_copy_and_paths<D: dsl::ToValue>(report: &mut FamilyReport, doc: &D, check_report: &CheckReport) {
    let root = dsl::ToValue::to_value(doc);
    let en = protocol::Locale::En;
    let de = protocol::Locale::De;
    for check in &check_report.checks {
        let title_en = check.title.resolve(&en);
        let title_de = check.title.resolve(&de);
        let explanation_en = check.explanation.resolve(&en);
        let explanation_de = check.explanation.resolve(&de);
        assert_localized_copy(report, &check.id, "title", title_en, title_de);
        assert_localized_copy(report, &check.id, "explanation", explanation_en, explanation_de);
        if title_en == explanation_en {
            report.fail(format!("{}: title.en identical to explanation.en", check.id));
        }
        if title_de == explanation_de {
            report.fail(format!("{}: title.de identical to explanation.de", check.id));
        }
        if !check.subject.path.is_empty() {
            if let Err(error) = parse_path(&check.subject.path) {
                report.fail(format!("{}: subject.path parse '{}': {error}", check.id, check.subject.path));
            } else if let Err(error) = get_value_at_path(&root, &check.subject.path) {
                report.fail(format!("{}: subject.path resolve '{}': {error}", check.id, check.subject.path));
            }
        }
        if matches!(check.status, CheckStatus::NotApplicable) {
            if explanation_en.trim().is_empty() || explanation_de.trim().is_empty() {
                report.fail(format!("{}: NotApplicable missing localized reason", check.id));
            }
        }
        if matches!(check.status, CheckStatus::Fail) {
            if check.remedies.is_empty() {
                report.fail(format!("{}: Fail without remedies", check.id));
            }
        }
        for (index, remedy) in check.remedies.iter().enumerate() {
            if remedy.target.path.is_empty() {
                continue;
            }
            if let Err(error) = parse_path(&remedy.target.path) {
                report.fail(format!("{}: remedy[{index}].target.path parse '{}': {error}", check.id, remedy.target.path));
            } else if let Err(error) = get_value_at_path(&root, &remedy.target.path) {
                report.fail(format!("{}: remedy[{index}].target.path resolve '{}': {error}", check.id, remedy.target.path));
            }
        }
    }
}

fn assert_remedies_flip_fails<F>(report: &mut FamilyReport, doc: &F::Document, check_report: &CheckReport)
where
    F: NormFamily,
    F::Document: Clone + dsl::ToValue + dsl::FromValue,
{
    for check in check_report.checks.iter().filter(|check| matches!(check.status, CheckStatus::Fail)) {
        if check.remedies.is_empty() {
            continue;
        }
        let applicables: Vec<(usize, &semio_s_artifact_norm_contract::document::Remedy)> = check
            .remedies
            .iter()
            .enumerate()
            .filter(|(_, remedy)| remedy.applicable)
            .collect();
        if applicables.is_empty() {
            continue;
        }
        let mut cleared = false;
        let mut partials = Vec::new();
        let mut tried = 0usize;
        for (remedy_index, remedy) in applicables {
            let has_scalar = !matches!(remedy.bound, RemedyBound::OneOf) || !remedy.options.is_empty();
            if !has_scalar {
                continue;
            }
            let mut probe = dsl::ToValue::to_value(doc);
            match get_value_at_path(&probe, &remedy.target.path) {
                Ok(dsl::DslValue::Array(_)) | Ok(dsl::DslValue::Map(_)) => {
                    partials.push(format!("remedy[{remedy_index}] targets non-scalar path (skipped for flip)"));
                    continue;
                }
                Err(error) => {
                    partials.push(format!("remedy[{remedy_index}] path: {error}"));
                    continue;
                }
                Ok(_) => {}
            }
            tried += 1;
            let mut tree = probe;
            if let Err(fault) = apply_remedy_edit(check_report, &check.id, remedy_index, 0, &mut tree) {
                partials.push(format!("remedy[{remedy_index}] apply failed: {fault:?}"));
                continue;
            }
            let fixed = match dsl::FromValue::from_value(tree) {
                Ok(doc) => doc,
                Err(error) => {
                    partials.push(format!("remedy[{remedy_index}] decode: {error}"));
                    continue;
                }
            };
            let after = F::evaluate(&fixed);
            let Some(updated) = after.checks.iter().find(|item| item.id == check.id) else {
                cleared = true;
                break;
            };
            if !matches!(updated.status, CheckStatus::Fail) {
                cleared = true;
                break;
            }
            if updated.utilization < check.utilization {
                partials.push(format!(
                    "remedy[{remedy_index}] only lowered utilization {:.4}→{:.4}",
                    check.utilization, updated.utilization
                ));
            } else if updated.utilization > check.utilization + 1e-9 {
                partials.push(format!(
                    "remedy[{remedy_index}] raised utilization {:.4}→{:.4} (still Fail)",
                    check.utilization, updated.utilization
                ));
            } else {
                partials.push(format!("remedy[{remedy_index}] left check failing (u={:.4})", updated.utilization));
            }
        }
        if tried == 0 {
            continue;
        }
        if !cleared && applicables.len() > 1 {
            let mut tree = dsl::ToValue::to_value(doc);
            let mut sequential_ok = true;
            for (remedy_index, remedy) in &applicables {
                let has_scalar = !matches!(remedy.bound, RemedyBound::OneOf) || !remedy.options.is_empty();
                if !has_scalar {
                    sequential_ok = false;
                    break;
                }
                if apply_remedy_edit(check_report, &check.id, *remedy_index, 0, &mut tree).is_err() {
                    sequential_ok = false;
                    break;
                }
            }
            if sequential_ok {
                if let Ok(fixed) = dsl::FromValue::from_value(tree) {
                    let after = F::evaluate(&fixed);
                    match after.checks.iter().find(|item| item.id == check.id) {
                        None => cleared = true,
                        Some(updated) if !matches!(updated.status, CheckStatus::Fail) => cleared = true,
                        Some(updated) => partials.push(format!(
                            "sequential applicables left failing (u={:.4})",
                            updated.utilization
                        )),
                        _ => {}
                    }
                }
            }
        }
        if !cleared {
            report.fail(format!(
                "{}: no applicable remedy cleared the Fail ({})",
                check.id,
                partials.join("; ")
            ));
        }
    }
}

fn annex_values_differ(left: &CheckReport, right: &CheckReport) -> bool {
    for left_check in &left.checks {
        if let Some(right_check) = right.checks.iter().find(|check| check.id == left_check.id) {
            if (left_check.limit.value - right_check.limit.value).abs() > 1e-12 {
                return true;
            }
            if (left_check.computed.value - right_check.computed.value).abs() > 1e-12 {
                return true;
            }
        }
    }
    false
}

/// 🗂️ Walk artifact rust sources under evaluate paths (schema + inferences), skipping tests.
fn walk_evaluate_rs(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if name.contains("tests") || name == "target" || name.starts_with('.') {
                continue;
            }
            walk_evaluate_rs(&path, out);
            continue;
        }
        if !name.ends_with(".rs") {
            continue;
        }
        let normalized = path.to_string_lossy().replace('\\', "/");
        let on_evaluate_path = normalized.contains("/🧬️schema/") || normalized.contains("/inferences/") || normalized.contains("evaluate");
        if on_evaluate_path {
            out.push(path);
        }
    }
}

/// 🧪 True when `.len().max(1)` sits on a quantity / assessor line (check quantity gaming).
fn len_max_one_is_check_quantity(line: &str) -> bool {
    if !line.contains(".len().max(1)") {
        return false;
    }
    line.contains("Quantity::")
        || line.contains(".minimum(")
        || line.contains(".maximum(")
        || line.contains("computed")
        || line.contains("limit")
        || line.contains("utilization")
}

/// 🔬 Field folded through `1e-9 *` (divide-guard `.max(1e-9)` alone is allowed).
fn has_field_epsilon_fold(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.contains(".max(1e-9)") && !trimmed.contains("1e-9 *") && !trimmed.contains("1e-9*") {
        return false;
    }
    trimmed.contains("1e-9 *") || trimmed.contains("1e-9*")
}

/// 🖨️ Leak a printed DSL document for gate example registration.
fn leaked_print_dsl<D: store::ArtifactDsl>(doc: &D) -> &'static str {
    Box::leak(doc.print_dsl().into_boxed_str())
}

/// 🛡️ Source-scan: reject fingerprint / score / epsilon-fold / len-as-quantity gaming in evaluate paths.
fn scan_evaluate_gaming() -> Vec<String> {
    let artifacts = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts");
    let mut files = Vec::new();
    walk_evaluate_rs(&artifacts, &mut files);
    let mut hits = Vec::new();
    for path in files {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let display = path.to_string_lossy().replace('\\', "/");
        for (line_no, line) in text.lines().enumerate() {
            let line_no = line_no + 1;
            for token in GAMING_TOKENS {
                if line.contains(token) {
                    hits.push(format!("{display}:{line_no}: token `{token}`"));
                }
            }
            if has_field_epsilon_fold(line) {
                hits.push(format!("{display}:{line_no}: field fold `1e-9 *`"));
            }
            if len_max_one_is_check_quantity(line) {
                hits.push(format!("{display}:{line_no}: `.len().max(1)` used as check quantity"));
            }
        }
    }
    hits
}

fn assert_annex_divergence<F>(report: &mut FamilyReport, doc: &F::Document)
where
    F: NormFamily,
    F::Document: Clone + dsl::ToValue + dsl::FromValue,
{
    if ANNEX_IDENTICAL_ALLOWLIST.contains(&report.id) {
        return;
    }
    let root = dsl::ToValue::to_value(doc);
    if get_value_at_path(&root, "annex").is_err() {
        report.fail("annex field missing but family not on ANNEX_IDENTICAL_ALLOWLIST");
        return;
    }
    let mut de_tree = root.clone();
    let mut en_tree = root;
    if let Err(error) = set_value_at_path(&mut de_tree, "annex", dsl::DslValue::String("De".into())) {
        report.fail(format!("set annex=De: {error}"));
        return;
    }
    if let Err(error) = set_value_at_path(&mut en_tree, "annex", dsl::DslValue::String("En".into())) {
        report.fail(format!("set annex=En: {error}"));
        return;
    }
    let de_doc = match dsl::FromValue::from_value(de_tree) {
        Ok(doc) => doc,
        Err(error) => {
            report.fail(format!("decode annex=De: {error}"));
            return;
        }
    };
    let en_doc = match dsl::FromValue::from_value(en_tree) {
        Ok(doc) => doc,
        Err(error) => {
            report.fail(format!("decode annex=En: {error}"));
            return;
        }
    };
    let de_report = F::evaluate(&de_doc);
    let en_report = F::evaluate(&en_doc);
    if !annex_values_differ(&de_report, &en_report) {
        report.fail("DE vs EN annex produced identical limit/computed values for all shared check ids");
    }
}

fn gate_family<F>(id: &'static str, examples: &[(&str, &str)]) -> FamilyReport
where
    F: NormFamily,
    F::Document: Clone + Default + store::ArtifactDsl + dsl::ToValue + dsl::FromValue,
{
    let mut report = FamilyReport::new(id);
    let default_doc = F::Document::default();
    let default_report = F::evaluate(&default_doc);
    if default_report.checks.is_empty() {
        report.fail("default evaluate produced zero checks");
    }
    assert_report_copy_and_paths(&mut report, &default_doc, &default_report);
    assert_annex_divergence::<F>(&mut report, &default_doc);

    let mut compliant = 0usize;
    let mut noncompliant = 0usize;
    for (label, text) in examples {
        let Some(doc) = decode_dsl::<F::Document>(label, text, &mut report) else {
            continue;
        };
        let check_report = F::evaluate(&doc);
        if check_report.checks.is_empty() {
            report.fail(format!("{label}: evaluate produced zero checks"));
        }
        assert_report_copy_and_paths(&mut report, &doc, &check_report);
        assert_remedies_flip_fails::<F>(&mut report, &doc, &check_report);
        if check_report.complies() {
            compliant += 1;
        }
        let fail_count = check_report.checks.iter().filter(|check| matches!(check.status, CheckStatus::Fail)).count();
        if fail_count >= 2 {
            noncompliant += 1;
        }
    }
    if examples.is_empty() {
        report.fail("no examples registered for gate");
    }
    if compliant == 0 {
        report.fail("no compliant example (report.complies())");
    }
    if noncompliant == 0 {
        report.fail("no non-compliant example with ≥2 Fail checks");
    }
    report
}

#[test]
fn compliance_gate_all_families() {
    let gaming = scan_evaluate_gaming();
    if !gaming.is_empty() {
        panic!("evaluate-path gaming source-scan failed ({} hits):\n{}", gaming.len(), gaming.join("\n"));
    }

    let mut results = Vec::new();

    results.push(gate_family::<semio_s_artifact_norm_din4108::editor::din4108::Din4108Family>(
        "din4108",
        &[
            ("demo", semio_s_artifact_norm_din4108::examples::demo::PRIMARY_TEXT),
            ("failing_thin_insulation", semio_s_artifact_norm_din4108::examples::failing_thin_insulation::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_din16798::editor::din16798::DinEn16798Family>(
        "din16798",
        &[
            ("demo", semio_s_artifact_norm_din16798::examples::demo::PRIMARY_TEXT),
            ("compliant_office", semio_s_artifact_norm_din16798::examples::compliant_office::PRIMARY_TEXT),
            ("noncompliant_office", semio_s_artifact_norm_din16798::examples::noncompliant_office::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_din18599::editor::din18599::DinV18599Family>(
        "din18599",
        &[
            ("demo", semio_s_artifact_norm_din18599::examples::demo::PRIMARY_TEXT),
            ("compliant_detached", semio_s_artifact_norm_din18599::examples::compliant_detached::PRIMARY_TEXT),
            ("noncompliant_detached", semio_s_artifact_norm_din18599::examples::noncompliant_detached::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1990::editor::en1990::En1990Family>(
        "en1990",
        &[
            (
                "high_consequence_office",
                semio_s_artifact_norm_en1990::standards::v1::subsets::any::examples::high_consequence_office::PRIMARY_TEXT,
            ),
            (
                "road_bridge_compliant",
                semio_s_artifact_norm_en1990::standards::v1::subsets::any::examples::road_bridge_compliant::PRIMARY_TEXT,
            ),
            (
                "road_bridge_failing",
                semio_s_artifact_norm_en1990::standards::v1::subsets::any::examples::road_bridge_failing::PRIMARY_TEXT,
            ),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1991::editor::en1991::En1991Family>(
        "en1991",
        &[
            ("de_office_compliant", semio_s_artifact_norm_en1991::de_office_compliant::PRIMARY_TEXT),
            ("multi_fail_noncompliant", semio_s_artifact_norm_en1991::multi_fail_noncompliant::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1992::editor::en1992::En1992Family>(
        "en1992",
        &[
            ("compliant_office_frame", semio_s_artifact_norm_en1992::compliant_office_frame::PRIMARY_TEXT),
            ("failing_under_reinforced", semio_s_artifact_norm_en1992::failing_under_reinforced::PRIMARY_TEXT),
            ("liquid_retaining_fem_anchor", semio_s_artifact_norm_en1992::liquid_retaining_fem_anchor::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1993::editor::en1993::En1993Family>(
        "en1993",
        &[
            ("heb240_compliant", semio_s_artifact_norm_en1993::heb240_compliant::PRIMARY_TEXT),
            ("high_strength_connection", semio_s_artifact_norm_en1993::high_strength_connection::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1994::editor::en1994::En1994Family>(
        "en1994",
        &[
            ("composite_floor_beam", semio_s_artifact_norm_en1994::composite_floor_beam::PRIMARY_TEXT),
            ("composite_floor_beam_failing", semio_s_artifact_norm_en1994::composite_floor_beam_failing::PRIMARY_TEXT),
            ("composite_bridge_girder", semio_s_artifact_norm_en1994::composite_bridge_girder::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1995::editor::en1995::En1995Family>(
        "en1995",
        &[
            ("glulam_footbridge", semio_s_artifact_norm_en1995::glulam_footbridge::PRIMARY_TEXT),
            ("multi_fail_timber", semio_s_artifact_norm_en1995::multi_fail_timber::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1996::editor::en1996::En1996Family>(
        "en1996",
        &[
            ("loadbearing_wall", semio_s_artifact_norm_en1996::loadbearing_wall::PRIMARY_TEXT),
            ("multi_fail_masonry", semio_s_artifact_norm_en1996::multi_fail_masonry::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1997::editor::en1997::En1997Family>(
        "en1997",
        &[
            ("demo", semio_s_artifact_norm_en1997::examples::demo::PRIMARY_TEXT),
            ("compliant", semio_s_artifact_norm_en1997::examples::compliant::PRIMARY_TEXT),
            ("noncompliant", semio_s_artifact_norm_en1997::examples::noncompliant::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1998::editor::en1998::En1998Family>(
        "en1998",
        &[
            (
                "seismic_rc_frame",
                leaked_print_dsl(&semio_s_artifact_norm_en1998::seismic_rc_frame::snapshot()),
            ),
            (
                "seismic_rc_frame_fail",
                leaked_print_dsl(&semio_s_artifact_norm_en1998::seismic_rc_frame_fail::snapshot()),
            ),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_en1999::editor::en1999::En1999Family>(
        "en1999",
        &[
            ("aluminium_roof_purlin", semio_s_artifact_norm_en1999::aluminium_roof_purlin::PRIMARY_TEXT),
            ("noncompliant_multi_fail", semio_s_artifact_norm_en1999::noncompliant_multi_fail::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_iso16757::editor::iso16757::Iso16757Family>(
        "iso16757",
        &[
            ("demo", semio_s_artifact_norm_iso16757::examples::demo::PRIMARY_TEXT),
            ("broken", semio_s_artifact_norm_iso16757::examples::broken::PRIMARY_TEXT),
        ],
    ));
    results.push(gate_family::<semio_s_artifact_norm_vdi3805::editor::vdi3805::Vdi3805Family>(
        "vdi3805",
        &[
            ("demo", semio_s_artifact_norm_vdi3805::examples::demo::PRIMARY_TEXT),
            ("nonconforming", semio_s_artifact_norm_vdi3805::examples::nonconforming::PRIMARY_TEXT),
        ],
    ));

    let mut summary = String::new();
    let mut failed = 0usize;
    for result in &results {
        let _ = writeln!(summary, "{}", result.summary());
        if !result.ok() {
            failed += 1;
        }
    }
    eprintln!("[DEBUG] compliance-gate fleet:\n{summary}");
    assert_eq!(results.len(), 15, "gate must cover all fifteen families");
    if failed > 0 {
        panic!("{failed}/15 families failed compliance gate:\n{summary}");
    }
}
