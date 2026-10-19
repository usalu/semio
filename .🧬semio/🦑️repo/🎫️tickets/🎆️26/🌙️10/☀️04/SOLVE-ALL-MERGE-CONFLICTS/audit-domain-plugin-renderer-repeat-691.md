# Independent Domain, Course, Plugin and Renderer Preservation Audit — Repeat 691

Finite read-only source review against base `1bae6115bc56bf5e1b01212774dd76877a5f64fe`, HEAD `36fbe9da786b71ec7b6946a1fc1cbb4e9d4d25ec`, stash `a09b7afdff001185403f0decb7cb0714a7bb839e`. No actionable missing source port, malformed merge scope or absent relative source reference identified in the checks below. Store remains outside this audit.

## Independent Evidence

Read domain-result, plugin-result, renderer-rust-preservation, plugin-runtime-preservation and renderer-tests-preservation reports for repeat 691.

- Immutable-tree content census over changed domain and OS plugin/renderer locators: 4,557 current locators equal the stash blob exactly or preserve its deletion; 97 integrated/relocated; 12 retain HEAD. This census excludes teaching and the Course product. Separately, all 233 changed incoming Course product locators equal the incoming blob or deletion exactly. Teaching census: {'stash-exact/deleted': 189, 'integrated/HEAD': 1}. Equality counts do not classify absent old locators as losses where current owner extraction is intentional.
- Independently lexed all 2,371 existing Rust source locators linked by domain/plugin/renderer-Rust reports. Zero unmatched source delimiters and zero strict merge markers. Lexer skips nested comments, escaped strings, raw strings and character literals. This includes mapped renderer frame/runtime/mailbox/test scopes and plugin staged component owners.
- Independently scanned 926 relative references from renderer PluginRuntime/test `.ts` files and 1,268 from corresponding `.tsx` files. All 2,194 resolve to existing current source paths. This includes canonical actor diagnostic imports, moved typed-operation tests, dialog tutorial inverse helper and lower fixture dependencies.
- Compared newly introduced literal renderer test titles from immutable base/stash changed TS/TSX sources against all current renderer sources: all 23 incoming new cases remain present. This specifically provides independent content evidence for dissolved test-root relocation; tests were not run.
- Inspected current PluginRuntime thin facade and extracted typed-result/channel/invocation/handle-scope owners. Diagnostic token keys retain receiver/operation/generation; inbox receipt is awaited in the async owner accept path; diagnostics route to their owning live call and observer fanout; terminal/fault closes the matching inbox; canonical releaseInstanceMaps deletes the diagnostic fanout.
- Current runtime-singletons configuration retains both originalWorkStock declaration and rendererOriginalWorkStock initialization. The incoming flattened runtime has not replaced current scope/intake/turn/module ownership.
- Moved typed-operation-pages suite imports Node Buffer directly and obtains TYPED_OPERATION_PAGE_MAGIC through its dependency seam; the facade seam exposes that actual canonical actor constant and OperationDiagnosticObservers. Dialog-origin suite directly imports tutorialHistoryStepInverseV1 from its current tutorial owner.

## Scope and Limitations

Blob equality and case-title preservation establish source-content evidence, not complete behavioral equivalence. The 201 equivalent declaration mapping and twelve exact structural transform proofs remain the owning root report's evidence; this audit independently checked resulting owner references, source scopes and diagnostic/test closure rather than claiming a second full AST proof.

No source files, AGENTS files, Git/index/stash state or worktrees were edited. No compilation, build, type checking, tests, dependency resolution, install or application runtime ran. Known authored compiler/API gaps and absent generated build artifacts were not relabeled as preservation failures. This Markdown report is the only authored output.

## Final Focused Renderer Owner Recheck

- Independently extracted immutable incoming FrameOriginalRecovery enum and complete impl blocks and compared them with the final runtime-apply owner. Both are byte-equivalent after only the already established extracted `pub(crate)` to `pub` visibility normalization and newline normalization. No productive incoming body changed.
- The former frame-actions owner contains zero duplicate enum/impl declarations. Current runtime-mailbox mounts runtime-apply and publicly reexports its vocabulary; its existing frame-action imports supply the execution cells while maintenance/deferred owners reside in the same mailbox compilation owner. The inspected frame-actions and host-base manifests have no dependency back to runtime-mailbox, so this relocation introduces no reverse package edge.
- Current mailbox caller names `semio_framework_os_renderer_wgpu_host_base::host_io_request_trace_label` explicitly. Both native and WASM implementations are present behind complementary cfg gates; these are distinct target implementations, not duplicate unconditional functions.
- Current renderer contract source includes runtime-apply as well as the extracted mailbox content. Invocation-frames now explicitly imports TYPED_OPERATION_DIAGNOSTIC from typed-result and OwnedOperationDiagnostic from the canonical controlled-diagnostic owner; both referenced declarations exist in those owners.

No actionable last-change preservation/reference finding. This final recheck used immutable text extraction, source inspection and TOML parsing only; no compiler, typechecker, build, test or application runtime ran.
