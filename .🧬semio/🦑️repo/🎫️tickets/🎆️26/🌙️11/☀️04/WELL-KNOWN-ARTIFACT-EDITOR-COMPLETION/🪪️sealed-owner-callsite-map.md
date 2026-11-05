# Owner-Scoped Seal Ledger Call Sites

Use exact captured handle identity as the required first parameter. Do not resolve a replacement handle by plugin ID after an await. All paths below are under renderer/engine/elements/ShellHost.

| Source | Current call | Exact handle available |
| --- | --- | --- |
| shell-core-state/🟦️.ts515 | dropForSealedInstance reads ledger | Add required handle parameter to drop API; use it directly.|
| shell-uri-documents/close-document requestProgramRetirement seal closure | ledger.sealed/seal/unseal | owner.handle, captured in closure; preserve it through release.|
| spawned-session/switch-plugin-app53–54 | gate seal/unseal | Capture predecessor handle before gate from loadedPluginsRef and retain it. gate callbacks seal/unseal predecessor; avoid fresh lookup after successor publication.|
| shell-actions/input-funnel167 | drop target before dispatch | plugin variable resolved immediately above at165; pass that exact handle.|
| shell-actions/input-funnel/program-dispatch146 | drop action failure | plugin argument passed into dispatchProgramAction; remains exact through awaited handleAction.|
| host-effects88–89 | empty effect seal and drop | effectOwner.plugin captured by captureEffectOwner. It is nullable; reject missing owner before ledger call. Do not use current loaded handle.|
| host-effects/dispatch-media-and-extension-effects64 | drop extension answer | callback requestingPlugin.handle is exact dispatch owner.|
| same file77 | sealed check in catch | Capture requesterHandle from pluginsNow snapshot before dispatchInvokeExtensionEffect; callback variable is unavailable in catch. Same captured token must be used in both places.|
| plugin-lifecycle/reload-and-extension-callbacks | new forget(owner) cleanup | current.handle after successful old dispose. Keep old seals if disposal fails. New handle has separate ledger and can establish before old disposal without numeric-ID collision.|

Existing late-effect generation fence: hub-port-and-document-backbone captureEffectOwner279 stores exact loaded handle, and isCurrentEffectOwner287–295 compares it with the current loaded handle before accepting effects. Extension answer additionally compares current loaded handle to requestingPlugin.handle. Runtime dispatch queues belong to PluginWasmHandle and retired handles expose isPluginInstanceRetiredV1. These reinforce generation-scoped seals; bare plugin/numeric coordinates cannot replace them.

A handle-scoped ledger permits exact old-generation forget after successful disposal without unsealing the new generation. Fresh actions use their new handle even if plugin ID and instance ID are reused. Retain bounded per-owner keys and fix reseal FIFO duplicates (unseal must remove stale FIFO ownership, or eviction must compare an entry generation).

Tests needing API updates: engine/tests/surface-switch/🟦️.ts; engine/tests/program-retirement/🟦️.ts, host.tsx and callbacks.tsx. New neutral cases: two handles share plugin/id but only old is sealed; forgetting old keeps new seals; old completion is rejected by handle fence; unseal/reseal with capacity1 stays sealed; deferred release retains reservations until drain completes; failure releases only surviving owners.

## Required API Review

Latest root owner-scoped seal/unseal/sealed/forget call sites reviewed. Light TypeScript AST/transpile pass over16affected source/test files reports0syntax diagnostics and0missing required handle arguments for direct ledger/drop calls. This is not a full semantic typecheck.

One concrete unintended substitution: reload-and-extension-callbacks line374 inserted core.sealedInstancesRef.current.forget(current.handle) into useExtensionCallbacks/uninstallExtension, whose parameter list has no core. Root notified to pass the ledger explicitly or remove that accidental occurrence. Reload and uninstallPlugin have core in scope.

All other inspected direct calls pass captured owner handles: close hook owner.handle, switch predecessorHandle, input plugin, action-failure plugin, effectOwner.plugin, extension callback requestingPlugin.handle and captured requesterHandle in catch. Current ledger permits nullable read owner, while writes require object identity. Reload forget follows successful old-handle disposal.
