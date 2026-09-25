import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()
T = 'let wg8_t = std::time::Instant::now();'
def mark(label):
    return f'eprintln!("[DEBUG] wg8 open {label} {{:?}}", wg8_t.elapsed());'
pairs = [
    ("        let instance_id = program.create_app(&app.id).await?;\n        let view_state = ViewModel {\n",
     f"        {T}\n        let instance_id = program.create_app(&app.id).await?;\n        {mark('switch.create_app')}\n        let view_state = ViewModel {{\n"),
    ("        self.session = Some(ActiveSession { plugin_id: plugin_id.to_string(), instance_id, app, view_state });\n        self.refresh_ui(UiDirtyScope::Full).await\n    }\n",
     f"        self.session = Some(ActiveSession {{ plugin_id: plugin_id.to_string(), instance_id, app, view_state }});\n        let wg8_r = self.refresh_ui(UiDirtyScope::Full).await;\n        {mark('switch.refresh')}\n        wg8_r\n    }}\n"),
    ("        self.checkpoint_before_detach().await;\n        self.detach_sync_backbone_internal().await?;\n        self.presence_surface = surface;\n",
     f"        {T}\n        self.checkpoint_before_detach().await;\n        {mark('doc.checkpoint')}\n        self.detach_sync_backbone_internal().await?;\n        {mark('doc.detach')}\n        self.presence_surface = surface;\n"),
    ("            plugin.load_app_document_pack(session.instance_id, &genesis.pack, &genesis.spr).await.map_err(|error| format!(\"document genesis load: {error}\"))?;\n        }\n",
     f"            {mark('doc.genesis')}\n            plugin.load_app_document_pack(session.instance_id, &genesis.pack, &genesis.spr).await.map_err(|error| format!(\"document genesis load: {{error}}\"))?;\n            {mark('doc.load')}\n        }}\n"),
    ("        let events = self.document_host.subscribe_key(&channels.document_key).await;\n",
     f"        let events = self.document_host.subscribe_key(&channels.document_key).await;\n        {mark('doc.actor')}\n"),
    ("        self.refresh_history_snapshot().await;\n        self.refresh_ui(UiDirtyScope::Full).await\n    }\n\n    /// 🪪️ The kind identity",
     f"        {mark('doc.bind')}\n        self.refresh_history_snapshot().await;\n        {mark('doc.history')}\n        let wg8_r = self.refresh_ui(UiDirtyScope::Full).await;\n        {mark('doc.refresh')}\n        wg8_r\n    }}\n\n    /// 🪪️ The kind identity"),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
