//! 🪶️ Thin-walled open cross-sections from centreline walls: centroid and inertia, St Venant torsion (Σ l t³ / 3), sectorial warping constant and shear centre by Trefftz' orthogonality, shear areas from the shear-flow energy. The consistency test against the finite-element solver of `🧊️fem` bounds the thin-wall error. Mirrors `thin_proto.py` of the ticket NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS.

use super::mesh::P2;
use crate::{MechanicsError, MechanicsResult, ThinWall};

const G3: [(f64, f64); 3] = [(0.112_701_665_379_258_31, 5.0 / 18.0), (0.5, 8.0 / 18.0), (0.887_298_334_620_741_7, 5.0 / 18.0)];

/// 🪶️ Constants of a thin-walled open section in the input frame; `iy = ∫z²`, `iz = ∫y²`, `iyz = ∫yz` about the centroid.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Thin {
    pub area: f64,
    pub cy: f64,
    pub cz: f64,
    pub iy: f64,
    pub iz: f64,
    pub iyz: f64,
    pub j: f64,
    pub iw: f64,
    pub sc_y: f64,
    pub sc_z: f64,
    pub a_vy: f64,
    pub a_vz: f64,
}

fn invalid<T>(en: &str, de: &str) -> MechanicsResult<T> {
    Err(MechanicsError::invalid(en, de))
}

fn length(nodes: &[P2], a: usize, b: usize) -> f64 {
    (nodes[b][0] - nodes[a][0]).hypot(nodes[b][1] - nodes[a][1])
}

fn lerp(p: P2, q: P2, u: f64) -> P2 {
    [p[0] + u * (q[0] - p[0]), p[1] + u * (q[1] - p[1])]
}

fn tree(walls: &[ThinWall]) -> MechanicsResult<(Vec<P2>, Vec<(usize, usize, f64)>)> {
    if walls.is_empty() {
        return invalid("a thin-walled section needs at least one wall", "ein dünnwandiger Querschnitt braucht mindestens eine Wand");
    }
    let scale = walls.iter().fold(1e-300_f64, |m, w| m.max((w.y2 - w.y1).hypot(w.z2 - w.z1)));
    let tol = 1e-9 * scale;
    let mut nodes: Vec<P2> = Vec::new();
    let node = |p: P2, nodes: &mut Vec<P2>| -> usize {
        match nodes.iter().position(|q| (p[0] - q[0]).abs() < tol && (p[1] - q[1]).abs() < tol) {
            Some(i) => i,
            None => {
                nodes.push(p);
                nodes.len() - 1
            }
        }
    };
    let mut raw: Vec<(usize, usize, f64)> = Vec::new();
    for w in walls {
        if w.thickness <= 0.0 || ![w.y1, w.z1, w.y2, w.z2, w.thickness].iter().all(|v| v.is_finite()) {
            return invalid("wall thickness and coordinates must be finite and the thickness positive", "Wanddicke und Koordinaten müssen endlich und die Dicke positiv sein");
        }
        let a = node([w.y1, w.z1], &mut nodes);
        let b = node([w.y2, w.z2], &mut nodes);
        if a == b {
            return invalid("a wall has zero length", "eine Wand hat die Länge null");
        }
        raw.push((a, b, w.thickness));
    }
    let mut segments: Vec<(usize, usize, f64)> = Vec::new();
    for &(a, b, t) in &raw {
        let d = length(&nodes, a, b);
        let mut cuts: Vec<(f64, usize)> = Vec::new();
        for (k, p) in nodes.iter().enumerate() {
            if k == a || k == b {
                continue;
            }
            let u = ((p[0] - nodes[a][0]) * (nodes[b][0] - nodes[a][0]) + (p[1] - nodes[a][1]) * (nodes[b][1] - nodes[a][1])) / (d * d);
            let off = ((nodes[b][0] - nodes[a][0]) * (p[1] - nodes[a][1]) - (nodes[b][1] - nodes[a][1]) * (p[0] - nodes[a][0])).abs() / d;
            if off < tol && u > 0.0 && u < 1.0 {
                cuts.push((u, k));
            }
        }
        cuts.sort_by(|p, q| p.0.total_cmp(&q.0));
        let mut chain = vec![a];
        chain.extend(cuts.iter().map(|c| c.1));
        chain.push(b);
        segments.extend(chain.windows(2).map(|pair| (pair[0], pair[1], t)));
    }
    if segments.len() + 1 != nodes.len() {
        return invalid("the thin-walled solver needs an open, connected wall tree; use the finite-element solver for closed or disconnected sections", "der Dünnwandlöser braucht einen offenen, zusammenhängenden Wandbaum; für geschlossene oder getrennte Querschnitte den Finite-Elemente-Löser verwenden");
    }
    Ok((nodes, segments))
}

