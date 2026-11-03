//! 🧱️ Wall plan of a storey: the stiffness centre and the torsional radii that follow from the positioned wall rows of `buildings[].members[]` (EN 1998-1 4.2.3.2, 4.3.3.2.4).

use super::cracked_stiffness;
use semio_s_artifact_norm_en1998_model::{En1998Member, En1998Storey, MemberKind};

/// 🧪 Ratio G/E of the shear to the Young modulus of an isotropic material, `1 / (2 (1 + ν))`; ν is the member's `wallPoissonRatio` (ν = 0.25 gives the 0.4 of EN 1996-1-1 3.7.3, ν = 0.2 the 1/2.4 of EN 1992-1-1; both values are entered data and unverified here).
pub fn shear_to_young_ratio(poisson_ratio: f64) -> f64 {
    1.0 / (2.0 * (1.0 + poisson_ratio))
}

/// 🧱 One positioned wall row: centre of the wall axis in the building plan, length, thickness, axis direction from the plan x axis, storey reference, material, Poisson's ratio and the factor on the gross stiffness (cracked or declared effective, EN 1998-1 4.3.1(6)).
#[derive(Clone, Debug, PartialEq)]
pub struct PlacedWall {
    pub member_id: String,
    pub centre_x_m: f64,
    pub centre_y_m: f64,
    pub length_m: f64,
    pub thickness_m: f64,
    pub direction_rad: f64,
    pub storey_id: String,
    pub material: semio_s_artifact_norm_en1998_model::StructuralMaterial,
    pub poisson_ratio: f64,
    pub stiffness_factor: f64,
}

impl PlacedWall {
    /// 🏬 Whether the wall counts in the storey row `storey_id`: a wall without a storey reference counts in every storey.
    pub fn in_storey(&self, storey_id: &str) -> bool {
        self.storey_id.is_empty() || self.storey_id == storey_id
    }
}

/// 📍 Start and end point of a wall row in the building plan, `(x0, y0, x1, y1)`.
pub fn axis_of(member: &En1998Member) -> (f64, f64, f64, f64) {
    let (s, c) = member.wall_direction_rad.sin_cos();
    (member.wall_start_x_m, member.wall_start_y_m, member.wall_start_x_m + member.wall_length_m * c, member.wall_start_y_m + member.wall_length_m * s)
}

/// 🧱 Whether a member row is a wall row with a plan axis: kind wall, positive length and thickness (masonry and reinforced concrete alike).
pub fn is_wall_row(member: &En1998Member) -> bool {
    member.role == MemberKind::Wall && member.wall_length_m > 0.0 && member.wall_thickness_m > 0.0
}

/// 🧱 The positioned wall rows among the member rows.
pub fn placed_walls_of(members: &[En1998Member]) -> Vec<PlacedWall> {
    members.iter().filter(|m| is_wall_row(m)).map(|m| {
        let (x0, y0, x1, y1) = axis_of(m);
        PlacedWall { member_id: m.id.clone(), centre_x_m: 0.5 * (x0 + x1), centre_y_m: 0.5 * (y0 + y1), length_m: m.wall_length_m, thickness_m: m.wall_thickness_m, direction_rad: m.wall_direction_rad, storey_id: m.wall_storey_id.clone(), material: m.material, poisson_ratio: m.wall_poisson_ratio, stiffness_factor: cracked_stiffness::member_factor(m) }
    }).collect()
}

/// 📏 Lateral stiffness of a wall along its axis in units of the Young modulus, `k / E = t / (4 (h/l)³ + (h/l) / (G/E))`: the cantilever of height `h` with bending `4 h³ / (E t l³)` and shear `h / (G t l)` flexibility, `shear_to_young = G/E` (see [`shear_to_young_ratio`]); 0 for a missing dimension or a non-positive `G/E`.
pub fn stiffness_per_modulus_m(length_m: f64, thickness_m: f64, height_m: f64, shear_to_young: f64) -> f64 {
    if length_m <= 0.0 || thickness_m <= 0.0 || height_m <= 0.0 || shear_to_young <= 0.0 {
        return 0.0;
    }
    let slenderness = height_m / length_m;
    thickness_m / (4.0 * slenderness.powi(3) + slenderness / shear_to_young)
}

/// 🌀 Stiffness centre, lateral stiffnesses and torsional radii of a storey from its walls.
#[derive(Clone, Debug, PartialEq)]
pub struct WallPlan {
    pub centre_of_stiffness_x_m: f64,
    pub centre_of_stiffness_y_m: f64,
    pub stiffness_x: f64,
    pub stiffness_y: f64,
    pub torsional_stiffness: f64,
    pub radius_x_m: f64,
    pub radius_y_m: f64,
    pub wall_count: usize,
    pub poisson_ratio_range: (f64, f64),
    pub stiffness_factor_range: (f64, f64),
}

