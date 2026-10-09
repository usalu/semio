//! 🛝️ The ramp rows of the entity table: how a ramp reads off the snapshot and its inferred run, how an edited value becomes a `set-ramp` mutation, what a new ramp is, and the host rows of a railing
//! (the picker of the stair, ramp or slab it follows, its side, its slab edge and its inset) that become a `set-railing` mutation.

use super::{container, first, loop_text, number, parse_flag, parse_number, parse_point, parse_text, parse_top, partial, rename_element, zoning, Created, FieldRow, InferredRow};
use crate::editor::bim::terminology::BimLabels;
use crate::mutations::set_railing::SetRailing;
use crate::{Assigned, HostSide, ModelMutation, ModelSnapshot, Point2, RailingHost, RailingPatch, Vertex};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

/// 📏️ How far a new ramp stands from the previous ones of its storey, in metres.
const ROW_PITCH: f64 = 2.0;
/// 📏️ The length of a new ramp, in metres: two landings and seven metres of slope for half a metre of rise at 1:12.
const LENGTH: f64 = 10.0;
/// 📏️ The rise of a new ramp, in metres.
const RISE: f64 = 0.5;
/// 📏️ The inset of a new host, in metres.
const HOST_INSET: f64 = 0.05;

//#region 🔖️Path
/// 🛝️ The vertices a text names: at least two `x, y` or `x, y ⌒ bulge` vertices joined by `;`.
pub fn parse_path(text: &str) -> Option<Vec<Vertex>> {
    let vertices = text
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let (at, bulge) = match part.split_once('⌒') {
                Some((at, bulge)) => (at, parse_number(bulge)?),
                None => (part, 0.0),
            };
            Some(Vertex { point: parse_point(at)?, bulge })
        })
        .collect::<Option<Vec<_>>>()?;
    (vertices.len() >= 2).then_some(vertices)
}
//#endregion 🔖️Path

//#region 🔖️Hosts
/// 🪝️ The elements a railing can follow: every stair, ramp and slab of the model by its id and name, and nothing to release it.
pub fn host_choices(snapshot: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    let named = snapshot.stairs.iter().map(|(id, row)| (id.clone(), row.name.clone())).chain(snapshot.ramps.iter().map(|(id, row)| (id.clone(), row.name.clone()))).chain(snapshot.slabs.iter().map(|(id, row)| (id.clone(), row.name.clone())));
    std::iter::once((String::new(), labels.choice_none.as_str().to_string())).chain(named).collect()
}

/// 🧭️ The sides of a host a railing can stand on, by their stored name and their localized label.
pub fn side_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    vec![("Left".to_string(), labels.choice_left.as_str().to_string()), ("Right".to_string(), labels.choice_right.as_str().to_string())]
}

fn host_of<'a>(snapshot: &'a ModelSnapshot, id: &str) -> Option<&'a RailingHost> {
    snapshot.railings.get(id)?.host.as_ref()
}

fn hosting(id: &str, path: Option<Vec<Point2>>, host: Option<RailingHost>) -> Option<ModelMutation> {
    Some(ModelMutation::SetRailing(SetRailing::from_patch(id.into(), RailingPatch { path, host: Some(Assigned::new(host)), ..Default::default() })))
}

/// 🪝️ A new host for a railing: an element picks the host (a hosted railing has no path of its own, so the path goes with it), empty text releases it (the railing gets a metre of path back).
pub fn write_host(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let element = value.trim();
    snapshot.railings.get(id)?;
    if element.is_empty() {
        return hosting(id, Some(vec![Point2 { x: 0.0, y: 0.0 }, Point2 { x: 1.0, y: 0.0 }]), None);
    }
    let previous = host_of(snapshot, id).cloned().unwrap_or(RailingHost { element: String::new(), side: HostSide::Left, edge: 0, inset: HOST_INSET });
    hosting(id, Some(Vec::new()), Some(RailingHost { element: element.into(), ..previous }))
}

/// 🧭️ The side of the host a railing stands on; refused for a railing without host.
pub fn write_host_side(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let side = super::variant(value, &[HostSide::Left, HostSide::Right])?;
    hosting(id, None, Some(RailingHost { side, ..host_of(snapshot, id)?.clone() }))
}

/// 📐️ The slab edge a hosted railing follows; refused for a railing without host.
pub fn write_host_edge(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let edge = value.trim().parse::<u32>().ok()?;
    hosting(id, None, Some(RailingHost { edge, ..host_of(snapshot, id)?.clone() }))
}

/// 📏️ How far a hosted railing stands inside the edge of its host; refused for a railing without host.
pub fn write_host_inset(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let inset = parse_number(value)?;
    hosting(id, None, Some(RailingHost { inset, ..host_of(snapshot, id)?.clone() }))
}
//#endregion 🔖️Hosts

