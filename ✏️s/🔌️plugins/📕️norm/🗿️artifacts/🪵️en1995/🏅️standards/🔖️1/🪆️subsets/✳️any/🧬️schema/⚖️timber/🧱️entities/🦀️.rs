//! 🧱️ Checks of the typed physical entities of an EN 1995 document: tapered and curved members (6.4), notches and holes (5.2, 6.5), fastener patterns (8.1, 8.3), CLT panels (Annex CLT, 1-2), timber floors (7.2, 7.3), diaphragms (9.2.3, 9.2.4), trusses and bracings (9.2.5). Each entity is turned into the catalogue case rows it stands for (inputs read from the entity and the member or connection it references, in N, mm, N/mm² and degrees), evaluated by the shared design-case bridge and re-homed onto the entity fields, so every verdict and remedy addresses the entity and not a case row. See [`super`].

use super::*;
use semio_s_artifact_norm_en1995_model::{CaseNumber, CaseText};
use semio_s_plugin_norm_compliance::RemedyBound;

/// 🗂️ The collections of the typed entities, in document order.
pub const ENTITY_TABLES: [&str; 9] = ["tapers", "cutouts", "fasteners", "panels", "floors", "diaphragms", "trusses", "bracings", "decks"];

const MM: f64 = 1000.0;
const DEG: f64 = 57.295_779_513_082_32;
const CREEP_HORIZON_YEARS: f64 = 50.0;

/// 🔗️ A case input that stands for one field of an entity (or of the member or connection it references): `case = from + SI × scale`, where `from` is the value of another input of the row (empty: zero) and a negative scale mirrors the field (the effective notch depth `h_ef = h − depth`). A remedy on the input is converted into the stored unit with the inverse, so it is one click for every unit.
struct Binding {
    key: &'static str,
    path: &'static str,
    scale: f64,
    from: &'static str,
}

const fn bind(key: &'static str, path: &'static str, scale: f64) -> Binding {
    Binding { key, path, scale, from: "" }
}

const fn bind_from(key: &'static str, path: &'static str, scale: f64, from: &'static str) -> Binding {
    Binding { key, path, scale, from }
}

fn input_of(row: &CaseRow, key: &str) -> Option<f64> {
    row.numbers.iter().find(|n| n.id == key).map(|n| n.value)
}

fn offset_of(binding: &Binding, row: &CaseRow) -> f64 {
    if binding.from.is_empty() { 0.0 } else { input_of(row, binding.from).unwrap_or(0.0) }
}

fn stored(binding: &Binding, row: &CaseRow, case_value: f64) -> f64 {
    (case_value - offset_of(binding, row)) / binding.scale
}

fn flipped(bound: RemedyBound) -> RemedyBound {
    match bound {
        RemedyBound::AtLeast => RemedyBound::AtMost,
        RemedyBound::AtMost => RemedyBound::AtLeast,
        RemedyBound::Exactly => RemedyBound::Exactly,
        RemedyBound::OneOf => RemedyBound::OneOf,
        RemedyBound::Remove => RemedyBound::Remove,
        RemedyBound::Insert => RemedyBound::Insert,
    }
}

const TAPER: &[Binding] = &[
    bind("alpha_deg", "tapers[id={row}].taperAngleRad", DEG),
    bind("h_ap", "tapers[id={row}].apexDepthM", MM),
    bind("r_in", "tapers[id={row}].innerRadiusM", MM),
    bind("t_lam", "tapers[id={row}].lamellaThicknessM", MM),
    bind("alpha_ap_deg", "tapers[id={row}].apexAngleRad", DEG),
    bind("n_rods", "tapers[id={row}].rods", 1.0),
    bind("r_ax_k", "tapers[id={row}].rodResistanceN", 1.0),
    bind("side", "tapers[id={row}].tensionSide", 1.0),
    bind("b", "members[id={member}].bM", MM),
    bind("h", "members[id={member}].hM", MM),
    bind("class", "members[id={member}].strengthClass", 1.0),
    bind("l", "members[id={member}].spanM", MM),
];

const CUTOUT: &[Binding] = &[
    bind("x", "cutouts[id={row}].distanceM", MM),
    bind("incl", "cutouts[id={row}].slope", 1.0),
    bind_from("h_ef", "cutouts[id={row}].depthM", -MM, "h"),
    bind("hole_d", "cutouts[id={row}].lengthM", MM),
    bind("n_rods", "cutouts[id={row}].rods", 1.0),
    bind("r_ax_k", "cutouts[id={row}].rodResistanceN", 1.0),
    bind("b", "members[id={member}].bM", MM),
    bind("h", "members[id={member}].hM", MM),
    bind("class", "members[id={member}].strengthClass", 1.0),
    bind("l", "members[id={member}].spanM", MM),
    bind("notch_l", "cutouts[id={row}].lengthM", MM),
    bind("hole_x", "cutouts[id={row}].distanceM", MM),
    bind("hole_z", "cutouts[id={row}].centreHeightM", MM),
];

const FASTENER: &[Binding] = &[
    bind("a1", "fasteners[id={row}].spacingParallelM", MM),
    bind("a2", "fasteners[id={row}].spacingPerpendicularM", MM),
    bind("a3t", "fasteners[id={row}].endDistanceM", MM),
    bind("a4c", "fasteners[id={row}].edgeDistanceM", MM),
    bind("n", "fasteners[id={row}].columns", 1.0),
    bind("alpha_deg", "fasteners[id={row}].loadAngleRad", DEG),
    bind("d", "connections[id={connection}].diameterM", MM),
    bind("a4t", "fasteners[id={row}].edgeDistanceM", MM),
    bind("rows", "fasteners[id={row}].lines", 1.0),
    bind("t1", "connections[id={connection}].t1M", MM),
    bind("t2", "connections[id={connection}].t2M", MM),
];

const FASTENER_EXPLICIT: &[Binding] = &[
    bind("alpha_deg", "fasteners[id={row}].loadAngleRad", DEG),
    bind("d", "connections[id={connection}].diameterM", MM),
    bind("t1", "connections[id={connection}].t1M", MM),
    bind("t2", "connections[id={connection}].t2M", MM),
];

const PANEL: &[Binding] = &[bind("l", "panels[id={row}].spanM", MM), bind("class", "panels[id={row}].strengthClass", 1.0), bind("t_req", "panels[id={row}].fireDurationS", 1.0 / 60.0), bind("width_m", "panels[id={row}].widthM", 1.0)];

const FLOOR: &[Binding] = &[
    bind("span_m", "floors[id={row}].spanM", 1.0),
    bind("width_m", "floors[id={row}].widthM", 1.0),
    bind("ei_l", "floors[id={row}].eiLNM2PerM", 1.0),
    bind("ei_b", "floors[id={row}].eiBNM2PerM", 1.0),
    bind("mass_kg_m2", "floors[id={row}].massKgPerM2", 1.0),
    bind("zeta", "floors[id={row}].dampingRatio", 1.0),
];

const DIAPHRAGM: &[Binding] = &[
    bind("f_f_rd", "diaphragms[id={row}].fastenerCapacityN", 1.0),
    bind("b_i", "diaphragms[id={row}].panelWidthM", MM),
    bind("h", "diaphragms[id={row}].wallHeightM", MM),
    bind("s", "diaphragms[id={row}].fastenerSpacingM", MM),
    bind("n_panels", "diaphragms[id={row}].panels", 1.0),
    bind("f_v_ed", "diaphragms[id={row}].shearEdN", 1.0),
    bind("r_hold_down", "diaphragms[id={row}].holdDownN", 1.0),
    bind("f_d", "diaphragms[id={row}].diaphragmForceN", 1.0),
    bind("l_dia", "diaphragms[id={row}].diaphragmLengthM", MM),
    bind("q_v_rd", "diaphragms[id={row}].shearResistanceNPerM", 0.001),
];

const TRUSS: &[Binding] = &[
    bind("n_d", "trusses[id={row}].chordForceN", 1.0),
    bind("l_m", "trusses[id={row}].restraintLengthM", 1.0),
    bind("q_rd", "trusses[id={row}].bracingCapacityNPerM", 1.0),
    bind("span", "trusses[id={row}].spanM", MM),
    bind("height", "trusses[id={row}].heightM", MM),
    bind("bays", "trusses[id={row}].bays", 1.0),
    bind("chord_b", "trusses[id={row}].chordWidthM", MM),
    bind("chord_h", "trusses[id={row}].chordDepthM", MM),
    bind("web_b", "trusses[id={row}].webWidthM", MM),
    bind("web_h", "trusses[id={row}].webDepthM", MM),
];

const BRACING: &[Binding] = &[
    bind("n_braced", "bracings[id={row}].stabilisedCount", 1.0),
    bind("n_d", "bracings[id={row}].compressionForceN", 1.0),
    bind("l_m", "bracings[id={row}].lengthM", 1.0),
    bind("f_rd", "bracings[id={row}].resistanceN", 1.0),
    bind("k_br", "bracings[id={row}].stiffnessNPerM", 0.001),
    bind("spacing", "bracings[id={row}].spacingM", MM),
    bind("brace_b", "bracings[id={row}].braceWidthM", MM),
    bind("brace_h", "bracings[id={row}].braceDepthM", MM),
    bind("member_b", "members[id={member}].bM", MM),
    bind("member_h", "members[id={member}].hM", MM),
];

const MPA: f64 = 1e-6;

