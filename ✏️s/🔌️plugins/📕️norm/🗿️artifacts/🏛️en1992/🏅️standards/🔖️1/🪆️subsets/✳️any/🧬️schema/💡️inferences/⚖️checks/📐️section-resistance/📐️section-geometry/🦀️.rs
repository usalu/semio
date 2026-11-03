//! 📐️ General-section geometry of EN 1992-1-1 on [`Sec`]: gross properties, chord widths, effective depth, the web width b_w of 6.2.3, the thin-walled torsion section of 6.3.1, the effective tension area A_c,eff of 7.3.2 and the linear-elastic cracked state of 7.1/7.2; every member check reads these from the one section instead of a rectangle b × h. See [`super`].

use super::*;

/// 📏 Area and moments of one polygon about the origin (mm): `sy = ∫y dA`, `sz = ∫z dA`, `iy = ∫z² dA` (about the y axis), `iz = ∫y² dA`, orientation-normalised so that the area is positive.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Moments {
    pub a: f64,
    pub sy: f64,
    pub sz: f64,
    pub iy: f64,
    pub iz: f64,
}

impl Moments {
    fn add(self, o: Moments, k: f64) -> Moments {
        Moments { a: self.a + k * o.a, sy: self.sy + k * o.sy, sz: self.sz + k * o.sz, iy: self.iy + k * o.iy, iz: self.iz + k * o.iz }
    }
}

fn moments(p: &[(f64, f64)]) -> Moments {
    let (mut a, mut sy, mut sz, mut iy, mut iz) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for i in 0..p.len() {
        let ((y0, z0), (y1, z1)) = (p[i], p[(i + 1) % p.len()]);
        let c = y0 * z1 - y1 * z0;
        a += c;
        sy += (y0 + y1) * c;
        sz += (z0 + z1) * c;
        iy += (z0 * z0 + z0 * z1 + z1 * z1) * c;
        iz += (y0 * y0 + y0 * y1 + y1 * y1) * c;
    }
    let s = if a < 0.0 { -1.0 } else { 1.0 };
    Moments { a: s * a / 2.0, sy: s * sy / 6.0, sz: s * sz / 6.0, iy: s * iy / 12.0, iz: s * iz / 12.0 }
}

fn crossings(p: &[(f64, f64)], z: f64) -> Vec<f64> {
    let mut ys: Vec<f64> = (0..p.len())
        .filter_map(|i| {
            let (a, b) = (p[i], p[(i + 1) % p.len()]);
            ((a.1 > z) != (b.1 > z)).then(|| a.0 + (z - a.1) / (b.1 - a.1) * (b.0 - a.0))
        })
        .collect();
    ys.sort_by(f64::total_cmp);
    ys
}

fn perimeter(p: &[(f64, f64)]) -> f64 {
    (0..p.len()).map(|i| (p[(i + 1) % p.len()].0 - p[i].0).hypot(p[(i + 1) % p.len()].1 - p[i].1)).sum()
}

/// 📏 Gross concrete properties about the centroid (mm, mm², mm⁴): `i_y` bends about the horizontal y axis, `i_z` about the vertical axis.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Gross {
    pub area: f64,
    pub yc: f64,
    pub zc: f64,
    pub i_y: f64,
    pub i_z: f64,
    pub z_top: f64,
    pub z_bot: f64,
    pub y_min: f64,
    pub y_max: f64,
}

/// 🌀 Equivalent thin-walled section of 6.3.1 (mm, mm²): total area and outer circumference, effective wall `t_ef`, and the area and perimeter inside the wall centre lines.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Torsion {
    pub a: f64,
    pub u: f64,
    pub t_ef: f64,
    pub a_k: f64,
    pub u_k: f64,
}

