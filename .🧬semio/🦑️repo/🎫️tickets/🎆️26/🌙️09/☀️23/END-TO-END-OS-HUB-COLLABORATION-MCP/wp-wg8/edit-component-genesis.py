import pathlib

root = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules")


def edit(rel, old, new):
    path = root / rel
    text = path.read_text()
    assert text.count(old) == 1, (rel, text.count(old), old[:100])
    path.write_text(text.replace(old, new))


edit(
    "🏪️store/🦀️.rs",
    """    /// 📥️ `codec.print-mirror`: the pair's text mirror, which is also its validation fence.
    fn print_mirror<'a>(&'a self, pack: &'a [u8], spr: &'a [u8]) -> ComponentDocumentCodecFuture<'a, ArtifactTextFiles>;
}
""",
    """    /// 📥️ `codec.print-mirror`: the pair's text mirror, which is also its validation fence.
    fn print_mirror<'a>(&'a self, pack: &'a [u8], spr: &'a [u8]) -> ComponentDocumentCodecFuture<'a, ArtifactTextFiles>;
    /// 🌱️ `codec.genesis`: the zero-history document of `document_id` the component mints.
    fn genesis<'a>(&'a self, document_id: &'a str) -> ComponentDocumentCodecFuture<'a, ComponentDocumentGenesis>;
}

/// 🌱️ One document's genesis pair as its owning component minted it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentDocumentGenesis {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
}
""",
)

edit(
    "🏪️store/🦀️.rs",
    """/// @emoji 📜️ Reads the document schema id from an encoded `.spr` history log.
pub async fn lane_schema_from_spr(""",
    """/// 🌱️ The genesis of `document_id` as the mounted component that owns `schema` mints it, refused
/// unless it is a zero-history document of exactly that identity; `None` when no mounted component
/// owns the kind. Creation authority is the component's, always — the hub's trusted catalog seeds a
/// new artifact through the same export and the same check (`🌎️hub/🗿️artifact-authority/
/// 🔏️trusted-catalog`, `initial_pair`) — so a host that opens a document on this genesis starts from
/// the hub's own baseline, and every mutation it authors carries the document's identity.
pub async fn component_document_genesis(schema: &str, document_id: &str) -> Result<Option<ComponentDocumentGenesis>, VcsError> {
    let codec = component_document_codec_registry().read().map_err(|_| VcsError::ValidationFailed("component codec registry is unavailable".into()))?.get(schema).cloned();
    let Some(codec) = codec else { return Ok(None) };
    let genesis = codec.genesis(document_id).await?;
    let log = crate::os_spr::decode_history(&genesis.spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(format!("component genesis history for {schema:?}: {error}")))?;
    if log.doc_id != document_id || log.schema != schema || !log.edits.is_empty() || !log.transitions.is_empty() || !log.conflicts.is_empty() {
        return Err(VcsError::ValidationFailed(format!("component genesis for {schema:?} is not a zero-history document {document_id:?}")));
    }
    Ok(Some(genesis))
}

/// @emoji 📜️ Reads the document schema id from an encoded `.spr` history log.
pub async fn lane_schema_from_spr(""",
)

edit(
    "🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs",
    "use semio_framework_os_kernel::os_store::{ArtifactTextFiles, ComponentDocumentCodec, ComponentDocumentCodecFuture, VcsError};",
    "use semio_framework_os_kernel::os_store::{ArtifactTextFiles, ComponentDocumentCodec, ComponentDocumentCodecFuture, ComponentDocumentGenesis, VcsError};",
)
edit(
    "🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs",
    """            Ok(ArtifactTextFiles { dsl: mirror.dsl, ops: mirror.ops })
        })
    }
}""",
    """            Ok(ArtifactTextFiles { dsl: mirror.dsl, ops: mirror.ops })
        })
    }

    fn genesis<'a>(&'a self, document_id: &'a str) -> ComponentDocumentCodecFuture<'a, ComponentDocumentGenesis> {
        Box::pin(async move {
            let pair = self.owned().codec_genesis(&self.compiled, &self.schema, document_id, GUEST_CODEC_BUDGET).await.map_err(|fault| VcsError::ValidationFailed(format!("component codec.genesis({}): {fault}", self.schema)))?;
            Ok(ComponentDocumentGenesis { pack: pair.pack, spr: pair.spr })
        })
    }
}""",
)

edit(
    "🏪️store/🔄️sync/🧪️tests/🔬️document-socket-connect/🦀️.rs",
    """    fn print_mirror<'a>(&'a self, _pack: &'a [u8], _spr: &'a [u8]) -> crate::os_store::ComponentDocumentCodecFuture<'a, crate::os_store::ArtifactTextFiles> {
        Box::pin(async move { Err(crate::os_store::VcsError::Deserialize("fixture component prints no mirror".into())) })
    }
}""",
    """    fn print_mirror<'a>(&'a self, _pack: &'a [u8], _spr: &'a [u8]) -> crate::os_store::ComponentDocumentCodecFuture<'a, crate::os_store::ArtifactTextFiles> {
        Box::pin(async move { Err(crate::os_store::VcsError::Deserialize("fixture component prints no mirror".into())) })
    }

    fn genesis<'a>(&'a self, _document_id: &'a str) -> crate::os_store::ComponentDocumentCodecFuture<'a, crate::os_store::ComponentDocumentGenesis> {
        Box::pin(async move { Err(crate::os_store::VcsError::ValidationFailed("fixture component mints no genesis".into())) })
    }
}""",
)
print("ok")
