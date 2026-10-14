# Current Enduser Integration Audit

Read-only source audit on 2026-10-14. No build, test, cache mutation, production edit, or runtime qualification performed. Reuses the assigned existing ticket; ticket/goals lifecycle belongs to Root. Root-provided native evidence is prior evidence only.

## Fresh Source Findings

- Actual guest export remains consuming: `plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs:30` moves `events` through `dispatch_value_step` into `reactor::poll`; owned export at line40 moves `input.events`. `plugin/⚛️reactor/🦀️.rs:1268-1281` receives `Vec<WIT Event>`, allocates `Vec::with_capacity(events.len())`, then consumes each event through `wit_event_to_kernel`. No original resident event owner is exposed by this poll signature. The actual pre-lift admission/custody seam is therefore still prerequisite to paid mesh Message input; a result-held source alone cannot return refused input.
- Existing host source authority is concrete: `plugin/🖥️host/🧵️shard/🦀️.rs:1754` places the same taken source Event in `OriginalShardEvent::new`; `:1984-1986` forwards raw original-call refusal while retaining original source/failure. Preserve this host owner, rather than substituting a separately decoded event.
- Generation3d still has the cold actual render route: `procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2765` calls scratch-session `generation3d_render_body`; native render signature at `:2779` has `_cx`. The prior transport/custody diaries identify the same editor/viewer preview payload route and remain the detailed handoff.
- World changed since the old MeshData description: `os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:9638-9640` now has `WorldMeshSource::{Placeholder, Inline(Box<WorldMeshBuffers>)}`. It still has no Native variant. Actual bridge invokes `WorldPlaceholderMeshCursor::inline` at `:13328`; cursor construction boxes buffers at `:9736`. Do not state current code still boxes MeshData.
- Actual native renderer `os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:9348-9407` applies turn patches, stores receipt in owner, and routes Shell SendMessage at `:9357`. Existing typed-result ack messages are emitted at `:9331`. A source scan of this defining renderer finds no `PatchAck` emitter. Stored `patch_receipt` at `:6768/:9407` is not final guest resource acknowledgement.
- Reactor ack machinery exists: `plugin/⚛️reactor/📨️pending/🦀️.rs:381` pins receipt/surface/revision, and `⚛️reactor/🔄️turn/🦀️.rs:1010` consumes `Event::PatchAck`. Thus the missing step is the genuine native receiver completion emitter and custody, not a new ack family.

Paths above are repository-relative prefixes under `🧰️framework/🛍️products/💻️os/🔨️modules` unless explicitly procedural, which is under `✏️s/🔌️plugins/🌀️procedural`.

## Coherent Next Execution Scope

1. Fund canonical lifting/reservation in the SAME existing resident actor Entry before consuming/lifting original incoming Event. Preserve host OriginalShardEvent and exact supplied original context/currency receipts; refusal must return full same source, and partial/cancel paths must close through its paid authority.
2. Add bounded scalar descriptor/geometry/component-label request/page protocol through existing Message/SendMessage and published ReadySlot. Use the already authored `with_published_native_resource_loans_on_original` callback and `describe_mesh_on_original/read_mesh_chunk_on_original`; no guest arena token can identify host pages.
3. Retain pages and original result in existing native surface receiving owner until actual host admission, original prepared-frame upload custody, and guest loans return. Only then emit exact existing PatchAck. Mount original receiving pages into typed World native source and wire E/V render to guarded current cache source.

## Compiler And Acceptance Floors

Root reports its last native compiler found nine Plugin errors and that source corrections landed, followed by ENOSPC before a new compiler launched. This audit ran no compiler and cannot certify those fixes. Next selected canonical Bun+Nx native gate must first compile fresh Plugin/TestKit/history/SDK chain. Prior native owner/page reader laws are local prerequisites; they do not qualify nonempty labels, cross-ABI receiver, Scene/GPU or browser.

After the admission seam, require genuine finite original caller tests with allocation/System accounting, zero-work denial, cancellation/partial close, same input return, full original failure, label/page admission and exact ack. Full arbitrary BRep/mesh create/edit/analysis via mutation+inference, Undo/Redo and English/German browser behavior remains unqualified.
