import json,sys
V="️"
F="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs"
def child(e,name,text): return {"dir":e+V+name,"doc":"//! %s%s %s See [`super`]."%(e,V,text)}
g={
 "doc":child("🎯","document-and-child-commands","Document and child commands: local interaction queries, context menus, envelope ingestion, composed children and the history read of one plugin exchange."),
 "arc":child("📦","document-archive-commands","Document archive commands: archive load, merge, poll, cancel, acknowledge and the identity and archive reads of one plugin exchange."),
 "cfg":child("⚙","config-commands","Config commands: config load and read and the window config load and read of one plugin exchange."),
 "med":child("🎞","media-commands","Media commands: media import, export submission, poll, cancel and fingerprint of one plugin exchange."),
 "txn":child("🔁","transaction-commands","Transaction commands: prepare, commit, rollback, undo and redo of one plugin exchange."),
 "rel":child("🌐","relay-and-conflict-commands","Relay and conflict commands: artifact open, default app, merge policy and conflict resolution of one plugin exchange."),
}
rows=[
 (281,"arm_local_interaction_query","doc","🔎","Reads, acknowledges or cancels a local interaction query and answers its receipt or rejection."),
 (313,"arm_context_menu","doc","📋","Decodes a context menu request and answers its items."),
 (327,"arm_apply_envelopes","doc","📨","Ingests remote envelopes, reports the merges and publishes the open conflicts."),
 (352,"arm_load_children","doc","🧸","Loads the composed children of a parent document."),
 (372,"arm_read_children","doc","👶","Reads the composed children of a parent document."),
 (382,"arm_read_child_heads","doc","🧬","Reads the head of every composed child."),
 (465,"arm_read_history","doc","📜","Reads the retained history of the document."),
 (392,"arm_load_document_archive","arc","📥","Starts loading a document archive."),
 (402,"arm_merge_document_archive","arc","🧩","Starts merging a document archive."),
 (412,"arm_poll_document_archive_load","arc","📊","Polls the progress of an archive load."),
 (425,"arm_cancel_document_archive_load","arc","🛑","Cancels an archive load."),
 (435,"arm_acknowledge_document_archive_load","arc","✅","Acknowledges a finished archive load."),
 (445,"arm_read_document_identity","arc","🪪","Reads the document identity."),
 (455,"arm_read_document_archive","arc","🗄","Reads the document archive."),
 (475,"arm_load_config","cfg","🔧","Loads the config pack and spr."),
 (485,"arm_read_config","cfg","📖","Reads the config pack, spr and ops."),
 (495,"arm_load_window_config","cfg","🪟","Loads one window config pack entry."),
 (512,"arm_read_window_configs","cfg","🗂","Reads every window config pack entry."),
 (528,"arm_media_in","med","🎙","Imports a media port payload."),
 (541,"arm_submit_media_export","med","📤","Answers a submitted media export handle."),
 (545,"arm_poll_media_export","med","⏳","Answers the poll status of a media export."),
 (582,"arm_cancel_media_export","med","❌","Cancels a media export."),
 (590,"arm_media_fingerprint","med","🖐","Answers the fingerprint of a media port."),
 (604,"arm_transaction_prepare","txn","🧾","Prepares a transaction with its operations and origin."),
 (631,"arm_transaction_commit","txn","💾","Commits a prepared transaction."),
 (646,"arm_transaction_rollback","txn","↩","Rolls a prepared transaction back."),
 (659,"arm_transaction_undo","txn","⏪","Undoes a transaction group."),
 (672,"arm_transaction_redo","txn","⏩","Redoes a transaction group."),
 (685,"arm_open_artifact","rel","🚪","Relays an artifact open request."),
 (692,"arm_set_default_app","rel","📌","Relays a default app assignment."),
 (699,"arm_clear_default_app","rel","🧹","Relays a default app removal."),
 (706,"arm_set_merge_policy","rel","⚖","Sets the merge policy."),
 (716,"arm_resolve_conflict","rel","🤝","Resolves one open conflict."),
 (726,"arm_read_conflicts","rel","⚔","Reads the open conflicts."),
]
used=set()
arms=[]
for line,name,gr,em,doc in rows:
    assert em not in used,em; used.add(em)
    arms.append({"line":line,"name":name,"doc":em+V+" "+doc,"child":g[gr],"allow_returns":True,"binds":{}})
types={
 "runtime":"&PluginRuntime<PA>","instance_id":"u32",
 "identity":"&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>",
 "cx":"&mut semio_framework_job::StepContext<'_>",
 "frames":"&mut Vec<protocol::AppFrame>","effect_bytes":"&mut Vec<Vec<u8>>","event_bytes":"&mut Vec<Vec<u8>>",
 "flags":"&mut ExchangeFlags","envelope_seq":"u64",
 "owner":"&mut semio_framework_plugin_runtime_ops::ops_ingress::OriginalDecodedPluginCommand",
}
spec={"file":F,"function":"plugin_exchange","match_line":279,"rematch":True,"async":True,"generics":"<PA: PluginApp>","generics_if":"runtime",
 "context":["runtime","instance_id","identity","cx","frames","effect_bytes","event_bytes","flags","envelope_seq","owner"],
 "types":types,"ret":"Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>","ensure_semicolon":True,"fall_tail":"Ok(())","call_suffix":".await?",
 "argexpr":{"frames":"&mut frames","effect_bytes":"&mut effect_bytes","event_bytes":"&mut event_bytes","flags":"&mut flags","owner":"&mut owner"},"arms":arms,"allow_reason":"one exchange context plus the command payload"}
json.dump(spec,open(sys.argv[1],'w',encoding='utf8'),ensure_ascii=False,indent=1)
