# WS-D — Plugin Contract (framework/plugin)

## Scope
- framework/plugin/rs/lib.rs: two-layer contract (DocumentApp typed / PluginApp runtime), VcsDocumentApp wrapper, generation-op vocabulary, TestApp.
- framework/wit/world.wit: new plugin exports + host backbone-status import.
- framework/plugin/rs component module: wasm WIT glue for new exports.
- framework/plugin/host/rs/lib.rs: native host — backbone registry placeholder (resolve_backbone gone), backbone-status import impl, caller methods for new exports.
- framework/core/rs/lib.rs: HostEffect variants + IconRenderExportItem. (DONE)
- framework/core/js/index.ts: typed ActionResponse, delete patchOpsFromActionResponse, new handle methods.

## Env blocker
Concurrent mathematical-crates refactor left a dangling path-dep in framework/graph/rs/Cargo.toml
(-> deleted mathematical/graph/port/directed/dag/rs). Blocks ALL cargo manifest loading.
Verification via temporary stub of the missing crate, removed afterward.
