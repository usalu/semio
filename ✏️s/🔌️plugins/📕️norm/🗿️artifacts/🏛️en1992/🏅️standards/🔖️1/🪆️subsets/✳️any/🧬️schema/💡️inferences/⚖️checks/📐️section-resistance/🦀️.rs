//! 📐️ Section resistance of EN 1992-1-1 §6.1 as catalogue entries S-01 … S-10: strain-compatibility integrator over arbitrary polygons with holes, N–M interaction, biaxial surface and slice, rectangular design. See [`super`].

use super::*;
use semio_s_plugin_norm_mechanics::section::fibre::{Fibre, FibreSection, Law, Reinforcement};

#[path = "📐️section-geometry/🦀️.rs"]
mod section_geometry;
pub use section_geometry::*;

/// 🔶 Polygon loop in local (y, z) mm; `hole` loops subtract.
#[derive(Clone, Debug)]
pub struct Loop {
    pub pts: Vec<(f64, f64)>,
    pub hole: bool,
}

/// 🔩 Reinforcing bar in section coordinates (mm, mm²).
#[derive(Clone, Copy, Debug)]
pub struct Bar {
    pub y: f64,
    pub z: f64,
    pub a: f64,
}

/// 🎻 Tendon with pre-strain in permille.
#[derive(Clone, Copy, Debug)]
pub struct Tendon {
    pub y: f64,
    pub z: f64,
    pub a: f64,
    pub pre: f64,
}

/// 🧱 Reinforced or prestressed section with its material set; stresses in MPa.
#[derive(Clone, Debug)]
pub struct Sec {
    pub loops: Vec<Loop>,
    pub bars: Vec<Bar>,
    pub tendons: Vec<Tendon>,
    pub c: Conc,
    pub fcd: f64,
    pub fyd: f64,
    pub es: f64,
    pub k: f64,
    pub eps_ud: f64,
    pub fp_d: f64,
    pub fpt_d: f64,
    pub ep: f64,
    pub eps_ud_p: f64,
    pub steps: usize,
}

fn signed_area(p: &[(f64, f64)]) -> (f64, f64, f64) {
    let (mut a, mut sy, mut sz) = (0.0, 0.0, 0.0);
    for i in 0..p.len() {
        let (y0, z0) = p[i];
        let (y1, z1) = p[(i + 1) % p.len()];
        let cr = y0 * z1 - y1 * z0;
        a += cr;
        sy += (y0 + y1) * cr;
        sz += (z0 + z1) * cr;
    }
    (a / 2.0, sy / 6.0, sz / 6.0)
}

fn clip(p: &[(f64, f64)], n: (f64, f64), c: f64, ge: bool) -> Vec<(f64, f64)> {
    let side = |q: (f64, f64)| if ge { q.0 * n.0 + q.1 * n.1 - c } else { c - q.0 * n.0 - q.1 * n.1 };
    let mut out = Vec::with_capacity(p.len() + 2);
    for i in 0..p.len() {
        let (a, b) = (p[i], p[(i + 1) % p.len()]);
        let (sa, sb) = (side(a), side(b));
        if sa >= 0.0 {
            out.push(a);
        }
        if (sa >= 0.0) != (sb >= 0.0) {
            let t = sa / (sa - sb);
            out.push((a.0 + t * (b.0 - a.0), a.1 + t * (b.1 - a.1)));
        }
    }
    out
}

