//! 🧾️ Schema-owned definition assembler for norm artifact declarations.

use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactDefinitionError, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

//#region 🧾️Capabilities
/// 🧷️ One immutable external identity claim owned by a leaf definition row.
pub struct ClaimSpec {
    pub namespace: &'static str,
    pub value: &'static str,
}

/// 🗺️ One immutable localized descriptor row owned by a leaf definition row.
pub struct LocalizationSpec {
    pub locale: &'static str,
    pub text: &'static str,
}

/// 🧾️ One immutable capability row owned by an artifact leaf.
pub struct CapabilitySpec {
    pub identity: &'static str,
    pub kind: &'static str,
    pub descriptor: &'static str,
    pub claims: &'static [ClaimSpec],
    pub localizations: &'static [LocalizationSpec],
}

/// 🧩️ Validates and assembles one leaf's already-declared capability rows.
pub async fn assemble_definition(identity: &'static str, capabilities: &'static [CapabilitySpec]) -> Result<ArtifactDefinition, ArtifactDefinitionError> {
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse(identity)?);
    for row in capabilities {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(row.identity)?, ArtifactCapabilityKind::parse(row.kind)?).descriptor(row.descriptor.as_bytes())?;
        for claim in row.claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(claim.namespace)?, claim.value)?)?;
        }
        for localization in row.localizations {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(localization.locale)?, localization.text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}
//#endregion 🧾️Capabilities
