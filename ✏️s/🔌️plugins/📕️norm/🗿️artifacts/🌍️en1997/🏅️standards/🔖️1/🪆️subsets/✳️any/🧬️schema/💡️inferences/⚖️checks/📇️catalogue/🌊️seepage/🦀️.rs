//! 🌊️ Seepage flow net of a single sheet pile by finite differences and the hydraulic failure checks that read it: Terzaghi heave prism, surface exit gradient and piping, discharge; catalogue entry HYD-03 (EN 1997-1 §10.3-10.4, DIN 1054 GZ 1A, Terzaghi and Peck, Harr). See [`super`].

use super::*;
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_CELLS: usize = 160;
const TOLERANCE: f64 = 1.0e-12;
const MAX_SWEEPS: usize = 20_000;

type EdgeId = (usize, usize, u8);

/// 🕸️ Steady head field h(x, z) of a homogeneous isotropic layer of thickness `t` cut by a vertical impermeable wall of embedment `d`: cell-centred finite volumes of size d/m, head ΔH on the retained surface, 0 on the excavation surface, no flow through the wall, bottom and the lateral ends; solved by successive over-relaxation until the largest update is below 1e-12 ΔH.
#[derive(Clone, Debug)]
pub struct FlowNet {
    pub nx: usize,
    pub nz: usize,
    pub spacing: f64,
    pub wall_rows: usize,
    pub delta_h: f64,
    pub sweeps: usize,
    head: Vec<f64>,
}

impl FlowNet {
    /// 🧮️ Solves the head field; `per_wall` cells span the embedment (4 … 40), the layer is truncated at 160 cells in depth and half-width.
    pub fn solve(delta_h: f64, d: f64, t: f64, per_wall: usize) -> Result<Self, String> {
        gate(delta_h > 0.0 && d > 0.0 && t > d && delta_h.is_finite() && t.is_finite(), "flow net needs ΔH > 0 and a layer thicker than the embedment")?;
        let wall_rows = per_wall.clamp(4, 40);
        let spacing = d / wall_rows as f64;
        let nz = ((t / spacing).round() as usize).clamp(wall_rows + 1, MAX_CELLS);
        let half = (((2.0 * t.max(d)) / spacing).round() as usize + wall_rows).clamp(wall_rows + 2, MAX_CELLS);
        let mut net = Self { nx: 2 * half, nz, spacing, wall_rows, delta_h, sweeps: 0, head: vec![0.5 * delta_h; 2 * half * nz] };
        net.relax();
        Ok(net)
    }

    fn relax(&mut self) {
        let (nx, nz, c, m) = (self.nx, self.nz, self.nx / 2, self.wall_rows);
        let omega = 2.0 / (1.0 + (std::f64::consts::PI / (nx.max(nz) as f64 + 1.0)).sin());
        for sweep in 1..=MAX_SWEEPS {
            let mut worst = 0.0_f64;
            for j in 0..nz {
                for i in 0..nx {
                    let at = j * nx + i;
                    let (mut sum, mut weight) = (0.0, 0.0);
                    if i > 0 && !(i == c && j < m) {
                        sum += self.head[at - 1];
                        weight += 1.0;
                    }
                    if i + 1 < nx && !(i + 1 == c && j < m) {
                        sum += self.head[at + 1];
                        weight += 1.0;
                    }
                    if j == 0 {
                        sum += 2.0 * if i < c { self.delta_h } else { 0.0 };
                        weight += 2.0;
                    } else {
                        sum += self.head[at - nx];
                        weight += 1.0;
                    }
                    if j + 1 < nz {
                        sum += self.head[at + nx];
                        weight += 1.0;
                    }
                    let step = omega * (sum / weight - self.head[at]);
                    self.head[at] += step;
                    worst = worst.max(step.abs());
                }
            }
            self.sweeps = sweep;
            if worst < TOLERANCE * self.delta_h {
                break;
            }
        }
    }

    /// 📍️ First column of the excavation side; the wall stands on the face between column `centre() − 1` and `centre()`.
    pub fn centre(&self) -> usize {
        self.nx / 2
    }

    /// 💧 Head of the cell in column `i` and row `j` (row 0 is the surface).
    pub fn head_at(&self, i: usize, j: usize) -> f64 {
        self.head[j * self.nx + i]
    }

