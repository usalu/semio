//! 🥞️ Floor diaphragm of the bracing walls (EN 1998-1 4.3.1(4)): the diaphragm is classified rigid or flexible by comparing its in-plane deflection with the drift of the walls that brace it, and the storey shear reaches the walls by stiffness when it is rigid and by tributary width when it is flexible. See [`super`].

use super::*;

/// 📏️ A diaphragm is rigid while its largest in-plane displacement stays within this multiple of the displacement of the rigid-diaphragm assumption (EN 1998-1 4.3.1(4), "10 %": recalled, unverified, `ndp.en1996.m95.diaphragm-ratio`).
pub const RIGID_RATIO: f64 = 1.10;

const NODES: usize = 201;
const NDP_RATIO: &str = "ndp.en1996.m95.diaphragm-ratio";

/// ↔️ The plan direction of the horizontal action; the walls parallel to it brace.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanAxis {
    X,
    Y,
}

impl PlanAxis {
    /// 🔑️ The id fragment of the direction.
    pub fn key(self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
        }
    }
}

/// 🥞️ The outcome of the displacement comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiaphragmClass {
    Rigid,
    Flexible,
}

/// 🧱️ One wall that braces the diaphragm in a direction, with its share of the storey shear under both distribution rules.
#[derive(Clone, Debug, PartialEq)]
pub struct BracingWall {
    pub wall_id: String,
    pub s_m: f64,
    pub stiffness_n_per_m: f64,
    pub drift_m: f64,
    pub declared_n: f64,
    pub rigid_n: f64,
    pub tributary_n: f64,
}

impl BracingWall {
    /// 🔀️ The share of the storey shear this wall carries once the diaphragm is classified.
    pub fn share_n(&self, class: DiaphragmClass) -> f64 {
        match class {
            DiaphragmClass::Rigid => self.rigid_n,
            DiaphragmClass::Flexible => self.tributary_n,
        }
    }
}

/// 🥞️ The diaphragm of one plan direction: the classification, the displacement comparison, the wall shares and the deflection profile across the diaphragm.
#[derive(Clone, Debug, PartialEq)]
pub struct Diaphragm {
    pub axis: PlanAxis,
    pub class: DiaphragmClass,
    pub ratio: f64,
    pub force_n: f64,
    pub declared_n: f64,
    pub span_m: (f64, f64),
    pub depth_m: f64,
    pub g_t_n_per_m: f64,
    pub rigid_drift_m: f64,
    pub mean_drift_m: f64,
    pub deflection_m: f64,
    pub walls: Vec<BracingWall>,
    pub springs: Vec<(f64, f64)>,
    pub profile: Vec<(f64, f64)>,
}

impl Diaphragm {
    /// 🧭️ The tributary fractions of the walls `ids` (in that order) when only they brace the diaphragm; `None` when one of them does not.
    pub fn tributary_of(&self, ids: &[&str]) -> Option<Vec<f64>> {
        let springs: Option<Vec<(f64, f64)>> = ids.iter().map(|id| self.walls.iter().find(|w| w.wall_id == *id).map(|w| (w.s_m, w.stiffness_n_per_m))).collect();
        Some(tributary_fractions(self.span_m, &springs?))
    }
}

/// 🧱️ Lateral stiffness of a wall fixed at both floors: flexure `h³/(12 E I)` plus shear `1.2 h/(G A)` with `E = K_E f_k` and `G = 0.4 E` (EN 1996-1-1 3.7.2, 3.7.3); the openings are ignored.
pub fn wall_stiffness_n_per_m(annex: AnnexChoice, wall: &MasonryWall) -> f64 {
    let e = k_e(annex) * f_k_pa(annex, wall);
    let (l, t, h) = (wall.length_m.max(1e-3), wall.thickness_m.max(1e-3), wall.height_m.max(1e-3));
    1.0 / (h.powi(3) / (e * t * l.powi(3)) + 1.2 * h / (0.4 * e * t * l))
}

