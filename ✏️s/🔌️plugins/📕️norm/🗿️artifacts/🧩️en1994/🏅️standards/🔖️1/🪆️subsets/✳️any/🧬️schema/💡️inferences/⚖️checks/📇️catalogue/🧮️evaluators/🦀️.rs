//! 🧮️ Evaluators of the EN 1994 design-case catalogue: each entry calls the existing pure catalogue formulas (SI: m, Pa, N, N·m) and returns demand and capacity rows. See [`super`] and [EN 1994](https://eurocodes.jrc.ec.europa.eu/).

use crate::catalogue as formulas;
use formulas::{ISection, InterfaceKind};
use semio_s_plugin_norm_compliance::design_case::DesignCase;
use semio_s_plugin_norm_compliance::QuantityKind;

/// 📏️ One verification row: what is demanded, what is available and the worked steps.
pub struct Row {
    pub key: &'static str,
    pub en: &'static str,
    pub de: &'static str,
    pub kind: QuantityKind,
    pub demand: f64,
    pub capacity: f64,
    pub steps: String,
}

impl Row {
    pub fn new(key: &'static str, en: &'static str, de: &'static str, kind: QuantityKind, demand: f64, capacity: f64, steps: String) -> Self {
        Self { key, en, de, kind, demand, capacity, steps }
    }

    pub fn ratio(key: &'static str, en: &'static str, de: &'static str, utilisation: f64, steps: String) -> Self {
        Self::new(key, en, de, QuantityKind::Dimensionless, utilisation, 1.0, steps)
    }
}

/// 🧾 Rows of one evaluation, or the reason an input is missing.
pub type Outcome = Result<Vec<Row>, String>;

/// 🔢️ The typed numbers and texts of one design case.
pub struct Inputs<'a>(pub &'a DesignCase);

impl Inputs<'_> {
    pub fn n(&self, key: &str) -> Result<f64, String> {
        self.0.values.numbers.get(key).copied().ok_or_else(|| format!("missing input {key}"))
    }

    fn positive(&self, key: &str) -> Result<f64, String> {
        let value = self.n(key)?;
        if value.is_finite() && value > 0.0 {
            Ok(value)
        } else {
            Err(format!("input {key} must be positive"))
        }
    }

    pub fn t(&self, key: &str) -> Result<&str, String> {
        self.0.values.texts.get(key).map(String::as_str).ok_or_else(|| format!("missing input {key}"))
    }
}

/// 🔥 Table 4.6 for a partially encased column under standard fire exposure.
pub fn partially_encased_fire_table(q: &Inputs<'_>) -> Outcome {
    use formulas::column_fire::{COLUMN_TABLE, Tabulated, check_tabulated};

    if q.t("fire_col_restraint")? != "table" {
        return Err("Table 4.6 requires a braced frame with fixed ends in standard fire and normal-weight concrete".into());
    }
    let grade = q.t("fire_col_steel")?;
    if !matches!(grade, "S235" | "S275" | "S355") {
        return Err(format!("unknown strength class {grade}"));
    }
    let rating = match q.t("fire_col_rating")? {
        "R30" => 30,
        "R60" => 60,
        "R90" => 90,
        "R120" => 120,
        rating => return Err(format!("unknown model {rating}")),
    };
    let fck = q.n("fck")?;
    if !(20e6..=50e6).contains(&fck) {
        return Err("input fck must lie within 20000000 to 50000000".into());
    }
    let eta = q.n("fire_col_load_level")?;
    if !(0.0..=0.66).contains(&eta) {
        return Err("input fire_col_load_level must lie within 0 to 0.66".into());
    }
    let (width, depth, _) = column_dimensions(q)?;
    let size = width.min(depth);
    let axis = q.positive("fire_col_axis")?;
    let ambient_axis = q.positive("fire_col_ambient_axis")?;
    let reinforcement = q.n("fire_col_rebar_ratio")?;
    if !(0.01..=0.06).contains(&reinforcement) {
        return Err("input fire_col_rebar_ratio must lie within 0.01 to 0.06".into());
    }
    let web_flange = q.positive("fire_col_web_flange")?;
    let Tabulated::Evaluated { utilization, .. } = check_tabulated(COLUMN_TABLE, rating, eta, size, axis, reinforcement) else {
        return Err("Table 4.6 has no complete row for this fire class and load level".into());
    };
    Ok(vec![
        Row::ratio("table_minima", "Section and reinforcement against Table 4.6", "Querschnitt und Bewehrung nach Tabelle 4.6",
            utilization, format!("η_fi,t = E_fi,d / R_d(e, L_cr,20) = {eta:.3}. L_cr,20 = 2 L_cr,fi.")),
        Row::ratio("ambient_axis", "Ambient reinforcement axis distance", "Achsabstand der Bewehrung bei Normaltemperatur",
            ambient_axis / axis, format!("u_s = {:.1} mm ≥ {:.1} mm.", axis * 1000.0, ambient_axis * 1000.0)),
        Row::ratio("web_flange", "Minimum web-to-flange thickness ratio", "Mindestverhältnis von Steg- zu Flanschdicke",
            0.5 / web_flange, format!("e_w/e_f = {web_flange:.3} ≥ 0.50.")),
    ])
}