    /// 📐️ Vertical gradient on the excavation surface next to the wall, i = h/(Δz/2) of the first cell.
    pub fn exit_gradient(&self) -> f64 {
        2.0 * self.head_at(self.centre(), 0) / self.spacing
    }

    /// 🧭️ Hydraulic gradient |∇h| of the cell in column `i` and row `j`: central differences of the cell heads that never cross the impermeable wall, one-sided at the layer edges, and on the surface row the vertical difference to the boundary head (so `gradient_at(centre(), 0)` equals [`Self::exit_gradient`] in its vertical part).
    pub fn gradient_at(&self, i: usize, j: usize) -> f64 {
        let [horizontal, vertical] = self.head_slope_at(i, j);
        vertical.hypot(horizontal)
    }

    /// 🏹️ Seepage direction of the cell: the negative head slope `−∇h = [−∂h/∂x, −∂h/∂depth]` with x towards the excavation side and depth downwards (Darcy: the water flows from the retained side, round the wall toe and up on the excavation side), same stencil as [`Self::gradient_at`].
    pub fn flow_direction_at(&self, i: usize, j: usize) -> [f64; 2] {
        let [horizontal, vertical] = self.head_slope_at(i, j);
        [-horizontal, -vertical]
    }

    fn head_slope_at(&self, i: usize, j: usize) -> [f64; 2] {
        let (c, m, h) = (self.centre(), self.wall_rows, self.spacing);
        let vertical = if j == 0 {
            (self.head_at(i, 0) - if i < c { self.delta_h } else { 0.0 }) / (0.5 * h)
        } else if j + 1 < self.nz {
            (self.head_at(i, j + 1) - self.head_at(i, j - 1)) / (2.0 * h)
        } else {
            (self.head_at(i, j) - self.head_at(i, j - 1)) / h
        };
        let left = i > 0 && !(i == c && j < m);
        let right = i + 1 < self.nx && !(i + 1 == c && j < m);
        let horizontal = match (left, right) {
            (true, true) => (self.head_at(i + 1, j) - self.head_at(i - 1, j)) / (2.0 * h),
            (true, false) => (self.head_at(i, j) - self.head_at(i - 1, j)) / h,
            (false, true) => (self.head_at(i + 1, j) - self.head_at(i, j)) / h,
            (false, false) => 0.0,
        };
        [horizontal, vertical]
    }

    /// 🧭️ Largest hydraulic gradient of the net, the governing internal-erosion gradient at the wall toe.
    pub fn max_gradient(&self) -> f64 {
        (0..self.nz).flat_map(|j| (0..self.nx).map(move |i| (i, j))).map(|(i, j)| self.gradient_at(i, j)).fold(0.0, f64::max)
    }

    /// 💧 Discharge per metre of wall [m³/(s·m)] through the excavation surface, q = k·Σ 2·h_i.
    pub fn discharge(&self, k: f64) -> f64 {
        2.0 * k * (self.centre()..self.nx).map(|i| self.head_at(i, 0)).sum::<f64>()
    }

    /// 💧 Discharge per metre of wall [m³/(s·m)] entering through the retained surface; equal to [`Self::discharge`] by continuity.
    pub fn discharge_retained(&self, k: f64) -> f64 {
        2.0 * k * (0..self.centre()).map(|i| self.delta_h - self.head_at(i, 0)).sum::<f64>()
    }

    /// 🔺️ Mean excess head on the base of the Terzaghi prism, the strip of width d/2 next to the wall at the depth of the wall toe.
    pub fn prism_head(&self) -> f64 {
        let (c, m) = (self.centre(), self.wall_rows);
        let width = (m / 2).max(1);
        (c..c + width).map(|i| 0.5 * (self.head_at(i, m - 1) + self.head_at(i, m))).sum::<f64>() / width as f64
    }

    /// 🌀️ Stream function ψ on the (nx + 1) × (nz + 1) cell corners, ψ(0, 0) = 0 and ψ(nx, 0) = 0; the contour ψ = Q is the wall, the largest value is the total discharge per unit permeability.
    pub fn stream_function(&self) -> Vec<f64> {
        let (nx, nz, c, m) = (self.nx, self.nz, self.centre(), self.wall_rows);
        let w = nx + 1;
        let mut psi = vec![0.0; w * (nz + 1)];
        for i in 0..nx {
            let boundary = if i < c { self.delta_h } else { 0.0 };
            psi[i + 1] = psi[i] + 2.0 * (boundary - self.head_at(i, 0));
        }
        for j in 0..nz {
            for i in 0..=nx {
                let flux = if i == 0 || i == nx || (i == c && j < m) { 0.0 } else { self.head_at(i - 1, j) - self.head_at(i, j) };
                psi[(j + 1) * w + i] = psi[j * w + i] - flux;
            }
        }
        psi
    }

