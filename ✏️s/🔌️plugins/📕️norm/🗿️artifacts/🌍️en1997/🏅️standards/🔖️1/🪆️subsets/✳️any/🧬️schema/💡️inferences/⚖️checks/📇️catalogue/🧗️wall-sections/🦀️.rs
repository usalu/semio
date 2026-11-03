//! 🧗️ Retaining structures of EN 1997-1 section 9 beyond the plain gravity wall: the gravity or cantilever wall with a heel (RW-11, virtual back through the heel end) and the embedded wall in soil and water with free earth support, Blum fixed earth support or cantilever action (RW-12, EAB, DIN 4085). See [`super`].

use super::*;

const BLUM_ID: &str = "ndp.en1997.9.blum-embedment-increase";
const STEPS: usize = 4000;

/// 📐️ Geometry, forces and arms of a wall with a heel per metre run, taken about the toe corner of the base: the weights W and their arm, the active thrust of the virtual back through the heel end (height H = h + t_base) split into a permanent and a surcharge part, and the vertical thrust component.
#[derive(Clone, Copy, Debug)]
pub struct HeelWall {
    pub heel: f64,
    pub w: f64,
    pub x_w: f64,
    pub height: f64,
    pub ka: f64,
    pub eah_g: f64,
    pub eah_q: f64,
    pub eav: f64,
}

/// 🧱️ Forces of a wall with stem thickness `t_stem`, base width `b`, toe length `a_toe`, base slab thickness `t_base` and stem height `h` on a backfill of unit weight `gamma` and friction φ with wall friction δ on the virtual back; concrete weighs `gamma_c`, the soil over the toe `d_embed` deep is counted as weight only.
pub fn heel_wall(h: f64, t_base: f64, b: f64, a_toe: f64, t_stem: f64, gamma_c: f64, gamma: f64, phi_deg: f64, delta_deg: f64, q: f64, d_embed: f64) -> Result<HeelWall, String> {
    let heel = b - a_toe - t_stem;
    gate(heel > 0.0, "base width must exceed toe length plus stem thickness")?;
    let height = h + t_base;
    let ka = earth_pressure::ka_coulomb(phi_deg, delta_deg, 0.0, 0.0);
    let parts = [
        (t_stem * h * gamma_c, a_toe + t_stem / 2.0),
        (b * t_base * gamma_c, b / 2.0),
        (heel * h * gamma, b - heel / 2.0),
        (a_toe * d_embed * gamma, a_toe / 2.0),
    ];
    let w: f64 = parts.iter().map(|(force, _)| force).sum();
    let x_w = parts.iter().map(|(force, arm)| force * arm).sum::<f64>() / w;
    let (e_g, e_q) = (0.5 * ka * gamma * height * height, ka * q * height);
    let angle = rad(delta_deg);
    Ok(HeelWall { heel, w, x_w, height, ka, eah_g: e_g * angle.cos(), eah_q: e_q * angle.cos(), eav: (e_g + e_q) * angle.sin() })
}

fn rw_11(c: &Case) -> R {
    let (h, t_base, b) = (c.positive("h")?, c.positive("t_base")?, c.positive("b")?);
    let (a_toe, t_stem) = (c.o("a_toe", 0.0), c.positive("t_stem")?);
    let phi = c.positive("phi_deg")?;
    let wall = heel_wall(h, t_base, b, a_toe, t_stem, c.o("gamma_c", 25_000.0), c.positive("gamma")?, phi, c.o("delta_deg", 2.0 / 3.0 * phi), c.o("q", 0.0), c.o("d_toe", 0.0))?;
    let base = Case::new(&c.id, c.annex)
        .with_s("approach", c.so("approach", "da2"))
        .with_s("situation", c.so("situation", "bsP"))
        .with_n("b", b)
        .with_n("w", wall.w)
        .with_n("x_w", wall.x_w)
        .with_n("eah_g", wall.eah_g)
        .with_n("arm_g", wall.height / 3.0)
        .with_n("eah_q", wall.eah_q)
        .with_n("arm_q", wall.height / 2.0)
        .with_n("eav", wall.eav)
        .with_n("x_v", b)
        .with_n("phi_base_deg", c.o("phi_base_deg", phi));
    let mut out = evaluate_outcomes("RW-06", &base)?;
    if let Some(bearing) = c.opt("sigma_rd") {
        let sigma = out.iter().find(|o| o.key == "sigma_max").map_or(0.0, Outcome::value);
        out.insert(0, Outcome::pressure("bearing", "Maximum base pressure vs bearing pressure σ_R,d", "Maximale Sohlpressung gegen Sohlwiderstand σ_R,d", sigma, bearing));
    }
    out.extend([
        Outcome::metre("heel", "Heel length b − a − t", "Fersenlänge b − a − t", wall.heel),
        Outcome::newton("w", "Weight of wall and soil on heel and toe", "Gewicht von Wand und Boden auf Ferse und Spitze", wall.w),
        Outcome::metre("x_w", "Arm of the weight about the toe", "Hebelarm des Gewichts um die Spitze", wall.x_w),
        Outcome::number("ka", "Active coefficient K_a of the virtual back", "Aktiver Beiwert K_a der Ersatzwand", wall.ka),
        Outcome::newton("e_ah", "Horizontal thrust E_ah (permanent)", "Horizontale Erddruckkraft E_ah (ständig)", wall.eah_g),
        Outcome::newton("e_av", "Vertical thrust component E_av", "Vertikale Erddruckkomponente E_av", wall.eav),
    ]);
    Ok(out)
}

