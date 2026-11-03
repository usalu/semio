//! 🧩️ Diagrams of the Annex B and C, fire and bridge entries, each with its table twin: the slip efficiency of T-90 over the slip modulus, the effective slenderness of T-93 over the spacer distance, the reduced-properties factors of T-106 and the CLT char with fall-off of T-107 over the fire duration, the S-N curve and the Miner blocks of T-121, the prestress stages of T-122 and the crowd acceleration of T-120 over the density. Markers drag the case leaves. See [`super`].

use super::*;
use entities::jointed_parts;

pub(super) fn val(c: &CaseRow, key: &str) -> Option<f64> {
    c.numbers.iter().find(|n| n.id == key).map(|n| n.value)
}

pub(super) fn text<'a>(c: &'a CaseRow, key: &str) -> Option<&'a str> {
    c.texts.iter().find(|t| t.id == key).map(|t| t.value.as_str())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn marker(id: &str, at: Vec2, tone: Tone, label: String, utilisation: Option<f64>, bound_check: Option<String>, binding: Option<FieldBinding>) -> Marker {
    Marker { id: id.into(), at, shape: MarkerShape::Diamond, tone, label: loc(&label, &label), utilisation, bound_check, binding }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn diagram(c: &CaseRow, key: &str, title: (&str, &str), description: (&str, &str), axes: Vec<Axis>, series: Vec<Series>, markers: Vec<Marker>, twin: Vec<DataColumn>) -> Diagram2d {
    Diagram2d {
        id: format!("{key}:{}", c.id),
        title: loc(&format!("{} – {}", title.0, c.label_en), &format!("{} – {}", title.1, c.label_de)),
        description: loc(description.0, description.1),
        axes,
        series,
        markers,
        annotations: Vec::new(),
        focus: focus_case(c),
        legend: true,
        table_twin: DataTable { columns: twin },
    }
}

fn gamma_efficiency(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let (parts, k, l) = jointed_parts(c).filter(|(_, k, _)| *k > 0.0)?;
    let at = |x: f64| t::gamma_beam(&parts, x, l);
    let (lo, hi) = (0.02 * k, 4.0 * k);
    let efficiency = curves::sample(|x| at(x).ei_ef / at(x).ei_rigid, lo, hi, SAMPLES);
    let gamma_1 = curves::sample(|x| at(x).gamma[0], lo, hi, SAMPLES);
    let mut series = vec![polyline("efficiency", "(EI)_ef / (EI) rigid", "(EI)_ef / (EI) starr", efficiency.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), polyline("gamma-1", "γ₁", "γ₁", gamma_1.clone(), stroke(Tone::Warning, 1.5, Dash::Dashed))];
    let mut twin = vec![column("Slip modulus K", "Verschiebungsmodul K", "N/mm", efficiency.iter().map(|q| q[0]).collect()), column("Efficiency", "Wirkungsgrad", "1", efficiency.iter().map(|q| q[1]).collect()), column("γ₁", "γ₁", "1", gamma_1.iter().map(|q| q[1]).collect())];
    if parts.len() > 2 {
        let gamma_3 = curves::sample(|x| at(x).gamma[2], lo, hi, SAMPLES);
        series.push(polyline("gamma-3", "γ₃", "γ₃", gamma_3.clone(), stroke(Tone::Neutral, 1.5, Dash::Dotted)));
        twin.push(column("γ₃", "γ₃", "1", gamma_3.iter().map(|q| q[1]).collect()));
    }
    let now = at(k);
    let check = worst(report, &c.id, "en1995.t-90");
    let m = marker("marker-k", Vec2::new(k, now.ei_ef / now.ei_rigid), check.map_or(Tone::Neutral, |k| utilisation_tone(k.utilization)), format!("K = {k:.0} N/mm, efficiency {:.2}", now.ei_ef / now.ei_rigid), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "k"), "N/mm", Affine::IDENTITY, 100.0)));
    Some(diagram(c, "case-gamma", ("Slip efficiency of the γ-method", "Verbundwirkungsgrad des γ-Verfahrens"), ("Effective over rigid stiffness and the reduction factor γ₁ against the slip modulus of the fasteners (EN 1995-1-1 Annex B); the marker drags K.", "Wirksame zu starrer Steifigkeit und Abminderungsbeiwert γ₁ über dem Verschiebungsmodul der Verbindungsmittel (EN 1995-1-1 Anhang B); der Marker ändert K."), vec![axis("Slip modulus K", "Verschiebungsmodul K", QuantityKind::ForcePerLength, "N/mm", Some((lo, hi)), TickRule::Auto), axis("Efficiency", "Wirkungsgrad", QuantityKind::Dimensionless, "1", Some((0.0, 1.05)), TickRule::Auto)], series, vec![m], twin))
}

