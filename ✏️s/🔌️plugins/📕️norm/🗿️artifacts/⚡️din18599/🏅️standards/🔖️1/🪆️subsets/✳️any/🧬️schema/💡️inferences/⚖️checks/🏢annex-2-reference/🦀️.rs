//! 🏢 Envelope reference values for GModG §18 and Annex 2 items 1.1 to 1.11.

use super::{Adjacency, ElementKind, EnvelopeElement};

/// 🪟 Reference transmittances and bridge allowance for a non-residential component.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Annex2EnvelopeValues {
    pub clause: &'static str,
    pub u_value_w_m2k: f64,
    pub solar_transmittance: Option<f64>,
    pub visible_transmittance: Option<f64>,
    pub bridge_surcharge_w_m2k: f64,
}

fn opaque(clause: &'static str, warm: f64, reduced: f64, normal_temperature: bool, bridge: f64) -> Annex2EnvelopeValues {
    Annex2EnvelopeValues {
        clause,
        u_value_w_m2k: if normal_temperature { warm } else { reduced },
        solar_transmittance: None,
        visible_transmittance: None,
        bridge_surcharge_w_m2k: bridge,
    }
}

fn glazed(clause: &'static str, u: f64, g: f64, visible: f64, bridge: f64) -> Annex2EnvelopeValues {
    Annex2EnvelopeValues {
        clause,
        u_value_w_m2k: u,
        solar_transmittance: Some(g),
        visible_transmittance: Some(visible),
        bridge_surcharge_w_m2k: bridge,
    }
}

/// 🌡️ Annex 2 reference envelope at an entered design heating setpoint, before solar-control glazing substitutions.
pub fn annex2_reference_values(kind: ElementKind, adjacency: Adjacency, heating_setpoint_c: f64) -> Option<Annex2EnvelopeValues> {
    if !heating_setpoint_c.is_finite() || heating_setpoint_c < 12.0 || adjacency == Adjacency::Heated {
        return None;
    }
    let normal = heating_setpoint_c >= 19.0;
    let bridge = if normal { 0.05 } else { 0.10 };
    Some(match kind {
        ElementKind::Wall | ElementKind::Floor if adjacency == Adjacency::Outdoor => opaque("1.1", 0.28, 0.35, normal, bridge),
        ElementKind::CurtainWall if normal => glazed("1.2", 1.4, 0.48, 0.72, 0.0),
        ElementKind::CurtainWall => glazed("1.2", 1.9, 0.60, 0.78, 0.0),
        ElementKind::Wall | ElementKind::Floor | ElementKind::GroundSlabPerimeter | ElementKind::GroundSlabInterior => opaque("1.3", 0.35, 0.35, normal, bridge),
        ElementKind::Roof => opaque("1.4", 0.20, 0.35, normal, bridge),
        ElementKind::RoofGlazing => glazed("1.5", 2.7, 0.63, 0.76, bridge),
        ElementKind::RooflightStrip => glazed("1.6", 2.4, 0.55, 0.48, bridge),
        ElementKind::RoofDome => glazed("1.7", 2.7, 0.64, 0.59, bridge),
        ElementKind::Window | ElementKind::GlazedDoor => glazed("1.8", if normal { 1.3 } else { 1.9 }, 0.60, 0.78, bridge),
        ElementKind::RoofWindow => glazed("1.9", if normal { 1.4 } else { 1.9 }, 0.60, 0.78, bridge),
        ElementKind::Door => opaque("1.10", 1.8, 2.9, normal, bridge),
    })
}

/// 📐 Sum the Annex 2 bridge surcharge over the stated surfaces, excluding curtain walls and internal components.
pub(super) fn annex2_reference_bridge_w_k(elements: &[&EnvelopeElement], setpoint_c: f64) -> f64 {
    elements.iter().filter_map(|element| {
        annex2_reference_values(element.kind, element.adjacency, setpoint_c)
            .map(|value| element.area_m2 * value.bridge_surcharge_w_m2k)
    }).sum()
}
