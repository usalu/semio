//! 📋️ FEM 2D clipboard: the copied nodes, members and regions as one JSON fragment, and the paste that re-creates them under fresh ids. See [`super`].

use super::*;
use semio_framework::kernel::{ClipboardError, ClipboardFragment, PasteAnchor, PastePlacement};
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::{fem2d_element_endpoints, fem2d_entity_kind, FEM2D_GRANULARITY_ELEMENT, FEM2D_GRANULARITY_NODE, FEM2D_GRANULARITY_REGION};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::{element_id, FemElement, FemRegion, FEM_2D_SCHEMA};
use semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element::CreateElement;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node::CreateNode;
use semio_s_fem_node::planar::FemNodeValue;

//#region 🔖️Fem2dClipboard
/// 🎨️ The clipboard media type of FEM 2D structure fragments.
pub fn fem2d_clipboard_media_type() -> MediaType {
    MediaType { class: MediaClass::TwoD, form: MediaForm::Design }
}

fn fem2d_entity_id(identity: &str) -> Option<String> {
    match protocol::wire::decode_scoped_identity(identity).ok()?.as_slice() {
        [_, id] => Some((*id).to_string()),
        _ => None,
    }
}

/// 📋️ Copies every selected node, member and region, plus the end nodes of each selected member, in document order.
pub fn fem2d_copy_fragment(doc: &Fem2dSnapshot, selected: &[String]) -> Result<ClipboardFragment, ClipboardError> {
    let mut node_ids = std::collections::BTreeSet::new();
    let mut element_ids = std::collections::BTreeSet::new();
    let mut region_ids = std::collections::BTreeSet::new();
    for identity in selected {
        let Some(id) = fem2d_entity_id(identity) else { continue };
        match fem2d_entity_kind(doc, identity) {
            Some(FEM2D_GRANULARITY_NODE) => { node_ids.insert(id); }
            Some(FEM2D_GRANULARITY_ELEMENT) => { element_ids.insert(id); }
            Some(FEM2D_GRANULARITY_REGION) => { region_ids.insert(id); }
            _ => {}
        }
    }
    let elements: Vec<&FemElement> = doc.elements.iter().filter(|element| element_ids.contains(element_id(element))).collect();
    for element in &elements {
        let (start, end) = fem2d_element_endpoints(element);
        node_ids.insert(start.to_string());
        node_ids.insert(end.to_string());
    }
    let nodes: Vec<Value> = doc.nodes.iter().filter(|node| node_ids.contains(node.id.as_str())).map(|node| semio_framework_pack_json::json!({ "id": node.id.as_str(), "x": node.x, "y": node.y })).collect();
    let regions: Vec<Value> = doc.regions.iter().filter(|region| region_ids.contains(&region.id)).map(|region| semio_framework_pack_json::json!({ "id": region.id, "name": region.name, "outline": region.outline, "holes": region.holes, "thickness": region.thickness, "materialId": region.material_id, "meshSize": region.mesh_size })).collect();
    if nodes.is_empty() && elements.is_empty() && regions.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let elements: Vec<Value> = elements.into_iter().map(|element| match element {
        FemElement::Bar { id, start, end, material_id, section_id } => semio_framework_pack_json::json!({ "kind": "bar", "id": id, "start": start, "end": end, "materialId": material_id, "sectionId": section_id }),
        FemElement::Beam { id, start, end, material_id, section_id } => semio_framework_pack_json::json!({ "kind": "beam", "id": id, "start": start, "end": end, "materialId": material_id, "sectionId": section_id }),
    }).collect();
    let label = format!("{} nodes, {} members, {} regions", nodes.len(), elements.len(), regions.len());
    Ok(ClipboardFragment {
        schema: FEM_2D_SCHEMA.to_string(),
        media_type: fem2d_clipboard_media_type(),
        dsl_text: semio_framework_pack_json::to_string(&semio_framework_pack_json::json!({ "nodes": nodes, "elements": elements, "regions": regions })),
        pack_bytes: None,
        source_app: "fem2d".to_string(),
        label,
    })
}

/// ✂️ Removes exactly what [`fem2d_copy_fragment`] copies: the selected nodes, members and regions.
pub fn fem2d_cut_operations(doc: &ArtifactView<'_, Fem2dSnapshot>, selected: &[String]) -> Vec<Fem2dMutation> {
    let ids: Vec<String> = selected.iter().filter(|identity| matches!(fem2d_entity_kind(doc.snapshot, identity), Some(FEM2D_GRANULARITY_NODE | FEM2D_GRANULARITY_ELEMENT | FEM2D_GRANULARITY_REGION))).cloned().collect();
    remove_selection::remove_ids(doc, &ids).map(|emit| emit.artifact_mutations).unwrap_or_default()
}

