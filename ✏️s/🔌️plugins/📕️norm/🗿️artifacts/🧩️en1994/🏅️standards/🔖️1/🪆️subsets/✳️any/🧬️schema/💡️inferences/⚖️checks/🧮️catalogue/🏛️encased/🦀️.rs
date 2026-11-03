//! 🏛️ Concrete-encased and partially encased I-section columns: the N–M interaction polygon A, C, D, B of EN 1994-1-1, 6.7.3.2 (Fig. 6.19) from the true cross-section by plastic strip integration, so the axes of the polygon are consistent (review finding 31b).

const STRIPS: usize = 800;

/// 🧱️ Encasement kind: concrete around the whole I-section, or only between its flanges.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encasement {
    Full,
    BetweenFlanges,
}

/// 📐️ Geometry (m, m²) of the I-section with its concrete and longitudinal bars, bending about the strong axis with the depth `h_c` along the bending plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EncasedI {
    pub kind: Encasement,
    pub b_c: f64,
    pub h_c: f64,
    pub h: f64,
    pub b: f64,
    pub tw: f64,
    pub tf: f64,
    pub as_m2: f64,
    pub cover: f64,
}

/// 💪️ Design strengths [Pa]: steel f_yd, concrete f_cd including the 0.85 factor, bars f_sd.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Strengths {
    pub fyd: f64,
    pub fcd: f64,
    pub fsd: f64,
}

impl EncasedI {
    /// 📐️ Steel area of the I-section.
    pub fn steel_area(&self) -> f64 {
        2.0 * self.b * self.tf + (self.h - 2.0 * self.tf).max(0.0) * self.tw
    }

    fn concrete_extent(&self) -> (f64, f64) {
        match self.kind {
            Encasement::Full => (self.b_c, self.h_c),
            Encasement::BetweenFlanges => (self.b, self.h),
        }
    }

    fn concrete_gross(&self) -> f64 {
        match self.kind {
            Encasement::Full => self.b_c * self.h_c - self.steel_area(),
            Encasement::BetweenFlanges => (self.b - self.tw).max(0.0) * (self.h - 2.0 * self.tf).max(0.0),
        }
    }

    /// 📐️ Net concrete area (gross minus steel and bars).
    pub fn concrete_area(&self) -> f64 {
        (self.concrete_gross() - self.as_m2).max(0.0)
    }

    fn bar_level(&self) -> f64 {
        let (_, h) = self.concrete_extent();
        let limit = match self.kind {
            Encasement::Full => h / 2.0,
            Encasement::BetweenFlanges => h / 2.0 - self.tf,
        };
        (h / 2.0 - self.cover).clamp(0.0, limit.max(0.0))
    }

    fn widths(&self, y: f64) -> (f64, f64) {
        let (_, h_ext) = self.concrete_extent();
        let in_flange = y.abs() >= self.h / 2.0 - self.tf && y.abs() <= self.h / 2.0;
        let in_web = y.abs() < self.h / 2.0 - self.tf;
        let steel = if in_flange { self.b } else if in_web { self.tw } else { 0.0 };
        let gross = if y.abs() <= h_ext / 2.0 { self.concrete_extent().0 } else { 0.0 };
        let concrete = match self.kind {
            Encasement::Full => (gross - steel).max(0.0),
            Encasement::BetweenFlanges => if in_web { (self.b - self.tw).max(0.0) } else { 0.0 },
        };
        (steel, concrete)
    }

    /// 🧮️ Resultant `(N, M)` of the plastic stress blocks with the neutral axis at `y_na` above the centroid (compression above, moments about the centroid, N and M positive for compression and counter-clockwise lever arms).
    pub fn resultant(&self, s: &Strengths, y_na: f64) -> (f64, f64) {
        let (_, h_ext) = self.concrete_extent();
        let top = h_ext.max(self.h) / 2.0;
        let dy = 2.0 * top / STRIPS as f64;
        let bar_area = self.as_m2 / 2.0;
        let net = self.concrete_area() / self.concrete_gross().max(1e-12);
        let (mut n, mut m) = (0.0, 0.0);
        for k in 0..STRIPS {
            let y = -top + (k as f64 + 0.5) * dy;
            let (steel, concrete) = self.widths(y);
            let compressed = y > y_na;
            let f = (if compressed { steel * s.fyd + concrete * net * s.fcd } else { -steel * s.fyd }) * dy;
            n += f;
            m += f * y;
        }
        let level = self.bar_level();
        for y in [level, -level] {
            let f = bar_area * s.fsd * if y > y_na { 1.0 } else { -1.0 };
            n += f;
            m += f * y;
        }
        (n, m)
    }

    /// 🌡️ The section as strips for the strain-compatibility reference curve.
    pub fn strips(&self, s: &Strengths) -> super::strain_curve::Strips {
        let top = self.concrete_extent().1.max(self.h) / 2.0;
        let dy = 2.0 * top / STRIPS as f64;
        let net = self.concrete_area() / self.concrete_gross().max(1e-12);
        let (steel, concrete): (Vec<f64>, Vec<f64>) = (0..STRIPS)
            .map(|k| {
                let (steel, concrete) = self.widths(-top + (k as f64 + 0.5) * dy);
                (steel, concrete * net)
            })
            .unzip();
        let level = self.bar_level();
        super::strain_curve::Strips { top, steel, concrete, bars: [(level, self.as_m2 / 2.0), (-level, self.as_m2 / 2.0)], fyd: s.fyd, fcd: s.fcd, fsd: s.fsd }
    }

    /// 📐️ Plastic resistance N_pl,Rd = A_a·f_yd + A_c·f_cd + A_s·f_sd.
    pub fn n_pl(&self, s: &Strengths) -> f64 {
        self.steel_area() * s.fyd + self.concrete_area() * s.fcd + self.as_m2 * s.fsd
    }

    /// 📐️ Concrete resistance N_pm,Rd = A_c·f_cd.
    pub fn n_pm(&self, s: &Strengths) -> f64 {
        self.concrete_area() * s.fcd
    }
}

/// 🏛️ Polygon vertices A, C, D, B as `(N, M)` [N, N·m] from the true geometry: A `(N_pl, 0)`, C `(N_pm, M_pl)`, D `(N_pm/2, M_max)` and B `(0, M_pl)`.
pub fn interaction_polygon(g: &EncasedI, s: &Strengths) -> [(f64, f64); 4] {
    let top = g.concrete_extent().1.max(g.h) / 2.0;
    let (mut lo, mut hi) = (-top, top);
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if g.resultant(s, mid).0 > 0.0 { lo = mid } else { hi = mid }
    }
    let b = g.resultant(s, 0.5 * (lo + hi));
    let d = g.resultant(s, 0.0);
    [(g.n_pl(s), 0.0), (g.n_pm(s), b.1), (0.5 * g.n_pm(s), d.1), (0.0, b.1)]
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
