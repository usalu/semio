//! 📋️ FEM 3D clipboard: the copied nodes and members as one JSON fragment, and the paste that re-creates them under fresh ids. See [`super`].

use super::*;
use semio_framework::kernel::{ClipboardError, ClipboardFragment, PasteAnchor, PastePlacement};
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::{fem3d_element_endpoints, fem3d_entity_id, fem3d_entity_kind, FEM3D_GRANULARITY_ELEMENT, FEM3D_GRANULARITY_NODE, FEM3D_GRANULARITY_SOLID};
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{element_id, FemAxis, FemElement, FemSolid, FEM_3D_SCHEMA};
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element::CreateElement;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node::CreateNode;
use semio_s_artifact_fem_3d_mesh_solids::standards::v1::subsets::any::schema::mutations::create_solid::CreateSolid;
use semio_s_fem_node::volumetric::FemNodeValue;

//#region 🔖️Fem3dClipboard
/// 🎨️ The clipboard media type of FEM 3D structure fragments.
pub fn fem3d_clipboard_media_type() -> MediaType {
    MediaType { class: MediaClass::ThreeD, form: MediaForm::Design }
}

/// 📋️ Copies every selected node, member and solid, plus the end nodes of each selected member, in document order.
pub fn fem3d_copy_fragment(doc: &Fem3dSnapshot, selected: &[String]) -> Result<ClipboardFragment, ClipboardError> {
    let mut node_ids = std::collections::BTreeSet::new();
    let mut element_ids = std::collections::BTreeSet::new();
    let mut solid_ids = std::collections::BTreeSet::new();
    for identity in selected {
        let Some(id) = fem3d_entity_id(identity) else { continue };
        match fem3d_entity_kind(doc, identity) {
            Some(FEM3D_GRANULARITY_NODE) => { node_ids.insert(id.to_string()); }
            Some(FEM3D_GRANULARITY_ELEMENT) => { element_ids.insert(id.to_string()); }
            Some(FEM3D_GRANULARITY_SOLID) => { solid_ids.insert(id.to_string()); }
            _ => {}
        }
    }
    let elements: Vec<&FemElement> = doc.elements.iter().filter(|element| element_ids.contains(element_id(element))).collect();
    for element in &elements {
        let (start, end) = fem3d_element_endpoints(element);
        node_ids.insert(start.to_string());
        node_ids.insert(end.to_string());
    }
    let nodes: Vec<Value> = doc.nodes.iter().filter(|node| node_ids.contains(node.id.as_str())).map(|node| semio_framework_pack_json::json!({ "id": node.id.as_str(), "x": node.x, "y": node.y, "z": node.z })).collect();
    let solids: Vec<Value> = doc.solids.iter().filter(|solid| solid_ids.contains(&solid.id)).map(|solid| semio_framework_pack_json::json!({ "id": solid.id, "name": solid.name, "outline": solid.outline, "holes": solid.holes, "baseZ": solid.base_z, "height": solid.height, "layers": solid.layers, "meshSize": solid.mesh_size, "materialId": solid.material_id, "axis": solid.axis.key() })).collect();
    if nodes.is_empty() && elements.is_empty() && solids.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let elements: Vec<Value> = elements.into_iter().map(|element| match element {
        FemElement::Bar { id, start, end, material_id, section_id } => semio_framework_pack_json::json!({ "kind": "bar", "id": id, "start": start, "end": end, "materialId": material_id, "sectionId": section_id }),
        FemElement::Frame { id, start, end, material_id, section_id, roll } => semio_framework_pack_json::json!({ "kind": "frame", "id": id, "start": start, "end": end, "materialId": material_id, "sectionId": section_id, "roll": roll }),
    }).collect();
    let label = format!("{} nodes, {} members, {} solids", nodes.len(), elements.len(), solids.len());
    Ok(ClipboardFragment {
        schema: FEM_3D_SCHEMA.to_string(),
        media_type: fem3d_clipboard_media_type(),
        dsl_text: semio_framework_pack_json::to_string(&semio_framework_pack_json::json!({ "nodes": nodes, "elements": elements, "solids": solids })),
        pack_bytes: None,
        source_app: "fem3d".to_string(),
        label,
    })
}

/// ✂️ Removes exactly what [`fem3d_copy_fragment`] copies: the selected nodes, members and solids.
pub fn fem3d_cut_operations(doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, NoConfig>, selected: &[String]) -> Vec<Fem3dMutation> {
    let ids = selected.iter().filter(|identity| matches!(fem3d_entity_kind(doc.snapshot, identity), Some(FEM3D_GRANULARITY_NODE | FEM3D_GRANULARITY_ELEMENT | FEM3D_GRANULARITY_SOLID))).cloned().collect();
    remove_selection::handle(&remove_selection::RemoveSelection { ids }, doc, cfg).map(|emit| emit.artifact_mutations).unwrap_or_default()
}