impl Sec {
    /// 🟦 Rectangular section b × h centred on the origin.
    pub fn rect(b: f64, h: f64, c: Conc, fcd: f64, fyd: f64) -> Self {
        let pts = vec![(-b / 2.0, -h / 2.0), (b / 2.0, -h / 2.0), (b / 2.0, h / 2.0), (-b / 2.0, h / 2.0)];
        Self { loops: vec![Loop { pts, hole: false }], bars: vec![], tendons: vec![], c, fcd, fyd, es: 200000.0, k: 1.08, eps_ud: 45.0, fp_d: 0.0, fpt_d: 0.0, ep: 195000.0, eps_ud_p: 20.0, steps: 600 }
    }
    /// 📏 Gross concrete area, first moments and second moments about the origin.
    pub fn gross(&self) -> (f64, f64, f64) {
        self.loops.iter().fold((0.0, 0.0, 0.0), |acc, l| {
            let s = if l.hole { -1.0 } else { 1.0 };
            let (a, sy, sz) = signed_area(&l.pts);
            (acc.0 + s * a.abs(), acc.1 + s * sy * a.signum(), acc.2 + s * sz * a.signum())
        })
    }
    /// 🎯 Move the origin to the gross centroid.
    pub fn centred(mut self) -> Self {
        let (a, sy, sz) = self.gross();
        let (cy, cz) = (sy / a, sz / a);
        for l in &mut self.loops { for p in &mut l.pts { p.0 -= cy; p.1 -= cz; } }
        for b in &mut self.bars { b.y -= cy; b.z -= cz; }
        for t in &mut self.tendons { t.y -= cy; t.z -= cz; }
        self
    }
    pub fn extent(&self, n: (f64, f64)) -> (f64, f64) {
        let mut hi = f64::MIN;
        let mut lo = f64::MAX;
        for l in &self.loops { for p in &l.pts { let s = p.0 * n.0 + p.1 * n.1; hi = hi.max(s); lo = lo.min(s); } }
        (lo, hi)
    }
    fn strip(&self, n: (f64, f64), c0: f64, c1: f64) -> (f64, f64, f64) {
        let mut t = (0.0, 0.0, 0.0);
        for l in &self.loops {
            let q = clip(&clip(&l.pts, n, c0, true), n, c1, false);
            if q.len() < 3 { continue; }
            let (a, sy, sz) = signed_area(&q);
            let s = if l.hole { -1.0 } else { 1.0 } * if signed_area(&l.pts).0 < 0.0 { -1.0 } else { 1.0 };
            t = (t.0 + s * a, t.1 + s * sy, t.2 + s * sz);
        }
        t
    }
    fn steel(&self, eps: f64) -> f64 {
        sigma_s(self.fyd, self.es, self.k, self.eps_ud / 0.9, true, eps)
    }
    fn tendon_stress(&self, eps_tension: f64) -> f64 {
        let fpd = self.fp_d;
        let ey = fpd / self.ep * 1000.0;
        let a = eps_tension.abs();
        let s = if a <= ey { self.ep * a / 1000.0 } else { fpd + (self.fpt_d - fpd) * (a.min(self.eps_ud_p) - ey) / (self.eps_ud_p - ey) };
        s * eps_tension.signum()
    }
    /// ⚙️ Internal forces (N compression positive, My = ∫σ z, Mz = −∫σ y, N·mm) at neutral axis depth `x` from the compressed extreme fibre in direction `alpha` (rad, direction towards the compressed side).
    pub fn state(&self, alpha: f64, x: f64) -> (f64, f64, f64) {
        self.state_plane(alpha, self.top_strain(alpha, x), x)
    }
    /// 📉 Strain (permille, compression positive) at the compressed extreme fibre for the neutral axis depth `x`, following the pivots A, B and C of 6.1: the steel or tendon at ε_ud, the concrete at ε_cu2, or the section rotating about the point at ε_c2.
    pub fn top_strain(&self, alpha: f64, x: f64) -> f64 {
        let n = (alpha.cos(), alpha.sin());
        let (lo, hi) = self.extent(n);
        let depth = hi - lo;
        let u_of = |y: f64, z: f64| hi - (y * n.0 + z * n.1);
        let (ecu, ec2) = (self.c.eps_cu2, self.c.eps_c2);
        let etop = if x <= depth {
            let mut e = ecu;
            for b in &self.bars { let u = u_of(b.y, b.z); if u > x { e = e.min(self.eps_ud * x / (u - x)); } }
            for t in &self.tendons { let u = u_of(t.y, t.z); if u > x { e = e.min((self.eps_ud_p - t.pre).max(0.0) * x / (u - x)); } }
            e
        } else {
            let uc = depth * (1.0 - ec2 / ecu);
            ec2 * x / (x - uc)
        };
        etop.max(1e-12)
    }
    /// ⚙️ Internal forces for the strain plane with edge strain `etop` (permille, compression positive at the extreme compressed fibre) and zero strain at depth `x`; no ultimate strain limitation.
    pub fn state_plane(&self, alpha: f64, etop: f64, x: f64) -> (f64, f64, f64) {
        let n = (alpha.cos(), alpha.sin());
        let (lo, hi) = self.extent(n);
        let depth = hi - lo;
        let u_of = |y: f64, z: f64| hi - (y * n.0 + z * n.1);
        let eps = |u: f64| etop * (x - u) / x;
        let zone = x.min(depth);
        let steps = self.steps;
        let du = zone / steps as f64;
        let (mut nn, mut my, mut mz) = (0.0, 0.0, 0.0);
        for i in 0..steps {
            let (u0, u1) = (i as f64 * du, (i as f64 + 1.0) * du);
            let s = sigma_c_pr(&self.c, self.fcd, eps(0.5 * (u0 + u1)));
            if s == 0.0 { continue; }
            let (a, sy, sz) = self.strip(n, hi - u1, hi - u0);
            nn += s * a;
            my += s * sz;
            mz -= s * sy;
        }
        for b in &self.bars {
            let e = eps(u_of(b.y, b.z));
            let mut st = self.steel(e);
            if e > 0.0 { st -= sigma_c_pr(&self.c, self.fcd, e); }
            let f = st * b.a;
            nn += f;
            my += f * b.z;
            mz -= f * b.y;
        }
        for t in &self.tendons {
            let e = eps(u_of(t.y, t.z));
            let st = self.tendon_stress(t.pre - e);
            let f = -st * t.a;
            nn += f;
            my += f * t.z;
            mz -= f * t.y;
        }
        (nn, my, mz)
    }
    /// 🧶 Fibre section of the polygons for the shared kernel [`FibreSection`]: strips of the concrete (state II, or linear in tension for `uncracked`), the bars displacing the concrete they replace and the tendons with their pre-strain.
    fn fibre_section(&self, uncracked: bool) -> FibreSection<'_> {
        let (lo, hi) = self.extent((0.0, 1.0));
        let steps = self.steps.max(120);
        let dz = (hi - lo) / steps as f64;
        let concrete = (0..steps).map(|i| {
            let (z0, z1) = (lo + i as f64 * dz, lo + (i as f64 + 1.0) * dz);
            let (a, _, s) = self.strip((0.0, 1.0), z0, z1);
            Fibre { z: 0.5 * (z0 + z1), a, s }
        }).collect();
        let bars = self.bars.iter().map(|b| Reinforcement { fibre: Fibre::lumped(b.z, b.a), law: Box::new(move |e: f64| self.steel(e)), displaces: true });
        let tendons = self.tendons.iter().map(|t| {
            let pre = t.pre;
            Reinforcement { fibre: Fibre::lumped(t.z, t.a), law: Box::new(move |e: f64| -self.tendon_stress(pre - e)), displaces: false }
        });
        let concrete_law: Law<'_> = Box::new(move |e: f64| if e >= 0.0 { sigma_c_pr(&self.c, self.fcd, e) } else if uncracked { self.c.ecm * e / 1000.0 } else { 0.0 });
        FibreSection { concrete, concrete_law, reinforcement: bars.chain(tendons).collect(), top: hi, cache: Default::default() }
    }
    /// 🧵 Axial force and moment (N, N·mm; compression positive, compression at +z) of the strain plane ε(z) = `eps_m` + `kappa` z (permille, permille/mm) by strip integration over the polygons; concrete in tension is zero (state II) or linear (`uncracked`).
    pub fn fibre(&self, eps_m: f64, kappa: f64, uncracked: bool) -> (f64, f64) {
        self.fibre_section(uncracked).integrate(eps_m, kappa)
    }
    /// 📍 Bending moment (N·mm) at the curvature `kappa` (1/m) and normal force `n_ed` (N, compression positive), `None` when the strain plane cannot balance `n_ed` below the ultimate concrete strain.
    pub fn moment_at(&self, kappa: f64, n_ed: f64, uncracked: bool) -> Option<f64> {
        self.fibre_section(uncracked).moment_at(kappa, n_ed, self.c.eps_cu2)
    }
    /// 📉 Moment-curvature curve at the normal force `n_ed` (N, compression positive): (κ in 1/m, M_y in N·mm) up to the ultimate strain of concrete, steel or tendon; `uncracked` selects state I (linear tension) against state II.
    pub fn moment_curvature(&self, n_ed: f64, points: usize, uncracked: bool) -> Vec<(f64, f64)> {
        let (lo_z, hi_z) = self.extent((0.0, 1.0));
        let kappa_u = (self.c.eps_cu2 + self.eps_ud) / (hi_z - lo_z).max(1.0);
        self.fibre_section(uncracked).curve(n_ed, kappa_u, points, self.c.eps_cu2)
    }
    /// 🧊 Cracking moment (N·mm) and curvature (1/m) of the gross polygon at `n_ed`, with f_ctm of the concrete and the transformed bars ignored: M_cr = (f_ctm + N/A) I/z_t.
    pub fn cracking(&self, n_ed: f64) -> (f64, f64) {
        let (a, sy, sz) = self.gross();
        let zc = sz / a;
        let mut i_y = 0.0;
        let (lo, hi) = self.extent((0.0, 1.0));
        let steps = self.steps.max(120);
        let dz = (hi - lo) / steps as f64;
        for i in 0..steps {
            let (z0, z1) = (lo + i as f64 * dz, lo + (i as f64 + 1.0) * dz);
            let zm = 0.5 * (z0 + z1);
            i_y += self.strip((0.0, 1.0), z0, z1).0 * (zm - zc).powi(2);
        }
        let _ = sy;
        let zt = (zc - lo).max(1e-9);
        let m_cr = (self.c.fctm + n_ed / a) * i_y / zt;
        (m_cr, m_cr / (self.c.ecm * i_y) * 1000.0)
    }
    /// 🔻 Pure tension resistance (concrete cracked).
    pub fn n_tension(&self) -> f64 {
        -self.bars.iter().map(|b| b.a).sum::<f64>() * self.fyd - self.tendons.iter().map(|t| t.a).sum::<f64>() * self.fp_d
    }
    /// 🔺 Pure compression resistance (ε_c = ε_c2 over the whole section).
    pub fn n_compression(&self) -> f64 {
        self.state(0.0, 1.0e7 * 1.0e3).0
    }
    /// 🧮 Neutral axis depth giving the axial force `n_ed`, `None` outside the resistance range.
    pub fn solve_x(&self, alpha: f64, n_ed: f64) -> Option<f64> {
        let n = (alpha.cos(), alpha.sin());
        let (lo_e, hi_e) = self.extent(n);
        let h = hi_e - lo_e;
        let (mut lo, mut hi) = (1e-3 * h, 1e4 * h);
        let (nlo, nhi) = (self.state(alpha, lo).0, self.state(alpha, hi).0);
        if n_ed < nlo - 1e-9 * nlo.abs() - 1.0 || n_ed > nhi + 1e-9 * nhi.abs() + 1.0 { return None; }
        for _ in 0..70 {
            let mid = (lo * hi).sqrt();
            if self.state(alpha, mid).0 < n_ed { lo = mid } else { hi = mid }
        }
        Some((lo * hi).sqrt())
    }
    /// 💪 Uniaxial resistance M_Rd (N·mm) about the y axis (compression at +z) for `n_ed`.
    pub fn m_rd_y(&self, n_ed: f64) -> Option<(f64, f64)> {
        let x = self.solve_x(std::f64::consts::FRAC_PI_2, n_ed)?;
        Some((self.state(std::f64::consts::FRAC_PI_2, x).1, x))
    }
    /// 🗺 Interaction curve points (N, My) for the compressed side +z.
    pub fn interaction(&self, points: usize) -> Vec<(f64, f64, f64)> {
        let a = std::f64::consts::FRAC_PI_2;
        let (lo, hi) = self.extent((0.0, 1.0));
        let h = hi - lo;
        let mut v = vec![(self.n_tension(), 0.0, 0.0)];
        for i in 0..points {
            let x = h * 0.02 * (100.0f64 / 0.02).powf(i as f64 / (points as f64 - 1.0));
            let s = self.state(a, x);
            v.push((s.0, s.1, x));
        }
        v
    }
    /// 🌐 Constant-N slice of the biaxial surface: (My, Mz, angle) per neutral axis direction, step in degrees.
    pub fn slice(&self, n_ed: f64, step_deg: f64) -> Vec<(f64, f64, f64)> {
        let mut out = Vec::new();
        let mut a = 0.0;
        while a < 360.0 - 1e-9 {
            let r = a.to_radians();
            if let Some(x) = self.solve_x(r, n_ed) { let s = self.state(r, x); out.push((s.1, s.2, a)); }
            a += step_deg;
        }
        out
    }
    /// 🎯 Utilisation η = |M_Ed| / |M_Rd(direction)| on the slice at N_Ed.
    pub fn biaxial_eta(&self, n_ed: f64, my: f64, mz: f64, step_deg: f64) -> Option<f64> {
        let curve = self.slice(n_ed, step_deg);
        if curve.len() < 8 { return None; }
        let m = (my * my + mz * mz).sqrt();
        let dir = (my / m.max(1e-12), mz / m.max(1e-12));
        let n = curve.len();
        for i in 0..n {
            let p0 = (curve[i].0, curve[i].1);
            let p1 = (curve[(i + 1) % n].0, curve[(i + 1) % n].1);
            let e = (p1.0 - p0.0, p1.1 - p0.1);
            let den = e.0 * dir.1 - e.1 * dir.0;
            if den.abs() < 1e-12 { continue; }
            let t = -(p0.0 * dir.1 - p0.1 * dir.0) / den;
            if !(0.0..=1.0).contains(&t) { continue; }
            let s = (p0.0 + t * e.0) * dir.0 + (p0.1 + t * e.1) * dir.1;
            if s > 0.0 { return Some(m / s); }
        }
        None
    }
}