struct Profile {
    h: f64,
    gamma: f64,
    gamma_buoyant: f64,
    gamma_w: f64,
    z_w: f64,
    z_wf: f64,
    q: f64,
    kah: f64,
    kph: f64,
    gamma_g: f64,
    gamma_q: f64,
    gamma_re: f64,
}

impl Profile {
    fn stress_behind(&self, z: f64) -> f64 {
        if z <= self.z_w { self.gamma * z } else { self.gamma * self.z_w + self.gamma_buoyant * (z - self.z_w) }
    }
    fn stress_in_front(&self, z: f64) -> f64 {
        let zf = z - self.h;
        let dry = (self.z_wf - self.h).max(0.0);
        if zf <= dry { self.gamma * zf } else { self.gamma * dry + self.gamma_buoyant * (zf - dry) }
    }
    fn water(&self, z: f64) -> f64 {
        self.gamma_w * ((z - self.z_w).max(0.0) - (z - self.z_wf).max(0.0))
    }
    fn active(&self, z: f64) -> f64 {
        self.gamma_g * (self.kah * self.stress_behind(z) + self.water(z)) + self.gamma_q * self.kah * self.q
    }
    fn passive(&self, z: f64) -> f64 {
        if z <= self.h { 0.0 } else { self.kph * self.stress_in_front(z) / self.gamma_re }
    }
    fn net(&self, z: f64) -> f64 {
        self.active(z) - self.passive(z)
    }
    fn integrate(&self, from: f64, to: f64, weight: &dyn Fn(f64) -> f64) -> f64 {
        let dz = (to - from) / STEPS as f64;
        (0..STEPS).map(|i| {
            let z = from + (i as f64 + 0.5) * dz;
            self.net(z) * weight(z) * dz
        }).sum()
    }
    fn resisting(&self, from: f64, to: f64, weight: &dyn Fn(f64) -> f64) -> f64 {
        -self.integrate(from, to, weight)
    }
    fn first_zero_below_dredge(&self) -> Result<f64, String> {
        let limit = self.h + 4.0 * self.h + 5.0;
        let dz = 0.002;
        let mut z = self.h + dz;
        while z < limit {
            if self.net(z) <= 0.0 {
                let (mut lo, mut hi) = (z - dz, z);
                for _ in 0..60 {
                    let mid = 0.5 * (lo + hi);
                    if self.net(mid) > 0.0 { lo = mid } else { hi = mid }
                }
                return Ok(0.5 * (lo + hi));
            }
            z += dz;
        }
        Err("no depth of zero net pressure found below the dredge level".into())
    }
}

fn bisect(mut lo: f64, mut hi: f64, falling: &dyn Fn(f64) -> f64) -> Result<f64, String> {
    gate(falling(lo) > 0.0 && falling(hi) < 0.0, "no equilibrium embedment found")?;
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if falling(mid) > 0.0 { lo = mid } else { hi = mid }
    }
    Ok(0.5 * (lo + hi))
}

fn shear_moment(p: &Profile, depth: f64, supports: &[(f64, f64)]) -> f64 {
    let dz = depth / STEPS as f64;
    let (mut shear, mut moment, mut peak) = (0.0, 0.0, 0.0_f64);
    let mut pending: Vec<(f64, f64)> = supports.to_vec();
    pending.sort_by(|a, b| a.0.total_cmp(&b.0));
    pending.reverse();
    for i in 0..STEPS {
        let z = (i as f64 + 0.5) * dz;
        shear += p.net(z) * dz;
        while pending.last().is_some_and(|(at, _)| *at <= z) {
            shear -= pending.pop().map_or(0.0, |(_, force)| force);
        }
        moment += shear * dz;
        peak = peak.max(moment.abs());
    }
    peak
}

