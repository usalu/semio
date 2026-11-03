//! 🧱️ EN 1991-1-1 catalogue entries E11.01 … E11.09: densities, partitions, imposed loads and reductions, barriers, forklifts, load spreading and wheel pressures.

use super::*;

fn material_gamma(c: &Case, suffix: &str) -> Result<(String, f64), String> {
    let name = c.s(&format!("material{suffix}"))?.to_string();
    let gamma = match c.opt(&format!("gamma{suffix}")) {
        Some(g) if g > 0.0 => g,
        Some(_) => return Err(format!("input gamma{suffix} must be positive")),
        None => part_1_1::self_weight_n_m3(&name, c.annex).ok_or_else(|| format!("unknown material {name}"))?,
    };
    Ok((name, gamma))
}

fn e11_01(c: &Case) -> R {
    let mut out = Vec::new();
    let mut total = 0.0;
    for suffix in ["", "2", "3"] {
        if suffix != "" && !c.text.contains_key(&format!("material{suffix}")) { continue; }
        let (name, gamma) = material_gamma(c, suffix)?;
        let t = c.pos(&format!("thickness{suffix}"))?;
        let g = gamma * t;
        total += g;
        out.push(Outcome::info(&format!("g{suffix}"), &format!("Layer {name}"), &format!("Schicht {name}"), QuantityKind::Pressure, g).steps(format!("g = γ·t = {:.2} kN/m³ · {:.3} m = {:.3} kN/m².", gamma / 1000.0, t, g / 1000.0)).basis(&["ndp.en1991.e11.density"]));
    }
    out.push(Outcome::info("total", "Sum of layers g_k", "Summe der Schichten g_k", QuantityKind::Pressure, total).steps(format!("g_k = Σ γ·t = {:.3} kN/m².", total / 1000.0)).basis(&["ndp.en1991.e11.density"]));
    Ok(out)
}

fn e11_02(c: &Case) -> R {
    let line = match c.opt("wall_load") {
        Some(v) => v,
        None => c.pos("wall_gamma")? * c.pos("wall_thickness")? * c.pos("wall_height")? + c.o("finish", 0.0),
    };
    let q = part_1_1::partition_allowance_pa(line / 1000.0, c.annex).ok_or_else(|| format!("wall load {:.2} kN/m exceeds the equivalent-load range; apply the wall as an individual line load", line / 1000.0))?;
    Ok(vec![
        Outcome::info("line", "Wall line load g", "Wandlast g", QuantityKind::Force, line).steps(format!("g = {:.3} kN/m.", line / 1000.0)).basis(&["ndp.en1991.e11.partition"]),
        Outcome::info("delta_q", "Equivalent uniform load Δq_k", "Ersatzflächenlast Δq_k", QuantityKind::Pressure, q).steps(format!("Δq_k = {:.2} kN/m² for g = {:.3} kN/m.", q / 1000.0, line / 1000.0)).steps_de(format!("Δq_k = {:.2} kN/m² für g = {:.3} kN/m.", q / 1000.0, line / 1000.0)).basis(&["ndp.en1991.e11.partition"]),
    ])
}

fn category_row(c: &Case) -> Result<(String, f64, f64), String> {
    let cat = c.s("category")?.to_string();
    let (q, big_q) = part_1_1::imposed_row(&cat, c.annex).ok_or_else(|| format!("unknown category {cat}; admissible: {}", part_1_1::categories(c.annex).join(", ")))?;
    Ok((cat, q * 1000.0, big_q * 1000.0))
}

fn e11_03(c: &Case) -> R {
    let (cat, q, big_q) = category_row(c)?;
    let mut out = vec![
        Outcome::info("q_k", "Uniformly distributed load q_k", "Gleichmäßig verteilte Last q_k", QuantityKind::Pressure, q).steps(format!("Category {cat}: q_k = {:.2} kN/m².", q / 1000.0)).basis(&["ndp.en1991.e11.na-table"]),
        Outcome::info("Q_k", "Concentrated load Q_k", "Einzellast Q_k", QuantityKind::Force, big_q).steps(format!("Category {cat}: Q_k = {:.2} kN on 50 × 50 mm.", big_q / 1000.0)).basis(&["ndp.en1991.e11.na-table"]),
    ];
    if let Some(depth) = c.opt("depth") {
        out.push(Outcome::info("line", "Line load over the depth", "Linienlast über die Tiefe", QuantityKind::Force, q * depth).steps(format!("q_k·depth = {:.2} kN/m² · {depth:.2} m = {:.2} kN/m.", q / 1000.0, q * depth / 1000.0)).basis(&["ndp.en1991.e11.na-table"]));
    }
    Ok(out)
}

