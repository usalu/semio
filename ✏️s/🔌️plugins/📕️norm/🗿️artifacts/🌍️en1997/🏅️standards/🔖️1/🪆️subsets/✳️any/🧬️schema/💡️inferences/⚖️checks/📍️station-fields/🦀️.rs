//! 📍️ EN 1997 check stations: the spatial distribution of a check's governing utilisation (contact pressure under a footing, earth pressure over the wall depth, mobilisation per slip slice, shaft resistance per pile layer). The peak of every distribution carries the check's utilisation, so attaching it never changes a verdict. See [`semio_s_plugin_norm_compliance::Station`].

use super::*;
use semio_s_plugin_norm_compliance::Station;

const FOOTING_COLUMNS: usize = 9;
const WALL_STATIONS: usize = 10;

fn pressure(value: f64) -> Quantity {
    Quantity::new(QuantityKind::Pressure, value)
}

fn scaled(utilization: f64, value: f64, peak: f64) -> f64 {
    if peak > 1e-12 && value.is_finite() { utilization * value / peak } else { utilization }
}

/// 🦶️ Base pressure of a footing as the field `soil-pressure` at `[x, y]` metres from the centre of the base (x along B toward the eccentricity): linear trapezoid for `e ≤ B/6`, triangle over the contact length `3(B/2 − e)` beyond.
pub fn contact_pressure(width: f64, length: f64, eccentricity: f64, v_d: f64, utilization: f64) -> Vec<Station> {
    let (b, l, e) = (width.max(1e-9), length.max(1e-9), eccentricity.abs());
    let contact = 3.0 * (b / 2.0 - e);
    let sigma = |x: f64| -> f64 {
        if e <= b / 6.0 {
            v_d / (b * l) * (1.0 + 12.0 * e * x / (b * b))
        } else if contact <= 0.0 {
            f64::INFINITY
        } else {
            let from_edge = b / 2.0 - x;
            if from_edge <= contact { 2.0 * v_d / (contact * l) * (1.0 - from_edge / contact) } else { 0.0 }
        }
    };
    let peak = sigma(b / 2.0);
    let cap = if utilization > 1e-12 && peak.is_finite() { peak / utilization } else { 0.0 };
    let mut out = Vec::with_capacity(FOOTING_COLUMNS * 3);
    for i in 0..FOOTING_COLUMNS {
        let x = -b / 2.0 + b * i as f64 / (FOOTING_COLUMNS - 1) as f64;
        for y in [-l / 2.0, 0.0, l / 2.0] {
            let value = sigma(x);
            let finite = if value.is_finite() { value } else { 0.0 };
            out.push(Station::new(&[x, y], scaled(utilization, value, peak), vec![pressure(finite), pressure(cap)]));
        }
    }
    out
}

/// 🧱️ Horizontal earth pressure `k (γ z + q) cos δ` over the retained height as the member field (s = z / H): sliding utilisation shared by the pressure ordinates, so the base ordinate carries the check.
pub fn earth_pressure_profile(k: f64, gamma: f64, surcharge: f64, height: f64, delta_cos: f64, utilization: f64) -> Vec<Station> {
    let at = |z: f64| k * (gamma * z + surcharge) * delta_cos;
    let peak = at(height);
    let cap = if utilization > 1e-12 { peak / utilization } else { 0.0 };
    (0..=WALL_STATIONS)
        .map(|i| {
            let s = i as f64 / WALL_STATIONS as f64;
            let value = at(s * height);
            Station::new(&[s], scaled(utilization, value, peak), vec![pressure(value), pressure(cap)])
        })
        .collect()
}

/// 🍰️ Local mobilisation `μ = F d_i / r_i` of the slices of the critical circle as the member field `slip-slice` (s across the slip width), scaled so the most mobilised slice carries the governing `1 / F`.
pub fn slip_slices(mobilisation: &[(f64, f64)], utilization: f64) -> Vec<Station> {
    let peak = mobilisation.iter().map(|m| m.1).fold(0.0, f64::max);
    mobilisation
        .iter()
        .map(|&(s, mu)| Station::new(&[s.clamp(0.0, 1.0)], scaled(utilization, mu, peak), vec![Quantity::new(QuantityKind::Dimensionless, mu)]))
        .collect()
}

pub fn effective_stress(layers: &[SoilLayer], gwl: f64, depth: f64) -> f64 {
    layers
        .iter()
        .map(|l| {
            let (top, bottom) = (l.depth_top, l.depth_bottom.min(depth));
            if bottom <= top {
                return 0.0;
            }
            let dry = (bottom.min(gwl) - top).clamp(0.0, bottom - top);
            l.gamma * dry + l.gamma_prime * (bottom - top - dry)
        })
        .sum()
}

/// 🪵️ Shaft resistance of a pile per soil layer as the member field `pile-shaft` (s = layer mid-depth / pile length): the design shaft resistance `r_shaft` is shared by layer thickness times unit friction (`c_u` of cohesive layers, `σ'_v tan φ'` of granular ones), so the sum equals the check's shaft term. Stations hold R_s,i and the axial force N(z) at the layer mid-depth under uniform mobilisation; the utilisation is the compression utilisation at every layer.
pub fn pile_shaft(layers: &[SoilLayer], gwl: f64, length: f64, r_shaft: f64, n_ed: f64, utilization: f64) -> Vec<Station> {
    let length = length.max(1e-9);
    let cuts: Vec<(f64, f64, f64)> = layers
        .iter()
        .filter_map(|l| {
            let (top, bottom) = (l.depth_top.max(0.0), l.depth_bottom.min(length));
            if bottom <= top {
                return None;
            }
            let mid = 0.5 * (top + bottom);
            let unit = if part_1::is_cohesive_layer(l) { l.cohesion_undrained } else { effective_stress(layers, gwl, mid) * l.phi_prime_deg.to_radians().tan() };
            Some((mid, bottom - top, unit.max(0.0)))
        })
        .collect();
    let weights: Vec<f64> = cuts.iter().map(|c| c.1 * c.2).collect();
    let total: f64 = weights.iter().sum();
    let thickness: f64 = cuts.iter().map(|c| c.1).sum();
    let mut above = 0.0;
    cuts.iter()
        .zip(&weights)
        .map(|(c, w)| {
            let share = if total > 1e-12 { w / total } else if thickness > 1e-12 { c.1 / thickness } else { 0.0 };
            let r_i = r_shaft * share;
            let n_z = n_ed - utilization * (above + 0.5 * r_i);
            above += r_i;
            Station::new(&[(c.0 / length).clamp(0.0, 1.0)], utilization, vec![Quantity::new(QuantityKind::Force, r_i), Quantity::new(QuantityKind::Force, n_z.max(0.0))])
        })
        .collect()
}