fn column_fire_rating(q: &Inputs<'_>, key: &str) -> Result<u32, String> {
    match q.t(key)? {
        "R30" => Ok(30),
        "R60" => Ok(60),
        "R90" => Ok(90),
        "R120" => Ok(120),
        "R180" => Ok(180),
        "R240" => Ok(240),
        rating => Err(format!("unknown model {rating}")),
    }
}

/// 🏙️ Table 4.7 for a concrete-filled rectangular or circular hollow section.
pub fn filled_column_fire_table(q: &Inputs<'_>) -> Outcome {
    use formulas::column_fire::{FILLED_COLUMN_TABLE, Tabulated, check_tabulated};

    if q.t("fire_fill_basis")? != "table" {
        return Err("Table 4.7 requires full standard fire exposure, a braced frame with fixed ends in fire and the specified ambient resistance basis".into());
    }
    let (width, depth, length) = column_dimensions(q)?;
    let thickness = q.positive("fire_fill_wall")?;
    let shape = q.t("fire_fill_shape")?;
    if !matches!(shape, "rectangular" | "circular") {
        return Err("Table 4.7 requires a rectangular or circular section".into());
    }
    if shape == "circular" && width != depth {
        return Err("For a circular section, enter the same diameter for both outer dimensions".into());
    }
    let wall_limit = width / 25.0;
    if thickness - wall_limit > f64::EPSILON * wall_limit {
        return Err("Table 4.7 requires an outer width or diameter of at least 25 times the wall thickness".into());
    }
    let eta = q.n("fire_col_load_level")?;
    if !(0.0..=0.66).contains(&eta) {
        return Err("input fire_col_load_level must lie within 0 to 0.66".into());
    }
    let ratio = q.n("fire_fill_ratio")?;
    let axis = q.n("fire_fill_axis")?;
    let ambient_axis = q.n("fire_col_ambient_axis")?;
    if !(0.0..1.0).contains(&ratio) || !axis.is_finite() || !ambient_axis.is_finite() || axis < 0.0 || ambient_axis < 0.0 {
        return Err("Table 4.7 requires nonnegative axis distances and a reinforcement ratio below 1".into());
    }
    if ratio > 0.0 && (q.t("fire_fill_rebar")? != "S500" || axis == 0.0 || ambient_axis == 0.0) {
        return Err("Reinforced sections in Table 4.7 require S500 and positive actual and ambient-minimum bar axis distances".into());
    }
    let rating = column_fire_rating(q, "fire_col_ext_rating")?;
    let Tabulated::Evaluated { utilization, .. } = check_tabulated(FILLED_COLUMN_TABLE, rating, eta, width.min(depth), axis, ratio) else {
        return Err("Table 4.7 has no row for this fire class and load level".into());
    };
    let mut rows = vec![Row::ratio("table_minima", "Section and reinforcement against Table 4.7", "Querschnitt und Bewehrung nach Tabelle 4.7",
        utilization, format!("η_fi,t = {eta:.3}, L = {length:.2} m. The supplied ambient resistance uses f_y = 235 MPa and at most 3% reinforcement."))];
    if ratio > 0.0 {
        rows.push(Row::ratio("ambient_axis", "Ambient reinforcement axis distance", "Achsabstand der Bewehrung bei Normaltemperatur",
            ambient_axis / axis, format!("u_s = {:.1} mm ≥ {:.1} mm.", axis * 1000.0, ambient_axis * 1000.0)));
    }
    Ok(rows)
}