impl WallPlan {
    /// 🧪 The Poisson's ratio of the walls as text: one value, or `lowest–highest` when the rows differ.
    pub fn poisson_ratio_text(&self) -> String {
        let (lo, hi) = self.poisson_ratio_range;
        if lo == hi { format!("{lo}") } else { format!("{lo}–{hi}") }
    }

    /// 🪨 The factors on the gross wall stiffness as percent text: one value, or `lowest–highest` when the rows differ.
    pub fn stiffness_factor_text(&self) -> String {
        let (lo, hi) = self.stiffness_factor_range;
        if lo == hi { format!("{:.0} %", lo * 100.0) } else { format!("{:.0}–{:.0} %", lo * 100.0, hi * 100.0) }
    }
}

/// 🌀 Rigid diaphragm assembly `K = Σ f k aᵀa` (`f` the cracked or effective stiffness factor of the wall, so walls with different factors move the stiffness centre) with `a = (cos α, sin α, xc sin α − yc cos α)` over the walls in `storey` of one material (their Young moduli are not known, so mixed materials give `None`), the stiffness centre where translation and rotation decouple and the radii `r_x = √(K_θ / K_y)`, `r_y = √(K_θ / K_x)` (EN 1998-1 4.2.3.2); `None` without walls, with parallel walls or when a direction has no stiffness. The wall's own torsional stiffness is neglected.
pub fn wall_plan_of_storey(walls: &[PlacedWall], storey: &En1998Storey) -> Option<WallPlan> {
    let own: Vec<&PlacedWall> = walls.iter().filter(|w| w.in_storey(&storey.id)).collect();
    let first = own.first()?;
    if own.iter().any(|w| w.material != first.material) {
        return None;
    }
    let mut k = [[0.0_f64; 3]; 3];
    for w in &own {
        let (s, c) = w.direction_rad.sin_cos();
        let a = [c, s, w.centre_x_m * s - w.centre_y_m * c];
        let stiffness = w.stiffness_factor * stiffness_per_modulus_m(w.length_m, w.thickness_m, storey.height_m, shear_to_young_ratio(w.poisson_ratio));
        for (row, ai) in k.iter_mut().zip(a) {
            for (cell, aj) in row.iter_mut().zip(a) {
                *cell += stiffness * ai * aj;
            }
        }
    }
    let (kxx, kyy, kxy) = (k[0][0], k[1][1], k[0][1]);
    let determinant = kxx * kyy - kxy * kxy;
    if kxx <= 0.0 || kyy <= 0.0 || determinant <= 1e-12 * kxx * kyy {
        return None;
    }
    let (kxt, kyt, ktt) = (k[0][2], k[1][2], k[2][2]);
    let xs = (kxx * kyt - kxy * kxt) / determinant;
    let ys = (kxy * kyt - kyy * kxt) / determinant;
    let k_theta = (ktt + 2.0 * (ys * kxt - xs * kyt) + xs * xs * kyy - 2.0 * xs * ys * kxy + ys * ys * kxx).max(0.0);
    Some(WallPlan { centre_of_stiffness_x_m: xs, centre_of_stiffness_y_m: ys, stiffness_x: kxx, stiffness_y: kyy, torsional_stiffness: k_theta, radius_x_m: (k_theta / kyy).sqrt(), radius_y_m: (k_theta / kxx).sqrt(), wall_count: own.len(), stiffness_factor_range: own.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), w| (lo.min(w.stiffness_factor), hi.max(w.stiffness_factor))), poisson_ratio_range: own.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), w| (lo.min(w.poisson_ratio), hi.max(w.poisson_ratio))) })
}

/// 📐 Plan regularity metrics `(e0x, e0y, r_x, r_y, ℓₛ)` of a storey whose stiffness comes from its walls (see [`wall_plan_of_storey`]), with the radius of gyration of the floor `ℓₛ = √((Lx² + Ly²) / 12)`.
pub fn plan_metrics(plan: &WallPlan, cm_x: f64, cm_y: f64, plan_width_m: f64, plan_length_m: f64) -> (f64, f64, f64, f64, f64) {
    let (lx, ly) = (plan_width_m.max(0.0), plan_length_m.max(0.0));
    ((cm_x - plan.centre_of_stiffness_x_m).abs(), (cm_y - plan.centre_of_stiffness_y_m).abs(), plan.radius_x_m, plan.radius_y_m, ((lx * lx + ly * ly) / 12.0).sqrt())
}

/// 🎯 Stiffness centre `(x, y)` of a storey: the wall plan when the positioned walls give one, else the entered centre of stiffness.
pub fn stiffness_centre_of(members: &[En1998Member], storey: &En1998Storey) -> (f64, f64, bool) {
    match wall_plan_of_storey(&placed_walls_of(members), storey) {
        Some(plan) => (plan.centre_of_stiffness_x_m, plan.centre_of_stiffness_y_m, true),
        None => (storey.centre_of_stiffness_x_m, storey.centre_of_stiffness_y_m, false),
    }
}