/// 🏗 Member section (explicit outline or rectangle b × h) with the longitudinal layers distributed over the chord of the section at their height (top layers at cover + φ/2 below the top fibre, other layers with their centroid at the effective depth below it), SI input (m, Pa) converted to mm and MPa; `accidental` selects the accidental material factors.
pub fn member_section(m: &RcMember, f_ck_pa: f64, f_yk_pa: f64, es_pa: f64, annex: AnnexChoice, accidental: bool) -> Sec {
    let (b, h, d) = (m.width * 1000.0, m.height * 1000.0, m.effective_depth * 1000.0);
    let fck = f_ck_pa / 1.0e6;
    let mut s = Sec::rect(b, h, conc(fck), f_cd(annex, fck, accidental), f_yd(annex, f_yk_pa / 1.0e6, accidental));
    s.es = es_pa / 1.0e6;
    if m.section_points.len() >= 3 {
        s.loops = vec![Loop { pts: m.outline().into_iter().map(|p| (p.0 * 1000.0, p.1 * 1000.0)).collect(), hole: false }];
    }
    let top = s.extent((0.0, 1.0)).1;
    for l in &m.longitudinal {
        let own: Vec<_> = m.bars.iter().filter(|b| b.layer_id == l.id).collect();
        if !own.is_empty() {
            for b in own {
                let phi_b = b.diameter * 1000.0;
                let (by, bz) = m.bar_position(b);
                s.bars.push(Bar { y: by * 1000.0, z: bz * 1000.0, a: std::f64::consts::PI * phi_b * phi_b / 4.0 });
            }
            continue;
        }
        let phi = l.diameter * 1000.0;
        let n = l.count.max(1) as usize;
        let a = std::f64::consts::PI * phi * phi / 4.0;
        let z = if l.position == semio_s_artifact_norm_en1992_model::BarPosition::Top { top - (m.cover * 1000.0 + phi / 2.0) } else { top - d };
        let inset = m.cover * 1000.0 + phi / 2.0;
        let (y0, y1) = s.chord(z).unwrap_or((-b / 2.0, b / 2.0));
        let (lo, hi) = if y1 - y0 > 2.0 * inset { (y0 + inset, y1 - inset) } else { (0.5 * (y0 + y1), 0.5 * (y0 + y1)) };
        for i in 0..n {
            let y = if n == 1 { 0.5 * (lo + hi) } else { lo + (hi - lo) * i as f64 / (n - 1) as f64 };
            s.bars.push(Bar { y, z, a });
        }
    }
    s
}

