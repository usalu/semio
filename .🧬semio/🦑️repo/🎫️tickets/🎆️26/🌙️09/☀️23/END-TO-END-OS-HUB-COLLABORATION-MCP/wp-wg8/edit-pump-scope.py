import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()
start = text.index("    pub async fn pump_sync_events(&mut self) -> bool {")
end = text.index("    fn footer_presence_rows(&self)", start)
body = text[start:end]


def replace(old, new, count=1):
    global body
    assert body.count(old) == count, (body.count(old), old[:90])
    body = body.replace(old, new)


replace("        let mut changed = directory_changed;\n", "        let mut changed = directory_changed;\n        let mut document_changed = false;\n        let mut sync_panel_changed = false;\n")
replace("                            Ok(()) => changed = true,\n", "                            Ok(()) => {\n                                changed = true;\n                                document_changed = true;\n                            }\n", count=2)
replace("""                    self.sync_bootstrap_progress = Some((received_bytes, total_bytes, received_chunks, total_chunks));
                    changed = true;
""", """                    self.sync_bootstrap_progress = Some((received_bytes, total_bytes, received_chunks, total_chunks));
                    changed = true;
                    sync_panel_changed = true;
""")
replace("""                    self.sync_status = Some(status);
                    changed = true;
""", """                    self.sync_status = Some(status);
                    changed = true;
                    sync_panel_changed = true;
""")
replace("""                    self.sync_card_kind = Some("conflict".into());
                    changed = true;
                }
                // 👥️ Peer session identity""", """                    self.sync_card_kind = Some("conflict".into());
                    changed = true;
                    sync_panel_changed = true;
                }
                // 👥️ Peer session identity""")
replace("""                                    self.queue_host_effects(&controller_id, remaining);
                                    changed = true;
""", """                                    self.queue_host_effects(&controller_id, remaining);
                                    changed = true;
                                    document_changed = true;
""")
replace("""            self.sync_card_kind = Some("conflict".into());
            changed = true;
        }
        if changed {
            let _ = self.refresh_ui(UiDirtyScope::Full).await;
        }
        changed
""", """            self.sync_card_kind = Some("conflict".into());
            changed = true;
            document_changed = true;
        }
        if document_changed {
            let _ = self.refresh_ui(UiDirtyScope::Full).await;
        } else if sync_panel_changed {
            let _ = self.republish_shell_panel_document(FRAMEWORK_SYNC_PANEL_TAB_ID);
        }
        changed
""")
doc_old = """    /// whether anything changed (and a re-render was issued).
"""
doc_new = """    /// whether anything changed. Only a change to the guest's document refreshes the guest's bodies: a
    /// status, bootstrap or conflict change republishes the host-owned Sync panel, and a presence roster
    /// is painted from shell state by the footer on the next frame. Refreshing every guest body per
    /// presence frame (ten per second per peer) held each pump for 3.4–5.6 s in a debug build and froze
    /// the shell while a peer was online (ticket 26/09/23 slice WG8, two-user gate run 15).
"""
text = text[:start] + body + text[end:]
assert text.count(doc_old) == 1
text = text.replace(doc_old, doc_new)
path.write_text(text)
print("ok")
