#!/usr/bin/env python3
from pathlib import Path

main = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/links/os-store")
sync = next(p for p in main.iterdir() if p.is_dir() and "sync" in p.name)
rs = next(p for p in sync.glob("*.rs"))
text = rs.read_text()

old = '''        async fn finish_connect_hub(&mut self, connection: Result<ConnectedDocumentSocket, ()>) {
            match connection {
                Ok(ConnectedDocumentSocket { mut stream, socket_actor, authority }) => {
                    let local_schema_hash = crate::os_store::document_codec(&self.schema).await.ok().flatten().map(|codec| codec.pack_schema_hash);
                    let now = now_ms().await;
                    if self.operation_cancel.is_cancelled_now()
                        || authority.expires_at_unix_ms <= now
                        || authority.hub_origin.trim_end_matches('/') != self.hub_base_url.as_deref().unwrap_or_default().trim_end_matches('/')
                        || authority.scope.space_id != self.hub_space_id.as_deref().unwrap_or_default()
                        || authority.scope.document_id != self.document_id
                        || authority.artifact.schema != self.schema
                        || local_schema_hash != Some(authority.pack_schema_hash)
                        || self.hub_surface.as_ref().is_some_and(|surface| surface != &authority.surface.surface_id)
                        || self.document_execution_target_lease.as_ref().is_some_and(|lease| !authority.matches_lease_fields(lease))
                    {
                        let _ = stream.close(None).await;
                        self.clear_socket_epoch();
                        self.schedule_reconnect().await;
                        return;
                    }'''

new = '''        async fn finish_connect_hub(&mut self, connection: Result<ConnectedDocumentSocket, ()>) {
            match connection {
                Ok(ConnectedDocumentSocket { mut stream, socket_actor, authority }) => {
                    let local_schema_hash = crate::os_store::document_codec(&self.schema).await.ok().flatten().map(|codec| codec.pack_schema_hash);
                    let now = now_ms().await;
                    if self.operation_cancel.is_cancelled_now()
                        || authority.expires_at_unix_ms <= now
                        || authority.hub_origin.trim_end_matches('/') != self.hub_base_url.as_deref().unwrap_or_default().trim_end_matches('/')
                        || authority.scope.space_id != self.hub_space_id.as_deref().unwrap_or_default()
                        || authority.scope.document_id != self.document_id
                        || authority.artifact.schema != self.schema
                        || local_schema_hash != Some(authority.pack_schema_hash)
                        || self.hub_surface.as_ref().is_some_and(|surface| surface != &authority.surface.surface_id)
                        || self.document_execution_target_lease.as_ref().is_some_and(|lease| !authority.matches_lease_fields(lease))
                    {
                        m10b_debug(&format!(
                            "[DEBUG] m10b finish_connect REJECT cancel={} expired={} origin_ok={} space_ok={} doc_ok={} schema_ok={} hash_ok={:?} surface_ok={} lease_ok={} local_hash={:?} auth_hash={:02x?} schema={} auth_schema={}",
                            self.operation_cancel.is_cancelled_now(),
                            authority.expires_at_unix_ms <= now,
                            authority.hub_origin.trim_end_matches('/') == self.hub_base_url.as_deref().unwrap_or_default().trim_end_matches('/'),
                            authority.scope.space_id == self.hub_space_id.as_deref().unwrap_or_default(),
                            authority.scope.document_id == self.document_id,
                            authority.artifact.schema == self.schema,
                            local_schema_hash == Some(authority.pack_schema_hash),
                            !self.hub_surface.as_ref().is_some_and(|surface| surface != &authority.surface.surface_id),
                            !self.document_execution_target_lease.as_ref().is_some_and(|lease| !authority.matches_lease_fields(lease)),
                            local_schema_hash,
                            authority.pack_schema_hash,
                            self.schema,
                            authority.artifact.schema
                        ));
                        let _ = stream.close(None).await;
                        self.clear_socket_epoch();
                        self.schedule_reconnect().await;
                        return;
                    }
                    m10b_debug(&format!("[DEBUG] m10b finish_connect OK actor={}", socket_actor));'''

if old not in text:
    raise SystemExit("finish_connect block not found")
text = text.replace(old, new, 1)

old2 = '''                Err(()) => {
                    self.semio_hub = None;
                    self.clear_socket_epoch();
                    self.schedule_reconnect().await;
                }
            }
        }'''
# might appear multiple times - be more specific
idx = text.find("async fn finish_connect_hub")
chunk = text[idx:idx+2500]
if "Err(()) =>" not in chunk:
    raise SystemExit("Err arm not in finish_connect")
# find Err within finish_connect only
# simpler append after OK path's schedule for Err
old_err = None
# search from finish_connect
rest = text[idx:]
marker = "Err(()) => {\n                    self.semio_hub = None;\n                    self.clear_socket_epoch();\n                    self.schedule_reconnect().await;\n                }\n            }\n        }"
if marker not in rest:
    # try alternate formatting
    print("looking for Err arm...")
    for i, line in enumerate(rest.splitlines()[:80]):
        if "Err(())" in line:
            print(i, line)
    raise SystemExit("err marker missing")
text = text[:idx] + rest.replace(marker, "Err(()) => {\n                    m10b_debug(\"[DEBUG] m10b finish_connect Err(()) — connect/admit failed\");\n                    self.semio_hub = None;\n                    self.clear_socket_epoch();\n                    self.schedule_reconnect().await;\n                }\n            }\n        }", 1)

rs.write_text(text)
print("patched finish_connect")