/// 🧷 [`member_section`] plus the hardening k and ε_ud = 0.9 ε_uk of the reinforcement grade and the tendon of the member (eccentricity at midspan from the control points, pre-strain from P_m,∞ and E_p = 195 GPa, design strengths from the prestressing steel of the document).
pub fn member_section_in(doc: &En1992Snapshot, m: &RcMember, f_ck_pa: f64, f_yk_pa: f64, es_pa: f64, annex: AnnexChoice, accidental: bool) -> Sec {
    let mut s = member_section(m, f_ck_pa, f_yk_pa, es_pa, annex, accidental);
    if let Some(grade) = doc.reinforcement(&m.reinforcement_grade_id) {
        s.k = grade.k;
        s.eps_ud = 0.9 * grade.eps_uk * 1000.0;
    }
    let (Some(p), Some(steel)) = (m.prestress.as_ref().filter(|p| p.area > 0.0), doc.prestress_steels.iter().find(|x| x.id == m.prestress_steel_id)) else { return s };
    let gs = ndp(annex, if accidental { "ndp.en1992.gamma-s-acc" } else { "ndp.en1992.gamma-s" });
    let ecc = m.tendon_eccentricity_mid().unwrap_or(p.eccentricity);
    s.ep = 195000.0;
    s.fp_d = steel.f_p0_1k / 1.0e6 / gs;
    s.fpt_d = steel.f_pk / 1.0e6 / gs;
    let sigma_inf = p.force * (1.0 - p.loss_ratio.clamp(0.0, 1.0)) / p.area / 1.0e6;
    s.tendons.push(Tendon { y: 0.0, z: -ecc * 1000.0, a: p.area * 1.0e6, pre: sigma_inf / s.ep * 1000.0 });
    s
}

