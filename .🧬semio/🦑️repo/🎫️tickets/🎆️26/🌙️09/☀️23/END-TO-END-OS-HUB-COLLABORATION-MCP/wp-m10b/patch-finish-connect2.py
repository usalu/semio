#!/usr/bin/env python3
from pathlib import Path

main = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/links/os-store")
sync = next(p for p in main.iterdir() if p.is_dir() and "sync" in p.name)
rs = next(p for p in sync.glob("*.rs"))
text = rs.read_text()

old = """                    {
                        let _ = stream.close(None).await;
                        self.clear_socket_epoch();
                        self.schedule_reconnect().await;
                        return;
                    }
                    let pack_schema_hash = authority.pack_schema_hash;
                    let (write, read) = stream.split();
                    self.semio_hub = Some(HubConn { write, read });
                    self.socket_actor = Some(socket_actor);
"""

new = """                    {
                        m10b_debug(&format!(
                            "[DEBUG] m10b finish_connect REJECT cancel={} expired={} origin_ok={} space_ok={} doc_ok={} schema_ok={} hash_ok={} surface_ok={} lease_ok={} schema={} auth_schema={}",
                            self.operation_cancel.is_cancelled_now(),
                            authority.expires_at_unix_ms <= now,
                            authority.hub_origin.trim_end_matches('/') == self.hub_base_url.as_deref().unwrap_or_default().trim_end_matches('/'),
                            authority.scope.space_id == self.hub_space_id.as_deref().unwrap_or_default(),
                            authority.scope.document_id == self.document_id,
                            authority.artifact.schema == self.schema,
                            local_schema_hash == Some(authority.pack_schema_hash),
                            !self.hub_surface.as_ref().is_some_and(|surface| surface != &authority.surface.surface_id),
                            !self.document_execution_target_lease.as_ref().is_some_and(|lease| !authority.matches_lease_fields(lease)),
                            self.schema,
                            authority.artifact.schema
                        ));
                        let _ = stream.close(None).await;
                        self.clear_socket_epoch();
                        self.schedule_reconnect().await;
                        return;
                    }
                    m10b_debug(&format!("[DEBUG] m10b finish_connect OK actor={}", socket_actor));
                    let pack_schema_hash = authority.pack_schema_hash;
                    let (write, read) = stream.split();
                    self.semio_hub = Some(HubConn { write, read });
                    self.socket_actor = Some(socket_actor);
"""

if old not in text:
    raise SystemExit("OK block not found")
text = text.replace(old, new, 1)

old_err = """                Err(()) => {
                    self.clear_socket_epoch();
                    self.schedule_reconnect().await;
                }
            }
        }

        async fn schedule_reconnect(&mut self) {
"""
new_err = """                Err(()) => {
                    m10b_debug("[DEBUG] m10b finish_connect Err(()) — connect/admit failed");
                    self.clear_socket_epoch();
                    self.schedule_reconnect().await;
                }
            }
        }

        async fn schedule_reconnect(&mut self) {
"""
if old_err not in text:
    raise SystemExit("Err block not found")
text = text.replace(old_err, new_err, 1)

# also log start_connect early returns
old_start = """        async fn start_connect_hub(&mut self) {
            let Some(base_url) = self.hub_base_url.clone() else { return };
            if self.semio_hub.is_some() || self.connect_future.is_some() || self.reconnect_at.is_some_and(|deadline| deadline > Instant::now()) {
                return;
            }
            if self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
                self.schedule_reconnect().await;
                return;
            }
"""
new_start = """        async fn start_connect_hub(&mut self) {
            let Some(base_url) = self.hub_base_url.clone() else {
                m10b_debug("[DEBUG] m10b start_connect skip: no hub_base_url");
                return;
            };
            if self.semio_hub.is_some() || self.connect_future.is_some() || self.reconnect_at.is_some_and(|deadline| deadline > Instant::now()) {
                m10b_debug(&format!(
                    "[DEBUG] m10b start_connect skip: hub={} future={} reconnect_wait={}",
                    self.semio_hub.is_some(),
                    self.connect_future.is_some(),
                    self.reconnect_at.is_some_and(|deadline| deadline > Instant::now())
                ));
                return;
            }
            if self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
                m10b_debug("[DEBUG] m10b start_connect skip: no credential");
                self.schedule_reconnect().await;
                return;
            }
            m10b_debug(&format!("[DEBUG] m10b start_connect BEGIN url={}", base_url));
"""
if old_start not in text:
    raise SystemExit("start_connect not found")
text = text.replace(old_start, new_start, 1)

rs.write_text(text)
print("patched ok")
