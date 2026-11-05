# Folder Persistence Audit — 2026-11-04

Read-only source review of the current MCP folder changes. No Rust edits, builds, or native tests were performed by this auditor. Native test session 70721 was pending when assigned; this report makes no native passing claim. Reviewed folder-persistence-research.md and launch-summary.md.

Paths below are relative to 🧰️framework/🛍️products/💻️os/🔨️modules/.

## Findings

### P1: A Post-Append Durability Failure Can Reopen the Failed Commit

Store 🏪️store/🔄️sync/🖥️native/📥️lifted/🔄️sync/🦀️.rs:222 writes the full record before sync_data at 223 and directory sync at 225–226. Any later error is returned as an ordinary persistence failure, although the complete record can already be readable. Routing 🌉️mcp/🏠️workspace/🔀️routing-and-shell-routed/🦀️.rs:325–327 retains the old binding on failure. The actual adapter 🌉️mcp/🔀️dispatch/🦀️.rs:529–531 correctly attempts TransactionUndo. However, when undo returns the guest to the original pair, routing line 323 treats equality to the retained binding as persistence success without inspecting durable custody. A subsequent reopen can therefore load the failed commit record rather than the undone guest. This is distinct from a pre-write I/O failure. Add a post-write sync failure law and represent/reconcile uncertain persistence explicitly; a successful durable compensating event must follow any complete failed commit record before treating undo as durable.

### P1: Reopen Selects Stale Pair Custody After an Archive Replacement

Store lifted sync file:349–350 searches backwards only for DOCUMENT_PUT_EVENT; read_archive at 364–365 independently searches only for DOCUMENT_ARCHIVE_PUT_EVENT. A pair P followed by archive A makes read return P, while write_if_current correctly finds A at 204–208 and rejects P as stale. MCP explicit folder reopen 🌉️mcp/🏠️workspace/🛂️gateway-backend/🏗️open-and-discovery/🦀️.rs:163–166 calls read, so reopening does not resolve this conflict and silently loads obsolete custody. Select the latest event across both formats and expose its format/identity; refuse unsupported current format rather than returning an older one. A pair/archive/pair neutral law should verify both readers and CAS. The owned archive projection is semio_framework_os_spr::decode_document_archive_bytes, returning parent_pack and parent_spr (existing use: native actor 📁️actor-folder-persistence-and-watcher/🦀️.rs:277–282). Projection alone must not discard archive identity needed by CAS.

### P2: Plugin-Only Shell Read Preempts the Exact Alias

MCP open-and-discovery file:178–183 invokes read_session_artifact_bytes(binding.plugin_id) before read_live_session_artifact_bytes(artifact_id, plugin_id, app_id). The former in gateway-backend/🔌️plugin-activation-and-bindings/🦀️.rs:142–150 creates a ShellDirect channel for_plugin and reads instance 0 without artifact/app identity. In a shell-attached folder workspace with several artifacts or editor apps of one plugin this can supply an unrelated canonical pair. Route the shell read through exact artifact/plugin/app identity or refuse an ambiguous selection; test two same-plugin artifacts with distinct pairs.

## Verified Source Properties and Limits

All event appends use append_checked. Its shared in-process writer mutex is acquired at store line197 and the opened event file takes std File.try_lock at202 before reading the expected baseline, scanning/truncating tail, writing and syncing. This is a real OS file lock, not merely a process-local mutex; this review did not execute a two-process contention law. Ordinary reads do not take the OS lock; they may read the preceding complete prefix during a concurrent partial append. CAS itself reads from the locked handle and compares schema plus both bytes, and rejects a latest archive event. Both bytes are encoded into one checksummed record.

Declared ownership is resolved by exact schema plus unique plugin/app pair in gateway-backend/🔌️plugin-activation-and-bindings/🦀️.rs:204–215; missing or ambiguous owners fail. Folder reopen refreshes the shared binding at220–229. Routing compares seeded pair at431–441 and reseeds a selected guest when the rebound pair changed. Commit persistence failures preserve TransactionCommitted with an unacknowledged outcome at routing600–607, enabling the actual undo branch in dispatch529–531. This is valid compensation wiring, but the P1 post-append case above remains unverified and incorrect by inspection. Undo and redo persistence failures otherwise return a fault, without fabricated success.

All findings were communicated promptly to the root and launch_execution agents. No generated output files or copied source assets were created.