fn column_slenderness(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let (lef, i_tot, l1, i1, n) = (val(c, "lef")?, val(c, "i_tot")?, val(c, "l1")?, val(c, "i1")?, val(c, "n_parts")?);
    if i_tot <= 0.0 || i1 <= 0.0 || l1 <= 0.0 {
        return None;
    }
    let eta = val(c, "eta").unwrap_or_else(|| norm::ndp_common(&format!("ndp.en1995.annex-c.eta.{}", match text(c, "joint").unwrap_or("nailed") { joint @ ("glued" | "bolted" | "connectors") => joint, _ => "nailed" })));
    let lam = lef / i_tot;
    let effective = |spacing: f64| (lam * lam + eta * n / 2.0 * (spacing / i1).powi(2)).sqrt();
    let curve = curves::sample(effective, 0.1 * l1, 2.0 * l1, SAMPLES);
    let rigid = curves::sample(|_| lam, 0.1 * l1, 2.0 * l1, SAMPLES);
    let check = worst(report, &c.id, "en1995.t-93");
    let m = marker("marker-l1", Vec2::new(l1, effective(l1)), check.map_or(Tone::Neutral, |k| utilisation_tone(k.utilization)), format!("l₁ = {l1:.0} mm, λ_ef = {:.1}", effective(l1)), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "l1"), "mm", Affine::IDENTITY, 10.0)));
    Some(diagram(c, "case-column", ("Effective slenderness of the jointed column", "Wirksamer Schlankheitsgrad des verbundenen Druckstabs"), ("λ_ef = √(λ² + η n/2 λ₁²) over the spacer distance l₁ against the rigid slenderness (EN 1995-1-1 Annex C); the marker drags l₁.", "λ_ef = √(λ² + η n/2 λ₁²) über dem Distanzholzabstand l₁ gegen den starren Schlankheitsgrad (EN 1995-1-1 Anhang C); der Marker ändert l₁."), vec![axis("Spacer distance l₁", "Distanzholzabstand l₁", QuantityKind::Length, "mm", Some((0.1 * l1, 2.0 * l1)), TickRule::Auto), axis("Slenderness", "Schlankheitsgrad", QuantityKind::Dimensionless, "1", None, TickRule::Auto)], vec![polyline("lambda-ef", "λ_ef", "λ_ef", curve.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), polyline("lambda-rigid", "λ rigid", "λ starr", rigid.clone(), stroke(Tone::Neutral, 1.5, Dash::Dashed))], vec![m], vec![column("Spacer distance l₁", "Distanzholzabstand l₁", "mm", curve.iter().map(|q| q[0]).collect()), column("λ_ef", "λ_ef", "1", curve.iter().map(|q| q[1]).collect()), column("λ rigid", "λ starr", "1", rigid.iter().map(|q| q[1]).collect())]))
}