/// 🧱 Table 4.4 for a fully encased composite column.
pub fn fully_encased_fire_table(q: &Inputs<'_>) -> Outcome {
    use formulas::column_fire::check_fully_encased;

    if q.t("fire_enc_basis")? != "table" {
        return Err("Table 4.4 requires a braced, fully encased column in standard fire with the applicable ambient resistance and detailing verified".into());
    }
    let (width, depth, _) = column_dimensions(q)?;
    let rating = column_fire_rating(q, "fire_col_ext_rating")?;
    let cover = q.positive("fire_enc_steel_cover")?;
    let axis = q.positive("fire_col_axis")?;
    let ambient_axis = q.positive("fire_col_ambient_axis")?;
    let bars = q.positive("fire_enc_bar_count")?;
    let diameter = q.positive("fire_enc_bar_diameter")?;
    if bars.fract() != 0.0 {
        return Err("input fire_enc_bar_count must be an integer".into());
    }
    let Some((row, utilization)) = check_fully_encased(rating, width.min(depth), cover, axis) else {
        return Err("Table 4.4 has no row for this fire class".into());
    };
    Ok(vec![
        Row::ratio("table_minima", "Section, cover and reinforcement against Table 4.4", "Querschnitt, Betondeckung und Bewehrung nach Tabelle 4.4",
            utilization, format!("Alternative {}, min(b, h) ≥ {:.0} mm, c ≥ {:.0} mm, u_s ≥ {:.0} mm.",
                row.alternative, row.min_dimension_m * 1000.0, row.min_cover_m * 1000.0, row.min_axis_m * 1000.0)),
        Row::ratio("ambient_axis", "Ambient reinforcement axis distance", "Achsabstand der Bewehrung bei Normaltemperatur",
            ambient_axis / axis, format!("u_s = {:.1} mm ≥ {:.1} mm.", axis * 1000.0, ambient_axis * 1000.0)),
        Row::ratio("longitudinal_bars", "At least four longitudinal bars of at least 12 mm", "Mindestens vier Längsstäbe von mindestens 12 mm",
            (4.0 / bars).max(0.012 / diameter), format!("n = {bars:.0}, ø = {:.0} mm.", diameter * 1000.0)),
    ])
}

fn column_dimensions(q: &Inputs<'_>) -> Result<(f64, f64, f64), String> {
    let width = q.positive("fire_col_width")?;
    let depth = q.positive("fire_col_depth")?;
    let length = q.positive("l_col")?;
    if length > 30.0 * width.min(depth) {
        return Err("The column length must be at most 30 times its smallest outer section dimension".into());
    }
    Ok((width, depth, length))
}

/// 🔥 Concrete used solely as insulation in a composite beam, EN 1994-1-2 Table 4.3.
pub fn insulating_beam_fire_table(q: &Inputs<'_>) -> Outcome {
    if q.t("fire_ins_beam_basis")? != "verified" {
        return Err("Confirm that the filled web compartments and beam detailing satisfy the applicable standard fire conditions of Table 4.3".into());
    }
    insulating_encasement_fire_table(q)
}

/// 🏛️ Concrete used solely as insulation in a composite column, EN 1994-1-2 Table 4.5.
pub fn insulating_column_fire_table(q: &Inputs<'_>) -> Outcome {
    if q.t("fire_ins_column_basis")? != "verified" {
        return Err("Confirm that the filled web compartments and column detailing satisfy the applicable standard fire conditions of Table 4.5".into());
    }
    column_dimensions(q)?;
    insulating_encasement_fire_table(q)
}

