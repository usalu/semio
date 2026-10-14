//! 📚️ Native JSON admission of schema-first geometry catalogue definitions.
use semio_s_artifact_procedural_3d_model::catalogue::*;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use std::sync::{Arc, OnceLock};

/// 📚️ Every bundled category file with its slug, in the order of the folder listing.
pub const CATEGORY_SOURCES: [(&str, &str); 27] = [
    semio_s_artifact_procedural_3d_brep_primitive::CATEGORY,
    semio_s_artifact_procedural_3d_brep_curve::CATEGORY,
    semio_s_artifact_procedural_3d_brep_surface::CATEGORY,
    semio_s_artifact_procedural_3d_brep_solid::CATEGORY,
    semio_s_artifact_procedural_3d_brep_boolean::CATEGORY,
    semio_s_artifact_procedural_3d_brep_feature::CATEGORY,
    semio_s_artifact_procedural_3d_brep_transform::CATEGORY,
    semio_s_artifact_procedural_3d_brep_intersect::CATEGORY,
    semio_s_artifact_procedural_3d_brep_evaluate::CATEGORY,
    semio_s_artifact_procedural_3d_brep_topology::CATEGORY,
    semio_s_artifact_procedural_3d_io_geometry::brep_interchange::CATEGORY,
    semio_s_artifact_procedural_3d_mesh_primitive::CATEGORY,
    semio_s_artifact_procedural_3d_mesh_convert::CATEGORY,
    semio_s_artifact_procedural_3d_mesh_transform::CATEGORY,
    semio_s_artifact_procedural_3d_mesh_component::CATEGORY,
    semio_s_artifact_procedural_3d_mesh_edit::CATEGORY,
    semio_s_artifact_procedural_3d_mesh_repair::CATEGORY,
    semio_s_artifact_procedural_3d_mesh_inspect::CATEGORY,
    semio_s_artifact_procedural_3d_io_geometry::mesh_interchange::CATEGORY,
    semio_s_artifact_procedural_3d_mesh_shading::CATEGORY,
    semio_s_artifact_procedural_3d_mesh_uv::CATEGORY,
    semio_s_artifact_procedural_3d_analysis_measure::CATEGORY,
    semio_s_artifact_procedural_3d_analysis_check::CATEGORY,
    semio_s_artifact_procedural_3d_math_values::CATEGORY,
    semio_s_artifact_procedural_3d_math_arithmetic::CATEGORY,
    semio_s_artifact_procedural_3d_math_vector::CATEGORY,
    semio_s_artifact_procedural_3d_math_list::CATEGORY,
];

/// 📥️ Admits native category texts before pure catalogue assembly.
pub fn parse_catalogue<'a>(sources: impl IntoIterator<Item = (&'a str, &'a str)>) -> Result<Catalogue, CatalogueError> {
    let categories = sources.into_iter().map(|(slug, text)| from_json_str(text, JsonMemberPolicy::Reject).map(|file| (slug.to_string(), file)).map_err(|error| CatalogueError { source: slug.to_string(), message: error.to_string() })).collect::<Result<Vec<_>, _>>()?;
    Catalogue::from_categories(categories)
}

/// 📦️ Admits the bundled JSON definitions into an independently owned catalogue.
pub fn bundled_catalogue() -> Result<Catalogue, CatalogueError> { parse_catalogue(CATEGORY_SOURCES) }

/// 🧷️ Shares the admitted immutable bundle with explicit inference contexts.
pub fn catalogue() -> &'static Arc<Catalogue> {
    static BUNDLED: OnceLock<Arc<Catalogue>> = OnceLock::new();
    BUNDLED.get_or_init(|| Arc::new(bundled_catalogue().expect("the bundled geometry catalogue is validated by its tests")))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
