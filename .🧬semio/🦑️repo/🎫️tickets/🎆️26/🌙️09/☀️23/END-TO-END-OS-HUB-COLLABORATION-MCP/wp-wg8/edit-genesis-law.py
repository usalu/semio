import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs")
text = path.read_text()


def replace(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


replace(
    """/// 🧬️ A component codec that answers `codec.print-mirror` with the pair's byte counts, so a law can
/// see the pair it was handed.
struct EchoComponentCodec {
    schema: &'static str,
}
""",
    """/// 🧬️ A component codec that answers `codec.print-mirror` with the pair's byte counts, so a law can
/// see the pair it was handed, and mints a zero-history genesis stamped `minted_identity` (the asked
/// identity when `None`).
struct EchoComponentCodec {
    schema: &'static str,
    minted_identity: Option<&'static str>,
}
""",
)
replace(
    """    fn print_mirror<'a>(&'a self, pack: &'a [u8], spr: &'a [u8]) -> ComponentDocumentCodecFuture<'a, ArtifactTextFiles> {
        Box::pin(async move { Ok(ArtifactTextFiles { dsl: pack.len().to_string(), ops: spr.len().to_string() }) })
    }
}
""",
    """    fn print_mirror<'a>(&'a self, pack: &'a [u8], spr: &'a [u8]) -> ComponentDocumentCodecFuture<'a, ArtifactTextFiles> {
        Box::pin(async move { Ok(ArtifactTextFiles { dsl: pack.len().to_string(), ops: spr.len().to_string() }) })
    }

    fn genesis<'a>(&'a self, document_id: &'a str) -> ComponentDocumentCodecFuture<'a, ComponentDocumentGenesis> {
        Box::pin(async move {
            let history = crate::os_spr::HistoryLog { doc_id: self.minted_identity.unwrap_or(document_id).to_string(), schema: self.schema.to_string(), ..crate::os_spr::HistoryLog::default() };
            let spr = crate::os_spr::encode_history(&history, &crate::os_spr::EncodeOptions::default()).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
            Ok(ComponentDocumentGenesis { pack: document_id.as_bytes().to_vec(), spr })
        })
    }
}
""",
)
replace('EchoComponentCodec { schema: "test.kind-codec-linked/v1" }', 'EchoComponentCodec { schema: "test.kind-codec-linked/v1", minted_identity: None }')
replace('EchoComponentCodec { schema: "test.kind-codec-component/v1" }', 'EchoComponentCodec { schema: "test.kind-codec-component/v1", minted_identity: None }')
replace(
    """    assert!(document_kind_codec("test.kind-codec-unowned/v1").await.expect("registry").is_none());
}
""",
    """    assert!(document_kind_codec("test.kind-codec-unowned/v1").await.expect("registry").is_none());
}

/// 🌱️ A host opens a document on the genesis its owning component mints for exactly that identity — the
/// baseline the hub seeds a new artifact from — so every mutation the guest then authors names the
/// document it opened. Measured without it (ticket 26/09/23 slice WG8, two-user gate run 12): the native
/// guest authored against its app id (`s.block.block2d@1/*#editor`) and the document actor refused
/// every edit as `document backbone scope mismatch`. A component minting another identity is refused;
/// a kind no mounted component owns has no genesis here.
#[semio_framework_async_macros::async_test]
async fn a_document_opens_on_the_genesis_its_owning_component_mints_for_its_identity() {
    register_component_document_codec(Arc::new(EchoComponentCodec { schema: "test.genesis-owned/v1", minted_identity: None })).expect("component codec registers");
    register_component_document_codec(Arc::new(EchoComponentCodec { schema: "test.genesis-foreign/v1", minted_identity: Some("another-document") })).expect("component codec registers");

    let genesis = component_document_genesis("test.genesis-owned/v1", "artifact-1").await.expect("owned genesis").expect("a mounted component owns the kind");
    let history = crate::os_spr::decode_history(&genesis.spr, &crate::os_spr::DecodeOptions::default()).await.expect("genesis history decodes");
    assert_eq!((history.doc_id.as_str(), history.schema.as_str(), history.edits.len()), ("artifact-1", "test.genesis-owned/v1", 0));
    assert_eq!(genesis.pack, b"artifact-1".to_vec(), "the pair is the component's own answer");

    assert!(matches!(component_document_genesis("test.genesis-foreign/v1", "artifact-1").await, Err(VcsError::ValidationFailed(_))), "a genesis of another identity is refused");
    assert_eq!(component_document_genesis("test.genesis-unowned/v1", "artifact-1").await, Ok(None));
}
""",
)
path.write_text(text)
print("ok")
