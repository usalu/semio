//! 🔥️ Thermal and fire: the uniform and linear temperature actions on the building or the bridge deck (EN 1991-1-5) and the fire compartment with its gas temperature (EN 1991-1-2).

use super::kit::*;
use super::layout::DECK_Z;
use super::model::{En1991Snapshot, FireLoadBasis, FireMode};
use super::scene::*;
use super::{checks, handles, Ctx};
use checks::{part_1_2, part_1_5};

pub const FIRE_MARGIN: f64 = 1.0;
/// 🌡️ Layers, drawn height (m, exaggerated), side (m) and offset from the structure (m) of the temperature strip that shows the linear difference ΔT_M through the depth.
const PROFILE_LAYERS: usize = 10;
const PROFILE_HEIGHT: f64 = 3.0;
const PROFILE_SIDE: f64 = 0.6;
const PROFILE_OFFSET: f64 = 1.5;
/// 🆔 Id of the temperature field of the strip layers.
pub const PROFILE_FIELD: &str = "thermal-profile";
const PARAMETRIC_T_LIM_H: f64 = 20.0 / 60.0;

/// 🧱 Wall thickness drawn per unit of thermal inertia b (m per J/(m²·s½·K)): 0.2 m at the default b = 1160, so the wall thickens with b and the grip edits b along the wall normal. Display scale only, not a normative value.
pub const WALL_THICKNESS_PER_B: f64 = 0.2 / 1160.0;
/// 🪟 Window height as a share of the compartment height; display only, the opening area A_v follows from the opening factor.
const WINDOW_HEIGHT_RATIO: f64 = 0.6;

/// 🏠 The drawn compartment: floor side, height, the window of the opening factor on the front wall and the wall thickness of the thermal inertia.
pub struct Compartment {
    pub side: f64,
    pub height: f64,
    pub window_width: f64,
    pub window_height: f64,
    pub window_sill: f64,
    pub window_offset: f64,
    pub opening_area: f64,
    pub total_area: f64,
    pub wall: f64,
}

/// 📐 The compartment of the document: window width from A_v = O·A_t/√h (the entered opening factor, else the geometry value of the square model), wall thickness from b.
pub fn compartment(d: &En1991Snapshot) -> Compartment {
    let side = d.fire_compartment_area.max(1.0).sqrt().min(d.depth.min(d.width) - 2.0 * FIRE_MARGIN).max(1.0);
    let height = d.fire_compartment_height.max(0.5);
    let (total_area, geometry_area, equivalent) = part_1_2::compartment_geometry(d.fire_compartment_area, height);
    let opening_area = if d.fire_opening_factor > 1e-9 { d.fire_opening_factor * total_area / equivalent.sqrt() } else { geometry_area };
    let window_height = (height * WINDOW_HEIGHT_RATIO).max(0.3).min(height);
    let window_width = (opening_area / window_height).min(side - 0.4).max(0.1);
    Compartment { side, height, window_width, window_height, window_sill: (height - window_height) / 2.0, window_offset: (side - window_width) / 2.0, opening_area, total_area, wall: (d.fire_thermal_inertia * WALL_THICKNESS_PER_B).clamp(0.05, 0.6) }
}

/// 🧯 Characteristic and design fire load density (J/m²) of the chosen basis, `None` when the basis is not satisfiable.
pub fn fire_load_j_m2(d: &En1991Snapshot) -> Option<(f64, f64)> {
    let characteristic = part_1_2::characteristic_fire_load_for_basis_j_m2(d.fire_load_basis, d.fire_load_density_qf, d.annex, d.fire_occupancy)?;
    Some((characteristic, part_1_2::design_fire_load_j_m2(d.fire_load_basis, d.fire_load_density_qf, d.annex, d.fire_occupancy, d.fire_compartment_area, 1.0)?))
}

