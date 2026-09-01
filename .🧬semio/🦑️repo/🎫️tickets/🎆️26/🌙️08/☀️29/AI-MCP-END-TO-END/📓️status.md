# 🌉️ AI MCP End to End — status

Goal: `semio-os-mcp` (the `semio` entry in `.mcp.json`, crate `semio-framework-os-mcp`, bin
`semio-os-mcp`) works **end to end** — plugin/app/artifact **independent**, with **progressive
enhancement**, so an agent can reach the artifact, inferences, UI information, UI actions, mutations
and history.

Module: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/` (~13k lines Rust, 17 facets).

## Verified baseline (read from source, not assumed)

| # | Fact | Evidence |
|---|---|---|
| B1 | 20 tools registered; **11 real**, **9 hard stubs** returning `PLUGIN_UNAVAILABLE` | `🦀️component.rs:244` `DECLARED_STUB_TOOL_NAMES` = artifact_create/open/validate/export/snapshot, job_get, job_cancel, ui_focus, ui_reveal |
| B2 | The **production catalog is a hardcoded note+cad fixture** | `🦀️component.rs:160` `build_catalog()` → `note_and_cad_source()` (`🧫️fixtures/🦀️component.rs:417`) |
| B3 | The real artifact channel is **hardcoded to the `note` plugin** | `🦀️component.rs` `server_for_workspace_options` → `workspace.open_artifact_channel("note")` |
| B4 | `resources/*` is served by `CatalogResourceRegistry` — the **workspace's own `read_resource`/`list_resources` is never reached** | `🦀️component.rs:520` + `:552`; `🧠️context/🦀️component.rs:137`; workspace impl at `🏠️workspace/🦀️component.rs:938,957` |
| B5 | Only **3 resource URIs** exposed: `semio://capability`, `semio://capability/{id}`, `semio://workspace` | `🧠️context/🦀️component.rs:147` |
| B6 | Every **mutation** command through the real plugin channel returns `channel.not-wired` | `🏠️workspace/🦀️component.rs:702-710` (PureCommand, TransactionPrepare/Commit/Rollback/Undo/Redo) |
| B7 | `HeadlessWorkspace::prepare_action`/`invoke_action` return `PLUGIN_UNAVAILABLE` | `🏠️workspace/🦀️component.rs:926,930` |
| B8 | `semio://artifact/{id}/validation` is **hardcoded** `{"valid":true}`; `/schema` is unavailable | `🏠️workspace/🦀️component.rs:1033,1044` |
| B9 | **Zero inference access** anywhere in the module | no `infer` symbol in `🌉️mcp/**` |
| B10 | **Zero prompts** registered (`InMemoryPromptRegistry::new()`) | `🦀️component.rs:521,552` |
| B11 | One fixed session id for every connection | `🦀️component.rs:252` `DEFAULT_SESSION_ID = "sess_default"` |
| B12 | The bridge **is** real and has a real consumer | `🧵️bridge/🦀️component.rs` (16 frame kinds) + React `AgentBridge` hook in the renderer |
| B13 | `HeadlessWorkspace` is **real** — real `ArtifactHost`, real wasmtime runtime, real plugin registry + `.wasm` resolution, real VCS backbone | `🏠️workspace/🦀️component.rs:84-119,317-328,733-751,816-855` |

**Bottom line:** the protocol/transport/bridge/handles/policy/audit layers are genuinely finished.
Everything that makes the server *useful to an agent over a real workspace* is either fixture-backed
(B2/B3), unreachable (B4/B5), or a stub (B1/B6/B7/B8/B9).

## Work packages

| id | scope | owned files |
|---|---|---|
| W1 | Live catalog compiled from the **real plugin registry**, not the note/cad fixture; progressive fallback to gateway-only | new `📇️registry/🦀️component.rs` |
| W2 | Workspace-backed `ResourceRegistry` so `semio://artifact/**`, `semio://workspace/**` actually reach clients; full URI family + templates | `🧠️context/🦀️component.rs` |
| W3 | Real mutation exchange through `PluginArtifactChannel` (kill `channel.not-wired`); plugin-agnostic channel routing | `🏠️workspace/🦀️component.rs` |
| W4 | Real `artifact_*` tools against the live workspace | new `🗿️artifact/🦀️component.rs` |
| W5 | **Inference** access — tool + `semio://artifact/{id}/inference/**` | new `💡️inference/🦀️component.rs` |
| W6 | **UI information + UI actions** over the bridge (`ui_focus`, `ui_reveal`, `semio://window`, `semio://ui/active-context`), degrading cleanly with no shell attached; `job_get`/`job_cancel` | new `🖥️ui/🦀️component.rs` |
| W7 | End-to-end gate: spawn the binary, speak JSON-RPC over a real folder workspace, assert the whole loop; `launch.json` entries | `📦️packages/🟦️typescript/`, `launch.json` |
| W0 | Root wiring + facet skeletons + glue mounts (coordinator) | `🦀️component.rs`, `📦️glue.rs` |

## Progressive-enhancement contract (applies to every package)

Three tiers, chosen at runtime, never a hard failure:

1. **Bare** (no `--folder`/`--hub`): gateway-only capabilities; every workspace-bound tool returns a
   structured, retryable `PLUGIN_UNAVAILABLE` naming what binding it needs.
2. **Headless** (`--folder`/`--hub`): artifacts, inferences, mutations, history — everything except
   live UI.
3. **Attached** (a live shell dialed the `/bridge` socket): additionally UI information and UI
   actions.

A tool's *presence* in `tools/list` never depends on the tier — only its result does. Same for
resources: templates are always listed, reads degrade with a typed error.

## Progress

(appended as waves land)

---

## Wave 1 — landed

| pkg | what landed | file |
|---|---|---|
| W1 | Catalog now compiled from the **real installed plugin registry**; `note_and_cad_source()` demoted to a test fixture. Never falls back to fixture data — a server with no plugins advertises gateway capabilities only. | new `📇️registry/🦀️component.rs` (198 ln) |
| W2 | `WorkspaceResourceRegistry` replaces `CatalogResourceRegistry` — the workspace's own `read_resource`/`list_resources` (B4: previously dead code) now reach the client. | `🧠️context/🦀️component.rs` |
| W3 | Plugin-agnostic channel routing (`resolve_default_plugin_id()` replaces the hardcoded `"note"`); mutation channel work in `🏠️workspace`. | `🏠️workspace/🦀️component.rs` (+485 ln) |
| W4 | Five real `artifact_*` tools with real schemas; `artifact_export` enumerates the plugin's own committed `export_formats`. | new `🗿️artifact/🦀️component.rs` (657 ln) |
| W5 | `inference_list`/`inference_get` + `semio://artifact/{id}/inference[/{field}]`, over each plugin's committed `ContributedInferenceMetadata` roster. | new `💡️inference/🦀️component.rs` (617 ln) |
| W6 | `ui_focus`/`ui_reveal` over the bridge's `ShellCommand` frames; `job_get`/`job_cancel` over a plugin-agnostic `JobRegistry`; `semio://window`, `semio://ui/active-context`, `semio://ui/selection`, `semio://job/{id}`. | new `🖥️ui/🦀️component.rs` (973 ln) |
| W0 | Glue mounts, facet re-exports, catalog folding, tool registration, bridge slot plumbing, resource composition, test repointing. | `📦️glue.rs`, `🦀️component.rs`, `🚚️transport/`, `🧠️context/`, `🖥️ui/` |

### The headline change

`DECLARED_STUB_TOOL_NAMES` **is gone.** It is replaced by `GATEWAY_TOOL_NAMES` — a 22-name census
of tools that are all real, asserted against the registry by
`the_tool_census_matches_the_registry_exactly`:

```
capabilities_search  capabilities_describe  context_resolve
action_prepare  action_invoke  action_cancel
transaction_begin  transaction_commit  transaction_rollback
history_undo  history_redo
artifact_open  artifact_create  artifact_validate  artifact_snapshot  artifact_export
inference_list  inference_get
ui_focus  ui_reveal  job_get  job_cancel
```

### The bridge slot

`McpServer` is built **before** the HTTP transport mints its `BridgeHandle` (the handle comes from
the transport's own worker pool, so hoisting it would spin a second pool), and `stdio` never serves
`/bridge` at all. So the tool registry captures a `BridgeSlot = Arc<OnceLock<Arc<BridgeHandle>>>` at
build time and `HttpTransport::publishing_bridge_into` fills it once, on `start`. An unfilled slot
*is* the "no bridge" tier — not an error.

### Progressive enhancement, as implemented

A tool's presence in `tools/list` and a resource's presence in `resources/list` **never** depend on
tier. Only results do:

| tier | binding | artifacts / inferences / mutations / history | UI |
|---|---|---|---|
| bare | none | retryable `PLUGIN_UNAVAILABLE` naming `--folder`/`--hub` | same |
| headless | `--folder` / `--hub` | real | retryable "no shell is attached" |
| attached | + a shell dialed `/bridge` | real | real |

## Blocker (external, not ours)

`semio-framework-plugin-host` — a dependency — was broken by a peer's in-flight
`MUTATION-OUTCOMES-MERGE-POLICIES` wave. Fixed from 38 → 8 errors (see
`📓️fix-plugin-host-blocker.md`); the residual 8 (`ui_patch_receipt` missing from `TurnResult`
initializers; `byte_page`/`instance_lifetime` WIT host bindings) belong to a peer's actor/kernel
wave still in flight. Nothing downstream typechecks until those land.

## Wave 2 — landed

| pkg | what landed | file |
|---|---|---|
| W7a | **Prompts** — the `prompts/*` registry shipped empty (`InMemoryPromptRegistry::new()`). Now five protocol-teaching prompts (`explore_workspace`, `safe_mutation`, `inspect_artifact`, `drive_the_ui`, `undo_last_change`), each fully bilingual EN/DE, none naming a plugin (asserted by a test). | new `💬️prompts/🦀️component.rs` |
| W7b | **End-to-end gate** — a real-process suite that spawns the binary and speaks raw JSON-RPC: 22-tool census, no tool self-describing as a stub, object-typed schemas, bilingual prompts, the resource families, and all three progressive-enhancement tiers actually behaving differently. | new `📦️packages/🟦️typescript/🧪️end-to-end.test.ts` |
| W7c | **Binary path graduated.** `resolveMcpBinaryPath` resolved against `26/08/17/…/🎯️target` — a *previous ticket's scratch dir*. Once that ticket's scratch was cleaned, all three conformance suites skipped themselves and the gate read green over nothing. Now `target/debug`, honouring `CARGO_TARGET_DIR`/`SEMIO_OS_MCP_BIN`. | `🟦️component.ts`, `🧪️vitest.config.ts` |

`launch.json` already carried `🛠️dev🌉️os-mcp🧵️stdio`, `🛠️dev🌉️os-mcp🌐️http`, `🖱️mcpinspector🌉️os` and the
compound `🧭️compound🖥️s⚛️react🌉️os-mcp` (gateway + s React shell — the "attached" tier), so no new
entries were needed.

### Why W7c mattered more than it looks

This is the [[taxonomy-filename-drift]] failure mode in a different costume: a gate that silently
measures nothing still reports success. `describe.skipIf(!BIN_PRESENT)` warns rather than passing
quietly, which is why it was findable at all — but the resolved path had drifted to a directory no
build writes to any more.

## Wave 3 — the defect the first two waves created

W1 (real registry) and W3 (no hardcoded plugin) were each correct and **jointly broke production**.
W3 replaced the hardcoded `"note"` with `resolve_default_plugin_id()`, which resolves only when the
catalog names **exactly one** plugin. W1 then made the catalog name every installed plugin (~59). So
on any real install the resolver always took its "2+ plugins is ambiguous" branch and the entire
mutation protocol returned `PLUGIN_UNAVAILABLE` — strictly worse than the hardcoded literal it
replaced. Its own doc comment named the missing piece; nothing had built it.

**W8 built it.** `RoutingArtifactChannel` resolves the plugin from the capability being invoked
(`AppCommand::PureCommand` carries `capability_id`; every `CapabilityDefinition` carries
`owner: CapabilityOwner::Plugin { plugin_id, .. }`), lazily opening and caching one
`PluginArtifactChannel` per plugin id. `resolve_default_plugin_id` is **deleted**, not deprecated;
all four call sites migrated (`server_for_workspace_options`, `action_adapter`, `🗿️artifact`,
`💡️inference` — the last now unions inferences across all plugins instead of failing).

One step beyond the brief was needed: `ActionAdapter::prepare` issues a bare `ReadHistory` *before*
its `PureCommand`, and `ReadHistory` carries no capability id — so `prepare_action` now derives the
instance slot from the catalog up front, or the first call of every mutation would still dead-end.

**This is the general lesson of the ticket:** two independently-correct plugin-independence changes
composed into a regression that only shows up at real installation scale. Fixture-sized test data
(note + cad = 2 plugins) hides it perfectly — a one-plugin catalog and a two-plugin catalog both
"work" under a resolver that only accepts exactly one.
