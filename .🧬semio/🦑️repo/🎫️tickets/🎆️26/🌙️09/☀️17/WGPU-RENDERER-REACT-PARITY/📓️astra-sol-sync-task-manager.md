# 🔄️ Detached Sync Attach and Task Manager Shell Packet

## Scope and reference

This packet implements the two current-source omissions proved by `📓️astra-terra-current-runtime-audit.md`: the detached Sync Attach leaf/body and the Task Manager leaf/body/command. It does not alter dock geometry, pointer cancellation, scene transfer, scheduling, or actor-runtime architecture.

The React reference is:

- `💻️os/🟦️.ts::buildFrameworkSyncUtilities`, which always returns File, Folder, and Remote, including for `null`.
- `🔄️ShellSync/🟦️.tsx::SyncAttachCard`, which exposes the selected draft field, file/folder browse, Attach, and attached-state Detach.
- `🏛️ShellHost/🟦️.tsx`, which always declares `s-sync-status`, routes attach through `openDocument`, routes detach through `closeDocument`, declares `os.task-manager` at bottom-right, and handles `os.openTaskManager`.
- `🧵️TaskManager/🟦️.tsx`, which distinguishes `no-runtime` from `no-actors` and supplies actor actions only when an `ActivationRegistry` exists.

## Shared contract

Added target-neutral schema and fixture:

- `🧑‍🎨engine/🧬️schema/🔄️shell-utility-leaves/🔣️.json`
- `🧑‍🎨engine/🧫️fixtures/🔄️shell-utility-leaves/🔣️.json`

The fixture freezes:

- detached Sync at `bottom-left` with label `Remote: detached` / `Remote: getrennt`;
- exact choice ids `framework.sync.file`, `.folder`, `.remote`;
- attached URI selection and Detach availability;
- Task Manager at `bottom-right`, command `os.openTaskManager`, state `no-runtime`, exact EN/DE copy, and no actor actions.

## WGPU implementation

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` now:

- adds `s-sync-status` unconditionally for every mounted session;
- derives its tab label from the existing live `sync_pill()` projection, so detached cold state is `Remote: detached` rather than generic `Sync`;
- publishes a retained Sync body with all three real `framework.sync` controller choices;
- publishes selected kind, draft input, file/folder browse, Attach, current URI, and attached-state Detach;
- accepts retained input commits in the interpreter's canonical `{ value }` shape while retaining the existing picker `{ path }` shape;
- closes the retained ownership set by adding Sync to `shell_owned_panel_leaves` and `publish_shell_panel_document`;
- adds `os.task-manager` after Marketplace in the bottom-right order and places History after it;
- registers and routes `os.openTaskManager` through `sync_dock_tabs` + `reveal_dock_tab`;
- publishes the exact unattached-runtime text through a retained `os.task-manager.no-runtime` node;
- deliberately publishes no actor rows or suspend/resume/cancel actions because this renderer has no actor-registry metrics/dispatch bridge;
- closes failed native Sync Attach admission by retiring the just-opened `ArtifactHost` document key before returning the bridge error.

The existing shell-owned closure law was updated for Task Manager order and both new body branches.

## Backend reachability

The retained controls target real existing action paths, but complete Sync backend parity is not present and this packet does not claim it.

Native WGPU `attach_sync_backbone` opens and subscribes an `ArtifactHost`, then calls `ProgramBridgeEntry::attach_backbone`. Its WASM backend delegates to `wasm_program_exchange::attach_backbone`, whose exact result is the intentional error:

`attach_backbone: retired in channel v12 — backbone is now event-driven (design-abi.md §2/§4); no EffectBackbone replacement has landed yet`

The production repair in this packet closes the newly admitted document key on that error, so a reachable Attach control cannot leak an actor. Native Detach still sends `ArtifactActorMsg::Detach`, calls the retired guest detach best-effort, closes the document key, clears status/presence, and removes the URI. The retired guest error is ignored because local teardown remains authoritative.

The WGPU browser branch currently records the URI locally and has no equivalent document-open bridge in this function. React instead parses `remote://host/space/document`, derives persistence bindings, calls its canonical `openDocument`, and uses `closeDocument` on Detach. An external owner must implement the event-driven `EffectBackbone` replacement and browser document-open bridge before end-to-end Sync Attach parity can be claimed; restoring the removed channel commands would be a compatibility path and was not done.

## Laws and validation

Added:

- `🧑‍🎨engine/🧪️tests/🔄️shell-utility-leaves/🟦️.tsx`
- `🐚️Shell/🧪️tests/🔄️detached-sync-task-manager/🦀️.rs`

The TypeScript suite validates the neutral schema, an independent URI-kind oracle, React's actual Sync producer, React's mounted `TaskManagerWindow registry={null}` oracle, and WGPU source ownership. It is registered in the existing React test configuration.

Red observation before production:

- four neutral/React laws passed;
- WGPU ownership failed because `FRAMEWORK_TASK_MANAGER_PANEL_ID`, both builders, command routing, and leaf closure were absent.

Focused post-repair command:

`SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache -- '../../../../🧪️tests/🔄️shell-utility-leaves/🟦️.tsx' --run --reporter=dot --silent=true`

Result: **1 file, 5 tests passed**.

The Rust laws cover cold detached dock/body identity, all three choices, retained `{ value }` draft routing, exact Attach/Detach descriptors, local detach state, Task Manager order/body/no-actions, command reveal, and retained-document projection. They were added but not run here because root owns native/Cargo builds. No runtime behavior was claimed.