/// 🧯 Cover and retaining mesh for concrete with a purely insulating function.
fn insulating_encasement_fire_table(q: &Inputs<'_>) -> Outcome {
    use formulas::column_fire::insulating_cover_m;
    let rating = column_fire_rating(q, "fire_col_ext_rating")?;
    let Some(minimum) = insulating_cover_m(rating) else {
        return Err("The insulating encasement tables cover the R30 to R180 classes".into());
    };
    let actual = q.n("fire_ins_cover")?;
    if !actual.is_finite() || actual < 0.0 {
        return Err("Concrete cover must be a finite nonnegative length".into());
    }
    if minimum == 0.0 {
        return Ok(vec![Row::ratio("insulating_cover", "Concrete fills the web compartments", "Kammern am Steg ausbetoniert",
            1.0, "R30 allows filled web compartments without additional cover to the steel profile.".into())]);
    }
    let mut rows = vec![Row::new("insulating_cover", "Concrete cover to the steel profile", "Betondeckung des Stahlprofils",
        QuantityKind::Length, minimum, actual, format!("c ≥ {:.0} mm.", minimum * 1000.0))];
    if q.t("fire_ins_mesh")? != "yes" {
        rows.push(Row::ratio("insulating_mesh", "Mesh surrounding the section", "Betonstahlmatte um den Querschnitt", 2.0,
            "R60 to R180 require mesh surrounding the section in both directions.".into()));
        return Ok(rows);
    }
    let spacing = q.positive("fire_ins_spacing")?;
    let diameter = q.positive("fire_ins_diameter")?;
    let min_cover = q.positive("fire_ins_mesh_cover_min")?;
    let max_cover = q.positive("fire_ins_mesh_cover_max")?;
    if min_cover > max_cover {
        return Err("The smallest mesh cover must not exceed the largest mesh cover".into());
    }
    rows.extend([
        Row::new("mesh_spacing", "Maximum mesh bar spacing in either direction", "Größter Mattenstab-Abstand in beiden Richtungen",
            QuantityKind::Length, spacing, 0.250, "s ≤ 250 mm, §5.1(6).".into()),
        Row::new("mesh_diameter", "Minimum mesh bar diameter in either direction", "Kleinster Mattenstab-Durchmesser in beiden Richtungen",
            QuantityKind::Length, 0.004, diameter, "ø ≥ 4 mm, §5.1(6).".into()),
        Row::new("mesh_cover_min", "Minimum reinforcement cover on fire-exposed surfaces", "Mindestbetondeckung der Bewehrung auf beflammten Oberflächen",
            QuantityKind::Length, 0.020, min_cover, "c ≥ 20 mm, §5.1(5).".into()),
        Row::new("mesh_cover_max", "Maximum reinforcement cover on fire-exposed surfaces", "Höchstbetondeckung der Bewehrung auf beflammten Oberflächen",
            QuantityKind::Length, max_cover, 0.050, "c ≤ 50 mm, §5.1(5).".into()),
    ]);
    Ok(rows)
}

