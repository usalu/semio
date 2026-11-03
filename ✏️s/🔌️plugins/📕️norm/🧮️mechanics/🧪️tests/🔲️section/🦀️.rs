use crate::section::{mesh, profiles};
use crate::*;
use semio_s_plugin_norm_compliance::python_oracle;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::sync::atomic::AtomicBool;
use std::sync::OnceLock;

type Outline = (Vec<[f64; 2]>, Vec<Vec<[f64; 2]>>);

fn material(nu: f64) -> Material {
    Material { id: "M".into(), e: 1.0, g: 1.0 / (2.0 * (1.0 + nu)), rho: 0.0 }
}

fn oracle() -> &'static BTreeMap<String, f64> {
    static CACHE: OnceLock<BTreeMap<String, f64>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🔮️oracles/🔲️section/🐍️.py");
        let out = python_oracle::run_ok(&script, &[], None);
        String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| l.split_once(' ')).map(|(k, v)| (k.to_string(), v.parse().expect("number"))).collect()
    })
}

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol * expected.abs().max(1e-300), "{what}: {actual} vs {expected} (rel tol {tol})");
}

fn near(actual: f64, expected: f64, abs: f64, what: &str) {
    assert!((actual - expected).abs() <= abs, "{what}: {actual} vs {expected} (abs tol {abs})");
}

fn rect(w: f64, h: f64) -> Vec<[f64; 2]> {
    vec![[0.0, 0.0], [w, 0.0], [w, h], [0.0, h]]
}

fn circle(d: f64, n: usize) -> Vec<[f64; 2]> {
    (0..n).map(|k| [d / 2.0 * (2.0 * PI * k as f64 / n as f64).cos(), d / 2.0 * (2.0 * PI * k as f64 / n as f64).sin()]).collect()
}

fn i_sharp() -> Vec<[f64; 2]> {
    let (b, h, tf, tw) = (0.1, 0.12, 0.008, 0.006);
    let (x0, x1) = ((b - tw) / 2.0, (b + tw) / 2.0);
    vec![[0.0, 0.0], [b, 0.0], [b, tf], [x1, tf], [x1, h - tf], [b, h - tf], [b, h], [0.0, h], [0.0, h - tf], [x0, h - tf], [x0, tf], [0.0, tf]]
}

fn channel_sharp() -> Vec<[f64; 2]> {
    vec![[0.0, 0.0], [0.075, 0.0], [0.075, 0.011], [0.0085, 0.011], [0.0085, 0.189], [0.075, 0.189], [0.075, 0.2], [0.0, 0.2]]
}

fn angle_sharp() -> Vec<[f64; 2]> {
    vec![[0.0, 0.0], [0.1, 0.0], [0.1, 0.01], [0.01, 0.01], [0.01, 0.1], [0.0, 0.1]]
}

fn tee_sharp() -> Vec<[f64; 2]> {
    vec![[0.046, 0.0], [0.054, 0.0], [0.054, 0.09], [0.1, 0.09], [0.1, 0.1], [0.0, 0.1], [0.0, 0.09], [0.046, 0.09]]
}

fn hexagon_hole() -> Outline {
    (rect(0.1, 0.08), vec![(0..6).map(|k| [0.02 + 0.015 * (2.0 * PI * k as f64 / 6.0).cos(), 0.04 + 0.015 * (2.0 * PI * k as f64 / 6.0).sin()]).collect()])
}

fn blessed() -> Vec<(&'static str, Outline, f64)> {
    vec![
        ("rectangle", (rect(0.07, 0.14), vec![]), 4.0),
        ("i-sharp", (i_sharp(), vec![]), 3.0),
        ("channel-sharp", (channel_sharp(), vec![]), 3.0),
        ("angle-sharp", (angle_sharp(), vec![]), 3.0),
        ("rhs-sharp", (rect(0.1, 0.06), vec![vec![[0.004, 0.004], [0.096, 0.004], [0.096, 0.056], [0.004, 0.056]]]), 3.0),
        ("chs-96", (circle(0.1, 96), vec![circle(0.09, 96)]), 3.0),
        ("hexagon-hole", hexagon_hole(), 4.0),
        ("ipe300", profiles::i_section(0.3, 0.15, 0.0107, 0.0071, 0.015, 16), 2.0),
        ("pfc200", profiles::channel_section(0.2, 0.075, 0.0115, 0.0085, 0.0115, 16), 2.0),
        ("l100x10", profiles::angle_section(0.1, 0.1, 0.01, 0.012, 0.006, 16), 2.0),
        ("t150", profiles::tee_section(0.15, 0.15, 0.0107, 0.0071, 0.012, 16), 2.0),
        ("plate-girder", profiles::mono_i_section(0.8, [0.3, 0.02], [0.4, 0.03], 0.012, 0.0, 1), 2.0),
        ("rhs-200x100x8", profiles::rectangular_hollow_section(0.2, 0.1, 0.008, 0.016, 16), 2.0),
        ("ehs-300x200x10", profiles::elliptical_hollow_section(0.3, 0.2, 0.01, 128), 2.0),
    ]
}

