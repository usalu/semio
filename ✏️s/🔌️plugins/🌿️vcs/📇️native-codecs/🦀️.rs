//! 🌿 VCS-owned native codec receipts; preview is inert and never registers a codec.

use semio_framework_hash::{Hasher, Sha256};
use semio_framework_plugin::PluginAssemblyError;

#[derive(Clone, Copy)]
enum VcsCodecV1 {
    Vcs,
}

/// 🧾️ Read-only identity projection; the projection cannot construct a factory receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeVcsCodecIdentityV1 {
    pub plugin_id: &'static str,
    pub package_id: &'static str,
    pub package_version: &'static str,
    pub factory_id: &'static str,
    pub artifact_kind: &'static str,
    pub schema: &'static str,
    pub extension: &'static str,
    pub capability: &'static str,
    pub pack_schema_hash: [u8; 32],
}

/// 🔒️ One package-owned closed factory, constructible only by the exact one-codec preview.
pub struct NativeVcsCodecReceiptV1 {
    artifact: VcsCodecV1,
}

impl NativeVcsCodecReceiptV1 {
    /// 🪧️ Returns identity data without exposing executable construction authority.
    pub fn identity(&self) -> NativeVcsCodecIdentityV1 {
        let (factory_id, artifact_kind, schema, extension, capability, protocol) = match self.artifact {
            VcsCodecV1::Vcs => (
                "vcs.vcs.v1", "s.vcs.vcs", "vcs.vcs", "vcs", "s.vcs.vcs.codec.document",
                include_bytes!("../🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/📡️.protocol.semio").as_slice(),
            ),
        };
        NativeVcsCodecIdentityV1 { plugin_id: "vcs", package_id: "semio:vcs", package_version: env!("CARGO_PKG_VERSION"), factory_id, artifact_kind, schema, extension, capability, pack_schema_hash: Sha256::digest(protocol) }
    }

    fn validate(&self) -> Result<(), PluginAssemblyError> {
        let identity = self.identity();
        let definition = match self.artifact {
            VcsCodecV1::Vcs => crate::artifacts::vcs::definition(),
        }
        .map_err(PluginAssemblyError::definition)?;
        let mut codecs = definition.codecs();
        let codec = codecs.next().ok_or_else(|| invalid("missing VCS codec capability"))?;
        let descriptor = format!("{}:{}", identity.schema, identity.extension);
        let extension_claim = format!("{}:{}:{}", identity.schema.len(), identity.schema, identity.extension);
        if definition.identity().as_str() != identity.artifact_kind
            || codec.identity().as_str() != identity.capability
            || codec.descriptor_bytes() != descriptor.as_bytes()
            || codecs.next().is_some()
            || identity.pack_schema_hash == [0; 32]
            || codec.claims().len() != 2
            || !codec.claims().iter().any(|claim| claim.namespace().as_str() == "codec" && claim.value() == identity.schema)
            || !codec.claims().iter().any(|claim| claim.namespace().as_str() == "codec-extension" && claim.value() == extension_claim)
        {
            return Err(invalid("VCS codec declaration differs from its private factory"));
        }
        Ok(())
    }

    /// 🧬 Consumes the inert receipt into the exact typed codec after declaration revalidation.
    pub fn into_codec(self) -> Result<store::ArtifactCodec, PluginAssemblyError> {
        self.validate()?;
        let identity = self.identity();
        let (mut codec, extension) = match self.artifact {
            VcsCodecV1::Vcs => (
                store::ArtifactCodec::of::<crate::artifacts::vcs::VcsSnapshot, crate::artifacts::vcs::VcsDemoMutation>(identity.schema),
                <crate::artifacts::vcs::VcsSnapshot as store::ArtifactDsl>::EXTENSION,
            ),
        };
        if codec.schema != identity.schema || extension != identity.extension {
            return Err(invalid("VCS typed codec differs from its private receipt"));
        }
        codec.extension = extension;
        codec.pack_schema_hash = identity.pack_schema_hash;
        Ok(codec)
    }
}

/// 🪤 Previews the complete fixed VCS closure without process-global registration.
pub fn native_codec_factory_receipts() -> Result<[NativeVcsCodecReceiptV1; 1], PluginAssemblyError> {
    let receipts = [NativeVcsCodecReceiptV1 { artifact: VcsCodecV1::Vcs }];
    for receipt in &receipts {
        receipt.validate()?;
    }
    Ok(receipts)
}

fn invalid(message: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("vcs.native-codec-receipt", message)
}