/// 🌉 Tables 4.1 and 4.2 for a simply supported partially encased composite beam.
pub fn partially_encased_beam_fire_table(q: &Inputs<'_>) -> Outcome {
    use formulas::beam_fire_tables::{BEAM_FIRE_ALTERNATIVES, beam_load_level, beam_reinforcement_axes};

    if q.t("fire_beam_basis")? != "verified" {
        return Err("Confirm the simply supported member, composite slab connection, standard fire, and ambient resistance basis of Table 4.1".into());
    }
    let coverage = q.n("fire_beam_deck_coverage")?;
    match q.t("fire_beam_deck")? {
        "solid" => {}
        "profiled" if coverage.is_finite() && (0.85..=1.0).contains(&coverage) => {}
        "closed_voids" => {}
        _ => return Err("Profiled sheeting requires at least 85% top flange coverage or duly fire-sealed voids".into()),
    }
    let width = q.positive("fire_beam_width")?;
    let depth = q.positive("fire_beam_depth")?;
    let web = q.positive("fire_beam_web")?;
    let flange = q.positive("fire_beam_flange")?;
    let slab = q.positive("fire_beam_slab")?;
    let effective_width = q.positive("fire_beam_effective_width")?;
    let steel_area = q.n("fire_beam_as")?;
    let concrete_area = q.positive("fire_beam_ac")?;
    if !steel_area.is_finite() || steel_area < 0.0 {
        return Err("The area of additional reinforcement must be finite and nonnegative".into());
    }
    let encased_ratio = steel_area / (concrete_area + steel_area);
    if width / web + 1e-12 < 15.0 || flange / web > 2.0 + 1e-12 || slab < 0.12 || effective_width > 5.0
        || !(0.0..=0.05).contains(&encased_ratio) {
        return Err("Table 4.1 requires b/e_w ≥ 15, e_f/e_w ≤ 2, h_c ≥ 120 mm, b_eff ≤ 5 m and A_s/(A_c + A_s) ≤ 5%".into());
    }
    let rating = column_fire_rating(q, "fire_col_ext_rating")?;
    let Some(rating_idx) = [30, 60, 90, 120, 180].into_iter().position(|class| class == rating) else {
        return Err("Table 4.1 covers R30 to R180".into());
    };
    let eta = q.n("fire_beam_load_level")?;
    let level = beam_load_level(eta).ok_or("Table 4.1 requires a fire load level between 0 and 0.70")?;
    let supplied = steel_area / (width * flange);
    let fy_ratio = q.positive("fire_beam_fy")? / 355e6;
    let axes = beam_reinforcement_axes(rating, width);
    let selected = BEAM_FIRE_ALTERNATIVES.iter().filter(|row| row.load_level == level)
        .filter_map(|row| {
            let (min_width_mm, ratio) = row.rating_cells[rating_idx]?;
            let min_width = f64::from(min_width_mm) / 1000.0;
            let min_depth = min_width * row.depth_factor;
            let min_ratio = ratio * fy_ratio;
            let steel_ratio = if min_ratio == 0.0 { 0.0 } else if supplied > 0.0 { min_ratio / supplied } else { 1e6 };
            Some((min_width, min_depth, min_ratio, (min_width / width).max(min_depth / depth).max(steel_ratio)))
        })
        .min_by(|left, right| left.3.total_cmp(&right.3));
    let Some((min_width, min_depth, min_ratio, utilization)) = selected else {
        return Err("Table 4.1 has no complete alternative for this fire class and load level".into());
    };
    let mut rows = vec![Row::ratio("beam_minima", "Beam section and reinforcement against Table 4.1", "Trägerquerschnitt und Bewehrung nach Tabelle 4.1",
        utilization, format!("η_fi,t = {eta:.3} ≤ {level:.2} · b ≥ {:.1} mm · h ≥ {:.1} mm · A_s/A_f ≥ {min_ratio:.3}.",
            min_width * 1000.0, min_depth * 1000.0))];
    if min_ratio == 0.0 || supplied == 0.0 {
        return Ok(rows);
    }
    if q.t("fire_beam_rebar")? != "S500" {
        return Err("Table 4.1 requires S500 for credited additional longitudinal bars".into());
    }
    let Some((table_u1, table_u2)) = axes else {
        return Err("Table 4.2 has no reinforcement axis distances for this fire class and profile width".into());
    };
    let u1 = q.positive("fire_beam_u1")?;
    let u2 = q.positive("fire_beam_u2")?;
    let u1_min = table_u1.max(q.positive("fire_beam_ambient_u1")?);
    let u2_min = table_u2.max(q.positive("fire_beam_ambient_u2")?);
    rows.push(Row::ratio("beam_bar_axes", "Reinforcement axis distances against Table 4.2 and ambient detailing",
        "Bewehrungsachsabstände nach Tabelle 4.2 und der Bemessung bei Normaltemperatur",
        (u1_min / u1).max(u2_min / u2), format!("u₁ ≥ {:.1} mm · u₂ ≥ {:.1} mm.", u1_min * 1000.0, u2_min * 1000.0)));
    Ok(rows)
}

/// 🔩 EN 1994-1-1 §6.6.4.2: stud design resistance P_Rd reduced by the sheeting factor k_t or k_l against the force per stud.
pub fn stud_sheeting(q: &Inputs<'_>) -> Outcome {
    let (d, h_sc, fck) = (q.n("d_stud")?, q.n("h_sc")?, q.n("fck")?);
    let (_, _, p_rd) = formulas::stud_resistance_n(d, h_sc, q.n("fu_stud")?, fck, formulas::ecm_pa(fck), q.n("gamma_v")?);
    let parallel = q.t("ribs")? == "parallel";
    let k = formulas::sheeting_reduction(q.n("h_p")?, q.n("b_0")?, h_sc, d, q.n("t_sheet")?, q.n("n_r")?.round() as u32, parallel).ok_or("sheeting reduction applies up to two studs per rib")?;
    let p_ed = q.n("p_ed")?;
    Ok(vec![Row::new("stud", "PEd ≤ k PRd", "PEd ≤ k PRd", QuantityKind::Force, p_ed, k * p_rd, format!("PRd = {p_rd:.0} N; k = {k:.3}; k PRd = {:.0} N", k * p_rd))])
}

