# W8 — Per-capability plugin routing for the artifact channel

## The defect, as confirmed

- W1 (`📇️registry/🦀️component.rs`) made the catalog compile from the real installed plugin
  registry — confirmed `find "🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json"` names **59**
  plugins including literal ids `note` and `cad`.
- W3 removed the hardcoded `"note"` literal from `server_for_workspace_options`
  (`🦀️component.rs`) and `HeadlessWorkspace::action_adapter()` and replaced both with
  `HeadlessWorkspace::resolve_default_plugin_id()`.
- `resolve_default_plugin_id` (pre-W8, `🏠️workspace/🦀️component.rs:1052`) only succeeded when the
  catalog named **exactly one** plugin-owned capability; 0 or 2+ was a typed, retryable
  `PLUGIN_UNAVAILABLE` naming the ambiguity. On any real install (59 plugins) this is **always**
  the 2+ case, so every real mutation call (`action_prepare`, and `HeadlessWorkspace`'s own
  `prepare_action`/`invoke_action`) failed before ever touching a channel. Confirmed by reading the
  call sites directly — no test exercised the real 59-plugin catalog against this path.

Two SEPARATE call sites depended on this broken resolver:
1. `🦀️component.rs`'s `server_for_workspace_options` — builds the **root-owned** `ActionAdapter`
   that the top-level `action_prepare`/`action_invoke`/`transaction_*`/`history_*` MCP tools
   actually drive. This is the real, agent-facing production path.
2. `HeadlessWorkspace::action_adapter()` — a second, workspace-owned `ActionAdapter` that
   `GatewayBackend::prepare_action`/`invoke_action` delegate to (used directly by tests and by any
   future in-process caller of the trait).

Two more callers existed outside `🏠️workspace` itself, found by grepping the whole `🌉️mcp` module:
`🗿️artifact/🦀️component.rs`'s `require_resolvable_plugin` (gated `artifact_open/create/validate/
snapshot/export`) and `💡️inference/🦀️component.rs`'s `declared_inferences_for_workspace` (read one
plugin's declared inference roster) — both inherited the same "exactly one plugin" assumption and
would have failed identically on a real multi-plugin catalog.

## The routing design

### `resolve_plugin_for_capability`

`HeadlessWorkspace::resolve_plugin_for_capability(&self, capability_id: &str) -> Result<String,
GatewayError>` — `catalog.get(capability_id)`, then match `CapabilityOwner`: `Plugin{plugin_id,..}`
→ `Ok`; unknown id → `NotFound`; any other owner (`Os`/`Framework`/`Shell`/`Gateway`/`Extension`) →
retryable `PluginUnavailable` naming which owner it actually was (`CapabilityOwner::dedup_key()`).
Its real body is the free fn `resolve_plugin_for_capability_in(catalog: &Catalog, ..)` so
`RoutingArtifactChannel` (below) can call the identical logic without a `&HeadlessWorkspace`.

### The ReadHistory-before-PureCommand problem (why routing needed more than "read the command")

The ticket brief suggested routing purely by inspecting `AppCommand::PureCommand{capability_id,..}`
at `exchange()` time. Reading `🔀️dispatch/🦀️component.rs::ActionAdapter::prepare` in full showed
this is **insufficient alone**: `prepare()` issues a bare `AppCommand::ReadHistory` (no capability
id at all) on `instance` **before** the `PureCommand` that names one. Naively caching
"last-PureCommand-seen-for-this-instance" is unsafe too: every pre-W8 call site hardcoded
`instance = 0` for every capability regardless of plugin, so a `ReadHistory` for capability B could
silently reuse a stale routing entry left by capability A's PREVIOUS call — a silent misroute, not
an error, forbidden by this ticket's own constraints.

Fix: `HeadlessWorkspace::prepare_action` now resolves `capability_id`'s plugin **first**, then picks
`instance = plugin_instance_slot(catalog, plugin_id)` — the plugin's position in
`distinct_plugin_ids(catalog)` (the catalog's own `CapabilityOwner::Plugin` ids, deduplicated,
`BTreeSet`-sorted) — instead of the old bare `0`. Both `prepare_action` and
`RoutingArtifactChannel::exchange`'s decode step (`plugin_for_instance_slot`, the inverse) compute
this from the **same catalog**, so no shared mutable state is needed and every command in one
`prepare`+`invoke` sequence (`ReadHistory` included) decodes to the correct plugin. This is beyond
the ticket's literal 6-item list but was necessary for the fix to actually revive `prepare_action`
end-to-end rather than trade one permanent `PLUGIN_UNAVAILABLE` for another.

### `RoutingArtifactChannel` (`🏠️workspace/🦀️component.rs`, new `//#region 🔖️Routing`)

