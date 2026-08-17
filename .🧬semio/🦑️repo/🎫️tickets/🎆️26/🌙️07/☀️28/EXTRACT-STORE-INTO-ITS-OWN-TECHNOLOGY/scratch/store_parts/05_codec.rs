//#region 🔖️CodecRegistry
/// @emoji 🗂️ Type-erased document codec — the bridge a schema-string-keyed caller (chiefly
/// `framework/sync`'s `FolderEndpoint`) uses to print/parse pack+ops without naming the concrete
/// `P`/`Operation` types at that layer. Built once per document kind via `DocumentCodec::of`
/// (wrapped one line per app by `register_document_codec_for_app` in `framework/plugin/rs/lib.rs`,
/// wave 2) and looked up by `schema` string through `register_document_codec`/`document_codec`.
#[derive(Clone)]
pub struct DocumentCodec {
    pub schema: String,
    pub extension: &'static str,
    /// @emoji 📤️ `envelope_json -> (pack files, dsl mirror text)` — the write path: `pack` is what
    /// `FolderTextStorage::write_pack`/`FolderSqliteStorage::write_pack` persist as authoritative,
    /// the returned `String` is the always-written DSL mirror (`print_dsl` on the initial
    /// projection).
    pub print: fn(&str) -> Result<(DocumentPackFiles, String), VcsError>,
    /// @emoji 📥️ `(pack bytes, ops text) -> envelope_json` — the pack-first read path.
    pub parse: fn(&[u8], &str) -> Result<String, VcsError>,
    /// @emoji 📥️ `(dsl text, ops text) -> envelope_json` — the DSL-mirror fallback read path (no
    /// `.pack` file yet: hand-authored or freshly imported documents).
    pub parse_dsl: fn(&str, &str) -> Result<String, VcsError>,
}

impl DocumentCodec {
    /// @emoji 🏗️ Monomorphizes three non-capturing bridge functions for `(P, Operation)` — each a
    /// genuine zero-sized `fn` item, coercible to a bare `fn` pointer — and pairs them with `schema`/
    /// `P::EXTENSION`. One call site per document kind (`register_document_codec_for_app`).
    pub fn of<P, Operation>(schema: impl Into<String>) -> Self
    where
        P: Clone + PartialEq + Serialize + DeserializeOwned + DocumentDsl + DocumentPack + Send + 'static,
        Operation: crate::Operation<P> + PartialEq + Serialize + DeserializeOwned + OpText + Send + 'static,
    {
        fn print_impl<P, Operation>(envelope_json: &str) -> Result<(DocumentPackFiles, String), VcsError>
        where
            P: DocumentDsl + DocumentPack + Serialize + DeserializeOwned,
            Operation: OpText + Serialize + DeserializeOwned,
        {
            let envelope: DocumentEnvelope<P, Operation> =
                serde_json::from_str(envelope_json).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            let pack_files = print_document_pack(&envelope)?;
            let dsl_mirror = envelope.vcs.initial_projection.print_dsl();
            Ok((pack_files, dsl_mirror))
        }

        fn parse_impl<P, Operation>(pack: &[u8], ops: &str) -> Result<String, VcsError>
        where
            P: Clone + DocumentPack + Serialize + DeserializeOwned,
            Operation: OpText + crate::Operation<P> + Serialize + DeserializeOwned,
        {
            let parsed: ParsedDocumentText<P, Operation> = parse_document_pack(pack, ops).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            serde_json::to_string(&parsed.envelope).map_err(|error| VcsError::Serialize(error.to_string()))
        }

        fn parse_dsl_impl<P, Operation>(dsl: &str, ops: &str) -> Result<String, VcsError>
        where
            P: Clone + DocumentDsl + Serialize + DeserializeOwned,
            Operation: OpText + crate::Operation<P> + Serialize + DeserializeOwned,
        {
            let parsed: ParsedDocumentText<P, Operation> = parse_document_text(dsl, ops).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            serde_json::to_string(&parsed.envelope).map_err(|error| VcsError::Serialize(error.to_string()))
        }

        Self {
            schema: schema.into(),
            extension: P::EXTENSION,
            print: print_impl::<P, Operation>,
            parse: parse_impl::<P, Operation>,
            parse_dsl: parse_dsl_impl::<P, Operation>,
        }
    }
}

static DOCUMENT_CODEC_REGISTRY: std::sync::OnceLock<std::sync::RwLock<HashMap<String, DocumentCodec>>> = std::sync::OnceLock::new();

fn document_codec_registry() -> &'static std::sync::RwLock<HashMap<String, DocumentCodec>> {
    DOCUMENT_CODEC_REGISTRY.get_or_init(|| std::sync::RwLock::new(HashMap::new()))
}

/// @emoji 📝️ Registers (or overwrites) the codec for `codec.schema` — idempotent, safe to call
/// repeatedly (every app's registration fn calls this once per document kind at plugin-init time).
pub fn register_document_codec(codec: DocumentCodec) {
    let mut registry = document_codec_registry().write().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.insert(codec.schema.clone(), codec);
}

/// @emoji 🔎️ Looks up the codec registered for `schema`, if any.
pub fn document_codec(schema: &str) -> Option<DocumentCodec> {
    let registry = document_codec_registry().read().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.get(schema).cloned()
}
//#endregion 🔖️CodecRegistry