/// 🧊 Linear-elastic state of the section under `M` and `N` (mm, N, MPa): strain at the origin and curvature, depth `x` of the compressed zone from the compressed face, concrete stress at that face and the largest tension and compression stress of the bars; `i_ii = M / (E_cm κ)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Elastic {
    pub eps0: f64,
    pub kappa: f64,
    pub x: f64,
    pub sigma_c: f64,
    pub sigma_s: f64,
    pub sigma_s2: f64,
    pub i_ii: f64,
}

impl Sec {
    /// 📏 Area, centroid, second moments and extents of the net concrete (holes subtract).
    pub fn gross_props(&self) -> Gross {
        let m = self.loops.iter().fold(Moments::default(), |t, l| t.add(moments(&l.pts), if l.hole { -1.0 } else { 1.0 }));
        let (z_bot, z_top) = self.extent((0.0, 1.0));
        let (y_min, y_max) = self.extent((1.0, 0.0));
        if m.a <= 1e-12 {
            return Gross { z_top, z_bot, y_min, y_max, ..Gross::default() };
        }
        let (yc, zc) = (m.sy / m.a, m.sz / m.a);
        Gross { area: m.a, yc, zc, i_y: m.iy - m.a * zc * zc, i_z: m.iz - m.a * yc * yc, z_top, z_bot, y_min, y_max }
    }

    /// ↔️ Net width of the concrete at height `z` (holes subtract).
    pub fn width_at(&self, z: f64) -> f64 {
        let w = |l: &Loop| crossings(&l.pts, z).chunks_exact(2).map(|c| c[1] - c[0]).sum::<f64>();
        self.loops.iter().map(|l| if l.hole { -w(l) } else { w(l) }).sum::<f64>().max(0.0)
    }

    /// ↔️ Outermost points `(y_min, y_max)` of the solid loops at height `z`, `None` outside the section.
    pub fn chord(&self, z: f64) -> Option<(f64, f64)> {
        let ys: Vec<f64> = self.loops.iter().filter(|l| !l.hole).flat_map(|l| crossings(&l.pts, z)).collect();
        Some((ys.iter().copied().fold(f64::INFINITY, f64::min), ys.iter().copied().fold(f64::NEG_INFINITY, f64::max))).filter(|c| c.0.is_finite() && c.1.is_finite())
    }

    /// 🔩 Height of the centroid of the tension reinforcement (bars below the concrete centroid), `None` without such bars.
    pub fn tension_level(&self) -> Option<f64> {
        let zc = self.gross_props().zc;
        let (a, s) = self.bars.iter().filter(|b| b.z < zc).fold((0.0, 0.0), |t, b| (t.0 + b.a, t.1 + b.a * b.z));
        (a > 0.0).then(|| s / a)
    }

    /// ➕ Area of the tension reinforcement (bars below the concrete centroid).
    pub fn tension_area(&self) -> f64 {
        let zc = self.gross_props().zc;
        self.bars.iter().filter(|b| b.z < zc).map(|b| b.a).sum()
    }

    /// 📏 Effective depth d from the top fibre to the tension reinforcement, `None` without tension bars.
    pub fn effective_depth(&self) -> Option<f64> {
        Some(self.gross_props().z_top - self.tension_level()?)
    }

    /// 🕸️ Web width b_w: the least net width between the tension reinforcement and the centroid (6.2.3(1), 6.2.2).
    pub fn web_width(&self) -> f64 {
        let g = self.gross_props();
        let zs = self.tension_level().unwrap_or(g.z_bot + 0.1 * (g.z_top - g.z_bot));
        let (lo, hi) = (zs.min(g.zc), zs.max(g.zc));
        let eps = 1e-6 * (g.z_top - g.z_bot).max(1.0);
        let mut cuts: Vec<f64> = self.loops.iter().flat_map(|l| l.pts.iter().map(|p| p.1)).filter(|z| *z > lo && *z < hi).collect();
        cuts.extend([lo, hi]);
        cuts.sort_by(f64::total_cmp);
        let least = cuts.windows(2).filter(|w| w[1] - w[0] > 2.0 * eps).flat_map(|w| [w[0] + eps, w[1] - eps]).map(|z| self.width_at(z)).fold(f64::INFINITY, f64::min);
        if least.is_finite() { least } else { self.width_at(g.zc) }
    }