/// 📌️ Re-creates the copied nodes, members and regions under fresh ids, shifted to the placement position unless the anchor keeps the original place.
pub fn fem2d_paste_operations(doc: &ArtifactView<'_, Fem2dSnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Fem2dMutation>, ClipboardError> {
    let parsed: Value = semio_framework_pack_json::parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    let text = |row: &Value, key: &str| row.get(key).and_then(Value::as_str).map(str::to_string).ok_or_else(|| ClipboardError::ParseFailed(format!("clipboard row lacks {key}")));
    let number = |row: &Value, key: &str| row.get(key).and_then(Value::as_f64).ok_or_else(|| ClipboardError::ParseFailed(format!("clipboard row lacks {key}")));
    let rows = |key: &str| parsed.get(key).and_then(Value::as_array).cloned().unwrap_or_default();
    let nodes = rows("nodes").iter().map(|row| Ok((text(row, "id")?, [number(row, "x")?, number(row, "y")?]))).collect::<Result<Vec<_>, ClipboardError>>()?;
    let snapshot = doc.snapshot;
    let operation = doc.operation().map_err(|fault| ClipboardError::ParseFailed(fault.message))?;
    let fresh = |prefix: &str, kind: &str, old: &str| operation.entity_id(prefix, &format!("paste:{kind}:{old}")).map_err(|fault| ClipboardError::ParseFailed(fault.message));
    let region_rows = rows("regions").iter().map(parse_region).collect::<Result<Vec<_>, ClipboardError>>()?;
    let corners = nodes.iter().map(|(_, point)| *point).chain(region_rows.iter().flat_map(|region| region.outline.iter().copied())).collect::<Vec<_>>();
    let shift = match (placement.anchor, placement.position) {
        (PasteAnchor::Original, _) | (_, None) => [0.0; 2],
        (_, Some(_)) if corners.is_empty() => [0.0; 2],
        (_, Some(position)) => {
            let bounds = |axis: usize| corners.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), point| (low.min(point[axis]), high.max(point[axis])));
            [0, 1].map(|axis| { let (low, high) = bounds(axis); position[axis] - (low + high) * 0.5 })
        }
    };
    let mut mutations = Vec::new();
    let mut renamed = std::collections::HashMap::new();
    for (old, point) in &nodes {
        let id = fresh("n", "node", old)?;
        renamed.insert(old.clone(), id.clone());
        mutations.push(Fem2dMutation::CreateNode(CreateNode { node: FemNodeValue { id, x: point[0] + shift[0], y: point[1] + shift[1] }, index: None }));
    }
    let material = snapshot.materials.iter().map(|material| material.id.as_str().to_string()).collect::<Vec<_>>();
    let section = snapshot.sections.iter().map(|section| section.id.clone()).collect::<Vec<_>>();
    let known = |wanted: String, available: &[String]| if available.contains(&wanted) { Some(wanted) } else { available.first().cloned() };
    for row in rows("elements") {
        let (Some(start), Some(end)) = (renamed.get(&text(&row, "start")?), renamed.get(&text(&row, "end")?)) else { continue };
        let (Some(material_id), Some(section_id)) = (known(text(&row, "materialId")?, &material), known(text(&row, "sectionId")?, &section)) else { continue };
        let id = fresh("e", "element", &text(&row, "id")?)?;
        let element = match row.get("kind").and_then(Value::as_str) {
            Some("beam") => FemElement::Beam { id, start: start.clone(), end: end.clone(), material_id, section_id },
            _ => FemElement::Bar { id, start: start.clone(), end: end.clone(), material_id, section_id },
        };
        mutations.push(Fem2dMutation::CreateElement(CreateElement { element: Box::new(element), index: None }));
    }
    for region in region_rows {
        let id = fresh("r", "region", &region.id)?;
        let Some(material_id) = known(region.material_id.clone(), &material) else { continue };
        let moved = |ring: &[[f64; 2]]| ring.iter().map(|[x, y]| [x + shift[0], y + shift[1]]).collect::<Vec<_>>();
        mutations.push(Fem2dMutation::CreateRegion(CreateRegion { region: FemRegion { id, outline: moved(&region.outline), holes: region.holes.iter().map(|ring| moved(ring)).collect(), material_id, ..region }, index: None }));
    }
    Ok(mutations)
}

fn parse_region(row: &Value) -> Result<FemRegion, ClipboardError> {
    let missing = |key: &str| ClipboardError::ParseFailed(format!("clipboard row lacks {key}"));
    let text = |key: &str| row.get(key).and_then(Value::as_str).map(str::to_string).ok_or_else(|| missing(key));
    let number = |key: &str| row.get(key).and_then(Value::as_f64).ok_or_else(|| missing(key));
    let ring = |value: &Value| value.as_array().map(|points| points.iter().map(|point| { let pair = point.as_array()?; Some([pair.first()?.as_f64()?, pair.get(1)?.as_f64()?]) }).collect::<Option<Vec<_>>>()).flatten().ok_or_else(|| missing("outline"));
    Ok(FemRegion {
        id: text("id")?,
        name: text("name")?,
        outline: ring(row.get("outline").ok_or_else(|| missing("outline"))?)?,
        holes: row.get("holes").and_then(Value::as_array).ok_or_else(|| missing("holes"))?.iter().map(ring).collect::<Result<Vec<_>, _>>()?,
        thickness: number("thickness")?,
        material_id: text("materialId")?,
        mesh_size: number("meshSize")?,
    })
}
//#endregion 🔖️Fem2dClipboard

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
