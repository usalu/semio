//! 💡️ En1994 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::En1994Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1994 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1994.inference")]
pub struct En1994Inference {
    #[derived]
    pub outline: En1994Outline,
}

impl protocol::Inference<En1994Snapshot> for En1994Inference {
    fn infer(snapshot: &En1994Snapshot) -> Self {
        Self { outline: En1994Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1994Snapshot> for En1994Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1994.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1994.inference.outline", reads: &["annex", "structureKind", "steelFYPa", "beams", "columns", "slabs", "fireRating", "insulationThicknessM", "fatigueDetail", "beams.actions", "columns.actions", "slabs.actions"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1994Builder {
    type Snapshot = En1994Snapshot;
    type Inference = En1994Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1994.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1994_artifact_schema_descriptor`'s registration.
pub fn en1994_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1994.inference",
        inference: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests


//#region 🔖️ComplianceReport
/// ©️ En1994Snapshot → CheckReport — complete composite structure assessment.
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use crate::standards::v1::subsets::any::schema::{part_1_1, part_1_2, part_2, part_column, part_en1990, part_slab, AnnexParams};
use crate::{CompositeBeam, CompositeColumn, CompositeSlab, SteelSection};

fn lc(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn beam_label(beam: &CompositeBeam) -> LocalizedCopy {
    lc(format!("Composite beam {}", beam.id), format!("Verbundträger {}", beam.id))
}
fn beam_path(beam: &CompositeBeam, leaf: &str) -> String {
    format!("beams[id={}].{leaf}", beam.id)
}
fn beam_ref(beam: &CompositeBeam, leaf: &str) -> SubjectRef {
    SubjectRef::new(beam.id.clone(), beam_path(beam, leaf), beam_label(beam))
}
fn col_path(col: &CompositeColumn, leaf: &str) -> String {
    format!("columns[id={}].{leaf}", col.id)
}
fn slab_path(slab: &CompositeSlab, leaf: &str) -> String {
    format!("slabs[id={}].{leaf}", slab.id)
}

fn evaluate_beam(report: &mut CheckReport, doc: &En1994Snapshot, beam: &CompositeBeam) {
    let annex = doc.annex;
    let f_y = doc.steel_f_y_pa;
    let mut beam = beam.clone();
    beam.steel = SteelSection::resolve(&beam.steel);
    let b0 = beam.steel.width_m;
    let b_eff = part_1_1::effective_width_m(beam.span_m, b0, beam.spacing_m);
    let b_uncapped = part_1_1::uncapped_effective_width_m(beam.span_m, b0);

    let uls = part_en1990::uls_composite(&beam.actions, beam.span_m, &beam.support, beam.spacing_m, annex);
    let uls_c = part_en1990::uls_construction(&beam.actions, beam.span_m, &beam.support, beam.spacing_m, annex);
    let sls_char = part_en1990::sls_characteristic(&beam.actions, beam.span_m, &beam.support, beam.spacing_m, annex);
    let sls_freq = part_en1990::sls_frequent(&beam.actions, beam.span_m, &beam.support, beam.spacing_m, annex);
    let sls_qp = part_en1990::sls_quasi_permanent(&beam.actions, beam.span_m, &beam.support, beam.spacing_m, annex);
    let m_ed = uls.m_nm;
    let v_ed = uls.v_n;

    let mut beff = CheckResult::assess(
        format!("en1994.5.4.1.2.beff.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§5.4.1.2", "5.4.1.2"), beam_ref(&beam, "spacingM"),
        lc("Effective width b_eff vs beam spacing", "Mitwirkende Breite b_eff gegen Trägerabstand"),
    )
    .utilization(Quantity::length_m(b_uncapped), Quantity::length_m(beam.spacing_m))
    .annex(annex)
    .explanation(lc(
        format!("Uncapped 2·be1 = {b_uncapped:.3} m, spacing = {:.3} m → b_eff = {b_eff:.3} m.", beam.spacing_m),
        format!("Unbegrenzt 2·be1 = {b_uncapped:.3} m, Abstand = {:.3} m → b_eff = {b_eff:.3} m.", beam.spacing_m),
    ));
    if b_uncapped > beam.spacing_m {
        beff = beff.remedy(Remedy::at_least(beam_ref(&beam, "spacingM"), Quantity::length_m(beam.spacing_m), Quantity::length_m(b_uncapped),
            lc(format!("Increase spacing to ≥ {b_uncapped:.3} m."), format!("Trägerabstand auf ≥ {b_uncapped:.3} m erhöhen."))));
    }
    report.push(beff.build());

    // Section classification uses tw/tf
    let class = part_1_1::section_class(&beam.steel, f_y);
    let mut cls = CheckResult::assess(
        format!("en1994.5.5.class.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§5.5 / EN 1993-1-1 Table 5.2", "5.5"),
        beam_ref(&beam, "steel.twM"),
        lc("Steel section classification", "Querschnittsklasse Stahlprofil"),
    )
    .utilization(Quantity::new(QuantityKind::Dimensionless, class as f64), Quantity::new(QuantityKind::Dimensionless, 3.0))
    .annex(annex)
    .explanation(lc(
        format!("Section class {class} from c/t_f and h_w/t_w (t_w={:.1} mm, t_f={:.1} mm).", beam.steel.tw_m * 1000.0, beam.steel.tf_m * 1000.0),
        format!("Querschnittsklasse {class} aus c/t_f und h_w/t_w (t_w={:.1} mm, t_f={:.1} mm).", beam.steel.tw_m * 1000.0, beam.steel.tf_m * 1000.0),
    ));
    if class > 3 {
        cls = cls.remedy(Remedy::at_least(beam_ref(&beam, "steel.twM"), Quantity::length_m(beam.steel.tw_m), Quantity::length_m(beam.steel.tw_m * 1.2),
            lc("Increase web/flange thickness to improve class.", "Steg-/Flanschdicke erhöhen, um die Klasse zu verbessern.")));
    }
    report.push(cls.build());

    let m_pla = part_1_1::steel_mpl_from_plates_nm(&beam.steel, f_y, AnnexParams::for_annex(annex).gamma_m0)
        .max(part_1_1::steel_plastic_moment_nm(beam.steel.w_pl_y_m3, f_y, AnnexParams::for_annex(annex).gamma_m0));
    let m_pl_rd = part_1_1::full_plastic_moment_nm(&beam, b_eff, f_y, annex);
    let n_req = part_1_1::n_f_req(&beam, b_eff, f_y, annex);
    let n_layout = part_1_1::studs_in_shear_span(&beam);
    let n_f = beam.studs.total_count.max(n_layout);
    let eta = part_1_1::shear_connection_degree(n_f, n_req);
    let m_rd = part_1_1::plastic_moment_partial_nm(m_pla, m_pl_rd, eta);

    let mut bending = CheckResult::assess(
        format!("en1994.6.2.1.3.mrd.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§6.2.1.3", "6.2.1.3"), beam_ref(&beam, "actions"),
        lc("Composite plastic bending resistance", "Plastische Verbund-Biegetragfähigkeit"),
    )
    .utilization(Quantity::new(QuantityKind::Moment, m_ed), Quantity::new(QuantityKind::Moment, m_rd))
    .annex(annex)
    .explanation(lc(
        format!("Governing {}: M_Ed = {:.1} kNm, M_Rd(η={eta:.2}, b_eff={b_eff:.3} m) = {:.1} kNm.", uls.label_en, m_ed / 1e3, m_rd / 1e3),
        format!("Maßgebend {}: M_Ed = {:.1} kNm, M_Rd(η={eta:.2}, b_eff={b_eff:.3} m) = {:.1} kNm.", uls.label_de, m_ed / 1e3, m_rd / 1e3),
    ));
    if m_ed > m_rd {
        bending = bending
            .remedy(Remedy::at_most(beam_ref(&beam, "actions[id=Q-office].qAreaPa"), Quantity::new(QuantityKind::Dimensionless, 2.0e3), Quantity::new(QuantityKind::Dimensionless, 2.0e3 * m_rd / m_ed.max(1.0)),
                lc("Reduce imposed floor load.", "Nutzlast reduzieren.")))
            .remedy(Remedy::at_least(beam_ref(&beam, "slabThicknessM"), Quantity::length_m(beam.slab_thickness_m), Quantity::length_m((beam.slab_thickness_m * m_ed / m_rd.max(1.0)).min(0.30)),
                lc("Increase slab thickness.", "Plattendicke erhöhen.")));
        let heavier = SteelSection::heavier_heb_options(&beam.steel.designation);
        if !heavier.is_empty() {
            bending = bending.remedy(Remedy::one_of(beam_ref(&beam, "steel.designation"), heavier,
                lc("Select a heavier steel section from the catalogue.", "Schwereren Stahlquerschnitt aus dem Katalog wählen.")));
        }
    }
    report.push(bending.build());


    // Construction-stage steel-alone demand (EN 1994-1-1 §9.3 / §6.2) — reads construction actions.
    let m_c = uls_c.m_nm;
    let mut cst = CheckResult::assess(
        format!("en1994.6.2.1.construction.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§6.2.1 / §9.3", "6.2.1"), beam_ref(&beam, "actions"),
        lc("Construction-stage steel bending", "Biegung Stahl im Bauzustand"),
    )
    .utilization(Quantity::new(QuantityKind::Moment, m_c), Quantity::new(QuantityKind::Moment, m_pla))
    .annex(annex)
    .explanation(lc(
        format!("Construction {}: M_Ed,c = {:.1} kNm ({}), M_pl,a,Rd = {:.1} kNm.", uls_c.label_en, m_c / 1e3, beam.construction, m_pla / 1e3),
        format!("Herstellung {}: M_Ed,c = {:.1} kNm ({}), M_pl,a,Rd = {:.1} kNm.", uls_c.label_de, m_c / 1e3, beam.construction, m_pla / 1e3),
    ));
    if m_c > m_pla {
        cst = cst.remedy(Remedy::at_most(beam_ref(&beam, "actions[id=Q-constr].qAreaPa"), Quantity::new(QuantityKind::Dimensionless, 1.0e3), Quantity::new(QuantityKind::Dimensionless, 1.0e3 * m_pla / m_c.max(1.0)),
            lc("Reduce construction load.", "Belastung im Bauzustand reduzieren.")));
    }
    report.push(cst.build());

    // Stud spacing §6.6.5.5
    let (s_min, s_max) = part_1_1::stud_spacing_limits_m(&beam);
    let mut ssp = CheckResult::assess(
        format!("en1994.6.6.5.5.spacing.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§6.6.5.5", "6.6.5.5"), beam_ref(&beam, "studs.spacingM"),
        lc("Stud longitudinal spacing limits", "Grenzwerte Bolzenlängsabstand"),
    )
    .utilization(Quantity::length_m(beam.studs.spacing_m), Quantity::length_m(s_max))
    .annex(annex)
    .explanation(lc(
        format!("s = {:.0} mm; limits {:.0}–{:.0} mm; layout n_f = {n_layout} (declared {}).", beam.studs.spacing_m * 1000.0, s_min * 1000.0, s_max * 1000.0, beam.studs.total_count),
        format!("s = {:.0} mm; Grenzen {:.0}–{:.0} mm; Anordnung n_f = {n_layout} (deklariert {}).", beam.studs.spacing_m * 1000.0, s_min * 1000.0, s_max * 1000.0, beam.studs.total_count),
    ));
    if beam.studs.spacing_m > s_max || beam.studs.spacing_m < s_min {
        ssp = ssp.remedy(Remedy::at_most(beam_ref(&beam, "studs.spacingM"), Quantity::length_m(beam.studs.spacing_m), Quantity::length_m(s_max.min(beam.studs.spacing_m.max(s_min))),
            lc(format!("Set stud spacing within {:.0}–{:.0} mm.", s_min * 1000.0, s_max * 1000.0), format!("Bolzenabstand auf {:.0}–{:.0} mm einstellen.", s_min * 1000.0, s_max * 1000.0))));
    }
    report.push(ssp.build());

    let p_rd = part_1_1::connector_resistance_n(&beam, annex);
    let n_cf = {
        let p = AnnexParams::for_annex(annex);
        let n_a = beam.steel.a_m2 * f_y / p.gamma_m0;
        let h_c = (beam.slab_thickness_m - beam.sheeting.height_m).max(0.04);
        let n_c = 0.85 * (beam.concrete_f_ck_pa / p.gamma_c) * b_eff * h_c;
        n_a.min(n_c)
    };
    let v_l_ed = n_cf; // force to transfer in critical shear span
    let v_ed_stud = v_l_ed / n_f.max(1) as f64;
    let mut stud = CheckResult::assess(
        format!("en1994.6.6.3.1.prd.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§6.6.3.1", "6.6.3.1"), beam_ref(&beam, "studs.diameterM"),
        lc("Headed stud resistance P_Rd", "Tragfähigkeit Kopfbolzen P_Rd"),
    )
    .utilization(Quantity::new(QuantityKind::Force, v_ed_stud), Quantity::new(QuantityKind::Force, p_rd))
    .annex(annex)
    .explanation(lc(
        format!("V_L,Ed/n_f = {:.1} kN (n_f={n_f} from spacing), P_Rd = {:.1} kN.", v_ed_stud / 1e3, p_rd / 1e3),
        format!("V_L,Ed/n_f = {:.1} kN (n_f={n_f} aus Abstand), P_Rd = {:.1} kN.", v_ed_stud / 1e3, p_rd / 1e3),
    ));
    if v_ed_stud > p_rd {
        let n_need = ((v_l_ed / p_rd).ceil() as u32).max(n_f + 1);
        stud = stud.remedy(Remedy::at_least(beam_ref(&beam, "studs.totalCount"), Quantity::new(QuantityKind::Dimensionless, beam.studs.total_count as f64), Quantity::new(QuantityKind::Dimensionless, n_need as f64),
            lc(format!("Increase stud count to ≥ {n_need}."), format!("Bolzenanzahl auf ≥ {n_need} erhöhen."))));
        stud = stud.remedy(Remedy::at_most(beam_ref(&beam, "studs.spacingM"), Quantity::length_m(beam.studs.spacing_m), Quantity::length_m((beam.span_m / 2.0) / (n_need as f64 / beam.studs.count_per_rib.max(1) as f64).max(1.0)),
            lc("Tighten stud spacing.", "Bolzenabstand verringern.")));
    }
    report.push(stud.build());

    let eta_min = part_1_1::min_shear_connection_degree(beam.span_m, f_y);
    let mut eta_chk = CheckResult::assess(
        format!("en1994.6.6.1.2.etamin.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§6.6.1.2", "6.6.1.2"), beam_ref(&beam, "studs.spacingM"),
        lc("Minimum degree of shear connection", "Mindestverdübelungsgrad"),
    )
    .minimum(Quantity::new(QuantityKind::Dimensionless, eta), Quantity::new(QuantityKind::Dimensionless, eta_min))
    .annex(annex)
    .explanation(lc(format!("η = {eta:.3} (n_f={n_f}), η_min = {eta_min:.3}."), format!("η = {eta:.3} (n_f={n_f}), η_min = {eta_min:.3}.")));
    if eta < eta_min {
        let n_need = ((eta_min * n_req as f64).ceil() as u32).max(n_f + 1);
        eta_chk = eta_chk.remedy(Remedy::at_least(beam_ref(&beam, "studs.totalCount"), Quantity::new(QuantityKind::Dimensionless, beam.studs.total_count as f64), Quantity::new(QuantityKind::Dimensionless, n_need as f64),
            lc(format!("Provide ≥ {n_need} studs so η ≥ η_min."), format!("≥ {n_need} Bolzen vorsehen, damit η ≥ η_min."))));
    }
    report.push(eta_chk.build());

    let v_pl = part_1_1::vertical_shear_resistance_n(&beam, f_y, annex);
    let sb_util = part_1_1::shear_buckling_util(&beam.steel, f_y);
    let mut vchk = CheckResult::assess(
        format!("en1994.6.2.2.vpl.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§6.2.2", "6.2.2"), beam_ref(&beam, "steel.twM"),
        lc("Vertical shear resistance", "Querkrafttragfähigkeit"),
    )
    .utilization(Quantity::new(QuantityKind::Force, v_ed), Quantity::new(QuantityKind::Force, v_pl))
    .annex(annex)
    .explanation(lc(
        format!("Governing {}: V_Ed = {:.1} kN, V_pl,Rd = {:.1} kN (A_v from t_w), h_w/t_w util = {sb_util:.2}.", uls.label_en, v_ed / 1e3, v_pl / 1e3),
        format!("Maßgebend {}: V_Ed = {:.1} kN, V_pl,Rd = {:.1} kN (A_v aus t_w), h_w/t_w-Ausn. = {sb_util:.2}.", uls.label_de, v_ed / 1e3, v_pl / 1e3),
    ));
    if v_ed > v_pl || sb_util > 1.0 {
        vchk = vchk.remedy(Remedy::at_least(beam_ref(&beam, "steel.twM"), Quantity::length_m(beam.steel.tw_m), Quantity::length_m(beam.steel.tw_m * (v_ed / v_pl.max(1.0)).max(sb_util)),
            lc("Increase web thickness t_w.", "Stegdicke t_w erhöhen.")));
    }
    report.push(vchk.build());

    let v_l_rd = part_1_1::longitudinal_shear_resistance_n(&beam, b_eff, annex);
    let mut vl = CheckResult::assess(
        format!("en1994.6.6.6.vlrd.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§6.6.6", "6.6.6"), beam_ref(&beam, "transverseAsM2PerM"),
        lc("Longitudinal shear in slab", "Längsschub in der Platte"),
    )
    .utilization(Quantity::new(QuantityKind::Force, v_l_ed), Quantity::new(QuantityKind::Force, v_l_rd))
    .annex(annex)
    .explanation(lc(format!("V_L,Ed = {:.1} kN, V_L,Rd = {:.1} kN.", v_l_ed / 1e3, v_l_rd / 1e3), format!("V_L,Ed = {:.1} kN, V_L,Rd = {:.1} kN.", v_l_ed / 1e3, v_l_rd / 1e3)));
    if v_l_ed > v_l_rd {
        let as_need = beam.transverse_as_m2_per_m * v_l_ed / v_l_rd.max(1.0);
        vl = vl.remedy(Remedy::at_least(beam_ref(&beam, "transverseAsM2PerM"), Quantity::new(QuantityKind::Area, beam.transverse_as_m2_per_m), Quantity::new(QuantityKind::Area, as_need),
            lc(format!("Increase transverse reinforcement to ≥ {:.0} mm²/m.", as_need * 1e6), format!("Querbewehrung auf ≥ {:.0} mm²/m erhöhen.", as_need * 1e6))));
    }
    report.push(vl.build());

    // LTB from construction / hogging combinations
    let m_b = part_1_1::ltb_moment_resistance_nm(&beam, f_y, annex);
    let construction_applies = beam.construction == "unpropped" && uls_c.m_nm > 1.0;
    let hogging_applies = uls.m_hog_nm > 1.0;
    if construction_applies || hogging_applies {
        let (m_ltb_ed, leaf, stage_en, stage_de, comb_en, comb_de) = if construction_applies && (!hogging_applies || uls_c.m_nm >= uls.m_hog_nm) {
            (uls_c.m_nm, "construction", "construction stage (steel alone)", "Herstellung (nur Stahl)", uls_c.label_en.clone(), uls_c.label_de.clone())
        } else {
            (uls.m_hog_nm, "support", "hogging", "Stützbereich", uls.label_en.clone(), uls.label_de.clone())
        };
        let mut ltb = CheckResult::assess(
            format!("en1994.6.4.ltb.{}", beam.id), "DIN EN 1994-1-1",
            ClauseId::new("EN 1994-1-1", "§6.4", "6.4"), beam_ref(&beam, leaf),
            lc("Lateral-torsional buckling", "Biegedrillknicken"),
        )
        .utilization(Quantity::new(QuantityKind::Moment, m_ltb_ed), Quantity::new(QuantityKind::Moment, m_b))
        .annex(annex)
        .explanation(lc(
            format!("LTB ({stage_en}) via {comb_en}: M_Ed = {:.1} kNm, M_b,Rd = {:.1} kNm.", m_ltb_ed / 1e3, m_b / 1e3),
            format!("Biegedrillknicken ({stage_de}) über {comb_de}: M_Ed = {:.1} kNm, M_b,Rd = {:.1} kNm.", m_ltb_ed / 1e3, m_b / 1e3),
        ));
        if m_ltb_ed > m_b {
            ltb = ltb.remedy(Remedy::at_most(beam_ref(&beam, "ltbLengthM"), Quantity::length_m(beam.ltb_length_m), Quantity::length_m(beam.ltb_length_m * m_b / m_ltb_ed.max(1.0)),
                lc("Reduce LTB length (add restraints).", "Biegedrillknicklänge verkürzen.")));
        }
        report.push(ltb.build());
    } else {
        report.push(CheckResult::assess(
            format!("en1994.6.4.ltb.{}", beam.id), "DIN EN 1994-1-1",
            ClauseId::new("EN 1994-1-1", "§6.4", "6.4"), beam_ref(&beam, "construction"),
            lc("Lateral-torsional buckling", "Biegedrillknicken"),
        ).not_applicable(lc("§6.4 not applicable: propped construction without hogging demand.", "§6.4 nicht anwendbar: mit Hilfsstützen ohne Stützmoment."))
        .annex(annex).build());
    }

    // SLS characteristic — stress limit EN 1994-1-1 §7.2.2
    let w_el = (beam.steel.i_y_m4 / (beam.steel.height_m / 2.0).max(1e-6)).max(1e-9);
    let sigma_a = sls_char.m_nm.abs() / w_el;
    let sigma_lim = 0.9 * f_y;
    let mut stress = CheckResult::assess(
        format!("en1994.7.2.2.stress.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§7.2.2", "7.2.2"), beam_ref(&beam, "actions"),
        lc("SLS characteristic steel stress", "GZG charakteristisch Stahlspannung"),
    )
    .utilization(Quantity::new(QuantityKind::Pressure, sigma_a), Quantity::new(QuantityKind::Pressure, sigma_lim))
    .annex(annex)
    .explanation(lc(
        format!("{}: σ_a = {:.1} MPa ≤ 0.9 f_y = {:.1} MPa.", sls_char.label_en, sigma_a / 1e6, sigma_lim / 1e6),
        format!("{}: σ_a = {:.1} MPa ≤ 0.9 f_y = {:.1} MPa.", sls_char.label_de, sigma_a / 1e6, sigma_lim / 1e6),
    ));
    if sigma_a > sigma_lim {
        let q_owned = if beam.actions.iter().any(|a| a.id == "Q-office") {
            "actions[id=Q-office].qAreaPa".to_string()
        } else if let Some(a) = beam.actions.iter().find(|a| matches!(a.kind.as_str(), "imposed" | "construction")) {
            format!("actions[id={}].qAreaPa", a.id)
        } else {
            "spanM".into()
        };
        stress = stress.remedy(Remedy::at_most(beam_ref(&beam, &q_owned), Quantity::new(QuantityKind::Dimensionless, sigma_a), Quantity::new(QuantityKind::Dimensionless, sigma_lim),
            lc("Reduce load or span so σ_a ≤ 0.9 f_y.", "Last oder Spannweite reduzieren, damit σ_a ≤ 0.9 f_y.")));
    }
    report.push(stress.build());

    // SLS frequent — deflection EN 1994-1-1 §7.3.1 / DE NA
    let n_l = part_1_1::modular_ratio_long_term(beam.concrete_e_cm_pa, 2.0);
    let i_eff = beam.steel.i_y_m4 * (1.0 + 0.35 * n_l / 10.0);
    let delta = 5.0 * sls_freq.m_nm * beam.span_m.powi(2) / (48.0 * 210e9 * i_eff.max(1e-12));
    let delta_lim = part_1_1::deflection_limit_m(beam.span_m);
    let mut sls = CheckResult::assess(
        format!("en1994.7.3.1.deflection.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§7.3.1", "7.3.1"), beam_ref(&beam, "actions"),
        lc("SLS frequent deflection", "Durchbiegung GZG häufig"),
    )
    .utilization(Quantity::length_m(delta), Quantity::length_m(delta_lim))
    .annex(annex)
    .explanation(lc(
        format!("{}: δ = {:.1} mm, limit L/250 = {:.1} mm.", sls_freq.label_en, delta * 1000.0, delta_lim * 1000.0),
        format!("{}: δ = {:.1} mm, Grenze L/250 = {:.1} mm.", sls_freq.label_de, delta * 1000.0, delta_lim * 1000.0),
    ));
    if delta > delta_lim {
        sls = sls.remedy(Remedy::at_most(beam_ref(&beam, "actions[id=Q-office].qAreaPa"), Quantity::new(QuantityKind::Dimensionless, 2e3), Quantity::new(QuantityKind::Dimensionless, 2e3 * delta_lim / delta),
            lc("Reduce imposed load or increase stiffness.", "Nutzlast reduzieren oder Steifigkeit erhöhen.")));
    }
    report.push(sls.build());

    // SLS quasi-permanent — crack control EN 1994-1-1 §7.4 with n_L
    let as_min = part_1_1::as_min_hogging_m2_per_m(&beam, b_eff);
    let s_max_bar = part_1_1::max_bar_spacing_m(beam.wk_limit_m.max(0.0003));
    let crack_demand = (as_min / beam.as_hogging_m2_per_m.max(1e-12)).max(beam.bar_spacing_m / s_max_bar.max(1e-9));
    // Long-term modular ratio scales tension demand under QP hogging
    let qp_hog = sls_qp.m_hog_nm.abs().max(sls_qp.m_nm.abs() * 0.05);
    let n_l_fac = 0.9 + 0.1 * (n_l / 10.0).min(2.0);
    let crack_util = crack_demand * n_l_fac * (1.0 + qp_hog / (sls_char.m_nm.abs().max(1.0) + qp_hog));
    let mut crack = CheckResult::assess(
        format!("en1994.7.4.crack.{}", beam.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§7.4 / EN 1992-1-1 §7.3", "7.4"), beam_ref(&beam, "asHoggingM2PerM"),
        lc("SLS quasi-permanent crack control", "GZG quasi-ständig Rissbreitenbegrenzung"),
    )
    .utilization(Quantity::new(QuantityKind::Dimensionless, crack_util), Quantity::new(QuantityKind::Dimensionless, 1.0))
    .annex(annex)
    .explanation(lc(
        format!("{}: n_L={n_l:.2}; A_s,min={:.1} mm²/m, provided {:.1}; bar s={:.0}/{:.0} mm.", sls_qp.label_en, as_min * 1e6, beam.as_hogging_m2_per_m * 1e6, beam.bar_spacing_m * 1000.0, s_max_bar * 1000.0),
        format!("{}: n_L={n_l:.2}; A_s,min={:.1} mm²/m, vorhanden {:.1}; Stababstand={:.0}/{:.0} mm.", sls_qp.label_de, as_min * 1e6, beam.as_hogging_m2_per_m * 1e6, beam.bar_spacing_m * 1000.0, s_max_bar * 1000.0),
    ));
    if beam.as_hogging_m2_per_m < as_min || crack_util > 1.0 {
        let as_req = (as_min * crack_util.max(1.0)).max(as_min);
        crack = crack.remedy(Remedy::at_least(beam_ref(&beam, "asHoggingM2PerM"), Quantity::new(QuantityKind::Area, beam.as_hogging_m2_per_m), Quantity::new(QuantityKind::Area, as_req),
            lc(format!("Provide ≥ {:.0} mm²/m hogging reinforcement.", as_req * 1e6), format!("≥ {:.0} mm²/m Stützbewehrung vorsehen.", as_req * 1e6))));
    }
    if beam.bar_spacing_m > s_max_bar {
        crack = crack.remedy(Remedy::at_most(beam_ref(&beam, "barSpacingM"), Quantity::length_m(beam.bar_spacing_m), Quantity::length_m(s_max_bar),
            lc(format!("Reduce bar spacing to ≤ {:.0} mm.", s_max_bar * 1000.0), format!("Stababstand auf ≤ {:.0} mm verringern.", s_max_bar * 1000.0))));
    }
    report.push(crack.build());
}

fn evaluate_column(report: &mut CheckReport, doc: &En1994Snapshot, col: &CompositeColumn) {
    let annex = doc.annex;
    let uls = part_en1990::uls_composite(&col.actions, col.length_m, "simply_supported", 1.0, annex);
    let n_ed = uls.n_n.abs();
    let m_ed = uls.m_nm.abs().max(uls.m_hog_nm.abs());
    let subject = SubjectRef::new(col.id.clone(), col_path(col, "actions"), lc(format!("Column {}", col.id), format!("Stütze {}", col.id)));
    let n_pl = part_column::n_pl_rd_n(col, annex);
    let chi = part_column::chi(col, annex);
    let n_rd = chi * n_pl;
    let lambda = part_column::relative_slenderness(col, annex);
    let lb = part_column::local_buckling_util(col);
    let mut nchk = CheckResult::assess(
        format!("en1994.6.7.3.npl.{}", col.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§6.7.3", "6.7.3"), subject.clone(),
        lc("Composite column axial resistance", "Normalkrafttragfähigkeit Verbundstütze"),
    )
    .utilization(Quantity::new(QuantityKind::Force, n_ed), Quantity::new(QuantityKind::Force, n_rd))
    .annex(annex)
    .explanation(lc(
        format!("Kind {}, {}: N_Ed = {:.0} kN, χ={chi:.3}, λ̄={lambda:.3}, N_Rd={:.0} kN, d/t util={lb:.2}.", col.kind, uls.label_en, n_ed / 1e3, n_rd / 1e3),
        format!("Art {}, {}: N_Ed = {:.0} kN, χ={chi:.3}, λ̄={lambda:.3}, N_Rd={:.0} kN, d/t-Ausn.={lb:.2}.", col.kind, uls.label_de, n_ed / 1e3, n_rd / 1e3),
    ));
    if n_ed > n_rd || lb > 1.0 {
        nchk = nchk.remedy(Remedy::at_least(
            SubjectRef::new(col.id.clone(), col_path(col, "wallThicknessM"), lc(format!("Column {}", col.id), format!("Stütze {}", col.id))),
            Quantity::length_m(col.wall_thickness_m), Quantity::length_m(col.wall_thickness_m * lb.max(n_ed / n_rd.max(1.0))),
            lc("Increase wall thickness / section capacity.", "Wanddicke / Querschnittstragfähigkeit erhöhen."),
        ));
    }
    report.push(nchk.build());

    let util = part_column::interaction_utilization(col, annex, n_ed, m_ed);
    let poly = part_column::interaction_polygon(col, annex);
    let mut mn = CheckResult::assess(
        format!("en1994.6.7.3.mn.{}", col.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§6.7.3.6", "6.7.3.6"),
        SubjectRef::new(col.id.clone(), col_path(col, "kind"), lc(format!("Column {}", col.id), format!("Stütze {}", col.id))),
        lc("Composite column M–N interaction", "M–N-Interaktion Verbundstütze"),
    )
    .utilization(Quantity::new(QuantityKind::Dimensionless, util), Quantity::new(QuantityKind::Dimensionless, 1.0))
    .annex(annex)
    .explanation(lc(
        format!("Kind {}, {}: util={util:.3} (N_pl={:.0} kN, M_pl={:.1} kNm).", col.kind, uls.label_en, poly[0].n_n / 1e3, poly[3].m_nm / 1e3),
        format!("Art {}, {}: Ausn.={util:.3} (N_pl={:.0} kN, M_pl={:.1} kNm).", col.kind, uls.label_de, poly[0].n_n / 1e3, poly[3].m_nm / 1e3),
    ));
    if util > 1.0 {
        mn = mn.remedy(Remedy::at_most(
            SubjectRef::new(col.id.clone(), col_path(col, "actions[id=Q-col].nKN"), lc(format!("Column {}", col.id), format!("Stütze {}", col.id))),
            Quantity::new(QuantityKind::Force, n_ed), Quantity::new(QuantityKind::Force, n_ed / util),
            lc("Reduce axial / moment demand.", "Normalkraft-/Momentenbeanspruchung reduzieren."),
        ));
    }
    report.push(mn.build());
}

fn evaluate_slab(report: &mut CheckReport, doc: &En1994Snapshot, slab: &CompositeSlab) {
    let annex = doc.annex;
    let uls = part_en1990::uls_composite(&slab.actions, slab.span_m, &slab.support, 1.0, annex);
    // For slabs, q_area acts on 1 m strip: action_internals uses spacing — pass 1.0 so q_area*1 = line per m
    let m_ed = uls.m_nm;
    let v_ed = uls.v_n;
    let label = lc(format!("Composite slab {}", slab.id), format!("Verbunddecke {}", slab.id));

    // Construction-stage sheeting
    let m_sheet = part_1_1::sheeting_construction_mrd_nm_per_m(&slab.sheeting, 280e6);
    let q_constr = slab.actions.iter().filter(|a| a.stage == "construction" || a.kind == "construction").map(|a| a.q_area_pa).sum::<f64>()
        + slab.actions.iter().filter(|a| a.kind == "permanent" && a.stage == "construction").map(|a| a.q_area_pa).sum::<f64>();
    let m_constr = if q_constr > 0.0 { q_constr * slab.span_m.powi(2) / 8.0 } else {
        // wet concrete self-weight on sheeting
        25e3 * (slab.concrete_thickness_m - slab.sheeting.height_m).max(0.04) * slab.span_m.powi(2) / 8.0
    };
    let mut sh = CheckResult::assess(
        format!("en1994.9.3.sheeting.{}", slab.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§9.3 / §9.7", "9.3"),
        SubjectRef::new(slab.id.clone(), slab_path(slab, "sheeting.thicknessM"), label.clone()),
        lc("Sheeting resistance at construction stage", "Tragfähigkeit Profilblech im Bauzustand"),
    )
    .utilization(Quantity::new(QuantityKind::Moment, m_constr), Quantity::new(QuantityKind::Moment, m_sheet))
    .annex(annex)
    .explanation(lc(
        format!("Construction M_Ed = {:.2} kNm/m, M_Rd,sheeting(t={:.2} mm) = {:.2} kNm/m.", m_constr / 1e3, slab.sheeting.thickness_m * 1000.0, m_sheet / 1e3),
        format!("Herstellung M_Ed = {:.2} kNm/m, M_Rd,Blech(t={:.2} mm) = {:.2} kNm/m.", m_constr / 1e3, slab.sheeting.thickness_m * 1000.0, m_sheet / 1e3),
    ));
    if m_constr > m_sheet {
        sh = sh.remedy(Remedy::at_least(
            SubjectRef::new(slab.id.clone(), slab_path(slab, "sheeting.thicknessM"), label.clone()),
            Quantity::length_m(slab.sheeting.thickness_m), Quantity::length_m(slab.sheeting.thickness_m * m_constr / m_sheet.max(1.0)),
            lc("Increase sheeting thickness.", "Profilblechdicke erhöhen."),
        ));
    }
    report.push(sh.build());

    let m_rd = part_slab::bending_resistance_nm_per_m(slab, annex);
    let mut mb = CheckResult::assess(
        format!("en1994.9.7.2.mrd.{}", slab.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§9.7.2", "9.7.2"),
        SubjectRef::new(slab.id.clone(), slab_path(slab, "actions"), label.clone()),
        lc("Composite slab bending", "Biegung Verbunddecke"),
    )
    .utilization(Quantity::new(QuantityKind::Moment, m_ed), Quantity::new(QuantityKind::Moment, m_rd))
    .annex(annex)
    .explanation(lc(
        format!("{}: m_Ed = {:.2} kNm/m, m_Rd = {:.2} kNm/m (incl. A_p from t).", uls.label_en, m_ed / 1e3, m_rd / 1e3),
        format!("{}: m_Ed = {:.2} kNm/m, m_Rd = {:.2} kNm/m (inkl. A_p aus t).", uls.label_de, m_ed / 1e3, m_rd / 1e3),
    ));
    if m_ed > m_rd {
        let as_need = slab.as_m2_per_m * m_ed / m_rd.max(1.0);
        mb = mb.remedy(Remedy::at_least(SubjectRef::new(slab.id.clone(), slab_path(slab, "asM2PerM"), label.clone()), Quantity::new(QuantityKind::Area, slab.as_m2_per_m), Quantity::new(QuantityKind::Area, as_need),
            lc(format!("Increase slab reinforcement to ≥ {:.0} mm²/m.", as_need * 1e6), format!("Plattenbewehrung auf ≥ {:.0} mm²/m erhöhen.", as_need * 1e6))));
    }
    report.push(mb.build());

    let v_l = part_slab::longitudinal_shear_mk_n_per_m(slab).max(part_slab::longitudinal_shear_tau_n_per_m(slab));
    let mut vl = CheckResult::assess(
        format!("en1994.9.7.3.mk.{}", slab.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§9.7.3", "9.7.3"),
        SubjectRef::new(slab.id.clone(), slab_path(slab, "sheeting.thicknessM"), label.clone()),
        lc("Composite slab longitudinal shear (m-k / τ_u)", "Längsschub Verbunddecke (m-k / τ_u)"),
    )
    .utilization(Quantity::new(QuantityKind::Force, v_ed), Quantity::new(QuantityKind::Force, v_l))
    .annex(annex)
    .explanation(lc(format!("v_Ed = {:.1} kN/m, v_l,Rd = {:.1} kN/m.", v_ed / 1e3, v_l / 1e3), format!("v_Ed = {:.1} kN/m, v_l,Rd = {:.1} kN/m.", v_ed / 1e3, v_l / 1e3)));
    if v_ed > v_l {
        vl = vl.remedy(Remedy::at_least(SubjectRef::new(slab.id.clone(), slab_path(slab, "sheeting.thicknessM"), label.clone()), Quantity::length_m(slab.sheeting.thickness_m), Quantity::length_m(slab.sheeting.thickness_m * v_ed / v_l.max(1.0)),
            lc("Increase sheeting thickness or m-k parameters.", "Profilblechdicke oder m-k-Parameter erhöhen.")));
    }
    report.push(vl.build());

    let v_rd = part_slab::vertical_shear_resistance_n_per_m(slab, annex);
    let mut vv = CheckResult::assess(
        format!("en1994.9.7.5.vrd.{}", slab.id), "DIN EN 1994-1-1",
        ClauseId::new("EN 1994-1-1", "§9.7.5", "9.7.5"),
        SubjectRef::new(slab.id.clone(), slab_path(slab, "concreteThicknessM"), label.clone()),
        lc("Composite slab vertical shear", "Querkraft Verbunddecke"),
    )
    .utilization(Quantity::new(QuantityKind::Force, v_ed), Quantity::new(QuantityKind::Force, v_rd))
    .annex(annex)
    .explanation(lc(format!("v_Ed = {:.1} kN/m, v_Rd,c = {:.1} kN/m.", v_ed / 1e3, v_rd / 1e3), format!("v_Ed = {:.1} kN/m, v_Rd,c = {:.1} kN/m.", v_ed / 1e3, v_rd / 1e3)));
    if v_ed > v_rd {
        vv = vv.remedy(Remedy::at_least(SubjectRef::new(slab.id.clone(), slab_path(slab, "concreteThicknessM"), label), Quantity::length_m(slab.concrete_thickness_m), Quantity::length_m(slab.concrete_thickness_m * v_ed / v_rd.max(1.0)),
            lc("Increase slab thickness.", "Plattendicke erhöhen.")));
    }
    report.push(vv.build());
}

pub fn evaluate(document: &En1994Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    let annex = document.annex;
    if document.beams.is_empty() && document.columns.is_empty() && document.slabs.is_empty() {
        report.push(CheckResult::assess("en1994.subject.empty", "DIN EN 1994-1-1", ClauseId::new("EN 1994-1-1", "§1", "1"),
            SubjectRef::whole(lc("Composite structure", "Verbundtragwerk")), lc("Subject presence", "Gegenstand vorhanden"))
            .not_applicable(lc("No beams, columns or slabs to assess.", "Keine Träger, Stützen oder Decken zur Bewertung.")).annex(annex).build());
        return report;
    }
    for beam in &document.beams { evaluate_beam(&mut report, document, beam); }
    for col in &document.columns { evaluate_column(&mut report, document, col); }
    for slab in &document.slabs { evaluate_slab(&mut report, document, slab); }

    let steel_h = document.beams.first().map(|b| b.steel.height_m).unwrap_or(0.3);
    let deck = document.beams.first().map(|b| b.sheeting.profile.as_str()).unwrap_or("trapezoidal");
    match part_1_2::parse_fire_rating(&document.fire_rating) {
        Some(rating) => {
            let t_req = part_1_2::insulation_thickness_m(rating, deck, steel_h, annex);
            let mut fire = CheckResult::assess("en1994.1-2.4.2.insulation", "DIN EN 1994-1-2", ClauseId::new("EN 1994-1-2", "§4.2 / Table 4.2", "4.2"),
                SubjectRef::new("", "insulationThicknessM", lc("Fire insulation", "Brandschutzdämmung")),
                lc("Tabulated fire insulation thickness", "Tabellarische Brandschutzdämmstärke"))
            .minimum(Quantity::length_m(document.insulation_thickness_m), Quantity::length_m(t_req)).annex(annex)
            .explanation(lc(format!("Provided {:.0} mm, required {:.0} mm for {:?} / {deck} / h={steel_h:.3} m.", document.insulation_thickness_m * 1000.0, t_req * 1000.0, rating),
                format!("Vorhanden {:.0} mm, erforderlich {:.0} mm für {:?} / {deck} / h={steel_h:.3} m.", document.insulation_thickness_m * 1000.0, t_req * 1000.0, rating)));
            if document.insulation_thickness_m < t_req {
                fire = fire.remedy(Remedy::at_least(SubjectRef::new("", "insulationThicknessM", lc("Fire insulation", "Brandschutzdämmung")), Quantity::length_m(document.insulation_thickness_m), Quantity::length_m(t_req),
                    lc(format!("Increase insulation to ≥ {:.0} mm.", t_req * 1000.0), format!("Dämmstärke auf ≥ {:.0} mm erhöhen.", t_req * 1000.0))));
            }
            report.push(fire.build());
            let steel = document.beams.first().map(|b| b.steel.clone()).unwrap_or_else(SteelSection::heb300);
            let am_v = part_1_2::section_factor_m_inv(&steel);
            let theta_a = part_1_2::steel_temperature_c(rating, am_v, document.insulation_thickness_m);
            // Fire combination load level η_fi from Eq. 6.11 / G+ψ_fi Q vs ambient ULS
            let eta_fi = document.beams.first().map(|b| {
                let fire_e = part_en1990::fire_combination(&b.actions, b.span_m, &b.support, b.spacing_m, annex);
                let uls_e = part_en1990::uls_composite(&b.actions, b.span_m, &b.support, b.spacing_m, annex);
                (fire_e.m_nm / uls_e.m_nm.max(1.0)).clamp(0.2, 0.7)
            }).unwrap_or(0.65);
            let theta_cr = part_1_2::critical_temperature_c(eta_fi);
            let mut crit = CheckResult::assess("en1994.1-2.4.3.theta", "DIN EN 1994-1-2", ClauseId::new("EN 1994-1-2", "§4.3", "4.3"),
                SubjectRef::new("", "insulationThicknessM", lc("Fire insulation", "Brandschutzdämmung")),
                lc("Critical temperature method", "Kritische-Temperatur-Verfahren"))
            .utilization(Quantity::new(QuantityKind::Dimensionless, theta_a), Quantity::new(QuantityKind::Dimensionless, theta_cr)).annex(annex)
            .explanation(lc(format!("θ_a={theta_a:.0}°C (A_m/V={am_v:.0}), θ_cr={theta_cr:.0}°C (η_fi={eta_fi:.2} from Eq. 6.11)."),
                format!("θ_a={theta_a:.0}°C (A_m/V={am_v:.0}), θ_cr={theta_cr:.0}°C (η_fi={eta_fi:.2} aus Gl. 6.11).")));
            if theta_a > theta_cr {
                crit = crit.remedy(Remedy::at_least(SubjectRef::new("", "insulationThicknessM", lc("Fire insulation", "Brandschutzdämmung")), Quantity::length_m(document.insulation_thickness_m), Quantity::length_m(t_req.max(document.insulation_thickness_m) * 1.15),
                    lc("Increase insulation to keep θ_a ≤ θ_cr.", "Dämmung erhöhen, damit θ_a ≤ θ_cr.")));
            }
            report.push(crit.build());
        }
        None => {
            report.push(CheckResult::assess("en1994.1-2.4.2.insulation", "DIN EN 1994-1-2", ClauseId::new("EN 1994-1-2", "§4.2", "4.2"),
                SubjectRef::new("", "fireRating", lc("Fire rating", "Feuerwiderstandsklasse")),
                lc("Tabulated fire insulation thickness", "Tabellarische Brandschutzdämmstärke"))
                .not_applicable(lc("Fire rating not recognised (use R30/R60/R90/R120).", "Feuerwiderstandsklasse unbekannt.")).annex(annex).build());
        }
    }

    if document.structure_kind == "bridge" {
        for beam in &document.beams {
            let steel = beam.steel.clone();
            let w_el = steel.w_pl_y_m3 * 0.85; // elastic approx
            let mut delta = beam.actions.iter().filter(|a| a.kind == "fatigue").map(|a| a.delta_sigma_k_pa).fold(0.0_f64, f64::max);
            if delta < 1.0 {
                delta = part_en1990::flm3_delta_sigma_pa(beam.span_m, w_el);
            }
            let limit = part_2::steel_fatigue_resistance_pa(&document.fatigue_detail, beam.n_cycles, annex);
            let mut fat = CheckResult::assess(format!("en1994.2.6.8.2.delta-sigma.{}", beam.id), "DIN EN 1994-2",
                ClauseId::new("EN 1994-2", "§6.8.2", "6.8.2"), beam_ref(beam, "actions"),
                lc("Bridge steel fatigue Δσ (FLM3)", "Brückenstahl Ermüdung Δσ (FLM3)"))
            .utilization(Quantity::new(QuantityKind::Stress, delta), Quantity::new(QuantityKind::Stress, limit)).annex(annex)
            .explanation(lc(format!("FLM3 Δσ = {:.1} MPa, Δσ_R = {:.1} MPa (detail {}, N={:.2e}).", delta / 1e6, limit / 1e6, document.fatigue_detail, beam.n_cycles),
                format!("FLM3 Δσ = {:.1} MPa, Δσ_R = {:.1} MPa (Detail {}, N={:.2e}).", delta / 1e6, limit / 1e6, document.fatigue_detail, beam.n_cycles)));
            if delta > limit {
                fat = fat.remedy(Remedy::at_least(beam_ref(beam, "steel.wPlYM3"), Quantity::new(QuantityKind::Dimensionless, steel.w_pl_y_m3), Quantity::new(QuantityKind::Dimensionless, steel.w_pl_y_m3 * delta / limit),
                    lc("Increase section modulus / reduce FLM stress.", "Widerstandsmoment erhöhen / FLM-Spannung reduzieren.")));
            }
            report.push(fat.build());
            let mut dtau = beam.actions.iter().filter(|a| a.kind == "fatigue").map(|a| a.delta_tau_k_pa).fold(0.0_f64, f64::max);
            if dtau < 1.0 { dtau = part_en1990::flm3_delta_tau_pa(beam.studs.diameter_m); }
            let stud_lim = part_2::stud_fatigue_resistance_pa(beam.n_cycles, annex);
            let mut sf = CheckResult::assess(format!("en1994.2.6.8.3.stud.{}", beam.id), "DIN EN 1994-2",
                ClauseId::new("EN 1994-2", "§6.8.3", "6.8.3"), beam_ref(beam, "studs.diameterM"),
                lc("Stud fatigue Δτ (FLM3)", "Bolzenermüdung Δτ (FLM3)"))
            .utilization(Quantity::new(QuantityKind::Stress, dtau), Quantity::new(QuantityKind::Stress, stud_lim)).annex(annex)
            .explanation(lc(format!("FLM3 Δτ = {:.1} MPa, Δτ_c = {:.1} MPa.", dtau / 1e6, stud_lim / 1e6), format!("FLM3 Δτ = {:.1} MPa, Δτ_c = {:.1} MPa.", dtau / 1e6, stud_lim / 1e6)));
            if dtau > stud_lim {
                sf = sf.remedy(Remedy::at_least(beam_ref(beam, "studs.diameterM"), Quantity::length_m(beam.studs.diameter_m), Quantity::length_m(beam.studs.diameter_m * (dtau / stud_lim).sqrt()),
                    lc("Increase stud diameter.", "Bolzendurchmesser erhöhen.")));
            }
            report.push(sf.build());
        }
    } else {
        report.push(CheckResult::assess("en1994.2.fatigue", "DIN EN 1994-2", ClauseId::new("EN 1994-2", "§6.8", "6.8"),
            SubjectRef::new("", "structureKind", lc("Structure kind", "Tragwerkstyp")),
            lc("Bridge fatigue", "Brückenermüdung"))
            .not_applicable(lc("Structure is not a bridge — EN 1994-2 fatigue not applicable.", "Kein Brückentragwerk — EN 1994-2 Ermüdung nicht anwendbar.")).annex(annex).build());
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
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::outline::En1994Outline;
//#endregion 🔁️Re-exports
