//! 🥫️ Cold-formed sheeting diagrams (EN 1999-1-4): the local buckling reduction ρ_c over the slenderness β/ε with the class limits and a marker that drags the flat width, and the utilisation of the axial, bending, shear and combined checks over the sheet thickness with a marker that drags the thickness. The resistances are the formulas of the sheet checks; the demands are the governing check values.

use super::*;
use crate::model::ColdFormedSheet;
use crate::scene::projection_kit::handle_kit::{field_binding, round};

const BETA_MAX: f64 = 60.0;
const THICKNESS_MIN: f64 = 0.0005;
const THICKNESS_MAX: f64 = 0.008;
const UTILISATION_CAP: f64 = 3.0;

fn rib_of(d: &En1999Snapshot, sheet: &ColdFormedSheet) -> Option<part_1_4_1_5::Rib> {
    d.sheet_profiles.iter().find(|p| p.sheet_id == sheet.id).map(|p| part_1_4_1_5::Rib { pitch: p.pitch, top: p.top_flange, bottom: p.bottom_flange, height: p.web_height }).filter(part_1_4_1_5::Rib::is_valid)
}

fn sheet_focus(sheet: &ColdFormedSheet) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Member, &sheet.id, format!("coldFormed[id={}]", sheet.id)))
}

/// 🥫️ `(N_Rd, M_Rd, V_Rd)` of the flat strip `b × t` of an alloy: `ρ_c·b·t·ρ_o,haz·f_o/γ_M1`, `ρ_o,haz·(b·t²/6)·f_o/γ_M1` and `b·t·f_o/(√3·γ_M1)`, with ρ_o,haz = 1 for an unwelded strip.
pub fn sheet_resistances(width: f64, thickness: f64, welded: bool, alloy: &AlloyProps, gamma_m1: f64) -> (f64, f64, f64) {
    let beta_eps = width / thickness.max(1e-12) / part_1_1::epsilon(alloy.f_o_pa);
    let rho = part_1_1::rho_c(beta_eps, alloy.buckling_class_a, false, welded);
    let rho_haz = if welded { alloy.rho_o_haz } else { 1.0 };
    (rho * width * thickness * rho_haz * alloy.f_o_pa / gamma_m1, rho_haz * width * thickness * thickness / 6.0 * alloy.f_o_pa / gamma_m1, width * thickness * alloy.f_o_pa / (3.0f64.sqrt() * gamma_m1))
}