const DECK: &[Binding] = &[
    bind("sigma_p_0", "decks[id={row}].initialPrestressPa", MPA),
    bind("loss_relax", "decks[id={row}].lossRelaxation", 1.0),
    bind("loss_creep", "decks[id={row}].lossCreep", 1.0),
    bind("loss_mc", "decks[id={row}].lossMoisture", 1.0),
    bind("tau_h", "decks[id={row}].shearStressPa", MPA),
    bind("bar_spacing", "decks[id={row}].barSpacingM", MM),
    bind("deck_depth", "decks[id={row}].deckDepthM", MM),
    bind("h", "decks[id={row}].deckDepthM", MM),
    bind("f_bar_d", "decks[id={row}].barResistancePa", MPA),
    bind("f_c_90_d", "decks[id={row}].compressionPerpendicularPa", MPA),
    bind("wheel_n", "decks[id={row}].wheelLoadN", 1.0),
    bind("contact_mm", "decks[id={row}].wheelContactM", MM),
    bind("span_m", "decks[id={row}].spanM", 1.0),
    bind("deck_width_m", "decks[id={row}].deckWidthM", 1.0),
    bind("bar_d", "decks[id={row}].barDiameterM", MM),
    bind("plate_side", "decks[id={row}].plateSideM", MM),
    bind("lam_b", "decks[id={row}].laminationWidthM", MM),
];

/// 🏷️ Where the re-homed results land.
struct Dest<'a> {
    collection: &'static str,
    row: &'a str,
    member: &'a str,
    connection: &'a str,
    layers: &'a [PanelLayer],
}

/// 🧮️ The design effects of the governing ULS combinations of a member in the units of the catalogue (N, N·mm).
struct Demand {
    m_nmm: f64,
    v_n: f64,
    q_n_per_mm: f64,
    n_t_n: f64,
    duration: &'static str,
    line: Option<(&'static str, f64)>,
    apex_nmm: Option<f64>,
}

/// 📈️ The catalogue load case and point-load share of the moment line of the ULS combination with the highest moment: simply supported, cantilever and two-span members under a line load and an optional point load at midspan (of each span) or at the free end, and, when the actions are given as forces, the line load and point load that statics derive from the peak moment and shear ([`Profile::from_forces`]); `None` when the forces are no statically admissible combination of both (or the loads act in opposite senses), so the dependent checks are refused instead of assuming a constant moment.
fn load_line(m: &TimberMember, params: &AnnexParams) -> Option<(&'static str, f64)> {
    let combos = enumerate_combos(m, params);
    let c = combos.iter().filter(|c| c.kind == ComboKind::Uls).max_by(|a, b| a.m_ed_nm.abs().total_cmp(&b.m_ed_nm.abs()))?;
    let loaded = c.q_ed_line.abs() > 0.0 || c.f_ed_point.abs() > 0.0;
    let profile = if loaded { Profile::of(m, c.q_ed_line, c.f_ed_point).filter(|_| c.q_ed_line * c.f_ed_point >= 0.0) } else { Profile::from_forces(m, c.m_ed_nm, c.v_ed_n) };
    let profile = profile?;
    let (mq, mf) = profile.moment_weights();
    let case = match (m.support, mq > 0.0) {
        (SupportType::SimplySupported, true) => "udl_ss",
        (SupportType::SimplySupported, false) => "point_mid",
        (SupportType::Cantilever, true) => "cantilever_udl",
        (SupportType::Cantilever, false) => "cantilever_point",
        (SupportType::ContinuousTwoSpan, true) => "two_span_udl",
        (SupportType::ContinuousTwoSpan, false) => "two_span_point",
    };
    (mq + mf > 0.0).then(|| (case, mf / (mq + mf)))
}

/// 📈️ The moment line of a member without any moment: the (trivial) line of its support, so that the moment-line checks have a line to sweep.
fn unloaded_line(m: &TimberMember) -> (&'static str, f64) {
    (
        match m.support {
            SupportType::SimplySupported => "udl_ss",
            SupportType::Cantilever => "cantilever_udl",
            SupportType::ContinuousTwoSpan => "two_span_udl",
        },
        0.0,
    )
}

/// 📐️ The moment at the apex (the middle of the member) in N·mm: the peak moment times the moment line at midlength (1 for a simply supported member with a midspan or uniform load and for the interior support of two spans, 1/4 or 1/2 for a cantilever); `None` without a moment line.
fn apex_moment(m_nmm: f64, line: Option<(&'static str, f64)>) -> Option<f64> {
    line.and_then(|(case, share)| moment_shape(case, share, 0.5)).map(|shape| m_nmm * shape)
}

/// 📏️ The length of the member in metres: the span, twice for the two equal spans of a continuous beam.
fn member_length_m(m: &TimberMember) -> f64 {
    if m.support == SupportType::ContinuousTwoSpan { 2.0 * m.span_m } else { m.span_m }
}

fn duration_name(duration: LoadDuration) -> &'static str {
    match duration {
        LoadDuration::Permanent => "permanent",
        LoadDuration::Long => "long",
        LoadDuration::Medium => "medium",
        LoadDuration::Short => "short",
        LoadDuration::Instantaneous => "instantaneous",
    }
}

fn demand(m: &TimberMember, params: &AnnexParams) -> Demand {
    let combos = enumerate_combos(m, params);
    let peak = |f: fn(&LoadCombo) -> f64| combos.iter().filter(|c| c.kind == ComboKind::Uls).map(f).fold(0.0_f64, f64::max);
    let m_nmm = peak(|c| c.m_ed_nm.abs()) * MM;
    let line = load_line(m, params).or_else(|| (m_nmm == 0.0 && m.span_m > 0.0).then(|| unloaded_line(m)));
    Demand {
        m_nmm,
        v_n: peak(|c| c.v_ed_n.abs()),
        q_n_per_mm: peak(|c| c.q_ed_line.abs()) / MM,
        n_t_n: peak(|c| c.n_t_ed_n),
        duration: governing_uls(&combos).map_or("medium", |c| duration_name(c.duration)),
        line,
        apex_nmm: apex_moment(m_nmm, line),
    }
}

fn case_row(id: &str, entry: &str, label: &(String, String), numbers: &[(&str, f64)], texts: &[(&str, &str)]) -> CaseRow {
    CaseRow {
        id: id.to_string(),
        label_en: label.0.clone(),
        label_de: label.1.clone(),
        entry_id: entry.to_string(),
        numbers: numbers.iter().map(|(key, value)| CaseNumber { id: (*key).to_string(), value: *value }).collect(),
        texts: texts.iter().map(|(key, value)| CaseText { id: (*key).to_string(), value: (*value).to_string() }).collect(),
    }
}

fn home_of<'b>(path: &str, case_id: &str, bindings: &'b [Binding], home: &Dest<'_>) -> (String, Option<&'b Binding>) {
    let prefix = format!("cases[id={case_id}]");
    let Some(rest) = path.strip_prefix(&prefix) else { return (path.to_string(), None) };
    let key = rest.strip_prefix(".numbers[id=").or_else(|| rest.strip_prefix(".texts[id=")).and_then(|tail| tail.split(']').next());
    match key.and_then(|key| bindings.iter().find(|b| b.key == key)) {
        Some(b) => (b.path.replace("{row}", home.row).replace("{member}", home.member).replace("{connection}", home.connection), Some(b)),
        None => match key.and_then(|key| layer_thickness(key, home)) {
            Some((path, binding)) => (path, Some(binding)),
            None => (format!("{}[id={}]", home.collection, home.row), None),
        },
    }
}

static LAYER_THICKNESS: Binding = bind("t", "panels[id={row}].layers[id={layer}].thicknessM", MM);

/// 🧱️ The stored thickness of the layer a case input `t<k>` stands for (the k-th layer of the panel, bottom to top).
fn layer_thickness(key: &str, home: &Dest<'_>) -> Option<(String, &'static Binding)> {
    let layer = home.layers.get(key.strip_prefix('t')?.parse::<usize>().ok()?.checked_sub(1)?)?;
    Some((LAYER_THICKNESS.path.replace("{row}", home.row).replace("{layer}", &layer.id), &LAYER_THICKNESS))
}

fn entity_of(path: &str, fallback: &str) -> String {
    path.split_once("[id=").and_then(|(_, tail)| tail.split(']').next()).map_or_else(|| fallback.to_string(), str::to_string)
}

fn rehome(results: Vec<CheckResult>, row: &CaseRow, bindings: &[Binding], home: &Dest<'_>) -> Vec<CheckResult> {
    results
        .into_iter()
        .map(|mut result| {
            let (path, _) = home_of(&result.subject.path, &row.id, bindings, home);
            result.subject.entity_id = entity_of(&path, home.row);
            result.subject.path = path;
            for remedy in &mut result.remedies {
                let (path, binding) = home_of(&remedy.target.path, &row.id, bindings, home);
                remedy.target.entity_id = entity_of(&path, home.row);
                remedy.target.path = path;
                match binding {
                    Some(b) if remedy.options.is_empty() && b.scale != 0.0 => {
                        remedy.current.value = stored(b, row, remedy.current.value);
                        remedy.required.value = stored(b, row, remedy.required.value);
                        if b.scale < 0.0 {
                            remedy.bound = flipped(remedy.bound);
                        }
                    }
                    Some(_) => {}
                    None => remedy.applicable = false,
                }
            }
            result
        })
        .collect()
}