fn size_of(outline: &Outline) -> f64 {
    let fold = |axis: usize| {
        let (lo, hi) = outline.0.iter().fold((f64::MAX, f64::MIN), |m, p| (m.0.min(p[axis]), m.1.max(p[axis])));
        hi - lo
    };
    fold(0).max(fold(1))
}

fn compare(name: &str, c: &SectionConstants, size: f64, tol: f64) {
    let o = oracle();
    let q = |k: &str| *o.get(&format!("{name}.{k}")).unwrap_or_else(|| panic!("oracle value {name}.{k}"));
    let what = |k: &str| format!("{name} {k}");
    close(c.area, q("area"), 1e-6, &what("area"));
    near(c.centroid_y, q("cy"), 1e-6 * size, &what("cy"));
    near(c.centroid_z, q("cz"), 1e-6 * size, &what("cz"));
    close(c.iy, q("iy"), 1e-6, &what("iy"));
    close(c.iz, q("iz"), 1e-6, &what("iz"));
    near(c.iyz, q("iyz"), 1e-6 * c.iy.max(c.iz), &what("iyz"));
    close(c.j, q("j"), tol, &what("j"));
    near(c.iw, q("iw"), tol * q("iw").abs() + 1e-6 * c.iy * size * size, &what("iw"));
    near(c.shear_centre_y, q("scy"), 2e-3 * size, &what("scy"));
    near(c.shear_centre_z, q("scz"), 2e-3 * size, &what("scz"));
    close(c.a_vy, q("avy"), tol, &what("avy"));
    close(c.a_vz, q("avz"), tol, &what("avz"));
    close(c.w_pl_y, q("wply"), 2e-3, &what("wply"));
    close(c.w_pl_z, q("wplz"), 2e-3, &what("wplz"));
    close(c.w_el_y_pos, q("wel_y_pos"), 1e-6, &what("wel_y_pos"));
    close(c.w_el_y_neg, q("wel_y_neg"), 1e-6, &what("wel_y_neg"));
    close(c.w_el_z_pos, q("wel_z_pos"), 1e-6, &what("wel_z_pos"));
    close(c.w_el_z_neg, q("wel_z_neg"), 1e-6, &what("wel_z_neg"));
}

#[test]
fn finite_element_constants_match_sectionproperties_within_half_a_percent() {
    for (name, outline, cells) in blessed() {
        let options = SectionOptions { cells_across_wall: cells, ..SectionOptions::standard() };
        let c = analyse_section_with(&outline.0, &outline.1, &material(0.0), &options, &Control::none()).expect(name);
        assert_eq!(c.method, SectionMethod::FiniteElement);
        assert!(c.triangles > 100 && c.nodes > c.triangles, "{name}: mesh");
        compare(name, &c, size_of(&outline), 5e-3);
    }
}

#[test]
fn poissons_ratio_enters_the_shear_areas_like_in_sectionproperties() {
    let o = oracle();
    let c = analyse_section(&rect(0.07, 0.14), &[], &material(0.3)).unwrap();
    close(c.poisson, 0.3, 1e-12, "poisson");
    close(c.a_vy, o["rectangle-nu.avy"], 5e-3, "avy");
    close(c.a_vz, o["rectangle-nu.avz"], 5e-3, "avz");
    close(c.j, o["rectangle-nu.j"], 5e-3, "j");
    close(c.iw, o["rectangle-nu.iw"], 5e-3, "iw");
}