/// 💪 M_Rd (N·m) of the member section for N_Ed in N (tension positive, the snapshot convention) by strain compatibility, 0 when N_Ed is outside the resistance.
pub fn member_m_rd_nm(s: &Sec, n_ed_tension_positive: f64) -> f64 {
    s.m_rd_y(-n_ed_tension_positive / 1.0).map(|r| r.0 / 1000.0).unwrap_or(0.0)
}

/// 🧮 Factor k ≥ 1 on the tension steel area so that M_Rd reaches `m_ed` (N·m), `None` when even 20× is insufficient.
pub fn required_tension_scale(s: &Sec, n_ed_tension_positive: f64, m_ed: f64) -> Option<f64> {
    let mut hi = 20.0;
    let ok = |k: f64| { let mut t = s.clone(); for b in t.bars.iter_mut().filter(|b| b.z < 0.0) { b.a *= k; } member_m_rd_nm(&t, n_ed_tension_positive) >= m_ed };
    if !ok(hi) { return None; }
    let mut lo = 1.0;
    if ok(lo) { return Some(1.0); }
    for _ in 0..24 { let mid = 0.5 * (lo + hi); if ok(mid) { hi = mid } else { lo = mid } }
    Some(hi)
}

/// 🧮 Section from the case: rectangle b × h or polygon lists, bars and tendons; origin at the gross centroid.
pub fn section_of(c: &Case) -> Result<Sec, String> {
    let fck = c.pos("fck")?;
    let g = conc(fck);
    let acc = c.so("situation", "persistent") == "accidental";
    let fcd = f_cd(c.annex, fck, acc);
    let fyk = c.o("fyk", 500.0);
    let fyd = f_yd(c.annex, fyk, acc);
    let mut s = if let (Ok(py), Ok(pz)) = (c.l("poly_y"), c.l("poly_z")) {
        let pts: Vec<(f64, f64)> = py.iter().copied().zip(pz.iter().copied()).collect();
        if pts.len() < 3 { return Err("polygon needs three points".into()); }
        let mut loops = vec![Loop { pts, hole: false }];
        if let (Ok(hy), Ok(hz)) = (c.l("hole_y"), c.l("hole_z")) {
            loops.push(Loop { pts: hy.iter().copied().zip(hz.iter().copied()).collect(), hole: true });
        }
        let mut s = Sec::rect(1.0, 1.0, g, fcd, fyd);
        s.loops = loops;
        s
    } else {
        Sec::rect(c.pos("b")?, c.pos("h")?, g, fcd, fyd)
    };
    s.es = c.o("es", 200000.0);
    s.k = c.o("k", 1.08);
    s.eps_ud = 0.9 * c.o("eps_uk", 50.0);
    if let (Ok(y), Ok(z), Ok(a)) = (c.l("bar_y"), c.l("bar_z"), c.l("bar_a")) {
        s.bars = (0..y.len().min(z.len()).min(a.len())).map(|i| Bar { y: y[i], z: z[i], a: a[i] }).collect();
    }
    if let (Ok(y), Ok(z), Ok(a)) = (c.l("t_y"), c.l("t_z"), c.l("t_a")) {
        let pre = c.l("t_pre").map(|p| p.to_vec()).unwrap_or_else(|_| vec![0.0; y.len()]);
        s.tendons = (0..y.len().min(z.len()).min(a.len())).map(|i| Tendon { y: y[i], z: z[i], a: a[i], pre: pre.get(i).copied().unwrap_or(0.0) }).collect();
        let fp01k = c.pos("fp01k")?;
        let fpk = c.o("fpk", fp01k * 1.1);
        let gs = ndp(c.annex, "ndp.en1992.gamma-s");
        s.fp_d = fp01k / gs;
        s.fpt_d = fpk / gs;
        s.ep = c.o("ep", 195000.0);
        s.eps_ud_p = c.o("eps_ud_p", 20.0);
    }
    Ok(s.centred())
}

pub fn s01(c: &Case) -> R {
    let s = section_of(c)?;
    let alpha = c.o("alpha_deg", 90.0).to_radians();
    let x = c.pos("x")?;
    let (n, my, mz) = s.state(alpha, x);
    Ok(vec![
        Outcome::n("n", "Axial force N(x)", "Normalkraft N(x)", n, format!("neutral axis depth {} mm", f(x))),
        Outcome::nmm("my", "Moment M_y(x)", "Moment M_y(x)", my, "∫σ z dA".into()),
        Outcome::nmm("mz", "Moment M_z(x)", "Moment M_z(x)", mz, "−∫σ y dA".into()),
    ])
}

pub fn s02(c: &Case) -> R {
    let s = section_of(c)?;
    let (a, _, _) = s.gross();
    let as_ = s.bars.iter().map(|b| b.a).sum::<f64>() + s.tendons.iter().map(|t| t.a).sum::<f64>();
    let n0 = a * s.fcd + as_ * s.fyd;
    let nt = s.n_tension();
    let mut v = vec![
        Outcome::n("n_rd0", "Pure compression N_Rd,0", "Zentrische Druckkraft N_Rd,0", n0, format!("A_c f_cd + A_s f_yd, A_c = {}", f(a))),
        Outcome::n("n_rdt", "Pure tension N_Rd,t", "Zentrische Zugkraft N_Rd,t", nt, "−A_s f_yd".into()),
        Outcome::n("n_rd0_integrated", "N_Rd,0 by integration", "N_Rd,0 durch Integration", s.n_compression(), "ε_c = ε_c2 over the section".into()),
    ];
    if let Some(ned) = c.opt("n_ed") {
        v.push(Outcome::force("util", "Axial utilisation", "Normalkraftausnutzung", ned.max(0.0), n0, "N_Ed ≤ N_Rd,0".into()));
        if ned < 0.0 { v.push(Outcome::force("util_t", "Tension utilisation", "Zugausnutzung", -ned, -nt, "|N_Ed| ≤ |N_Rd,t|".into())); }
    }
    Ok(v)
}

