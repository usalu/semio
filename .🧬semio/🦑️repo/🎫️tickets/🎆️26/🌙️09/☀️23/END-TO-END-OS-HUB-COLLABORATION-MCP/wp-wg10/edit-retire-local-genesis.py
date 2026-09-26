#!/usr/bin/env python3
"""WG10 s13: the hub's canonical checkpoint pair is the one seed of a hub open on every shell, so the local
`codec.genesis` mint that only served hub opens (native owned interpreter + wasm32 jco) is retired: the store's
`ComponentDocumentCodec::genesis`, `ComponentDocumentGenesis`, `component_document_genesis`, their law and every
implementation, and the JS bridge's `codecGenesis`. Local (non-hub) documents never used it."""
import pathlib

M = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules")


def edit(rel, pairs):
    path = M / rel
    text = path.read_text()
    for old, new in pairs:
        assert text.count(old) == 1, (rel, old[:100])
        text = text.replace(old, new)
    path.write_text(text)


edit("🏪️store/🦀️.rs", [
    ("""    /// 📥️ `codec.print-mirror`: the pair's text mirror, which is also its validation fence.
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
""", """    /// 📥️ `codec.print-mirror`: the pair's text mirror, which is also its validation fence.
    fn print_mirror<'a>(&'a self, pack: &'a [u8], spr: &'a [u8]) -> ComponentDocumentCodecFuture<'a, ArtifactTextFiles>;
}
"""),
    ("""/// 🌱️ The genesis of `document_id` as the mounted component that owns `schema` mints it, refused
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

""", ""),
])
edit("🏪️store/🧪️tests/🔬️unit/🦀️.rs", [
    ("""/// 🧬️ A component codec that answers `codec.print-mirror` with the pair's byte counts, so a law can
/// see the pair it was handed, and mints a zero-history genesis stamped `minted_identity` (the asked
/// identity when `None`).
struct EchoComponentCodec {
    schema: &'static str,
    minted_identity: Option<&'static str>,
}
""", """/// 🧬️ A component codec that answers `codec.print-mirror` with the pair's byte counts, so a law can
/// see the pair it was handed.
struct EchoComponentCodec {
    schema: &'static str,
}
"""),
    ("""
    fn genesis<'a>(&'a self, document_id: &'a str) -> ComponentDocumentCodecFuture<'a, ComponentDocumentGenesis> {
        Box::pin(async move {
            let history = crate::os_spr::HistoryLog { doc_id: self.minted_identity.unwrap_or(document_id).to_string(), schema: self.schema.to_string(), ..crate::os_spr::HistoryLog::default() };
            let spr = crate::os_spr::encode_history(&history, &crate::os_spr::EncodeOptions::default()).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
            Ok(ComponentDocumentGenesis { pack: document_id.as_bytes().to_vec(), spr })
        })
    }
}
""", """}
"""),
    ("""    register_component_document_codec(Arc::new(EchoComponentCodec { schema: "test.kind-codec-linked/v1", minted_identity: None })).expect("component codec registers");
    register_component_document_codec(Arc::new(EchoComponentCodec { schema: "test.kind-codec-component/v1", minted_identity: None })).expect("component codec registers");""",
     """    register_component_document_codec(Arc::new(EchoComponentCodec { schema: "test.kind-codec-linked/v1" })).expect("component codec registers");
    register_component_document_codec(Arc::new(EchoComponentCodec { schema: "test.kind-codec-component/v1" })).expect("component codec registers");"""),
    ("""
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
""", ""),
])
edit("🏪️store/🔄️sync/🧪️tests/🔬️document-socket-connect/🦀️.rs", [
    ("""
    fn genesis<'a>(&'a self, _document_id: &'a str) -> crate::os_store::ComponentDocumentCodecFuture<'a, crate::os_store::ComponentDocumentGenesis> {
        Box::pin(async move { Err(crate::os_store::VcsError::ValidationFailed("fixture component mints no genesis".into())) })
    }
""", ""),
])
edit("🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs", [
    ("use semio_framework_os_kernel::os_store::{ArtifactTextFiles, ComponentDocumentCodec, ComponentDocumentCodecFuture, ComponentDocumentGenesis, VcsError};",
     "use semio_framework_os_kernel::os_store::{ArtifactTextFiles, ComponentDocumentCodec, ComponentDocumentCodecFuture, VcsError};"),
    ("""
    fn genesis<'a>(&'a self, document_id: &'a str) -> ComponentDocumentCodecFuture<'a, ComponentDocumentGenesis> {
        Box::pin(async move {
            let pair = self.owned().codec_genesis(&self.compiled, &self.schema, document_id, GUEST_CODEC_BUDGET).await.map_err(|fault| VcsError::ValidationFailed(format!("component codec.genesis({}): {fault}", self.schema)))?;
            Ok(ComponentDocumentGenesis { pack: pair.pack, spr: pair.spr })
        })
    }
""", ""),
])
edit("📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs", [
    ("""/// bridge (`codecPackSchemaHash`/`codecGenesis`/`codecPrintMirror`, `🐚️plugin-bridge/🟦️.ts`), on a live
/// instance's shard actor — the twin of the native `OwnedComponentDocumentCodec` a wgpu shell registers
/// in `create_app`. With it the mounted component IS the kind identity of a hub document (no
/// execution-target lease, no component SHA-256 equality with the catalog) and a hub-bound open seeds
/// the guest from the genesis the component mints (ticket 26/09/23 slices WG8 + WG7).""",
     """/// bridge (`codecPackSchemaHash`/`codecPrintMirror`, `🐚️plugin-bridge/🟦️.ts`), on a live instance's shard
/// actor — the twin of the native `OwnedComponentDocumentCodec` a wgpu shell registers in `create_app`. With it
/// the mounted component IS the kind identity of a hub document (ticket 26/09/23 slices WG8 + WG7); the document
/// itself opens on the hub's canonical checkpoint pair, on every shell (slice WG10)."""),
    ("    use semio_framework_os_kernel::os_store::{self, ArtifactTextFiles, ComponentDocumentCodec, ComponentDocumentCodecFuture, ComponentDocumentGenesis, VcsError};",
     "    use semio_framework_os_kernel::os_store::{self, ArtifactTextFiles, ComponentDocumentCodec, ComponentDocumentCodecFuture, VcsError};"),
    ("""
        fn genesis<'a>(&'a self, document_id: &'a str) -> ComponentDocumentCodecFuture<'a, ComponentDocumentGenesis> {
            Box::pin(async move {
                let fault = |error: String| VcsError::ValidationFailed(format!("component codec.genesis({}): {error}", self.schema));
                let answer = self.call("codecGenesis", &Array::of2(&JsValue::from_str(&self.schema), &JsValue::from_str(document_id))).await.map_err(fault)?;
                let half = |name: &str| Reflect::get(&answer, &JsValue::from_str(name)).map_err(|_| fault(format!("genesis has no {name}"))).and_then(|value| bytes(&value, name).map_err(fault));
                Ok(ComponentDocumentGenesis { pack: half("pack")?, spr: half("spr")? })
            })
        }
""", ""),
])
edit("📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts", [
    ("""  /** 🌱️ `codec.genesis(artifactKind, documentId)`: the zero-history `{ pack, spr }` the component mints. */
  readonly codecGenesis: (artifactKind: string, documentId: string) => Promise<{ readonly pack: Uint8Array; readonly spr: Uint8Array }>;
""", ""),
    ("""    codecGenesis: (artifactKind, documentId) => handle.codec({ operation: "genesis", artifactKind, documentId }).then((value) => codecPair(value)),
""", ""),
])
print("retired")
