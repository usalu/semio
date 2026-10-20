//! 🪪️ Declaration of the GIS map artifact: the artifact kind, the immutable runtime definition and the typed declaration. See [`super`].

use crate::schema::snapshot::GisMapSnapshot;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use crate::{GISMAP_DIALECT, GIS_MAP_SCHEMA};
use crate::standards::v1::subsets::any::inference_service::{gis_map_inference_service};
use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::schema::mutations::GisMapMutation;

//#region 🔹ArtifactKind
/// 🗺️ The canonical GIS map artifact-kind declaration; payload schemas remain independent.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: GISMAP_DIALECT.artifact_kind.into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("2D Map", "2D-Karte"),
        source_format: GIS_MAP_SCHEMA.into(),
        component_kind: "gismap".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        schema: GIS_MAP_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: crate::standards::v1::subsets::any::io::export_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),
        import_stdio_kinds: crate::standards::v1::subsets::any::io::import_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),
    }
}
//#endregion 🔹ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// the plugin root's `register_gis_exports` fan-out. Relocated from `⚙️engine` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g2): `declaration()` describes the artifact
/// (kind, schema, io ports, ownership), which is not engine behaviour.
/// 🧾️ Defines s.gis.gismap's immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    ArtifactDefinition::new(ArtifactIdentity::parse("s.gis.gismap")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.gis.gismap")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.gis.gismap")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.gis.gismap.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.gis.gismap.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.gis.gismap@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.gis.gismap@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.svg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.svg@1.1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.svg@1.1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.pdf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.pdf@1.4/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.pdf@1.4/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.png")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.png@1.2/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.png@1.2/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.geojson")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/geojson")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/geojson")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.dwg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dwg@ac1018/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.dxf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dxf@r12/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dxf@r12/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"gis.map:gismap")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "gis.map")?)?
                .claim(ArtifactIdentityClaim::codec_extension("gis.map", "gismap")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"GIS Map")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "GIS Map")?)?)?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"GIS Karte")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "GIS Karte")?)?,
        )
}

/// 🔖️ Assembles s.gis.gismap's typed runtime declaration.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::schema::gismap_artifact_schema_descriptor())
        .inferences([crate::standards::v1::subsets::any::schema::inferences::gismap_artifact_inference_descriptor()])
        .inference_services([gis_map_inference_service()])
        .composers(crate::standards::v1::subsets::any::io::io_registry::entries())
        .document_codec_bare::<GisMapSnapshot, GisMapMutation>(GIS_MAP_SCHEMA, GISMAP_DIALECT)
        .try_build()
}
//#endregion 🔖️Register
