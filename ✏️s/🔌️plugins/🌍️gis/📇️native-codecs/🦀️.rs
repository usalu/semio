//! 🪢 GIS-owned native codec receipts; preview is inert and never registers a codec.

use semio_framework_hash::{Hasher, Sha256};
use semio_framework_plugin::PluginAssemblyError;

#[derive(Clone, Copy)]
enum GisCodecV1 {
    Map,
    Terrain,
}

/// 🪪 Read-only identity projection; the projection cannot construct a factory receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeGisCodecIdentityV1 {
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

/// 🔐 One package-owned closed factory, constructible only by the exact two-codec preview.
pub struct NativeGisCodecReceiptV1 {
    artifact: GisCodecV1,
}

impl NativeGisCodecReceiptV1 {
    /// 🧾 Returns identity data without exposing executable construction authority.
    pub fn identity(&self) -> NativeGisCodecIdentityV1 {
        let (factory_id, artifact_kind, schema, extension, capability, protocol) = match self.artifact {
            GisCodecV1::Map => (
                "gis.gismap.v1", "s.gis.gismap", "gis.map", "gismap", "s.gis.gismap.codec.document",
                include_bytes!("../🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio").as_slice(),
            ),
            GisCodecV1::Terrain => (
                "gis.gisterrain.v1", "s.gis.gisterrain", "gis.terrain", "gisterrain", "s.gis.gisterrain.codec.document",
                include_bytes!("../🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio").as_slice(),
            ),
        };
        NativeGisCodecIdentityV1 { plugin_id: "gis", package_id: "semio:gis", package_version: env!("CARGO_PKG_VERSION"), factory_id, artifact_kind, schema, extension, capability, pack_schema_hash: Sha256::digest(protocol) }
    }

    fn validate(&self) -> Result<(), PluginAssemblyError> {
        let identity = self.identity();
        let declaration = match self.artifact {
            GisCodecV1::Map => crate::artifacts::gismap::declaration(),
            GisCodecV1::Terrain => crate::artifacts::gisterrain::declaration(),
        }.map_err(PluginAssemblyError::definition)?;
        let definition = declaration.definition();
        let mut codecs = definition.codecs();
        let codec = codecs.next().ok_or_else(|| invalid("missing GIS codec capability"))?;
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
            return Err(invalid("GIS codec declaration differs from its private factory"));
        }
        Ok(())
    }

    /// 🧬 Consumes the inert receipt into the exact typed codec after declaration revalidation.
    pub fn into_codec(self) -> Result<store::ArtifactCodec, PluginAssemblyError> {
        self.validate()?;
        let identity = self.identity();
        let (mut codec, extension) = match self.artifact {
            GisCodecV1::Map => (
                store::ArtifactCodec::of::<crate::artifacts::gismap::GisMapSnapshot, crate::artifacts::gismap::GisMapMutation>(identity.schema),
                <crate::artifacts::gismap::GisMapSnapshot as store::ArtifactDsl>::EXTENSION,
            ),
            GisCodecV1::Terrain => (
                store::ArtifactCodec::of::<crate::artifacts::gisterrain::GisTerrainSnapshot, crate::artifacts::gisterrain::GisTerrainMutation>(identity.schema),
                <crate::artifacts::gisterrain::GisTerrainSnapshot as store::ArtifactDsl>::EXTENSION,
            ),
        };
        if codec.schema != identity.schema || extension != identity.extension {
            return Err(invalid("GIS typed codec differs from its private receipt"));
        }
        codec.extension = extension;
        codec.pack_schema_hash = identity.pack_schema_hash;
        Ok(codec)
    }
}

/// 🧷 Previews the complete fixed GIS closure without process-global registration.
pub fn native_codec_factory_receipts() -> Result<[NativeGisCodecReceiptV1; 2], PluginAssemblyError> {
    let receipts = [NativeGisCodecReceiptV1 { artifact: GisCodecV1::Map }, NativeGisCodecReceiptV1 { artifact: GisCodecV1::Terrain }];
    for receipt in &receipts {
        receipt.validate()?;
    }
    Ok(receipts)
}

fn invalid(message: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("gis.native-codec-receipt", message)
}
