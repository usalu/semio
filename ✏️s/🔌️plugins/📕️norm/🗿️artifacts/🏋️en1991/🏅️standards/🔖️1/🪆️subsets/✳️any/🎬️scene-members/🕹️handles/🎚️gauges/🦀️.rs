//! 🎚️ Gauges: the viewport grips of the gauge tables ([`fields::gauges`](crate::fields::gauges)): a rack of diamonds above the first geometric grip of each owner, one per numeric field without a geometric grip. The drag parameter `s` is the displacement in metres along `+Z` of the gauge anchor, so the field is `value + 20 * step * s`, quantised to its step and held inside its physical range.
//! 🔗️ Handle laws: [`assert_handle_laws`](crate::scene::projection_kit::test_support::assert_handle_laws).

use super::*;
use crate::fields::gauges::*;
pub use crate::fields::gauges::{coverage, value_at, Contract, EXEMPT};

const PITCH: f64 = 0.4;
const LIFT: f64 = 0.8;

/// 🎚️ The gauges of one owner as handles: a rack of diamonds above `base`, the `i`-th beside the previous one. `prefix` is the path head of a row (`collection[id=ID].`) or empty for a document scalar; `id` makes the names of the rows distinguishable.
fn rack<T>(table: &[Gauge<T>], row: &T, prefix: &str, id: &str, owner: &EntityRef, base: &Vec3, out: &mut Vec<Handle>) {
    let n = table.len() as f64;
    for (i, g) in table.iter().enumerate() {
        let value = (g.get)(row);
        let tag = format!("gauge-{}", g.tag);
        let suffix = if id.is_empty() { String::new() } else { format!(" ({id})") };
        let (en, de) = (format!("{}{suffix}", g.en), format!("{}{suffix}", g.de));
        out.push(
            Grip {
                owner,
                tag: &tag,
                kind: HandleKind::ScalarAlong { axis: Vec3::Z },
                anchor: add(base, &v3(0.0, (i as f64 - (n - 1.0) / 2.0) * PITCH, LIFT)),
                style: GripStyle::Diamond,
                path: format!("{prefix}{}", g.field),
                unit: g.unit,
                base: value,
                k: STEPS_PER_METRE * g.step,
                round: if g.int { Round::Int } else { Round::Step(g.step) },
                range: (g.lo.min(value), g.hi.max(value)),
                names: (en.as_str(), de.as_str()),
            }
            .build(),
        );
    }
}

/// 🎚️ The anchor of the thermal entity: above the deck of a bridge, above the roof of a building.
fn thermal_anchor(d: &Site, layout: &Layout) -> Vec3 {
    if d.structure_kind == StructureKind::Bridge {
        layout.bridge.at(d.bridge_span / 2.0, deck_width(d) / 2.0, DECK_Z + 3.0)
    } else {
        layout.building.at(d.depth / 2.0, d.width / 2.0, d.height + RoofShape::of(d, d.roofs.first()).rise() + 3.0)
    }
}

/// 🎚️ The gauges of the entity `id`, racked above the first geometric grip of the entity (above the entity itself for the thermal action); an entity that advertises no geometric grip has none, because it is not drawn.
pub fn of(d: &Site, layout: &Layout, id: &str, geometric: &[Handle]) -> Vec<Handle> {
    let mut out = Vec::new();
    if let Some(Owner::Thermal) = owner_of(d, id) {
        rack(THERMAL, d, "", "", &entity_ref(EntityKind::Load, "thermal", "@site"), &thermal_anchor(d, layout), &mut out);
        return out;
    }
    let Some(first) = geometric.first() else { return out };
    let (owner, base) = (&first.entity, &first.anchor);
    macro_rules! row {
        ($table:expr, $row:expr, $collection:literal) => {
            rack($table, $row, &format!("{}[id={}].", $collection, $row.id), &$row.id, owner, base, &mut out)
        };
    }
    match owner_of(d, id) {
        Some(Owner::Building) => {
            rack(SITE, d, "", "", owner, base, &mut out);
        }
        Some(Owner::Bridge) => rack(BRIDGE, d, "", "", owner, base, &mut out),
        Some(Owner::Crane) => rack(CRANE, d, "", "", owner, base, &mut out),
        Some(Owner::Silo) => rack(SILO, d, "", "", owner, base, &mut out),
        Some(Owner::Fire) => rack(FIRE, d, "", "", owner, base, &mut out),
        Some(Owner::Floor(r)) => row!(FLOORS, r, "floors"),
        Some(Owner::SelfWeight(r)) => row!(SELF_WEIGHT_ELEMENTS, r, "selfWeightElements"),
        Some(Owner::Roof(_, r)) => row!(ROOFS, r, "roofs"),
        Some(Owner::Face(r)) => row!(WIND_FACES, r, "windFaces"),
        Some(Owner::Stage(r)) => row!(CONSTRUCTION_STAGES, r, "constructionStages"),
        Some(Owner::BridgeWind(r)) => row!(WIND_BRIDGES, r, "windBridges"),
        Some(Owner::Dynamic(r)) => row!(DYNAMIC_ELEMENTS, r, "dynamicElements"),
        Some(Owner::Impact(r)) => row!(IMPACT_ACTIONS, r, "impactActions"),
        Some(Owner::Explosion(r)) => row!(INTERNAL_EXPLOSIONS, r, "internalExplosions"),
        Some(Owner::Section(r)) => row!(THERMAL_SECTIONS, r, "thermalSections"),
        Some(Owner::Helicopter(r)) => row!(HELICOPTER_LANDINGS, r, "helicopterLandings"),
        Some(Owner::Tie(r)) => row!(TIE_RUNS, r, "tieRuns"),
        Some(Owner::ConstructionLoad(r)) => row!(CONSTRUCTION_LOADS, r, "constructionLoads"),
        Some(Owner::Falling(r)) => row!(FALLING_LOADS, r, "fallingLoads"),
        Some(Owner::Imperfection(r)) => row!(EXECUTION_IMPERFECTIONS, r, "executionImperfections"),
        Some(Owner::Derailment(r)) => row!(DERAILMENT_CASES, r, "derailmentCases"),
        _ => {}
    }
    out
}