pub fn s03(c: &Case) -> R {
    let (b, d, fck) = (c.pos("b")?, c.pos("d")?, c.pos("fck")?);
    let med = c.n("m_ed")?;
    let ned = c.o("n_ed", 0.0);
    let g = conc(fck);
    let fcd = f_cd(c.annex, fck, false);
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let es = c.o("es", 200000.0);
    let (lam, eta) = lambda_eta(fck);
    let zs1 = if ned != 0.0 { d - c.pos("h")? / 2.0 } else { 0.0 };
    let meds = med + ned * zs1;
    let xi_bal = g.eps_cu2 / (g.eps_cu2 + fyd / es * 1000.0);
    let xi_lim = c.o("xi_lim", xi_bal);
    let mu = meds / (b * d * d * fcd);
    if mu > 0.5 * eta { return Err("μ_Eds above its maximum: section cannot carry the moment".into()); }
    let a_lim = lam * xi_lim;
    let mu_lim = eta * a_lim * (1.0 - a_lim / 2.0);
    let as1;
    let as2;
    let z;
    let xi;
    let eps_s1;
    if mu <= mu_lim {
        let alpha = 1.0 - (1.0 - 2.0 * mu / eta).sqrt();
        as1 = (eta * alpha * b * d * fcd - ned) / fyd;
        as2 = 0.0;
        z = d * (1.0 - 0.5 * alpha);
        xi = alpha / lam;
        eps_s1 = g.eps_cu2 * (1.0 - xi) / xi;
    } else {
        let d2 = c.pos("d2")?;
        if d2 >= xi_lim * d { return Err("compression steel d2 lies below the neutral axis at the strain limit".into()); }
        let dm = meds - mu_lim * b * d * d * fcd;
        let eps_s2 = g.eps_cu2 * (xi_lim * d - d2) / (xi_lim * d);
        let sig2 = (es * eps_s2 / 1000.0).min(fyd);
        as2 = dm / ((d - d2) * sig2);
        as1 = (eta * a_lim * b * d * fcd + dm / (d - d2) - ned) / fyd;
        z = d * (1.0 - 0.5 * a_lim);
        xi = xi_lim;
        eps_s1 = g.eps_cu2 * (1.0 - xi) / xi;
    }
    let mut v = vec![
        Outcome::mm2("as1", "Required tension steel A_s1", "Erforderliche Zugbewehrung A_s1", as1, format!("μ = {}, λ = {}, η = {}", f(mu), f(lam), f(eta))),
        Outcome::mm2("as2", "Required compression steel A_s2", "Erforderliche Druckbewehrung A_s2", as2, format!("μ_lim = {}, ξ_lim = {}", f(mu_lim), f(xi_lim))),
        Outcome::mm("z", "Lever arm z", "Hebelarm z", z, "z = d (1 − α/2)".into()),
        Outcome::num("xi", "ξ = x/d", "ξ = x/d", xi, "α/λ".into()),
        Outcome::num("eps_s1", "Steel strain ε_s1 (permille)", "Stahldehnung ε_s1 (Promille)", eps_s1, "ε_cu2 (1−ξ)/ξ".into()),
    ];
    if let Some(prov) = c.opt("as_provided") {
        v.push(Outcome::area("util", "Reinforcement utilisation A_s,req/A_s,prov", "Bewehrungsausnutzung A_s,erf/A_s,vorh", as1, prov, "A_s,req ≤ A_s,prov".into()));
    }
    Ok(v)
}

pub fn s04(c: &Case) -> R {
    let s = section_of(c)?;
    let ned = c.n("n_ed")?;
    let n_pts = c.o("points", 24.0) as usize;
    let mut v = Vec::new();
    match s.m_rd_y(ned) {
        Some((m, x)) => {
            v.push(Outcome::nmm("m_rd", "Resistance M_Rd(N_Ed)", "Tragmoment M_Rd(N_Ed)", m, format!("neutral axis x = {} mm", f(x))));
            v.push(Outcome::mm("x", "Neutral axis depth x", "Nulllinientiefe x", x, "solved from N(x) = N_Ed".into()));
            if let Some(med) = c.opt("m_ed") { v.push(Outcome::moment("util", "Bending utilisation", "Biegeausnutzung", med.abs(), m, "M_Ed ≤ M_Rd(N_Ed)".into())); }
        }
        None => v.push(Outcome::force("util_n", "N_Ed outside the resistance range", "N_Ed außerhalb des Tragbereichs", ned.abs(), 0.0, "η = ∞".into())),
    }
    for (i, p) in s.interaction(n_pts).iter().enumerate() {
        v.push(Outcome::nmm(&format!("curve_m_{i}"), &format!("Interaction point {i}: M"), &format!("Interaktionspunkt {i}: M"), p.1, format!("N = {}", f(p.0))));
        v.push(Outcome::n(&format!("curve_n_{i}"), &format!("Interaction point {i}: N"), &format!("Interaktionspunkt {i}: N"), p.0, format!("x = {}", f(p.2))));
    }
    Ok(v)
}

/// 📐 Exponent a of Eq. 5.39 by linear interpolation.
pub fn biaxial_exponent(n_ratio: f64) -> f64 {
    interp(&[(0.1, 1.0), (0.7, 1.5), (1.0, 2.0)], n_ratio)
}