fn run(annex: AnnexChoice, rows: Vec<(CaseRow, &'static [Binding])>, home: &Dest<'_>) -> Vec<CheckResult> {
    rows.into_iter().flat_map(|(row, bindings)| rehome(evaluate_case_row(annex, &row), &row, bindings, home)).collect()
}

fn dangling(annex: AnnexChoice, collection: &str, row: &str, leaf: &str, label: &(String, String), target: &str, options: Vec<String>) -> CheckResult {
    let subject = SubjectRef::new(row, format!("{collection}[id={row}].{leaf}"), loc(&label.0, &label.1));
    CheckResult::assess(
        format!("en1995.integrity.dangling.{collection}.{leaf}.{row}"),
        "EN 1995 integrity",
        ClauseId::new("EN 1995-1-1", "§1", "ref"),
        subject.clone(),
        loc(&format!("Existing {target}"), &format!("Vorhandenes Bezugselement ({target})")),
    )
    .annex(annex)
    .explanation(loc(&format!("{collection} '{row}' refers to a {target} that does not exist."), &format!("{collection} '{row}' verweist auf ein nicht vorhandenes Element ({target}).")))
    .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
    .remedy(Remedy::one_of(subject, options, loc(&format!("Select an existing {target}."), &format!("Ein vorhandenes Element ({target}) wählen."))))
    .build()
}

fn member_ids(document: &En1995Snapshot) -> Vec<String> {
    document.members.iter().map(|m| m.id.clone()).collect()
}

fn taper_results(document: &En1995Snapshot, params: &AnnexParams, t: &Taper) -> Vec<CheckResult> {
    let annex = document.annex;
    let label = (format!("Tapered member {}", t.id), format!("Verjüngtes Bauteil {}", t.id));
    let Some(m) = document.members.iter().find(|m| m.id == t.member_id) else { return vec![dangling(annex, "tapers", &t.id, "memberId", &label, "member", member_ids(document))] };
    let g = demand(m, params);
    let home = Dest { collection: "tapers", row: &t.id, member: &m.id, connection: "", layers: &[] };
    let id = |entry: &str| format!("tapers:{}:{entry}", t.id);
    let (b, h, sc) = (m.b_m * MM, m.h_m * MM, f64::from(m.service_class));
    let class = m.strength_class.id();
    let mut rows = Vec::new();
    if t.kind == TaperKind::SingleTapered {
        let mut numbers = vec![("m_d", g.m_nmm), ("b", b), ("h", h), ("alpha_deg", t.taper_angle_rad * DEG), ("sc", sc)];
        let mut texts = vec![("class", class), ("side", t.tension_side.id()), ("duration", g.duration)];
        if let Some((load_case, share)) = g.line {
            numbers.extend([("l", m.span_m * MM), ("point_share", share)]);
            texts.push(("load_case", load_case));
        }
        rows.push((case_row(&id("T-30"), "T-30", &label, &numbers, &texts), TAPER));
        return run(annex, rows, &home);
    }
    let h_ap = if t.apex_depth_m > 0.0 { t.apex_depth_m * MM } else { h };
    let alpha_ap = if t.kind == TaperKind::Curved { 0.0 } else { t.apex_angle_rad * DEG };
    let (r_in, t_lam) = (t.inner_radius_m * MM, t.lamella_thickness_m * MM);
    let shape = if t.kind == TaperKind::PitchedCambered { "pitched_cambered" } else { "double_tapered" };
    let texts = [("class", class), ("duration", g.duration)];
    let apex = |numbers: &[(&'static str, f64)]| -> Vec<(&'static str, f64)> { g.apex_nmm.map(|v| ("m_ap", v)).into_iter().chain(numbers.iter().copied()).collect() };
    rows.push((case_row(&id("T-31"), "T-31", &label, &apex(&[("b", b), ("h_ap", h_ap), ("r_in", r_in), ("t_lam", t_lam), ("alpha_ap_deg", alpha_ap), ("sc", sc)]), &texts), TAPER));
    let tau = 1.5 * g.v_n / (b * h_ap).max(1e-9);
    rows.push((
        case_row(&id("T-32"), "T-32", &label, &apex(&[("b", b), ("h_ap", h_ap), ("r_in", r_in), ("alpha_ap_deg", alpha_ap), ("v_beam", b * h_ap * member_length_m(m) * MM), ("p_d", g.q_n_per_mm), ("tau_d", tau), ("sc", sc)]), &[("class", class), ("duration", g.duration), ("shape", shape)]),
        TAPER,
    ));
    if t.rods > 0 {
        rows.push((case_row(&id("T-33"), "T-33", &label, &apex(&[("b", b), ("h_ap", h_ap), ("r_in", r_in), ("alpha_ap_deg", alpha_ap), ("n_rods", f64::from(t.rods)), ("r_ax_k", t.rod_resistance_n), ("sc", sc)]), &texts), TAPER));
    }
    run(annex, rows, &home)
}

fn cutout_results(document: &En1995Snapshot, params: &AnnexParams, c: &Cutout) -> Vec<CheckResult> {
    let annex = document.annex;
    let label = (format!("Cut-out {}", c.id), format!("Ausschnitt {}", c.id));
    let Some(m) = document.members.iter().find(|m| m.id == c.member_id) else { return vec![dangling(annex, "cutouts", &c.id, "memberId", &label, "member", member_ids(document))] };
    let g = demand(m, params);
    let home = Dest { collection: "cutouts", row: &c.id, member: &m.id, connection: "", layers: &[] };
    let id = |entry: &str| format!("cutouts:{}:{entry}", c.id);
    let (b, h, sc) = (m.b_m * MM, m.h_m * MM, f64::from(m.service_class));
    let texts = [("class", m.strength_class.id()), ("duration", g.duration)];
    let mut rows = Vec::new();
    if c.kind == CutoutKind::Hole {
        rows.push((case_row(&id("T-18"), "T-18", &label, &[("n_t", g.n_t_n), ("b", b), ("h", h), ("hole_d", c.length_m * MM), ("hole_count", 1.0), ("sc", sc)], &texts), CUTOUT));
        return run(annex, rows, &home);
    }
    let h_ef = h - c.depth_m * MM;
    let side = if c.face == CutoutFace::Bottom { "support" } else { "other" };
    rows.push((case_row(&id("T-45"), "T-45", &label, &[("v_d", g.v_n), ("b", b), ("h", h), ("h_ef", h_ef), ("x", c.distance_m * MM), ("incl", c.slope), ("sc", sc)], &[("class", texts[0].1), ("duration", g.duration), ("side", side)]), CUTOUT));
    if c.rods > 0 {
        rows.push((case_row(&id("T-46"), "T-46", &label, &[("v_d", g.v_n), ("h", h), ("h_ef", h_ef), ("n_rods", f64::from(c.rods)), ("r_ax_k", c.rod_resistance_n), ("sc", sc)], &texts), CUTOUT));
    }
    run(annex, rows, &home)
}

fn fastener_results(document: &En1995Snapshot, f: &Fastener) -> Vec<CheckResult> {
    let annex = document.annex;
    let label = (format!("Fastener pattern {}", f.id), format!("Verbindungsmittelbild {}", f.id));
    let Some(c) = document.connections.iter().find(|c| c.id == f.connection_id) else { return vec![dangling(annex, "fasteners", &f.id, "connectionId", &label, "connection", document.connections.iter().map(|c| c.id.clone()).collect())] };
    let Some(m) = document.members.iter().find(|m| m.id == f.member_id) else { return vec![dangling(annex, "fasteners", &f.id, "memberId", &label, "member", member_ids(document))] };
    let home = Dest { collection: "fasteners", row: &f.id, member: &m.id, connection: &c.id, layers: &[] };
    let id = |entry: &str| format!("fasteners:{}:{entry}", f.id);
    let rho = properties_for_class(m.strength_class.id()).map_or(350.0, |p| p.rho_k);
    let (d, alpha) = (c.diameter_m * MM, f.load_angle_rad * DEG);
    let kind = fastener_name(c.fastener_type);
    let facts = pattern_facts(f, m.span_m * MM, m.h_m * MM);
    let bindings = if f.positions.is_empty() { FASTENER } else { FASTENER_EXPLICIT };
    let rows = vec![
        (case_row(&id("T-74"), "T-74", &label, &[("d", d), ("alpha_deg", alpha), ("rho_k", rho), ("a1", facts.a1), ("a2", facts.a2), ("a3t", facts.a3t), ("a4c", facts.a4c)], &[("kind", kind)]), bindings),
        (case_row(&id("T-75"), "T-75", &label, &[("n", f64::from(facts.per_row)), ("a1", facts.a1), ("d", d), ("alpha_deg", alpha)], &[("kind", kind)]), bindings),
    ];
    let mut out = run(annex, rows, &home);
    let pattern = u64::from(facts.count);
    if pattern != u64::from(c.number) {
        let subject = conn_subject(c, "number");
        let shape = if f.positions.is_empty() { format!("{} × {}", f.columns, f.lines) } else { format!("{} explicit positions", f.positions.len()) };
        let shape_de = if f.positions.is_empty() { format!("{} × {}", f.columns, f.lines) } else { format!("{} explizite Lagen", f.positions.len()) };
        out.push(
            CheckResult::assess(format!("en1995.8.pattern-count.{}", f.id), "EN 1995-1-1", ClauseId::new("EN 1995-1-1", "§8", "8.1"), subject.clone(), loc("Fastener count of the pattern", "Verbindungsmittelzahl des Bildes"))
                .annex(annex)
                .explanation(loc(&format!("The pattern ({shape}) holds {pattern} fasteners, the connection declares {}.", c.number), &format!("Das Bild ({shape_de}) enthält {pattern} Verbindungsmittel, die Verbindung gibt {} an.", c.number)))
                .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .remedy(Remedy::exactly(subject, Quantity::new(QuantityKind::Dimensionless, f64::from(c.number)), Quantity::new(QuantityKind::Dimensionless, pattern as f64), loc("Set the number of fasteners to the pattern count.", "Die Anzahl der Verbindungsmittel auf die Bildzahl setzen.")))
                .build(),
        );
    }
    out
}

/// 🔩️ The largest pattern the scene draws: columns along the grain and rows across it.
pub const PATTERN_COLUMNS_MAX: u32 = 12;
pub const PATTERN_LINES_MAX: u32 = 6;

/// 🔩️ The most explicit fastener positions the scene draws.
pub const PATTERN_POINTS_MAX: usize = 72;

/// 🔩️ Two fasteners whose bottom-edge distances differ by at most this many mm stand in the same row.
pub const ROW_TOLERANCE_MM: f64 = 0.5;

/// 🔩️ The fastener positions of a pattern in mm from the member end and the bottom edge: the explicit `positions` of an irregular pattern, else row by row the regular pattern whose first fastener stands at the end distance a₃ and the edge distance a₄ and whose others follow at a₁ along and a₂ across the grain.
pub fn pattern_mm(f: &Fastener) -> Vec<(f64, f64)> {
    if !f.positions.is_empty() {
        return f.positions.iter().take(PATTERN_POINTS_MAX).map(|p| (p.x_m * MM, p.y_m * MM)).collect();
    }
    let (columns, lines) = (f.columns.clamp(1, PATTERN_COLUMNS_MAX), f.lines.clamp(1, PATTERN_LINES_MAX));
    (0..lines).flat_map(|j| (0..columns).map(move |i| ((f.end_distance_m + f64::from(i) * f.spacing_parallel_m) * MM, (f.edge_distance_m + f64::from(j) * f.spacing_perpendicular_m) * MM))).collect()
}

/// 🔩️ The count, the longest row and the governing spacings of a pattern in mm: the declared values of a regular pattern, the minima of the actual positions of an irregular one (a₁ between neighbours of one row, a₂ between neighbouring rows, a₃,t of the first fastener, a₄,c to the nearer edge); a missing neighbour never governs (the member span or depth stands in).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PatternFacts {
    pub count: u32,
    pub per_row: u32,
    pub a1: f64,
    pub a2: f64,
    pub a3t: f64,
    pub a4c: f64,
}

/// 🔩️ The positions of an irregular pattern grouped into rows (ascending bottom-edge distance, ascending end distance inside a row).
pub fn pattern_rows_mm(points: &[(f64, f64)]) -> Vec<Vec<(f64, f64)>> {
    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.total_cmp(&b.0)));
    let mut rows: Vec<Vec<(f64, f64)>> = Vec::new();
    for point in sorted {
        match rows.last_mut() {
            Some(row) if (point.1 - row[0].1).abs() <= ROW_TOLERANCE_MM => row.push(point),
            _ => rows.push(vec![point]),
        }
    }
    for row in &mut rows {
        row.sort_by(|a, b| a.0.total_cmp(&b.0));
    }
    rows
}