/// 🥫️ The diagram `sheet-local:<id>`: ρ_c(β/ε) of the flat element with the three class limits, `None` without an alloy.
pub(super) fn sheet_local(d: &En1999Snapshot, report: &CheckReport, sheet: &ColdFormedSheet) -> Option<Diagram2d> {
    let material = d.materials.iter().find(|m| m.id == sheet.material_id)?;
    let alloy = part_1_1::resolve_alloy_for_thickness(&material.designation, sheet.thickness)?;
    let eps = part_1_1::epsilon(alloy.f_o_pa);
    let limits = part_1_1::class_limits(alloy.buckling_class_a, false, sheet.welded);
    let beta_eps = sheet.width / sheet.thickness.max(1e-12) / eps;
    let rho = part_1_1::rho_c(beta_eps, alloy.buckling_class_a, false, sheet.welded);
    let xs: Vec<f64> = (0..=SAMPLES).map(|i| BETA_MAX * i as f64 / SAMPLES as f64).collect();
    let ys: Vec<f64> = xs.iter().map(|x| part_1_1::rho_c(*x, alloy.buckling_class_a, false, sheet.welded)).collect();
    let check = exact(report, &format!("en1999.1-4.local.{}", sheet.id));
    let tone = if beta_eps > limits[2] { Tone::Warning } else { Tone::Info };
    let mut series = vec![Series::Polyline { id: "rho-c".into(), label: loc(&format!("ρ_c ({} flat element)", if sheet.welded { "welded" } else { "unwelded" }), &format!("ρ_c (ebenes Element, {})", if sheet.welded { "geschweißt" } else { "ungeschweißt" })), points: xs.iter().zip(&ys).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) }];
    for (k, limit) in limits.iter().enumerate() {
        series.push(Series::Limit { id: format!("limit-class-{}", k + 1), label: loc(&format!("Class {} limit β{} = {limit:.1}", k + 1, k + 1), &format!("Grenze Klasse {} β{} = {limit:.1}", k + 1, k + 1)), axis: AxisRef::X, value: *limit, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) });
    }
    let marker = Marker {
        id: "width".into(),
        at: Vec2::new(beta_eps.min(BETA_MAX), rho),
        shape: MarkerShape::Diamond,
        tone,
        label: loc(&format!("b = {:.0} mm, t = {:.2} mm: β/ε = {beta_eps:.2}, ρ_c = {rho:.3}", sheet.width * 1e3, sheet.thickness * 1e3), &format!("b = {:.0} mm, t = {:.2} mm: β/ε = {beta_eps:.2}, ρ_c = {rho:.3}", sheet.width * 1e3, sheet.thickness * 1e3)),
        utilisation: Some(beta_eps / limits[2]),
        bound_check: check.map(|c| c.id.clone()),
        binding: Some(field_binding(format!("coldFormed[id={}].width", sheet.id), Affine { base: 0.0, k: eps * sheet.thickness }, "m", round(0.001))),
    };
    Some(Diagram2d {
        id: format!("sheet-local:{}", sheet.id),
        title: loc(&format!("Local buckling of the sheet element – {}", sheet.id), &format!("Lokales Beulen des Blechelements – {}", sheet.id)),
        description: loc(
            "Reduction factor ρ_c of the flat element of the cold-formed sheet over the slenderness β/ε (EN 1999-1-4 with EN 1999-1-1, 6.1.4 and 6.1.5) and the class limits β1 to β3; the diamond is the current sheet, dragging it changes the flat width.",
            "Abminderungsfaktor ρ_c des ebenen Elements des Kaltprofils über der Schlankheit β/ε (EN 1999-1-4 mit EN 1999-1-1, 6.1.4 und 6.1.5) und die Klassengrenzen β1 bis β3; die Raute ist das aktuelle Blech, Ziehen ändert die Flachbreite.",
        ),
        axes: vec![axis("Slenderness β/ε", "Schlankheit β/ε", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, BETA_MAX))), axis("Reduction ρ_c", "Abminderung ρ_c", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.05)))],
        series,
        markers: vec![marker],
        annotations: Vec::new(),
        focus: sheet_focus(sheet),
        legend: true,
        table_twin: DataTable { columns: vec![column("Slenderness β/ε", "Schlankheit β/ε", "1", xs), column("Reduction ρ_c", "Abminderung ρ_c", "1", ys)] },
    })
}