/// 🔥 Required fire gas temperature in K at the design duration: nominal curve or Annex A parametric curve, `None` when the inputs are inadmissible.
pub fn gas_temperature_k(d: &En1991Snapshot) -> Option<f64> {
    match d.fire_mode {
        FireMode::None => None,
        FireMode::Nominal => part_1_2::nominal_gas_temp_k(d.fire_curve, d.fire_duration),
        FireMode::Parametric => {
            let qfd = part_1_2::design_fire_load_j_m2(d.fire_load_basis, d.fire_load_density_qf, d.annex, d.fire_occupancy, d.fire_compartment_area, 1.0)?;
            let opening = (d.fire_opening_factor > 1e-9).then_some(d.fire_opening_factor);
            let (theta, fire) = part_1_2::parametric_gas_temp_k(d.fire_duration, d.fire_compartment_area, d.fire_compartment_height, d.fire_thermal_inertia, qfd, opening, PARAMETRIC_T_LIM_H);
            Some(theta.max(fire.theta_max_c + 273.15))
        }
    }
}

pub fn thermal(ctx: &Ctx, out: &mut Out) {
    if !out.own(global::THERMAL) {
        return;
    }
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let bridge = d.structure_kind == super::model::StructureKind::Bridge;
    let (target, anchor) = if bridge {
        (entity_ref(EntityKind::Solid, "bridge", "@site"), layout.bridge.at(d.bridge_span / 2.0, super::layout::deck_width(d) / 2.0, DECK_Z + 3.0))
    } else {
        (entity_ref(EntityKind::Solid, "building", "@site"), layout.building.at(d.depth / 2.0, d.width / 2.0, d.height + super::envelope::RoofShape::of(d, d.roofs.first()).rise() + 3.0))
    };
    let bound = binding_prefixed(report, &["en1991.1-5."]);
    let grips = handles::ids(d, layout, "thermal");
    if let Some(delta) = part_1_5::required_delta_t(&d.thermal_element_type, d.t_max, d.t_min, d.t_0, d.thermal_bridge_type, d.delta_t_m).filter(|v| v.is_finite()) {
        out.push(
            Spec::new(
                EntityKind::Load,
                "thermal",
                "@site",
                Geometry::Load(LoadGlyph::Thermal { target: target.clone(), delta_t: Quantity::new(QuantityKind::Temperature, delta), case: case("thermal-uniform", LoadKind::Thermal) }),
                load(LoadKind::Thermal),
                loc("Uniform temperature action", "Gleichmäßige Temperatureinwirkung"),
                LAYER_THERMAL,
            )
            .bound(bound.clone())
            .detail(bi(|de| format!("ΔT = {} K, T_max = {} °C, T_min = {} °C, T_0 = {} °C", num(delta, 1, de), num(d.t_max, 1, de), num(d.t_min, 1, de), num(d.t_0, 1, de))))
            .handles(grips.clone()),
            &[anchor.clone()],
        );
        out.label("thermal:label".into(), anchor.clone(), bi(|de| format!("ΔT = {} K", num(delta, 1, de))), tone_of(&bound));
    }
    if d.delta_t_m.abs() > 1e-9 {
        out.push(
            Spec::new(
                EntityKind::Load,
                "thermal:gradient",
                "@site",
                Geometry::Load(LoadGlyph::Thermal { target, delta_t: Quantity::new(QuantityKind::Temperature, d.delta_t_m), case: case("thermal-gradient", LoadKind::Thermal) }),
                load(LoadKind::Thermal),
                loc("Linear temperature gradient", "Linearer Temperaturgradient"),
                LAYER_THERMAL,
            )
            .bound(bound.clone())
            .detail(bi(|de| format!("ΔT_M = {} K", num(d.delta_t_m, 1, de))))
            .handles(grips),
            &[anchor],
        );
        let (pose, origin) = if bridge { (&layout.bridge, [d.bridge_span + PROFILE_OFFSET, super::layout::deck_width(d) / 2.0 - PROFILE_SIDE / 2.0, DECK_Z + 0.5]) } else { (&layout.building, [d.depth + PROFILE_OFFSET, d.width / 2.0 - PROFILE_SIDE / 2.0, 0.0]) };
        let layer = PROFILE_HEIGHT / PROFILE_LAYERS as f64;
        for k in 0..PROFILE_LAYERS {
            let (z0, delta) = (origin[2] + PROFILE_HEIGHT - (k + 1) as f64 * layer, d.delta_t_m * (0.5 - (k as f64 + 0.5) / PROFILE_LAYERS as f64));
            out.push(
                Spec::new(
                    EntityKind::Solid,
                    format!("thermal:profile-{k}"),
                    "@site",
                    Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([origin[0], origin[1], z0]), base: polygon2(&rect2(0.0, 0.0, PROFILE_SIDE, PROFILE_SIDE)), height: layer } }),
                    MaterialRole::Concrete,
                    loc("Temperature difference through the depth", "Temperaturdifferenz über die Höhe"),
                    LAYER_THERMAL,
                )
                .bound(bound.clone())
                .channel(ColourChannel::Fields { active: PROFILE_FIELD.into(), fields: vec![ScalarField { id: PROFILE_FIELD.into(), unit: QuantityKind::Temperature, samples: FieldSamples::Instance { values: vec![delta] } }] })
                .detail(bi(|de| format!("ΔT(z) = {} K ({})", num(delta, 2, de), if de { "oben minus Mittel; Höhe überhöht dargestellt, linear über die Tiefe" } else { "top minus mean; depth exaggerated, linear through the depth" }))),
                &[pose.at(origin[0], origin[1], z0), pose.at(origin[0] + PROFILE_SIDE, origin[1] + PROFILE_SIDE, z0 + layer)],
            );
        }
        out.label("thermal:profile-label".into(), pose.at(origin[0] + PROFILE_SIDE / 2.0, origin[1] + PROFILE_SIDE / 2.0, origin[2] + PROFILE_HEIGHT + 0.3), bi(|de| format!("{} {} K", if de { "Oben/unten ΔT_M =" } else { "Top/bottom ΔT_M =" }, num(d.delta_t_m, 1, de))), tone_of(&bound));
    }
}

