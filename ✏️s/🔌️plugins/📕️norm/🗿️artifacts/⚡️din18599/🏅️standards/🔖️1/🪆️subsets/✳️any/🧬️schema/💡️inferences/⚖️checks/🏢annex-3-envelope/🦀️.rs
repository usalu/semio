//! 🏢 GModG §19 and Annex 3 envelope groups for non-residential buildings.

use super::{Adjacency, Din18599Snapshot, ElementKind, EnvelopeElement};
use std::collections::BTreeMap;

/// 🧱 Annex 3 envelope class in table order.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Annex3EnvelopeClass {
    Opaque,
    Transparent,
    CurtainWall,
    RoofGlazing,
}

impl Annex3EnvelopeClass {
    pub const ALL: [Self; 4] = [Self::Opaque, Self::Transparent, Self::CurtainWall, Self::RoofGlazing];

    pub const fn clause(self) -> &'static str {
        match self {
            Self::Opaque => "1",
            Self::Transparent => "2",
            Self::CurtainWall => "3",
            Self::RoofGlazing => "4",
        }
    }

    pub const fn labels(self) -> (&'static str, &'static str) {
        match self {
            Self::Opaque => ("Opaque exterior components", "Opake Außenbauteile"),
            Self::Transparent => ("Transparent exterior components", "Transparente Außenbauteile"),
            Self::CurtainWall => ("Curtain walls", "Vorhangfassaden"),
            Self::RoofGlazing => ("Roof glazing and rooflights", "Glasdächer, Lichtbänder und Lichtkuppeln"),
        }
    }
}

/// 🌡 Annex 3 mean transmittance limit for a given heating setpoint and envelope class.
pub fn annex3_limit(class: Annex3EnvelopeClass, setpoint_c: f64) -> Option<f64> {
    if !setpoint_c.is_finite() || setpoint_c < 12.0 {
        return None;
    }
    let (normal, reduced) = match class {
        Annex3EnvelopeClass::Opaque => (0.28, 0.50),
        Annex3EnvelopeClass::Transparent => (1.5, 2.8),
        Annex3EnvelopeClass::CurtainWall => (1.5, 3.0),
        Annex3EnvelopeClass::RoofGlazing => (2.5, 3.1),
    };
    Some(if setpoint_c >= 19.0 { normal } else { reduced })
}

/// 🧮 Area-weighted transmittance. The adjacency multiplier applies to U in the numerator.
pub fn annex3_mean_u(rows: impl IntoIterator<Item = (f64, f64, f64)>) -> Option<f64> {
    let mut area_sum = 0.0;
    let mut weighted_u_sum = 0.0;
    for (area, u, adjacency_multiplier) in rows {
        if !area.is_finite() || area <= 0.0 || !u.is_finite() || u < 0.0 || !matches!(adjacency_multiplier, 0.5 | 1.0) {
            return None;
        }
        area_sum += area;
        weighted_u_sum += area * u * adjacency_multiplier;
    }
    let mean: f64 = weighted_u_sum / area_sum;
    (area_sum.is_finite() && mean.is_finite()).then_some(mean)
}

/// 📐 An assessed envelope class at a single heating setpoint.
pub struct Annex3EnvelopeGroup<'a> {
    pub class: Annex3EnvelopeClass,
    pub setpoint_c: f64,
    pub elements: Vec<&'a EnvelopeElement>,
}

impl Annex3EnvelopeGroup<'_> {
    pub fn check_id(&self) -> String {
        format!("din18599.geg.mean-u.{}.{:016x}", self.class.clause(), self.setpoint_c.to_bits())
    }

    pub fn limit(&self) -> f64 {
        annex3_limit(self.class, self.setpoint_c).expect("Annex 3 group has a heating setpoint in scope")
    }

    pub fn mean_u(&self) -> Option<f64> {
        let rows = self.elements.iter().map(|e| {
            let adjacent_factor = match e.adjacency {
                Adjacency::Outdoor | Adjacency::UnheatedAttic => 1.0,
                Adjacency::Unheated | Adjacency::Ground => 0.5,
                Adjacency::Heated => unreachable!("internal elements cannot form an Annex 3 group"),
            };
            let area = match (e.kind, e.adjacency) {
                (ElementKind::Floor, Adjacency::Ground) => f64::NAN,
                _ => e.area_m2,
            };
            (area, e.u_value_w_m2k, adjacent_factor)
        });
        annex3_mean_u(rows)
    }
}

/// 🏗 Groups the recorded exterior elements where both statutory class and setpoint are known.
pub fn annex3_envelope_groups(doc: &Din18599Snapshot) -> (Vec<Annex3EnvelopeGroup<'_>>, bool) {
    let temperatures: BTreeMap<_, _> = doc.zones.iter().map(|z| (z.id.as_str(), z.theta_i_heat_c)).collect();
    let mut missing_basis = false;
    let mut groups: BTreeMap<_, Annex3EnvelopeGroup<'_>> = BTreeMap::new();
    for element in doc.elements.iter().filter(|e| e.adjacency != Adjacency::Heated) {
        let Some(&setpoint_c) = temperatures.get(element.zone_id.as_str()).filter(|v| v.is_finite()) else {
            missing_basis = true;
            continue;
        };
        if setpoint_c < 12.0 {
            continue;
        }
        if matches!(element.kind, ElementKind::GroundSlabPerimeter | ElementKind::GroundSlabInterior)
            && element.adjacency != Adjacency::Ground
        {
            missing_basis = true;
            continue;
        }
        let class = match element.kind {
            ElementKind::GroundSlabInterior => continue,
            ElementKind::Window | ElementKind::GlazedDoor | ElementKind::RoofWindow => Annex3EnvelopeClass::Transparent,
            ElementKind::CurtainWall => Annex3EnvelopeClass::CurtainWall,
            ElementKind::RoofGlazing | ElementKind::RoofDome | ElementKind::RooflightStrip => Annex3EnvelopeClass::RoofGlazing,
            ElementKind::Wall | ElementKind::Roof | ElementKind::Floor | ElementKind::Door | ElementKind::GroundSlabPerimeter => Annex3EnvelopeClass::Opaque,
        };
        let group = groups.entry((class, setpoint_c.to_bits())).or_insert_with(|| Annex3EnvelopeGroup {
            class,
            setpoint_c,
            elements: Vec::new(),
        });
        group.elements.push(element);
    }
    (groups.into_values().collect(), missing_basis)
}