/// 🔩️ The facts of the pattern of a fastener row; `span_mm` and `depth_mm` belong to the member it stands on.
pub fn pattern_facts(f: &Fastener, span_mm: f64, depth_mm: f64) -> PatternFacts {
    if f.positions.is_empty() {
        return PatternFacts { count: f.columns.saturating_mul(f.lines), per_row: f.columns, a1: f.spacing_parallel_m * MM, a2: f.spacing_perpendicular_m * MM, a3t: f.end_distance_m * MM, a4c: f.edge_distance_m * MM };
    }
    let points: Vec<(f64, f64)> = f.positions.iter().map(|p| (p.x_m * MM, p.y_m * MM)).collect();
    let rows = pattern_rows_mm(&points);
    let along = rows.iter().flat_map(|row| row.windows(2).map(|pair| pair[1].0 - pair[0].0)).fold(span_mm, f64::min);
    let across = rows.windows(2).map(|pair| pair[1][0].1 - pair[0][0].1).fold(depth_mm, f64::min);
    let (low, high) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), p| (low.min(p.1), high.max(p.1)));
    let end = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    PatternFacts { count: u32::try_from(points.len()).unwrap_or(u32::MAX), per_row: u32::try_from(rows.iter().map(Vec::len).max().unwrap_or(0)).unwrap_or(u32::MAX), a1: along, a2: across, a3t: end, a4c: low.min(depth_mm - high) }
}

/// 📐️ The spacing demand of fastener `index` in an irregular pattern from the actual pair distances: a pair is admissible when it keeps a₁ along the grain or a₂ across it, so its demand is `min(a₁ ÷ |Δx|, a₂ ÷ |Δy|)`; the demand of the fastener is the largest over its pairs (at most 1000).
pub fn pairwise_demand(index: usize, positions: &[(f64, f64)], a1: Option<f64>, a2: Option<f64>) -> f64 {
    let (x, y) = positions[index];
    let escape = |required: Option<f64>, gap: f64| required.map_or(0.0, |r| (r / gap.max(1e-9)).min(1.0e3));
    positions.iter().enumerate().filter(|(other, _)| *other != index).map(|(_, p)| escape(a1, (p.0 - x).abs()).min(escape(a2, (p.1 - y).abs()))).fold(0.0_f64, f64::max)
}

/// 🔺️ The joints `(x, z)` in metres and the members `(from joint, to joint, is a chord)` of a truss with parallel chords (Warren: alternating diagonals, joints of the top chord at the bay centres; Pratt and Howe: a vertical at every joint, diagonals falling or rising towards midspan) or of a king-post truss (rafters, tie beam and king post, `bays` ignored). A simply supported truss with pins at the first and last bottom joint and a roller at the last: every layout is statically determinate (`2·joints − 3 = members`).
pub fn truss_layout(kind: TrussKind, span: f64, height: f64, bays: u32) -> (Vec<(f64, f64)>, Vec<(usize, usize, bool)>) {
    let n = bays.max(2) as usize;
    let dx = span / n as f64;
    let mut joints: Vec<(f64, f64)> = (0..=n).map(|i| (i as f64 * dx, 0.0)).collect();
    let mut members: Vec<(usize, usize, bool)> = (0..n).map(|i| (i, i + 1, true)).collect();
    match kind {
        TrussKind::KingPost => {
            joints = vec![(0.0, 0.0), (span / 2.0, 0.0), (span, 0.0), (span / 2.0, height)];
            members = vec![(0, 1, true), (1, 2, true), (0, 3, true), (3, 2, true), (1, 3, false)];
        }
        TrussKind::Warren => {
            joints.extend((0..n).map(|i| ((i as f64 + 0.5) * dx, height)));
            let top = |i: usize| n + 1 + i;
            members.extend((0..n - 1).map(|i| (top(i), top(i + 1), true)));
            for i in 0..n {
                members.push((i, top(i), false));
                members.push((top(i), i + 1, false));
            }
        }
        TrussKind::Pratt | TrussKind::Howe => {
            joints.extend((0..=n).map(|i| (i as f64 * dx, height)));
            let top = |i: usize| n + 1 + i;
            members.extend((0..n).map(|i| (top(i), top(i + 1), true)));
            members.extend((0..=n).map(|i| (i, top(i), false)));
            for i in 0..n {
                let left = 2 * i < n;
                let falling = left == (kind == TrussKind::Pratt);
                members.push(if falling { (top(i), i + 1, false) } else { (i, top(i + 1), false) });
            }
        }
    }
    (joints, members)
}

/// 🧱️ The layers of a CLT panel bottom to top as `(thickness in mm, along the span)`; `None` for a panel without layers, a layer of non-positive or non-finite thickness or an unknown orientation.
pub fn layers_mm(p: &Panel) -> Option<Vec<(f64, bool)>> {
    let layers = p
        .layers
        .iter()
        .map(|layer| {
            let along = layer.orientation == LayerOrientation::Longitudinal;
            let thickness = layer.thickness_m * MM;
            (thickness.is_finite() && thickness > 0.0).then_some((thickness, along))
        })
        .collect::<Option<Vec<_>>>()?;
    (!layers.is_empty()).then_some(layers)
}

/// 🧱️ The case inputs `n_layers`, `t<k>` (mm) and `o<k>` (1 along the span, 0 across) of the layers.
fn layer_numbers(layers: &[(f64, bool)]) -> Vec<(String, f64)> {
    let mut numbers: Vec<(String, f64)> = vec![("n_layers".into(), layers.len() as f64)];
    for (k, (thickness, along)) in layers.iter().enumerate() {
        numbers.push((format!("t{}", k + 1), *thickness));
        numbers.push((format!("o{}", k + 1), if *along { 1.0 } else { 0.0 }));
    }
    numbers
}

/// 🔩 The lower-case name of a fastener type as the kernel tables spell it.
fn fastener_name(t: semio_s_artifact_norm_en1995_model::FastenerType) -> &'static str {
    match t { semio_s_artifact_norm_en1995_model::FastenerType::Nail => "nail", semio_s_artifact_norm_en1995_model::FastenerType::Screw => "screw", semio_s_artifact_norm_en1995_model::FastenerType::Bolt => "bolt", semio_s_artifact_norm_en1995_model::FastenerType::Dowel => "dowel" }
}

/// 🧱️ The lamella class the kernel evaluates a panel with: the strength class of the panel.
fn panel_class(p: &Panel) -> &str {
    p.strength_class.id()
}

