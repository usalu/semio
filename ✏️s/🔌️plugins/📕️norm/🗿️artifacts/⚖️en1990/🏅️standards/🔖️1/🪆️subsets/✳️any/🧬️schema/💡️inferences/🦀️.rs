//! 💡️ En1990 inference schema — outline + full basis-of-design evaluate().

use crate::En1990Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1990 snapshot.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1990.inference")]
pub struct En1990Inference {
    #[derived]
    pub outline: En1990Outline,
}

impl protocol::Inference<En1990Snapshot> for En1990Inference {
    fn infer(snapshot: &En1990Snapshot) -> Self {
        Self { outline: En1990Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1990Snapshot> for En1990Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1990.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.norm.en1990.inference.outline",
            reads: &[
                "annex",
                "projectId",
                "structureKind",
                "altitudeM",
                "consequenceClass",
                "reliabilityClass",
                "designWorkingLifeCategory",
                "designWorkingLifeYears",
                "referencePeriodYears",
                "supervisionLevel",
                "inspectionLevel",
                "kFiDeclared",
                "betaComputed",
                "permanents",
                "variables",
                "accidentals",
                "seismics",
                "members",
                "bridgeSls",
                "effects",
            ],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1990Builder {
    type Snapshot = En1990Snapshot;
    type Inference = En1990Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1990.inference`'s facet leaves into the OS-wide inference catalog.
pub fn en1990_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1990.inference",
        inference: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, DesignSituation, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use crate::standards::v1::subsets::any::schema::{
    check_reliability_index_for, combination_6_11, combination_6_12b, combination_equ_for, combination_geo_a24c, combination_sls_char, combination_sls_frequent,
    combination_sls_quasi_permanent, combination_str_geo_610a_for, combination_str_geo_610b_for, k_fi, resolve_psi_category, ActionSet, NaDe, NaEn, NationalAnnex, NationalAnnexes,
};
use crate::{Member, MemberEffect};

fn copy(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn q_force(n: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, n)
}

fn q_len(m: f64) -> Quantity {
    Quantity::new(QuantityKind::Length, m)
}

fn q_dim(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

fn member_label(member: &Member) -> LocalizedCopy {
    copy(member.label_en.clone(), member.label_de.clone())
}

fn project_subject(document: &En1990Snapshot) -> SubjectRef {
    SubjectRef::new(
        document.project_id.clone(),
        "projectId",
        copy(
            format!("Project {}", document.project_id),
            format!("Projekt {}", document.project_id),
        ),
    )
}

fn str_title(section: &str) -> LocalizedCopy {
    copy(
        format!("STR {section} — ultimate limit state"),
        format!("STR Gl. {section} — Grenzzustand der Tragfähigkeit"),
    )
}

fn geo_title(section: &str) -> LocalizedCopy {
    copy(
        format!("GEO {section} — geotechnical ultimate limit state"),
        format!("GEO Gl. {section} — geotechnischer Grenzzustand der Tragfähigkeit"),
    )
}

fn member_index(document: &En1990Snapshot, member_id: &str) -> usize {
    document.members.iter().position(|m| m.id == member_id).unwrap_or(0)
}

fn member_path(_document: &En1990Snapshot, member_id: &str, field: &str) -> String {
    format!("members[id={member_id}].{field}")
}

fn influence_for(effects: &[MemberEffect], member_id: &str, action_id: &str) -> f64 {
    effects.iter().find(|e| e.member_id == member_id && e.action_id == action_id).map(|e| e.influence).unwrap_or(0.0)
}

pub(crate) fn action_set_for_member(document: &En1990Snapshot, member_id: &str) -> ActionSet {
    let g_k: f64 = document
        .permanents
        .iter()
        .filter(|p| p.kind == "g_sup")
        .map(|p| influence_for(&document.effects, member_id, &p.id) * p.gk)
        .sum();
    let g_inf: f64 = document
        .permanents
        .iter()
        .filter(|p| p.kind == "g_inf")
        .map(|p| influence_for(&document.effects, member_id, &p.id) * p.gk)
        .sum();
    let p_k: f64 = document
        .permanents
        .iter()
        .filter(|p| p.kind == "prestress")
        .map(|p| influence_for(&document.effects, member_id, &p.id) * p.gk)
        .sum();
    let q_k: Vec<(String, f64)> = document
        .variables
        .iter()
        .map(|v| {
            let cat = resolve_psi_category(document.annex, &v.category, document.altitude_m);
            (cat, influence_for(&document.effects, member_id, &v.id) * v.qk)
        })
        .collect();
    let a_d: f64 = document.accidentals.iter().map(|a| influence_for(&document.effects, member_id, &a.id) * a.ad).sum();
    let a_ed: f64 = document.seismics.iter().map(|s| influence_for(&document.effects, member_id, &s.id) * s.a_ed()).sum();
    ActionSet { g_k, g_k_inf: g_inf, p_k, q_k, a_d, a_ed }
}

fn push_uls_str_geo<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    let ed_a = combination_str_geo_610a_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let ed_b = combination_str_geo_610b_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let ed_610 = ed_a.max(ed_b);
    let rd = member.rd_str;
    let path = member_path(document, &member.id, "rdStr");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    for (rule, ed, section) in [("6.10a", ed_a, "6.10a"), ("6.10b", ed_b, "6.10b"), ("6.10", ed_610, "6.10")] { // governing 6.10 = max(6.10a,6.10b) per DIN EN 1990/NA NDP A1.3.1(4)
        let id = format!("en1990.str.{}.{}.{leading}", member.id, rule);
        let mut builder = CheckResult::assess(id, if is_bridge(&document.structure_kind) { "EN 1990 STR A2.4(B)" } else { "EN 1990 STR A1.2(B)" }, ClauseId::new("EN 1990", "§6.4", section), subject.clone(), str_title(section))
            .utilization(q_force(ed), q_force(rd))
            .annex(annex.choice())
            .explanation({
                let k = k_fi(document.consequence_class);
                if rule == "6.10" {
                    copy(
                        format!("Governing E_d = max(6.10a, 6.10b) = {ed:.1} N per DIN EN 1990/NA NDP A1.3.1(4); R_d = {rd:.1} N, K_FI = {k:.2}, leading={leading}"),
                        format!("Maßgebend E_d = max(6.10a, 6.10b) = {ed:.1} N nach DIN EN 1990/NA NDP A1.3.1(4); R_d = {rd:.1} N, K_FI = {k:.2}, führend={leading}"),
                    )
                } else {
                    copy(
                        format!("E_d = {ed:.1} N, R_d = {rd:.1} N, K_FI = {k:.2}, leading={leading}"),
                        format!("E_d = {ed:.1} N, R_d = {rd:.1} N, K_FI = {k:.2}, führend={leading}"),
                    )
                }
            });
        if ed > rd {
            builder = builder.remedy(Remedy::at_least(
                subject.clone(),
                q_force(rd),
                q_force(ed),
                copy(
                    format!("Increase R_d,STR of {} from {:.1} kN to at least {:.1} kN.", member.label_en, rd / 1000.0, ed / 1000.0),
                    format!("R_d,STR von {} von {:.1} kN auf mindestens {:.1} kN erhöhen.", member.label_de, rd / 1000.0, ed / 1000.0),
                ),
            ));
        }
        report.push(builder.build());
    }
}

fn push_uls_equ<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    let ed = combination_equ_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let rd = member.rd_equ_destab;
    let path = member_path(document, &member.id, "rdEquDestab");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.equ.destab.{}.{leading}", member.id),
        if is_bridge(&document.structure_kind) { "EN 1990 EQU A2.4(A)" } else { "EN 1990 EQU A1.2(A)" },
        ClauseId::new("EN 1990", "Table A1.2(A)", "EQU"),
        subject.clone(),
        copy("EQU equilibrium", "EQU Gleichgewicht"),
    )
    .utilization(q_force(ed.max(0.0)), q_force(rd))
    .annex(annex.choice())
    .explanation(copy(
        format!("Destabilising E_d = {ed:.1} N vs R_d,EQU = {rd:.1} N"),
        format!("Entstabilisierendes E_d = {ed:.1} N gegenüber R_d,EQU = {rd:.1} N"),
    ));
    if ed > rd {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(rd),
            q_force(ed),
            copy(
                format!("Increase R_d,EQU (destabilising) of {} to at least {:.1} kN.", member.label_en, ed / 1000.0),
                format!("R_d,EQU (entstabilisierend) von {} auf mindestens {:.1} kN erhöhen.", member.label_de, ed / 1000.0),
            ),
        ));
    }
    report.push(builder.build());
}

fn push_accidental<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    if actions.a_d.abs() < f64::EPSILON && document.accidentals.is_empty() {
        report.push(
            CheckResult::assess(
                format!("en1990.6.11.{}", member.id),
                "EN 1990",
                ClauseId::new("EN 1990", "§6.4", "6.11"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "rdStr"), member_label(member)),
                copy("Accidental combination 6.11", "Außergewöhnliche Kombination 6.11"),
            )
            .not_applicable(copy("No accidental action A_d defined.", "Keine außergewöhnliche Einwirkung A_d definiert."))
            .annex(annex.choice())
            .build(),
        );
        return;
    }
    let ed = combination_6_11(annex, actions, leading);
    let rd = member.rd_str;
    let path = member_path(document, &member.id, "rdStr");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.6.11.{}.{leading}", member.id),
        "EN 1990",
        ClauseId::new("EN 1990", "§6.4", "6.11"),
        subject.clone(),
        copy("Accidental combination 6.11", "Außergewöhnliche Kombination 6.11"),
    )
    .utilization(q_force(ed), q_force(rd))
    .annex(annex.choice())
    .explanation(copy(
        format!("Accidental ULS: E_d = ΣG + A_d + ψ₁·Q₁ + Σψ₂·Qᵢ = {ed:.1} N (leading={leading})"),
        format!("Außergewöhnlicher GZT: E_d = ΣG + A_d + ψ₁·Q₁ + Σψ₂·Qᵢ = {ed:.1} N (führend={leading})"),
    ));
    if ed > rd {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(rd),
            q_force(ed),
            copy(
                format!("Increase R_d,STR to at least {:.1} kN for accidental 6.11.", ed / 1000.0),
                format!("R_d,STR für außergewöhnliche Kombination 6.11 auf mindestens {:.1} kN erhöhen.", ed / 1000.0),
            ),
        ));
    }
    report.push(builder.build());
}

fn push_seismic<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet) {
    if actions.a_ed.abs() < f64::EPSILON {
        report.push(
            CheckResult::assess(
                format!("en1990.6.12b.{}", member.id),
                "EN 1990",
                ClauseId::new("EN 1990", "§6.4.3.4", "6.12b"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "rdStr"), member_label(member)),
                copy("Seismic combination 6.12b", "Erdbebenkombination 6.12b"),
            )
            .not_applicable(copy("No seismic action A_Ed defined.", "Keine seismische Einwirkung A_Ed definiert."))
            .annex(annex.choice())
            .build(),
        );
        return;
    }
    let ed = combination_6_12b(annex, actions, actions.a_ed);
    let rd = member.rd_str;
    let path = member_path(document, &member.id, "rdStr");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.6.12b.{}", member.id),
        "EN 1990",
        ClauseId::new("EN 1990", "§6.4.3.4", "6.12b"),
        subject.clone(),
        copy("Seismic combination 6.12b", "Erdbebenkombination 6.12b"),
    )
    .utilization(q_force(ed), q_force(rd))
    .annex(annex.choice())
    .explanation(copy(
        format!("Seismic ULS: E_d = ΣG + A_Ed + Σψ₂·Q = {ed:.1} N"),
        format!("Erdbeben-GZT: E_d = ΣG + A_Ed + Σψ₂·Q = {ed:.1} N"),
    ));
    if ed > rd {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(rd),
            q_force(ed),
            copy(
                format!("Increase R_d,STR to at least {:.1} kN for seismic 6.12b.", ed / 1000.0),
                format!("R_d,STR für Erdbebenkombination 6.12b auf mindestens {:.1} kN erhöhen.", ed / 1000.0),
            ),
        ));
    }
    report.push(builder.build());
}

fn push_sls_action_effects<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    let rd = member.rd_str;
    let path = member_path(document, &member.id, "rdStr");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    for (rule, ed, section, title_en, title_de) in [
        ("6.14b", combination_sls_char(annex, actions, leading), "6.14b", "SLS characteristic", "GZG charakteristisch"),
        ("6.15b", combination_sls_frequent(annex, actions, leading), "6.15b", "SLS frequent", "GZG häufig"),
        ("6.16b", combination_sls_quasi_permanent(annex, actions), "6.16b", "SLS quasi-permanent", "GZG quasi-ständig"),
    ] {
        let id = format!("en1990.sls.{}.{}.{leading}", member.id, rule);
        let mut builder = CheckResult::assess(id, "EN 1990 SLS", ClauseId::new("EN 1990", "§6.5", section), subject.clone(), copy(title_en, title_de))
            .utilization(q_force(ed), q_force(rd))
            .annex(annex.choice())
            .explanation(copy(format!("Service E = {ed:.1} N vs capacity {rd:.1} N"), format!("Gebrauchstauglichkeit E = {ed:.1} N gegenüber {rd:.1} N")));
        if ed > rd {
            builder = builder.remedy(Remedy::at_least(
                subject.clone(),
                q_force(rd),
                q_force(ed),
                copy(format!("Increase resistance to at least {:.1} kN for {title_en}.", ed / 1000.0), format!("Widerstand für {title_de} auf mindestens {:.1} kN erhöhen.", ed / 1000.0)),
            ));
        }
        report.push(builder.build());
        if rule == "6.16b" {
            break;
        }
    }
}

fn push_sls_deflection(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    let limit = if member.deflection_limit_ratio > 0.0 { member.span / member.deflection_limit_ratio } else { member.span / 250.0 };
    let path = member_path(document, &member.id, "deflectionW");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.a14.deflection.{}", member.id),
        "EN 1990 Annex A1.4",
        ClauseId::new("EN 1990", "Annex A1.4", "deflection"),
        subject.clone(),
        copy("Deflection limit", "Durchbiegungsgrenze"),
    )
    .utilization(q_len(member.deflection_w), q_len(limit))
    .annex(document.annex)
    .explanation(copy(
        format!("w = {:.1} mm vs l/{:.0} = {:.1} mm", member.deflection_w * 1000.0, member.deflection_limit_ratio, limit * 1000.0),
        format!("w = {:.1} mm gegenüber l/{:.0} = {:.1} mm", member.deflection_w * 1000.0, member.deflection_limit_ratio, limit * 1000.0),
    ));
    if member.deflection_w > limit {
        let required_stiffness_ratio = member.deflection_w / limit.max(f64::EPSILON);
        builder = builder.remedy(Remedy::at_most(
            subject,
            q_len(member.deflection_w),
            q_len(limit),
            copy(
                format!(
                    "Reduce deflection of {} from {:.1} mm to at most {:.1} mm (increase stiffness by ≥ {:.2}×).",
                    member.label_en,
                    member.deflection_w * 1000.0,
                    limit * 1000.0,
                    required_stiffness_ratio
                ),
                format!(
                    "Durchbiegung von {} von {:.1} mm auf höchstens {:.1} mm reduzieren (Steifigkeit um ≥ {:.2}× erhöhen).",
                    member.label_de,
                    member.deflection_w * 1000.0,
                    limit * 1000.0,
                    required_stiffness_ratio
                ),
            ),
        ));
    }
    report.push(builder.build());
}

fn push_sls_vibration(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    if member.vibration_frequency_min <= 0.0 {
        report.push(
            CheckResult::assess(
                format!("en1990.a14.vibration.{}", member.id),
                "EN 1990 Annex A1.4",
                ClauseId::new("EN 1990", "Annex A1.4", "vibration"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "vibrationFrequency"), member_label(member)),
                copy("Vibration frequency", "Schwingungsfrequenz"),
            )
            .not_applicable(copy("No vibration frequency criterion defined.", "Kein Schwingungsfrequenzkriterium definiert."))
            .annex(document.annex)
            .build(),
        );
        return;
    }
    let path = member_path(document, &member.id, "vibrationFrequency");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.a14.vibration.{}", member.id),
        "EN 1990 Annex A1.4",
        ClauseId::new("EN 1990", "Annex A1.4", "vibration"),
        subject.clone(),
        copy("Vibration frequency", "Schwingungsfrequenz"),
    )
    .minimum(q_dim(member.vibration_frequency), q_dim(member.vibration_frequency_min))
    .annex(document.annex)
    .explanation(copy(
        format!("f = {:.2} Hz vs f_min = {:.2} Hz", member.vibration_frequency, member.vibration_frequency_min),
        format!("f = {:.2} Hz gegenüber f_min = {:.2} Hz", member.vibration_frequency, member.vibration_frequency_min),
    ));
    if member.vibration_frequency < member.vibration_frequency_min {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_dim(member.vibration_frequency),
            q_dim(member.vibration_frequency_min),
            copy(
                format!("Increase natural frequency of {} from {:.2} Hz to at least {:.2} Hz.", member.label_en, member.vibration_frequency, member.vibration_frequency_min),
                format!("Eigenfrequenz von {} von {:.2} Hz auf mindestens {:.2} Hz erhöhen.", member.label_de, member.vibration_frequency, member.vibration_frequency_min),
            ),
        ));
    }
    report.push(builder.build());
}

fn push_fat<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet) {
    if member.rd_fat <= 0.0 {
        report.push(
            CheckResult::assess(
                format!("en1990.fat.{}", member.id),
                "EN 1990 FAT",
                ClauseId::new("EN 1990", "§2", "FAT"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "rdFat"), member_label(member)),
                copy("Fatigue limit state", "Ermüdungsgrenzzustand"),
            )
            .not_applicable(copy("FAT resistance not applicable for this member.", "FAT-Widerstand für dieses Bauteil nicht anwendbar."))
            .annex(document.annex)
            .build(),
        );
        return;
    }
    let ed = combination_sls_frequent(annex, actions, 0);
    let path = member_path(document, &member.id, "rdFat");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(format!("en1990.fat.{}", member.id), "EN 1990 FAT", ClauseId::new("EN 1990", "§2", "FAT"), subject.clone(), copy("Fatigue limit state", "Ermüdungsgrenzzustand"))
        .utilization(q_force(ed), q_force(member.rd_fat))
        .annex(document.annex)
        .explanation(copy(format!("Fatigue effect {ed:.1} N vs R_d,FAT {:.1} N", member.rd_fat), format!("Ermüdungswirkung {ed:.1} N gegenüber R_d,FAT {:.1} N", member.rd_fat)));
    if ed > member.rd_fat {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(member.rd_fat),
            q_force(ed),
            copy(format!("Increase R_d,FAT to at least {:.1} kN.", ed / 1000.0), format!("R_d,FAT auf mindestens {:.1} kN erhöhen.", ed / 1000.0)),
        ));
    }
    report.push(builder.build());
}


fn is_bridge(kind: &str) -> bool {
    matches!(kind, "road_bridge" | "footbridge" | "rail_bridge")
}

fn push_uls_geo<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    let ed_a = combination_str_geo_610a_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let ed_b = combination_str_geo_610b_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let ed_610 = ed_a.max(ed_b);
    let rd = member.rd_geo;
    let path = member_path(document, &member.id, "rdGeo");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    for (rule, ed, section) in [("6.10a", ed_a, "6.10a"), ("6.10b", ed_b, "6.10b"), ("6.10", ed_610, "6.10")] { // governing 6.10 = max(6.10a,6.10b) per DIN EN 1990/NA NDP A1.3.1(4)
        let id = format!("en1990.geo.{}.{}.{leading}", member.id, rule);
        let mut builder = CheckResult::assess(
            id,
            if is_bridge(&document.structure_kind) { "EN 1990 GEO A2.4(B)" } else { "EN 1990 GEO A1.2(B)" },
            ClauseId::new("EN 1990", "§6.4", section),
            subject.clone(),
            geo_title(section),
        )
        .utilization(q_force(ed), q_force(rd))
        .annex(annex.choice())
        .explanation(copy(
            format!("E_d = {ed:.1} N, R_d,GEO = {rd:.1} N, leading={leading}"),
            format!("E_d = {ed:.1} N, R_d,GEO = {rd:.1} N, führend={leading}"),
        ));
        if ed > rd {
            builder = builder.remedy(Remedy::at_least(
                subject.clone(),
                q_force(rd),
                q_force(ed),
                copy(
                    format!("Increase R_d,GEO of {} to at least {:.1} kN.", member.label_en, ed / 1000.0),
                    format!("R_d,GEO von {} auf mindestens {:.1} kN erhöhen.", member.label_de, ed / 1000.0),
                ),
            ));
        }
        report.push(builder.build());
    }
}

fn push_uls_equ_stab<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    let k = k_fi(document.consequence_class);
    let ed_net = combination_equ_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let demand = ed_net + k * (if is_bridge(&document.structure_kind) { 0.95 } else { 0.90 }) * actions.g_k_inf;
    let g_stab = k * (if is_bridge(&document.structure_kind) { 0.95 } else { 0.90 }) * actions.g_k_inf;
    let capacity = member.rd_equ_stab + g_stab;
    let path = member_path(document, &member.id, "rdEquStab");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.equ.stab.{}.{leading}", member.id),
        if is_bridge(&document.structure_kind) { "EN 1990 EQU A2.4(A)" } else { "EN 1990 EQU A1.2(A)" },
        ClauseId::new("EN 1990", "Table A1.2(A)", "EQU-stab"),
        subject.clone(),
        copy("EQU stabilising", "EQU stabilisierend"),
    )
    .utilization(q_force(demand), q_force(capacity))
    .annex(annex.choice())
    .explanation(copy(
        format!("Destabilising demand {demand:.1} N vs stabilising capacity {capacity:.1} N"),
        format!("Entstabilisierende Beanspruchung {demand:.1} N gegenüber stabilisierender Kapazität {capacity:.1} N"),
    ));
    if demand > capacity {
        let required_rd = (demand - g_stab).max(0.0);
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(member.rd_equ_stab),
            q_force(required_rd),
            copy(
                format!("Increase R_d,EQU (stabilising) of {} to at least {:.1} kN.", member.label_en, required_rd / 1000.0),
                format!("R_d,EQU (stabilisierend) von {} auf mindestens {:.1} kN erhöhen.", member.label_de, required_rd / 1000.0),
            ),
        ));
    }
    report.push(builder.build());
}

fn push_design_working_life(report: &mut CheckReport, document: &En1990Snapshot) {
    use crate::standards::v1::subsets::any::schema::design_working_life_indicative;
    let (lo, hi) = design_working_life_indicative(document.design_working_life_category);
    let years = document.design_working_life_years;
    let bridge = is_bridge(&document.structure_kind);
    let years_subject = SubjectRef::new(document.project_id.clone(), "designWorkingLifeYears", copy("Design working life", "Nutzungsdauer"));
    let cat_subject = SubjectRef::new(document.project_id.clone(), "designWorkingLifeCategory", copy("Design working life category", "Nutzungskategorie"));

    let years_ok = if bridge && document.design_working_life_category == 5 {
        (years - 100.0).abs() < 1e-6
    } else if bridge {
        false
    } else {
        years + 1e-9 >= lo && years - 1e-9 <= hi
    };
    let mut years_builder = CheckResult::assess(
        "en1990.table-2.1.design-working-life",
        "EN 1990 §2.3",
        ClauseId::new("EN 1990", "§2.3", "Table 2.1"),
        years_subject.clone(),
        copy("Design working life Table 2.1", "Nutzungsdauer Tabelle 2.1"),
    )
    .annex(document.annex)
    .explanation(copy(
        format!("Project {}: category {} indicative [{lo:.0}; {hi:.0}] a; declared {years:.0} a.", document.project_id, document.design_working_life_category),
        format!("Projekt {}: Kategorie {} indikativ [{lo:.0}; {hi:.0}] a; angegeben {years:.0} a.", document.project_id, document.design_working_life_category),
    ));
    if years_ok {
        years_builder = years_builder.status(crate::document::CheckStatus::Pass);
    } else {
        let required_years = if bridge { 100.0 } else { hi };
        years_builder = years_builder.status(crate::document::CheckStatus::Fail).remedy(Remedy::exactly(
            years_subject,
            q_dim(years),
            q_dim(required_years),
            copy(
                format!("Set design working life years to {required_years:.0} a."),
                format!("Nutzungsdauer auf {required_years:.0} a setzen."),
            ),
        ));
    }
    let mut years_built = years_builder.build();
    years_built.computed = q_dim(years);
    years_built.limit = q_dim(if bridge { 100.0 } else { hi });
    let lim = if bridge { 100.0 } else { hi }; years_built.utilization = if lim.abs() < f64::EPSILON { 0.0 } else { years / lim };
    report.push(years_built);

    let cat_ok = (1..=5).contains(&document.design_working_life_category) && (!bridge || document.design_working_life_category == 5);
    let mut cat_builder = CheckResult::assess(
        "en1990.table-2.1.category",
        "EN 1990 §2.3",
        ClauseId::new("EN 1990", "§2.3", "Table 2.1"),
        cat_subject.clone(),
        copy("Design working life category", "Nutzungskategorie"),
    )
    .annex(document.annex)
    .explanation(copy(
        format!("Project {}: declared category {}; bridges require Table 2.1 / A2 category 5 (100 a).", document.project_id, document.design_working_life_category),
        format!("Projekt {}: angegebene Kategorie {}; Brücken erfordern Tabelle 2.1 / A2 Kategorie 5 (100 a).", document.project_id, document.design_working_life_category),
    ));
    if cat_ok {
        cat_builder = cat_builder.status(crate::document::CheckStatus::Pass);
    } else {
        let target = if bridge { 5u8 } else { 4u8 };
        cat_builder = cat_builder.status(crate::document::CheckStatus::Fail).remedy(Remedy::one_of(
            cat_subject,
            vec![target.to_string()],
            copy(
                format!("Set design working life category to {target} (bridges: category 5)."),
                format!("Nutzungskategorie auf {target} setzen (Brücken: Kategorie 5)."),
            ),
        ));
    }
    let mut cat_built = cat_builder.build();
    cat_built.computed = q_dim(document.design_working_life_category as f64);
    cat_built.limit = q_dim(5.0);
    cat_built.utilization = document.design_working_life_category as f64 / 5.0;
    report.push(cat_built);
}

fn push_annex_b_dsl_il(report: &mut CheckReport, document: &En1990Snapshot) {
    use crate::standards::v1::subsets::any::schema::annex_b_dsl_il_for_rc;
    let (dsl_req, il_req) = annex_b_dsl_il_for_rc(document.reliability_class);
    for (id, path, declared, required, title_en, title_de) in [
        ("en1990.annex-b.dsl", "supervisionLevel", document.supervision_level.as_str(), dsl_req, "Design supervision level vs RC", "Überwachungsstufe gegenüber RC"),
        ("en1990.annex-b.il", "inspectionLevel", document.inspection_level.as_str(), il_req, "Inspection level vs RC", "Inspektionsstufe gegenüber RC"),
    ] {
        let subject = SubjectRef::new("", path, copy(title_en, title_de));
        let ok = declared == required;
        let mut builder = CheckResult::assess(id, "EN 1990 Annex B", ClauseId::new("EN 1990", "Annex B", path), subject.clone(), copy(title_en, title_de))
            .annex(document.annex)
            .explanation(copy(
                format!("Declared {declared} for RC{}; Annex B recommends {required}.", document.reliability_class),
                format!("Angegeben {declared} für RC{}; Anhang B empfiehlt {required}.", document.reliability_class),
            ));
        if ok {
            builder = builder.status(crate::document::CheckStatus::Pass);
        } else {
            builder = builder.status(crate::document::CheckStatus::Fail).remedy(Remedy::one_of(
                subject,
                vec![required.to_string()],
                copy(
                    format!("Set {path} to {required} for the reliability class."),
                    format!("{path} für die Zuverlässigkeitsklasse auf {required} setzen."),
                ),
            ));
        }
        let mut built = builder.build();
        built.computed = q_dim(if ok { 1.0 } else { 0.0 });
        built.limit = q_dim(1.0);
        built.utilization = if ok { 1.0 } else { 2.0 };
        report.push(built);
    }
}

fn push_a2_bridge_sls(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    if !is_bridge(&document.structure_kind) {
        report.push(
            CheckResult::assess(
                format!("en1990.a2.bridge.{}", member.id),
                "EN 1990 Annex A2",
                ClauseId::new("EN 1990", "Annex A2", "SLS"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "rdStr"), member_label(member)),
                copy("Annex A2 bridge SLS", "Anhang A2 Brücken-GZG"),
            )
            .not_applicable(copy(
                "Structure kind is not a bridge; Annex A2 SLS criteria live on bridgeSls[].",
                "Tragwerksart ist keine Brücke; Anhang-A2-GZG-Kriterien liegen auf bridgeSls[].",
            ))
            .annex(document.annex)
            .build(),
        );
        return;
    }
    let Some(sls) = document.bridge_sls.iter().find(|b| b.member_id == member.id) else {
        report.push(
            CheckResult::assess(
                format!("en1990.a2.bridge.{}", member.id),
                "EN 1990 Annex A2",
                ClauseId::new("EN 1990", "Annex A2", "SLS"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "rdStr"), member_label(member)),
                copy("Annex A2 bridge SLS", "Anhang A2 Brücken-GZG"),
            )
            .not_applicable(copy(
                format!("No bridgeSls entry for member {}.", member.id),
                format!("Kein bridgeSls-Eintrag für Bauteil {}.", member.id),
            ))
            .annex(document.annex)
            .build(),
        );
        return;
    };
    let sls_path = |field: &str| format!("bridgeSls[id={}].{field}", sls.id);
    if matches!(document.structure_kind.as_str(), "footbridge" | "rail_bridge") {
        let path = sls_path("deckAcceleration");
        let subject = SubjectRef::new(&sls.id, &path, member_label(member));
        let mut builder = CheckResult::assess(
            format!("en1990.a2.acceleration.{}.{}", member.id, sls.id),
            "EN 1990 Annex A2.4",
            ClauseId::new("EN 1990", "Annex A2.4", "acceleration"),
            subject.clone(),
            copy("Deck acceleration", "Deckbeschleunigung"),
        )
        .utilization(q_dim(sls.deck_acceleration), q_dim(sls.deck_acceleration_limit))
        .annex(document.annex)
        .explanation(copy(
            format!("a = {:.3} m/s² vs limit {:.3} m/s²", sls.deck_acceleration, sls.deck_acceleration_limit),
            format!("a = {:.3} m/s² gegenüber Grenze {:.3} m/s²", sls.deck_acceleration, sls.deck_acceleration_limit),
        ));
        if sls.deck_acceleration > sls.deck_acceleration_limit {
            builder = builder.remedy(Remedy::at_most(
                subject,
                q_dim(sls.deck_acceleration),
                q_dim(sls.deck_acceleration_limit),
                copy(
                    format!("Reduce deck acceleration to at most {:.3} m/s².", sls.deck_acceleration_limit),
                    format!("Deckbeschleunigung auf höchstens {:.3} m/s² reduzieren.", sls.deck_acceleration_limit),
                ),
            ));
        }
        report.push(builder.build());
    }
    if document.structure_kind == "rail_bridge" {
        let path = sls_path("deckTwist");
        let subject = SubjectRef::new(&sls.id, &path, member_label(member));
        let mut builder = CheckResult::assess(
            format!("en1990.a2.twist.{}.{}", member.id, sls.id),
            "EN 1990 Annex A2.4",
            ClauseId::new("EN 1990", "Annex A2.4", "twist"),
            subject.clone(),
            copy("Deck twist", "Deckverwindung"),
        )
        .utilization(q_dim(sls.deck_twist), q_dim(sls.deck_twist_limit))
        .annex(document.annex)
        .explanation(copy(
            format!("twist = {:.5} rad vs limit {:.5} rad", sls.deck_twist, sls.deck_twist_limit),
            format!("Verwindung = {:.5} rad gegenüber Grenze {:.5} rad", sls.deck_twist, sls.deck_twist_limit),
        ));
        if sls.deck_twist > sls.deck_twist_limit {
            builder = builder.remedy(Remedy::at_most(
                subject,
                q_dim(sls.deck_twist),
                q_dim(sls.deck_twist_limit),
                copy(
                    format!("Reduce deck twist to at most {:.5} rad.", sls.deck_twist_limit),
                    format!("Deckverwindung auf höchstens {:.5} rad reduzieren.", sls.deck_twist_limit),
                ),
            ));
        }
        report.push(builder.build());
    }
    if sls.bridge_deflection_limit > 0.0 {
        let path = sls_path("bridgeDeflection");
        let subject = SubjectRef::new(&sls.id, &path, member_label(member));
        let mut builder = CheckResult::assess(
            format!("en1990.a2.deflection.{}.{}", member.id, sls.id),
            "EN 1990 Annex A2.4",
            ClauseId::new("EN 1990", "Annex A2.4", "deflection"),
            subject.clone(),
            copy("Bridge vertical deflection", "Vertikale Brückendurchbiegung"),
        )
        .utilization(q_len(sls.bridge_deflection), q_len(sls.bridge_deflection_limit))
        .annex(document.annex)
        .explanation(copy(
            format!("δ = {:.1} mm vs limit {:.1} mm", sls.bridge_deflection * 1000.0, sls.bridge_deflection_limit * 1000.0),
            format!("δ = {:.1} mm gegenüber Grenze {:.1} mm", sls.bridge_deflection * 1000.0, sls.bridge_deflection_limit * 1000.0),
        ));
        if sls.bridge_deflection > sls.bridge_deflection_limit {
            builder = builder.remedy(Remedy::at_most(
                subject,
                q_len(sls.bridge_deflection),
                q_len(sls.bridge_deflection_limit),
                copy(
                    format!("Reduce bridge deflection to at most {:.1} mm.", sls.bridge_deflection_limit * 1000.0),
                    format!("Brückendurchbiegung auf höchstens {:.1} mm reduzieren.", sls.bridge_deflection_limit * 1000.0),
                ),
            ));
        }
        report.push(builder.build());
    }
}

fn push_k_fi(report: &mut CheckReport, document: &En1990Snapshot) {
    let expected = k_fi(document.consequence_class);
    let declared = document.k_fi_declared;
    let subject = SubjectRef::new(document.project_id.clone(), "kFiDeclared", copy(format!("Declared K_FI ({})", document.project_id), format!("Deklariertes K_FI ({})", document.project_id)));
    let mut builder = CheckResult::assess(
        "en1990.annex-b.kfi",
        "EN 1990 Annex B",
        ClauseId::new("EN 1990", "Annex B", "K_FI"),
        subject.clone(),
        copy("Declared K_FI vs consequence class", "Deklariertes K_FI gegenüber Schadensfolgeklasse"),
    )
    .status(if (declared - expected).abs() < 1e-9 { crate::document::CheckStatus::Pass } else { crate::document::CheckStatus::Fail })
    .annex(document.annex)
    .explanation(copy(
        format!("Declared K_FI = {declared:.2}; tabulated for CC{} = {expected:.2}", document.consequence_class),
        format!("Deklariertes K_FI = {declared:.2}; tabellarisch für CC{} = {expected:.2}", document.consequence_class),
    ));
    if (declared - expected).abs() >= 1e-9 {
        builder = builder.remedy(Remedy::exactly(
            subject,
            q_dim(declared),
            q_dim(expected),
            copy(
                format!("Set declared K_FI to {expected:.2} for CC{}.", document.consequence_class),
                format!("Deklariertes K_FI für CC{} auf {expected:.2} setzen.", document.consequence_class),
            ),
        ));
    }
    let mut built = builder.build();
    built.computed = q_dim(declared);
    built.limit = q_dim(expected);
    built.utilization = if expected.abs() < f64::EPSILON { 0.0 } else { declared / expected };
    report.push(built);
}

fn push_geo_set_c<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    if !is_bridge(&document.structure_kind) {
        return;
    }
    let ed = combination_geo_a24c(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let rd = member.rd_geo;
    let path = member_path(document, &member.id, "rdGeo");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.geo.set-c.{}.{leading}", member.id),
        "EN 1990 GEO A2.4(C)",
        ClauseId::new("EN 1990", "Annex A2.4(C)", "GEO"),
        subject.clone(),
        copy("GEO set C", "GEO Satz C"),
    )
    .utilization(q_force(ed), q_force(rd))
    .annex(annex.choice())
    .explanation(copy(
        format!("GEO set C E_d = {ed:.1} N vs R_d,GEO = {rd:.1} N"),
        format!("GEO Satz C E_d = {ed:.1} N gegenüber R_d,GEO = {rd:.1} N"),
    ));
    if ed > rd {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(rd),
            q_force(ed),
            copy(
                format!("Increase R_d,GEO to at least {:.1} kN for set C.", ed / 1000.0),
                format!("R_d,GEO für Satz C auf mindestens {:.1} kN erhöhen.", ed / 1000.0),
            ),
        ));
    }
    report.push(builder.build());
}

/// 📋️ `En1990Snapshot -> CheckReport` — full basis-of-design conformance.

fn action_ids(document: &En1990Snapshot) -> Vec<String> {
    let mut ids = Vec::new();
    ids.extend(document.permanents.iter().map(|a| a.id.clone()));
    ids.extend(document.variables.iter().map(|a| a.id.clone()));
    ids.extend(document.accidentals.iter().map(|a| a.id.clone()));
    ids.extend(document.seismics.iter().map(|a| a.id.clone()));
    ids
}

fn member_ids(document: &En1990Snapshot) -> Vec<String> {
    document.members.iter().map(|m| m.id.clone()).collect()
}

fn push_duplicate_ids(report: &mut CheckReport, document: &En1990Snapshot, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, copy(format!("Duplicate {table} id"), format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let mut builder = CheckResult::assess(
            format!("en1990.integrity.duplicate.{table}.{id}"),
            "EN 1990 integrity",
            ClauseId::new("EN 1990", "§2", "id"),
            subject.clone(),
            copy(format!("Unique {table} id"), format!("Eindeutige {table}-Id")),
        )
        .annex(document.annex)
        .explanation(copy(
            format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
            format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
        ))
        .status(crate::document::CheckStatus::Fail);
        builder = builder.remedy(Remedy::one_of(
            subject,
            if options.is_empty() { vec![format!("{id}-unique")] } else { options },
            copy(
                format!("Rename the duplicated '{id}' entry to a free id."),
                format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
            ),
        ));
        report.push(builder.build());
    }
}

fn push_dangling_ref(report: &mut CheckReport, document: &En1990Snapshot, check_id: String, path: String, subject_id: &str, label_en: &str, label_de: &str, current: &str, options: Vec<String>) {
    let subject = SubjectRef::new(subject_id, &path, copy(label_en, label_de));
    let opts = if options.is_empty() { vec![format!("{current}-missing")] } else { options };
    report.push(
        CheckResult::assess(
            check_id,
            "EN 1990 integrity",
            ClauseId::new("EN 1990", "§2", "reference"),
            subject.clone(),
            copy(format!("Referential integrity — {label_en}"), format!("Referenzintegrität — {label_de}")),
        )
        .annex(document.annex)
        .explanation(copy(
            format!("'{current}' does not reference an existing target; dependent combination checks for this row must not Pass."),
            format!("'{current}' verweist auf kein vorhandenes Ziel; abhängige Kombinationsnachweise für diese Zeile dürfen nicht bestehen."),
        ))
        .status(crate::document::CheckStatus::Fail)
        .remedy(Remedy::one_of(
            subject,
            opts,
            copy(
                format!("Set {label_en} to one of the existing target ids."),
                format!("{label_de} auf eine der vorhandenen Ziel-Ids setzen."),
            ),
        ))
        .build(),
    );
}

/// 🔗 Referential integrity + unique entity ids (CORRECTION 14:42). Returns members that must not silently Pass combination checks.
fn push_referential_integrity(report: &mut CheckReport, document: &En1990Snapshot) -> std::collections::BTreeSet<String> {
    let actions = action_ids(document);
    let members = member_ids(document);
    let action_set: std::collections::BTreeSet<&str> = actions.iter().map(String::as_str).collect();
    let member_set: std::collections::BTreeSet<&str> = members.iter().map(String::as_str).collect();
    let mut broken = std::collections::BTreeSet::new();

    push_duplicate_ids(report, document, "permanents", &document.permanents.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("permanents[id={id}].id"));
    push_duplicate_ids(report, document, "variables", &document.variables.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("variables[id={id}].id"));
    push_duplicate_ids(report, document, "accidentals", &document.accidentals.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("accidentals[id={id}].id"));
    push_duplicate_ids(report, document, "seismics", &document.seismics.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("seismics[id={id}].id"));
    push_duplicate_ids(report, document, "members", &members, &|id| format!("members[id={id}].id"));
    push_duplicate_ids(report, document, "bridgeSls", &document.bridge_sls.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("bridgeSls[id={id}].id"));

    for (i, effect) in document.effects.iter().enumerate() {
        let base = format!("effects[{i}]");
        if !member_set.contains(effect.member_id.as_str()) {
            broken.insert(effect.member_id.clone());
            push_dangling_ref(report, document, format!("en1990.integrity.effects.{i}.memberId"), format!("{base}.memberId"), &effect.member_id, "Effect memberId", "Wirkung memberId", &effect.member_id, members.clone());
        }
        if !action_set.contains(effect.action_id.as_str()) {
            broken.insert(effect.member_id.clone());
            push_dangling_ref(report, document, format!("en1990.integrity.effects.{i}.actionId"), format!("{base}.actionId"), &effect.action_id, "Effect actionId", "Wirkung actionId", &effect.action_id, actions.clone());
        }
    }

    for sls in &document.bridge_sls {
        if !member_set.contains(sls.member_id.as_str()) {
            broken.insert(sls.member_id.clone());
            push_dangling_ref(
                report,
                document,
                format!("en1990.integrity.bridgeSls.{}.memberId", sls.id),
                format!("bridgeSls[id={}].memberId", sls.id),
                &sls.member_id,
                "Bridge SLS memberId",
                "Brücken-SLS memberId",
                &sls.member_id,
                members.clone(),
            );
        }
    }

    // Members with no wired effects while the catalogue has effects are orphaned (e.g. renamed id).
    if !document.effects.is_empty() {
        for m in &document.members {
            let wired = document.effects.iter().any(|e| e.member_id == m.id && action_set.contains(e.action_id.as_str()));
            if !wired {
                broken.insert(m.id.clone());
            }
        }
    }
    broken
}

fn push_member_unassessable(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    let path = member_path(document, &member.id, "id");
    report.push(
        CheckResult::assess(
            format!("en1990.integrity.member.{}.unassessable", member.id),
            "EN 1990 integrity",
            ClauseId::new("EN 1990", "§2", "reference"),
            SubjectRef::new(&member.id, &path, member_label(member)),
            copy("Member effects wiring", "Bauteil-Wirkungseinbindung"),
        )
        .annex(document.annex)
        .explanation(copy(
            format!("Member '{}' has broken or missing action wiring; ULS/SLS combination checks are not assessed as Pass.", member.id),
            format!("Bauteil '{}' hat fehlerhafte oder fehlende Wirkungseinbindung; ULS/SLS-Kombinationsnachweise gelten nicht als bestanden.", member.id),
        ))
        .status(crate::document::CheckStatus::Fail)
        .remedy(Remedy::one_of(
            SubjectRef::new(&member.id, &path, member_label(member)),
            member_ids(document),
            copy(
                "Repair effects[].memberId / effects[].actionId to existing catalogue ids.",
                "effects[].memberId / effects[].actionId auf vorhandene Katalog-Ids reparieren.",
            ),
        ))
        .build(),
    );
}

pub fn evaluate(document: &En1990Snapshot) -> CheckReport {
    let annex: NationalAnnexes = if document.annex == AnnexChoice::De { NaDe.into() } else { NaEn.into() };
    let mut report = CheckReport::default();
    if document.members.is_empty() {
        report.push(
            CheckResult::assess("en1990.members", "EN 1990", ClauseId::new("EN 1990", "§2", "members"), project_subject(document), copy("Members", "Bauteile"))
                .not_applicable(copy("No members / verification points defined.", "Keine Bauteile / Nachweisstellen definiert."))
                .annex(document.annex)
                .build(),
        );
        return report;
    }
    let broken_members = push_referential_integrity(&mut report, document);
    push_k_fi(&mut report, document);
    push_design_working_life(&mut report, document);
    push_annex_b_dsl_il(&mut report, document);
    report.push(check_reliability_index_for(
        document.beta_computed,
        document.consequence_class,
        document.reliability_class,
        document.reference_period_years,
        document.annex,
    ));
    for member in &document.members {
        if broken_members.contains(&member.id) {
            push_member_unassessable(&mut report, document, member);
            continue;
        }
        let actions = action_set_for_member(document, &member.id);
        let n_leading = actions.q_k.len().max(1);
        for leading in 0..n_leading {
            if actions.q_k.is_empty() && leading > 0 {
                break;
            }
            push_uls_str_geo(&mut report, &annex, document, member, &actions, leading);
            push_uls_geo(&mut report, &annex, document, member, &actions, leading);
            push_geo_set_c(&mut report, &annex, document, member, &actions, leading);
            push_uls_equ(&mut report, &annex, document, member, &actions, leading);
            push_uls_equ_stab(&mut report, &annex, document, member, &actions, leading);
            push_sls_action_effects(&mut report, &annex, document, member, &actions, leading);
            push_accidental(&mut report, &annex, document, member, &actions, leading);
        }
        push_seismic(&mut report, &annex, document, member, &actions);
        push_sls_deflection(&mut report, document, member);
        push_sls_vibration(&mut report, document, member);
        push_fat(&mut report, &annex, document, member, &actions);
        push_a2_bridge_sls(&mut report, document, member);
    }
    report
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
pub use super::outline::En1990Outline;
//#endregion 🔁️Re-exports
