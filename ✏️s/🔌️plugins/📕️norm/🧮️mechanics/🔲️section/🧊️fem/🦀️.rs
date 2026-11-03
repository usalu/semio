//! 🧊️ Saint-Venant torsion, warping and shear of a cross-section on a six-node triangle mesh (Pilkey, Analysis and Design of Elastic Beams): one pinned Neumann solve each for the warping function and the two shear functions with a shared skyline factorisation. Mirrors the Python prototype `proto.py` of the ticket NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS; reference: sectionproperties.

use super::mesh::{Mesh, P2};
use super::Geometry;
use crate::linalg::Skyline;
use crate::statics::rcm;
use crate::{Control, MechanicsResult};
use std::collections::BTreeMap;

const GAUSS: [([f64; 3], f64); 6] = [
    ([0.445948490915965, 0.445948490915965, 0.108103018168070], 0.223381589678011),
    ([0.445948490915965, 0.108103018168070, 0.445948490915965], 0.223381589678011),
    ([0.108103018168070, 0.445948490915965, 0.445948490915965], 0.223381589678011),
    ([0.091576213509771, 0.091576213509771, 0.816847572980459], 0.109951743655322),
    ([0.091576213509771, 0.816847572980459, 0.091576213509771], 0.109951743655322),
    ([0.816847572980459, 0.091576213509771, 0.091576213509771], 0.109951743655322),
];

/// 🧊️ Warping-based constants in the input frame: torsion constant, warping constant about the shear centre, shear centre and shear areas along the input axes.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Solved {
    pub j: f64,
    pub gamma: f64,
    pub sc_x: f64,
    pub sc_y: f64,
    pub a_sx: f64,
    pub a_sy: f64,
    pub nodes: usize,
}

#[derive(Clone, Copy, Default)]
struct Gp {
    w: f64,
    n: [f64; 6],
    bx: [f64; 6],
    by: [f64; 6],
    x: f64,
    y: f64,
}

fn shape(l: [f64; 3]) -> ([f64; 6], [[f64; 3]; 6]) {
    let n = [l[0] * (2.0 * l[0] - 1.0), l[1] * (2.0 * l[1] - 1.0), l[2] * (2.0 * l[2] - 1.0), 4.0 * l[0] * l[1], 4.0 * l[1] * l[2], 4.0 * l[2] * l[0]];
    let dn = [
        [4.0 * l[0] - 1.0, 0.0, 0.0],
        [0.0, 4.0 * l[1] - 1.0, 0.0],
        [0.0, 0.0, 4.0 * l[2] - 1.0],
        [4.0 * l[1], 4.0 * l[0], 0.0],
        [0.0, 4.0 * l[2], 4.0 * l[1]],
        [4.0 * l[2], 0.0, 4.0 * l[0]],
    ];
    (n, dn)
}

fn gauss_data(coords: &[P2], element: &[usize; 6]) -> [Gp; 6] {
    let c = [coords[element[0]], coords[element[1]], coords[element[2]]];
    let d = (c[1][0] - c[0][0]) * (c[2][1] - c[0][1]) - (c[2][0] - c[0][0]) * (c[1][1] - c[0][1]);
    let b = [c[1][1] - c[2][1], c[2][1] - c[0][1], c[0][1] - c[1][1]];
    let a = [c[2][0] - c[1][0], c[0][0] - c[2][0], c[1][0] - c[0][0]];
    let mut out = [Gp::default(); 6];
    for (g, (l, w)) in GAUSS.iter().enumerate() {
        let (n, dn) = shape(*l);
        let mut gp = Gp { w: w * d / 2.0, n, ..Gp::default() };
        gp.x = l[0] * c[0][0] + l[1] * c[1][0] + l[2] * c[2][0];
        gp.y = l[0] * c[0][1] + l[1] * c[1][1] + l[2] * c[2][1];
        for i in 0..6 {
            gp.bx[i] = (dn[i][0] * b[0] + dn[i][1] * b[1] + dn[i][2] * b[2]) / d;
            gp.by[i] = (dn[i][0] * a[0] + dn[i][1] * a[1] + dn[i][2] * a[2]) / d;
        }
        out[g] = gp;
    }
    out
}