//#region 🔖️Fields
/// 🧾️ The authored parameters of a ramp.
pub static RAMP_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.ramps.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.ramps.get(id).map(|row| row.storey.clone())),
    field!("path", field_path, Text, |s, id| s.ramps.get(id).map(|row| loop_text(&row.path)), parse_path => set_ramp::SetRamp),
    field!("width", field_width, Number, |s, id| s.ramps.get(id).map(|row| number(row.width)), parse_number => set_ramp::SetRamp),
    field!("landing_start", field_landing_start, Number, |s, id| s.ramps.get(id).map(|row| number(row.landing_start)), parse_number => set_ramp::SetRamp),
    field!("landing_end", field_landing_end, Number, |s, id| s.ramps.get(id).map(|row| number(row.landing_end)), parse_number => set_ramp::SetRamp),
    field!("landing_turn", field_landing_turn, Number, |s, id| s.ramps.get(id).map(|row| number(row.landing_turn)), parse_number => set_ramp::SetRamp),
    field!("max_slope", field_max_slope, Number, |s, id| s.ramps.get(id).map(|row| number(row.max_slope)), parse_number => set_ramp::SetRamp),
    field!("thickness", field_thickness, Number, |s, id| s.ramps.get(id).map(|row| number(row.thickness)), parse_number => set_ramp::SetRamp),
    field!("material", field_material, Text, |s, id| s.ramps.get(id).map(|row| row.material.clone()), choices: zoning::material_choices, parse_text => set_ramp::SetRamp),
    field!("base_offset", field_base_offset, Number, |s, id| s.ramps.get(id).map(|row| number(row.base_offset)), parse_number => set_ramp::SetRamp),
    field!("top", field_top, Text, |s, id| s.ramps.get(id).map(|row| super::top_text(&row.top)), parse_top => set_ramp::SetRamp),
    field!("railing_left", field_rail_left, Text, |s, id| s.ramps.get(id).map(|row| row.railing_left.to_string()), parse_flag => set_ramp::SetRamp),
    field!("railing_right", field_rail_right, Text, |s, id| s.ramps.get(id).map(|row| row.railing_right.to_string()), parse_flag => set_ramp::SetRamp),
];

/// 💡️ The inferred run of a ramp: its length, rise, sloped run, slope and landings, whether it holds its limit, and its areas and volume.
pub static RAMP_INFERRED: &[InferredRow] = &[
    inferred!("length", field_length, |_, inference, id| inference.ramp_runs.get(id).filter(|run| run.length > 0.0).map(|run| number(run.length))),
    inferred!("rise", field_rise, |_, inference, id| inference.ramp_runs.get(id).map(|run| number(run.rise))),
    inferred!("run_length", field_run_length, |_, inference, id| inference.ramp_runs.get(id).filter(|run| run.run_length > 0.0).map(|run| number(run.run_length))),
    inferred!("slope", field_ramp_slope, |_, inference, id| inference.ramp_runs.get(id).map(|run| format!("{:.1}", run.slope * 100.0))),
    inferred!("landings", field_landings, |_, inference, id| inference.ramp_runs.get(id).map(|run| run.landings.len().to_string())),
    inferred!("compliant", field_compliant, |_, inference, id| inference.ramp_runs.get(id).map(|run| run.compliance.compliant.to_string())),
    inferred!("area", field_area, |_, inference, id| inference.quantities.elements.get(id).map(|row| row.area()).filter(|area| *area > 0.0).map(number)),
    inferred!("volume", field_volume, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.net_volume > 0.0).map(|row| number(row.net_volume))),
];
//#endregion 🔖️Fields

//#region 🔖️Create
/// 🛝️ A new ramp: ten metres long along +X, half a metre of rise, the standard width, thickness, landings and 1:12 limit, of the first material of the project; it stands beside the ramps already on its storey.
pub fn create_ramp(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    let row = snapshot.ramps.values().filter(|ramp| ramp.storey == storey).count() as f64 * ROW_PITCH;
    let vertex = |x: f64| Vertex { point: Point2 { x, y: row }, bulge: 0.0 };
    Ok(ModelMutation::CreateRamp(crate::mutations::create_ramp::CreateRamp {
        id: id.into(),
        ramp: crate::Ramp {
            storey,
            path: vec![vertex(0.0), vertex(LENGTH)],
            width: crate::STANDARD_RAMP_WIDTH,
            landing_start: crate::STANDARD_RAMP_LANDING,
            landing_end: crate::STANDARD_RAMP_LANDING,
            landing_turn: crate::STANDARD_RAMP_LANDING,
            max_slope: crate::STANDARD_RAMP_MAX_SLOPE,
            thickness: crate::STANDARD_RAMP_THICKNESS,
            material,
            base_offset: 0.0,
            top: crate::TopConstraint::Unconnected { height: RISE },
            railing_left: false,
            railing_right: false,
            name: name.into(),
        },
    }))
}
//#endregion 🔖️Create
