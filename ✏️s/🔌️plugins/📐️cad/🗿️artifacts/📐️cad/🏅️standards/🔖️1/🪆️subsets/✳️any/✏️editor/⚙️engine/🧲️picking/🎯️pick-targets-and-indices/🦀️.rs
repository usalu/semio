//! 🎯️ Pick target construction of the CAD pick engine: the object and typology indices and the pick target creation, filtering and events. See [`super`].

use super::*;

//#region 🔖️Indices
pub(super) fn object_primitive_entries(object: &CadObject) -> Vec<(ModelEntityKind, &str)> {
    object.primitives.iter().filter_map(|slot| ModelEntityKind::parse(&slot.kind).map(|kind| (kind, slot.primitive_id.as_str()))).collect()
}

fn object_primary_primitive_ref(object: &CadObject) -> Option<&str> {
    object.primitives.first().map(|slot| slot.primitive_id.as_str())
}

/// 🧭️ Object rows whose typology is declared under a model definition.
pub fn list_model_objects_for_model_definition<'a>(objects: &'a [CadObject], model_definition_id: &str) -> Vec<&'a CadObject> {
    let typologies = model_definition_typology_ids(Some(model_definition_id));
    objects.iter().filter(|object| typologies.contains(&object.typology)).collect()
}

/// 🏷️ Maps kernel member keys (`face:f0`, `solid:s0`, …) to the typology of the owning object row.
pub fn build_geometry_typology_index(objects: &[CadObject], geometry: &CadGeometry, model_definition_id: &str) -> BTreeMap<String, String> {
    let buckets = geometry_buckets(geometry);
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for object in list_model_objects_for_model_definition(objects, model_definition_id) {
        for (kind, primitive_ref) in object_primitive_entries(object) {
            if kind == ModelEntityKind::Solid {
                for key in buckets.solid_primitive_member_ids(primitive_ref) {
                    out.insert(key, object.typology.clone());
                }
                continue;
            }
            out.insert(format!("{}:{primitive_ref}", kind.as_str()), object.typology.clone());
        }
    }
    for anchor in &buckets.anchors {
        let Some((kind, id)) = anchor.attachment.as_ref() else { continue };
        if let Some(mapped) = out.get(&format!("{kind}:{id}")).cloned() {
            out.insert(format!("anchor:{}", anchor.id), mapped);
        }
    }
    out
}

/// 🧭️ Maps kernel member keys to the owning object id, for hover-reveal gating.
pub fn build_geometry_object_index(objects: &[CadObject], geometry: &CadGeometry, model_definition_id: &str) -> BTreeMap<String, String> {
    let buckets = geometry_buckets(geometry);
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for object in list_model_objects_for_model_definition(objects, model_definition_id) {
        out.insert(format!("object:{}", object.id), object.id.clone());
        for (kind, primitive_ref) in object_primitive_entries(object) {
            out.insert(format!("{}:{primitive_ref}", kind.as_str()), object.id.clone());
            if kind == ModelEntityKind::Solid {
                out.insert(format!("object:{primitive_ref}"), object.id.clone());
                for key in buckets.solid_primitive_member_ids(primitive_ref) {
                    out.insert(key, object.id.clone());
                }
            }
        }
    }
    for solid in &geometry.solids {
        let key = format!("solid:{}", solid.id);
        if out.contains_key(&key) {
            continue;
        }
        out.insert(key, solid.id.clone());
        out.insert(format!("object:{}", solid.id), solid.id.clone());
        for member in buckets.solid_primitive_member_ids(&solid.id) {
            out.entry(member).or_insert_with(|| solid.id.clone());
        }
    }
    for anchor in &buckets.anchors {
        let Some((kind, id)) = anchor.attachment.as_ref() else { continue };
        if let Some(mapped) = out.get(&format!("{kind}:{id}")).cloned() {
            out.insert(format!("anchor:{}", anchor.id), mapped);
        }
    }
    out
}
//#endregion 🔖️Indices