fn with_extra<'a>(base: &'a [(String, f64)], extra: &[(&'a str, f64)]) -> Vec<(&'a str, f64)> {
    base.iter().map(|(key, value)| (key.as_str(), *value)).chain(extra.iter().copied()).collect()
}

fn panel_results(document: &En1995Snapshot, p: &Panel) -> Vec<CheckResult> {
    let annex = document.annex;
    let label = (format!("CLT panel {}", p.id), format!("BSP-Platte {}", p.id));
    let home = Dest { collection: "panels", row: &p.id, member: "", connection: "", layers: &p.layers };
    let Some(layers) = layers_mm(p) else {
        let subject = SubjectRef::new(&p.id, format!("panels[id={}].layers", p.id), loc(&label.0, &label.1));
        return vec![CheckResult::assess(format!("en1995.clt.layers.{}", p.id), "EN 1995-1-1", ClauseId::new("EN 1995-1-1", "Annex CLT", "layers"), subject.clone(), loc("CLT layers", "BSP-Lagen"))
            .annex(annex)
            .explanation(loc("A panel needs at least one layer; every layer needs a positive thickness and the orientation longitudinal or transverse.", "Eine Platte braucht mindestens eine Lage; jede Lage braucht eine positive Dicke und die Faserrichtung längs oder quer."))
            .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
            .remedy(advice(subject, "Enter at least one layer with a positive thickness and the orientation longitudinal or transverse.", "Mindestens eine Lage mit positiver Dicke und der Faserrichtung längs oder quer eingeben."))
            .build()];
    };
    let id = |entry: &str| format!("panels:{}:{entry}", p.id);
    let numbers = layer_numbers(&layers);
    let m_d = p.load_pa * p.span_m * p.span_m / 8.0;
    let v_d = p.load_pa * p.span_m / 2.0 / MM;
    let ratio = semio_s_plugin_norm_compliance::ndp_number("ndp.en1995.panel.fire-load-ratio", annex);
    let texts = [("class", panel_class(p))];
    let mut rows = vec![(case_row(&id("T-97"), "T-97", &label, &with_extra(&numbers, &[("l", p.span_m * MM), ("m_d", m_d), ("v_d", v_d)]), &texts), PANEL)];
    let mut out = mixed_grade(annex, p, &label);
    out.extend(strength_class_reference(annex, "panels", &p.id, "strengthClass", panel_class(p), &label));
    for layer in &p.layers {
        out.extend(strength_class_reference(annex, "panels", &p.id, &format!("layers[id={}].grade", layer.id), &layer.grade, &label));
    }
    out.extend(symmetry_note(annex, p, &label, &layers));
    if p.fire_duration_s > 0.0 {
        rows.push((case_row(&id("T-107"), "T-107", &label, &with_extra(&numbers, &[("t_req", p.fire_duration_s / 60.0), ("m_fi", ratio * m_d)]), &texts), PANEL));
    }
    for mut result in run(annex, rows, &home) {
        if result.id.contains("t-107") && !result.ndps.iter().any(|read| read == "ndp.en1995.panel.fire-load-ratio") {
            result.ndps.push("ndp.en1995.panel.fire-load-ratio".to_string());
        }
        out.push(result);
    }
    out
}

/// 🧱️ True unless the layers form a symmetric layup of an odd number of layers: the γ-method of the kernel (Annex B) is derived for it.
pub fn symmetric_layup(layers: &[(f64, bool)]) -> bool {
    layers.len() % 2 == 1 && layers.iter().zip(layers.iter().rev()).all(|(a, b)| (a.0 - b.0).abs() < 1e-9 && a.1 == b.1)
}

/// 🧱️ A warning when the layup is not symmetric with an odd number of layers: the effective stiffness of the kernel assumes it.
fn symmetry_note(annex: AnnexChoice, p: &Panel, label: &(String, String), layers: &[(f64, bool)]) -> Vec<CheckResult> {
    if symmetric_layup(layers) {
        return Vec::new();
    }
    let subject = SubjectRef::new(&p.id, format!("panels[id={}].layers", p.id), loc(&label.0, &label.1));
    vec![CheckResult::assess(format!("en1995.clt.symmetry.{}", p.id), "EN 1995-1-1", ClauseId::new("EN 1995-1-1", "Annex CLT", "layers"), subject.clone(), loc("CLT layup symmetry", "BSP-Aufbau Symmetrie"))
        .remedy(advice(subject, "Rebuild the layup symmetric about the mid-plane with an odd number of layers.", "Den Aufbau symmetrisch zur Mittelebene mit ungerader Lagenzahl umbauen."))
        .annex(annex)
        .explanation(loc("The effective stiffness (γ-method) assumes a symmetric layup with an odd number of layers; the result is an approximation for this layup.", "Die wirksame Steifigkeit (γ-Verfahren) setzt einen symmetrischen Aufbau mit ungerader Lagenzahl voraus; das Ergebnis ist für diesen Aufbau eine Näherung."))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Warning)
        .build()]
}

/// 🧱️ A warning when a layer carries another grade than the panel: the kernel evaluates the section with the strength class of the panel.
fn mixed_grade(annex: AnnexChoice, p: &Panel, label: &(String, String)) -> Vec<CheckResult> {
    let class = panel_class(p);
    if p.layers.iter().all(|layer| layer.grade == class) {
        return Vec::new();
    }
    let subject = SubjectRef::new(&p.id, format!("panels[id={}].layers", p.id), loc(&label.0, &label.1));
    vec![CheckResult::assess(format!("en1995.clt.mixed-grade.{}", p.id), "EN 1995-1-1", ClauseId::new("EN 1995-1-1", "Annex CLT", "layers"), subject.clone(), loc("CLT layer grades", "BSP-Lagenklassen"))
        .remedy(advice(subject, &format!("Give every layer the grade of the panel class {class}, or change the panel class."), &format!("Jeder Lage die Klasse der Plattenklasse {class} geben oder die Plattenklasse ändern.")))
        .annex(annex)
        .explanation(loc(&format!("A layer carries another grade than the panel class {class}; the section is evaluated with the panel class."), &format!("Eine Lage hat eine andere Klasse als die Plattenklasse {class}; der Querschnitt wird mit der Plattenklasse berechnet.")))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Warning)
        .build()]
}

/// 🧮️ The method letter the T-94 case input `method` expects.
fn method_letter(method: DiaphragmMethod) -> &'static str {
    match method {
        DiaphragmMethod::MethodA => "A",
        DiaphragmMethod::MethodB => "B",
    }
}

/// 🚧️ A failed layout check: a positive length, count or force the drawing and the checks cannot do without.
fn layout_failure(annex: AnnexChoice, collection: &str, row: &str, leaf: &str, label: &(String, String), clause: (&str, &str), explanation: (&str, &str)) -> CheckResult {
    layout_failure_in("EN 1995-1-1", annex, collection, row, leaf, label, clause, explanation)
}

/// 🩹 An advisory remedy that names what to change but is not a one-click write.
fn advice(target: SubjectRef, en: &str, de: &str) -> Remedy {
    Remedy { applicable: false, ..Remedy::one_of(target, Vec::new(), loc(en, de)) }
}

fn layout_failure_in(part: &str, annex: AnnexChoice, collection: &str, row: &str, leaf: &str, label: &(String, String), clause: (&str, &str), explanation: (&str, &str)) -> CheckResult {
    let subject = SubjectRef::new(row, format!("{collection}[id={row}].{leaf}"), loc(&label.0, &label.1));
    CheckResult::assess(format!("en1995.{collection}.layout.{row}"), part, ClauseId::new(part, clause.0, clause.1), subject.clone(), loc("Layout of the entity", "Anordnung des Elements"))
        .annex(annex)
        .explanation(loc(explanation.0, explanation.1))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .remedy(advice(subject, explanation.0, explanation.1))
        .build()
}

fn diaphragm_results(document: &En1995Snapshot, d: &Diaphragm) -> Vec<CheckResult> {
    let annex = document.annex;
    let label = (format!("Diaphragm {}", d.id), format!("Scheibe {}", d.id));
    let home = Dest { collection: "diaphragms", row: &d.id, member: "", connection: "", layers: &[] };
    let id = |entry: &str| format!("diaphragms:{}:{entry}", d.id);
    let texts = [("method", method_letter(d.method))];
    let wall = d.kind == DiaphragmKind::Wall;
    if wall && (d.panel_width_m <= 0.0 || d.wall_height_m <= 0.0 || d.fastener_spacing_m <= 0.0 || d.panels == 0) {
        return vec![layout_failure(annex, "diaphragms", &d.id, "panelWidthM", &label, ("9.2.4", "racking"), ("A wall diaphragm needs a positive panel width, wall height and fastener spacing and at least one panel.", "Eine Wandscheibe braucht eine positive Plattenbreite, Wandhöhe und Verbindungsmittelabstand und mindestens eine Platte."))];
    }
    if !wall && d.diaphragm_length_m <= 0.0 {
        return vec![layout_failure(annex, "diaphragms", &d.id, "diaphragmLengthM", &label, ("9.2.3", "shear-flow"), ("A floor or roof diaphragm needs a positive length.", "Eine Decken- oder Dachscheibe braucht eine positive Länge."))];
    }
    let numbers: Vec<(&str, f64)> = if wall {
        vec![("f_f_rd", d.fastener_capacity_n), ("b_i", d.panel_width_m * MM), ("h", d.wall_height_m * MM), ("s", d.fastener_spacing_m * MM), ("n_panels", f64::from(d.panels)), ("f_v_ed", d.shear_ed_n), ("r_hold_down", d.hold_down_n)]
    } else {
        vec![("f_d", d.diaphragm_force_n), ("l_dia", d.diaphragm_length_m * MM), ("q_v_rd", d.shear_resistance_n_per_m / MM)]
    };
    run(annex, vec![(case_row(&id("T-94"), "T-94", &label, &numbers, &texts), DIAPHRAGM)], &home)
}

