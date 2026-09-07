# Native UI Patch Marshalling Frontier

## Scope

This audit traces the current native WIT turn result through the retained kernel/shard patch owner and separately traces the authenticated browser actor selection path. It includes a bounded genuine Wasmtime scale-component journey, but does not claim a mounted GIS Shell journey.

## Current actor selection boundary

The browser worker does not accept a fixture path, caller module URL, package id, or digest as actor authority. The live Hub path is:

1. parse the authenticated document-open plan;
2. fetch the server-selected execution-target manifest, component and descriptor;
3. compare every lease field to the plan, verify component SHA-256/BLAKE3 and descriptor SHA-256, and compare the canonical descriptor body to the selected package, surface, artifact kind and renderer;
4. retain the exact lease and admit a browser-actor grant only after the accepted Session actor matches;
5. fetch only `/spaces/{space}/documents/{document}/execution-target/browser-actor` using the retained open intent;
6. verify the actor body length/SHA-256 from the selected lease, load it into the reserved child, invoke `describe`, and compare the result to the retained descriptor;
7. open and acknowledge the exact guest lifetime, transfer the retained checkpoint pair one page per turn, require exact `PageAccepted`/`Applied`, and feed each Shell `UiPatch` acknowledgement or rejection back to the same guest lifetime.

The ticket-owned `qualified-gis-actors/<actor-sha256>` directory produced by the registered GIS describe gate is evidence retention only. No production code consumes it, which is intentional: consuming it would introduce a fixture-selected path outside the trusted Hub generation/current authority. A real Shell acceptance therefore still requires the current GIS closure to be materialized into the normal trusted catalog generation and selected by the Hub plan/body routes.

## Native WIT finding

The comments in the sync and async native hosts that describe a `list<u32>` path versus string path mismatch are stale. The current WIT `ui.patch-op` is node-id addressed and mirrors the canonical `semio_framework_ui_contract::UiPatchOp` variants:

- `upsert` carries packed `UiNodeRecord`;
- component, layout, activity, style, accessibility, bindings and menu carry packed canonical fields;
- children carry `list<node-id>`;
- remove and set-root carry one `node-id`.

The guest reactor already implements the complete canonical kernel-to-WIT direction. Its activity encoder deliberately packs `{activity, disabled}` because the WIT activity record has one packed field. That implementation is the first-party shape oracle for the reverse direction.

The original defect was concrete and bounded:

- sync `WasmtimeRuntime::execute_turn` drains `emit_patch_sink` into an ignored vector and returns an empty `UiTurnPatches`, while preserving a possible patch receipt;
- async `PollTask` drains `take_patches()` into an ignored vector and `convert_poll_success` drops `turn.ui_patches`, while preserving a possible patch receipt;
- consequently a guest could return a nonempty patch plus receipt, but the host handed the shard zero patches plus that receipt, causing fail-closed pairing rejection instead of delivery.

The downstream retention boundary is already implemented. `host/🧵️shard` validates patch/receipt pairing, moves the exact `UiTurnPatches` owner into one generation/session-qualified transport producer, advances bounded work, publishes a one-claim token, and closes the owner on every refusal. OS-run and WGPU consumers already claim that token. Replacing this owner with serialized bytes at the WIT conversion point would weaken the current ownership design and is unnecessary.

## Implemented isolated bridge

The private `plugin/🖥️host/📥️ui-patch` converter is mounted in both native host paths. It:

- decode every packed field through `store::pack_rt::decode_wire_value`, convert through the canonical serde shape, and reject malformed/noncanonical semantic values;
- reconstruct `SurfaceId` from the exact WIT `{instance}:{surface}` identity and reject an invalid canonical UI text;
- enforce the fixed `UiPatchOps` and one-turn `UiTurnPatches` capacities before ownership publication;
- preserve emitted-first ordering, which means current capacity admits at most one patch across both sources and fails closed if both channels are populated;
- preserves the exact activity-plus-disabled packed payload convention already emitted by the guest;
- converts both sync and async paths through the same function;
- checks the patch receipt's instance against the exact host-owned component instance before decoding;
- validates the resulting patch count against `ui_patch_receipt` before returning the turn; every decode is completed into a local bounded owner before publication, so rejection cannot expose a partial patch.

The neutral schema/corpus covers all eleven operation variants and compares the decoded canonical patch to an independently serde-decoded first-party oracle. Hostile laws cover invalid surface/receipt instance identity, byte budget, both emitted and returned channels, maximum-plus-one patch count, receipt without patch, patch without receipt and malformed Pack refusal. The registered source gate also fixes exactly one sync insertion and one async insertion and rejects the former ignored-sink shapes.

## Runtime status

The registered source/schema gate is GREEN with four component cases: `plugin-host-ui-patch-marshalling-source: ajv=1 cases=7 component=4 ops=11 sync=1 async=1 passed`. The focused native gate is GREEN6 at `ui-patch-marshalling-exact/exact-cargo-laws-7H4AGq/00`. All six exact-listed laws ran from `semio_framework_plugin_host-0053a75c880c876b`, SHA-256 `dfbac660f50a6c8eec262da75fb2f98f3e6015387cf5b6ddb0079bd81dd25ace`:

- all eleven WIT variants move into one exact canonical kernel owner and match an independent serde oracle;
- emitted-only and returned-only channels are accepted, the second empty drain is empty, and both-channel/count/receipt mismatches reject atomically;
- wrong patch/receipt instance, byte budget, absent receipt and malformed Pack all reject before publication.
- the current WIT-generated scale actor component builds to 1,022,858 bytes, opens through real Wasmtime, returns exact Captured lifecycle authority, accepts the host ACK, and then publishes either returned root 201 or real `host-async.emit-patch` root 101 with exact instance/surface/patch receipt;
- the native shard bridge moves each genuine component patch into one session-qualified transport token, refuses a duplicate claim, returns one exact owner, and closes that owner boundedly;
- a guest that emits and returns in the same turn is refused atomically on both attempts, with no transport token;
- a first imported noncanonical Pack is refused after the sink is drained; the next genuine guest turn publishes only root 302 at patch sequence 2, proving no malformed/stale sink value leaked into the replacement owner.

Root's DB-only authority native cohort `W0deH7` also compiled the shared host dependency repair and passed four laws. Neither receipt invokes a genuine component that calls the async `emit-patch` import, so that component journey remains pending.

The last sentence above described the earlier GREEN3 receipt and is superseded by GREEN6: the component/import boundary is now qualified. The result still stops at the native shard token. The browser-neutral controlled-child law separately qualifies Shell transaction and ACK/rejection, but there is not yet one mounted browser run joining the genuine component token to a visible Shell surface.

The browser-neutral `MessageChannel` law remains GREEN and proves transactional Applied/rejected feedback using a controlled child. It does not qualify the native WIT converter or genuine GIS execution. The registered GIS source gate is GREEN and can retain a verified closure after native success, but the heavy native producer is still withheld while an existing one-job WASI build and a separate workspace check both compile the same full Stdio graph under severe memory pressure.