#[test]
fn rectangle_follows_the_torsion_series_and_the_cowper_shear_factor() {
    let (a, b) = (0.14_f64, 0.07_f64);
    let series: f64 = (0..20).map(|k| 2 * k + 1).map(|n| (n as f64 * PI * a / (2.0 * b)).tanh() / (n as f64).powi(5)).sum();
    let exact = a * b.powi(3) / 3.0 - 64.0 * b.powi(4) / PI.powi(5) * series;
    let c = analyse_section(&rect(b, a), &[], &material(0.0)).unwrap();
    close(c.j, exact, 2e-3, "J series");
    close(c.a_vz, 5.0 / 6.0 * a * b, 2e-3, "A_v,z = 5/6 A");
    close(c.a_vy, 5.0 / 6.0 * a * b, 2e-3, "A_v,y = 5/6 A");
    close(c.w_pl_y, b * a * a / 4.0, 1e-9, "W_pl,y");
    close(c.w_pl_z, a * b * b / 4.0, 1e-9, "W_pl,z");
    close(c.w_el_y_pos, b * a * a / 6.0, 1e-9, "W_el,y");
    near(c.shear_centre_y, b / 2.0, 1e-6, "sc y");
    near(c.shear_centre_z, a / 2.0, 1e-6, "sc z");
}

#[test]
fn circular_hollow_section_has_polar_torsion_and_no_warping() {
    let (d, t) = (0.1, 0.005);
    let (outer, holes) = profiles::circular_hollow_section(d, t, 128);
    let c = analyse_section(&outer, &holes, &material(0.0)).unwrap();
    close(c.j, PI / 32.0 * (d.powi(4) - (d - 2.0 * t).powi(4)), 3e-3, "J");
    close(c.area, PI / 4.0 * (d * d - (d - 2.0 * t).powi(2)), 1e-3, "A");
    assert!(c.iw.abs() < 1e-6 * c.j * d * d, "warping constant {}", c.iw);
    near(c.shear_centre_y, 0.0, 1e-6 * d, "sc y");
    near(c.shear_centre_z, 0.0, 1e-6 * d, "sc z");
}

#[test]
fn constants_are_invariant_under_translation_and_orientation() {
    let (outer, holes) = hexagon_hole();
    let base = analyse_section(&outer, &holes, &material(0.0)).unwrap();
    let moved: Vec<[f64; 2]> = outer.iter().rev().map(|p| [p[0] + 3.0, p[1] - 2.0]).collect();
    let moved_holes: Vec<Vec<[f64; 2]>> = holes.iter().map(|h| h.iter().rev().map(|p| [p[0] + 3.0, p[1] - 2.0]).collect()).collect();
    let c = analyse_section(&moved, &moved_holes, &material(0.0)).unwrap();
    close(c.j, base.j, 1e-6, "J");
    close(c.iw, base.iw, 1e-4, "I_w");
    close(c.a_vy, base.a_vy, 1e-6, "A_v,y");
    close(c.a_vz, base.a_vz, 1e-6, "A_v,z");
    near(c.shear_centre_y - 3.0, base.shear_centre_y, 1e-6, "sc y");
    near(c.shear_centre_z + 2.0, base.shear_centre_z, 1e-6, "sc z");
}

#[test]
fn meshes_are_conforming_positive_and_fill_the_outline() {
    for (outer, holes) in [(i_sharp(), vec![]), hexagon_hole()] {
        let rings = mesh::normalise(&outer, &holes).unwrap();
        let target = mesh::target_edge(&rings, 3.0, 0.0, 40_000);
        let grid = mesh::generate(&rings, target, 40_000, &Control::none()).unwrap();
        let total: f64 = grid.triangles.iter().map(|t| mesh::signed_area(&[grid.points[t[0]], grid.points[t[1]], grid.points[t[2]]])).sum();
        let expected: f64 = rings.iter().map(|r| mesh::signed_area(r)).sum();
        close(total, expected, 1e-9, "triangle area sum");
        let mut owners: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        for t in &grid.triangles {
            assert!(mesh::signed_area(&[grid.points[t[0]], grid.points[t[1]], grid.points[t[2]]]) > 0.0, "inverted triangle");
            for k in 0..3 {
                *owners.entry((t[k].min(t[(k + 1) % 3]), t[k].max(t[(k + 1) % 3]))).or_default() += 1;
            }
        }
        assert!(owners.values().all(|&n| n == 1 || n == 2), "non-manifold edge");
        let boundary: f64 = owners.iter().filter(|(_, n)| **n == 1).map(|(e, _)| (grid.points[e.0][0] - grid.points[e.1][0]).hypot(grid.points[e.0][1] - grid.points[e.1][1])).sum();
        let perimeter: f64 = rings.iter().map(|r| (0..r.len()).map(|i| (r[i][0] - r[(i + 1) % r.len()][0]).hypot(r[i][1] - r[(i + 1) % r.len()][1])).sum::<f64>()).sum();
        close(boundary, perimeter, 1e-9, "boundary length");
    }
}

