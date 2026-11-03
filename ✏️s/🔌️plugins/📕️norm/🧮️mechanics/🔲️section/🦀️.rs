//! 🔲️ Cross-section analysis: `analyse_section(polygon, holes, material) -> SectionConstants` meshes the outline with six-node triangles and solves the Saint-Venant torsion, warping and shear problems (St Venant J, warping constant I_w about the shear centre, shear centre, shear areas, plastic moduli); `analyse_thin_walled` is the open thin-wall alternative. Progress and cancellation through [`Control`]. SI units, y horizontal and z up. Reference: <https://github.com/robbievanleeuwen/section-properties>.

use crate::{Control, Material, MechanicsResult, Section, SectionConstants, SectionMethod, SectionOptions, ThinWall};
use mesh::P2;

#[path = "🕸️mesh/🦀️.rs"]
pub mod mesh;

#[path = "🧊️fem/🦀️.rs"]
mod fem;

#[path = "🪶️thin-walled/🦀️.rs"]
mod thin_walled;

#[path = "📐️profiles/🦀️.rs"]
pub mod profiles;

#[path = "🧵️fibre/🦀️.rs"]
pub mod fibre;

/// 📏️ Exact area, centroid and second moments about the centroid of a ring set (outer counter-clockwise, holes clockwise); `ixx = ∫z²`, `iyy = ∫y²`, `ixy = ∫yz` in the input frame (x is the horizontal coordinate y of the API).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Geometry {
    pub area: f64,
    pub cx: f64,
    pub cy: f64,
    pub ixx: f64,
    pub iyy: f64,
    pub ixy: f64,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

fn geometry(rings: &[Vec<P2>]) -> Geometry {
    let (mut area, mut sx, mut sy, mut sxx, mut syy, mut sxy) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    for ring in rings {
        let n = ring.len();
        for i in 0..n {
            let (p, q) = (ring[i], ring[(i + 1) % n]);
            let c = p[0] * q[1] - q[0] * p[1];
            area += c / 2.0;
            sx += (p[0] + q[0]) * c / 6.0;
            sy += (p[1] + q[1]) * c / 6.0;
            sxx += (p[0] * p[0] + p[0] * q[0] + q[0] * q[0]) * c / 12.0;
            syy += (p[1] * p[1] + p[1] * q[1] + q[1] * q[1]) * c / 12.0;
            sxy += (p[0] * q[1] + 2.0 * p[0] * p[1] + 2.0 * q[0] * q[1] + q[0] * p[1]) * c / 24.0;
        }
    }
    let (cx, cy) = (sx / area, sy / area);
    let fold = |axis: usize, init: f64, pick: fn(f64, f64) -> f64| rings[0].iter().fold(init, |m, p| pick(m, p[axis]));
    Geometry {
        area,
        cx,
        cy,
        ixx: syy - area * cy * cy,
        iyy: sxx - area * cx * cx,
        ixy: sxy - area * cx * cy,
        x_min: fold(0, f64::MAX, f64::min),
        x_max: fold(0, f64::MIN, f64::max),
        y_min: fold(1, f64::MAX, f64::min),
        y_max: fold(1, f64::MIN, f64::max),
    }
}

fn clip(ring: &[P2], axis: usize, value: f64, below: bool) -> Vec<P2> {
    let keep = |p: P2| if below { p[axis] <= value } else { p[axis] >= value };
    let cut = |a: P2, b: P2| -> P2 {
        let t = (value - a[axis]) / (b[axis] - a[axis]);
        [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])]
    };
    let n = ring.len();
    let mut out = Vec::with_capacity(n + 2);
    for i in 0..n {
        let (previous, current) = (ring[(i + n - 1) % n], ring[i]);
        if keep(current) {
            if !keep(previous) {
                out.push(cut(previous, current));
            }
            out.push(current);
        } else if keep(previous) {
            out.push(cut(previous, current));
        }
    }
    out
}

fn side(rings: &[Vec<P2>], axis: usize, value: f64, below: bool) -> (f64, f64) {
    let (mut area, mut moment) = (0.0, 0.0);
    for ring in rings {
        let part = clip(ring, axis, value, below);
        let n = part.len();
        for i in 0..n {
            let (p, q) = (part[i], part[(i + 1) % n]);
            let c = p[0] * q[1] - q[0] * p[1];
            area += c / 2.0;
            moment += (p[axis] + q[axis]) * c / 6.0;
        }
    }
    (area, moment)
}

fn plastic_modulus(rings: &[Vec<P2>], area: f64, axis: usize, low: f64, high: f64) -> f64 {
    let (mut lo, mut hi) = (low, high);
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if side(rings, axis, mid, true).0 < area / 2.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let line = 0.5 * (lo + hi);
    let (a_below, m_below) = side(rings, axis, line, true);
    let (a_above, m_above) = side(rings, axis, line, false);
    (line * a_below - m_below) + (m_above - line * a_above)
}

fn poisson(material: &Material) -> f64 {
    if material.e > 0.0 && material.g > 0.0 { (material.e / (2.0 * material.g) - 1.0).clamp(-0.5, 0.499) } else { 0.0 }
}

