//! 🧊️ Objects: every geometry object stands in the scene as its product body (the primitives of its shape tree), its space envelopes as translucent boxes, its ports as small markers and its semantic surfaces as thin sheets; coloured by the utilisation of the checks that name the object.

use super::kit::*;
use super::layout::{leaf_extent, leaves, Layout, Pose};
use super::model::part_2::{BoundingBox, GeometryObject, PortDefinition, SpaceKind};
use super::scene::*;
use super::SceneInput;

const PORT_HALF: f64 = 0.01;
const SHEET: f64 = 0.002;

fn space_words(kind: SpaceKind) -> (&'static str, &'static str) {
    match kind {
        SpaceKind::Overall => ("overall space", "Gesamtraum"),
        SpaceKind::Operation => ("operating space", "Bedienraum"),
        SpaceKind::Access => ("access space", "Zugangsraum"),
        SpaceKind::PlacementTransportation => ("placement and transport space", "Einbring- und Transportraum"),
        SpaceKind::Installation => ("installation space", "Montageraum"),
    }
}

fn corners(bounds: &BoundingBox, pose: &Pose) -> Vec<Vec3> {
    vec![pose.at(bounds.min[0], bounds.min[1], bounds.min[2]), pose.at(bounds.max[0], bounds.max[1], bounds.max[2])]
}

fn body(input: &SceneInput, id: &str, object: &GeometryObject, pose: &Pose) -> Option<(Spec, Vec<Vec3>)> {
    let shape = object.shape.as_ref().or(object.symbolic.as_ref())?;
    let mut found = Vec::new();
    leaves(input.document, shape, [0.0; 3], 0, &mut found);
    let mut mesh = MeshBuilder::default();
    for leaf in &found {
        let p = |name: &str| leaf.parameters.get(name).copied().unwrap_or(0.0).max(0.0);
        let (lo, hi) = leaf_extent(leaf);
        match leaf.kind.as_str() {
            "cylinder" => mesh.cylinder(pose, [leaf.offset[0], leaf.offset[1]], p("radius"), (lo[2], hi[2])),
            "sphere" => mesh.sphere(pose, leaf.offset, p("radius")),
            _ => mesh.cuboid(pose, lo, hi),
        }
    }
    if mesh.is_empty() {
        return None;
    }
    let points = mesh.points().to_vec();
    let mut spec = Spec::new(EntityKind::Solid, id, format!("geometry.objects.{id}"), mesh.solid(), MaterialRole::Neutral, LAYER_BODY, loc(&format!("Geometry object {id}"), &format!("Geometrieobjekt {id}")));
    spec.binding = binding(input.report, &[id]);
    spec.detail = loc(&format!("{} primitives, {} spaces, {} ports", found.len(), object.spaces.len(), object.ports.len()), &format!("{} Grundkörper, {} Räume, {} Anschlüsse", found.len(), object.spaces.len(), object.ports.len()));
    Some((spec, points))
}

fn port(id: &str, port: &PortDefinition, pose: &Pose) -> (Spec, Vec<Vec3>) {
    let [x, y, z] = port.position;
    let mut mesh = MeshBuilder::default();
    mesh.cuboid(pose, [x - PORT_HALF, y - PORT_HALF, z - PORT_HALF], [x + PORT_HALF, y + PORT_HALF, z + PORT_HALF]);
    let points = mesh.points().to_vec();
    let name = loc(&format!("Port {} ({})", port.id, port.medium), &format!("Anschluss {} ({})", port.id, port.medium));
    (Spec::new(EntityKind::Fastener, format!("{id}:{PORT_TAG}{}", port.id), format!("geometry.objects.{id}.ports[id={}]", port.id), mesh.solid(), MaterialRole::Steel, LAYER_PORTS, name), points)
}

/// 🎬️ Every geometry object of the document as scene entities.
pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for (id, object) in &input.document.geometry.objects {
        let Some(pose) = layout.pose(id) else { continue };
        if let Some((spec, points)) = body(input, id, object, pose) {
            out.push(spec, &points);
        }
        for space in &object.spaces {
            let mut mesh = MeshBuilder::default();
            mesh.cuboid(pose, space.bounds.min, space.bounds.max);
            let (en, de) = space_words(space.kind);
            let mut spec = Spec::new(EntityKind::Solid, format!("{id}:{SPACE_TAG}{}", space.id), format!("geometry.objects.{id}.spaces[id={}]", space.id), mesh.solid(), MaterialRole::Support, LAYER_SPACES, loc(&format!("{} {} of {id}", space.id, en), &format!("{} {} von {id}", space.id, de)));
            spec.opacity = 0.25;
            spec.binding = binding(input.report, &[id.as_str()]);
            out.push(spec, &corners(&space.bounds, pose));
        }
        for p in &object.ports {
            let (spec, points) = port(id, p, pose);
            out.push(spec, &points);
        }
        for surface in &object.surfaces {
            let mut mesh = MeshBuilder::default();
            let b = &surface.bounds;
            mesh.cuboid(pose, [b.min[0], b.min[1], b.min[2]], [b.max[0].max(b.min[0] + SHEET), b.max[1].max(b.min[1] + SHEET), b.max[2].max(b.min[2] + SHEET)]);
            let mut spec = Spec::new(EntityKind::Solid, format!("{id}:{SURFACE_TAG}{}", surface.id), format!("geometry.objects.{id}.surfaces[id={}]", surface.id), mesh.solid(), MaterialRole::Glass, LAYER_SURFACES, loc(&format!("Surface {} ({})", surface.id, surface.purpose), &format!("Fläche {} ({})", surface.id, surface.purpose)));
            spec.opacity = 0.6;
            spec.binding = binding(input.report, &[id.as_str()]);
            out.push(spec, &corners(&surface.bounds, pose));
        }
    }
}