#[test]
fn progress_is_reported_and_cancellation_stops_the_analysis() {
    let stages = RefCell::new(Vec::<String>::new());
    let sink = |p: &Progress| {
        assert!((0.0..=1.0).contains(&p.fraction));
        let mut list = stages.borrow_mut();
        if list.last() != Some(&p.stage) {
            list.push(p.stage.clone());
        }
    };
    let (outer, holes) = hexagon_hole();
    let control = Control::none().with_progress(&sink);
    analyse_section_with(&outer, &holes, &material(0.0), &SectionOptions::standard(), &control).unwrap();
    let seen = stages.borrow().clone();
    for stage in ["mesh", "assemble", "factorise", "solve", "integrate"] {
        assert!(seen.iter().any(|s| s == stage), "stage {stage} missing in {seen:?}");
    }
    let flag = AtomicBool::new(true);
    let cancelled = analyse_section_with(&outer, &holes, &material(0.0), &SectionOptions::standard(), &Control::none().with_cancel(&flag));
    assert_eq!(cancelled, Err(MechanicsError::Cancelled));
}

#[test]
fn analysis_is_deterministic() {
    let (outer, holes) = hexagon_hole();
    let first = analyse_section(&outer, &holes, &material(0.3)).unwrap();
    assert_eq!(first, analyse_section(&outer, &holes, &material(0.3)).unwrap());
}

#[test]
fn degenerate_outlines_are_refused() {
    let m = material(0.0);
    assert!(matches!(analyse_section(&[[0.0, 0.0], [1.0, 0.0]], &[], &m), Err(MechanicsError::Invalid(_))));
    assert!(matches!(analyse_section(&[[0.0, 0.0], [1.0, 0.0], [2.0, 0.0]], &[], &m), Err(MechanicsError::Invalid(_))));
    assert!(matches!(analyse_section(&rect(1.0, 1.0), &[rect(0.5, 0.5).iter().map(|p| [p[0] + 3.0, p[1]]).collect()], &m), Err(MechanicsError::Invalid(_))));
    assert!(matches!(analyse_section(&[[0.0, f64::NAN], [1.0, 0.0], [1.0, 1.0]], &[], &m), Err(MechanicsError::Invalid(_))));
}

fn wall(y1: f64, z1: f64, y2: f64, z2: f64, thickness: f64) -> ThinWall {
    ThinWall { y1, z1, y2, z2, thickness }
}

fn significant(thin: f64, fe: f64, scale: f64, what: &str) {
    if fe.abs() > 2e-2 * scale {
        close(thin, fe, 3e-2, what);
    } else {
        near(thin, fe, 5e-3 * scale, what);
    }
}

fn consistent(name: &str, walls: &[ThinWall], polygon: &[[f64; 2]], size: f64) {
    let thin = analyse_thin_walled(walls, &material(0.0)).expect(name);
    let fe = analyse_section(polygon, &[], &material(0.0)).expect(name);
    assert_eq!(thin.method, SectionMethod::ThinWalled);
    close(thin.area, fe.area, 3.5e-2, &format!("{name} A"));
    close(thin.iy, fe.iy, 3e-2, &format!("{name} I_y"));
    close(thin.j, fe.j, 4e-2, &format!("{name} J"));
    significant(thin.iw, fe.iw, fe.iy.max(fe.iz) * size * size, &format!("{name} I_w"));
    near(thin.shear_centre_y, fe.shear_centre_y, 1e-3, &format!("{name} sc y"));
    near(thin.shear_centre_z, fe.shear_centre_z, 1e-3, &format!("{name} sc z"));
    close(thin.a_vy, fe.a_vy, 1e-1, &format!("{name} A_v,y"));
    close(thin.a_vz, fe.a_vz, 1e-1, &format!("{name} A_v,z"));
}