    /// 🧱 Mean width b_t of the tension zone (concrete area below the centroid over its height), the b of 9.2.1.1(1).
    pub fn tension_zone_width(&self) -> f64 {
        let g = self.gross_props();
        if g.zc - g.z_bot <= 0.0 { return 0.0; }
        self.strip((0.0, 1.0), g.z_bot, g.zc).0 / (g.zc - g.z_bot)
    }

    /// 🌀 Equivalent thin-walled section of the outer loop with the wall at least `t_min`; with holes the wall is limited to the wall of the hollow section (6.3.1(1)).
    pub fn torsion_section(&self, t_min: f64) -> Option<Torsion> {
        let outer = self.loops.iter().find(|l| !l.hole)?;
        let (a, u) = (moments(&outer.pts).a, perimeter(&outer.pts));
        if a <= 0.0 || u <= 0.0 { return None; }
        let holes: Vec<&[(f64, f64)]> = self.loops.iter().filter(|l| l.hole).map(|l| l.pts.as_slice()).collect();
        let (a, u, t_ef, a_k, u_k) = shear_torsion::thin_walled_t(&outer.pts, (a / u).min(shear_torsion::hollow_wall(&outer.pts, &holes)).max(t_min));
        Some(Torsion { a, u, t_ef, a_k, u_k })
    }

    /// 🧲 Concrete area within `hc` of the tension face (bottom for sagging), the A_c,eff of 7.3.2(3).
    pub fn tension_face_area(&self, hc: f64) -> f64 {
        let (lo, _) = self.extent((0.0, 1.0));
        self.strip((0.0, 1.0), lo, lo + hc).0
    }

    /// ✂️ Effective tension height `h_c,eff = min(2.5 (h − d), (h − x) / 3, h / 2)` and the area A_c,eff of 7.3.2(3) for the neutral axis depth `x`.
    pub fn crack_zone(&self, x: f64) -> (f64, f64) {
        let g = self.gross_props();
        let h = g.z_top - g.z_bot;
        let d = self.effective_depth().unwrap_or(0.9 * h);
        let hc = (2.5 * (h - d)).min((h - x) / 3.0).min(h / 2.0).max(1e-6);
        (hc, self.tension_face_area(hc))
    }

    /// 🎨 The solid outline between the heights `z_lo` and `z_hi` as polygons (mm) for drawing, such as the effective tension area A_c,eff.
    pub fn band(&self, z_lo: f64, z_hi: f64) -> Vec<Vec<(f64, f64)>> {
        self.loops.iter().filter(|l| !l.hole).map(|l| clip(&clip(&l.pts, (0.0, 1.0), z_lo, true), (0.0, 1.0), z_hi, false)).filter(|p| p.len() >= 3).collect()
    }

    fn elastic_forces(&self, eps0: f64, kappa: f64, uncracked: bool) -> (f64, f64) {
        let mut q = Moments::default();
        for l in &self.loops {
            let kept = if uncracked || (kappa == 0.0 && eps0 > 0.0) {
                l.pts.clone()
            } else if kappa == 0.0 {
                Vec::new()
            } else {
                clip(&l.pts, (0.0, 1.0), -eps0 / kappa, kappa > 0.0)
            };
            if kept.len() >= 3 { q = q.add(moments(&kept), if l.hole { -1.0 } else { 1.0 }); }
        }
        let (mut n, mut m) = (self.c.ecm * (eps0 * q.a + kappa * q.sz), self.c.ecm * (eps0 * q.sz + kappa * q.iy));
        for b in &self.bars {
            let f = self.es * (eps0 + kappa * b.z) * b.a;
            n += f;
            m += f * b.z;
        }
        for t in &self.tendons {
            let f = -self.ep * (t.pre / 1000.0 - eps0 - kappa * t.z) * t.a;
            n += f;
            m += f * t.z;
        }
        (n, m)
    }