fn reduced_properties_curve(d: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let (b, h, t_req) = (val(c, "b")?, val(c, "h")?, val(c, "t_req")?);
    let faces = [val(c, "face_left").unwrap_or(1.0), val(c, "face_right").unwrap_or(1.0), val(c, "face_top").unwrap_or(0.0), val(c, "face_bottom").unwrap_or(1.0)];
    let beta = t::AnnexParams::charring_rate(d.annex, entities::product_of(c)) * 1e3 * 60.0;
    let end = (t_req * 1.5).max(90.0);
    let at = |time: f64| t::reduced_properties(b, h, faces, beta, time);
    let bending = curves::sample(|x| at(x).k_bend, 0.0, end, SAMPLES);
    let compression = curves::sample(|x| at(x).k_comp, 0.0, end, SAMPLES);
    let tension = curves::sample(|x| at(x).k_ten, 0.0, end, SAMPLES);
    let check = worst(report, &c.id, "en1995.t-106");
    let m = marker("marker-t-req", Vec2::new(t_req, at(t_req).k_bend), check.map_or(Tone::Neutral, |k| utilisation_tone(k.utilization)), format!("{t_req:.0} min: k_mod,fi = {:.2}", at(t_req).k_bend), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "t_req"), "min", Affine::IDENTITY, 5.0)));
    Some(diagram(c, "case-fire-reduced", ("Reduced properties over the fire duration", "Reduzierte Eigenschaften über der Branddauer"), ("k_mod,fi for bending, compression and tension from the perimeter over the residual area (EN 1995-1-2 4.2.3); the marker drags the required time.", "k_mod,fi für Biegung, Druck und Zug aus Umfang zu Restfläche (EN 1995-1-2 4.2.3); der Marker ändert die geforderte Zeit."), vec![axis("Fire duration", "Branddauer", QuantityKind::Time, "min", Some((0.0, end)), TickRule::Auto), axis("k_mod,fi", "k_mod,fi", QuantityKind::Dimensionless, "1", Some((0.0, 1.05)), TickRule::Auto)], vec![polyline("k-bending", "Bending", "Biegung", bending.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), polyline("k-compression", "Compression", "Druck", compression.clone(), stroke(Tone::Warning, 1.5, Dash::Dashed)), polyline("k-tension", "Tension and stiffness", "Zug und Steifigkeit", tension.clone(), stroke(Tone::Neutral, 1.5, Dash::Dotted))], vec![m], vec![column("Fire duration", "Branddauer", "min", bending.iter().map(|q| q[0]).collect()), column("k_mod,fi bending", "k_mod,fi Biegung", "1", bending.iter().map(|q| q[1]).collect()), column("k_mod,fi compression", "k_mod,fi Druck", "1", compression.iter().map(|q| q[1]).collect()), column("k_mod,fi tension", "k_mod,fi Zug", "1", tension.iter().map(|q| q[1]).collect())]))
}