/// 📌️ Re-creates the copied nodes, members and solids under fresh ids, shifted to the placement position unless the anchor keeps the original place.
pub fn fem3d_paste_operations(doc: &ArtifactView<'_, Fem3dSnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Fem3dMutation>, ClipboardError> {
    let parsed: Value = semio_framework_pack_json::parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    let text = |row: &Value, key: &str| row.get(key).and_then(Value::as_str).map(str::to_string).ok_or_else(|| ClipboardError::ParseFailed(format!("clipboard row lacks '{key}'")));
    let number = |row: &Value, key: &str| row.get(key).and_then(Value::as_f64).ok_or_else(|| ClipboardError::ParseFailed(format!("clipboard row lacks '{key}'")));
    let rows = |key: &str| parsed.get(key).and_then(Value::as_array).cloned().unwrap_or_default();
    let nodes = rows("nodes").iter().map(|row| Ok((text(row, "id")?, [number(row, "x")?, number(row, "y")?, number(row, "z")?]))).collect::<Result<Vec<_>, ClipboardError>>()?;
    let snapshot = doc.snapshot;
    let operation = doc.operation().map_err(|fault| ClipboardError::ParseFailed(fault.message))?;
    let fresh = |prefix: &str, kind: &str, old: &str| operation.entity_id(prefix, &format!("paste:{kind}:{old}")).map_err(|fault| ClipboardError::ParseFailed(fault.message));
    let solid_rows = rows("solids").iter().map(|row| parse_solid(row)).collect::<Result<Vec<_>, ClipboardError>>()?;
    let corners = nodes.iter().map(|(_, point)| *point).chain(solid_rows.iter().flat_map(|solid| solid.corners())).collect::<Vec<_>>();
    let shift = match (placement.anchor, placement.position) {
        (PasteAnchor::Original, _) | (_, None) => [0.0; 3],
        (_, Some(_)) if corners.is_empty() => [0.0; 3],
        (_, Some(position)) => {
            let bounds = |axis: usize| corners.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), point| (low.min(point[axis]), high.max(point[axis])));
            [0, 1, 2].map(|axis| { let (low, high) = bounds(axis); position[axis] - (low + high) * 0.5 })
        }
    };
    let mut mutations = Vec::new();
    let mut renamed = std::collections::HashMap::new();
    for (old, point) in &nodes {
        let id = fresh("n", "node", old)?;
        renamed.insert(old.clone(), id.clone());
        mutations.push(Fem3dMutation::CreateNode(CreateNode { node: FemNodeValue { id, x: point[0] + shift[0], y: point[1] + shift[1], z: point[2] + shift[2] }, index: None }));
    }
    let material = snapshot.materials.iter().map(|material| material.id.as_str().to_string()).collect::<Vec<_>>();
    let section = snapshot.sections.iter().map(|section| section.id.clone()).collect::<Vec<_>>();
    let known = |wanted: String, available: &[String]| if available.contains(&wanted) { Some(wanted) } else { available.first().cloned() };
    for row in rows("elements") {
        let (Some(start), Some(end)) = (renamed.get(&text(&row, "start")?), renamed.get(&text(&row, "end")?)) else { continue };
        let (Some(material_id), Some(section_id)) = (known(text(&row, "materialId")?, &material), known(text(&row, "sectionId")?, &section)) else { continue };
        let id = fresh("e", "element", &text(&row, "id")?)?;
        let element = match row.get("kind").and_then(Value::as_str) {
            Some("frame") => FemElement::Frame { id, start: start.clone(), end: end.clone(), material_id, section_id, roll: number(&row, "roll")? },
            _ => FemElement::Bar { id, start: start.clone(), end: end.clone(), material_id, section_id },
        };
        mutations.push(Fem3dMutation::CreateElement(CreateElement { element: Box::new(element), index: None }));
    }
    for solid in solid_rows {
        let id = fresh("sol", "solid", &solid.id)?;
        let material_id = known(solid.material_id.clone(), &material);
        let Some(material_id) = material_id else { continue };
        let ([du, dv], dw) = solid.axis.from_world(shift);
        let moved = |ring: &[[f64; 2]]| ring.iter().map(|[u, v]| [u + du, v + dv]).collect::<Vec<_>>();
        mutations.push(Fem3dMutation::CreateSolid(CreateSolid { solid: FemSolid { id, outline: moved(&solid.outline), holes: solid.holes.iter().map(|ring| moved(ring)).collect(), base_z: solid.base_z + dw, material_id, ..solid }, index: None }));
    }
    Ok(mutations)
}

trait SolidCorners {
    fn corners(&self) -> Vec<[f64; 3]>;
}

impl SolidCorners for FemSolid {
    fn corners(&self) -> Vec<[f64; 3]> {
        self.outline.iter().flat_map(|[u, v]| [self.axis.to_world(*u, *v, self.base_z), self.axis.to_world(*u, *v, self.base_z + self.height)]).collect()
    }
}

fn parse_solid(row: &Value) -> Result<FemSolid, ClipboardError> {
    let missing = |key: &str| ClipboardError::ParseFailed(format!("clipboard row lacks '{key}'"));
    let text = |key: &str| row.get(key).and_then(Value::as_str).map(str::to_string).ok_or_else(|| missing(key));
    let number = |key: &str| row.get(key).and_then(Value::as_f64).ok_or_else(|| missing(key));
    let ring = |value: &Value| value.as_array().map(|points| points.iter().map(|point| { let pair = point.as_array()?; Some([pair.first()?.as_f64()?, pair.get(1)?.as_f64()?]) }).collect::<Option<Vec<_>>>()).flatten().ok_or_else(|| missing("outline"));
    Ok(FemSolid {
        id: text("id")?,
        name: text("name")?,
        outline: ring(row.get("outline").ok_or_else(|| missing("outline"))?)?,
        holes: row.get("holes").and_then(Value::as_array).ok_or_else(|| missing("holes"))?.iter().map(ring).collect::<Result<Vec<_>, _>>()?,
        base_z: number("baseZ")?,
        height: number("height")?,
        layers: number("layers")? as usize,
        mesh_size: number("meshSize")?,
        material_id: text("materialId")?,
        axis: FemAxis::from_key(&text("axis")?).ok_or_else(|| missing("axis"))?,
    })
}
//#endregion 🔖️Fem3dClipboard

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