    fn elastic_strain(&self, kappa: f64, n_ed: f64, uncracked: bool) -> Option<f64> {
        let (mut lo, mut hi) = (-0.5, 0.5);
        if self.elastic_forces(lo, kappa, uncracked).0 > n_ed || self.elastic_forces(hi, kappa, uncracked).0 < n_ed { return None; }
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if self.elastic_forces(mid, kappa, uncracked).0 < n_ed { lo = mid } else { hi = mid }
        }
        Some(0.5 * (lo + hi))
    }

    /// 🧊 Linear-elastic state under `m_ed` (N·mm, compression at +z positive) and `n_ed` (N, compression positive): concrete in tension is zero (state II) or linear (`uncracked`, state I), bars and tendons are linear with E_s and E_p and carry their pre-strain; `None` when the loads cannot be balanced.
    pub fn elastic(&self, m_ed: f64, n_ed: f64, uncracked: bool) -> Option<Elastic> {
        let g = self.gross_props();
        let depth = g.z_top - g.z_bot;
        if depth <= 0.0 || g.area <= 0.0 { return None; }
        let moment = |k: f64| self.elastic_strain(k, n_ed, uncracked).map(|e| self.elastic_forces(e, k, uncracked).1);
        let (mut lo, mut hi) = (-0.05 / depth, 0.05 / depth);
        if m_ed < moment(lo)? || m_ed > moment(hi)? { return None; }
        for _ in 0..70 {
            let mid = 0.5 * (lo + hi);
            if moment(mid)? < m_ed { lo = mid } else { hi = mid }
        }
        let kappa = 0.5 * (lo + hi);
        let eps0 = self.elastic_strain(kappa, n_ed, uncracked)?;
        let x = if kappa.abs() < 1e-15 { depth } else if kappa > 0.0 { (g.z_top + eps0 / kappa).clamp(0.0, depth) } else { (-eps0 / kappa - g.z_bot).clamp(0.0, depth) };
        let z_face = if kappa >= 0.0 { g.z_top } else { g.z_bot };
        let stress: Vec<f64> = self.bars.iter().map(|b| self.es * (eps0 + kappa * b.z)).collect();
        let largest = |pick: fn(f64) -> f64| Some(stress.iter().map(|s| pick(*s)).fold(f64::NEG_INFINITY, f64::max)).filter(|v| v.is_finite()).unwrap_or(0.0);
        Some(Elastic {
            eps0,
            kappa,
            x,
            sigma_c: self.c.ecm * (eps0 + kappa * z_face).max(0.0),
            sigma_s: largest(|s| -s),
            sigma_s2: largest(|s| s),
            i_ii: if kappa.abs() > 1e-18 { m_ed / (self.c.ecm * kappa) } else { 0.0 },
        })
    }
}

/// 📐️ SI geometry of one member section (m, m², m⁴) read from [`Sec`]; the single source of b_w, d, z, A_c, I, u and the widths for the shear, torsion, prestress, crack, deflection and detailing checks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Geo {
    pub area: f64,
    pub h: f64,
    pub i_y: f64,
    pub i_min: f64,
    pub zc_top: f64,
    pub zc_bot: f64,
    pub b_w: f64,
    pub b_t: f64,
    pub b_c: f64,
    pub d: f64,
    pub a_s: f64,
    pub u: f64,
    pub extent_y: f64,
}