fn clt_fire_curve(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let g = entities::geo(c);
    let t_req = val(c, "t_req").filter(|t| *t > 0.0)?;
    if g.layers.is_empty() {
        return None;
    }
    let thick: Vec<f64> = g.layers.iter().map(|l| l.0 * 1e3).collect();
    let (beta, factor, protect, on) = (val(c, "beta_0").unwrap_or(0.65), val(c, "fall_off_factor").unwrap_or_else(|| norm::ndp_common("ndp.en1995.fire.clt-fall-off-factor")), val(c, "protect_mm").unwrap_or_else(|| norm::ndp_common("ndp.en1995.fire.clt-fall-off-protect")), val(c, "fall_off").unwrap_or(1.0) > 0.5);
    let total: f64 = thick.iter().sum();
    let end = (t_req * 1.5).max(90.0);
    let with = curves::sample(|x| t::clt_char_depth(&thick, beta, x, on, factor, protect).0, 0.0, end, SAMPLES);
    let without = curves::sample(|x| t::clt_char_depth(&thick, beta, x, false, factor, protect).0, 0.0, end, SAMPLES);
    let mut series = vec![polyline("with-fall-off", "With fall-off", "Mit Abfallen", with.clone(), stroke(Tone::Error, 2.0, Dash::Solid)), polyline("without-fall-off", "Without fall-off", "Ohne Abfallen", without.clone(), stroke(Tone::Neutral, 1.5, Dash::Dashed))];
    let mut edge = 0.0;
    for (i, t_i) in thick.iter().enumerate().take(thick.len().saturating_sub(1)) {
        edge += t_i;
        series.push(polyline(&format!("glue-line-{}", i + 1), &format!("Glue line {}", i + 1), &format!("Klebfuge {}", i + 1), vec![[0.0, edge], [end, edge]], stroke(Tone::Info, 1.0, Dash::Dotted)));
    }
    let now = t::clt_char_depth(&thick, beta, t_req, on, factor, protect).0;
    let check = worst(report, &c.id, "en1995.t-107");
    let m = marker("marker-t-req", Vec2::new(t_req, now), check.map_or(Tone::Neutral, |k| utilisation_tone(k.utilization)), format!("{t_req:.0} min: d_char = {now:.1} mm"), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "t_req"), "min", Affine::IDENTITY, 5.0)));
    Some(diagram(c, "case-clt-fire", ("CLT char with fall-off", "BSP-Abbrand mit Abfallen"), ("Char depth over the fire duration with and without fall-off of the char layer at the glue lines; the marker drags the required time.", "Abbrandtiefe über der Branddauer mit und ohne Abfallen der Kohleschicht an den Klebfugen; der Marker ändert die geforderte Zeit."), vec![axis("Fire duration", "Branddauer", QuantityKind::Time, "min", Some((0.0, end)), TickRule::Auto), axis("Char depth", "Abbrandtiefe", QuantityKind::Length, "mm", Some((0.0, total)), TickRule::Auto)], series, vec![m], vec![column("Fire duration", "Branddauer", "min", with.iter().map(|q| q[0]).collect()), column("Char depth with fall-off", "Abbrandtiefe mit Abfallen", "mm", with.iter().map(|q| q[1]).collect()), column("Char depth without fall-off", "Abbrandtiefe ohne Abfallen", "mm", without.iter().map(|q| q[1]).collect())]))
}