/// 🧷 EN 1993-1-5 §6 and §7.2 as used by EN 1994-1-1: web patch loading resistance and the interaction 0.8 FEd/FRd + MEd/MRd ≤ 1.4.
pub fn patch_loading(q: &Inputs<'_>) -> Outcome {
    let (f_rd, chi, l_y, lambda) = formulas::patch_loading(q.n("h_w")?, q.n("t_w")?, q.n("b_f")?, q.n("t_f")?, q.n("fyw")?, q.n("fyf")?, q.n("s_s")?, q.n("a_panel")?, q.n("gamma_m1")?);
    let f_ed = q.n("f_ed")?;
    let u = formulas::patch_interaction(f_ed, f_rd, q.n("m_ed")?, q.n("m_rd")?);
    Ok(vec![
        Row::new("patch", "FEd ≤ FRd", "FEd ≤ FRd", QuantityKind::Force, f_ed, f_rd, format!("FRd = {f_rd:.0} N; χF = {chi:.3}; ℓy = {l_y:.3} m; λF = {lambda:.3}")),
        Row::ratio("interaction", "0.8 FEd/FRd + MEd/MRd ≤ 1.4", "0,8 FEd/FRd + MEd/MRd ≤ 1,4", u, format!("utilisation against 1.4: {u:.3}")),
    ])
}

fn interface_kind(value: &str) -> Result<InterfaceKind, String> {
    match value {
        "concrete_encased" => Ok(InterfaceKind::ConcreteEncased),
        "filled_circular" => Ok(InterfaceKind::FilledCircular),
        "filled_rectangular" => Ok(InterfaceKind::FilledRectangular),
        "partially_encased_flange" => Ok(InterfaceKind::PartiallyEncasedFlange),
        "partially_encased_web" => Ok(InterfaceKind::PartiallyEncasedWeb),
        other => Err(format!("unknown interface {other}")),
    }
}

/// 🔗 EN 1994-1-1 §6.7.4: interface shear τEd = Nintro/(u lv) over the introduction length lv = min(2 d, L/3) against τRd (Table 6.6).
pub fn load_introduction(q: &Inputs<'_>) -> Outcome {
    let kind = interface_kind(q.t("kind")?)?;
    let length = formulas::introduction_length_m(q.n("d_min")?, q.n("l_col")?);
    let n = q.n("n_intro")?;
    let (tau_ed, tau_rd, u) = formulas::load_introduction(kind, n, q.n("perimeter")?, length);
    let studs = formulas::introduction_studs(n, q.n("p_rd")?);
    Ok(vec![Row::ratio("interface", "τEd ≤ τRd", "τEd ≤ τRd", u, format!("lv = {length:.3} m; τEd = {tau_ed:.0} Pa; τRd = {tau_rd:.0} Pa; studs for the full force: {studs}"))])
}

fn deflection_row(q: &Inputs<'_>, delta: f64) -> Outcome {
    let limit = q.n("l_span")? / q.n("span_ratio")?;
    Ok(vec![Row::new("deflection", "δ ≤ L/limit", "δ ≤ L/Grenzwert", QuantityKind::Length, delta, limit, format!("δ = {:.2} mm; L/limit = {:.2} mm", delta * 1e3, limit * 1e3))])
}

/// 📉 EN 1994-1-1 §7.3.1: mid-span deflection of a simply supported beam with long-term and short-term stiffness against L/limit.
pub fn deflection_simple_span(q: &Inputs<'_>) -> Outcome {
    deflection_row(q, formulas::deflection_simple(q.n("l_span")?, q.n("g_line")?, q.n("q_line")?, q.n("i_long")?, q.n("i_short")?))
}

/// 📊️ EN 1994-1-1 §7.3.1: maximum deflection of a two-span continuous beam against L/limit.
pub fn deflection_two_span(q: &Inputs<'_>) -> Outcome {
    deflection_row(q, formulas::deflection_two_span(q.n("l_span")?, q.n("g_line")?, q.n("q_line")?, q.n("i_long")?, q.n("i_short")?))
}