/// 🧮️ Solve the torsion, warping and shear problems of the mesh for the geometry `g` (area, centroid, second moments about the centroid) and Poisson's ratio `nu`.
pub fn solve(mesh: &Mesh, g: &Geometry, nu: f64, control: &Control<'_>) -> MechanicsResult<Solved> {
    let mut coords: Vec<P2> = mesh.points.iter().map(|p| [p[0] - g.cx, p[1] - g.cy]).collect();
    let mut mids: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let mut elements: Vec<[usize; 6]> = Vec::with_capacity(mesh.triangles.len());
    for t in &mesh.triangles {
        let mut e = [t[0], t[1], t[2], 0, 0, 0];
        for k in 0..3 {
            let (a, b) = (t[k], t[(k + 1) % 3]);
            let id = *mids.entry((a.min(b), a.max(b))).or_insert_with(|| {
                let m = [(coords[a][0] + coords[b][0]) / 2.0, (coords[a][1] + coords[b][1]) / 2.0];
                coords.push(m);
                coords.len() - 1
            });
            e[3 + k] = id;
        }
        elements.push(e);
    }
    let n = coords.len();
    let mut ends: Vec<(usize, usize)> = Vec::with_capacity(elements.len() * 15);
    for e in &elements {
        for i in 0..6 {
            for j in (i + 1)..6 {
                ends.push((e[i], e[j]));
            }
        }
    }
    let order = rcm(n, &ends);
    drop(ends);
    let mut pos = vec![0_usize; n];
    for (p, &v) in order.iter().enumerate() {
        pos[v] = p;
    }
    let mut first: Vec<usize> = (0..n).collect();
    for e in &elements {
        let lo = e.iter().map(|&v| pos[v]).min().unwrap_or(0);
        for &v in e {
            if lo < first[pos[v]] {
                first[pos[v]] = lo;
            }
        }
    }
    let pin = pos[0];
    let (ixx, iyy, ixy) = (g.ixx, g.iyy, g.ixy);
    let mut sky = Skyline::new(first);
    let (mut f, mut fpsi, mut fphi) = (vec![0.0; n], vec![0.0; n], vec![0.0; n]);
    let count = elements.len();
    for (ei, e) in elements.iter().enumerate() {
        if ei % 256 == 0 {
            control.check()?;
            control.report("assemble", ei as f64 / count as f64);
        }
        let mut ke = [[0.0_f64; 6]; 6];
        let (mut fe, mut fp, mut fh) = ([0.0_f64; 6], [0.0_f64; 6], [0.0_f64; 6]);
        for gp in gauss_data(&coords, e).iter() {
            let (x, y) = (gp.x, gp.y);
            let (r, q) = (x * x - y * y, 2.0 * x * y);
            let (d1, d2) = (ixx * r - ixy * q, ixy * r + ixx * q);
            let (h1, h2) = (-ixy * r + iyy * q, -iyy * r - ixy * q);
            for i in 0..6 {
                fe[i] += gp.w * (gp.bx[i] * y - gp.by[i] * x);
                fp[i] += gp.w * (nu / 2.0 * (gp.bx[i] * d1 + gp.by[i] * d2) + 2.0 * (1.0 + nu) * gp.n[i] * (ixx * x - ixy * y));
                fh[i] += gp.w * (nu / 2.0 * (gp.bx[i] * h1 + gp.by[i] * h2) + 2.0 * (1.0 + nu) * gp.n[i] * (iyy * y - ixy * x));
                for j in 0..6 {
                    ke[i][j] += gp.w * (gp.bx[i] * gp.bx[j] + gp.by[i] * gp.by[j]);
                }
            }
        }
        for i in 0..6 {
            let pi = pos[e[i]];
            f[pi] += fe[i];
            fpsi[pi] += fp[i];
            fphi[pi] += fh[i];
            if pi == pin {
                continue;
            }
            for j in i..6 {
                let pj = pos[e[j]];
                if pj != pin {
                    sky.add(pi, pj, ke[i][j]);
                }
            }
        }
    }
    sky.add(pin, pin, 1.0);
    sky.cholesky(control)?;
    control.report("solve", 0.0);
    let pinned = |rhs: &[f64]| -> Vec<f64> {
        let mut x = rhs.to_vec();
        x[pin] = 0.0;
        sky.solve(&mut x);
        x
    };
    let (xo, xp, xh) = (pinned(&f), pinned(&fpsi), pinned(&fphi));
    control.check()?;
    control.report("solve", 1.0);
    let dot = |a: &[f64], b: &[f64]| -> f64 { a.iter().zip(b).map(|(p, q)| p * q).sum() };
    let j = ixx + iyy - dot(&xo, &f);
    let om: Vec<f64> = (0..n).map(|v| xo[pos[v]]).collect();
    let psi: Vec<f64> = (0..n).map(|v| xp[pos[v]]).collect();
    let phi: Vec<f64> = (0..n).map(|v| xh[pos[v]]).collect();
    let (mut s_x, mut s_y, mut q_om, mut i_om, mut i_xom, mut i_yom) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let (mut kx, mut ky) = (0.0, 0.0);
    for (ei, e) in elements.iter().enumerate() {
        if ei % 256 == 0 {
            control.check()?;
            control.report("integrate", ei as f64 / count as f64);
        }
        for gp in gauss_data(&coords, e).iter() {
            let (x, y) = (gp.x, gp.y);
            let o: f64 = (0..6).map(|i| gp.n[i] * om[e[i]]).sum();
            s_x += gp.w * (iyy * x + ixy * y) * (x * x + y * y);
            s_y += gp.w * (ixx * y + ixy * x) * (x * x + y * y);
            q_om += gp.w * o;
            i_om += gp.w * o * o;
            i_xom += gp.w * x * o;
            i_yom += gp.w * y * o;
            let (r, q) = (x * x - y * y, 2.0 * x * y);
            let (d1, d2) = (ixx * r - ixy * q, ixy * r + ixx * q);
            let (h1, h2) = (-ixy * r + iyy * q, -iyy * r - ixy * q);
            let bpx: f64 = (0..6).map(|i| gp.bx[i] * psi[e[i]]).sum::<f64>() - nu / 2.0 * d1;
            let bpy: f64 = (0..6).map(|i| gp.by[i] * psi[e[i]]).sum::<f64>() - nu / 2.0 * d2;
            let bhx: f64 = (0..6).map(|i| gp.bx[i] * phi[e[i]]).sum::<f64>() - nu / 2.0 * h1;
            let bhy: f64 = (0..6).map(|i| gp.by[i] * phi[e[i]]).sum::<f64>() - nu / 2.0 * h2;
            kx += gp.w * (bpx * bpx + bpy * bpy);
            ky += gp.w * (bhx * bhx + bhy * bhy);
        }
    }
    let ds = 2.0 * (1.0 + nu) * (ixx * iyy - ixy * ixy);
    let x_se = (nu / 2.0 * s_x - dot(&xh, &f)) / ds;
    let y_se = (nu / 2.0 * s_y + dot(&xp, &f)) / ds;
    let gamma = i_om - q_om * q_om / g.area - y_se * i_xom + x_se * i_yom;
    control.report("integrate", 1.0);
    Ok(Solved { j, gamma, sc_x: x_se + g.cx, sc_y: y_se + g.cy, a_sx: ds * ds / kx, a_sy: ds * ds / ky, nodes: n })
}