fn truss_results(document: &En1995Snapshot, t: &Truss) -> Vec<CheckResult> {
    let annex = document.annex;
    let label = (format!("Truss {}", t.id), format!("Fachwerkträger {}", t.id));
    let home = Dest { collection: "trusses", row: &t.id, member: "", connection: "", layers: &[] };
    if t.span_m <= 0.0 || t.height_m <= 0.0 || t.bays < 2 || t.restraint_length_m <= 0.0 {
        return vec![layout_failure(annex, "trusses", &t.id, "spanM", &label, ("9.2.5", "truss-layout"), ("A truss needs a positive span, height and restraint length and at least two bays.", "Ein Fachwerkträger braucht eine positive Spannweite, Höhe und Haltelänge und mindestens zwei Felder."))];
    }
    let numbers = [("n_braced", 1.0), ("n_d", t.chord_force_n), ("l_m", t.restraint_length_m), ("q_rd", t.bracing_capacity_n_per_m)];
    run(annex, vec![(case_row(&format!("trusses:{}:T-95", t.id), "T-95", &label, &numbers, &[("product", t.product.id())]), TRUSS)], &home)
}

fn bracing_results(document: &En1995Snapshot, b: &Bracing) -> Vec<CheckResult> {
    let annex = document.annex;
    let label = (format!("Bracing {}", b.id), format!("Aussteifung {}", b.id));
    let Some(m) = document.members.iter().find(|m| m.id == b.member_id) else { return vec![dangling(annex, "bracings", &b.id, "memberId", &label, "member", member_ids(document))] };
    let home = Dest { collection: "bracings", row: &b.id, member: &m.id, connection: "", layers: &[] };
    if b.length_m <= 0.0 || b.stabilised_count == 0 || b.stiffness_n_per_m <= 0.0 {
        return vec![layout_failure(annex, "bracings", &b.id, "lengthM", &label, ("9.2.5", "bracing-layout"), ("A bracing needs a positive length and stiffness and at least one stabilised member.", "Eine Aussteifung braucht eine positive Länge und Steifigkeit und mindestens ein ausgesteiftes Bauteil."))];
    }
    let numbers = [("n_braced", f64::from(b.stabilised_count)), ("n_d", b.compression_force_n), ("l_m", b.length_m), ("f_rd", b.resistance_n), ("k_br", b.stiffness_n_per_m / MM)];
    run(annex, vec![(case_row(&format!("bracings:{}:T-96", b.id), "T-96", &label, &numbers, &[]), BRACING)], &home)
}

/// 🗜️ The case numbers of a stress-laminated deck in the units of T-122: stresses in N/mm², lengths in mm, the bar area from the bar diameter and the plate area from the plate side.
fn deck_numbers(d: &Deck) -> Vec<(&'static str, f64)> {
    let (bar_d, side) = (d.bar_diameter_m * MM, d.plate_side_m * MM);
    vec![
        ("sigma_p_0", d.initial_prestress_pa * MPA),
        ("loss_relax", d.loss_relaxation),
        ("loss_creep", d.loss_creep),
        ("loss_mc", d.loss_moisture),
        ("tau_h", d.shear_stress_pa * MPA),
        ("bar_spacing", d.bar_spacing_m * MM),
        ("deck_depth", d.deck_depth_m * MM),
        ("bar_area", std::f64::consts::FRAC_PI_4 * bar_d * bar_d),
        ("f_bar_d", d.bar_resistance_pa * MPA),
        ("plate_a", side * side),
        ("wheel_n", d.wheel_load_n),
        ("contact_mm", d.wheel_contact_m * MM),
        ("h", d.deck_depth_m * MM),
        ("f_c_90_d", d.compression_perpendicular_pa * MPA),
    ]
}

fn deck_results(document: &En1995Snapshot, d: &Deck) -> Vec<CheckResult> {
    let annex = document.annex;
    let label = (format!("Stress-laminated deck {}", d.id), format!("Vorgespanntes Brettstapeldeck {}", d.id));
    let home = Dest { collection: "decks", row: &d.id, member: "", connection: "", layers: &[] };
    let losses = [d.loss_relaxation, d.loss_creep, d.loss_moisture];
    let positive = [d.span_m, d.deck_width_m, d.deck_depth_m, d.lamination_width_m, d.bar_spacing_m, d.bar_diameter_m, d.plate_side_m, d.initial_prestress_pa, d.bar_resistance_pa];
    if positive.iter().any(|v| !v.is_finite() || *v <= 0.0) || losses.iter().any(|v| !(0.0..1.0).contains(v)) {
        return vec![layout_failure_in("EN 1995-2", annex, "decks", &d.id, "spanM", &label, ("Annex A", "deck-layout"), ("A stress-laminated deck needs positive dimensions, prestress and bar resistance and prestress losses from 0 up to (excluding) 1.", "Ein vorgespanntes Brettstapeldeck braucht positive Abmessungen, Vorspannung und Stabwiderstand und Vorspannverluste von 0 bis ausschließlich 1."))];
    }
    run(annex, vec![(case_row(&format!("decks:{}:T-122", d.id), "T-122", &label, &deck_numbers(d), &[]), DECK)], &home).into_iter().map(|result| deck_remedy(d, result)).collect()
}

/// 🩹 The catalogue sizes every failing result on the first lever input, the initial prestress; that helps the prestress and friction checks but worsens the bar force and the plate bearing, so those carry their own remedy (stronger bar, wider anchor plate by the square root of the utilisation) and the wheel spread none.
fn deck_remedy(d: &Deck, mut result: CheckResult) -> CheckResult {
    let sizing = if result.id.starts_with("en1995.t-122.bar_force") {
        Some(("barResistancePa", QuantityKind::Stress, d.bar_resistance_pa, 1.0, loc("Raise the design bar resistance", "Bemessungswiderstand des Stabes erhöhen"), loc("Use a stronger bar or a larger bar diameter.", "Einen stärkeren oder dickeren Stab wählen.")))
    } else if result.id.starts_with("en1995.t-122.plate_bearing") {
        Some(("plateSideM", QuantityKind::Length, d.plate_side_m, 0.5, loc("Widen the anchor plate", "Ankerplatte verbreitern"), loc("Use a larger anchor plate to spread the bar force.", "Eine größere Ankerplatte zur Verteilung der Stabkraft wählen.")))
    } else if result.id.starts_with("en1995.t-122.wheel_spread") {
        result.remedies.clear();
        None
    } else {
        None
    };
    if let Some((leaf, kind, current, exponent, subject_label, advice)) = sizing {
        result.remedies.clear();
        if result.utilization > 1.0 {
            let subject = SubjectRef::new(&d.id, format!("decks[id={}].{leaf}", d.id), subject_label);
            result.remedies.push(Remedy::at_least(subject, Quantity::new(kind, current), Quantity::new(kind, current * result.utilization.powf(exponent)), advice));
        }
    }
    result
}

/// 🗂️ The strength class of a layer or panel is an id of the strength-class catalogue; an unknown id is a failed reference with the catalogue as options.
fn strength_class_reference(annex: AnnexChoice, collection: &str, row: &str, leaf: &str, class: &str, label: &(String, String)) -> Vec<CheckResult> {
    if strength_row(class).is_some() {
        return Vec::new();
    }
    let subject = SubjectRef::new(row, format!("{collection}[id={row}].{leaf}"), loc(&label.0, &label.1));
    vec![CheckResult::assess(format!("en1995.catalogue.strength-class.{collection}.{row}"), "EN 1995-1-1", ClauseId::new("EN 1995-1-1", "§3", "3.2"), subject.clone(), loc("Strength class from the catalogue", "Festigkeitsklasse aus dem Katalog"))
        .annex(annex)
        .explanation(loc(&format!("'{class}' is not a strength class of the catalogue (EN 338, EN 14080)."), &format!("'{class}' ist keine Festigkeitsklasse des Katalogs (EN 338, EN 14080).")))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .remedy(Remedy::one_of(subject, strength_class_names().into_iter().map(str::to_string).collect(), loc("Select a strength class of the catalogue.", "Eine Festigkeitsklasse des Katalogs wählen.")))
        .build()]
}

fn floor_member<'a>(document: &'a En1995Snapshot, f: &Floor) -> Option<&'a TimberMember> {
    document.members.iter().find(|m| m.id == f.member_id)
}

/// ⏳ The creep deflection time curve of a floor strip in m at `years` after loading (see [`creep_deflection`]); `k_def` follows the joist material and service class.
pub fn floor_creep_at(document: &En1995Snapshot, f: &Floor, years: f64) -> f64 {
    let (w_g, w_q) = (floor_strip_deflection(f.permanent_load_pa, f.span_m, f.ei_l_n_m2_per_m), floor_strip_deflection(f.imposed_load_pa, f.span_m, f.ei_l_n_m2_per_m));
    creep_deflection(w_g, w_q, f.psi_2, floor_k_def(document, f), years)
}

/// 🧮️ k_def of the floor: from the joist member's product and service class, otherwise solid timber in service class 1.
pub fn floor_k_def(document: &En1995Snapshot, f: &Floor) -> f64 {
    let (material, sc) = floor_member(document, f).and_then(|m| properties_for_class(m.strength_class.id()).map(|p| (product_key(p.product), m.service_class))).unwrap_or(("solid", 1));
    k_def_for(material, sc).unwrap_or(0.0)
}

