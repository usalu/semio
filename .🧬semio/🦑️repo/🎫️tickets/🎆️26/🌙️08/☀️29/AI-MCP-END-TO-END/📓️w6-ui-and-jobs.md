# W6 — UI information, UI actions, and jobs

## File

`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🖥️ui/🦀️component.rs` (new, ~975 lines incl. 24 tests).
No other file touched — glue.rs mounting (`#[path = "../../🖥️ui/🦀️component.rs"] pub mod ui;`) and
`DECLARED_STUB_TOOL_NAMES` removal for these four names are W0's job.

## `ui_focus`/`ui_reveal` → `ShellCommand` mapping

Both are forwarded as opaque JSON bytes inside `GatewayToShell::ShellCommand{seq, command}` — I did
**not** take a new compile-time dependency on `semio-framework-os-shell`'s Rust `ShellCommand` enum
(the mcp crate's `Cargo.toml` does not currently depend on that crate, and editing `Cargo.toml`
felt like root-adjacent territory I was told to avoid). Instead I mirror `🧵️bridge`'s own documented
design ("`state`/`command` bytes are opaque, AgentBridge encodes/decodes them as JSON on the
renderer side") and build the exact same JSON shapes `🖥️shell/🦀️component.rs`'s own
`#[serde(tag="type", rename_all="camelCase")]` would produce, verified against its literal
TypeScript mirror string:

- `ui_focus` → `ShellCommand::FocusWindow{window_id}` → `{"type":"focusWindow","windowId":<string
  or null>}`. Input `{windowId?: string}` — omitted key means `null` (clears focus); no nullable-type
  schema needed.
- `ui_reveal` → **two** commands, both awaited in sequence: `SetPanelVisible{anchor,visible:true}`
  then `SetPanelPath{anchor,path}` — `ShellState` has no single "reveal" command; the closest
  existing pair is making the panel visible and navigating its path, which is exactly what
  "reveal an item in a panel" means. Input `{anchor: "left"|"right"|"top"|"bottom", path: string[]}`,
  both required.

If real typed access is ever wanted instead of hand-built JSON, the exact dependency to add is
`semio-framework-os-shell` (path `../../../../../../🔨️modules/🖥️shell/📦️packages/🦀️rust`) — I
deliberately left this to whoever owns `Cargo.toml`.

## Bridge API used — nothing added

Everything needed already exists as `pub`/`pub(crate)` on `BridgeHandle` (same crate, so
`pub(crate)` `register`/`unregister`/`record` are reachable from my tests too):
`connections()`, `send_to(id, GatewayToShell)`, `last_command_result(id)`,
`last_shell_state(id) -> Option<ShellToGateway>`. My own `dispatch_shell_command_with_timeout`
picks the highest (most-recently-registered) `ShellConnectionId` as "the" attached shell (bridge has
no explicit primary-shell concept yet), sends, then **polls** `last_command_result` for a matching
`in_reply_to` up to `SHELL_COMMAND_TIMEOUT_MS` (4000ms, 20ms poll) — tool handlers in this crate are
synchronous (`Fn(Value) -> CallToolResult`), so a bounded poll loop is the only option without
changing `ToolRegistry`'s trait shape. A stale/unrelated result already sitting on the connection
can never be mistaken for this call's reply (`in_reply_to` must match the freshly-minted `seq`).
`ShellStatePatch`-only connections (no full snapshot yet) degrade to a typed, retryable
`PLUGIN_UNAVAILABLE` rather than guessing a merge — merging a patch onto a base state is `🖥️shell`'s
reducer's job per `🧵️bridge`'s own doc comment, and no merge helper exists yet.

## The job seam — reconciled with W5's report

`crate::handles::HandleTable`'s `HandleKind::Job` gives session-owned, expiring, mint/resolve/
`mark_terminal` handles — but **no way to mutate a handle's payload after minting**, so it cannot
carry real progress (`job_get` must report progress, not just terminal/not-terminal). I built a
companion `JobRegistry` (own file, process-wide singleton via `pub fn job_registry() ->
&'static JobRegistry`) that a producer mints into directly:

```rust
pub fn job_registry() -> &'static JobRegistry;
impl JobRegistry {
    pub fn begin(&self, kind: &str) -> String;                                    // mints via crate::handles::mint_id(HandleKind::Job, …) — same "job_" id scheme
    pub fn begin_with_id(&self, job_id: impl Into<String>, kind: &str) -> String;  // for a producer that ALSO mints a HandleTable entry and wants matching progress tracking under the SAME id
    pub fn report_progress(&self, job_id: &str, progress: f64, message: Option<String>) -> bool;
    pub fn succeed(&self, job_id: &str, result: serde_json::Value) -> bool;
    pub fn fail(&self, job_id: &str, error: GatewayError) -> bool;
    pub fn is_cancel_requested(&self, job_id: &str) -> bool;   // cooperative — producer's own work loop polls this
    pub fn mark_cancelled(&self, job_id: &str) -> bool;        // producer calls once it has actually stopped
    pub fn request_cancel(&self, job_id: &str) -> Result<JobSnapshot, GatewayError>; // job_cancel's real effect
    pub fn snapshot(&self, job_id: &str) -> Option<JobSnapshot>;
}
```

Cancellation is real but cooperative: a `Pending` job (nothing running yet) finishes as `Cancelled`
immediately; a `Running` job only gets `cancel_requested=true` until its own producer observes
`is_cancel_requested` and calls `mark_cancelled` — this facet never force-kills work it doesn't own.
`job_get`/`job_cancel` work identically in every tier (bare/headless/attached) since they depend only
on the process-wide registry, never on `bridge`/`workspace`; an unknown id is `NOT_FOUND` in every
tier — honestly correct, since no job can exist without a producer, and none exists in bare tier.

**For W5**: mint via `job_registry().begin("inference.<inferenceSchema>")` (or `begin_with_id` if you
also mint a `HandleTable` entry for session ownership), call `report_progress`/`succeed`/`fail` from
wherever the execution actually runs, and `job_get`/`semio://job/{id}` will answer immediately —
zero further wiring needed on my side.

## Resources

`semio://window` (list) / `semio://window/{windowId}` (template) — windows + panel (`left/right/
top/bottom`: visible/size/path) inventory, projected from the SAME `ShellState` JSON `BridgeHandle`
mirrors. `semio://ui/active-context` — activeWindowId/activeToolId/activeUtilityByWindow/
activeExampleId/openWithFocusRole/activeTutorialId/uiLocale/uiThemeId/revision. `semio://ui/selection`
— **honesty note**: `ShellState` has exactly one selection-shaped field, `selectedConflictId` (merge
conflict preview); per-artifact object selection (e.g. a CAD scene's selected entities) is
app-instance state carried over `AppFrames`/`Instances`, not shell state, so it is **not** projected
here (never a second, invented source of truth) — the resource returns `selectedConflictId` +
`activeWindowId`/`activeToolId` as the closest real "what's focused" signals that exist.
`semio://job/{jobId}` (template) reads `job_registry()`.

`ui_resources`/`read_ui_resource` match the given contract exactly. I added one function beyond it,
`ui_resource_templates() -> Vec<ResourceTemplate>`, because `semio://window/{windowId}` and
`semio://job/{jobId}` are parametrized and the given contract had nowhere to list a
`ResourceTemplate` (only `ui_resources -> Vec<Resource>`) — W0 needs to fold this into
`resources/templates/list` alongside `ui_resources` into `resources/list`.

## Exact signatures (for root wiring)

```rust
pub fn ui_capabilities() -> Vec<CapabilityDefinition>;
pub fn register_ui_tools(registry: &mut InMemoryToolRegistry, bridge: Option<std::sync::Arc<BridgeHandle>>, workspace: Option<std::sync::Arc<HeadlessWorkspace>>);
pub fn ui_resources(bridge: Option<&std::sync::Arc<BridgeHandle>>) -> Vec<Resource>;
pub fn ui_resource_templates() -> Vec<ResourceTemplate>;
pub fn read_ui_resource(uri: &str, bridge: Option<&std::sync::Arc<BridgeHandle>>, workspace: Option<&std::sync::Arc<HeadlessWorkspace>>) -> Option<Result<Vec<ResourceContent>, GatewayError>>;
pub fn job_registry() -> &'static JobRegistry;
```

`register_ui_tools`'s `workspace`/`read_ui_resource`'s `workspace` parameters are accepted but
currently unread (`_workspace`) — today's UI projections read only the bridge's `ShellState` mirror
(design requirement #1), and jobs are workspace-independent by design (see above). Kept in the
signature for symmetry and so a later packet can layer workspace-scoped concerns in without a
signature change.

## Per-tier behaviour

- **Bare** (`bridge: None`) — `ui_focus`/`ui_reveal` return retryable `PLUGIN_UNAVAILABLE` naming
  "run under `http` transport". `semio://window`/`active-context`/`selection` reads degrade the same
  way. `job_get`/`job_cancel` unaffected (registry-only).
- **Headless-equivalent for UI** (`bridge: Some(handle)`, zero live connections) —
  `no_shell_attached_error()`: retryable `PLUGIN_UNAVAILABLE`, wording states this is expected,
  retry once a shell connects.
- **Attached** (a live connection) — real `ShellCommand` round trip / real `ShellState` projection.

Tool/resource **presence** never varies by tier — verified by `all_four_tools_register_under_valid_
mcp_names_with_object_top_level_schemas` and `ui_resources_and_templates_never_depend_on_bridge_
presence`.

## Verified vs written-but-unverified

**Not run** — `cargo check -p semio-framework-os-mcp` would not exercise this file at all today
(it is not yet mounted into `📦️glue.rs`; that is W0's job), and the shared tree's
`semio-framework-plugin-host` dependency was reported broken by another agent's in-flight wave for
the duration of this session, so I did not spend the two permitted `cargo check` calls on a check
that could not touch my own file. I did not run `cargo test`; no test's pass/fail is claimed.

**Verified by direct source reading** (every signature/field cross-checked against the real
definitions, not guessed): `BridgeHandle`/`GatewayToShell`/`ShellToGateway`/`ShellConnectionId`
(`🧵️bridge/🦀️component.rs`, full read of the `BridgeHandle` impl block + relevant tests proving
`pub(crate) register/record` are usable from sync `#[test]`s without a websocket or tokio runtime);
`ShellCommand`'s exact `camelCase` wire tags (`🖥️shell/🦀️component.rs`'s own TypeScript-mirror
string, read verbatim — confirms `focusWindow`/`setPanelVisible`/`setPanelPath` field names);
`Tool`/`CallToolResult`/`ToolRegistry`/`ResourceRegistry`/`Resource`/`ResourceTemplate`/
`ResourceContent` (`🧭️protocol/🦀️component.rs`); `GatewayError`/`GatewayErrorCode`
(`⚠️errors/🦀️component.rs`); `CapabilityDefinition`/`CapabilityKind::{Ui,Job}`/`ToolExposure`
(`🗂️catalog/🦀️component.rs`); `mint_id`/`HandleKind::Job` (`🎫️handles/🦀️component.rs`);
`WorkspaceResourceRegistry`'s list/read conventions (`🧠️context/🦀️component.rs`, mirrored, not
copied); confirmed `serde_json::json!`'s macro source (`to_value(&$other)`) borrows rather than
moves interpolated values, so reusing `anchor`/`path`/`window_id` across multiple `json!` calls in
one handler is safe.

**Written and self-consistent, not run**: the file as a whole, all 24 tests in `mod quick` — tool
registration + schema shape, bare/headless-equivalent `PLUGIN_UNAVAILABLE` for `ui_focus`/`ui_reveal`,
input validation, resource list/template stability across bridge presence, `read_ui_resource`'s
`None`/degraded-error cases, a real (non-websocket, in-process `BridgeHandle`) timeout test, a real
success-reply test and a real fault-reply test (both drive `bridge.record` from a spawned thread),
and the full `JobRegistry` lifecycle (begin/begin_with_id/progress/succeed/fail/cancel-pending/
cancel-running-cooperative/cancel-unknown/cancel-terminal) plus `job_get`/`job_cancel` tool
round-trips through the shared registry.