    /// 📈️ `count` equipotential lines at heads ΔH·k/(count + 1) as polylines of (x, depth) in metres from the wall axis (x > 0 on the excavation side, depth downwards); squares across the wall are skipped.
    pub fn equipotentials(&self, count: usize) -> Vec<Vec<[f64; 2]>> {
        let (c, m, h) = (self.centre(), self.wall_rows, self.spacing);
        (1..=count)
            .flat_map(|k| contour(&self.head, self.nx, self.nz, self.delta_h * k as f64 / (count as f64 + 1.0), &|i, j| i + 1 == c && j < m))
            .map(|line| line.into_iter().map(|[i, j]| [(i + 0.5 - c as f64) * h, (j + 0.5) * h]).collect())
            .collect()
    }

    /// 📉️ `count` streamlines at ψ = Q·k/(count + 1) as polylines of (x, depth) in metres from the wall axis.
    pub fn streamlines(&self, count: usize) -> Vec<Vec<[f64; 2]>> {
        let (c, h) = (self.centre(), self.spacing);
        let psi = self.stream_function();
        let total = psi.iter().copied().fold(0.0, f64::max);
        (1..=count)
            .flat_map(|k| contour(&psi, self.nx + 1, self.nz + 1, total * k as f64 / (count as f64 + 1.0), &|_, _| false))
            .map(|line| line.into_iter().map(|[i, j]| [(i - c as f64) * h, j * h]).collect())
            .collect()
    }
}

fn lerp(a: f64, b: f64, level: f64) -> f64 {
    (level - a) / (b - a)
}

fn contour(values: &[f64], width: usize, height: usize, level: f64, skip: &dyn Fn(usize, usize) -> bool) -> Vec<Vec<[f64; 2]>> {
    let at = |i: usize, j: usize| values[j * width + i];
    let mut points: BTreeMap<EdgeId, [f64; 2]> = BTreeMap::new();
    let mut links: BTreeMap<EdgeId, Vec<EdgeId>> = BTreeMap::new();
    for j in 0..height.saturating_sub(1) {
        for i in 0..width.saturating_sub(1) {
            if skip(i, j) {
                continue;
            }
            let (fi, fj) = (i as f64, j as f64);
            let edges: [(EdgeId, f64, f64, [f64; 2], [f64; 2]); 4] = [
                ((i, j, 0), at(i, j), at(i + 1, j), [fi, fj], [fi + 1.0, fj]),
                ((i + 1, j, 1), at(i + 1, j), at(i + 1, j + 1), [fi + 1.0, fj], [fi + 1.0, fj + 1.0]),
                ((i, j + 1, 0), at(i, j + 1), at(i + 1, j + 1), [fi, fj + 1.0], [fi + 1.0, fj + 1.0]),
                ((i, j, 1), at(i, j), at(i, j + 1), [fi, fj], [fi, fj + 1.0]),
            ];
            let hits: Vec<(EdgeId, [f64; 2])> = edges
                .iter()
                .filter(|(_, a, b, _, _)| (a - level) * (b - level) < 0.0)
                .map(|(id, a, b, p, q)| {
                    let t = lerp(*a, *b, level);
                    (*id, [p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1])])
                })
                .collect();
            for pair in hits.chunks_exact(2) {
                points.insert(pair[0].0, pair[0].1);
                points.insert(pair[1].0, pair[1].1);
                links.entry(pair[0].0).or_default().push(pair[1].0);
                links.entry(pair[1].0).or_default().push(pair[0].0);
            }
        }
    }
    let mut starts: Vec<EdgeId> = links.keys().copied().collect();
    starts.sort_by_key(|id| (links[id].len(), *id));
    let mut seen: BTreeSet<EdgeId> = BTreeSet::new();
    let mut lines = Vec::new();
    for start in starts {
        if seen.contains(&start) {
            continue;
        }
        let (mut line, mut here) = (Vec::new(), start);
        loop {
            seen.insert(here);
            line.push(points[&here]);
            match links[&here].iter().copied().find(|next| !seen.contains(next)) {
                Some(next) => here = next,
                None => break,
            }
        }
        if line.len() > 1 {
            lines.push(line);
        }
    }
    lines
}