/// 🔥 EN 1994-1-2 §3.2: steel stress σEd against the reduced yield strength ky,θ fy at the steel temperature θa.
pub fn steel_at_temperature(q: &Inputs<'_>) -> Outcome {
    let k = formulas::ky_theta(q.n("theta_a")?);
    let f = k * q.n("fy")?;
    let sigma = q.n("sigma_ed")?;
    Ok(vec![Row::new("strength", "σEd ≤ ky,θ fy", "σEd ≤ ky,θ fy", QuantityKind::Stress, sigma, f, format!("ky,θ = {k:.3}; ky,θ fy = {f:.0} Pa"))])
}

fn section(q: &Inputs<'_>) -> Result<ISection, String> {
    Ok(ISection { h: q.n("sec_h")?, b: q.n("sec_b")?, tf: q.n("sec_tf")?, tw: q.n("sec_tw")?, a: q.n("sec_a")?, i_y: q.n("sec_iy")? })
}

/// 📐 EN 1994-1-1 §6.2.1.2: sagging plastic moment resistance Mpl,Rd by the stress block of the concrete slab (0.85 fck/γC) and the steel section.
pub fn sagging_plastic(q: &Inputs<'_>) -> Outcome {
    let fcc = formulas::fcc_beam_pa(q.n("fck")?, q.n("gamma_c")?);
    let p = formulas::plastic_full(&section(q)?, q.n("fyd")?, q.n("ys")?, q.n("b_eff")?, q.n("h_c")?, fcc);
    let m_ed = q.n("m_ed")?.abs();
    Ok(vec![Row::new("moment", "MEd ≤ Mpl,Rd", "MEd ≤ Mpl,Rd", QuantityKind::Moment, m_ed, p.m_nm, format!("Mpl,Rd = {:.0} N·m; Nc = {:.0} N; x = {:.4} m; plastic axis {:.4} m ({})", p.m_nm, p.nc_n, p.x_m, p.y_na_m, p.zone))])
}

/// 🏔️ EN 1994-1-1 §6.2.1.2(2): hogging plastic moment resistance with the slab reinforcement and the steel section, concrete neglected.
pub fn hogging_plastic(q: &Inputs<'_>) -> Outcome {
    let (m_rd, y) = formulas::plastic_hogging(&section(q)?, q.n("fyd")?, q.n("ys")?, q.n("as_rebar")?, q.n("fsd")?, q.n("y_r")?);
    let m_ed = q.n("m_ed")?.abs();
    Ok(vec![Row::new("moment", "|MEd| ≤ Mpl,Rd", "|MEd| ≤ Mpl,Rd", QuantityKind::Moment, m_ed, m_rd.abs(), format!("Mpl,Rd = {:.0} N·m; plastic axis {y:.4} m below the slab top", m_rd.abs()))])
}

/// 🌊️ EN 1990 A1.4.4 and EN 1994-1-1 §7.3: lowest natural frequency f1 = π/(2 L²)·√(Ea I(n0)/m) of the simply supported composite beam against the required fmin.
pub fn floor_vibration(q: &Inputs<'_>) -> Outcome {
    let (span, i_short) = (q.n("l_span")?, q.n("i_short")?);
    let mass = formulas::vibration::mass_per_length_kg_m(q.n("g_line")?, q.n("q_line")?, q.n("psi_vib")?);
    if mass <= 0.0 {
        return Err("the vibrating mass is zero".into());
    }
    let f1 = formulas::vibration::natural_frequency_hz(span, i_short, mass);
    let f_min = q.n("f_min")?;
    let u = formulas::vibration::frequency_utilisation(f_min, f1);
    Ok(vec![Row::ratio("frequency", "fmin ≤ f1", "fmin ≤ f1", u, format!("m = {mass:.1} kg/m; f1 = {f1:.3} Hz; fmin = {f_min:.3} Hz"))])
}

fn welded(q: &Inputs<'_>) -> Result<bool, String> {
    match q.t("steel_kind")? {
        "rolled" => Ok(false),
        "welded" => Ok(true),
        other => Err(format!("unknown steel kind {other}")),
    }
}

