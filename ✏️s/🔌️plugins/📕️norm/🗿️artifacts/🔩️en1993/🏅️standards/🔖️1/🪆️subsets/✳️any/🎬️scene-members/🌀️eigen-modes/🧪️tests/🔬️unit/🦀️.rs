use super::*;
use crate::model::En1993Snapshot;

fn compliant() -> En1993Snapshot {
    En1993Snapshot::compliant_heb240_frame()
}

fn strut() -> (crate::model::SteelMember, crate::model::SteelSection, crate::model::SteelMaterial) {
    let d = compliant();
    let member = crate::model::SteelMember { member_type: semio_s_artifact_norm_en1993_model::SteelMemberType::Beam, length: 6.0, moment_diagram: semio_s_artifact_norm_en1993_model::MomentDiagram::Uniform, end_moment_ratio_psi: 1.0, ltb_restraint_spacing: 0.0, load_application: semio_s_artifact_norm_en1993_model::LoadApplication::ShearCenter, ..d.members[0].clone() };
    let section = d.sections.iter().find(|s| s.id == member.section_id).expect("section").clone();
    let material = d.materials.iter().find(|m| m.id == member.material_id).expect("material").clone();
    (member, section, material)
}

fn modes_oracle(member: &crate::model::SteelMember, section: &crate::model::SteelSection, material: &crate::model::SteelMaterial, moment: f64) -> std::collections::BTreeMap<String, f64> {
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🌀️modes.py");
    let args: Vec<String> = [material.e_modulus, material.g_modulus, section.iy, section.iz, section.it, section.iw, member.length, moment].iter().map(|v| format!("{v:e}")).collect();
    let refs: Vec<&std::ffi::OsStr> = args.iter().map(|a| a.as_ref()).collect();
    let output = crate::norm::python_oracle::run_ok(script, &refs, None);
    String::from_utf8_lossy(&output.stdout).lines().filter_map(|l| l.split_once(' ')).map(|(k, v)| (k.to_string(), v.parse().expect("number"))).collect()
}

fn near(actual: f64, expected: f64, tolerance: f64, what: &str) {
    assert!((actual - expected).abs() <= tolerance * expected.abs(), "{what}: {actual} vs {expected}");
}

#[test]
fn the_flexural_mode_is_the_kernel_eigenmode_with_the_euler_load() {
    let (member, section, material) = strut();
    let frame = crate::kit::member_frame(&crate::scene::Vec3::ZERO, &crate::kit::v3(1.0, 0.0, 0.0));
    let mode = flexural(&member, &section, &material, &frame).expect("mode");
    near(mode.factor, modes_oracle(&member, &section, &material, 1.0)["euler"], 2e-3, "Ncr");
    let weak = mode.samples.iter().map(|s| s.delta.y.abs()).fold(0.0, f64::max);
    let strong = mode.samples.iter().map(|s| s.delta.z.abs()).fold(0.0, f64::max);
    assert!(weak > 0.99 && strong < 1e-6, "buckles about the weak axis: {weak} {strong}");
    let mid = mode.samples.iter().min_by(|a, b| (a.at - 0.5).abs().total_cmp(&(b.at - 0.5).abs())).expect("sample");
    near(mid.delta.y.abs(), 1.0, 1e-3, "midspan amplitude");
}

#[test]
fn a_fixed_base_column_buckles_at_a_quarter_of_the_pinned_load() {
    let (mut member, section, material) = strut();
    let frame = crate::kit::member_frame(&crate::scene::Vec3::ZERO, &crate::kit::v3(1.0, 0.0, 0.0));
    let pinned = flexural(&member, &section, &material, &frame).expect("pinned").factor;
    member.member_type = semio_s_artifact_norm_en1993_model::SteelMemberType::Column;
    near(flexural(&member, &section, &material, &frame).expect("column").factor, pinned / 4.0, 2e-3, "cantilever Ncr");
}

#[test]
fn the_lateral_torsional_mode_has_the_elastic_critical_moment() {
    let (member, section, material) = strut();
    let frame = crate::kit::member_frame(&crate::scene::Vec3::ZERO, &crate::kit::v3(1.0, 0.0, 0.0));
    let mode = lateral_torsional(&member, &section, &material, &frame, 100_000.0).expect("mode");
    near(mode.factor, modes_oracle(&member, &section, &material, 1.0)["mcr"], 3e-3, "Mcr");
    assert!(mode.samples.first().is_some_and(|s| s.delta.y.abs() < 1e-9) && mode.samples.last().is_some_and(|s| s.delta.y.abs() < 1e-9));
    let braced = crate::model::SteelMember { ltb_restraint_spacing: 2.0, ..member.clone() };
    let stiffer = lateral_torsional(&braced, &section, &material, &frame, 100_000.0).expect("braced");
    assert!(stiffer.factor > 2.0 * mode.factor, "restraints raise Mcr: {} vs {}", stiffer.factor, mode.factor);
}

#[test]
fn the_deflection_line_is_the_static_solution_of_the_serviceability_moment() {
    let (member, section, material) = strut();
    let frame = crate::kit::member_frame(&crate::scene::Vec3::ZERO, &crate::kit::v3(1.0, 0.0, 0.0));
    let line = deflection(&member, &section, &material, &frame, 100_000.0).expect("deflection");
    let mid = line.samples.iter().min_by(|a, b| (a.at - 0.5).abs().total_cmp(&(b.at - 0.5).abs())).expect("sample");
    assert!(mid.delta.z < 0.0, "a sagging beam deflects downwards");
    near(mid.delta.z.abs(), modes_oracle(&member, &section, &material, 100_000.0)["deflection"], 1e-3, "midspan deflection");
    near(line.factor * mid.delta.z.abs(), DEFLECTION_TARGET * member.length, 1e-6, "display magnification");
}