impl Geo {
    /// 📐️ Geometry of `sec`; the effective depth falls back to the snapshot value for a section without tension bars.
    pub fn of(sec: &Sec, member: &RcMember) -> Self {
        let g = sec.gross_props();
        let u = sec.loops.iter().find(|l| !l.hole).map_or(0.0, |l| perimeter(&l.pts));
        Self {
            area: g.area * 1e-6,
            h: (g.z_top - g.z_bot) * 1e-3,
            i_y: g.i_y * 1e-12,
            i_min: g.i_y.min(g.i_z) * 1e-12,
            zc_top: (g.z_top - g.zc) * 1e-3,
            zc_bot: (g.zc - g.z_bot) * 1e-3,
            b_w: sec.web_width() * 1e-3,
            b_t: sec.tension_zone_width() * 1e-3,
            b_c: sec.width_at(g.z_top - 1e-6 * (g.z_top - g.z_bot).max(1.0)) * 1e-3,
            d: sec.effective_depth().map_or(member.effective_depth, |d| d * 1e-3),
            a_s: sec.tension_area() * 1e-6,
            u: u * 1e-3,
            extent_y: (g.y_max - g.y_min) * 1e-3,
        }
    }

    /// 📏 Lever arm z = 0.9 d of 6.2.3(1).
    pub fn z(&self) -> f64 {
        0.9 * self.d
    }

    /// ➗ Longitudinal reinforcement ratio ρ_l = A_sl / (b_w d) of 6.2.2(1).
    pub fn rho_l(&self) -> f64 {
        if self.b_w * self.d > 0.0 { self.a_s / (self.b_w * self.d) } else { 0.0 }
    }

    /// 📐️ Radius of gyration about the weak axis, i = √(I_min / A).
    pub fn radius(&self) -> f64 {
        if self.area > 0.0 { (self.i_min / self.area).sqrt() } else { 0.0 }
    }

    /// 📏 Smallest overall dimension for the fire tables (5.6): the web width of beams and bridges, the smaller side of columns, walls and tension members, the depth of slabs.
    pub fn fire_dimension(&self, kind: MemberKind) -> f64 {
        match kind {
            MemberKind::Slab | MemberKind::FlatSlab | MemberKind::RibbedSlab | MemberKind::LiquidRetaining => self.h,
            MemberKind::Column | MemberKind::Wall | MemberKind::TensionMember => self.extent_y.min(self.h),
            MemberKind::Beam | MemberKind::Bridge => self.b_w,
        }
    }
}

/// 📏 SG-01 (EN 1992-1-1 6.2.3(1), 6.3.1(1), 7.3.2(3), 9.2.1.1(1)): the geometric properties every member check reads from the general section — area, second moments, centroid, web width b_w, tension-zone width b_t, effective depth d, tension steel area and the equivalent thin-walled section with `t_min = 2 c + φ_l`.
pub fn sg01(c: &Case) -> R {
    let s = section_of(c)?;
    let g = s.gross_props();
    let mut v = vec![
        Outcome::mm2("area", "Gross area A_c", "Bruttofläche A_c", g.area, "polygon, holes subtract".into()),
        Outcome::info("i_y", "Second moment I_y (mm⁴)", "Flächenmoment I_y (mm⁴)", QuantityKind::Dimensionless, g.i_y, "about the centroidal y axis".into()),
        Outcome::info("i_z", "Second moment I_z (mm⁴)", "Flächenmoment I_z (mm⁴)", QuantityKind::Dimensionless, g.i_z, "about the centroidal z axis".into()),
        Outcome::mm("z_bot", "Centroid above the bottom fibre", "Schwerpunkt über der Unterkante", g.zc - g.z_bot, "∫z dA / A".into()),
        Outcome::mm("b_w", "Web width b_w", "Stegbreite b_w", s.web_width(), "least net width between the tension steel and the centroid".into()),
        Outcome::mm("b_t", "Tension zone width b_t", "Zugzonenbreite b_t", s.tension_zone_width(), "area below the centroid over its height".into()),
        Outcome::mm2("a_s", "Tension steel area A_s", "Zugbewehrung A_s", s.tension_area(), "bars below the centroid".into()),
    ];
    if let Some(d) = s.effective_depth() {
        v.push(Outcome::mm("d", "Effective depth d", "Nutzhöhe d", d, "top fibre to the tension steel centroid".into()));
    }
    if let Some(t) = s.torsion_section(2.0 * c.o("c", 0.0) + c.o("phi_l", 0.0)) {
        v.push(Outcome::mm("t_ef", "Effective wall t_ef", "Wirksame Wanddicke t_ef", t.t_ef, "max(A/u, 2 c + φ_l), at most the wall of a hollow section".into()));
        v.push(Outcome::mm2("a_k", "Enclosed area A_k", "Eingeschlossene Fläche A_k", t.a_k, "outline moved in by t_ef/2".into()));
        v.push(Outcome::mm("u_k", "Perimeter u_k", "Umfang u_k", t.u_k, "centre line".into()));
    }
    Ok(v)
}

