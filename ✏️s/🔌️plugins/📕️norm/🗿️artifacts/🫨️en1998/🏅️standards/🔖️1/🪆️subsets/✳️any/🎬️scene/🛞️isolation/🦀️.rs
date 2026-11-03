//! 🛞 Isolation systems and pier bases in 3D: an isolation system is a spring support at the deck of its bridge or under its building with translucent ghosts at ±d_cd and the draggable capacity and demand dimensions, coloured by its checks; a pier base is the foundation spring of Gazetas under the pier with its circular footing and the radius dimension.

use super::ground_and_bridges::{isolation_anchor, DECK_THICKNESS};
use super::handles::Index;
use super::kit::*;
use super::layout::Layout;
use super::response::{annex_params, site};
use super::scene::*;
use super::schema::isolation::Bilinear;
use super::schema::{part_2, soil_structure};
use super::SceneInput;

const GLYPH: f64 = 0.4;
const GHOST_OPACITY: f64 = 0.3;
const FOOTING_THICKNESS: f64 = 0.4;
const DIMENSION_OFFSET: f64 = 1.2;

pub fn project(input: &SceneInput, layout: &Layout, handles: &Index, out: &mut Out) {
    let d = input.document;
    for system in &d.isolation_systems {
        if out.own("isolationSystems", &system.id) {
            isolation_system(input, layout, handles, system, out);
        }
    }
    for base in &d.pier_bases {
        if out.own("pierBases", &base.id) {
            pier_base(input, layout, handles, base, out);
        }
    }
}

fn stiffness(values: [f64; 6]) -> Vec<Quantity> {
    values.iter().map(|v| Quantity::new(QuantityKind::Stiffness, *v)).collect()
}

fn isolation_system(input: &SceneInput, layout: &Layout, handles: &Index, system: &super::model::En1998IsolationSystem, out: &mut Out) {
    let Some(at) = isolation_anchor(input.document, layout, system, GLYPH) else { return };
    let law = Bilinear { strength_n: system.strength_n, post_yield_n_per_m: system.stiffness_n_per_m, yield_displacement_m: system.yield_displacement_m };
    if !law.is_valid() {
        return;
    }
    let path = path_of("isolationSystems", &system.id);
    let demand = computed_of(input.report, &format!("en1998.isolation.{}.displacement", system.id)).filter(|d| *d > 0.0);
    let k_eff = demand.map_or_else(|| law.elastic_stiffness(), |d| law.force_n(d) / d);
    let binding = binding_prefix(input.report, &format!("en1998.isolation.{}.", system.id));
    let name = loc(&format!("Isolation system {}: Q_d = {:.0} N, K_d = {:.3e} N/m, d_y = {:.4} m", system.id, system.strength_n, system.stiffness_n_per_m, system.yield_displacement_m), &format!("Isolationssystem {}: Q_d = {:.0} N, K_d = {:.3e} N/m, d_y = {:.4} m", system.id, system.strength_n, system.stiffness_n_per_m, system.yield_displacement_m));
    let mut spec = Spec::new(EntityKind::Support, system.id.clone(), path.clone(), Geometry::Support(SupportGeometry { kind: SupportKind::Spring { stiffness: stiffness([k_eff, k_eff, 0.0, 0.0, 0.0, 0.0]) }, at: at.clone(), frame: world_frame(at.clone()) }), MaterialRole::Support, name, LAYER_ISOLATION);
    spec.binding = binding;
    spec.handles = handles.of(&system.id);
    spec.detail = loc(&format!("Effective stiffness K_eff = {k_eff:.3e} N/m{}", demand.map_or(String::new(), |d| format!(" at d_cd = {d:.4} m"))), &format!("Effektive Steifigkeit K_eff = {k_eff:.3e} N/m{}", demand.map_or(String::new(), |d| format!(" bei d_cd = {d:.4} m"))));
    out.push(spec, &[at.clone()]);
    if let Some(d_cd) = demand {
        for (tag, sign) in [("plus", 1.0), ("minus", -1.0)] {
            let origin = v3(at.x + sign * d_cd - GLYPH / 2.0, at.y - GLYPH / 2.0, at.z - GLYPH / 2.0);
            let mut ghost = Spec::new(EntityKind::Solid, format!("{}:ghost-{tag}", system.id), path.clone(), box_solid(origin.clone(), v3(GLYPH, GLYPH, GLYPH)), MaterialRole::Neutral, loc(&format!("Isolator displaced by {}d_cd = {:.1} mm", if sign > 0.0 { "+" } else { "\u{2212}" }, d_cd * 1000.0), &format!("Isolator verschoben um {}d_cd = {:.1} mm", if sign > 0.0 { "+" } else { "\u{2212}" }, d_cd * 1000.0)), LAYER_ISOLATION);
            ghost.opacity = GHOST_OPACITY;
            ghost.binding = binding_exact(input.report, &format!("en1998.isolation.{}.displacement", system.id));
            out.push(ghost, &[origin.clone(), v3(origin.x + GLYPH, origin.y + GLYPH, origin.z + GLYPH)]);
        }
        let from = v3(at.x, at.y - DIMENSION_OFFSET, at.z);
        out.annotate(Annotation::Dimension { id: format!("{}:demand", system.id), from: from.clone(), to: v3(from.x + d_cd, from.y, from.z), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, d_cd), binding: None }, &from);
    }
    let capacity_from = v3(at.x, at.y - 2.0 * DIMENSION_OFFSET, at.z);
    out.annotate(Annotation::Dimension { id: format!("{}:capacity", system.id), from: capacity_from.clone(), to: v3(capacity_from.x + system.capacity_m, capacity_from.y, capacity_from.z), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, system.capacity_m), binding: Some(FieldBinding::new(format!("{path}.capacityM"), Affine::IDENTITY, "m")) }, &capacity_from);
    let period = computed_of(input.report, &format!("en1998.isolation.{}.maxPeriod", system.id));
    let damping = computed_of(input.report, &format!("en1998.isolation.{}.damping", system.id));
    if let (Some(period), Some(damping)) = (period, damping) {
        let anchor = v3(at.x, at.y, at.z - GLYPH);
        out.annotate(Annotation::Label { id: format!("{}:effective", system.id), anchor: anchor.clone(), text: loc(&format!("T_eff = {period:.2} s, \u{3be}_eff = {:.1} %, K_eff = {k_eff:.2e} N/m", 100.0 * damping), &format!("T_eff = {period:.2} s, \u{3be}_eff = {:.1} %, K_eff = {k_eff:.2e} N/m", 100.0 * damping)), tone: Tone::Info }, &anchor);
    }
    if let Some(clause) = binding_exact(input.report, &format!("en1998.isolation.{}.displacement", system.id)).and_then(|b| clause_of(input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", system.id), anchor: entity_ref(EntityKind::Support, &system.id, &path), clause }, &at);
    }
}