fn hyd_03(c: &Case) -> R {
    let (dh, d, t, gp) = (c.positive("delta_h")?, c.positive("d_embed")?, c.positive("t_layer")?, c.positive("gamma_prime")?);
    let net = FlowNet::solve(dh, d, t, c.o("cells", 12.0).round().max(4.0) as usize)?;
    let (u, gw) = (c.upl(), c.gamma_w());
    let (i_exit, h_m) = (net.exit_gradient(), net.prism_head());
    let q = net.discharge(c.o("k", 1.0));
    let mut out = vec![
        Outcome::force("heave_prism", "Terzaghi prism: seepage force γH·γw·h_m·d/2 vs submerged weight γH,stb·γ′·d²/2", "Terzaghi-Prisma: Strömungskraft γH·γw·h_m·d/2 gegen Auftriebsgewicht γH,stb·γ′·d²/2", u.gamma_hyd * gw * h_m * d / 2.0, u.gamma_hyd_stb * gp * d * d / 2.0),
        Outcome::ratio("piping", "Surface exit gradient: γH·i·γw vs γH,stb·γ′", "Austrittsgefälle an der Oberfläche: γH·i·γw gegen γH,stb·γ′", u.gamma_hyd * i_exit * gw, u.gamma_hyd_stb * gp),
        Outcome::number("i_exit", "Exit gradient i from the flow net", "Austrittsgefälle i aus dem Strömungsnetz", i_exit),
        Outcome::number("i_crit", "Critical gradient γ′/γw", "Kritisches Gefälle γ′/γw", gp / gw),
        Outcome::number("safety", "Safety i_crit/i", "Sicherheit i_crit/i", gp / gw / i_exit),
        Outcome::number("i_max", "Largest gradient in the net", "Größtes Gefälle im Strömungsnetz", net.max_gradient()),
        Outcome::number("internal_erosion", "Internal erosion index i_max/i_crit (10.4, information: the largest gradient sits at the singular wall tip and grows with the mesh)", "Index der inneren Erosion i_max/i_crit (10.4, Information: das größte Gefälle liegt an der singulären Wandspitze und wächst mit dem Netz)", net.max_gradient() / (gp / gw)),
        Outcome::metre("d_required", "Embedment required against heave of the prism, γH·γw·h_m/(γH,stb·γ′)", "Einbindetiefe gegen Aufbruch des Prismas, γH·γw·h_m/(γH,stb·γ′)", u.gamma_hyd * gw * h_m / (u.gamma_hyd_stb * gp)),
        Outcome::metre("h_prism", "Mean excess head h_m on the prism base", "Mittlere Überdruckhöhe h_m an der Prismenbasis", h_m),
        Outcome::number("h_prism_ratio", "Prism head ratio h_m/ΔH", "Prismenhöhenverhältnis h_m/ΔH", h_m / dh),
        Outcome::number("q", "Discharge per metre of wall q (m³/s per m)", "Sickerwassermenge je Meter Wand q (m³/s je m)", q),
        Outcome::number("continuity", "Continuity error |q_in − q_out|/q", "Kontinuitätsfehler |q_zu − q_ab|/q", (net.discharge_retained(c.o("k", 1.0)) - q).abs() / q),
        Outcome::number("cells", "Number of cells", "Zellenanzahl", (net.nx * net.nz) as f64),
        Outcome::number("sweeps", "Relaxation sweeps", "Relaxationsdurchläufe", net.sweeps as f64),
    ];
    if let Some(k) = c.opt("k") {
        out.push(Outcome::number("v_exit", "Darcy exit velocity v = k·i (m/s)", "Darcy-Austrittsgeschwindigkeit v = k·i (m/s)", k * i_exit));
    }
    Ok(out)
}

pub const ENTRIES: &[Entry] = &[Entry {
    id: "HYD-03",
    part: "EN 1997-1",
    clause: "10.3",
    en: "Seepage flow net: heave prism, exit gradient and discharge",
    de: "Strömungsnetz: Terzaghi-Prisma, Austrittsgefälle und Sickerwassermenge",
    inputs: &["delta_h", "d_embed", "t_layer", "gamma_prime", "k", "cells", "gamma_w", "situation"],
    eval: hyd_03,
}];