/// 🧊 SG-02 (EN 1992-1-1 7.1, 7.2, 7.3.4): the linear-elastic cracked (state II) or uncracked (state I) state of the general section under `m_ed` (N·mm, compression at +z) and `n_ed` (N, compression positive): neutral axis depth, concrete and steel stresses, `I_II = M / (E_cm κ)` and the effective tension area A_c,eff of 7.3.2(3).
pub fn sg02(c: &Case) -> R {
    let mut s = section_of(c)?;
    let g = conc(c.pos("fck")?);
    s.c.ecm = c.opt("phi_inf").map_or(g.ecm, |p| g.ecm / (1.0 + p));
    s.es = c.o("es", 200000.0);
    let e = s.elastic(c.n("m_ed")?, c.o("n_ed", 0.0), c.so("state", "cracked") == "uncracked").ok_or("the loads cannot be balanced by the section")?;
    let (hc, ac) = s.crack_zone(e.x);
    Ok(vec![
        Outcome::mm("x", "Neutral axis depth x", "Nulllinientiefe x", e.x, "from the compressed face".into()),
        Outcome::mpa("sigma_c", "Concrete stress σ_c", "Betonspannung σ_c", e.sigma_c, "E_cm ε at the compressed face".into()),
        Outcome::mpa("sigma_s", "Tension steel stress σ_s", "Zugstahlspannung σ_s", e.sigma_s, "largest bar stress".into()),
        Outcome::mpa("sigma_s2", "Compression steel stress σ_s2", "Druckstahlspannung σ_s2", e.sigma_s2, "largest compressed bar".into()),
        Outcome::info("i_ii", "Second moment I_II (mm⁴)", "Flächenmoment I_II (mm⁴)", QuantityKind::Dimensionless, e.i_ii, "M / (E_cm κ)".into()),
        Outcome::mm("hc_eff", "Effective tension height h_c,eff", "Wirksame Zugzonenhöhe h_c,eff", hc, "min(2.5 (h − d), (h − x)/3, h/2)".into()),
        Outcome::mm2("ac_eff", "Effective tension area A_c,eff", "Wirksame Zugfläche A_c,eff", ac, "concrete within h_c,eff of the tension face".into()),
    ])
}

/// 🗂 Entries of the general-section geometry.
pub const GEOMETRY_ENTRIES: &[Entry] = &[
    entry_of("SG-01", "DIN EN 1992-1-1", "6.2.3", "General section properties", "Allgemeine Querschnittswerte", &["fck", "poly_y", "poly_z", "hole_y", "hole_z", "bar_y", "bar_z", "bar_a", "c", "phi_l"], &[], sg01),
    entry_of("SG-02", "DIN EN 1992-1-1", "7.1", "Linear-elastic section state", "Linear-elastischer Querschnittszustand", &["fck", "poly_y", "poly_z", "hole_y", "hole_z", "bar_y", "bar_z", "bar_a", "es", "m_ed", "n_ed", "state", "phi_inf"], &[], sg02),
];