pub fn s05(c: &Case) -> R {
    let (nrd, mrdy, mrdz) = (c.pos("n_rd")?, c.pos("m_rdy")?, c.pos("m_rdz")?);
    let (ned, medy, medz) = (c.n("n_ed")?, c.n("m_edy")?, c.n("m_edz")?);
    let a = biaxial_exponent(ned / nrd);
    let lhs = (medz.abs() / mrdz).powf(a) + (medy.abs() / mrdy).powf(a);
    Ok(vec![Outcome::num("exponent", "Exponent a", "Exponent a", a, format!("N_Ed/N_Rd = {}", f(ned / nrd))), Outcome::ratio("util", "Biaxial utilisation (Eq. 5.39)", "Zweiachsige Ausnutzung (Gl. 5.39)", lhs, 1.0, "(M_Edz/M_Rdz)^a + (M_Edy/M_Rdy)^a ≤ 1".into())])
}

pub fn s06(c: &Case) -> R {
    let s = section_of(c)?;
    let (ned, my, mz) = (c.n("n_ed")?, c.n("m_edy")?, c.n("m_edz")?);
    let step = c.o("step_deg", 5.0);
    let curve = s.slice(ned, step);
    let eta = s.biaxial_eta(ned, my, mz, step).ok_or("N_Ed outside the biaxial resistance range")?;
    let (n, p) = (curve.len(), curve.iter().map(|c| (c.0 * c.0 + c.1 * c.1).sqrt()).fold(f64::MAX, f64::min));
    let mut v = vec![
        Outcome::ratio("util", "Biaxial utilisation η on the constant-N slice", "Zweiachsige Ausnutzung η auf dem Schnitt N = const.", eta, 1.0, format!("{n} slice points, minimum |M_Rd| = {}", f(p))),
        Outcome::nmm("m_rd_min", "Smallest |M_Rd| on the slice", "Kleinstes |M_Rd| auf dem Schnitt", p, "governing direction".into()),
    ];
    for (i, q) in curve.iter().enumerate() {
        v.push(Outcome::nmm(&format!("slice_my_{i}"), "Slice M_Rd,y", "Schnitt M_Rd,y", q.0, format!("α = {}°", f(q.2))));
        v.push(Outcome::nmm(&format!("slice_mz_{i}"), "Slice M_Rd,z", "Schnitt M_Rd,z", q.1, format!("α = {}°", f(q.2))));
    }
    Ok(v)
}

pub fn s07(c: &Case) -> R {
    let (bf, hf, bw, h) = (c.pos("bf")?, c.pos("hf")?, c.pos("bw")?, c.pos("h")?);
    let g = conc(c.pos("fck")?);
    let acc = c.so("situation", "persistent") == "accidental";
    let (fcd, fyd) = (f_cd(c.annex, g.fck, acc), f_yd(c.annex, c.o("fyk", 500.0), acc));
    let mut s = Sec::rect(1.0, 1.0, g, fcd, fyd);
    s.loops = vec![Loop { pts: vec![(-bw / 2.0, 0.0), (bw / 2.0, 0.0), (bw / 2.0, h - hf), (bf / 2.0, h - hf), (bf / 2.0, h), (-bf / 2.0, h), (-bf / 2.0, h - hf), (-bw / 2.0, h - hf)], hole: false }];
    let ds = c.pos("d")?;
    s.bars = vec![Bar { y: 0.0, z: h - ds, a: c.pos("as")? }];
    let s = s.centred();
    let zc = s.bars[0].z;
    let _ = zc;
    let ned = c.o("n_ed", 0.0);
    let (m, x) = s.m_rd_y(ned).ok_or("N_Ed outside range")?;
    let mut v = vec![Outcome::nmm("m_rd", "Resistance M_Rd", "Tragmoment M_Rd", m, format!("x = {}", f(x))), Outcome::num("x_above_flange", "Neutral axis below flange (1 = yes)", "Nulllinie unter Flansch (1 = ja)", if 0.8 * x > hf { 1.0 } else { 0.0 }, "λ x > h_f".into())];
    if let Some(med) = c.opt("m_ed") { v.push(Outcome::moment("util", "Bending utilisation", "Biegeausnutzung", med, m, "M_Ed ≤ M_Rd".into())); }
    Ok(v)
}

pub fn s08(c: &Case) -> R {
    let (h, ned) = (c.pos("h")?, c.n("n_ed")?);
    let e0 = (h / 30.0).max(20.0);
    let mut v = vec![Outcome::mm("e0", "Minimum eccentricity e_0", "Mindestausmitte e_0", e0, "max(h/30, 20 mm)".into()), Outcome::nmm("m_min", "Minimum moment N_Ed e_0", "Mindestmoment N_Ed e_0", ned.abs() * e0, "".into())];
    if let Some(med) = c.opt("m_ed") { v.push(Outcome::ratio_min("m_design", "Design moment against minimum", "Bemessungsmoment gegen Mindestmoment", med.abs(), ned.abs() * e0, "M_Ed ≥ N_Ed e_0".into())); }
    Ok(v)
}

pub fn s09(c: &Case) -> R {
    let (b, hw, fck, l0) = (c.pos("b")?, c.pos("hw")?, c.pos("fck")?, c.o("l0", 0.0));
    let fcd_pl = ndp(c.annex, "ndp.en1992.alpha-cc-pl") * fck / ndp(c.annex, "ndp.en1992.m-01.gamma-c-persistent");
    let e_tot = c.o("e_tot", 0.0);
    let phi = (1.14 * (1.0 - 2.0 * e_tot / hw) - 0.02 * l0 / hw).clamp(0.0, 1.0);
    let nrd = b * hw * fcd_pl * phi;
    let mut v = vec![Outcome::n("n_rd", "Plain concrete resistance N_Rd", "Tragkraft unbewehrter Beton N_Rd", nrd, format!("Φ = {}", f(phi)))];
    if let Some(ned) = c.opt("n_ed") { v.push(Outcome::force("util", "Plain concrete utilisation", "Ausnutzung unbewehrter Beton", ned, nrd, "N_Ed ≤ N_Rd".into())); }
    if c.de() { v.push(Outcome::length_min("hw_min", "Minimum thickness", "Mindestdicke", hw, 120.0, "h_w ≥ 120 mm cast in place".into())); }
    Ok(v)
}