fn fatigue_curves(report: &CheckReport, c: &CaseRow) -> Vec<Diagram2d> {
    let blocks: Vec<(usize, f64, f64)> = (1..=6).filter_map(|k| Some((k, val(c, &format!("ds{k}"))?, val(c, &format!("n{k}"))?))).filter(|b| b.1 > 0.0 && b.2 > 0.0).collect();
    if blocks.is_empty() {
        return Vec::new();
    }
    let (a, b, years, cut) = (val(c, "sn_a").unwrap_or(15.0), val(c, "sn_b").unwrap_or(4.0), val(c, "years").unwrap_or(1.0), val(c, "ds_cut").unwrap_or(0.0));
    let lowest = blocks.iter().fold(f64::MAX, |m, x| m.min(x.1)) * 0.5;
    let highest = blocks.iter().fold(0.0_f64, |m, x| m.max(x.1)) * 2.0;
    let curve: Vec<Xy> = (0..=SAMPLES).map(|i| lowest * (highest / lowest).powf(i as f64 / SAMPLES as f64)).map(|ds| [10f64.powf(a - b * ds.log10()), ds]).collect();
    let check = worst(report, &c.id, "en1995.t-121.miner");
    let markers = blocks.iter().map(|(k, ds, n)| {
        let damage = if *ds < cut { 0.0 } else { t::miner_damage(a, b, *ds, n * years) };
        marker(&format!("marker-n{k}"), Vec2::new(n * years, *ds), utilisation_tone(damage), format!("block {k}: Δσ {ds:.1} N/mm², {:.2e} cycles, D = {damage:.3}", n * years), Some(damage), check.map(|k| k.id.clone()), Some(binding(case_binding(c, &format!("n{k}")), "1", Affine { base: 0.0, k: 1.0 / years }, 1000.0)))
    }).collect();
    let sn = diagram(c, "case-sn", ("S-N curve with the load blocks", "Wöhlerlinie mit den Lastblöcken"), ("Stress range over the cycles to failure N = 10^(a − b·log Δσ) with the cycles of every load block; the markers drag the block cycles.", "Spannungsschwingbreite über den Bruchlastspielen N = 10^(a − b·log Δσ) mit den Lastspielen jedes Lastblocks; die Marker ändern die Lastspiele."), vec![Axis { scale: AxisScale::Log10, ..axis("Cycles N", "Lastspiele N", QuantityKind::Dimensionless, "1", None, TickRule::Auto) }, Axis { scale: AxisScale::Log10, ..axis("Stress range", "Spannungsschwingbreite", QuantityKind::Stress, "N/mm²", Some((lowest, highest)), TickRule::Auto) }], vec![polyline("s-n", "S-N curve", "Wöhlerlinie", curve.clone(), stroke(Tone::Info, 2.0, Dash::Solid))], markers, vec![column("Cycles to failure N", "Bruchlastspiele N", "1", curve.iter().map(|q| q[0]).collect()), column("Stress range", "Spannungsschwingbreite", "N/mm²", curve.iter().map(|q| q[1]).collect())]);
    let damages: Vec<f64> = blocks.iter().map(|(_, ds, n)| if *ds < cut { 0.0 } else { t::miner_damage(a, b, *ds, n * years) }).collect();
    let total: f64 = damages.iter().sum();
    let count = blocks.len() as f64;
    let bars = damages.iter().enumerate().map(|(i, d)| bar(&format!("damage-{}", blocks[i].0), &format!("Block {}", blocks[i].0), &format!("Block {}", blocks[i].0), i as f64 + 0.6, i as f64 + 1.4, *d, utilisation_tone(*d))).collect();
    let miner = diagram(c, "case-miner", ("Miner damage per load block", "Miner-Schädigung je Lastblock"), ("Damage n/N of every load block; the sum must stay below 1.", "Schädigung n/N jedes Lastblocks; die Summe muss unter 1 bleiben."), vec![axis("Block", "Block", QuantityKind::Dimensionless, "1", Some((0.4, count + 0.6)), TickRule::Explicit { values: (1..=blocks.len()).map(|i| i as f64).collect() }), axis("Damage", "Schädigung", QuantityKind::Dimensionless, "1", Some((0.0, total.max(1.0) * 1.1)), TickRule::Auto)], bars, Vec::new(), vec![column("Block", "Block", "1", (1..=blocks.len()).map(|i| i as f64).collect()), column("Damage", "Schädigung", "1", damages.clone())]);
    vec![sn, miner]
}