fn floor_creep_check(document: &En1995Snapshot, f: &Floor, label: &(String, String)) -> CheckResult {
    let annex = document.annex;
    let subject = SubjectRef::new(&f.id, format!("floors[id={}].eiLNM2PerM", f.id), loc(&label.0, &label.1));
    let divisor = |de: f64, en: f64| if annex == AnnexChoice::De { de } else { en };
    let limit = f.span_m / divisor(200.0, 250.0);
    let k_def = floor_k_def(document, f);
    let w_fin = floor_creep_at(document, f, CREEP_HORIZON_YEARS);
    let stations = (0..=10)
        .map(|i| {
            let s = f64::from(i) / 10.0;
            let years = s * CREEP_HORIZON_YEARS;
            Station::ratio_of(&[s], Quantity::new(QuantityKind::Length, floor_creep_at(document, f, years)), Quantity::new(QuantityKind::Length, limit)).with(Quantity::new(QuantityKind::Time, years * 365.25 * 86_400.0))
        })
        .collect();
    let mut creep = CheckResult::assess(format!("en1995.7.2.floor-creep.{}", f.id), "EN 1995-1-1", ClauseId::new("EN 1995-1-1", "§7.2", "7.2"), subject.clone(), loc("Final floor deflection with creep", "Enddurchbiegung der Decke mit Kriechen"))
        .utilization(Quantity::new(QuantityKind::Length, w_fin), Quantity::new(QuantityKind::Length, limit))
        .annex(annex)
        .ndps(&["ndp.en1995.creep.time-constant-years"])
        .explanation(loc(
            &format!("w_fin = w_G (1 + k_def) + w_Q (1 + ψ₂ k_def) = {:.2} mm with k_def = {k_def:.2} against L/{} = {:.2} mm; the intermediate curve uses an unverified time constant.", w_fin * MM, divisor(200.0, 250.0), limit * MM),
            &format!("w_fin = w_G (1 + k_def) + w_Q (1 + ψ₂ k_def) = {:.2} mm mit k_def = {k_def:.2} gegen L/{} = {:.2} mm; die Zwischenkurve nutzt eine ungeprüfte Zeitkonstante.", w_fin * MM, divisor(200.0, 250.0), limit * MM),
        ))
        .field("creep-time", stations);
    if w_fin > limit {
        let ei = Quantity::new(QuantityKind::Dimensionless, f.ei_l_n_m2_per_m);
        let required = Quantity::new(QuantityKind::Dimensionless, f.ei_l_n_m2_per_m * w_fin / limit);
        creep = creep.remedy(Remedy::at_least(subject, ei, required, loc(&format!("Increase the floor stiffness EI from {:.0} to at least {:.0} N·m²/m (deeper joists or a stiffer product) so that w_fin stays within L/{}.", ei.value, required.value, divisor(200.0, 250.0)), &format!("Die Deckensteifigkeit EI von {:.0} auf mindestens {:.0} N·m²/m erhöhen (höhere Balken oder steiferes Produkt), damit w_fin innerhalb von L/{} bleibt.", ei.value, required.value, divisor(200.0, 250.0)))));
    }
    creep.build()
}

fn floor_results(document: &En1995Snapshot, f: &Floor) -> Vec<CheckResult> {
    let annex = document.annex;
    let label = (format!("Timber floor {}", f.id), format!("Holzdecke {}", f.id));
    let Some(m) = floor_member(document, f) else { return vec![dangling(annex, "floors", &f.id, "memberId", &label, "member", member_ids(document))] };
    let home = Dest { collection: "floors", row: &f.id, member: &m.id, connection: "", layers: &[] };
    let id = |entry: &str| format!("floors:{}:{entry}", f.id);
    let (span, width, ei_l, ei_b, mass) = (f.span_m, f.width_m, f.ei_l_n_m2_per_m, f.ei_b_n_m2_per_m, f.mass_kg_per_m2);
    let rows = vec![
        (case_row(&id("T-63"), "T-63", &label, &[("span_m", span), ("ei_l", ei_l), ("mass_kg_m2", mass)], &[]), FLOOR),
        (case_row(&id("T-64"), "T-64", &label, &[("span_m", span), ("width_m", width), ("ei_l", ei_l), ("ei_b", ei_b)], &[]), FLOOR),
        (case_row(&id("T-65"), "T-65", &label, &[("span_m", span), ("width_m", width), ("ei_l", ei_l), ("ei_b", ei_b), ("mass_kg_m2", mass), ("zeta", f.damping_ratio)], &[]), FLOOR),
    ];
    let mut out: Vec<CheckResult> = run(annex, rows, &home)
        .into_iter()
        .map(|mut result| {
            for id in ["ndp.en1995.floor.f1-limit", "ndp.en1995.floor.unit-load-a"] {
                if !result.ndps.iter().any(|read| read == id) {
                    result.ndps.push(id.to_string());
                }
            }
            result
        })
        .collect();
    out.push(floor_creep_check(document, f, &label));
    out
}

/// 🎯️ The checks of row `index` of the entity collection `collection`; empty for an unknown collection or index.
pub fn evaluate_entity(document: &En1995Snapshot, collection: &str, index: usize) -> Vec<CheckResult> {
    let params = AnnexParams::new(document.annex, document.reliability_class).with_uls_equation(document.uls_equation);
    match collection {
        "tapers" => document.tapers.get(index).map(|t| taper_results(document, &params, t)),
        "cutouts" => document.cutouts.get(index).map(|c| cutout_results(document, &params, c)),
        "fasteners" => document.fasteners.get(index).map(|f| fastener_results(document, f)),
        "panels" => document.panels.get(index).map(|p| panel_results(document, p)),
        "floors" => document.floors.get(index).map(|f| floor_results(document, f)),
        "diaphragms" => document.diaphragms.get(index).map(|d| diaphragm_results(document, d)),
        "trusses" => document.trusses.get(index).map(|t| truss_results(document, t)),
        "bracings" => document.bracings.get(index).map(|b| bracing_results(document, b)),
        "decks" => document.decks.get(index).map(|d| deck_results(document, d)),
        _ => None,
    }
    .unwrap_or_default()
}

/// 🧾 The ids of the rows of an entity collection, in document order.
pub fn entity_ids(document: &En1995Snapshot, collection: &str) -> Vec<String> {
    match collection {
        "tapers" => document.tapers.iter().map(|r| r.id.clone()).collect(),
        "cutouts" => document.cutouts.iter().map(|r| r.id.clone()).collect(),
        "fasteners" => document.fasteners.iter().map(|r| r.id.clone()).collect(),
        "panels" => document.panels.iter().map(|r| r.id.clone()).collect(),
        "floors" => document.floors.iter().map(|r| r.id.clone()).collect(),
        "diaphragms" => document.diaphragms.iter().map(|r| r.id.clone()).collect(),
        "trusses" => document.trusses.iter().map(|r| r.id.clone()).collect(),
        "bracings" => document.bracings.iter().map(|r| r.id.clone()).collect(),
        "decks" => document.decks.iter().map(|r| r.id.clone()).collect(),
        _ => Vec::new(),
    }
}

/// 🔗️ A case input of a drawing row resolved to the document field it stands for (`case = offset + SI × scale`; the scale is negative for a mirrored field and `offset` is the value of the input it counts from).
#[derive(Clone, Debug, PartialEq)]
pub struct Home {
    pub key: String,
    pub path: String,
    pub scale: f64,
    pub offset: f64,
}

impl Home {
    /// 🔁️ The affine map `case = base + k·s` of a handle or marker, rewritten to the stored value `SI = base' + k'·s`.
    pub fn stored(&self, base: f64, k: f64) -> (f64, f64) {
        ((base - self.offset) / self.scale, k / self.scale)
    }

    /// 📏️ The unit of the stored field when the scale is one of the displayed conversions (`m` for millimetres, `rad` for degrees), else none.
    pub fn unit(&self) -> Option<&'static str> {
        if (self.scale.abs() - MM).abs() < 1e-9 {
            Some("m")
        } else if (self.scale.abs() - DEG).abs() < 1e-9 {
            Some("rad")
        } else {
            None
        }
    }
}

fn homes(bindings: &[Binding], home: &HomeIds<'_>, row: &CaseRow) -> Vec<Home> {
    bindings.iter().map(|b| Home { key: b.key.to_string(), path: b.path.replace("{row}", home.row).replace("{member}", home.member).replace("{connection}", home.connection), scale: b.scale, offset: offset_of(b, row) }).collect()
}

/// 🆔️ The ids a binding path is resolved against.
pub struct HomeIds<'a> {
    pub row: &'a str,
    pub member: &'a str,
    pub connection: &'a str,
}

fn drawing(row: &str, entry: &str, label: &(String, String), numbers: &[(&str, f64)], texts: &[(&str, &str)]) -> CaseRow {
    case_row(row, entry, label, numbers, texts)
}