/// 🥫️ The diagram `sheet-resistance:<id>`: the utilisation of the axial, bending, shear and combined checks over the thickness at the governing demands, the limit 1 and the thickness marker; `None` without the checks or an alloy.
pub(super) fn sheet_resistance(d: &En1999Snapshot, report: &CheckReport, sheet: &ColdFormedSheet) -> Option<Diagram2d> {
    let material = d.materials.iter().find(|m| m.id == sheet.material_id)?;
    let params = na_de::AnnexParams::for_choice(d.annex);
    let demand = |suffix: &str| exact(report, &format!("en1999.1-4.{suffix}.{}", sheet.id)).map(|c| c.computed.value.abs());
    let (n_ed, m_ed, v_ed) = (demand("axial")?, demand("bend")?, demand("shear")?);
    let nm = exact(report, &format!("en1999.1-4.nm.{}", sheet.id))?;
    let top_t = THICKNESS_MAX.max(1.5 * sheet.thickness);
    let ts: Vec<f64> = (0..=SAMPLES).map(|i| THICKNESS_MIN + (top_t - THICKNESS_MIN) * i as f64 / SAMPLES as f64).collect();
    let (rib, shear_constants) = (rib_of(d, sheet), part_1_4_1_5::sheet_web_shear_constants());
    let rows: Option<Vec<[f64; 4]>> = ts
        .iter()
        .map(|t| {
            let alloy = part_1_1::resolve_alloy_for_thickness(&material.designation, *t)?;
            let (n_rd, m_rd, v_rd) = rib.map_or_else(|| sheet_resistances(sheet.width, *t, sheet.welded, &alloy, params.gamma_m1), |r| {
                let f = part_1_4_1_5::rib_sheet_flexure(&r, sheet.width, *t, alloy.f_o_pa, alloy.buckling_class_a, sheet.welded, if sheet.welded { alloy.rho_o_haz } else { 1.0 }, params.gamma_m1);
                (f.n_rd, f.m_sag.min(f.m_hog), part_1_4_1_5::rib_sheet_shear(&r, sheet.width, *t, alloy.f_o_pa, shear_constants, params.gamma_m1))
            });
            let (un, um, uv) = (n_ed / n_rd.max(1e-12), m_ed / m_rd.max(1e-12), v_ed / v_rd.max(1e-12));
            Some([un, um, uv, un + um].map(|u| u.min(UTILISATION_CAP)))
        })
        .collect();
    let rows = rows?;
    let column_of = |k: usize| rows.iter().map(|r| r[k]).collect::<Vec<f64>>();
    let names: [(&str, &str, &str, &str, Tone); 4] = [("axial", "Axial N_Ed/N_Rd", "Normalkraft N_Ed/N_Rd", "n", Tone::Info), ("bend", "Bending M_Ed/M_Rd", "Biegung M_Ed/M_Rd", "m", Tone::Warning), ("shear", "Shear V_Ed/V_Rd", "Querkraft V_Ed/V_Rd", "v", Tone::Neutral), ("nm", "Combined N + M", "Interaktion N + M", "nm", Tone::Error)];
    let mut series: Vec<Series> = names.iter().enumerate().map(|(k, (id, en, de, _, tone))| Series::Polyline { id: (*id).into(), label: loc(en, de), points: ts.iter().zip(column_of(k)).map(|(t, u)| Vec2::new(*t, u)).collect(), style: stroke(*tone, if k == 3 { 2.5 } else { 1.5 }, if k == 3 { Dash::Solid } else { Dash::Dashed }) }).collect();
    series.push(Series::Limit { id: "limit".into(), label: loc("Utilisation limit 1", "Ausnutzungsgrenze 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.0, Dash::Dotted) });
    let marker = Marker {
        id: "thickness".into(),
        at: Vec2::new(sheet.thickness.clamp(THICKNESS_MIN, top_t), nm.utilization.min(UTILISATION_CAP)),
        shape: MarkerShape::Diamond,
        tone: utilisation_tone(nm.utilization),
        label: loc(&format!("t = {:.2} mm: N + M utilisation {:.2}", sheet.thickness * 1e3, nm.utilization), &format!("t = {:.2} mm: Ausnutzung N + M {:.2}", sheet.thickness * 1e3, nm.utilization)),
        utilisation: Some(nm.utilization),
        bound_check: Some(nm.id.clone()),
        binding: Some(field_binding(format!("coldFormed[id={}].thickness", sheet.id), Affine::IDENTITY, "m", round(0.0005))),
    };
    let mut columns = vec![column("Thickness t", "Dicke t", "m", ts.clone())];
    columns.extend(names.iter().enumerate().map(|(k, (_, en, de, _, _))| column(en, de, "1", column_of(k))));
    Some(Diagram2d {
        id: format!("sheet-resistance:{}", sheet.id),
        title: loc(&format!("Sheet utilisation over the thickness – {}", sheet.id), &format!("Blechausnutzung über der Dicke – {}", sheet.id)),
        description: loc(
            "Utilisation of the flat strip against N, M and V (EN 1999-1-4, effective width and HAZ) and of N + M together over the sheet thickness at the governing demands of each check; values above 3 are cut off. The diamond is the current thickness, dragging it edits coldFormed thickness.",
            "Ausnutzung des ebenen Streifens für N, M und V (EN 1999-1-4, wirksame Breite und WEZ) und für N + M zusammen über der Blechdicke bei den maßgebenden Einwirkungen jeder Prüfung; Werte über 3 sind abgeschnitten. Die Raute ist die aktuelle Dicke, Ziehen ändert die Blechdicke.",
        ),
        axes: vec![axis("Thickness t", "Dicke t", QuantityKind::Length, "m", AxisScale::Linear, Some((THICKNESS_MIN, top_t))), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, UTILISATION_CAP)))],
        series,
        markers: vec![marker],
        annotations: Vec::new(),
        focus: sheet_focus(sheet),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 🪗 The diagram `sheet-profile:<id>` of a profiled sheet: the mid-line of one rib with the gross and the effective neutral axis (compressed flange and web at their effective thickness, EN 1999-1-4 5.5 with 1999-1-1 6.1.5), and the markers that drag the pitch, the top and the bottom flange width; `None` without a valid profile or an alloy.
pub(super) fn sheet_profile(d: &En1999Snapshot, report: &CheckReport, sheet: &ColdFormedSheet) -> Option<Diagram2d> {
    let rib = rib_of(d, sheet)?;
    let profile = d.sheet_profiles.iter().find(|p| p.sheet_id == sheet.id)?;
    let alloy = part_1_1::resolve_alloy_for_thickness(&d.materials.iter().find(|m| m.id == sheet.material_id)?.designation, sheet.thickness)?;
    let (k, effective) = part_1_4_1_5::rib_effective(&rib, sheet.thickness, alloy.f_o_pa, alloy.buckling_class_a, sheet.welded);
    let gross = part_1_4_1_5::rib_props(&rib, sheet.thickness, [1.0; 3]);
    let (e, h) = (rib.projection(), rib.height);
    let outline = [(0.0, 0.0), (rib.bottom, 0.0), (rib.bottom + e, h), (rib.bottom + e + rib.top, h), (rib.pitch, 0.0)];
    let check = exact(report, &format!("en1999.1-4.bend.{}", sheet.id));
    let tone = check.map_or(Tone::Info, |c| utilisation_tone(c.utilization));
    let marker = |id: &str, at: (f64, f64), field: &str, base: f64, en: String, de: String| Marker { id: id.into(), at: Vec2::new(at.0, at.1), shape: MarkerShape::Diamond, tone, label: loc(&en, &de), utilisation: check.map(|c| c.utilization), bound_check: check.map(|c| c.id.clone()), binding: Some(field_binding(format!("sheetProfiles[id={}].{field}", profile.id), Affine { base, k: 1.0 }, "m", round(0.001))) };
    let (xs, zs): (Vec<f64>, Vec<f64>) = outline.iter().copied().unzip();
    Some(Diagram2d {
        id: format!("sheet-profile:{}", sheet.id),
        title: loc(&format!("Rib of the profiled sheet – {}", sheet.id), &format!("Rippe des Profilblechs – {}", sheet.id)),
        description: loc(
            &format!("Mid-line of one trapezoidal rib (p = {:.0} mm, h = {:.0} mm, φ = {:.1}°) with the neutral axis of the gross section at {:.1} mm and of the effective section at {:.1} mm (thickness factors: compressed flange {:.3}, webs {:.3}); the diamonds drag the pitch and the flange widths.", rib.pitch * 1e3, h * 1e3, rib.web_angle().to_degrees(), gross.z_c * 1e3, effective.z_c * 1e3, k[0], k[1]),
            &format!("Mittellinie einer Trapezrippe (p = {:.0} mm, h = {:.0} mm, φ = {:.1}°) mit der Nulllinie des Bruttoquerschnitts bei {:.1} mm und des wirksamen Querschnitts bei {:.1} mm (Dickenfaktoren: gedrückter Gurt {:.3}, Stege {:.3}); die Rauten ziehen den Rippenabstand und die Gurtbreiten.", rib.pitch * 1e3, h * 1e3, rib.web_angle().to_degrees(), gross.z_c * 1e3, effective.z_c * 1e3, k[0], k[1]),
        ),
        axes: vec![axis("Across the rib", "Quer zur Rippe", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, rib.pitch))), axis("Height above the bottom flange", "Höhe über dem Untergurt", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, h * 1.1)))],
        series: vec![
            Series::Polyline { id: "midline".into(), label: loc("Mid-line of the rib", "Mittellinie der Rippe"), points: outline.iter().map(|(x, z)| Vec2::new(*x, *z)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Limit { id: "axis-gross".into(), label: loc(&format!("Neutral axis, gross: {:.1} mm", gross.z_c * 1e3), &format!("Nulllinie, brutto: {:.1} mm", gross.z_c * 1e3)), axis: AxisRef::Y, value: gross.z_c, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) },
            Series::Limit { id: "axis-effective".into(), label: loc(&format!("Neutral axis, effective: {:.1} mm", effective.z_c * 1e3), &format!("Nulllinie, wirksam: {:.1} mm", effective.z_c * 1e3)), axis: AxisRef::Y, value: effective.z_c, style: stroke(Tone::Warning, 1.5, Dash::Dotted) },
        ],
        markers: vec![
            marker("pitch", (rib.pitch, 0.0), "pitch", 0.0, format!("p = {:.0} mm (drag: rib pitch)", rib.pitch * 1e3), format!("p = {:.0} mm (ziehen: Rippenabstand)", rib.pitch * 1e3)),
            marker("top-flange", (rib.bottom + e + rib.top, h), "topFlange", -(rib.bottom + e), format!("b_t = {:.0} mm (drag: top flange)", rib.top * 1e3), format!("b_t = {:.0} mm (ziehen: Obergurt)", rib.top * 1e3)),
            marker("bottom-flange", (rib.bottom, 0.0), "bottomFlange", 0.0, format!("b_b = {:.0} mm (drag: bottom flange)", rib.bottom * 1e3), format!("b_b = {:.0} mm (ziehen: Untergurt)", rib.bottom * 1e3)),
        ],
        annotations: Vec::new(),
        focus: sheet_focus(sheet),
        legend: true,
        table_twin: DataTable { columns: vec![column("Across the rib", "Quer zur Rippe", "m", xs), column("Height", "Höhe", "m", zs)] },
    })
}
