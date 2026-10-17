//! 🎯️ Mesh-edit element picks: the selection ports of the `mesh.edit` computes, validated against the input mesh (see [`picked`]).

use semio_framework_3d::mesh::{EdgeId, FaceId, VertexId};
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_geometry_value::WidgetFault;
use semio_s_artifact_procedural_3d_mesh_support::{picked, Element};

pub(super) fn edges(inputs: &WidgetInputs, mesh: &semio_framework_3d::mesh::HalfedgeMesh, port: &str) -> Result<Vec<EdgeId>, WidgetFault> {
    Ok(picked(inputs, port, mesh, Element::Edge)?.into_iter().map(EdgeId).collect())
}

pub(super) fn vertices(inputs: &WidgetInputs, mesh: &semio_framework_3d::mesh::HalfedgeMesh, port: &str) -> Result<Vec<VertexId>, WidgetFault> {
    Ok(picked(inputs, port, mesh, Element::Vertex)?.into_iter().map(VertexId).collect())
}

pub(super) fn faces(inputs: &WidgetInputs, mesh: &semio_framework_3d::mesh::HalfedgeMesh, port: &str) -> Result<Vec<FaceId>, WidgetFault> {
    Ok(picked(inputs, port, mesh, Element::Face)?.into_iter().map(FaceId).collect())
}