//#region 🔖️Targets
fn object_pick_points(object: &CadObject, buckets: &GeometryBuckets<'_>) -> Vec<[f64; 3]> {
    let Some(primitive_ref) = object_primary_primitive_ref(object) else { return Vec::new() };
    buckets.solids.get(primitive_ref).map(|solid| buckets.solid_points(solid)).unwrap_or_default()
}

fn model_object_pick_targets(objects: &[CadObject], buckets: &GeometryBuckets<'_>, model_definition_id: &str) -> Vec<SpatialPickTarget> {
    let mut out = Vec::new();
    for object in list_model_objects_for_model_definition(objects, model_definition_id) {
        let points = object_pick_points(object, buckets);
        let Some(point) = point_centroid(&points) else { continue };
        out.push(SpatialPickTarget { kind: SpatialPickTargetKind::Object, id: object.id.clone(), point, points, geometry_kind: None, typology_id: Some(object.typology.clone()) });
    }
    out
}

fn append_primitive_pick_targets(out: &mut Vec<SpatialPickTarget>, buckets: &GeometryBuckets<'_>, entity_kinds: &[ModelEntityKind], typology_index: &BTreeMap<String, String>, skip_solid_ids: &BTreeSet<String>) {
    let with_typology = |kind: SpatialPickTargetKind, geometry_kind: ModelEntityKind, id: &str, point: [f64; 3], points: Vec<[f64; 3]>| SpatialPickTarget {
        kind,
        id: id.to_string(),
        point,
        points,
        geometry_kind: Some(geometry_kind),
        typology_id: typology_index.get(&format!("{}:{id}", geometry_kind.as_str())).cloned(),
    };
    if entity_kinds.contains(&ModelEntityKind::Anchor) {
        for anchor in &buckets.anchors {
            out.push(with_typology(SpatialPickTargetKind::Vertex, ModelEntityKind::Anchor, &anchor.id, anchor.position, Vec::new()));
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Vertex) {
        for vertex in &buckets.order.vertices {
            out.push(with_typology(SpatialPickTargetKind::Vertex, ModelEntityKind::Vertex, &vertex.id, vertex.position, Vec::new()));
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Edge) {
        for edge in &buckets.order.edges {
            let points = buckets.edge_points(edge);
            if let Some(point) = point_centroid(&points) {
                out.push(with_typology(SpatialPickTargetKind::Edge, ModelEntityKind::Edge, &edge.id, point, points));
            }
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Wire) {
        for wire in &buckets.order.wires {
            let points = buckets.wire_points(wire);
            if let Some(point) = point_centroid(&points) {
                out.push(with_typology(SpatialPickTargetKind::Edge, ModelEntityKind::Wire, &wire.id, point, points));
            }
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Face) {
        for face in &buckets.order.faces {
            let points = buckets.face_points(face);
            if let Some(point) = point_centroid(&points) {
                out.push(with_typology(SpatialPickTargetKind::Face, ModelEntityKind::Face, &face.id, point, points));
            }
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Shell) {
        for shell in &buckets.order.shells {
            let segments = buckets.entity_wire_segments(ModelEntityKind::Shell, &shell.id);
            let points: Vec<[f64; 3]> = segments.into_iter().flat_map(|(a, b)| [a, b]).collect();
            if let Some(point) = point_centroid(&points) {
                out.push(with_typology(SpatialPickTargetKind::Face, ModelEntityKind::Shell, &shell.id, point, points));
            }
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Solid) {
        let all = buckets.all_vertex_points();
        let all_center = point_centroid(&all);
        for solid in &buckets.order.solids {
            if skip_solid_ids.contains(&solid.id) {
                continue;
            }
            let points = buckets.solid_points(solid);
            let Some(point) = point_centroid(&points).or(all_center) else { continue };
            let points = if points.is_empty() { all.clone() } else { points };
            out.push(with_typology(SpatialPickTargetKind::Object, ModelEntityKind::Solid, &solid.id, point, points));
        }
    }
}

/// 🧲️ Builds every snap/select target for a pane from its kernel geometry and typology object rows.
pub fn create_spatial_pick_targets(objects: &[CadObject], geometry: Option<&CadGeometry>, active_model_definition_id: Option<&str>) -> Vec<SpatialPickTarget> {
    let Some(geometry) = geometry else { return Vec::new() };
    let fallback: &str = default_model_definition_id();
    let md_id = active_model_definition_id.unwrap_or(fallback);
    let buckets = geometry_buckets(geometry);
    let entity_kinds = model_definition_selection_entity_kinds(md_id);
    let scoped_objects = if is_shape_model_definition(Some(md_id)) { Vec::new() } else { list_model_objects_for_model_definition(objects, md_id) };
    let typology_index = build_geometry_typology_index(objects, geometry, md_id);
    let mut targets: Vec<SpatialPickTarget> = Vec::new();
    if entity_kinds.contains(&ModelEntityKind::Object) && !scoped_objects.is_empty() {
        targets.extend(model_object_pick_targets(objects, &buckets, md_id));
    }
    if model_definition_uses_geometry_picking(md_id) {
        let skip_solid_ids: BTreeSet<String> = scoped_objects.iter().filter_map(|object| object_primary_primitive_ref(object)).map(str::to_string).collect();
        append_primitive_pick_targets(&mut targets, &buckets, &entity_kinds, &typology_index, &skip_solid_ids);
    }
    targets
}

/// 🎯️ Keeps targets admitted by an interaction's `selection.accept` list and the enabled pick kinds.
pub fn filter_spatial_pick_targets(targets: Vec<SpatialPickTarget>, accept: &[ModelEntityKind], toggles: SpatialPickKindToggles) -> Vec<SpatialPickTarget> {
    let accept_kinds = spatial_pick_kinds_for_selection_accept(accept);
    targets
        .into_iter()
        .filter(|target| {
            if !toggles.enabled(target.kind) {
                return false;
            }
            let Some(accept_kinds) = accept_kinds.as_ref() else { return true };
            if accept_kinds.contains(&target.kind) {
                return true;
            }
            pick_target_primitive_kind(target).is_some_and(|primitive| accept.contains(&primitive))
        })
        .collect()
}

/// 🧲️ The statechart event payload a canvas pick dispatches (snap point + selection metadata).
pub fn create_spatial_pick_event(kind: SpatialPickKind, point: [f64; 3], target: Option<&SpatialPickTarget>, modifiers: &[(&str, bool)]) -> DslValue {
    let vec3 = |p: [f64; 3]| DslValue::Array(p.iter().map(|v| DslValue::float(*v)).collect());
    let mut entries: Vec<(String, DslValue)> = vec![
        ("kind".to_string(), DslValue::String(kind.as_str().to_string())),
        ("point".to_string(), vec3(point)),
        ("modifiers".to_string(), DslValue::object(modifiers.iter().map(|(name, value)| ((*name).to_string(), DslValue::Bool(*value))))),
    ];
    if let Some(target) = target {
        let geometry_kind = if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() { ModelEntityKind::Object } else { kernel_geometry_kind_for_object_pick(target.kind, target.geometry_kind) };
        entries.push(("snap".to_string(), DslValue::object([("kind".to_string(), DslValue::String(geometry_kind.as_str().to_string())), ("id".to_string(), DslValue::String(target.id.clone())), ("point".to_string(), vec3(target.point))])));
        entries.push(("selection".to_string(), DslValue::object([("kind".to_string(), DslValue::String(geometry_kind.as_str().to_string())), ("id".to_string(), DslValue::String(target.id.clone()))])));
    }
    DslValue::object(entries)
}
//#endregion 🔖️Targets