fn prestress_stages(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let initial = val(c, "sigma_p_0")?;
    let staged: Vec<(&str, &str, &str)> = [("loss_relax", "After relaxation", "Nach Relaxation"), ("loss_creep", "After creep", "Nach Kriechen"), ("loss_mc", "After moisture change", "Nach Feuchteänderung")].into_iter().filter(|(k, _, _)| val(c, k).is_some()).collect();
    let mut levels = vec![initial];
    let mut names = vec![("Initial".to_string(), "Anfang".to_string())];
    for (key, en, de) in &staged {
        levels.push(levels[levels.len() - 1] * (1.0 - val(c, key).unwrap_or(0.0)));
        names.push(((*en).into(), (*de).into()));
    }
    if staged.is_empty() {
        levels.push(initial * (1.0 - val(c, "loss").unwrap_or(0.55)));
        names.push(("After losses".into(), "Nach Verlusten".into()));
    }
    let count = levels.len();
    let check = worst(report, &c.id, "en1995.t-122.prestress_min");
    let min_id = "ndp.en1995.deck.min-prestress";
    let min = norm::ndp_common(min_id);
    let (flag_en, flag_de) = if norm::ndp_is_verified_for(min_id, norm::AnnexChoice::En) { ("", "") } else { (" (unverified)", " (ungeprüft)") };
    let bars = (0..count).map(|i| bar(&format!("stage-{i}"), &names[i].0, &names[i].1, i as f64 + 0.6, i as f64 + 1.4, levels[i], if levels[i] < min { Tone::Error } else { Tone::Info })).collect();
    let markers = (1..count).map(|i| {
        let binding_for = staged.get(i - 1).map(|(key, _, _)| binding(case_binding(c, key), "1", Affine { base: 1.0, k: -1.0 / levels[i - 1].max(1e-9) }, 0.01));
        marker(&format!("marker-stage-{i}"), Vec2::new(i as f64 + 1.0, levels[i]), if levels[i] < min { Tone::Error } else { Tone::Info }, format!("{}: {:.3} N/mm²", names[i].0, levels[i]), check.map(|k| k.utilization), check.map(|k| k.id.clone()), binding_for)
    }).collect();
    let limit = polyline("minimum", &format!("Minimum {min} N/mm²{flag_en}"), &format!("Minimum {} N/mm²{flag_de}", format!("{min}").replace('.', ",")), vec![[0.4, min], [count as f64 + 0.6, min]], stroke(Tone::Error, 1.5, Dash::Dashed));
    let mut series: Vec<Series> = bars;
    series.push(limit);
    Some(diagram(c, "case-prestress", ("Prestress after the loss stages", "Vorspannung nach den Verluststufen"), (&format!("Prestress of the stress-laminated deck after relaxation, creep and moisture change against the minimum of {min} N/mm²{flag_en}; the markers drag the loss of their stage."), &format!("Vorspannung des Brettstapeldecks nach Relaxation, Kriechen und Feuchteänderung gegen das Minimum von {} N/mm²{flag_de}; die Marker ändern den Verlust ihrer Stufe.", format!("{min}").replace('.', ","))), vec![axis("Stage", "Stufe", QuantityKind::Dimensionless, "1", Some((0.4, count as f64 + 0.6)), TickRule::Explicit { values: (1..=count).map(|i| i as f64).collect() }), axis("Prestress", "Vorspannung", QuantityKind::Stress, "N/mm²", Some((0.0, initial * 1.1)), TickRule::Auto)], series, markers, vec![column("Stage", "Stufe", "1", (1..=count).map(|i| i as f64).collect()), column("Prestress", "Vorspannung", "N/mm²", levels.clone())]))
}

