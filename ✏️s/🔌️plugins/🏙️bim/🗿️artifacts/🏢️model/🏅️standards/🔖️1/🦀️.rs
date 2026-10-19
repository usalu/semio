//! 🏅️ BIM standard `1` root: mounts subset `any` and exports `standard() -> StandardDeclaration`.
//! The media identity is a synthesis: the artifact's capability rows only ever claimed the codec extension `bim`, never a MIME type.

use crate::standards::v1::subsets;
use semio_framework_artifact_reference::StandardId;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};

//#region 🔖️Standard
/// 🏅️ The complete declaration of standard `1`.
pub fn standard<A: crate::BimApplication>() -> StandardDeclaration<A> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.bim+json"], extensions: &["bim"] }, subsets: vec![subsets::any::subset()] }
}
//#endregion 🔖️Standard

//#region 🧩️ArtifactItems
pub mod artifact_items {
//#region 🔖️Declaration
/// 🔖️ This artifact's capability definition: standard, schema, inference, document codec and localization rows.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let codec_extension = format!("{}:{}:bim", semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::BIM_MODEL_DOCUMENT_SCHEMA.chars().count(), semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::BIM_MODEL_DOCUMENT_SCHEMA);
    let rows: Vec<(&str, &str, &str, Vec<(&str, &str)>, Option<(&str, &str)>)> = vec![
        ("s.bim.model.standard.v1", "standard", "1", vec![], None),
        ("s.bim.model.standard.v1.profile.any", "profile", "any", vec![], None),
        ("s.bim.model.schema.artifact", "schema", "s.bim.model", vec![("schema", "s.bim.model")], None),
        ("s.bim.model.inference.artifact", "inference", "s.bim.model.inference", vec![("schema", "s.bim.model.inference")], None),
        ("s.bim.model.codec.document-1", "codec", "bim.model:bim", vec![("codec", semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::BIM_MODEL_DOCUMENT_SCHEMA), ("codec-extension", codec_extension.as_str())], None),
        ("s.bim.model.localization.en", "localization", "BIM Model", vec![], Some(("en", "BIM Model"))),
        ("s.bim.model.localization.de", "localization", "BIM-Modell", vec![], Some(("de", "BIM-Modell"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.bim.model")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(identity)?, ArtifactCapabilityKind::parse(kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(namespace)?, value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(locale)?, text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

/// 🌳️ This artifact's declaration tree root, consumed by the hub composition through `declare_artifact`: schema, io (native codec and the IFC, glTF and SVG hops), viewer, editor and examples of every subset.
pub fn artifact<A: BimApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    ArtifactDeclaration { kind: semio_framework_artifact_reference::ArtifactKindId::parse(semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::BIM_MODEL_DIALECT.artifact_kind).expect("canonical BIM kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait BimApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<semio_s_artifact_bim_model_bim::editor::bim::BimModelApp>>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<semio_s_artifact_bim_model_viewer::viewer::bim::BimModelViewer>>>
{
}

impl<A> BimApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<semio_s_artifact_bim_model_bim::editor::bim::BimModelApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<semio_s_artifact_bim_model_viewer::viewer::bim::BimModelViewer>>>
{
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems
