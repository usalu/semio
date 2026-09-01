# W4 — real `artifact_*` tools

New, self-contained facet: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗿️artifact/🦀️component.rs`.
No other file was touched (root `🦀️component.rs`, `📦️glue.rs`, `🏠️workspace/`, `🧠️context/`, and every
other facet are untouched — root wiring is explicitly out of scope for this package).

## Public API (exact signatures)

```rust
pub fn artifact_capabilities() -> Vec<CapabilityDefinition>;

pub fn register_artifact_tools(
    registry: &mut InMemoryToolRegistry,
    workspace: Option<std::sync::Arc<HeadlessWorkspace>>,
);
```

Both match the contract verbatim. `artifact_capabilities()` returns 5 `CapabilityDefinition`s
(`artifact.open`, `artifact.create`, `artifact.validate`, `artifact.snapshot`, `artifact.export`),
`owner: CapabilityOwner::Gateway`, `exposure: ToolExposure::Direct{tool_name: "artifact_open"}` etc.
— modeled directly on `🦀️component.rs`'s `core_tool_capabilities()`/`capabilities_search_capability()`.
Root wiring only needs to (a) fold `artifact_capabilities()` into `CatalogSource.gateway` next to
`core_tool_capabilities()` in `build_catalog()`, and (b) call `register_artifact_tools(&mut tools,
workspace_option)` in `build_tool_registry`/`build_server_with_workspace`, removing these 5 names from
`DECLARED_STUB_TOOL_NAMES` (now `["job_get", "job_cancel", "ui_focus", "ui_reveal"]`, 4 not 9).

## Workspace functions depended on (all pre-existing, none invented)

- `HeadlessWorkspace::resolve_default_plugin_id(&self) -> Result<String, GatewayError>` — the ONE
  tier-2 gate every handler shares, forwarded verbatim (never re-worded).
- `HeadlessWorkspace::read_artifact_bytes(&self, id) -> Result<Option<(Vec<u8>, Vec<u8>)>, GatewayError>`
  — existence + size for `artifact_open`/`artifact_export`.
- `HeadlessWorkspace::workspace_artifact_ids(&self) -> Result<Vec<String>, GatewayError>` — duplicate-id
  guard in `artifact_create`.
- `HeadlessWorkspace::ensure_probe_artifact(&self, id, initial) -> Result<RevisionStamp, GatewayError>`
  (async, bridged via `semio_framework::io::resolve_ready`) — the real creation path.
- `GatewayBackend::read_resource(&self, uri)` (trait impl on `HeadlessWorkspace`/`Arc<HeadlessWorkspace>`)
  — used for `semio://artifact/{id}/schema`, `/history`, `/validation`, and the bare `semio://artifact/{id}`
  body, for `artifact_open`'s kind/revision, `artifact_validate`, and `artifact_snapshot`. No second
  decoder was written; every read goes through this one public entry point.
- `find_repo_root`, `load_plugin_registry`, `find_plugin_entry`, `load_package_descriptor` (all
  pre-existing `pub fn`s in `🏠️workspace/🦀️component.rs`, none gated `wasm32`) — used by
  `artifact_export` to read the resolved plugin's real, committed `export_formats`
  (`semio_framework::manifest::ArtifactKindSpec`) straight from `🔣️descriptor.json`.

**Nothing needed does not exist yet** — no dependency gap to flag against `🏠️workspace`.

## Progressive-enhancement behaviour (uniform across all 5 tools)

1. No workspace bound → retryable `PLUGIN_UNAVAILABLE`, "start the gateway with --folder <dir> or
   --hub <url> --space <id> to use artifact tools" (this file's own message).
2. Workspace bound, `resolve_default_plugin_id()` fails (0 or 2+ plugins) → that call's own retryable
   `PLUGIN_UNAVAILABLE`, forwarded unchanged.
3. Fully bound → real behaviour:
   - `artifact_open`: real bytes → size; best-effort real `kind` (schema id) and `revision`
     (derived from `appliedEditIds`) via `read_resource`, `None` when this workspace has no open
     probe/history for that id (never fabricated) — `NOT_FOUND` for an unknown id.
   - `artifact_create`: `PRECONDITION_FAILED` if the id already exists (checked against
     `workspace_artifact_ids()`); otherwise a real `ensure_probe_artifact` commit, returning a real
     `RevisionStamp`.
   - `artifact_validate`: forwards `🏠️workspace`'s own real, already-honest gap (no wire validate
     command yet) — never `{"valid": true}`.
   - `artifact_snapshot`: real pack/spr bytes (base64) for the CURRENT revision only; a supplied
     `revision` that doesn't match current → `PRECONDITION_FAILED` with both stamps in `details`
     (no fabricated historical snapshot).
   - `artifact_export`: verifies the artifact exists, then enumerates the resolved plugin's real
     `export_formats` and returns them inside a `PLUGIN_UNAVAILABLE` tool-error's `details`
     (`availableFormats`, `requestedFormat`) — the wire protocol has no live export command yet, so
     this never fabricates a successful export, even for a format the plugin does declare.
   Argument validation (`INPUT_INVALID` for a missing required field) runs before any tier check, on
   every tool.

Known, documented gap (stated in the file's own module doc, not hidden): `artifact_create`'s `kind`
argument is accepted but not yet wire-routed to a plugin-specific document type — every real creation
goes through this crate's own generic probe-document mechanism, because the real
`PluginArtifactChannel` protocol has no "create a document of schema X" command yet (same class of
gap `🏠️workspace` already documents for `validation`/`schema`).

## Verified vs. unverified

**Verified by reading, not by running**: every symbol/signature/field named above was confirmed by
reading `🏠️workspace/🦀️component.rs`, `🧭️protocol/🦀️component.rs`, `🗂️catalog/🦀️component.rs`,
`⚠️errors/🦀️component.rs`, `🧬️schema/🦀️component.rs`, and `🛂️manifest/🦀️component.rs`
(`ArtifactKindSpec.export_formats`, `AppDefinition.artifact_kinds`) directly, not assumed.

**Not run**: per this ticket's build-situation note, I did not run the test suite — only attempted
`cargo check -p semio-framework-os-mcp --message-format short` (at most twice, per instructions), and
that result is reported separately in chat, not claimed as a passing test run. The `#[cfg(test)] mod
quick` block (bottom of the new file) covers: all 5 tools register under their declared names; every
input/output schema is object-typed 2020-12 at the top level; every tool returns a retryable
`PLUGIN_UNAVAILABLE` with no workspace bound; missing a required field is `INPUT_INVALID` before any
workspace check; a workspace with zero resolvable plugins is still `PLUGIN_UNAVAILABLE`; a real
single-plugin round trip (`artifact_create` → duplicate-id rejection → `artifact_open` → unknown-id
`NOT_FOUND`) using a minimal hand-built `CatalogSource.gateway` entry (never `🧫️fixtures`, which its
own doc comment reserves for `🗂️catalog`/`🔎️search`/`🧠️context`/`🧪️conformance`); `artifact_validate`
never fabricates a pass; `artifact_snapshot` returns real non-empty bytes for the current revision and
rejects a stale one; `artifact_export` never fabricates a successful export. None of this was executed
in this session — it is written to compile and pass by the same reasoning that already-passing
sibling tests in `🏠️workspace/🦀️component.rs`'s own `mod quick` use (e.g. `resolve_ready` called from
a plain, non-`tokio::test` `#[test]` — the exact pattern `workspace_artifact_ids`/`read_artifact_bytes`
already rely on there).
