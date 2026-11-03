//! 🔁️ EN 1994-2 6.8 fatigue: the S–N curves of the three detail categories (steel detail, headed stud in shear, reinforcing steel), the endurance and the Palmgren–Miner damage of an equivalent constant stress range, and the distribution of the stud stress range over the stud rows. The curve constants stay unverified (`ndp.en1994.2.rebar-fatigue-range`); the independent oracle is `🧰️scripts/en1994-fatigue-oracle.py`.

use super::constants::{gamma_s_fat, REBAR_FATIGUE_DELTA_SIGMA_RSK, REBAR_FATIGUE_K2, REBAR_FATIGUE_N_STAR};

/// 🔁️ One S–N line `Δ_R(N) = Δ_c·(N_ref/N)^(1/m)/γ` of a detail category.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Curve {
    pub delta_c_pa: f64,
    pub n_ref: f64,
    pub m: f64,
    pub gamma: f64,
}

impl Curve {
    /// 🔁️ Design stress range resistance Δ_R [Pa] at `n` cycles.
    pub fn resistance_pa(&self, n: f64) -> f64 {
        self.delta_c_pa * (self.n_ref / n.max(1.0)).powf(1.0 / self.m) / self.gamma
    }

    /// 🔁️ Number of cycles to failure N_R of the constant stress range `delta_pa`.
    pub fn endurance(&self, delta_pa: f64) -> f64 {
        self.n_ref * (self.delta_c_pa / (self.gamma * delta_pa.max(1.0))).powf(self.m)
    }

    /// 🔁️ Damage D = n/N_R(Δ) of `n` cycles of the stress range `delta_pa` (Palmgren–Miner, one block).
    pub fn damage(&self, delta_pa: f64, n: f64) -> f64 {
        n.max(0.0) / self.endurance(delta_pa)
    }

    /// 🔁️ Utilisation Δ/Δ_R(n); D = utilisation^m.
    pub fn utilisation(&self, delta_pa: f64, n: f64) -> f64 {
        delta_pa / self.resistance_pa(n)
    }
}

/// 🔁️ The reinforcing-steel curve (EN 1992-1-1 Table 6.3N straight bars: Δσ_Rsk = 162.5 MPa at N* = 10⁶, k₂ = 9, γ_s,fat = 1.15; unverified `ndp.en1994.2.rebar-fatigue-range`).
pub fn rebar_curve() -> Curve {
    Curve { delta_c_pa: REBAR_FATIGUE_DELTA_SIGMA_RSK, n_ref: REBAR_FATIGUE_N_STAR, m: REBAR_FATIGUE_K2, gamma: gamma_s_fat() }
}

/// 🔩 One stud row: its position along the beam, its stress range and its damage.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Row {
    pub x_m: f64,
    pub share: f64,
    pub delta_pa: f64,
    pub damage: f64,
}

/// 🔩 The stud rows of `shares` (`(x, |V(x)|/|V|max)` of the elastic shear flow): the stress range scales with the share, the damage with `share^m`.
pub fn stud_rows(curve: &Curve, peak_delta_pa: f64, n: f64, shares: &[(f64, f64)]) -> Vec<Row> {
    shares.iter().map(|&(x_m, share)| Row { x_m, share, delta_pa: peak_delta_pa * share, damage: curve.damage(peak_delta_pa * share, n) }).collect()
}

/// 🔩 Index of the row with the largest damage.
pub fn governing_row(rows: &[Row]) -> Option<usize> {
    rows.iter().enumerate().max_by(|a, b| a.1.damage.total_cmp(&b.1.damage)).map(|(index, _)| index)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