fn crowd_acceleration_curve(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let (span, ei, mass, density, width) = (val(c, "span_m")?, val(c, "ei_nm2")?, val(c, "mass_kg_m")?, val(c, "crowd_density")?, val(c, "deck_width_m")?);
    let zeta = val(c, "zeta").unwrap_or(0.015);
    let limit = val(c, "a_limit").unwrap_or(0.7);
    let f1 = t::bridge_f1(span, ei, mass);
    let single = t::single_pedestrian_acceleration(f1, mass * span / 2.0, zeta, true);
    let harmonic = |d: f64| t::crowd_acceleration(span, width, d, f1, mass, zeta);
    let group = |d: f64| if d * span * width > 1.0 { 0.23 * single * d * span * width } else { single };
    let top = (density * 2.0).max(1.5);
    let setra = curves::sample(harmonic, 0.0, top, SAMPLES);
    let en = curves::sample(group, 0.0, top, SAMPLES);
    let check = worst(report, &c.id, "en1995.t-120.a_crowd");
    let m = marker("marker-density", Vec2::new(density, harmonic(density)), check.map_or(Tone::Neutral, |k| utilisation_tone(k.utilization)), format!("{density:.2} persons/m²: a = {:.2} m/s²", harmonic(density)), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "crowd_density"), "1", Affine::IDENTITY, 0.05)));
    Some(diagram(c, "case-crowd", ("Footbridge acceleration over the crowd density", "Brückenbeschleunigung über der Personendichte"), ("Resonant vertical acceleration of the harmonic crowd model and of the group formula 0.23·a₁·n against the comfort limit; the marker drags the density.", "Resonante vertikale Beschleunigung des harmonischen Menschenmengenmodells und der Gruppenformel 0,23·a₁·n gegen die Komfortgrenze; der Marker ändert die Dichte."), vec![axis("Crowd density", "Personendichte", QuantityKind::Dimensionless, "1/m²", Some((0.0, top)), TickRule::Auto), axis("Acceleration", "Beschleunigung", QuantityKind::Acceleration, "m/s²", None, TickRule::Auto)], vec![polyline("harmonic-crowd", "Harmonic crowd", "Harmonische Menschenmenge", setra.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), polyline("group-formula", "Group formula", "Gruppenformel", en.clone(), stroke(Tone::Warning, 1.5, Dash::Dashed)), polyline("limit", "Comfort limit", "Komfortgrenze", vec![[0.0, limit], [top, limit]], stroke(Tone::Error, 1.5, Dash::Dotted))], vec![m], vec![column("Crowd density", "Personendichte", "1/m²", setra.iter().map(|q| q[0]).collect()), column("Harmonic crowd", "Harmonische Menschenmenge", "m/s²", setra.iter().map(|q| q[1]).collect()), column("Group formula", "Gruppenformel", "m/s²", en.iter().map(|q| q[1]).collect())]))
}

/// 📈️ The stress over the depth of a jointed beam (EN 1995-1-1 Annex B) with the slip of the fasteners against the rigid composite: per part the fibre stress at its bottom and top face, so the jump at an interface shows the part carrying its own bending; part 1 on top.
fn joint_stress_profile(c: &CaseRow) -> Option<Diagram2d> {
    let (parts, k, l) = jointed_parts(c).filter(|(_, k, _)| *k > 0.0)?;
    let (m, v) = (val(c, "m_d")?, val(c, "v_d").unwrap_or(0.0));
    let (slip, rigid) = (t::joint_profile(&parts, k, l, m, v, false), t::joint_profile(&parts, k, l, m, v, true));
    let (mut y, mut depth, mut with_slip, mut without) = (0.0, Vec::new(), Vec::new(), Vec::new());
    for i in (0..parts.len()).rev() {
        depth.extend([y, y + parts[i].h]);
        with_slip.extend([slip.fibres[i].bottom, slip.fibres[i].top]);
        without.extend([rigid.fibres[i].bottom, rigid.fibres[i].top]);
        y += parts[i].h;
    }
    let line = |x: &[f64]| x.iter().zip(&depth).map(|(s, z)| [*s, *z]).collect::<Vec<_>>();
    let (a, b) = (line(&with_slip), line(&without));
    Some(diagram(c, "case-joint-stress", ("Stress over the depth of the jointed beam", "Spannung über die Höhe des nachgiebig verbundenen Trägers"), ("Fibre stress of every part with the slip of the fasteners (γ-method, EN 1995-1-1 Annex B) against the rigid composite; part 1 is on top, the jump at an interface is the bending of the single parts.", "Randspannung jedes Teils mit dem Schlupf der Verbindungsmittel (γ-Verfahren, EN 1995-1-1 Anhang B) gegenüber dem starren Verbund; Teil 1 liegt oben, der Sprung an einer Fuge ist die Biegung der Einzelteile."), vec![axis("Stress", "Spannung", QuantityKind::Stress, "N/mm²", None, TickRule::Auto), axis("Height above the bottom", "Höhe über der Unterkante", QuantityKind::Length, "mm", None, TickRule::Auto)], vec![polyline("slip", "With slip", "Mit Schlupf", a.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), polyline("rigid", "Rigid", "Starr", b.clone(), stroke(Tone::Neutral, 1.5, Dash::Dashed))], Vec::new(), vec![column("Height", "Höhe", "mm", depth.clone()), column("With slip", "Mit Schlupf", "N/mm²", with_slip), column("Rigid", "Starr", "N/mm²", without)]))
}