fn pier_base(input: &SceneInput, layout: &Layout, handles: &Index, base: &super::model::En1998PierBase, out: &mut Out) {
    let d = input.document;
    let (Some(bridge), Some(pier)) = (d.bridges.iter().find(|b| b.id == base.bridge_id), d.piers.iter().find(|p| p.id == base.pier_id && p.bridge_id == base.bridge_id)) else { return };
    let o = layout.origin(&bridge.id);
    let at = v3(o.x + pier.x_m, o.y + DECK_WIDTH / 2.0, o.z);
    let gamma_i = part_2::bridge_gamma_i(bridge.importance_class);
    let s = site(d);
    let annex = annex_params(d).choice();
    let ratio = soil_structure::table_4_1_vs_ratio(gamma_i * s.a_gr * s.s / 9.81, annex);
    let g = soil_structure::degraded_shear_modulus_pa(soil_structure::shear_modulus_max_pa(base.soil_unit_weight_n_m3, base.soil_vs_m_s), ratio);
    let Some(springs) = soil_structure::Springs::circular(g, base.radius_m, base.poisson_ratio) else { return };
    let path = path_of("pierBases", &base.id);
    let binding = binding_prefix(input.report, &format!("en1998.2.{}.base.{}.", bridge.id, base.id));
    let name = loc(&format!("Pier base {} of {}: K_x = {:.3e} N/m, K_rx = {:.3e} N\u{b7}m/rad", base.id, pier.id, springs.horizontal_n_per_m, springs.rocking_nm_per_rad), &format!("Pfeilerfu\u{df} {} von {}: K_x = {:.3e} N/m, K_rx = {:.3e} N\u{b7}m/rad", base.id, pier.id, springs.horizontal_n_per_m, springs.rocking_nm_per_rad));
    let mut spec = Spec::new(EntityKind::Support, base.id.clone(), path.clone(), Geometry::Support(SupportGeometry { kind: SupportKind::Spring { stiffness: stiffness([springs.horizontal_n_per_m, springs.horizontal_n_per_m, springs.vertical_n_per_m, springs.rocking_nm_per_rad, springs.rocking_nm_per_rad, springs.torsion_nm_per_rad]) }, at: at.clone(), frame: world_frame(at.clone()) }), MaterialRole::Support, name, LAYER_ISOLATION);
    spec.binding = binding;
    spec.handles = handles.of(&base.id);
    spec.detail = loc(&format!("G = {g:.3e} Pa (v_s/v_s,max = {ratio:.2}), R = {:.2} m, \u{3bd} = {:.2}", base.radius_m, base.poisson_ratio), &format!("G = {g:.3e} Pa (v_s/v_s,max = {ratio:.2}), R = {:.2} m, \u{3bd} = {:.2}", base.radius_m, base.poisson_ratio));
    out.push(spec, &[at.clone()]);
    let foot = v3(at.x, at.y, at.z - FOOTING_THICKNESS);
    let mut footing = Spec::new(EntityKind::Solid, format!("{}:footing", base.id), path.clone(), Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: world_frame(foot.clone()), radius: base.radius_m, height: FOOTING_THICKNESS } }), MaterialRole::Concrete, loc(&format!("Circular footing R = {:.2} m under pier {}", base.radius_m, pier.id), &format!("Kreisfundament R = {:.2} m unter Pfeiler {}", base.radius_m, pier.id)), LAYER_ISOLATION);
    footing.opacity = 0.6;
    out.push(footing, &[v3(foot.x - base.radius_m, foot.y - base.radius_m, foot.z), v3(foot.x + base.radius_m, foot.y + base.radius_m, at.z)]);
    let from = v3(at.x, at.y + base.radius_m + 0.5, at.z);
    out.annotate(Annotation::Dimension { id: format!("{}:radius", base.id), from: from.clone(), to: v3(from.x + base.radius_m, from.y, from.z), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, base.radius_m), binding: Some(FieldBinding::new(format!("{path}.radiusM"), Affine::IDENTITY, "m")) }, &from);
    if let Some(t_ssi) = computed_of(input.report, &format!("en1998.2.{}.base.{}.period", bridge.id, base.id)) {
        let anchor = v3(at.x, at.y, at.z + DECK_THICKNESS);
        out.annotate(Annotation::Label { id: format!("{}:period", base.id), anchor: anchor.clone(), text: loc(&format!("T_ssi = {t_ssi:.3} s on the foundation springs"), &format!("T_ssi = {t_ssi:.3} s auf den Fundamentfedern")), tone: Tone::Info }, &anchor);
    }
}
