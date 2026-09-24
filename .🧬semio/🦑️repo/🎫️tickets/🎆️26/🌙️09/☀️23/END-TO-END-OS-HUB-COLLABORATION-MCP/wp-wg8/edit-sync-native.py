import pathlib
path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs")
text = path.read_text()
def swap(old, new):
    global text
    assert text.count(old) == 1, old[:120]
    text = text.replace(old, new)
swap('''/// @emoji 🧬️ The pack schema identity a document socket's hello and bootstrap are checked against: the
/// linked codec's when this process registered one, otherwise the verified execution-target lease's —
/// the hub-selected package this client mounted, whose descriptor carries the kind the host links no
/// codec for. Neither is no identity at all, and the caller refuses to dial rather than guess.
pub async fn document_pack_schema_hash(schema: &str, lease: Option<&crate::os_directory::DocumentExecutionTargetLeaseFieldsV1>) -> Option<[u8; 32]> {
    if let Ok(Some(codec)) = crate::os_store::document_codec(schema).await {
        return Some(codec.pack_schema_hash);
    }
''', '''/// @emoji 🧬️ The pack schema identity a document socket's hello and bootstrap are checked against: the
/// kind's codec when this process resolves one ([`crate::os_store::document_kind_codec`] — the linked
/// Rust codec, else the codec of the mounted component that owns the kind, asked of that component
/// exactly as the hub's trusted catalog asks it), otherwise the verified execution-target lease's — the
/// hub-selected package a client that mounted no owning component will run. Neither is no identity at
/// all, and the caller refuses to dial rather than guess.
pub async fn document_pack_schema_hash(schema: &str, lease: Option<&crate::os_directory::DocumentExecutionTargetLeaseFieldsV1>) -> Option<[u8; 32]> {
    if let Ok(Some(codec)) = crate::os_store::document_kind_codec(schema).await {
        return codec.pack_schema_hash().await.ok();
    }
''')
swap('''                    let codec = crate::os_store::document_codec(schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec registered for schema {schema:?} — cannot persist synchronized archive mirrors"))?;
                    let mirror = (codec.print_mirror)(&archive.parent_pack, &archive.parent_spr).await.map_err(|error| error.to_string())?;''', '''                    let codec = crate::os_store::document_kind_codec(schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec resolves schema {schema:?} — cannot persist synchronized archive mirrors"))?;
                    let mirror = codec.print_mirror(&archive.parent_pack, &archive.parent_spr).await.map_err(|error| error.to_string())?;''')
swap('''            let Some(pack_schema_hash) = crate::os_store::document_codec(&schema).await.ok().flatten().map(|codec| codec.pack_schema_hash) else {''', '''            let Some(pack_schema_hash) = document_pack_schema_hash(&schema, self.document_execution_target_lease.as_ref()).await else {''')
swap('''                    let local_schema_hash = crate::os_store::document_codec(&self.schema).await.ok().flatten().map(|codec| codec.pack_schema_hash);''', '''                    let local_schema_hash = document_pack_schema_hash(&self.schema, self.document_execution_target_lease.as_ref()).await;''')
swap('''            let codec = match crate::os_store::document_codec(&self.schema).await {
                Ok(Some(codec)) => codec,
                Ok(None) => {
                    self.fail_artifact_bootstrap(format!("no document codec registered for schema {:?}", self.schema)).await;
                    return;
                }
                Err(error) => {
                    self.fail_artifact_bootstrap(error.to_string()).await;
                    return;
                }
            };
            if let Err(error) = validate_artifact_bootstrap_identity(&bootstrap, &self.document_id, &self.schema, codec.pack_schema_hash, &server_frontier) {''', '''            let Some(local_schema_hash) = document_pack_schema_hash(&self.schema, self.document_execution_target_lease.as_ref()).await else {
                self.fail_artifact_bootstrap(format!("no pack schema identity resolves schema {:?}", self.schema)).await;
                return;
            };
            if let Err(error) = validate_artifact_bootstrap_identity(&bootstrap, &self.document_id, &self.schema, local_schema_hash, &server_frontier) {''')
swap('''            let codec = crate::os_store::document_codec(&self.schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec registered for schema {:?}", self.schema))?;
            if codec.pack_schema_hash != pending.pack_schema_hash {
                return Err("artifact bootstrap codec changed during transfer".into());
            }
            (codec.print_mirror)(&pair.pack, &pair.spr).await.map_err(|error| format!("artifact bootstrap decode failed: {error}"))?;''', '''            let codec = crate::os_store::document_kind_codec(&self.schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec resolves schema {:?}", self.schema))?;
            if codec.pack_schema_hash().await.map_err(|error| error.to_string())? != pending.pack_schema_hash {
                return Err("artifact bootstrap codec changed during transfer".into());
            }
            codec.print_mirror(&pair.pack, &pair.spr).await.map_err(|error| format!("artifact bootstrap decode failed: {error}"))?;''')
path.write_text(text)
print("sync native edited")
