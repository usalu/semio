# OS-Kernel R17: Source-Owned Repair Proposal

Read-only proposal after the actual WGPU compile failed before tests. No production source, feature, runner, or test expectation was changed here. The complete actual compiler JSON is in 📓️wgpu-single-enqueue-r17-retained-compiler-diagnostics-2026-08-28.md.

## Exact Causes And Ownership

1. Directory/client native module481 imports TokioHostRuntime from semio_framework_async, but the concrete implementation is in semio_framework_os_services (services341). The same native module already imports services transport types. Its existing cfg requires native+sync+ureq; WGPU already requests sync+ureq. Move only that concrete import to its actual services owner, keeping HostAsyncRuntime/HostFuture/OperationContext/ScopeHandle in async. Do not introduce an async re-export, alternate runtime, new pool, feature toggle or compatibility path. A services typed compile assertion and the existing injected-pool identity law can validate the selected implementation.

2. SyncSession::detach900 awaits ArtifactStore::detach_backbone even though the actual store15687 now returns Result<Option<Backbones>,VcsError> synchronously. Removing await and discarding the Result is not a repair: the returned backbone owns channels/messages and must remain retained through retirement. The current method also sends Detach before possible store refusal and unconditionally clears its command/event owners.

   Proposed owned transaction: the existing Store owner prepares its exact displaced-backbone retirement destination before detachment; then perform the fallible revision/descriptor transition and transfer the original backbone into that already-installed owner. SyncSession changes command/event state and sends its actor detach request only after successful store admission/transition, with each owned channel receiver still structurally retained for its close phase. Busy/refusal/error leaves the original store backbone, session channels and unsent request available. Use the same Store displaced-retirement domain, not a second generic spill queue.

   The existing private replace_backbone_retained14177 is a reuse point but not yet a sufficient bounded primitive: it allocates Box<ArtifactStoreBackboneRetirement> after moving the previous backbone. Any adoption must precreate/admit the empty typed retirement shell and target slot before taking the source, and test construction failure/unwind. The current detach_backbone also clears the envelope descriptor before fallible bump; exact refusal conservation needs a test and staged commit ordering, not merely a new call syntax. Mutation owns Store, so this packet requires their explicit source coordination. No whole-backbone generic Drop is acceptable cleanup.

3–4. The native actor retains ActorTurnFuture: Future+Send at sync1953 and polls it through the worker pool at2245. FolderEndpoint::read1263 awaits codec.compile_dsl; write1277 awaits codec.print_mirror. Both ArtifactCodec function-pointer fields (Store9103/9108) erase their return values to dyn Future without Send. That erased contract, not the worker requirement, is the immediate compiler cause.

   Preserve ActorTurnFuture+Send. Add compile-time laws on the two actual registered codec futures first. Give those canonical codec slots and their monomorphized implementations the truthful Send return contract, then propagate only the concrete P/Mutation bounds needed by their retained borrows. ArtifactCodec::of already requires P:Send+Sync and Mutation:Send, but nested thunks omit these bounds and borrowed envelopes may additionally require Mutation:Sync across their print awaits. Confirm this with compiler diagnostics and coordinate the authored registration call sites; do not assert Send unsafely or hide a non-Send future behind block_on, spawn_local, resolve_ready or a new thread/pool.

   The remaining edit_text/apply_ops codec slots also erase futures without Send; inventory their actual native consumers before broadening their contract. Do not silently create a host-only compatibility registry. Codec byte work remains whole-operation work today; establishing Send does not certify bounded serialization, allocation or cancellation.

## Test-First Matrix

| Packet | New exact law / neutral contract | Existing regression / actual execution boundary |
| --- | --- | --- |
| Concrete runtime import | Type identity binds directory transport to the services-owned runtime using the original injected pool; no pool creation | tokio_host_runtime_with_pool_never_resizes_the_injected_pool; same native WGPU dependency build |
| Retained detach | Neutral attached→prepared→detached→retiring states; zero/short grant, occupied destination, revision overflow, injected construction failure and unwind preserve original pointer/queue contents; exact one detach request after successful transition | backbone_retirement_blocks_for_live_peer_then_drains_one_owned_message_or_byte_grant plus SyncSession receive laws after fixture coherence |
| Codec Send | assert_send on the actual compile_dsl/print_mirror returned futures (current erased types are compile-RED); schema-derived DSL/ops/pack/spr roundtrip compared through existing independent serde/byte oracles | document_codecs_share_complete_authoritative_history_validation and existing folder text/pack roundtrips; actual native ActorTurnFuture must still compile as Send |
| End-to-end compiler gate | Same declared WGPU native graph, no disabled sync/ureq features | Original runtime_single_enqueue_reader_cannot_observe_completion_without_its_scene_invalidation must finally execute; expected semantic RED remains unchanged |

The test names in the new-law column are proposed scope, not mounted/executed tests. Existing test names were read from source, not rerun or credited as passing. The native observer packet's separate success does not change these compiler or ownership defects. Root review/owner assignment is required before edits.