#[test]
fn thin_walled_solver_is_consistent_with_the_finite_element_solver() {
    consistent("I", &[wall(0.0, 0.004, 0.1, 0.004, 0.008), wall(0.05, 0.004, 0.05, 0.116, 0.006), wall(0.0, 0.116, 0.1, 0.116, 0.008)], &i_sharp(), 0.12);
    consistent("channel", &[wall(0.075, 0.0055, 0.00425, 0.0055, 0.011), wall(0.00425, 0.0055, 0.00425, 0.1945, 0.0085), wall(0.00425, 0.1945, 0.075, 0.1945, 0.011)], &channel_sharp(), 0.2);
    consistent("angle", &[wall(0.1, 0.005, 0.005, 0.005, 0.01), wall(0.005, 0.005, 0.005, 0.1, 0.01)], &angle_sharp(), 0.1);
    consistent("tee", &[wall(0.0, 0.095, 0.1, 0.095, 0.01), wall(0.05, 0.095, 0.05, 0.0, 0.008)], &tee_sharp(), 0.1);
}

#[test]
fn thin_walled_torsion_and_warping_have_closed_forms() {
    let (b, h, tf, tw) = (0.2, 0.3, 0.012, 0.008);
    let c = analyse_thin_walled(&[wall(0.0, 0.0, b, 0.0, tf), wall(b / 2.0, 0.0, b / 2.0, h, tw), wall(0.0, h, b, h, tf)], &material(0.0)).unwrap();
    close(c.j, (2.0 * b * tf.powi(3) + h * tw.powi(3)) / 3.0, 1e-12, "J = sum l t^3 / 3");
    close(c.iw, tf * b.powi(3) * h * h / 24.0, 1e-9, "I_w = t_f b^3 h^2 / 24");
    near(c.shear_centre_y, b / 2.0, 1e-9, "sc y");
    near(c.shear_centre_z, c.centroid_z, 1e-9, "sc z");
}

#[test]
fn closed_or_disconnected_walls_are_refused_by_the_thin_walled_solver() {
    let m = material(0.0);
    let boxed = [wall(0.0, 0.0, 1.0, 0.0, 0.01), wall(1.0, 0.0, 1.0, 1.0, 0.01), wall(1.0, 1.0, 0.0, 1.0, 0.01), wall(0.0, 1.0, 0.0, 0.0, 0.01)];
    assert!(matches!(analyse_thin_walled(&boxed, &m), Err(MechanicsError::Invalid(_))));
    assert!(matches!(analyse_thin_walled(&[wall(0.0, 0.0, 1.0, 0.0, 0.01), wall(0.0, 2.0, 1.0, 2.0, 0.01)], &m), Err(MechanicsError::Invalid(_))));
    assert!(matches!(analyse_thin_walled(&[], &m), Err(MechanicsError::Invalid(_))));
}

#[test]
fn profile_outlines_have_the_closed_form_areas() {
    let area = |o: &Outline| mesh::signed_area(&o.0).abs() - o.1.iter().map(|h| mesh::signed_area(h).abs()).sum::<f64>();
    let (d, b, tf, tw, r) = (0.3, 0.15, 0.0107, 0.0071, 0.015);
    close(area(&profiles::i_section(d, b, tf, tw, r, 16)), 2.0 * b * tf + (d - 2.0 * tf) * tw + (4.0 - PI) * r * r, 1e-3, "I area");
    close(area(&profiles::channel_section(0.2, 0.075, 0.0115, 0.0085, 0.0115, 16)), 2.0 * 0.075 * 0.0115 + (0.2 - 0.023) * 0.0085 + (4.0 - PI) / 2.0 * 0.0115 * 0.0115, 1e-3, "channel area");
    let (h, w, t, ro) = (0.2, 0.1, 0.008, 0.016);
    let ri = ro - t;
    close(area(&profiles::rectangular_hollow_section(h, w, t, ro, 16)), w * h - (4.0 - PI) * ro * ro - ((w - 2.0 * t) * (h - 2.0 * t) - (4.0 - PI) * ri * ri), 1e-3, "RHS area");
    close(area(&profiles::tee_section(0.15, 0.15, 0.0107, 0.0071, 0.012, 16)), 0.15 * 0.0107 + (0.15 - 0.0107) * 0.0071 + (4.0 - PI) / 2.0 * 0.012 * 0.012, 1e-3, "tee area");
}