/// 🦋️ EN 1994-1-1 §6.4: lateral-torsional buckling of the steel bottom flange of a continuous composite beam over the support, inverted U-frame stiffness ks (6.4.2) and χLT (EN 1993-1-1 6.3.2.2) against the hogging moment.
pub fn ltb_continuous(q: &Inputs<'_>) -> Outcome {
    let sec = section(q)?;
    let (fy, gamma) = (q.n("fy")?, q.n("gamma_m1")?);
    let length = q.n("l_ltb")?;
    let ks = formulas::uframe_ks(q.n("i2_slab")?, q.n("a_beam")?, &sec, q.n("k_alpha")?);
    let m_cr = formulas::mcr_uframe(&sec, length, ks);
    let m_pl_k = sec.w_pl() * fy;
    let (lambda, chi) = formulas::chi_lt(m_pl_k, m_cr, sec.h / sec.b, welded(q)?);
    let m_b_rd = chi * m_pl_k / gamma;
    let m_ed = q.n("m_ed")?.abs();
    Ok(vec![Row::new("ltb", "|MEd| ≤ Mb,Rd", "|MEd| ≤ Mb,Rd", QuantityKind::Moment, m_ed, m_b_rd, format!("ks = {ks:.0} N·m/m/rad; Mcr = {m_cr:.0} N·m; λLT = {lambda:.3}; χLT = {chi:.3}; Mb,Rd = {m_b_rd:.0} N·m"))])
}

/// ⏳️ EN 1994-1-1 §5.4.2.2 and §7.3.1: deflection of a simply supported composite beam with the modular ratios n0 (short term), nL (permanent loads, ψL = 1.1) and ns (shrinkage, ψL = 0.55) plus the shrinkage curvature against L/limit.
pub fn deflection_creep_shrinkage(q: &Inputs<'_>) -> Outcome {
    let sec = section(q)?;
    let (ys, beff, hc, span) = (q.n("ys")?, q.n("b_eff")?, q.n("h_c")?, q.n("l_span")?);
    let ecm = formulas::ecm_pa(q.n("fck")?);
    let phi = q.n("phi_t")?;
    let n0 = formulas::modular_ratio(ecm, 0.0, 0.0);
    let n_l = formulas::modular_ratio(ecm, q.n("psi_l")?, phi);
    let n_s = formulas::modular_ratio(ecm, formulas::constants::PSI_L_SHRINKAGE, phi);
    let (i0, i_l, el_s) = (formulas::transformed(&sec, ys, beff, hc, n0).i, formulas::transformed(&sec, ys, beff, hc, n_l).i, formulas::transformed(&sec, ys, beff, hc, n_s));
    let load = formulas::deflection_simple(span, q.n("g_line")?, q.n("q_line")?, i_l, i0);
    let shrink = formulas::deflection_shrinkage(span, q.n("eps_cs")?, beff, hc, n_s, (el_s.y_na - hc / 2.0).max(0.0), el_s.i);
    let limit = span / q.n("span_ratio")?;
    Ok(vec![Row::new("deflection", "δ(G, Q) + δ(εcs) ≤ L/limit", "δ(G, Q) + δ(εcs) ≤ L/Grenzwert", QuantityKind::Length, load + shrink, limit, format!("n0 = {n0:.2}; nL = {n_l:.2}; ns = {n_s:.2}; I(n0) = {i0:.6} m⁴; I(nL) = {i_l:.6} m⁴; δ(G,Q) = {:.2} mm; δ(εcs) = {:.2} mm; L/limit = {:.2} mm", load * 1e3, shrink * 1e3, limit * 1e3))])
}

/// 🔥 EN 1993-1-2 §4.2.5.2 (Eq. 4.27) and §4.2.4 (Eq. 4.22): temperature θa(t) of the protected steel section against the critical temperature θa,cr of the degree of utilisation μ0.
pub fn protected_steel_fire(q: &Inputs<'_>) -> Outcome {
    let t = q.n("t_fire")?;
    let theta = formulas::protected_steel_temperature_c(t, q.n("am_v")?, q.n("d_p")?, q.n("lambda_p")?, q.n("c_p")?, q.n("rho_p")?);
    let mu0 = q.n("mu0")?;
    let theta_cr = formulas::critical_temperature_c(mu0);
    Ok(vec![Row::new("temperature", "θa(t) ≤ θa,cr", "θa(t) ≤ θa,cr", QuantityKind::Temperature, theta, theta_cr, format!("θa({t:.0} s) = {theta:.1} °C; θa,cr(μ0 = {mu0:.3}) = {theta_cr:.1} °C; ISO 834 gas {:.1} °C", formulas::iso834_c(t)))])
}
