use semio_s_artifact_stdio_ifc_declarations::{embedded_catalog,IfcCatalog};
use std::sync::OnceLock;
static C:OnceLock<IfcCatalog>=OnceLock::new();
pub fn catalog()->&'static IfcCatalog{C.get_or_init(||embedded_catalog(include_bytes!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/catalog.ifcpack")))}