fn rw_12(c: &Case) -> R {
    let (h, gamma, phi) = (c.positive("h")?, c.positive("gamma")?, c.positive("phi_deg")?);
    let gw = c.gamma_w();
    let gamma_sat = c.o("gamma_sat", gamma);
    let p = c.factors();
    let phi_d = (rad(phi).tan() / p.gamma_phi).atan().to_degrees();
    let (kah, kph) = walls_and_anchors::horizontal_coefficients(phi_d, c.o("delta_a_deg", 0.0), c.o("delta_p_deg", 0.0));
    let kind = c.so("kind", "free_earth");
    let z_w = c.o("z_w", f64::INFINITY);
    let profile = Profile { h, gamma, gamma_buoyant: gamma_sat - gw, gamma_w: gw, z_w, z_wf: c.o("z_wf", z_w), q: c.o("q", 0.0), kah, kph, gamma_g: p.gamma_g, gamma_q: p.gamma_q, gamma_re: p.gamma_r_e };
    gate(profile.gamma_buoyant > 0.0, "γ_sat must exceed γ_w")?;
    let blum = semio_s_plugin_norm_compliance::ndp_number(BLUM_ID, c.annex);
    let (hi, lo) = (4.0 * h + 5.0, 0.01);
    let u_toe = |t: f64| profile.water(h + t);
    let mut out = Vec::new();
    let required = match kind {
        "cantilever" => {
            let t0 = bisect(lo, hi, &|t| -profile.integrate(0.0, h + t, &|z| z - (h + t)))?;
            let m = shear_moment(&profile, h + t0, &[]);
            out.extend([Outcome::newton_metre("m_max_d", "Maximum design bending moment", "Maximales Bemessungsmoment", m), Outcome::metre("t0", "Equilibrium embedment t0", "Gleichgewichtseinbindung t0", t0)]);
            blum * t0
        }
        "fixed_earth" => {
            let a = c.positive("prop_depth")?;
            gate(a < h, "prop depth must lie above the dredge level")?;
            let z_y = profile.first_zero_below_dredge()?;
            let reaction = profile.integrate(0.0, z_y, &|z| z - a) / (z_y - a);
            let anchor = profile.integrate(0.0, z_y, &|_| 1.0) - reaction;
            let x = bisect(1.0e-3, hi, &|x| reaction * x - profile.resisting(z_y, z_y + x, &|z| x - (z - z_y)))?;
            let m = shear_moment(&profile, z_y, &[(a, anchor)]);
            out.extend([
                Outcome::newton_metre("m_max_d", "Maximum design bending moment", "Maximales Bemessungsmoment", m),
                Outcome::newton("prop_force", "Prop/anchor force A_d per metre", "Stützen-/Ankerkraft A_d je m", anchor),
                Outcome::newton("c_force", "Equivalent beam reaction C_d at the zero-moment point", "Ersatzkraft C_d am Momentennullpunkt", reaction),
                Outcome::metre("y", "Depth y of zero net pressure below the dredge level", "Tiefe y des Lastnullpunkts unter der Baugrubensohle", z_y - h),
                Outcome::metre("x", "Blum length x below the zero-pressure point", "Blum-Länge x unter dem Lastnullpunkt", x),
            ]);
            (z_y - h) + blum * x
        }
        _ => {
            let a = c.positive("prop_depth")?;
            gate(a < h, "prop depth must lie above the dredge level")?;
            let t0 = bisect(lo, hi, &|t| profile.integrate(0.0, h + t, &|z| z - a))?;
            let anchor = profile.integrate(0.0, h + t0, &|_| 1.0);
            let m = shear_moment(&profile, h + t0, &[(a, anchor)]);
            out.extend([
                Outcome::newton_metre("m_max_d", "Maximum design bending moment", "Maximales Bemessungsmoment", m),
                Outcome::metre("t0", "Equilibrium embedment t0", "Gleichgewichtseinbindung t0", t0),
                Outcome::newton("prop_force", "Prop/anchor force A_d per metre", "Stützen-/Ankerkraft A_d je m", anchor),
            ]);
            t0
        }
    };
    if let Some(provided) = c.opt("t_provided") {
        out.insert(0, Outcome::length("embedment", "Required embedment vs provided", "Erforderliche Einbindung gegen vorhandene", required, provided));
    }
    out.extend([
        Outcome::metre("t_required", "Required embedment", "Erforderliche Einbindetiefe", required),
        Outcome::pascal("u_net", "Net water pressure on the wall at the toe", "Resultierender Wasserdruck am Wandfuß", u_toe(required)),
        Outcome::metre("delta_h", "Water level difference across the wall", "Wasserspiegeldifferenz an der Wand", if z_w.is_finite() && profile.z_wf.is_finite() { (profile.z_wf - z_w).max(0.0) } else { 0.0 }),
    ]);
    Ok(out)
}

pub const ENTRIES: &[Entry] = &[
    Entry { id: "RW-11", part: "EN 1997-1", clause: "9.7", en: "Gravity and cantilever wall with a heel", de: "Schwergewichts- und Winkelstützwand mit Ferse", inputs: &["h", "t_base", "b", "a_toe", "t_stem", "gamma_c", "gamma", "phi_deg", "delta_deg", "q", "d_toe", "phi_base_deg", "sigma_rd", "approach", "situation"], eval: rw_11 },
    Entry { id: "RW-12", part: "EN 1997-1", clause: "9.7", en: "Embedded wall in soil and water, free, fixed earth support or cantilever", de: "Eingebettete Wand in Boden und Wasser, freie, volle Einspannung oder Kragwand", inputs: &["h", "gamma", "gamma_sat", "phi_deg", "delta_a_deg", "delta_p_deg", "q", "kind", "prop_depth", "z_w", "z_wf", "t_provided", "gamma_w", "approach", "situation"], eval: rw_12 },
];