pub fn fire(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    if !out.own(global::FIRE) || d.fire_mode == FireMode::None {
        return;
    }
    let pose = &layout.building;
    let room = compartment(d);
    let (side, height) = (room.side, room.height);
    let bound = binding_prefixed(report, &["en1991.1-2."]);
    let grips = handles::ids(d, layout, "fire");
    let corners = [pose.at(FIRE_MARGIN, FIRE_MARGIN, 0.0), pose.at(FIRE_MARGIN + side, FIRE_MARGIN + side, height)];
    let compartment_ref = entity_ref(EntityKind::Solid, "fire", "@site");
    let fire_load = fire_load_j_m2(d);
    let basis = d.fire_load_basis;
    let mut volume = Spec::new(
        EntityKind::Solid,
        "fire",
        "@site",
        Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([FIRE_MARGIN, FIRE_MARGIN, 0.0]), base: polygon2(&rect2(0.0, 0.0, side, side)), height } }),
        load(LoadKind::Thermal),
        loc("Fire compartment", "Brandabschnitt"),
        LAYER_FIRE,
    )
    .bound(bound.clone())
    .opacity(0.4)
    .detail(bi(|de| {
        let curve = match d.fire_mode { FireMode::Parametric => if de { "parametrische Brandkurve" } else { "parametric fire" }, _ => if de { "nominelle Brandkurve" } else { "nominal fire curve" } };
        let source = match (basis, fire_load) {
            (FireLoadBasis::OccupancyTable, Some((qk, qd))) => format!("{} {:?} ({}), q_f,k = {} MJ/m², q_f,d = {} MJ/m²", if de { "Nutzungstabelle E.4" } else { "occupancy table E.4" }, d.fire_occupancy, if de { "80-%-Fraktile" } else { "80 % fractile" }, num(qk / 1e6, 0, de), num(qd / 1e6, 0, de)),
            (FireLoadBasis::UserSpecified, Some((qk, qd))) => format!("{}, q_f,k = {} MJ/m², q_f,d = {} MJ/m²", if de { "vorgegebenes q_f,k" } else { "user-specified q_f,k" }, num(qk / 1e6, 0, de), num(qd / 1e6, 0, de)),
            (_, None) => (if de { "Brandlastgrundlage nicht erfüllbar" } else { "fire load basis not satisfiable" }).to_string(),
        };
        format!("A_f = {} m², H = {} m, {}; {}", num(d.fire_compartment_area, 1, de), num(d.fire_compartment_height, 2, de), curve, source)
    }))
    .handles(grips.clone());
    volume = match fire_load {
        Some((qk, qd)) => volume.channel(super::fields::fire_load::channel(qk, qd)),
        None => volume.channel(ColourChannel::Role),
    };
    out.push(volume, &corners);
    let wall_loop: Vec<Vec3> = [[0.0, 0.0], [side, 0.0], [side, side], [0.0, side], [0.0, 0.0]].iter().map(|p| pose.at(FIRE_MARGIN + p[0], FIRE_MARGIN + p[1], 0.0)).collect();
    out.push(
        Spec::new(
            EntityKind::Solid,
            "fire:wall",
            "@site",
            Geometry::Solid(SolidGeometry { shape: SolidShape::WallAssembly { path: Polyline3 { points: wall_loop.clone() }, height, layers: vec![WallLayer { thickness: room.wall, role: MaterialRole::Concrete, label: loc("Compartment wall", "Brandabschnittswand") }], openings: vec![WallCut { offset: room.window_offset, width: room.window_width, sill: room.window_sill, height: room.window_height }] } }),
            MaterialRole::Concrete,
            loc("Compartment wall", "Brandabschnittswand"),
            LAYER_FIRE,
        )
        .bound(bound.clone())
        .detail(bi(|de| format!("b = {} J/(m²s½K), {} {} m", num(d.fire_thermal_inertia, 0, de), if de { "Wanddicke (Anzeige)" } else { "wall thickness (display)" }, num(room.wall, 2, de))))
        .handles(grips.clone()),
        &wall_loop,
    );
    let pane = [pose.at(FIRE_MARGIN + room.window_offset, FIRE_MARGIN, room.window_sill), pose.at(FIRE_MARGIN + room.window_offset + room.window_width, FIRE_MARGIN, room.window_sill)];
    out.push(
        Spec::new(
            EntityKind::Solid,
            "fire:window",
            "@site",
            Geometry::Solid(SolidGeometry { shape: SolidShape::WallAssembly { path: Polyline3 { points: pane.to_vec() }, height: room.window_height, layers: vec![WallLayer { thickness: 0.03, role: MaterialRole::Glass, label: loc("Opening", "Öffnung") }], openings: Vec::new() } }),
            MaterialRole::Glass,
            loc("Compartment opening", "Brandabschnittsöffnung"),
            LAYER_FIRE,
        )
        .bound(bound.clone())
        .opacity(0.35)
        .detail(bi(|de| format!("O = {} m½, A_v = {} m², A_t = {} m²", num(d.fire_opening_factor, 4, de), num(room.opening_area, 1, de), num(room.total_area, 1, de))))
        .handles(grips.clone()),
        &pane,
    );
    let top = pose.at(FIRE_MARGIN + side / 2.0, FIRE_MARGIN + side / 2.0, height);
    if let Some(theta) = gas_temperature_k(d).filter(|v| v.is_finite()) {
        out.push(
            Spec::new(
                EntityKind::Load,
                "fire:gas",
                "@site",
                Geometry::Load(LoadGlyph::Thermal { target: compartment_ref, delta_t: Quantity::new(QuantityKind::Temperature, theta - part_1_2::AMBIENT_K), case: case("fire-gas", LoadKind::Accidental) }),
                load(LoadKind::Accidental),
                loc("Fire gas temperature", "Brandgastemperatur"),
                LAYER_FIRE,
            )
            .bound(bound.clone())
            .detail(bi(|de| format!("θ_g = {} °C, t = {} min", num(theta - 273.15, 0, de), num(d.fire_duration / 60.0, 0, de))))
            .handles(grips),
            &[top.clone()],
        );
        out.label("fire:label".into(), top.clone(), bi(|de| format!("θ_g = {} °C ({} min)", num(theta - 273.15, 0, de), num(d.fire_duration / 60.0, 0, de))), tone_of(&bound));
    }
    out.annotate(
        Annotation::Dimension { id: "fire:height".into(), from: pose.at(FIRE_MARGIN, FIRE_MARGIN, 0.0), to: pose.at(FIRE_MARGIN, FIRE_MARGIN, height), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, height), binding: Some(FieldBinding::new("fireCompartmentHeight", Affine::IDENTITY, "m")) },
        &top,
    );
    super::dynamics::fire(d, out);
}