/// 🎨️ The case row an entity is drawn as (the case scene engine draws tapered and curved profiles, notches, fastener grids, CLT layups with their char layers and floor plate modes from case rows): `row.id` is the entity id, `entry_id` the catalogue entry whose diagrams apply, and the homes tell which document field each numeric input writes to. `None` when a referenced member or connection is missing or the layup is malformed.
pub fn drawing_row(document: &En1995Snapshot, collection: &str, index: usize) -> Option<(CaseRow, Vec<Home>)> {
    match collection {
        "tapers" => {
            let t = document.tapers.get(index)?;
            let m = document.members.iter().find(|m| m.id == t.member_id)?;
            let label = (format!("Tapered member {}", t.id), format!("Verjüngtes Bauteil {}", t.id));
            let (b, h, l) = (m.b_m * MM, m.h_m * MM, m.span_m * MM);
            let class = m.strength_class.id();
            let row = if t.kind == TaperKind::SingleTapered {
                let params = AnnexParams::new(document.annex, document.reliability_class).with_uls_equation(document.uls_equation);
                let mut numbers = vec![("b", b), ("h", h), ("l", l), ("alpha_deg", t.taper_angle_rad * DEG)];
                let mut texts = vec![("class", class), ("side", t.tension_side.id())];
                if let Some((load_case, share)) = load_line(m, &params) {
                    numbers.push(("point_share", share));
                    texts.push(("load_case", load_case));
                }
                drawing(&t.id, "T-30", &label, &numbers, &texts)
            } else {
                let h_ap = if t.apex_depth_m > 0.0 { t.apex_depth_m * MM } else { h };
                let alpha = if t.kind == TaperKind::Curved { 0.0 } else { t.apex_angle_rad * DEG };
                drawing(&t.id, "T-31", &label, &[("b", b), ("h_ap", h_ap), ("l", l), ("r_in", t.inner_radius_m * MM), ("t_lam", t.lamella_thickness_m * MM), ("alpha_ap_deg", alpha), ("n_rods", f64::from(t.rods))], &[("class", class)])
            };
            let homes = homes(TAPER, &HomeIds { row: &t.id, member: &m.id, connection: "" }, &row);
            Some((row, homes))
        }
        "cutouts" => {
            let c = document.cutouts.get(index)?;
            let m = document.members.iter().find(|m| m.id == c.member_id)?;
            let label = (format!("Cut-out {}", c.id), format!("Ausschnitt {}", c.id));
            let (b, h, l) = (m.b_m * MM, m.h_m * MM, m.span_m * MM);
            let class = m.strength_class.id();
            let row = if c.kind == CutoutKind::Hole {
                drawing(&c.id, "T-18", &label, &[("b", b), ("h", h), ("l", l), ("hole_d", c.length_m * MM), ("hole_x", c.distance_m * MM), ("hole_z", c.centre_height_m * MM)], &[("class", class)])
            } else {
                drawing(&c.id, "T-45", &label, &[("b", b), ("h", h), ("l", l), ("h_ef", h - c.depth_m * MM), ("x", c.distance_m * MM), ("incl", c.slope), ("notch_l", c.length_m * MM), ("n_rods", f64::from(c.rods))], &[("class", class), ("side", if c.face == CutoutFace::Bottom { "support" } else { "other" })])
            };
            let homes = homes(CUTOUT, &HomeIds { row: &c.id, member: &m.id, connection: "" }, &row);
            Some((row, homes))
        }
        "fasteners" => {
            let f = document.fasteners.get(index)?;
            let c = document.connections.iter().find(|c| c.id == f.connection_id)?;
            let m = document.members.iter().find(|m| m.id == f.member_id)?;
            let label = (format!("Fastener pattern {}", f.id), format!("Verbindungsmittelbild {}", f.id));
            let facts = pattern_facts(f, m.span_m * MM, m.h_m * MM);
            let mut numbers: Vec<(String, f64)> = [("d", c.diameter_m * MM), ("t1", c.t1_m * MM), ("t2", (if c.steel_plate { c.steel_plate_thickness_m } else { c.t2_m }) * MM), ("a1", facts.a1), ("a2", facts.a2), ("a3t", facts.a3t), ("a4t", facts.a4c), ("n", f64::from(facts.per_row)), ("rows", if f.positions.is_empty() { f64::from(f.lines) } else { 0.0 }), ("explicit", if f.positions.is_empty() { 0.0 } else { 1.0 }), ("alpha_deg", f.load_angle_rad * DEG)]
                .into_iter()
                .map(|(key, value)| (key.to_string(), value))
                .collect();
            for (k, (x, y)) in pattern_mm(f).into_iter().enumerate() {
                numbers.push((format!("px{}", k + 1), x));
                numbers.push((format!("py{}", k + 1), y));
            }
            let flat: Vec<(&str, f64)> = numbers.iter().map(|(key, value)| (key.as_str(), *value)).collect();
            let row = drawing(&f.id, "T-74", &label, &flat, &[("kind", fastener_name(c.fastener_type)), ("class", m.strength_class.id())]);
            let mut homes = homes(if f.positions.is_empty() { FASTENER } else { FASTENER_EXPLICIT }, &HomeIds { row: &f.id, member: &m.id, connection: &c.id }, &row);
            for (k, point) in f.positions.iter().take(PATTERN_POINTS_MAX).enumerate() {
                homes.push(Home { key: format!("px{}", k + 1), path: format!("fasteners[id={}].positions[id={}].xM", f.id, point.id), scale: MM, offset: 0.0 });
                homes.push(Home { key: format!("py{}", k + 1), path: format!("fasteners[id={}].positions[id={}].yM", f.id, point.id), scale: MM, offset: 0.0 });
            }
            Some((row, homes))
        }
        "panels" => {
            let p = document.panels.get(index)?;
            let layers = layers_mm(p)?;
            let label = (format!("CLT panel {}", p.id), format!("BSP-Platte {}", p.id));
            let mut numbers = layer_numbers(&layers);
            numbers.extend([("l".to_string(), p.span_m * MM), ("width_m".to_string(), p.width_m)]);
            if p.fire_duration_s > 0.0 {
                numbers.push(("t_req".into(), p.fire_duration_s / 60.0));
            }
            let flat: Vec<(&str, f64)> = numbers.iter().map(|(key, value)| (key.as_str(), *value)).collect();
            let row = drawing(&p.id, "T-97", &label, &flat, &[("class", panel_class(p))]);
            let mut homes = homes(PANEL, &HomeIds { row: &p.id, member: "", connection: "" }, &row);
            homes.extend(p.layers.iter().enumerate().map(|(k, layer)| Home { key: format!("t{}", k + 1), path: format!("panels[id={}].layers[id={}].thicknessM", p.id, layer.id), scale: MM, offset: 0.0 }));
            Some((row, homes))
        }
        "floors" => {
            let f = document.floors.get(index)?;
            let label = (format!("Timber floor {}", f.id), format!("Holzdecke {}", f.id));
            let numbers = [("span_m", f.span_m), ("width_m", f.width_m), ("ei_l", f.ei_l_n_m2_per_m), ("ei_b", f.ei_b_n_m2_per_m), ("mass_kg_m2", f.mass_kg_per_m2), ("zeta", f.damping_ratio)];
            let member = floor_member(document, f).map_or("", |m| m.id.as_str());
            let row = drawing(&f.id, "T-65", &label, &numbers, &[]);
            let homes = homes(FLOOR, &HomeIds { row: &f.id, member, connection: "" }, &row);
            Some((row, homes))
        }
        "diaphragms" => {
            let d = document.diaphragms.get(index)?;
            let label = (format!("Diaphragm {}", d.id), format!("Scheibe {}", d.id));
            let numbers = [("b_i", d.panel_width_m * MM), ("h", d.wall_height_m * MM), ("s", d.fastener_spacing_m * MM), ("n_panels", f64::from(d.panels)), ("l_dia", d.diaphragm_length_m * MM), ("f_v_ed", d.shear_ed_n), ("f_d", d.diaphragm_force_n), ("f_f_rd", d.fastener_capacity_n), ("q_v_rd", d.shear_resistance_n_per_m / MM)];
            let row = drawing(&d.id, "T-94", &label, &numbers, &[("kind", d.kind.id()), ("method", method_letter(d.method))]);
            let homes = homes(DIAPHRAGM, &HomeIds { row: &d.id, member: "", connection: "" }, &row);
            Some((row, homes))
        }
        "trusses" => {
            let t = document.trusses.get(index)?;
            let label = (format!("Truss {}", t.id), format!("Fachwerkträger {}", t.id));
            let numbers = [("n_braced", 1.0), ("n_d", t.chord_force_n), ("l_m", t.restraint_length_m), ("q_rd", t.bracing_capacity_n_per_m), ("span", t.span_m * MM), ("height", t.height_m * MM), ("bays", f64::from(t.bays)), ("chord_b", t.chord_width_m * MM), ("chord_h", t.chord_depth_m * MM), ("web_b", t.web_width_m * MM), ("web_h", t.web_depth_m * MM)];
            let row = drawing(&t.id, "T-95", &label, &numbers, &[("kind", t.kind.id()), ("product", t.product.id())]);
            let homes = homes(TRUSS, &HomeIds { row: &t.id, member: "", connection: "" }, &row);
            Some((row, homes))
        }
        "bracings" => {
            let b = document.bracings.get(index)?;
            let m = document.members.iter().find(|m| m.id == b.member_id)?;
            let label = (format!("Bracing {}", b.id), format!("Aussteifung {}", b.id));
            let numbers = [("n_braced", f64::from(b.stabilised_count)), ("n_d", b.compression_force_n), ("l_m", b.length_m), ("f_rd", b.resistance_n), ("k_br", b.stiffness_n_per_m / MM), ("spacing", b.spacing_m * MM), ("brace_b", b.brace_width_m * MM), ("brace_h", b.brace_depth_m * MM), ("member_b", m.b_m * MM), ("member_h", m.h_m * MM)];
            let row = drawing(&b.id, "T-96", &label, &numbers, &[("kind", b.kind.id())]);
            let homes = homes(BRACING, &HomeIds { row: &b.id, member: &m.id, connection: "" }, &row);
            Some((row, homes))
        }
        "decks" => {
            let d = document.decks.get(index)?;
            let label = (format!("Stress-laminated deck {}", d.id), format!("Vorgespanntes Brettstapeldeck {}", d.id));
            let mut numbers = deck_numbers(d);
            numbers.extend([("span_m", d.span_m), ("deck_width_m", d.deck_width_m), ("bar_d", d.bar_diameter_m * MM), ("plate_side", d.plate_side_m * MM), ("lam_b", d.lamination_width_m * MM)]);
            let row = drawing(&d.id, "T-122", &label, &numbers, &[("lamination", d.lamination_product.id()), ("bar", d.bar_product.id())]);
            let homes = homes(DECK, &HomeIds { row: &d.id, member: "", connection: "" }, &row);
            Some((row, homes))
        }
        _ => None,
    }
}