fn e11_04(c: &Case) -> R {
    let cat = c.s("category")?.to_string();
    let area = c.pos("area")?;
    let n = c.n("storeys")?;
    if n < 1.0 { return Err("input storeys must be at least 1".into()); }
    let a = part_1_1::alpha_a(&cat, area, c.annex);
    let an = part_1_1::alpha_n(&cat, n as u8, c.annex);
    let psi0 = part_1_1::psi_0(&cat);
    Ok(vec![
        Outcome::info("alpha_a", "Area reduction α_A", "Flächenabminderung α_A", QuantityKind::Dimensionless, a).steps(format!("Category {cat}, A = {area:.1} m², ψ₀ = {psi0}: α_A = {a:.4}.")).basis(&["ndp.en1991.e11.alpha"]),
        Outcome::info("alpha_n", "Storey reduction α_n", "Geschossabminderung α_n", QuantityKind::Dimensionless, an).steps(format!("n = {n:.0}, ψ₀ = {psi0}: α_n = {an:.4}.")).basis(&["ndp.en1991.e11.alpha"]),
    ])
}

fn e11_05(c: &Case) -> R {
    let cat = c.s("category")?.to_ascii_uppercase();
    let line = match (c.annex, cat.chars().next()) {
        (_, Some('A' | 'B')) => 0.5,
        (_, Some('C')) if cat == "C5" => 3.0,
        (_, Some('C' | 'D')) => 1.0,
        (_, Some('E')) => 2.0,
        _ => return Err(format!("no barrier line load for category {cat}")),
    };
    let mut out = vec![Outcome::info("q_k", "Barrier line load q_k at 1.2 m", "Geländerlast q_k in 1,2 m Höhe", QuantityKind::Force, line * 1000.0).steps(format!("Category {cat}: q_k = {line:.1} kN/m.")).basis(&["ndp.en1991.e11.barrier"])];
    if let (Some(m), Some(v)) = (c.opt("mass"), c.opt("speed")) {
        let travel = c.pos("delta_c")? + c.pos("delta_b")?;
        let f = 0.5 * m * v * v / travel;
        out.push(Outcome::info("F_k", "Vehicle barrier force F = ½mv²/(δ_c+δ_b)", "Anpralllast Rückhaltesystem F = ½mv²/(δ_c+δ_b)", QuantityKind::Force, f).steps(format!("F = ½·{m:.0}·{v:.2}²/{travel:.3} = {:.1} kN.", f / 1000.0)).basis(&["ndp.en1991.e11.barrier"]));
    }
    Ok(out)
}

fn e11_06(c: &Case) -> R {
    let annex = c.annex;
    let (q, big_q) = part_1_1::imposed_row("H", annex).ok_or("category H missing")?;
    Ok(vec![
        Outcome::info("q_k", "Roof category H uniform load", "Dach Kategorie H Flächenlast", QuantityKind::Pressure, q * 1000.0).steps(format!("q_k = {q:.2} kN/m² (not combined with snow when zero).")).steps_de(format!("q_k = {q:.2} kN/m² (nicht mit Schnee kombiniert, wenn null).")).basis(&["ndp.en1991.e11.na-table"]),
        Outcome::info("Q_k", "Roof category H concentrated load", "Dach Kategorie H Einzellast", QuantityKind::Force, big_q * 1000.0).steps(format!("Q_k = {big_q:.2} kN on 50 × 50 mm.")).basis(&["ndp.en1991.e11.na-table"]),
    ])
}

fn e11_07(c: &Case) -> R {
    let class = c.n("class")? as u8;
    let (net, hoist) = match class { 1 => (21.0, 10.0), 2 => (31.0, 15.0), 3 => (44.0, 25.0), 4 => (60.0, 40.0), 5 => (90.0, 60.0), 6 => (110.0, 60.0), _ => return Err("forklift class must be 1 to 6".into()) };
    let phi = match c.s("tyres")? { "pneumatic" => 1.4, "solid" => 2.0, other => return Err(format!("unknown tyres {other}; admissible: pneumatic, solid")) };
    let q = phi * (net + hoist) * 1000.0;
    Ok(vec![Outcome::info("axle", "Dynamic axle load φ(net + hoist)", "Dynamische Achslast φ(Eigen- + Hubgewicht)", QuantityKind::Force, q).steps(format!("FL{class}: φ = {phi}, net {net} kN + hoist {hoist} kN → {:.1} kN.", q / 1000.0)).steps_de(format!("FL{class}: φ = {phi}, netto {net} kN + Hubkraft {hoist} kN → {:.1} kN.", q / 1000.0)).basis(&["ndp.en1991.e11.forklift"])])
}