Implements `crate::actions::ArtifactChannel`. Fields: `catalog: Arc<Catalog>`, `repo_root:
Option<PathBuf>`, `actor_label: String`, `channels: Mutex<HashMap<String, PluginArtifactChannel>>`.
Deliberately holds only what it needs, not `Arc<HeadlessWorkspace>` (the ticket's own "or whatever
it needs" latitude) — `HeadlessWorkspace` has no self-`Arc` handle anywhere in the codebase and
adding one would mean changing every `open_folder`/`open_hub` caller's return type repo-wide, well
outside this ticket's scope.

`exchange(instance, commands)`:
1. `plugin_id_for(instance, &commands)` — if any command is a `PureCommand{capability_id,..}`,
   resolve via `resolve_plugin_for_capability_in` (authoritative — always re-validated per call,
   never stale); else decode `instance` via `plugin_for_instance_slot`. Neither found → typed
   `Fault{code:"plugin.unavailable",..}` naming the instance and the catalog's plugin count.
2. Lazily open-and-cache one real `PluginArtifactChannel` per plugin id (`channels` `HashMap`,
   locked only across the lookup/insert, never across a channel's own `exchange` — no lock-ordering
   hazard against `open_probes`/`action_adapter`'s mutexes, which this struct never touches).
3. Delegate to that plugin's own channel.

`open_plugin_artifact_channel(repo_root, plugin_id, actor_label)` — the shared body (registry entry
→ committed descriptor → editor app → real `PluginArtifactChannel`) both `RoutingArtifactChannel`
and the pre-existing `HeadlessWorkspace::open_artifact_channel` (kept, now a thin wrapper) call, so
the logic exists exactly once.

`routing_fault(GatewayError) -> Fault` maps `NotFound` → `"capability.not-found"` (client fault,
never retryable) and everything else → `"plugin.unavailable"` (retryable) — added two matching arms
to `🔀️dispatch/🦀️component.rs::map_fault` (`"capability.not-found"` → `GatewayErrorCode::NotFound`,
`"plugin.unavailable"` → `GatewayErrorCode::PluginUnavailable().retryable()`) so these surface
correctly through `ActionAdapter::exchange_one`, the only Fault→GatewayError boundary in the
mutation protocol.

Added to the `ArtifactChannels` `dyn_enum_close!` set as the third variant `Routing
(RoutingArtifactChannel)`, alongside `Mock`/`Plugin`.

## Call sites migrated

| site | before | after |
|---|---|---|
| `🦀️component.rs::server_for_workspace_options` (the one permitted edit) | `workspace.resolve_default_plugin_id().and_then(\|id\| workspace.open_artifact_channel(&id))`, falling back to `MockArtifactChannel` on any error | `ArtifactChannels::Routing(workspace.open_routing_channel())` — no fallback needed; a routing failure now surfaces per-call, not at server boot |
| `HeadlessWorkspace::action_adapter()` | `resolve_default_plugin_id()?` then `open_artifact_channel(&plugin_id)?`, both failing the WHOLE adapter build for a multi-plugin catalog | `ArtifactChannels::Routing(self.open_routing_channel())` — never fails to build; routing errors are per-call |
| `HeadlessWorkspace::prepare_action` | `instance = 0` | `instance = plugin_instance_slot(catalog, resolve_plugin_for_capability(capability_id)?)` |
| `🗿️artifact/🦀️component.rs::require_resolvable_plugin` (5 call sites) | `workspace.resolve_default_plugin_id()` gating all 5 handlers identically | split: `require_workspace_has_a_plugin` (≥1 plugin registered — gates `open/create/validate/snapshot`, none of which route through a specific plugin channel) and `require_resolvable_export_plugin(workspace, artifact_id)` (exactly 1 plugin — `artifact_export` alone needs a concrete plugin id, and this workspace has no artifact→plugin mapping to do better; documented as an honest, named gap, not a guess) |
| `💡️inference/🦀️component.rs::declared_inferences_for_workspace` | read ONE plugin's descriptor via `resolve_default_plugin_id()`, so it outright failed the moment a catalog named 2+ plugins — the doc comment's claim that this "widens to every installed plugin for free once W1 lands" was never true | now unions every plugin named by `workspace.catalog_plugin_ids()`'s declared inference roster — the behavior that comment always meant to describe |

New public API added to `HeadlessWorkspace`: `resolve_plugin_for_capability`, `catalog_plugin_ids`,
`open_routing_channel`.

## Deleted

`HeadlessWorkspace::resolve_default_plugin_id` — outright, no deprecation/compat shim (greenfield).
Confirmed zero remaining call sites repo-wide (`grep -rn resolve_default_plugin_id
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/`); the only surviving hits are doc-comment prose
explicitly narrating what it used to do and why it was replaced.

## Audit invariants preserved (`🏠️workspace/🦀️component.rs`'s source-shape audit)

Verified present after edits: `pending_exchanges` (14), `PendingResponsePage` (7),
`RejectedCommandBuildRegistry<1>` (1), `CommandBatchDriver` (1),
`close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)` (4), `terminal_is_empty` (7),
`persistent_command_completion_port_ready` (2). Verified absent:
`response: Option<Result<store::AppFrame, Fault>>` (0 matches).

## Tests added (`#[cfg(test)] mod quick`, `🏠️workspace/🦀️component.rs`)

Replaced the three OLD tests asserting `resolve_default_plugin_id`'s single-plugin-or-bust
behavior, and rewrote `prepare_action_and_invoke_action_are_a_well_formed_plugin_unavailable_not_a_
panic` (its `PLUGIN_UNAVAILABLE`-for-everything assumption no longer holds — routing is per-call
now, not per-adapter-build):

- `prepare_action_on_an_unknown_capability_is_not_found_not_a_panic`
- `invoke_action_on_an_unknown_handle_is_not_found_not_a_panic` — demonstrates the actual fix:
  `action_adapter()` no longer fails to build on a multi-plugin catalog, so this now reaches the
  real `HandleTable` and gets `NOT_FOUND`, not the old blanket `PLUGIN_UNAVAILABLE`.
- `resolve_plugin_for_capability_is_not_found_for_an_unknown_id`
- `resolve_plugin_for_capability_is_plugin_unavailable_for_a_gateway_owned_id`
- `resolve_plugin_for_capability_routes_note_and_cad_to_different_plugins`
- `routing_artifact_channel_purecommand_unknown_capability_is_not_found_before_opening_any_channel`
- `routing_artifact_channel_purecommand_gateway_owned_capability_is_plugin_unavailable`
- `routing_artifact_channel_exchange_on_an_unrouted_instance_without_a_purecommand_is_plugin_unavailable`
- `routing_artifact_channel_routes_two_capabilities_to_two_different_plugins_opening_each_once` — the
  literal case named in the work order: note+cad routed to two different real channels, plus a
  `ReadHistory` (no capability id) on the `note` instance proving `plugin_for_instance_slot` decodes
  correctly, then asserts `channels.lock().len() == 2` (one cached channel per plugin, not per
  call). Skips with a clear `eprintln!` (never a fabricated pass) when `note`/`cad` `.wasm` are not
  built at `target/wasm32-wasip2/{debug,wasm-release}` — same convention as the pre-existing
  `plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired` test it sits next to.
  Checked in this environment: neither `note.wasm` nor `cad.wasm` is built here, so this test's
  real-channel assertions did **not** execute (skip path only) — see Verified vs unverified below.

## Verified vs written-but-unverified

**Verified by direct reading, not assumed:**
- Every call site of the deleted `resolve_default_plugin_id` (repo-wide grep, before and after).
- The exact `AppCommand`/`ActionAdapter::prepare`/`invoke_uncached` control flow that motivated the
  `plugin_instance_slot` design (read `🔀️dispatch/🦀️component.rs` in full, not skimmed).
- `note`/`cad` both exist as literal plugin ids in the real generated registry
  (`🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json`, 59 entries).
- `note.editor.setGridVisible`/`cad.editor.addObject`/`capabilities.search` are real capability ids
  the note+cad fixture catalog compiles (traced the exact `format!("{plugin_id}.{app_id}.{action_id}")`
  id-construction in `🗂️catalog/🦀️component.rs::compile`, and `capabilities.search`'s
  `CapabilityOwner::Gateway` literal in `🦀️component.rs::capabilities_search_capability`).
- The audit-required strings/absence in `🏠️workspace/🦀️component.rs` (grep counts above).

**Written but NOT run — no `cargo test` was executed** (the task brief explicitly caps me to `cargo
check`, and `cargo check` alone cannot execute `#[test]` fns): every test listed above compiles by
my own reading, not by an observed pass. Do not treat any of them as passing until someone runs
`cargo test -p semio-framework-os-mcp`.

**`cargo check -p semio-framework-os-mcp` itself**: launched twice; the crate build graph was under
heavy contention (15-20+ concurrent `cargo check` processes from other agents/tickets the whole
time) and neither run produced output before this report was written — see the session's own final
message for whichever result, if any, eventually landed. If neither returned, this change's
compilation is **unverified by any build**, only by manual type-level reading of every call site.

## Known, deliberately out-of-scope gaps

- `require_resolvable_export_plugin` cannot determine which of 2+ registered plugins owns a given
  `artifact_id` — no artifact→plugin mapping exists anywhere in this crate (every artifact this
  workspace manages is the generic, schema-agnostic probe document, never a plugin-typed one). Named
  honestly as a retryable `PLUGIN_UNAVAILABLE` rather than guessed; building a real mapping is a
  `🗿️artifact`-owned (W4) architectural question, not this ticket's.
- `server_for_workspace_options`'s own doc comment (lines above the one edited line) still describes
  the OLD "targets `note`... falls back to `MockArtifactChannel`" behavior — left untouched per this
  work order's explicit "one call site only" restriction on `🦀️component.rs`; its owner should
  update that prose separately.
