//! 📚️ Native JSON admission of schema-first geometry catalogue definitions.
use crate::standards::v1::subsets::any::schema::catalogue::*;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use std::sync::{Arc, OnceLock};

/// 📚️ Every bundled category file with its slug, in the order of the folder listing.
pub const CATEGORY_SOURCES: [(&str, &str); 27] = [
    ("brep-primitive", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-primitive.json")),
    ("brep-curve", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-curve.json")),
    ("brep-surface", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-surface.json")),
    ("brep-solid", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-solid.json")),
    ("brep-boolean", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-boolean.json")),
    ("brep-feature", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-feature.json")),
    ("brep-transform", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-transform.json")),
    ("brep-intersect", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-intersect.json")),
    ("brep-evaluate", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-evaluate.json")),
    ("brep-topology", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-topology.json")),
    ("brep-interchange", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️brep-interchange.json")),
    ("mesh-primitive", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️mesh-primitive.json")),
    ("mesh-convert", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️mesh-convert.json")),
    ("mesh-transform", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️mesh-transform.json")),
    ("mesh-component", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️mesh-component.json")),
    ("mesh-edit", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️mesh-edit.json")),
    ("mesh-repair", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️mesh-repair.json")),
    ("mesh-inspect", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️mesh-inspect.json")),
    ("mesh-interchange", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️mesh-interchange.json")),
    ("mesh-shading", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️mesh-shading.json")),
    ("mesh-uv", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️mesh-uv.json")),
    ("analysis-measure", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️analysis-measure.json")),
    ("analysis-check", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️analysis-check.json")),
    ("math-values", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️math-values.json")),
    ("math-arithmetic", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️math-arithmetic.json")),
    ("math-vector", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️math-vector.json")),
    ("math-list", include_str!("../../../../🧬️schema/🗂️catalogue/🔣️math-list.json")),
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