fn plan_ends(wall: &MasonryWall) -> ((f64, f64), (f64, f64)) {
    let (sin, cos) = wall.plan_angle_deg.to_radians().sin_cos();
    ((wall.plan_x_m, wall.plan_y_m), (wall.plan_x_m + wall.length_m * cos, wall.plan_y_m + wall.length_m * sin))
}

fn axis_of(wall: &MasonryWall) -> PlanAxis {
    let (a, b) = plan_ends(wall);
    if (b.0 - a.0).abs() >= (b.1 - a.1).abs() { PlanAxis::X } else { PlanAxis::Y }
}

/// 📐️ In-plane deflections of the diaphragm as a shear beam of rigidity `rigidity` [N] loaded uniformly by `force` and carried by wall springs `(position, stiffness)`: an axial bar of that rigidity on spring supports solved by the shared mechanics kernel ([`mechanics::Analysis`]) on a uniform mesh refined by the spring positions; `(position, displacement)` per station, empty when the kernel refuses the model.
pub fn deflections(span: (f64, f64), rigidity: f64, springs: &[(f64, f64)], force: f64) -> Vec<(f64, f64)> {
    let length = span.1 - span.0;
    let mut stations: Vec<f64> = (0..NODES).map(|i| span.0 + length * i as f64 / (NODES - 1) as f64).collect();
    stations.extend(springs.iter().map(|s| s.0.clamp(span.0, span.1)));
    stations.sort_by(f64::total_cmp);
    stations.dedup_by(|current, previous| *current - *previous < length * 1e-9);
    let mut stiffness = vec![0.0; stations.len()];
    for (s, k) in springs {
        if let Some((i, _)) = stations.iter().enumerate().min_by(|a, b| (a.1 - s).abs().total_cmp(&(b.1 - s).abs())) {
            stiffness[i] += k;
        }
    }
    let last = stations.len() - 1;
    let load = [force / length, 0.0, 0.0];
    let model = mechanics::Model {
        nodes: stations.iter().enumerate().map(|(i, s)| mechanics::Node { id: format!("n{i}"), x: *s, ..Default::default() }).collect(),
        materials: vec![mechanics::Material { id: "G".into(), e: 1.0, g: 1.0, rho: 0.0 }],
        sections: vec![mechanics::Section { id: "D".into(), a: rigidity, iy: 1.0, iz: 1.0, j: 1.0, ..Default::default() }],
        members: (0..last).map(|j| mechanics::Member { id: format!("m{j}"), node_i: format!("n{j}"), node_j: format!("n{}", j + 1), material: "G".into(), section: "D".into(), ..Default::default() }).collect(),
        supports: stiffness.iter().enumerate().map(|(i, k)| mechanics::Support { node: format!("n{i}"), fixed: [false, true, true, true, true, true], spring: [*k, 0.0, 0.0, 0.0, 0.0, 0.0], ..Default::default() }).collect(),
        load_cases: vec![mechanics::LoadCase {
            id: "q".into(),
            distributed: (0..last).map(|j| mechanics::DistributedLoad { member: format!("m{j}"), frame: mechanics::LoadFrame::Global, start: load, end: load }).collect(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let control = mechanics::Control::none();
    let solved = mechanics::Analysis::new(&model, &control).and_then(|analysis| analysis.solve_case(&model.load_cases[0], &control));
    solved.map_or_else(|_| Vec::new(), |result| stations.iter().zip(&result.displacements).map(|(s, d)| (*s, d.u[0])).collect())
}

fn displacement_at(w: &[(f64, f64)], s: f64) -> f64 {
    let i = w.partition_point(|p| p.0 <= s).clamp(1, w.len() - 1);
    let ((s_a, w_a), (s_b, w_b)) = (w[i - 1], w[i]);
    w_a + (w_b - w_a) * ((s - s_a) / (s_b - s_a).max(1e-12)).clamp(0.0, 1.0)
}

/// 🧮️ The classification ratio: the largest displacement of the flexible diaphragm over the displacement `F / Σk` of the rigid assumption; 1 for a diaphragm without extent.
pub fn displacement_ratio(span: (f64, f64), rigidity: f64, springs: &[(f64, f64)]) -> f64 {
    if span.1 - span.0 < 1e-6 || springs.is_empty() {
        return 1.0;
    }
    let w = deflections(span, rigidity, springs, 1.0);
    w.iter().map(|p| p.1).fold(0.0, f64::max) * springs.iter().map(|s| s.1).sum::<f64>()
}

/// 🧭️ The tributary fraction of the diaphragm width of every spring: half the distance to the neighbouring wall line on each side, the outer lines out to the diaphragm edge; walls on one line split their fraction by stiffness.
pub fn tributary_fractions(span: (f64, f64), springs: &[(f64, f64)]) -> Vec<f64> {
    let len = (span.1 - span.0).max(1e-9);
    let mut order: Vec<usize> = (0..springs.len()).collect();
    order.sort_by(|a, b| springs[*a].0.total_cmp(&springs[*b].0));
    let mut groups: Vec<(f64, Vec<usize>)> = Vec::new();
    for i in order {
        let joins = groups.last().is_some_and(|(s, _)| (springs[i].0 - s).abs() < 1e-6);
        if joins {
            groups.last_mut().expect("a group").1.push(i);
        } else {
            groups.push((springs[i].0, vec![i]));
        }
    }
    let mut out = vec![0.0; springs.len()];
    for (g, (s, members)) in groups.iter().enumerate() {
        let lo = if g == 0 { span.0 } else { 0.5 * (groups[g - 1].0 + *s) };
        let hi = groups.get(g + 1).map_or(span.1, |next| 0.5 * (*s + next.0));
        let k_sum: f64 = members.iter().map(|i| springs[*i].1).sum();
        for i in members {
            out[*i] = (hi - lo) / len * springs[*i].1 / k_sum;
        }
    }
    out
}

/// 🥞️ The diaphragms of the document, one per plan direction braced by at least two walls: the weakest declared diaphragm stiffness `G t` acts over the plan depth in the load direction, the storey shear is the sum of the declared in-plane wall shears (a unit force when none is declared), and the plan frame is the envelope of all wall ends. Empty when no wall declares a diaphragm stiffness.
pub fn diaphragms(annex: AnnexChoice, walls: &[MasonryWall]) -> Vec<Diaphragm> {
    let g_t = walls.iter().map(|w| w.diaphragm_stiffness_n_per_m).filter(|k| *k > 0.0).fold(f64::INFINITY, f64::min);
    if !g_t.is_finite() {
        return Vec::new();
    }
    let ends: Vec<(f64, f64)> = walls.iter().flat_map(|w| {
        let (a, b) = plan_ends(w);
        [a, b]
    }).collect();
    let range = |pick: fn(&(f64, f64)) -> f64| ends.iter().map(pick).fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| (lo.min(v), hi.max(v)));
    let (x_range, y_range) = (range(|p| p.0), range(|p| p.1));
    [PlanAxis::X, PlanAxis::Y].into_iter().filter_map(|axis| {
        let bracing: Vec<&MasonryWall> = walls.iter().filter(|w| !matches!(w.wall_type, WallType::NonLoadBearing) && axis_of(w) == axis).collect();
        if bracing.len() < 2 {
            return None;
        }
        let (span, along) = if axis == PlanAxis::X { (y_range, x_range) } else { (x_range, y_range) };
        let depth_m = (along.1 - along.0).max(1e-3);
        let springs: Vec<(f64, f64)> = bracing.iter().map(|w| {
            let (a, b) = plan_ends(w);
            (if axis == PlanAxis::X { 0.5 * (a.1 + b.1) } else { 0.5 * (a.0 + b.0) }, wall_stiffness_n_per_m(annex, w))
        }).collect();
        let declared_n: f64 = bracing.iter().map(|w| w.in_plane_shear_n.abs()).sum();
        let force_n = if declared_n > 0.0 { declared_n } else { 1.0 };
        let k_sum: f64 = springs.iter().map(|s| s.1).sum();
        let rigid_drift_m = force_n / k_sum;
        let flat = span.1 - span.0 < 1e-6;
        let w = if flat { vec![(span.0, rigid_drift_m), (span.0 + 1.0, rigid_drift_m)] } else { deflections(span, g_t * depth_m, &springs, force_n) };
        if w.is_empty() {
            return None;
        }
        let max_w = w.iter().map(|p| p.1).fold(0.0, f64::max);
        let drifts: Vec<f64> = springs.iter().map(|s| displacement_at(&w, s.0)).collect();
        let mean_drift_m = drifts.iter().sum::<f64>() / drifts.len() as f64;
        let ratio = if flat { 1.0 } else { max_w / rigid_drift_m };
        let tributary = tributary_fractions(span, &springs);
        let walls_out: Vec<BracingWall> = bracing.iter().enumerate().map(|(i, wall)| BracingWall {
            wall_id: wall.id.clone(),
            s_m: springs[i].0,
            stiffness_n_per_m: springs[i].1,
            drift_m: drifts[i],
            declared_n: wall.in_plane_shear_n.abs(),
            rigid_n: force_n * springs[i].1 / k_sum,
            tributary_n: force_n * tributary[i],
        }).collect();
        Some(Diaphragm {
            axis,
            class: if ratio <= RIGID_RATIO + 1e-12 { DiaphragmClass::Rigid } else { DiaphragmClass::Flexible },
            ratio,
            force_n,
            declared_n,
            span_m: span,
            depth_m,
            g_t_n_per_m: g_t,
            rigid_drift_m,
            mean_drift_m,
            deflection_m: (max_w - mean_drift_m).max(0.0),
            walls: walls_out,
            springs,
            profile: w,
        })
    }).collect()
}

fn dimensionless(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

fn class_words(class: DiaphragmClass) -> (&'static str, &'static str) {
    match class {
        DiaphragmClass::Rigid => ("rigid: the walls share the storey shear by stiffness", "starr: die Wände teilen sich die Geschossquerkraft nach Steifigkeit"),
        DiaphragmClass::Flexible => ("flexible: the walls carry their tributary width of the storey shear", "verformbar: die Wände tragen ihre Einflussbreite der Geschossquerkraft"),
    }
}

/// 🥞️ The diaphragm checks of the document: per direction the classification (utilisation = displacement ratio against 1.10, a warning above with the stiffness that makes the diaphragm rigid as remedy). The shear distribution that follows the classification is read by the bracing case of the walls (M-21, [`bracing_case`]).
pub fn diaphragm_checks(annex: AnnexChoice, walls: &[MasonryWall]) -> Vec<CheckResult> {
    let part = "EN 1998-1";
    let mut out = Vec::new();
    for d in diaphragms(annex, walls) {
        let weakest = walls.iter().filter(|w| w.diaphragm_stiffness_n_per_m > 0.0).min_by(|a, b| a.diaphragm_stiffness_n_per_m.total_cmp(&b.diaphragm_stiffness_n_per_m)).expect("a declared stiffness");
        let subject = SubjectRef::new(&weakest.id, wall_path(weakest, "diaphragmStiffnessNPerM"), loc(&weakest.label_en, &weakest.label_de));
        let (class_en, class_de) = class_words(d.class);
        let axis = d.axis.key();
        let mut check = CheckResult::assess(format!("en1996.9.3.diaphragm.{axis}"), part, clause(part, "§4.3.1", "4.3.1"), subject.clone(), loc("Floor diaphragm classified rigid or flexible", "Deckenscheibe als starr oder verformbar eingestuft"))
            .utilization(dimensionless(d.ratio), dimensionless(RIGID_RATIO))
            .annex(annex)
            .explanation(loc(
                &format!("Direction {axis}, {} bracing walls: G t = {:.3e} N/m over {:.2} m; diaphragm deflection {:.3} mm against the average wall drift {:.3} mm (rigid assumption {:.3} mm); largest displacement {:.3} times the rigid one against {RIGID_RATIO:.2}: {class_en}. Walls without openings, fixed at both floors; unverified NDP basis: m95.diaphragm-ratio.", d.walls.len(), d.g_t_n_per_m, d.depth_m, d.deflection_m * 1e3, d.mean_drift_m * 1e3, d.rigid_drift_m * 1e3, d.ratio),
                &format!("Richtung {axis}, {} aussteifende Wände: G t = {:.3e} N/m über {:.2} m; Scheibenverformung {:.3} mm gegenüber der mittleren Wandverschiebung {:.3} mm (starre Annahme {:.3} mm); größte Verschiebung {:.3}-fach der starren gegenüber {RIGID_RATIO:.2}: {class_de}. Wände ohne Öffnungen, beidseitig an den Decken eingespannt; ungeprüfte NDP-Grundlage: m95.diaphragm-ratio.", d.walls.len(), d.g_t_n_per_m, d.depth_m, d.deflection_m * 1e3, d.mean_drift_m * 1e3, d.rigid_drift_m * 1e3, d.ratio),
            ))
            .status(CheckStatus::Pass)
            .warn_above(1.0)
            .ndp(NDP_RATIO);
        if d.class == DiaphragmClass::Flexible {
            let springs = d.springs.clone();
            let need = solve_threshold(d.g_t_n_per_m, 1.0e12, 1.0e5, |g| displacement_ratio(d.span_m, g * d.depth_m, &springs) <= RIGID_RATIO);
            if let Some(g) = need {
                check = check.remedy(Remedy::at_least(subject.clone(), dimensionless(d.g_t_n_per_m), dimensionless(g), loc("Stiffen the diaphragm so that it is rigid and the walls share by stiffness.", "Scheibe aussteifen, damit sie starr ist und die Wände nach Steifigkeit mittragen.")));
            }
        }
        out.push(check.build());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_artifact_norm_en1996_model::En1996Snapshot;

    fn walls(g_t: f64, shear: [f64; 2], lengths: [f64; 2]) -> Vec<MasonryWall> {
        let base = En1996Snapshot::compliant_clay_wall().walls.remove(0);
        [(0.0, 0usize), (6.0, 1)].into_iter().map(|(y, i)| {
            let mut w = base.clone();
            w.id = format!("w{i}");
            w.length_m = lengths[i];
            w.plan_x_m = 0.0;
            w.plan_y_m = y;
            w.plan_angle_deg = 0.0;
            w.in_plane_shear_n = shear[i];
            w.diaphragm_stiffness_n_per_m = g_t;
            w
        }).collect()
    }

    #[test]
    fn the_shear_beam_on_two_springs_reproduces_the_closed_form_midspan_deflection() {
        let (l, rigidity, k, f) = (6.0, 3.0e8, 1.0e8, 1000.0);
        let w = deflections((0.0, l), rigidity, &[(0.0, k), (l, k)], f);
        let want = f / (2.0 * k) + f * l / (8.0 * rigidity);
        let mid = displacement_at(&w, 0.5 * l);
        assert!((mid - want).abs() < 1e-6 * want, "{mid} against {want}");
        assert!((w[0].1 - f / (2.0 * k)).abs() < 1e-6 * want);
    }

    #[test]
    fn a_stiff_diaphragm_is_rigid_and_the_walls_share_by_stiffness() {
        let set = walls(1.0e12, [60_000.0, 40_000.0], [6.0, 3.0]);
        let d = &diaphragms(AnnexChoice::De, &set)[0];
        assert_eq!(d.class, DiaphragmClass::Rigid);
        assert!(d.ratio >= 1.0 - 1e-9 && d.ratio <= RIGID_RATIO);
        assert!((d.force_n - 100_000.0).abs() < 1e-6);
        assert!((d.walls.iter().map(|b| b.rigid_n).sum::<f64>() - d.force_n).abs() < 1e-6);
        assert!(d.walls[0].rigid_n > d.walls[1].rigid_n, "the longer wall is stiffer");
    }

    #[test]
    fn a_soft_diaphragm_is_flexible_and_the_walls_carry_their_tributary_width() {
        let set = walls(1.0e5, [50_000.0, 50_000.0], [6.0, 3.0]);
        let d = &diaphragms(AnnexChoice::De, &set)[0];
        assert_eq!(d.class, DiaphragmClass::Flexible);
        assert!(d.ratio > RIGID_RATIO && d.deflection_m > 0.0);
        assert!((d.walls.iter().map(|b| b.tributary_n).sum::<f64>() - d.force_n).abs() < 1e-6);
        assert!((d.walls[0].tributary_n - 0.5 * d.force_n).abs() < 1e-6, "two walls at the diaphragm edges take half each");
        assert!(d.walls[0].share_n(d.class) != d.walls[0].rigid_n);
    }

    #[test]
    fn the_check_warns_for_a_flexible_diaphragm_with_a_stiffness_remedy() {
        let set = walls(1.0e5, [10_000.0, 90_000.0], [6.0, 6.0]);
        let ids = |checks: &[CheckResult]| checks.iter().map(|c| c.id.as_str()).collect::<Vec<_>>();
        let checks = diaphragm_checks(AnnexChoice::De, &set);
        assert_eq!(ids(&checks), ["en1996.9.3.diaphragm.x"]);
        let class = &checks[0];
        assert!(class.utilization > 1.0 && class.status != CheckStatus::Fail && class.remedies.len() == 1);
        let stiff = walls(1.0e12, [50_000.0, 50_000.0], [6.0, 6.0]);
        assert!(diaphragm_checks(AnnexChoice::De, &stiff).iter().all(|c| c.status != CheckStatus::Fail && c.utilization <= 1.0));
    }

    #[test]
    fn the_tributary_fractions_of_a_subset_of_walls_are_recomputed() {
        let set = walls(1.0e5, [50_000.0, 50_000.0], [6.0, 3.0]);
        let d = &diaphragms(AnnexChoice::De, &set)[0];
        assert_eq!(d.tributary_of(&["w1", "w0"]).unwrap().len(), 2);
        assert!(d.tributary_of(&["w0", "missing"]).is_none());
    }

    #[test]
    fn nothing_is_classified_without_a_declared_stiffness_or_with_a_single_bracing_wall() {
        assert!(diaphragms(AnnexChoice::De, &walls(0.0, [1.0, 1.0], [6.0, 6.0])).is_empty());
        assert!(diaphragms(AnnexChoice::De, &walls(1.0e8, [1.0, 1.0], [6.0, 6.0])[..1]).is_empty());
    }

    #[test]
    fn the_stiffness_remedy_makes_the_diaphragm_rigid() {
        let set = walls(1.0e5, [50_000.0, 50_000.0], [6.0, 6.0]);
        let d = &diaphragms(AnnexChoice::De, &set)[0];
        let need = solve_threshold(d.g_t_n_per_m, 1.0e12, 1.0e5, |g| displacement_ratio(d.span_m, g * d.depth_m, &d.springs) <= RIGID_RATIO).expect("a stiffness");
        let mut fixed = set.clone();
        fixed.iter_mut().for_each(|w| w.diaphragm_stiffness_n_per_m = need);
        assert_eq!(diaphragms(AnnexChoice::De, &fixed)[0].class, DiaphragmClass::Rigid);
    }
}
