//! 🧮️ Evaluators of the EN 1998 design-case catalogue: each entry calls the existing pure kernel functions (SI: kg, m, s, N, Pa) and returns demand and capacity rows. See [`super`] and [EN 1998](https://eurocodes.jrc.ec.europa.eu/).

use crate::horizontal::diaphragm;
use crate::{part_1, part_5};
use semio_s_plugin_norm_compliance::design_case::DesignCase;
use semio_s_plugin_norm_compliance::{CheckStatus, QuantityKind};

/// 📏️ One verification row: what is demanded, what is available and the worked steps.
pub struct Row {
    pub key: &'static str,
    pub en: &'static str,
    pub de: &'static str,
    pub kind: QuantityKind,
    pub demand: f64,
    pub capacity: f64,
    pub steps: String,
    pub status: Option<CheckStatus>,
}

impl Row {
    pub fn new(key: &'static str, en: &'static str, de: &'static str, kind: QuantityKind, demand: f64, capacity: f64, steps: String) -> Self {
        Self { key, en, de, kind, demand, capacity, steps, status: None }
    }
    pub fn status(mut self, status: CheckStatus) -> Self {
        self.status = Some(status);
        self
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

    pub fn t(&self, key: &str) -> Result<&str, String> {
        self.0.values.texts.get(key).map(String::as_str).ok_or_else(|| format!("missing input {key}"))
    }
}

fn base_shear_row(key: &'static str, en: &'static str, de: &'static str, force: f64, q: &Inputs<'_>) -> Outcome {
    let v_rd = q.n("v_rd")?;
    Ok(vec![Row::new(key, en, de, QuantityKind::Force, force, v_rd, format!("Fb = {force:.0} N; VRd = {v_rd:.0} N"))])
}

/// 📉 EN 1998-1 §3.2.2.5 and §4.3.3.2.2: base shear Fb = Sd(T1) m λ against the base shear resistance.
pub fn base_shear_design(q: &Inputs<'_>) -> Outcome {
    let sd = part_1::design_spectrum_horizontal(q.n("a_g")?, q.n("s_soil")?, q.n("t_b")?, q.n("t_c")?, q.n("t_d")?, q.n("q_factor")?, q.n("t_1")?);
    base_shear_row("base-shear", "Fb = Sd(T1) m λ ≤ VRd", "Fb = Sd(T1) m λ ≤ VRd", sd * q.n("mass")? * q.n("lambda_c")?, q)
}

/// 📈 EN 1998-1 §3.2.2.2: elastic response force Se(T1, η) m against the base shear resistance.
pub fn elastic_force(q: &Inputs<'_>) -> Outcome {
    let eta = part_1::damping_correction(q.n("xi_percent")?);
    let se = part_1::elastic_response_spectrum_eta(q.n("a_g")?, q.n("s_soil")?, q.n("t_b")?, q.n("t_c")?, q.n("t_d")?, q.n("t_1")?, eta);
    base_shear_row("elastic", "Fel = Se(T1, η) m ≤ VRd", "Fel = Se(T1, η) m ≤ VRd", se * q.n("mass")?, q)
}

/// ↕️ EN 1998-1 §3.2.2.3: vertical seismic force Fv = Sve(Tv) m against the vertical resistance.
pub fn vertical_force(q: &Inputs<'_>) -> Outcome {
    let sve = part_1::vertical_elastic_spectrum(q.n("a_g")?, q.n("avg_ratio")?, q.n("t_v")?);
    let force = sve * q.n("mass")?;
    let f_rd = q.n("f_rd_v")?;
    Ok(vec![Row::new("vertical", "Fv = Sve(Tv) m ≤ FRd,v", "Fv = Sve(Tv) m ≤ FRd,v", QuantityKind::Force, force, f_rd, format!("Sve = {sve:.3} m/s²; Fv = {force:.0} N; FRd,v = {f_rd:.0} N"))])
}

/// 🎼 EN 1998-1 §4.3.3.1(3) and §4.3.3.3: effective modal masses of a three-storey shear building (at least 90 % of the mass) and the base shear by SRSS, or by CQC when two periods are closer than 10 %.
pub fn modal_analysis(q: &Inputs<'_>) -> Outcome {
    let masses = [q.n("m_1")?, q.n("m_2")?, q.n("m_3")?];
    let stiffness = [q.n("k_1")?, q.n("k_2")?, q.n("k_3")?];
    let (periods, shapes) = part_1::shear_building_modes(&masses, &stiffness);
    let effective = part_1::effective_modal_masses(&masses, &shapes);
    let (total, covered) = (masses.iter().sum::<f64>(), effective.iter().sum::<f64>());
    let (a_g, s_soil, t_b, t_c, t_d, q_factor) = (q.n("a_g")?, q.n("s_soil")?, q.n("t_b")?, q.n("t_c")?, q.n("t_d")?, q.n("q_factor")?);
    let modal: Vec<f64> = periods.iter().zip(&effective).map(|(t, m)| m * part_1::design_spectrum_horizontal(a_g, s_soil, t_b, t_c, t_d, q_factor, *t)).collect();
    let separated = periods.iter().enumerate().all(|(i, a)| periods.iter().skip(i + 1).all(|b| (*a).min(*b) <= 0.9 * (*a).max(*b)));
    let (method, v_b) = if separated { ("SRSS", part_1::srss_combine(&modal)) } else { ("CQC", part_1::cqc_combine(&modal, &periods, q.n("xi_percent")? / 100.0)) };
    let v_rd = q.n("v_rd")?;
    Ok(vec![
        Row::new("modal-mass", "Σ Meff,n ≥ 0.9 m", "Σ Meff,n ≥ 0,9 m", QuantityKind::Mass, 0.9 * total, covered, format!("T = {periods:.3?} s; Meff = {effective:.0?} kg; Σ Meff = {covered:.0} kg of {total:.0} kg")),
        Row::new("modal-shear", "Vb ≤ VRd (modal combination)", "Vb ≤ VRd (modale Überlagerung)", QuantityKind::Force, v_b, v_rd, format!("{method} combination of {modal:.0?} N gives Vb = {v_b:.0} N; VRd = {v_rd:.0} N")),
    ])
}

/// 🌍️ EN 1998-1 §3.1.2 Eq. 3.1: vs,30 = 30/Σ(hi/vi) of three layers against the lower bound of the assumed ground type.
pub fn ground_velocity(q: &Inputs<'_>) -> Outcome {
    let layers = [(q.n("h_1")?, q.n("vs_1")?), (q.n("h_2")?, q.n("vs_2")?), (q.n("h_3")?, q.n("vs_3")?)];
    let vs30 = part_1::vs30_from_layers(&layers);
    let class = part_1::ground_class_from_vs30(vs30).map_or("-".to_string(), |c| c.to_string());
    let v_min = q.n("vs_min")?;
    Ok(vec![Row::new("vs30", "vs,30 ≥ lower bound", "vs,30 ≥ Untergrenze", QuantityKind::Dimensionless, v_min, vs30, format!("vs,30 = {vs30:.1} m/s (ground type {class}); lower bound {v_min:.0} m/s"))])
}

/// 🌊️ EN 1998-5 §4.1.4: factor of safety against liquefaction FL = CRR7.5 MSF/CSR against the required value.
pub fn liquefaction(q: &Inputs<'_>) -> Outcome {
    let f_l = part_5::liquefaction_safety_factor(q.n("n1_60")?, q.n("fines_percent")?, q.n("a_max")?, q.n("sigma_v")?, q.n("sigma_v_eff")?, q.n("z_depth")?, q.n("magnitude")?);
    let required = q.n("f_l_min")?;
    Ok(vec![Row::new("liquefaction", "FL ≥ FL,min", "FL ≥ FL,min", QuantityKind::Dimensionless, required, f_l, format!("FL = {f_l:.3}; required {required:.2}"))])
}

/// 🏗️ EN 1998-5 §5.4.2: bending moment at depth z of a long pile under the inertial head shear against the moment resistance.
pub fn pile_inertial(q: &Inputs<'_>) -> Outcome {
    let fixed = q.t("head_fixity")? == "fixed";
    let shear = q.n("head_shear")?;
    let response = part_5::pile_inertial(q.n("e_pile")?, q.n("d_pile")?, q.n("l_pile")?, q.n("k_h")?, shear, fixed);
    let moment = part_5::pile_inertial_moment_at(&response, shear, fixed, q.n("z_check")?).abs();
    let m_rd = q.n("m_rd")?;
    Ok(vec![Row::new("pile-moment", "|M(z)| ≤ MRd", "|M(z)| ≤ MRd", QuantityKind::Moment, moment, m_rd, format!("β = {:.3} 1/m; |M(z)| = {moment:.0} N·m; maximum {:.0} N·m; MRd = {m_rd:.0} N·m", response.beta, response.max_moment_nm))])
}

/// 🔗 EN 1998-1 §4.4.2.5: chord force N(x) = Fd x (L − x)/(2 L D) of a diaphragm under the storey force against the chord resistance A fd.
pub fn diaphragm_chord(q: &Inputs<'_>) -> Outcome {
    let f_d = diaphragm::design_force_n(q.n("f_storey")?, q.n("gamma_d")?);
    let n_ed = diaphragm::chord_force_at_n(f_d, q.n("span_dia")?, q.n("depth_dia")?, q.n("x_pos")?);
    let n_rd = diaphragm::chord_resistance_n(q.n("chord_area")?, q.n("chord_fd")?);
    Ok(vec![Row::new("chord", "NEd(x) ≤ NRd", "NEd(x) ≤ NRd", QuantityKind::Force, n_ed, n_rd, format!("Fd = {f_d:.0} N; NEd(x) = {n_ed:.0} N; NRd = {n_rd:.0} N"))])
}

/// 🗼 EN 1998-6 Annex B: component-energy modal damping and its elastic horizontal spectrum.
pub fn tower_modal_damping(q: &Inputs<'_>) -> Outcome {
    use crate::part_6::{energy_weighted_modal_damping, MODAL_DAMPING_RATIOS};

    let source = q.t("tw_damping_source")?;
    let experimental = source == "experimental";
    let damping = if experimental {
        let ratio = q.n("xi_percent")? / 100.0;
        if !(0.0..=1.0).contains(&ratio) { return Err("Experimental damping must be between 0 and 100 percent".into()); }
        ratio
    } else if source == "energy" {
        let damage_limit = match q.t("tw_action")? {
            "damage" => true,
            "design" => false,
            _ => return Err("Select the damage limitation or design seismic action".into()),
        };
        let energies = [
            q.n("tw_welded_energy")?, q.n("tw_bolted_energy")?, q.n("tw_reinforced_energy")?, q.n("tw_prestressed_energy")?,
            q.n("tw_ceramic_energy")?, q.n("tw_masonry_energy")?, q.n("tw_liner_steel_energy")?, q.n("tw_frp_energy")?,
            q.n("tw_sway_energy")?, q.n("tw_rocking_energy")?, q.n("tw_vertical_energy")?,
        ];
        let mut damping_ratios = MODAL_DAMPING_RATIOS.map(|(_, lower, upper)| if damage_limit { lower } else { upper });
        for (index, key) in [(8, "tw_sway_xi"), (9, "tw_rocking_xi"), (10, "tw_vertical_xi")] {
            if energies[index] == 0.0 { continue; }
            let (_, lower, upper) = MODAL_DAMPING_RATIOS[index];
            let ratio = q.n(key)? / 100.0;
            if !(lower..=upper).contains(&ratio) {
                return Err("Foundation component damping lies outside its representative Annex B interval".into());
            }
            damping_ratios[index] = ratio;
        }
        energy_weighted_modal_damping(&energies, &damping_ratios).ok_or("Supply positive total strain energy and finite nonnegative component energies for this mode")?
    } else {
        return Err("Select calculated or experimentally justified modal damping".into());
    };
    let (ag, soil, tb, tc, td, period, mass, resistance) = (q.n("a_g")?, q.n("s_soil")?, q.n("t_b")?, q.n("t_c")?,
        q.n("t_d")?, q.n("t_1")?, q.n("mass")?, q.n("v_rd")?);
    if ![ag, soil, tb, tc, td, period, mass, resistance].iter().all(|value| value.is_finite())
        || ag <= 0.0 || soil <= 0.0 || tb <= 0.0 || tc < tb || td < tc || period < 0.0 || mass <= 0.0 || resistance <= 0.0 {
        return Err("Use positive spectrum and resistance parameters and ordered periods T_B ≤ T_C ≤ T_D".into());
    }
    let eta = part_1::damping_correction(100.0 * damping);
    let se = part_1::elastic_response_spectrum_eta(ag, soil, tb, tc, td, period, eta);
    let (limit, explanation) = if experimental {
        (1.0, format!("ξ = {:.2} % · η = {eta:.4}.", 100.0 * damping))
    } else {
        (0.15, format!("ξ = {:.2} % ≤ 15 % · η = {eta:.4} · EN 1998-6 B(9).", 100.0 * damping))
    };
    Ok(vec![
        Row::new("modal-damping", "Equivalent modal damping ratio", "Äquivalentes modales Dämpfungsverhältnis", QuantityKind::Dimensionless,
            damping, limit, explanation),
        Row::new("modal-force", "Elastic force from the mode", "Elastische Erdbebenkraft der Eigenform", QuantityKind::Force,
            mass * se, resistance, format!("T = {period:.3} s · S_e = {se:.4} m/s² · m_eff = {mass:.0} kg.")),
    ])
}

/// 🏗 EN 1998-6 §8.2: one assessed guy cable of a steel mast.
pub fn guyed_mast_cable(q: &Inputs<'_>) -> Outcome {
    use crate::part_6::guy_cable_stiffness_n_m;

    if q.t("guy_mass_model")? != "included" {
        return Err("Include all cable and ice mass and address any significant impulsive cable response in the separate mast model".into());
    }
    let h = q.n("guy_mast_height")?;
    if !h.is_finite() || h <= 0.0 { return Err("Enter the positive mast height".into()); }
    let weight = match q.t("guy_sag_model")? {
        "short" if h < 40.0 => 0.0,
        "negligible" => 0.0,
        "sag" => {
            let weight = q.n("guy_unit_weight")?;
            if weight <= 0.0 { return Err("The sag model requires the combined positive cable and ice unit weight".into()); }
            weight
        },
        _ => return Err("Model the equilibrium-dependent sag of a tall mast cable unless negligible sag is substantiated".into()),
    };
    let wrap = match q.t("guy_modulus_basis")? {
        "cable" => 0.0,
        "strand" => q.n("guy_wrap_angle")?,
        _ => return Err("Select the tested cable modulus or the modulus of the wrapped strand".into()),
    };
    let length = q.n("guy_length")?;
    let area = q.n("guy_area")?;
    let preload = q.n("guy_preload_stress")?;
    let seismic = q.n("guy_seismic_stress")?;
    if !preload.is_finite() || !seismic.is_finite() || preload <= 0.0 || seismic < 0.0 {
        return Err("The preload must be positive and the maximum seismic stress variation nonnegative".into());
    }
    let Some(stiffness) = guy_cable_stiffness_n_m(length, area, q.n("guy_inclination")?,
        q.n("guy_modulus")?, wrap, weight, q.n("guy_equilibrium_stress")?) else {
        return Err("Use valid cable geometry and positive tensile stress for the equilibrium state".into());
    };
    let behaviour = q.n("q_factor")?;
    if !behaviour.is_finite() || behaviour < 1.0 { return Err("The behaviour factor must be at least one".into()); }
    let stress_status = if seismic < preload { CheckStatus::Pass } else { CheckStatus::Fail };
    Ok(vec![
        Row::new("stress", "Seismic stress variation below the cable preload", "Seismische Spannungsänderung unter Seilvorspannung", QuantityKind::Stress,
            seismic, preload, "EN 1998-6 §8.2(2): Δσ_Ed < σ_pre.".into()).status(stress_status),
        Row::new("behaviour", "Low-dissipative mast behaviour factor", "Verhaltensbeiwert für gering dissipativen Mast", QuantityKind::Dimensionless,
            behaviour, 1.5, "EN 1998-6 §8.2(1): q ≤ 1.5.".into()),
        Row::new("horizontal-stiffness", "Horizontal guy-cable stiffness", "Horizontale Abspannseilsteifigkeit", QuantityKind::Force,
            stiffness, 0.0, format!("EN 1998-6 (8.1): K_eff,h = cos²(α)·A_c·E_eq/ℓ = {stiffness:.3} N/m. F = K_eff,h × 1 m.")).status(CheckStatus::Info),
    ])
}

/// 🌊️ EN 1998-4:2006 Annex B wave passage and §6.5.2 welded-steel strain verification.
pub fn pipe_wave(q: &Inputs<'_>) -> Outcome {
    use crate::part_4::{PipeWaveInput, bonded_pipe_wave_response};
    if q.t("pipe_wave_basis")? != "stable-welded" {
        return Err("Confirm stable homogeneous ground, a welded steel pipe and the Annex B wave model.".into());
    }
    let inputs = PipeWaveInput {
        peak_longitudinal_velocity_m_s: q.n("pipe_peak_velocity")?,
        peak_longitudinal_acceleration_m_s2: q.n("pipe_peak_a_long")?,
        longitudinal_wave_speed_m_s: q.n("pipe_speed_long")?,
        peak_transverse_acceleration_m_s2: q.n("pipe_peak_a_trans")?,
        transverse_wave_speed_m_s: q.n("pipe_speed_trans")?,
        pipe_radius_m: q.n("pipe_radius")?,
        wall_thickness_m: q.n("pipe_thickness")?,
        elastic_modulus_pa: q.n("pipe_modulus")?,
        available_bond_shear_pa: q.n("pipe_available_shear")?,
    };
    let response = bonded_pipe_wave_response(inputs).ok_or("Enter finite wave amplitudes, positive wave speeds and valid pipe geometry and material properties.")?;
    let bond = Row::new("bond", "Perfect bond under (B.6)", "Vollständiger Verbund nach (B.6)", QuantityKind::Stress,
        response.required_bond_shear_pa, inputs.available_bond_shear_pa,
        format!("τₑ = t E a∥ / c∥² = {:.3} Pa, τavailable = {:.3} Pa", response.required_bond_shear_pa, inputs.available_bond_shear_pa));
    let bond = if response.required_bond_shear_pa == 0.0 { bond.status(CheckStatus::Info) } else { bond };
    if !response.bond_sufficient {
        return Ok(vec![bond]);
    }
    let steps = format!("εax = v/c∥ = {:.7}, χ = a⊥/c⊥² = {:.7} m⁻¹, |ε| ≤ εax + χr = {:.7}",
        response.axial_strain, response.curvature_per_m, response.envelope_strain);
    Ok(vec![
        bond,
        Row::new("tension", "Upper bound on tensile strain", "Obere Grenze der Zugdehnung", QuantityKind::Dimensionless,
            response.envelope_strain, response.tensile_strain_limit, steps.clone()),
        Row::new("compression", "Upper bound on compressive strain", "Obere Grenze der Druckdehnung", QuantityKind::Dimensionless,
            response.envelope_strain, response.compressive_strain_limit, steps),
    ])
}

/// 🌉️ EN 1998-2:2005+A2:2011 §3.3 Set A per support in one direction.
pub fn bridge_spatial_set_a(q: &Inputs<'_>) -> Outcome {
    use crate::part_2::{spatial_set_a, spatial_variability_required};
    let length = q.n("spatial_deck_length")?;
    let limit = q.n("spatial_length_limit")?;
    let required = spatial_variability_required(length, limit, q.t("spatial_ground_types")? == "variable")
        .ok_or("Enter positive finite deck and limiting lengths.")?;
    if !required {
        return Err("The ground type is uniform and the deck does not exceed the supplied length limit.".into());
    }
    let (strain, displacement) = spatial_set_a(q.n("spatial_ground_displacement")?, q.n("spatial_reference_distance")?, q.n("spatial_decorrelation")?)
        .ok_or("Enter a nonnegative design displacement and reference distance and a positive decorrelation distance.")?;
    Ok(vec![
        Row::new("set-a", "Same-sign Set A support displacement", "Gleichgerichtete Auflagerverschiebung nach Satz A", QuantityKind::Length,
            displacement, 0.0, format!("dri = min(εr Li, √2 dg) = {displacement:.6} m")).status(CheckStatus::Info),
        Row::new("strain", "Set A ground strain", "Bodendehnung nach Satz A", QuantityKind::Dimensionless,
            strain, 0.0, format!("εr = √2 dg / Lg = {strain:.8}")).status(CheckStatus::Info),
    ])
}

/// ↔️ EN 1998-2:2005+A2:2011 §3.3 Set B gives magnitudes to apply with opposite signs at adjacent supports.
pub fn bridge_spatial_set_b(q: &Inputs<'_>) -> Outcome {
    use crate::part_2::spatial_set_b;
    let displacement = spatial_set_b(q.n("spatial_left_spacing")?, q.n("spatial_right_spacing")?,
        q.n("spatial_governing_strain")?, q.n("spatial_opposite_factor")?)
        .ok_or("Enter the actual adjacent support spacing and nonnegative values for the governing strain and βr.")?;
    Ok(vec![Row::new("set-b", "Absolute displacement magnitude at this support", "Betrag der absoluten Verschiebung an diesem Auflager",
        QuantityKind::Length, displacement, 0.0, format!("|di| = βr εr Lav / 2 = {displacement:.6} m")).status(CheckStatus::Info)])
}

/// 📊️ EN 1998-2:2005+A2:2011 §3.3(7) SRSS with the governing pseudo-static effect.
pub fn bridge_spatial_effect(q: &Inputs<'_>) -> Outcome {
    use crate::part_2::spatial_combined_effect;
    let kind = match q.t("spatial_effect_kind")? {
        "force" => QuantityKind::Force,
        "moment" => QuantityKind::Moment,
        "movement" => QuantityKind::Length,
        _ => return Err("Select the effect quantity and use the same units for all three inputs.".into()),
    };
    let values = [q.n("spatial_inertia")?, q.n("spatial_effect_a")?, q.n("spatial_effect_b")?];
    let effect = spatial_combined_effect(values[0], values[1], values[2]).ok_or("Enter finite structural effects in consistent units.")?;
    Ok(vec![Row::new("srss", "Combined structural effect", "Überlagerte Auswirkung", kind, effect, 0.0,
        format!("√(EI² + max(|EA|, |EB|)²) = {effect:.6}")).status(CheckStatus::Info)])
}
