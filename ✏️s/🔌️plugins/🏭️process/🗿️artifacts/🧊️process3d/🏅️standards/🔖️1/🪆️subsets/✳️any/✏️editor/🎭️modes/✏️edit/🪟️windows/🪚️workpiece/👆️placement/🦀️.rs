//! 👆️ Wireframe of the next capability-sized placement.

use semio_framework::DslValue;
use semio_s_artifact_process_process3d_inferences::{capability_for_measure_kind, find_capability, measure_for_capability, stock_extent, validate_capability, validation_context_for_stock};
use semio_s_artifact_process_process3d_model::snapshot::{Capability, MeasureKind, Process3dSnapshot, ProcessMeasure, WorkingSolid, WorkshopMachine};

pub const PLACE_CAPABILITY_UTILITY: &str = "placeCapability";

/// 🎯️ The utility ids participating in click placement and its preview.
pub fn placement_kind(utility: &str) -> Option<MeasureKind> {
    match utility {
        "cut" => Some(MeasureKind::Cut),
        "drill" => Some(MeasureKind::Drill),
        "attach" => Some(MeasureKind::Attach),
        _ => None,
    }
}

/// 🎛️ A window utility with pointer-based placement.
pub fn is_placement_utility(utility: &str) -> bool {
    utility == PLACE_CAPABILITY_UTILITY || placement_kind(utility).is_some()
}

/// 🔍️ Resolves a chosen capability from authored workshop machines and the current stock.
fn available_workshop_capability<'a>(snapshot: &'a Process3dSnapshot, machine_id: &str, capability_id: &str) -> Option<(&'a WorkshopMachine, &'a Capability)> {
    let (machine, capability) = find_capability(&snapshot.workshop, machine_id, capability_id)?;
    let stock = validation_context_for_stock(&snapshot.stock_payload)?;
    validate_capability(capability, &stock).is_empty().then_some((machine, capability))
}

/// 🧰️ Builds the same positioned measure for the preview and the committed step.
pub fn resolve_placement<T>(snapshot: &Process3dSnapshot, utility: &str, chosen: Option<(&str, &str)>, position: [f64; 3], use_measure: impl FnOnce(&str, &Capability, ProcessMeasure) -> T) -> Option<T> {
    if utility == PLACE_CAPABILITY_UTILITY {
        let (machine, capability) = chosen.and_then(|(machine, capability)| available_workshop_capability(snapshot, machine, capability))?;
        let measure = measure_for_capability(capability, Some(position));
        let dimensions = match &measure {
            ProcessMeasure::Cut { tool, .. } | ProcessMeasure::Attach { component: tool, .. } => stock_extent(tool)?,
            ProcessMeasure::Drill { radius, depth, .. } => [*radius, *radius, *depth],
        };
        dimensions.into_iter().all(|dimension| dimension.is_finite() && dimension > 0.0)
            .then(|| use_measure(&machine.id, capability, measure))
    } else {
        let (machine, capability) = capability_for_measure_kind(&snapshot.workshop, placement_kind(utility)?);
        Some(use_measure(&machine.id, &capability, measure_for_capability(&capability, Some(position))))
    }
}

/// 📐️ The preview uses the upcoming measure's own dimensions without evaluating a Boolean.
pub fn preview_json(snapshot: &Process3dSnapshot, utility: &str, chosen: Option<(&str, &str)>, position: [f64; 3]) -> Option<String> {
    if !position.into_iter().all(f64::is_finite) || matches!(snapshot.stock_payload.solid, WorkingSolid::ImportedMesh { .. }) {
        return None;
    }
    let measure = resolve_placement(snapshot, utility, chosen, position, |_, _, measure| measure)?;
    let role = measure.kind_slug();
    let solid = match measure {
        ProcessMeasure::Cut { tool, .. } => tool,
        ProcessMeasure::Attach { component, .. } => component,
        ProcessMeasure::Drill { radius, depth, .. } => WorkingSolid::Cylinder { radius, height: depth },
    };
    let positive = |value: f64| value.is_finite() && value > 0.0;
    let mut items = Vec::new();
    let mut segment = |from: [f64; 3], to: [f64; 3]| {
        let translate = |coordinate: [f64; 3]| std::array::from_fn::<_, 3, _>(|i| coordinate[i] + position[i]);
        let (from, to) = (translate(from), translate(to));
        if from.into_iter().chain(to).all(f64::is_finite) {
            let point = |value: [f64; 3]| DslValue::Array(value.into_iter().map(DslValue::float).collect());
            items.push(DslValue::object([
                ("kind".into(), DslValue::String("segment".into())),
                ("role".into(), DslValue::String(role.into())),
                ("from".into(), point(from)),
                ("to".into(), point(to)),
            ]));
        }
    };
    match solid {
        WorkingSolid::Box { width, depth, height } if [width, depth, height].into_iter().all(positive) => {
            let half = [width / 2.0, depth / 2.0, height / 2.0];
            let corner = |bits: usize| std::array::from_fn(|axis| if bits & (1 << axis) == 0 { -half[axis] } else { half[axis] });
            for bits in 0..8 {
                for axis in 0..3 {
                    if bits & (1 << axis) == 0 {
                        segment(corner(bits), corner(bits | (1 << axis)));
                    }
                }
            }
        }
        WorkingSolid::Cylinder { radius, height } if positive(radius) && positive(height) => {
            let ring = |index, z| {
                let (sin, cos) = (std::f64::consts::TAU * f64::from(index) / 12.0).sin_cos();
                [radius * cos, radius * sin, z]
            };
            let half = height / 2.0;
            for index in 0..12 {
                for z in [-half, half] {
                    segment(ring(index, z), ring((index + 1) % 12, z));
                }
                if index % 3 == 0 {
                    segment(ring(index, -half), ring(index, half));
                }
            }
        }
        _ => return None,
    }
    (!items.is_empty()).then(|| semio_framework_pack_json::to_json_string(&DslValue::Array(items)))
}