/// 📈️ The slip of the fastener rows over the span of a jointed beam: the shear follows cos(πx/l), so the slip F/K is largest at the supports and zero at midspan; the marker drags the slip modulus K.
fn joint_slip_curve(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let (parts, k, l) = jointed_parts(c).filter(|(_, k, _)| *k > 0.0)?;
    let (m, v) = (val(c, "m_d")?, val(c, "v_d").unwrap_or(0.0));
    if v <= 0.0 {
        return None;
    }
    let profile = t::joint_profile(&parts, k, l, m, v, false);
    let owner = if profile.slip[0] >= profile.slip[2] { 0 } else { 2 };
    let slip = curves::sample(|x| profile.slip_at(owner, x, l), 0.0, l, SAMPLES);
    let force = curves::sample(|x| profile.fastener[owner].abs() * (std::f64::consts::PI * x / l).cos().abs(), 0.0, l, SAMPLES);
    let check = worst(report, &c.id, "en1995.t-90");
    let m = marker("marker-slip", Vec2::new(0.0, profile.slip[owner]), check.map_or(Tone::Neutral, |k| utilisation_tone(k.utilization)), format!("u = {:.3} mm, F = {:.0} N at the support", profile.slip[owner], profile.fastener[owner].abs()), check.map(|k| k.utilization), check.map(|k| k.id.clone()), Some(binding(case_binding(c, "k"), "N/mm", Affine::IDENTITY, 100.0)));
    Some(diagram(c, "case-joint-slip", ("Slip of the fasteners over the span", "Schlupf der Verbindungsmittel über die Spannweite"), ("Slip u = F/K and fastener force F of the interface of part 1 or 3 along the span for the shear V cos(πx/l) (EN 1995-1-1 Annex B); the marker at the support drags K.", "Schlupf u = F/K und Verbindungsmittelkraft F der Fuge von Teil 1 oder 3 entlang der Spannweite für die Querkraft V cos(πx/l) (EN 1995-1-1 Anhang B); der Marker am Auflager ändert K."), vec![axis("Position x", "Stelle x", QuantityKind::Length, "mm", Some((0.0, l)), TickRule::Auto), axis("Slip", "Schlupf", QuantityKind::Length, "mm", None, TickRule::Auto)], vec![polyline("slip", "Slip u", "Schlupf u", slip.clone(), stroke(Tone::Info, 2.0, Dash::Solid))], vec![m], vec![column("Position x", "Stelle x", "mm", slip.iter().map(|q| q[0]).collect()), column("Slip u", "Schlupf u", "mm", slip.iter().map(|q| q[1]).collect()), column("Fastener force F", "Verbindungsmittelkraft F", "N", force.iter().map(|q| q[1]).collect())]))
}

/// 📈️ The special diagrams of one case, chosen by its catalogue entry.
pub(crate) fn case_diagrams(document: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Vec<Diagram2d> {
    match c.entry_id.as_str() {
        "T-90" => gamma_efficiency(report, c).into_iter().chain(joint_stress_profile(c)).chain(joint_slip_curve(report, c)).collect(),
        "T-93" => column_slenderness(report, c).into_iter().collect(),
        "T-106" => reduced_properties_curve(document, report, c).into_iter().collect(),
        "T-107" => clt_fire_curve(report, c).into_iter().collect(),
        "T-121" => fatigue_curves(report, c),
        "T-122" => prestress_stages(report, c).into_iter().collect(),
        "T-120" => crowd_acceleration_curve(report, c).into_iter().collect(),
        _ => Vec::new(),
    }
}