fn e11_08(c: &Case) -> R {
    let (a, b) = (c.pos("a")?, c.pos("b")?);
    let cover = c.o("cover", 0.0);
    let slab_half = c.o("slab_thickness", 0.0) / 2.0;
    let (sa, sb) = (a + 2.0 * (cover + slab_half), b + 2.0 * (cover + slab_half));
    let p = c.pos("load")? / (sa * sb);
    Ok(vec![
        Outcome::info("footprint", "Spread footprint area at mid-plane", "Ausbreitungsfläche in Plattenmitte", QuantityKind::Area, sa * sb).steps(format!("{a:.3}×{b:.3} m spread at 45° through {:.3} m → {sa:.3}×{sb:.3} m.", cover + slab_half)).steps_de(format!("{a:.3}×{b:.3} m Ausbreitung unter 45° über {:.3} m → {sa:.3}×{sb:.3} m.", cover + slab_half)),
        Outcome::info("pressure", "Spread pressure", "Pressung nach Ausbreitung", QuantityKind::Pressure, p).steps(format!("p = Q/(a'·b') = {:.2} kN/m².", p / 1000.0)),
    ])
}

fn e11_09(c: &Case) -> R {
    let (q, a, b) = (c.pos("load")?, c.pos("a")?, c.pos("b")?);
    let p = q / (a * b);
    Ok(vec![Outcome::info("pressure", "Wheel contact pressure", "Radaufstandspressung", QuantityKind::Pressure, p).steps(format!("p = {:.1} kN / ({a:.3}·{b:.3} m²) = {:.1} kN/m².", q / 1000.0, p / 1000.0))])
}

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "E11.01", standard: "EN 1991-1-1", clause: "Annex A", en: "Self-weight of construction layers", de: "Eigenlast von Bauteilschichten", inputs: &["material", "thickness", "gamma", "material2", "thickness2", "material3", "thickness3"], eval: e11_01 },
    Entry { id: "E11.02", standard: "EN 1991-1-1", clause: "5.2", en: "Movable partitions and façades", de: "Bewegliche Trennwände und Fassaden", inputs: &["wall_load", "wall_gamma", "wall_thickness", "wall_height", "finish"], eval: e11_02 },
    Entry { id: "E11.03", standard: "EN 1991-1-1", clause: "6.3.1.2", en: "Imposed loads on buildings by category", de: "Nutzlasten Gebäude nach Kategorie", inputs: &["category", "depth"], eval: e11_03 },
    Entry { id: "E11.04", standard: "EN 1991-1-1", clause: "6.3.1.2", en: "Reduction factors α_A and α_n", de: "Abminderungsbeiwerte α_A und α_n", inputs: &["category", "area", "storeys"], eval: e11_04 },
    Entry { id: "E11.05", standard: "EN 1991-1-1", clause: "6.4", en: "Horizontal loads on barriers", de: "Horizontallasten auf Absturzsicherungen", inputs: &["category", "mass", "speed", "delta_c", "delta_b"], eval: e11_05 },
    Entry { id: "E11.06", standard: "EN 1991-1-1", clause: "6.3.4", en: "Roofs category H", de: "Dächer Kategorie H", inputs: &[], eval: e11_06 },
    Entry { id: "E11.07", standard: "EN 1991-1-1", clause: "6.3.3", en: "Forklift trucks", de: "Gabelstapler", inputs: &["class", "tyres"], eval: e11_07 },
    Entry { id: "E11.08", standard: "EN 1991-1-1", clause: "6.3.1", en: "Load spreading through layers", de: "Lastausbreitung durch Schichten", inputs: &["load", "a", "b", "cover", "slab_thickness"], eval: e11_08 },
    Entry { id: "E11.09", standard: "EN 1991-1-1", clause: "6.3.3", en: "Wheel contact pressure", de: "Radaufstandspressung", inputs: &["load", "a", "b"], eval: e11_09 },
];