fn principal(iy: f64, iz: f64, iyz: f64) -> (f64, f64) {
    let radius = (((iy - iz) / 2.0).powi(2) + iyz * iyz).sqrt();
    ((iy + iz) / 2.0 + radius, (iy + iz) / 2.0 - radius)
}

fn longest_edge(mesh: &mesh::Mesh) -> f64 {
    mesh.triangles.iter().flat_map(|t| (0..3).map(move |k| (t[k], t[(k + 1) % 3]))).fold(0.0_f64, |m, (a, b)| m.max((mesh.points[a][0] - mesh.points[b][0]).hypot(mesh.points[a][1] - mesh.points[b][1])))
}

/// 🔲️ Constants of a polygon with holes (`outer` and `holes` as `[y, z]` vertex lists in metres, any orientation) with the default mesh density and no progress; see [`analyse_section_with`].
pub fn analyse_section(outer: &[[f64; 2]], holes: &[Vec<[f64; 2]>], material: &Material) -> MechanicsResult<SectionConstants> {
    analyse_section_with(outer, holes, material, &SectionOptions::standard(), &Control::none())
}

/// 🔲️ Finite-element constants of a polygon with holes: the outline is meshed with edges no longer than the mean wall thickness 2A/P divided by `options.cells_across_wall` (or `options.max_edge`), never exceeding about `options.max_triangles` triangles; stages `mesh`, `assemble`, `factorise`, `solve`, `integrate` are reported through `control`, which also cancels at every checkpoint. `material` supplies Poisson's ratio for the shear functions (the constants are geometric otherwise). The shear centre is the elasticity shear centre, I_w refers to it, A_v,y and A_v,z are the shear areas for shear along y and z through the shear centre.
pub fn analyse_section_with(outer: &[[f64; 2]], holes: &[Vec<[f64; 2]>], material: &Material, options: &SectionOptions, control: &Control<'_>) -> MechanicsResult<SectionConstants> {
    let rings = mesh::normalise(outer, holes)?;
    let g = geometry(&rings);
    let nu = poisson(material);
    let target = mesh::target_edge(&rings, options.cells_across_wall, options.max_edge, options.max_triangles);
    let grid = mesh::generate(&rings, target, options.max_triangles, control)?;
    let solved = fem::solve(&grid, &g, nu, control)?;
    let (major, minor) = principal(g.ixx, g.iyy, g.ixy);
    Ok(SectionConstants {
        area: g.area,
        centroid_y: g.cx,
        centroid_z: g.cy,
        iy: g.ixx,
        iz: g.iyy,
        iyz: g.ixy,
        i_major: major,
        i_minor: minor,
        j: solved.j,
        iw: solved.gamma,
        shear_centre_y: solved.sc_x,
        shear_centre_z: solved.sc_y,
        a_vy: solved.a_sx,
        a_vz: solved.a_sy,
        w_el_y_pos: g.ixx / (g.y_max - g.cy),
        w_el_y_neg: g.ixx / (g.cy - g.y_min),
        w_el_z_pos: g.iyy / (g.x_max - g.cx),
        w_el_z_neg: g.iyy / (g.cx - g.x_min),
        w_pl_y: plastic_modulus(&rings, g.area, 1, g.y_min, g.y_max),
        w_pl_z: plastic_modulus(&rings, g.area, 0, g.x_min, g.x_max),
        poisson: nu,
        method: SectionMethod::FiniteElement,
        triangles: grid.triangles.len(),
        nodes: solved.nodes,
        max_edge: longest_edge(&grid),
    })
}

/// 🪶️ Constants of an open thin-walled section from centreline walls: A, I, J = Σ l t³ / 3, I_w and the shear centre by sectorial coordinates, shear areas from the shear-flow energy. Elastic and plastic moduli are not derived (zero) because the walls carry no outline. Closed cells and disconnected walls are refused; use [`analyse_section`] for them.
pub fn analyse_thin_walled(walls: &[ThinWall], material: &Material) -> MechanicsResult<SectionConstants> {
    let t = thin_walled::analyse(walls)?;
    let (major, minor) = principal(t.iy, t.iz, t.iyz);
    Ok(SectionConstants {
        area: t.area,
        centroid_y: t.cy,
        centroid_z: t.cz,
        iy: t.iy,
        iz: t.iz,
        iyz: t.iyz,
        i_major: major,
        i_minor: minor,
        j: t.j,
        iw: t.iw,
        shear_centre_y: t.sc_y,
        shear_centre_z: t.sc_z,
        a_vy: t.a_vy,
        a_vz: t.a_vz,
        poisson: poisson(material),
        w_el_y_pos: 0.0,
        w_el_y_neg: 0.0,
        w_el_z_pos: 0.0,
        w_el_z_neg: 0.0,
        w_pl_y: 0.0,
        w_pl_z: 0.0,
        method: SectionMethod::ThinWalled,
        triangles: 0,
        nodes: 0,
        max_edge: 0.0,
    })
}

impl SectionConstants {
    /// 🧱️ The beam section of the kernel model with these constants: `iy` is the bending inertia about y (∫z²), `asy`/`asz` the shear areas along y and z.
    pub fn to_section(&self, id: &str) -> Section {
        Section { id: id.to_string(), a: self.area, iy: self.iy, iz: self.iz, j: self.j, iw: self.iw, asy: self.a_vy, asz: self.a_vz }
    }
}
