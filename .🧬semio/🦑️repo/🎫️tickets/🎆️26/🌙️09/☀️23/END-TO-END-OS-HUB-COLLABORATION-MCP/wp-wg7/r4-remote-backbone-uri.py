"""🐍️ WG7 R4 — the wgpu sync card's `remote://host/space/document` names all three parts (React C1c parity)."""
from pathlib import Path

ENGINE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SHELL = Path(ENGINE + "/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
LAWS = Path(ENGINE + "/🧱️elements/🐚️Shell/🧪️tests/📂️wgpu-document-relay/🦀️.rs")

def swap(text, old, new):
    assert text.count(old) == 1, (text.count(old), old[:160])
    return text.replace(old, new, 1)

s = SHELL.read_text(encoding="utf-8")
l = LAWS.read_text(encoding="utf-8")

s = swap(s, '''    async fn attach_sync_backbone(&mut self, uri: String) -> Result<(), String> {
        let session = self.session.clone().ok_or("session missing")?;
        let document_id = self.sync_document_id().ok_or("session missing")?;
        let bindings = Self::parse_persistence_binding(&uri)?;
        self.open_document(document_id, session.app.io.artifact_schema.clone(), bindings, None, Some(uri)).await
    }''', '''    ///
    /// 🌐️ A `remote://` uri names the hub document itself — host, space AND document
    /// ([`parse_remote_backbone_uri`]) — and binds the session app's own canonical surface, so two
    /// shells attaching the same uri open the same hub document (the React shell's C1c fix).
    async fn attach_sync_backbone(&mut self, uri: String) -> Result<(), String> {
        let session = self.session.clone().ok_or("session missing")?;
        let (document_id, bindings, surface) = match parse_remote_backbone_uri(&uri) {
            Some(remote) => {
                let surface = semio_framework::manifest::surface_app_id(&session.app.dialect, session.app.role);
                (remote.document_id, vec![PersistenceBinding::Hub { base_url: format!("http://{}", remote.host_port), space_id: remote.space_id, surface: Some(surface.clone()) }], Some(surface))
            }
            None => (self.sync_document_id().ok_or("session missing")?, Self::parse_persistence_binding(&uri)?, None),
        };
        self.open_document(document_id, session.app.io.artifact_schema.clone(), bindings, surface, Some(uri)).await
    }''')

s = swap(s, '''    /// @emoji 🧭️ Parses a shell sync-card uri into the `framework/sync` persistence bindings a
    /// document actor opens. `folder://` → the multi-document append-only event log; `file://x.json` → its
    /// parent folder's store (single-blob export demoted per the plan); `remote://host:port[/space_id]`
    /// → the semio_hub over WebSocket, studio-scoped (an omitted studio segment falls back to `"default"`).
    /// Superseded the fetch/CRUD `shell_backbone_read`/`write` pair.
    fn parse_persistence_binding(uri: &str) -> Result<Vec<PersistenceBinding>, String> {
        if let Some(rest) = uri.strip_prefix("remote://") {
            let (host_port, space_id) = rest.split_once('/').unwrap_or((rest, "default"));
            let space_id = if space_id.is_empty() { "default" } else { space_id };
            return Ok(vec![PersistenceBinding::Hub { base_url: format!("http://{host_port}"), space_id: space_id.to_string(), surface: None }]);
        }''', '''    /// @emoji 🧭️ Parses a local sync-card uri into the `framework/sync` persistence bindings a
    /// document actor opens. `folder://` → the multi-document append-only event log; `file://x.json` → its
    /// parent folder's store (single-blob export demoted per the plan). A `remote://` uri names a hub
    /// document of its own and is read by [`parse_remote_backbone_uri`]; one missing a part is refused.
    fn parse_persistence_binding(uri: &str) -> Result<Vec<PersistenceBinding>, String> {
        if uri.starts_with("remote://") {
            return Err(format!("a remote backbone names host, space and document: {uri}"));
        }''')

s = swap(s, '''/// 🪪️ Admits a hub-selected execution-target lease only for the package this shell actually mounted:''', '''/// 🌐️ The three parts a `remote://host:port/space/document` sync-card uri names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RemoteBackboneUri {
    pub(crate) host_port: String,
    pub(crate) space_id: String,
    pub(crate) document_id: String,
}

/// 🌐️ The wgpu twin of the React shell's `parseRemoteBackboneUri` (`💻️os/🟦️.ts`): everything up to
/// the first slash is the host, up to the second the space, the rest the document; a uri missing a
/// part names no hub document.
pub(crate) fn parse_remote_backbone_uri(uri: &str) -> Option<RemoteBackboneUri> {
    let rest = uri.strip_prefix("remote://")?;
    let first = rest.find('/').filter(|index| *index > 0)?;
    let second = rest[first + 1..].find('/').map(|index| index + first + 1).filter(|index| *index > 0)?;
    Some(RemoteBackboneUri { host_port: rest[..first].to_string(), space_id: rest[first + 1..second].to_string(), document_id: rest[second + 1..].to_string() })
}

/// 🪪️ Admits a hub-selected execution-target lease only for the package this shell actually mounted:''')

l += '''
/// 🌐️ The sync card's remote uri names all three parts exactly as React's `parseRemoteBackboneUri`
/// reads them — the regression React's C1c fixed was a parser that took `space/document` as the space.
#[test]
fn a_remote_backbone_uri_names_host_space_and_document() {
    assert_eq!(
        parse_remote_backbone_uri("remote://127.0.0.1:7800/space-1/doc-a"),
        Some(RemoteBackboneUri { host_port: "127.0.0.1:7800".into(), space_id: "space-1".into(), document_id: "doc-a".into() })
    );
    assert_eq!(parse_remote_backbone_uri("remote://127.0.0.1:7800/space-1"), None, "no document part");
    assert_eq!(parse_remote_backbone_uri("remote:///space-1/doc-a"), None, "no host part");
    assert_eq!(parse_remote_backbone_uri("folder:///tmp/space"), None, "not a remote uri");
    assert!(ShellState::parse_persistence_binding("remote://127.0.0.1:7800/space-1").is_err(), "a partial remote uri is refused, never guessed");
}
'''

SHELL.write_text(s, encoding="utf-8")
LAWS.write_text(l, encoding="utf-8")
print("r4: applied")