/// 🪶️ Constants of the open wall tree; walls meeting at a point or ending on another wall are joined, closed cells and disconnected walls are refused.
pub fn analyse(walls: &[ThinWall]) -> MechanicsResult<Thin> {
    let (nodes, segments) = tree(walls)?;
    let nn = nodes.len();
    let area: f64 = segments.iter().map(|&(a, b, t)| length(&nodes, a, b) * t).sum();
    let cy = segments.iter().map(|&(a, b, t)| length(&nodes, a, b) * t * (nodes[a][0] + nodes[b][0]) / 2.0).sum::<f64>() / area;
    let cz = segments.iter().map(|&(a, b, t)| length(&nodes, a, b) * t * (nodes[a][1] + nodes[b][1]) / 2.0).sum::<f64>() / area;
    let p: Vec<P2> = nodes.iter().map(|q| [q[0] - cy, q[1] - cz]).collect();
    let (mut syy, mut szz, mut syz) = (0.0, 0.0, 0.0);
    for &(a, b, t) in &segments {
        let l = length(&p, a, b);
        let (c, s) = ((p[b][0] - p[a][0]) / l, (p[b][1] - p[a][1]) / l);
        for (u, w) in G3 {
            let q = lerp(p[a], p[b], u);
            syy += w * l * t * q[0] * q[0];
            szz += w * l * t * q[1] * q[1];
            syz += w * l * t * q[0] * q[1];
        }
        let normal = l * t * t * t / 12.0;
        syy += normal * s * s;
        szz += normal * c * c;
        syz -= normal * c * s;
    }
    let det = syy * szz - syz * syz;
    let mut adjacent: Vec<Vec<(usize, usize)>> = vec![Vec::new(); nn];
    for (si, &(a, b, _)) in segments.iter().enumerate() {
        adjacent[a].push((si, b));
        adjacent[b].push((si, a));
    }
    let mut order = vec![0_usize];
    let mut seen = vec![false; nn];
    seen[0] = true;
    let mut omega = vec![0.0; nn];
    let mut walk: Vec<(usize, usize, usize)> = Vec::new();
    let mut k = 0;
    while k < order.len() {
        let v = order[k];
        k += 1;
        for &(si, w) in &adjacent[v] {
            if !seen[w] {
                seen[w] = true;
                omega[w] = omega[v] + p[v][0] * p[w][1] - p[v][1] * p[w][0];
                order.push(w);
                walk.push((si, v, w));
            }
        }
    }
    if order.len() != nn {
        return invalid("the walls are not connected", "die Wände sind nicht verbunden");
    }
    let (mut q_om, mut i_oy, mut i_oz) = (0.0, 0.0, 0.0);
    for &(si, v, w) in &walk {
        let (l, t) = (length(&p, v, w), segments[si].2);
        for (u, wt) in G3 {
            let q = lerp(p[v], p[w], u);
            let o = omega[v] + u * (omega[w] - omega[v]);
            q_om += wt * l * t * o;
            i_oy += wt * l * t * o * q[0];
            i_oz += wt * l * t * o * q[1];
        }
    }
    let mean = q_om / area;
    let a_s = (syy * i_oz - syz * i_oy) / det;
    let b_s = (syz * i_oz - szz * i_oy) / det;
    let mut iw = 0.0;
    for &(si, v, w) in &walk {
        let (l, t) = (length(&p, v, w), segments[si].2);
        for (u, wt) in G3 {
            let q = lerp(p[v], p[w], u);
            let o = omega[v] + u * (omega[w] - omega[v]) - mean + b_s * q[0] - a_s * q[1];
            iw += wt * l * t * o * o;
        }
    }
    let j = segments.iter().map(|&(a, b, t)| length(&nodes, a, b) * t * t * t / 3.0).sum::<f64>();
    let (mut sub_y, mut sub_z) = (vec![0.0; nn], vec![0.0; nn]);
    for &(si, v, w) in walk.iter().rev() {
        let (l, t) = (length(&p, v, w), segments[si].2);
        sub_y[v] += sub_y[w] + t * l * (p[v][0] + p[w][0]) / 2.0;
        sub_z[v] += sub_z[w] + t * l * (p[v][1] + p[w][1]) / 2.0;
    }
    let (mut energy_z, mut energy_y) = (0.0, 0.0);
    for &(si, v, w) in &walk {
        let (l, t) = (length(&p, v, w), segments[si].2);
        for (u, wt) in G3 {
            let my = sub_y[w] + t * l * (p[v][0] * (1.0 - u) + (p[w][0] - p[v][0]) * (1.0 - u * u) / 2.0);
            let mz = sub_z[w] + t * l * (p[v][1] * (1.0 - u) + (p[w][1] - p[v][1]) * (1.0 - u * u) / 2.0);
            let qz = (syy * mz - syz * my) / det;
            let qy = (szz * my - syz * mz) / det;
            energy_z += wt * l * qz * qz / t;
            energy_y += wt * l * qy * qy / t;
        }
    }
    Ok(Thin { area, cy, cz, iy: szz, iz: syy, iyz: syz, j, iw, sc_y: a_s + cy, sc_z: b_s + cz, a_vy: 1.0 / energy_y, a_vz: 1.0 / energy_z })
}