pub fn s10(c: &Case) -> R {
    let s = section_of(c)?;
    if s.tendons.is_empty() { return Err("tendon lists t_y, t_z, t_a are required".into()); }
    let ned = c.o("n_ed", 0.0);
    let (m, x) = s.m_rd_y(ned).ok_or("N_Ed outside range")?;
    let mut v = vec![Outcome::nmm("m_rd", "Prestressed resistance M_Rd", "Tragmoment vorgespannt M_Rd", m, format!("x = {}", f(x)))];
    if let Some(med) = c.opt("m_ed") { v.push(Outcome::moment("util", "Bending utilisation", "Biegeausnutzung", med, m, "M_Ed ≤ M_Rd".into())); }
    Ok(v)
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("S-01", "DIN EN 1992-1-1", "6.1", "Strain-compatibility integrator", "Dehnungsebene", &["fck", "b", "h", "bar_y", "bar_z", "bar_a", "x", "alpha_deg"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], s01),
    entry_of("S-02", "DIN EN 1992-1-1", "6.1", "Pure axial resistance", "Zentrische Tragfähigkeit", &["fck", "b", "h", "bar_y", "bar_z", "bar_a", "n_ed"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], s02),
    entry_of("S-03", "DIN EN 1992-1-1", "6.1", "Rectangular section bending design", "Biegebemessung Rechteck", &["fck", "b", "d", "h", "d2", "m_ed", "n_ed", "xi_lim", "as_provided"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], s03),
    entry_of("S-04", "DIN EN 1992-1-1", "6.1", "Uniaxial N–M interaction", "M-N-Interaktion", &["fck", "b", "h", "bar_y", "bar_z", "bar_a", "n_ed", "m_ed", "points"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], s04),
    entry_of("S-05", "DIN EN 1992-1-1", "5.8.9", "Biaxial bending, simplified", "Zweiachsige Biegung vereinfacht", &["n_rd", "m_rdy", "m_rdz", "n_ed", "m_edy", "m_edz"], &[], s05),
    entry_of("S-06", "DIN EN 1992-1-1", "6.1", "Exact biaxial N–My–Mz interaction", "Exakte zweiachsige Interaktionsfläche", &["fck", "b", "h", "bar_y", "bar_z", "bar_a", "n_ed", "m_edy", "m_edz", "step_deg"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], s06),
    entry_of("S-07", "DIN EN 1992-1-1", "6.1", "T-section bending", "Plattenbalken", &["fck", "bf", "hf", "bw", "h", "d", "as", "n_ed", "m_ed"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], s07),
    entry_of("S-08", "DIN EN 1992-1-1", "6.1(4)", "Minimum eccentricity", "Mindestausmitte", &["h", "n_ed", "m_ed"], &[], s08),
    entry_of("S-09", "DIN EN 1992-1-1", "12.6", "Plain concrete resistance", "Unbewehrter Beton", &["b", "hw", "fck", "l0", "e_tot", "n_ed"], &["ndp.en1992.alpha-cc-pl", "ndp.en1992.m-01.gamma-c-persistent"], s09),
    entry_of("S-10", "DIN EN 1992-1-1", "6.1(6)", "Prestressed section bending", "Vorgespannter Querschnitt", &["fck", "b", "h", "t_y", "t_z", "t_a", "t_pre", "fp01k", "fpk", "n_ed", "m_ed"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], s10),
    entry_of("S-11", "DIN EN 1992-1-1", "5.8.7", "Moment-curvature of the cracked section", "Moment-Krümmung des gerissenen Querschnitts", &["fck", "b", "h", "poly_y", "poly_z", "bar_y", "bar_z", "bar_a", "fyk", "eps_uk", "k", "n_ed", "kappa", "points"], &[], s11),
];

/// 📉 S-11 (EN 1992-1-1 5.8.7 and 7.4): moment-curvature M(κ) of the cracked section at N_Ed and the cracking moment; `kappa` (1/m) lists the evaluated curvatures, otherwise a ladder up to the ultimate strain.
pub fn s11(c: &Case) -> R {
    let s = section_of(c)?;
    let ned = c.n("n_ed")?;
    let kappas: Vec<f64> = match c.l("kappa") {
        Ok(k) => k.to_vec(),
        Err(_) => s.moment_curvature(ned, c.o("points", 24.0) as usize, false).into_iter().skip(1).map(|p| p.0).collect(),
    };
    let mut v = Vec::new();
    for (i, k) in kappas.iter().enumerate() {
        if let Some(m) = s.moment_at(*k, ned, false) {
            v.push(Outcome::nmm(&format!("m_{i}"), &format!("Moment at κ{i}"), &format!("Moment bei κ{i}"), m, format!("κ = {} 1/m", f(*k))));
        }
    }
    let (m_cr, k_cr) = s.cracking(ned);
    v.push(Outcome::nmm("m_cr", "Cracking moment M_cr", "Rissmoment M_cr", m_cr, "(f_ctm + N/A) I/z_t".into()));
    v.push(Outcome::num("k_cr", "Cracking curvature κ_cr (1/m)", "Rissverkrümmung κ_cr (1/m)", k_cr, "M_cr/(E_cm I)".into()));
    Ok(v)
}
