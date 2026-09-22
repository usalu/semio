# 📓️ M10 — an MCP agent COMMITS an edit to a live hub document

Slice M10 (session 8, 2026-09-22). Continues M9, which closed D2's `repo root not found` and
stopped at `wasmtime` instantiate on the `semio:framework/codec@1.0.0` world boundary.
Bar: a `Commands` frame in the hub ledger authored by an agent (`principalKind: agent` on the
presence beat) and a human browser seeing it land.

Status legend: ✅ landed and executed live · 🚧 landed and compiled, not executed · ❌ not done.

## 0. Headline / coordination

**Outcome 4's bar — an agent's `Commands` frame in the hub ledger with `principalKind: agent` — is
NOT crossed, and this slice does not claim it.** What it did is turn M9's two "unstarted, with a
named unknown" steps into landed code — three legs measured live on hub 7621, plus the codec that
unknown was about — so that what is left between the gateway and a `Commands` frame is no longer a
line of code but a hub.

Live on 7621 (`🗑️generated/m10-write-path-7621.txt`, §5.2), a hub-bound agent's `artifact_open` now:

* **binds** the hub document to the plugin and app the hub's own execution-target lease names
  (`pluginId=gis appId=s.gis.gismap@1/*#editor`),
* on the lease's **own surface** (`surfaceId=s.gis.gismap@1/*#editor`) — the pinned
  `PersistenceBinding::Hub { surface: Some(PROBE_SURFACE_ID) }` at `🏠️workspace/🦀️.rs:501` that the
  brief names is gone,
* holding the document's **canonical pair** (`packBytes=80801 sprBytes=237`) so the plugin's guest is
  seeded with the hub's document instead of the plugin's genesis, and
* reporting its **write path** as a fact the agent can read before spending a mutation.

**M9 §8.1's named unknown — "`ArtifactHost::open` needs a registered native codec for `gis.map`,
which this process does not have" — is closed, not merely measured** (§3.3): the codec now comes from
the package's own `codec` export, through four `GuestRuntimes` forwarding methods and two
`WasmtimeRuntime` implementations that did not exist before this slice. Laws green against a real
`note` component. What remains between here and a `Commands` frame is **not code**: it is a hub whose
catalog was published after TC3b added `export codec` to `world actor`. On the only hub reachable all day
(7621, catalog 2026-09-21 05:49) the write path and `action_prepare` now fail with the SAME single
refusal — ``no exported instance named `semio:framework/codec@1.0.0` `` — i.e. one catalog age, not
two code gaps. C8's fresh-catalog 7671 came up at 15:57:32, showed
**`features.mcpWorkspace: true`** (a first — M8 §4's flag, never observed live before), refused
every sign-in with a session store pointed at its own catalog-validation sandbox, and exited two
minutes later (§5.1).

| item | state |
|---|---|
| 1 — `LoadDocument` of the canonical pair | ✅ landed; binding + pair + surface measured live (§2, §5.2) |
| 2 — the `…/socket-grants` document socket and the `Commands` frame | 🚧 all three legs landed — guest backbone egress captured and routed, the lease's real surface, and the document codec taken from the package's own `codec` export (§3.3, laws green on a real component). The socket has never CONNECTED, because no hub with a post-TC3b catalog came up (§5.1) |
| 3 — `inference_list` includes extension-contributed services | ✅ landed, laws green, **and observed on the live full install** (§4) |
| 4 — live proof | partial (§5): 5/8 on a new hub probe; no edit crossed, no browser run, no screenshot |

### 0.1 Coordination

* **CE3 owns running `hub-agent-participant-check`.** I ran it once on **7621** only, to regression-
  check §2's new bind step (row 8 still PASS); its 14/17 is unchanged and its three reds are the same
  stale-hub-artifact ones CE2 §4 pinned. **I edited no gate file** (`🤖️hub-agent-participant/🟦️.ts`
  untouched). Running it on **7651** is CE3's; 7651 answered `000` throughout this slice.
* **No hub Rust change is needed by the write path**, so there is nothing for HT16 to route from me.
* **The codec step is landed** (§3.3), in `🔌️plugin/🖥️host/🦀️.rs` + `🌉️mcp`, with laws green
  against a real `note` component. `semio-framework-plugin` — FP11's crate — does **not** depend on
  `semio-framework-plugin-host`, so FP11 is unaffected; six other crates do (§7.12).
* **For C8 / HT16**: hub 7671 refused every `POST /auth/sessions` with a session store resolved under
  `…/c8-boot/trusted-catalog/validation/gis-L29LgB/candidate-data/instance/sessions/` — the catalog
  VALIDATION sandbox, not the hub's instance root — then exited with
  `UnsafeAuthConfiguration("local bootstrap endpoint closed")` (§5.1,
  `🗑️generated/m10-7671-auth-fault.txt`). That single fault is what stands between this slice and an
  8/8 write-path run; whoever brings a fresh-catalog hub up next should check that the auth instance
  root is not inherited from a bootstrap validation candidate.
* **`features.mcpWorkspace: true` has now been observed live**, on 7671 at 15:57:32 — M8 §4 and M9 §8.5
  both closed asking for exactly that rerun, and `hub-agent-participant-check` row 0c would pass on
  a hub of that build.
* **7621 is CE2/CE3's hub.** I never restarted it; I minted two delegations across the slice
  (`agent:01a0c8a0-…`, `agent:01a0c8bd-…`) and **revoked both (HTTP 204)**
  (`🗑️generated/m10-delegate.txt`). **7671 is C8's**: I never restarted or republished into it, and
  the one space-creation attempt I made on it (`🐍️gm1-live-open-plan.ts`) never got past sign-in.

## 1. Inherited state

Nothing of my own: `🗑️generated | grep m10` was empty at slice start, no `📓️m10-*`, no `🐍️m10-*`.
Context: M9 (the whole write-path chain and its §8.1 next steps), M8 §8, G19 §1d + gaps 3/6/7,
C7 §2, CE2 §4, `📓️status.md` session 8.

**The codec boundary M9 §2.4 / G19 gap 6 stopped on is CLEARED in the folder lane.** Measured at
slice start (`🗑️generated/m10-component-codec-export.txt`): TC3e's rebuilt components carry the
export M9's tree lacked —

| component | built | bytes | `semio:framework/codec` strings |
|---|---|---|---|
| `🗒️note` component-release | 2026-09-22 04:12 | 14 715 767 | 29 |
| `🗒️note` component-dev | 2026-09-22 02:58 | 64 690 968 | 34 |
| `🌍️gis` component-release | 2026-09-22 05:45 | 47 950 467 | 29 |
| `🌍️gis` component-dev | 2026-09-22 02:39 | 215 377 769 | 34 |

Hub **7651** answered `curl -s :7651/readyz` → `000` at 11:1x, 11:4x and every later probe of this
slice, so the HUB lane's catalog is still the pre-TC3b one wherever it is published. 7621 (CE2/CE3's)
answers 200; 7641 answers 503; 7611/7661 are down.

## 2. Item 1 — `LoadDocument` of the canonical pair ✅ (landed; bind + pair + surface measured live, §5.2)

### 2.1 What was actually missing

`plugin_artifacts` — the `artifact_id → PluginArtifactBinding` map every stamp and every routed
command reads — had **exactly one writer, `create_plugin_artifact`, a folder-only path**. A hub-bound
agent that opened a real hub document therefore had no binding at all, with two consequences M8/M9
measured separately without connecting them:

* every `RevisionStamp` the channel answered named the pseudo-id `plugin:gis` rather than the
  document, and
* `PluginArtifactChannel`'s guest ran against **the plugin's own genesis document**. `PureCommand`
  sends `document: Vec::new(), document_spr: Vec::new()` on the wire — by design, the guest uses its
  session document — so an agent's `action_prepare` would have priced and produced ops against an
  empty `gis.map`, not against the 80 801 bytes it had just read.

### 2.2 What landed

| file | change |
|---|---|
| `🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` | `NativeHubBindingDriver::fetch_execution_target_lease(scope, client_instance_id)` — the per-document execution-target lease (≈3 KB), with a scope cross-check. `refresh_catalog`'s catalog snapshot deduplicates by PACKAGE, so its `scope` names whichever document first resolved that package: it is **not** a per-document surface source, and a surface assembled host-side would be this gateway's guess at what the hub authorized |
| `🌉️mcp/🏠️workspace/🦀️.rs` | `SessionDocumentPair`; `PluginArtifactBinding` gains `surface_id`, `document`, `backbone`, `backbone_blocked_by`; `HeadlessWorkspace::bind_hub_session_document` |
| `🌉️mcp/🏠️workspace/🦀️.rs` | `PluginArtifactChannel::load_session_document` + the per-instance `session_documents` map; `discard_instance` forgets it |
| `🌉️mcp/🏠️workspace/🦀️.rs` | `RoutingArtifactChannel::session_document_for` and the seed step in `exchange` |
| `🌉️mcp/🗿️artifact/🦀️.rs` | `artifact_open` binds the hub document it just read |

Every binding field comes from an authenticated route: the lease is fetched for that exact scope and
its `package.plugin_id` / `artifact.schema` / `artifact.kind` are cross-checked against the
descriptor snapshot the hub binding already verified. Nothing is derived from the artifact id or
guessed from a single-plugin workspace.

`load_session_document` is **idempotent per (instance, artifact)**: re-loading on every exchange
would discard the guest's uncommitted transaction between `TransactionPrepare` and
`TransactionCommit`, which is the whole two-phase contract. An `Infer` command skips the seed
entirely — the inference lane drives its own guest on its own actor ordinal.

## 3. Item 2 — the document socket and the `Commands` frame 🚧 (all three legs landed; blocked only on a fresh-catalog hub)

### 3.1 The chain, and which leg was missing

A committed guest op becomes a hub `Commands` frame over four hops. Three of them already existed;
the two the gateway owned were both absent, and the fourth is blocked.

| # | hop | before M10 | now |
|---|---|---|---|
| 1 | the guest publishes its committed envelopes as `Effect::SendMessage { target: Backbone { uri } }` | real, and **dropped on the floor** — `exchange_one_turn` named every effect for diagnostics and admitted only the `Shell{instance}` reply lane | `document_backbone_payload` + `PluginArtifactChannel::backbone_egress` / `drain_backbone_egress`, fenced on the channel's own actor uri exactly as the wgpu shell's `route_document_backbone_effects` fences it |
| 2 | the host hands them to the document actor as `ArtifactActorMsg::DocumentBackbone` | **no document actor existed** for a hub document — the gateway's only `ArtifactHost::open` call site is `ensure_probe_artifact`, for its own `os.agent.probe/v1` schema | `HeadlessWorkspace::open_hub_document_actor` + `RoutingArtifactChannel::relay_backbone_egress` |
| 3 | the actor persists, retains, and sends `ClientFrame::Commands` on the document socket; the hub stamps `principalKind` onto the presence beat from the authenticated session kind | already real (`🏪️store/🔄️sync/🦀️.rs` `relay_operations_to_hub`; `🌎️hub/🏗️bootstrap/🦀️.rs:4839`) | untouched — correctly |
| 4 | the actor's `connect_hub` builds `SocketHelloV1` from `document_codec(&schema).pack_schema_hash` | **no codec for a fourth package's kind**, so it fell straight through to `schedule_reconnect()` — forever, in silence | `register_guest_document_codec` takes it from the package's own `codec` export (§3.3) |

### 3.2 The surface pin the brief names is gone

`🏠️workspace/🦀️.rs:501` pinned `PersistenceBinding::Hub { surface: Some(PROBE_SURFACE_ID) }` for
every hub document. `open_hub_document_actor` binds **the lease's own
`surface.surface_id`** (`s.gis.gismap@1/*#editor` for the M9 document) and hands the whole lease to
`ArtifactHost::set_document_execution_target_lease` before `open`, because `open` consumes it when it
spawns the actor and the actor sends it inside `DocumentSocketExpectationV1`. A surface id is an
authorization coordinate — `finish_connect_hub` compares the socket authority's scope, schema and
pack-schema hash against the client's own — so claiming the probe editor's surface for a `gis`
document was never going to be admitted. `WorkspaceOrigin::persistence_binding` keeps
`PROBE_SURFACE_ID` for this crate's own probe documents, which genuinely are that surface.

### 3.3 The codec — landed (was the blocker)

`store::ArtifactCodec` is a `fn`-pointer erasure table registered per artifact schema, and this
process links only the three native codecs `📇️native-openable-provider` compiles in. `gis.map` is a
fourth package's kind, so `document_codec("gis.map")` was `None`, `connect_hub` could not build its
`SocketHelloV1` pack-schema hash, and the actor reconnect-looped in silence. **The codec is now
taken from the package's own `codec` export** (`🔌️plugin/🧬️schema/📜️.wit` `interface codec`, TC3b).

#### 3.3.1 `GuestRuntimes` could not reach a codec at all

Every real caller of a compiled plugin holds the `GuestRuntimes` ENUM, never a concrete runtime — a
`wasmtime::Component` belongs to the `Engine` that compiled it, so a second runtime could not
instantiate it. The enum forwarded the `GuestRuntime` trait and **nothing else**, and on top of that
`WasmtimeRuntime` carried only two of the four exports (`pack_schema_hash`, `genesis`); `print_mirror`
and `apply_ops` existed on the owned interpreter alone. Landed in
`🔌️plugin/🖥️host/🦀️.rs`:

| added | note |
|---|---|
| `WasmtimeRuntime::codec_print_mirror`, `WasmtimeRuntime::codec_apply_ops` | the compiled half's **first** implementation of either — the exact shape of the two TC3d already wrote, through `bindings.semio_framework_codec()` |
| `impl GuestRuntimes { codec_pack_schema_hash, codec_genesis, codec_print_mirror, codec_apply_ops }` | dispatches Owned/Wasmtime; the two test-only scripted variants refuse by name rather than fabricating |

#### 3.3.2 The guest-backed `store::ArtifactCodec`

`🌉️mcp/🏠️workspace/🦀️.rs`, region `🗂️GuestDocumentCodec`:
`GuestCodecRoute`, `guest_codec_routes`, `guest_print_mirror`, `guest_apply_ops_binary`,
`guest_compile_dsl`, `guest_edit_text_from_envelope`, `register_guest_document_codec`,
`headless_codec_budget`; wired from `HeadlessWorkspace::register_hub_document_codec`, called by
`open_hub_document_actor` exactly when nothing already links the kind.

Three decisions worth a reviewer's attention, each for a reason:

1. **The hash is the component's, cross-checked against the hub's.** `register_guest_document_codec`
   takes `lease.artifact.pack_schema_hash` as an `expected_` argument and refuses on disagreement;
   the value it registers is what `codec.pack-schema-hash` returned from the component bytes the hub
   authorized and this process SHA-256-verified. Registering the hub's own number would make
   `finish_connect_hub`'s `local_schema_hash == authority.pack_schema_hash` compare the hub's value
   with itself, which is not satisfying that check — it is deleting it (§7.3 of the earlier draft,
   still refused).
2. **The thunks resolve their route by asking the components.** `ArtifactCodec`'s operations are
   bare, non-capturing `fn` pointers whose signatures carry only bytes — no schema, no component —
   so a thunk can close over nothing. It therefore walks the registered routes in order and takes
   the first that answers, which is not a guess: `interface codec`'s own contract states that "a pair
   the component cannot print back is not a document of this kind", so this runs the fence the
   interface defines, once per candidate. In a gateway session the candidate list is the packages
   whose documents the agent has open — one or two.
3. **Two operations refuse, by name.** `compile_dsl` and `edit_text_from_envelope` have no
   counterpart in the WIT at all: both belong to the folder `.dsl`/`.ops` text lane, which a
   guest-backed codec exists precisely because a hub binding does not have. They answer a typed
   `VcsError::Deserialize` naming the four exports that do exist. Fabricating text there would put
   bytes in a `.ops` file no component ever produced.

#### 3.3.3 Verified

* `cargo check -p semio-framework-plugin-host --all-targets` → 0 errors
  (`m10-plugin-host-check-{1,2}.txt`).
* `cargo test -p semio-framework-plugin-host --lib -- guest_runtimes_forwards` → **1 passed**,
  169 s, against the REAL `semio_s_plugin_note.wasm`
  (`m10-plugin-host-law-1.txt`): all four exports forward identically to the concrete runtime,
  an empty `apply-ops` batch returns the pair unchanged, a genesis pair prints a non-empty dsl
  mirror, and a foreign kind is a typed refusal.
* `cargo test -p semio-framework-os-mcp --lib -- guest_backed backbone …` → **11 passed**
  (`m10-guest-codec-law-1.txt`), including the two new codec laws: with no route registered every
  thunk refuses and reports its candidate count, and the two non-exported operations refuse by
  naming the WIT's four.
* **Live on 7621** (`m10-write-path-7621-codec.txt`): the chain now runs all the way INTO the
  component — the gateway fetched the authorized component, compiled it, called
  `codec.pack-schema-hash`, and the write path's remaining refusal is
  ``` `gis` could not answer codec.pack-schema-hash for `gis.map`: wasmtime: no exported instance
  named `semio:framework/codec@1.0.0` ```. That is **7621's 09-21 catalog**, the identical cause as
  row 7's — both reds on that hub are now one fact, and it is a catalog age, not a code gap.

## 4. Item 3 — `inference_list` roster includes extension-contributed services ✅

**Landed and executed (unit laws).** G19 gap 3 is closed, and its stated root cause was one step off.

### 4.1 The real root cause, measured

G19 says "the roster compiler walks the plugin registry, not the extension-contribution registry".
The registry is not the problem: `🔌️plugins.json` carries all 60 packages, 26 of them `role:
extension`, and `RegistryDiscovery::scan` decodes every one. The break is one hop later:

1. `catalog::compile` (`🗂️catalog/🦀️.rs`) walks a descriptor's apps, commands, modes and
   `contributions.{inference,mutation,io,composer}_services` — and **never**
   `contributions.artifact_contributions`.
2. `cad-extension-aec-building`'s committed descriptor declares **0 apps, 0 commands,
   `inferenceServices: null`**, and exactly one `artifactContributions[0].inferences[0]`
   (`s.cad-extension-aec-building.building-structure-summary` on `s.cad.cad`). So it compiles to
   **zero catalog entries**.
3. Folder-mode `HeadlessWorkspace::discovery_descriptors` derives its descriptor set from
   `catalog_plugin_ids()` — i.e. from exactly those entries. A package with no entry is never loaded.
4. `declared_inferences_for_workspace` therefore never saw the descriptor, although
   `declared_inferences_from_descriptor` has always chained `artifact_contributions[].inferences`.

A contribution-only package was **structurally unreachable** from `inference_list`, for any
workspace, however complete the install. A sweep of every committed `🔣️.json`
(`🗑️generated/m10-inference-declarations.txt`) sizes it exactly: **74 owner-authored** services
(`gis ×1`, `wfc ×5` — CE1 §3b's measured roster — plus `stdio ×68`, G19 gap 2's dark ones, which
DO have a committed descriptor declaring them: gap 2 is a decode/catalog failure, not a missing
declaration) and **exactly one contributed service in the whole tree**, the one that was invisible.

### 4.2 What landed

`🌉️mcp/💡️inference/🦀️.rs`: `owned_artifact_kinds`, `contributed_inference_descriptors`,
`installed_descriptors_for_contributions`, and the union step in `declared_inferences_for_workspace`.

The membership rule is **reachability, not "every installed package"**: a contributed inference is
listed exactly when the artifact kind it contributes onto is owned by a package already in the
roster's source set (an app's own `dialect.artifact_kind`, or a library package's plugin-level
`artifact_kinds`). An extension whose host plugin is absent contributes to nothing here and stays
out, so the roster never advertises a service no artifact in this workspace could carry.

Hub mode is untouched and needs nothing: `discovery_descriptors`' hub arm is the authenticated
catalog's whole selection set, extension packages included. The registry read happens on the folder
arm only — a hub-bound agent must never fall back on a local registry.

### 4.3 Laws, executed

`cargo test -p semio-framework-os-mcp --lib -- inference::quick::declared_inferences
workspace::quick::a_routed_channel` → **7 passed, 0 failed** (`🗑️generated/m10-inference-law-1.txt`),
including the two new ones:

- `declared_inferences_for_workspace_lists_an_extension_contributed_service_on_a_kind_it_reaches` —
  a `cad`-only workspace lists exactly
  `("cad-extension-aec-building", "s.cad.cad", "s.cad-extension-aec-building.building-structure-summary")`,
  once, routed by its own contributor id.
- `declared_inferences_for_workspace_omits_an_extension_contribution_onto_an_unreachable_kind` — a
  `wfc`-only workspace gains nothing.

and the pre-existing `declared_inferences_for_workspace_finds_the_real_wfc_roster` (the regression
guard that this is a union, not a replacement) still passes unchanged.

### 4.4 …and observed on the live full install

`🐍️m10-inference-roster-probe.mjs` drives a real stdio MCP session against the built gateway and
prints every row `inference_list` declares (`🗑️generated/m10-inference-roster-live.txt`):

```
declared=74
   1 cad-extension-aec-building | cad-extension-aec-building | s.cad.cad | s.cad-extension-aec-building.building-structure-summary
   1 gis
  67 stdio
   5 wfc
```

The contributed row is there, carrying its own `contributor` id — which is what
`DeclaredInference::route_plugin_id` routes an execution by, so the roster names the guest that
would actually answer it rather than the kind's owner. `client-e2e`'s
`os: inference_list declares the pinned service` row reports the same 74 over the same install.

## 5. Item 4 — live proof

### 5.1 The hub the bar needs — 11 minutes, and not authenticable

Hub **7651** (TC3e's) answered `000` on every probe of this slice (11:1x → 16:07); TC3e re-queued its
bootstrap behind CA1's describe batch. 7641 answers 503; 7611 is down. The only hub reachable all
day is **7621**, whose catalog is C5's, generation `8086b61f336e08b5…`, published 2026-09-21 05:49 —
**pre-TC3b**, so its actors carry no `codec` export.

**C8's 7671 — the fresh `stdio,gis` catalog published from today's tree — came up at 15:57:32 and
was gone by 15:59.** What it showed while it lived is worth recording
(`🗑️generated/m10-7671-auth-fault.txt`):

* `/readyz` answered `"status":"ready"` with **`features.mcpWorkspace: true`** — the first live TRUE
  of M8 §4's derived flag anywhere, i.e. `hub-agent-participant-check`'s row 0c would have passed on
  it. M8 and M9 both closed with "needs coordinator hub rerun" on exactly that value.
* `POST /auth/sessions` as `user1@semio.dev` answered **HTTP 503** after 17.585 s, and the hub's own
  log gave the reason: `server.auth.session.mint … instance-session-create-Backend(
  ".../c8-boot/trusted-catalog/validation/gis-L29LgB/candidate-data/instance/sessions/….json:
  background task failed")`, followed by
  `Error: UnsafeAuthConfiguration("local bootstrap endpoint closed")` and process exit. **The session
  store resolved under the trusted-catalog VALIDATION sandbox rather than under the hub's own
  instance root.** C8's own provisioning failed the same run (`artifact creation reached failed`).
  This is C8's hub and a hub-side fault; it is recorded, not touched.

So **no agent edit crossed, no `Commands` frame was sent, no presence beat carried
`principalKind: agent`, no browser was opened and no screenshot is offered** — the same rule M9
applied. `🐍️c3-collab-scenario.mjs`'s attach half was not run: a human attached to an unchanged
document beside an empty agent roster row would be theatre.

### 5.2 What WAS proven live — the three legs M10 added

`🐍️m10-hub-write-path-probe.ts` (new) against 7621, delegation
`agent:01a0c8a0-3386-730c-9114-9fc53a2b63b7` (audience `edit`, credential mode 0600, revoked HTTP
204 at the end), space `01a0c314-…`, document `artifact-0954e2d10d8fff9605f101b0dba34f3b`.
Capture `🗑️generated/m10-write-path-7621.txt`. **5/8 rows green.**

| # | row | result |
|---|---|---|
| 1 | the gateway serves over the delegated hub session | PASS `semio-os-mcp@0.1.0` |
| 2 | `context_resolve` names the agent's own principal | PASS `agent:01a0c8a0-…` |
| 3 | the agent sees the space's own documents | PASS |
| 4 | `artifact_open` of a HUB document answers | PASS `kind=gis.map artifactKind=s.gis.gismap sizeBytes=81038` |
| **5** | **`artifact_open` BINDS the document to a plugin session** | **PASS `pluginId=gis appId=s.gis.gismap@1/*#editor`** |
| **5b** | **the binding carries the HUB lease's own surface, not the probe surface** | **PASS `surfaceId=s.gis.gismap@1/*#editor`** |
| **5c** | **the binding holds the canonical pair for `LoadDocument`** | **PASS `packBytes=80801 sprBytes=237`** |
| 5d | the document's write path is open | **FAIL**, with its own reason: ``no `store::ArtifactCodec` is registered for artifact schema `gis.map` in this process, so the document socket cannot send its `SocketHelloV1` pack-schema hash — the package's own `codec` export is not wired into the codec registry yet`` |
| 6 | a mutation verb typed against the opened document's kind | PASS `gis.s.gis.gismap@1/*#editor.addFeature` |
| 7 | `action_prepare` reaches a guest the HUB authorized | FAIL — `instantiate: wasmtime: no exported instance named semio:framework/codec@1.0.0` (**7621's 09-21 catalog**, §5.1) |
| 8 | `action_invoke` commits the agent's edit | FAIL — no handle |

Rows 5, 5b and 5c are this slice's three legs, and all three are measured on a live hub rather than
argued. **Row 5d turns M9 §8.1's named unknown into a measurement**: "`ArtifactHost::open` needs a
registered native codec for `gis.map`, which this process does not have" was an inference from
source; it is now what a live gateway answers a live agent, in the gateway's own words.

### 5.3 The folder lane, which TC3e's rebuild unblocked

`client-e2e` (`🗑️generated/m10-client-e2e-1.txt`): **36/38**, against M9's 15/17-with-abort. Two
rows M9 left red are green and **neither is mine to claim** — they are TC3e's rebuilt components:

* `os: artifact_create (a real plugin artifact kind)` — the row whose abort destroyed the 38-row
  shape on M9's tree.
* `os: the snapshot shows the mutation` — **M8 §8 gap 3 / M9 §4 / CE1 gap 4**, open since 2026-09-20:
  "a committed guest transaction is not visible in that guest's own `ReadDocument`". It now reads
  `701 → 1361 base64 chars across the commit (pack 400 → 400, spr 300 → 960)`. M9 narrowed it to
  `PluginApp::document_pack()` inside the guest and said it needed "a `note` component rebuilt
  against the post-TC3b world"; that component exists now and the row is green. I did not fix it and
  do not claim it.

The two remaining reds are both other slices': `os: capability catalog health` (PZ1's) and
`os: inference_run` (`tool factory 'semio.infer' rejected …` — CE3/JB1's wfc lane).

`live-agent-loop-check` was **not run**. M9 measured 8/20 against another slice's shell on 6190 and
established that rows (f1)–(f8) fail on that shell's open-instance set rather than on the gateway;
re-running it against the same foreign shell would have reproduced a number that means nothing, and
starting a private `s` serve needed the wasm mutex (§5.1). Named, not papered over.

## 6. Gates

| gate | M9 | M10 | capture |
|---|---|---|---|
| `client-e2e` | 15/17, run ABORTS at `artifact_create` | **36/38** | `m10-client-e2e-1.txt` |
| `hub-agent-participant-check` (7621) | 14/17 | **14/17**, same three reds, byte-identical diagnostics | `m10-gate-participant-7621.txt` |
| `m10-write-path` (new, 7621) | — | **5/8** | `m10-write-path-7621.txt` |
| `cargo check -p semio-framework-os-mcp --all-targets` | 0 errors | **0 errors**, 16 lib / 83 lib-test warnings | `m10-mcp-check-{1..4}.txt` |
| `cargo test -p semio-framework-os-mcp --lib -- guest_backed backbone document_backbone a_routed_channel declared_inferences` | — | **11 passed, 0 failed** | `m10-guest-codec-law-1.txt` |
| `cargo check -p semio-framework-plugin-host --all-targets` | — | **0 errors** | `m10-plugin-host-check-{1,2}.txt` |
| `cargo test -p semio-framework-plugin-host --lib -- guest_runtimes_forwards` | — | **1 passed**, 169 s, real `note` component | `m10-plugin-host-law-1.txt` |
| `m10-write-path` after the codec landed (7621) | — | **5/8**, both reds now ONE cause (7621's 09-21 catalog) | `m10-write-path-7621-codec.txt` |
| `schema-mirror` | — | `exports=70 ajv-draft07-resolved=70` regenerated for the new `sessionDocument` field | `m10-schema-mirror.txt` |
| gateway `build` | exit 0 | **exit 0**, staged ×3 | `m10-mcp-build-{1..3}.txt` |

`hub-agent-participant-check` is unchanged at its 7621 ceiling, and that is the right result: all
three of its reds are stale hub ARTIFACTS (a hub binary built 09-21 03:42, a catalog published 09-21
05:49), which CE2 established by reproducing them byte-for-byte across a hub restart. Row 8
(`artifact_open` of a hub document) still passes, which is the regression check on §2's new bind
step. **CE3 owns running this gate on 7651; I ran it on 7621 only, and edited no gate file.**

Warning counts are quoted because zero errors alone proves nothing about whether a build actually
type-checked the changed code.

## 7. Honest gaps

1. **No agent has committed an edit to a hub document.** The bar is uncrossed. After this slice the
   distance is **not code**: it is a hub with a post-TC3b catalog that stays up and can mint a
   session. C8's 7671 published one and lived 11 minutes without being authenticable (§5.1); TC3e's
   7651 never came up. Everything downstream of the socket connect — the `Commands` frame, the
   ledger row, the `principalKind: agent` presence beat, a human browser seeing either — is
   untouched and unclaimed.
2. **The document socket has never CONNECTED.** The three legs are landed and unit-proven, and on
   7621 the codec chain is exercised live all the way into the component, but `connect_hub`,
   `finish_connect_hub`, the `Commands` send and the presence beat have not run once. Concretely
   unexecuted: the `Some(backbone)` arm of `relay_backbone_egress`, and every line of
   `open_hub_document_actor` after `ArtifactHost::open`.
3. **The guest-backed codec is partial, by the WIT's shape rather than by choice.** `interface
   codec` exports four functions; `store::ArtifactCodec` needs six. `compile_dsl` and
   `edit_text_from_envelope` refuse by name (§3.3.2 decision 3). A folder-bound workspace that
   somehow reached a guest-backed codec's text lane would get that refusal, not a `.dsl`/`.ops`
   file — which is correct for a hub binding and is a real limit for anything else.
4. **Route resolution is O(registered routes) guest calls in the worst case** (§3.3.2 decision 2).
   It is bounded by the number of distinct document kinds one agent session has open, and each
   candidate call is a pure export on a throwaway instance under `headless_codec_budget`
   (60 s) — but it is a loop over guest calls, and a reviewer should see it as one rather than as a
   map lookup.
5. **`artifact_open` of a hub document now fetches and compiles the package's component.** On 7621
   that is 47 466 541 bytes and a wasmtime compile, on the first open of a kind, cached after
   (`HubPluginComponents` by `(catalog generation, component sha256)`; `shared_compiled_component` by
   content hash). The tool used to answer in milliseconds. The cost is real, it is the price of
   deriving the codec from the code the hub authorized rather than trusting a number, and it is paid
   once per kind per process.
6. **The pack-schema-hash shortcut stays refused.** Registering `lease.artifact.pack_schema_hash`
   would make the socket connect today on any hub and would make `finish_connect_hub`'s
   `local_schema_hash == authority.pack_schema_hash` compare the hub's value with itself. Not done,
   and `register_guest_document_codec` takes that value only as a cross-check that refuses on
   disagreement.
7. **My own first draft of `relay_backbone_egress` deadlocked** — it called `session_artifact_for`
   while already holding the `plugin_artifacts` mutex, which `std::sync::Mutex` does not allow to be
   re-entered. It hung the first committed message and was caught by the law in the same change, not
   by a reviewer. Recorded because the shape (a helper that takes the same lock as its caller) is two
   lines apart in this file and will recur.
8. **`live-agent-loop-check` was not run** (§5.3): M9 measured 8/20 against another slice's shell on
   6190 and established the reds were that shell's open-instance set, not the gateway; re-running it
   against the same foreign shell would reproduce a number that means nothing, and a private `s`
   serve needed the wasm mutex (held by `c8` throughout).
9. **No browser run and no screenshot** (§5.1) — the same rule M9 applied. The two-browser rig needs
   a fresh-catalog hub first; with none, a human attached to an unchanged document beside an empty
   agent roster row would be theatre.
10. **Two rows went green that are not mine** (§5.3): `client-e2e`'s `artifact_create` and `the
    snapshot shows the mutation` (open since 2026-09-20 as M8 §8 gap 3 / M9 §4). TC3e's rebuilt
    components fixed them. I ran the gate; I did not fix the defect.
11. **`artifact_open`'s output grew a field.** `sessionDocument` is additive and the mirrors were
    regenerated (`schema-mirror`, exports=70), but any client pinning that tool's output shape sees a
    new key. It is there because a `writePath` an agent can read before spending a mutation is worth
    more than one it discovers when the edit goes nowhere.
12. **`🔌️plugin/🖥️host/🦀️.rs` is a widely-depended-on file.** Six crates depend on
    `semio-framework-plugin-host` (`🏃️run`, `🖨️describe`, `🌉️mcp`, the wgpu renderer, the os host,
    and the `gis` plugin), so this slice forces a rebuild for anyone compiling those.
    `semio-framework-plugin` — FP11's crate — is **not** among them and is unaffected.
13. **A discovery diagnostic found in passing, not chased**: the live gateway prints
    ``[mcp registry] skipping plugin `puzzle`: … 🔣️.json did not decode as a PackageDescriptor:
    missing field `artifactSchema` at line 1 column 6451``. A whole plugin dark in the catalog,
    adjacent to `client-e2e`'s `os: capability catalog health` red (PZ1's). Not mine, not
    investigated, recorded with the exact message.
14. **`declared_inferences_for_artifact` has a redundant predicate** —
    `item.artifact_schema == schema || item.artifact_schema == schema`
    (`💡️inference/🦀️.rs`), the same comparison twice. It is very likely meant to match
    `artifact_kind` as the second arm (the two vocabularies M9 §5.1 separates), which would change
    behaviour. Left alone rather than guessed at in a slice that could not exercise the per-artifact
    roster against a hub document.

## 8. Files changed

**Modified — plugin host** (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/`)

- `🦀️.rs` — `WasmtimeRuntime::{codec_print_mirror, codec_apply_ops}` (the compiled half's first
  implementation of either) and the `🗂️GuestCodecDispatch` region: all four `codec` exports
  forwarded on `GuestRuntimes`
- `🧪️tests/🔬️owned-instance-open/🦀️.rs` — the forwarding law, executed against the real
  `semio_s_plugin_note.wasm`

**Modified — os-mcp** (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/`)

- `🏠️workspace/🔗️remote/🦀️.rs` — `NativeHubBindingDriver::fetch_execution_target_lease`
- `🏠️workspace/🦀️.rs` —
  `SessionDocumentPair`; `PluginArtifactBinding` gains `surface_id` / `document` / `backbone` /
  `backbone_blocked_by` (and a hand-written `Debug` that prints document BYTE COUNTS, never bytes);
  `HeadlessWorkspace::bind_hub_session_document`; `HeadlessWorkspace::open_hub_document_actor`;
  `PluginArtifactChannel::{session_documents, backbone_egress, load_session_document,
  drain_backbone_egress}` + `discard_instance` forgetting both;
  `document_backbone_payload`; `RoutingArtifactChannel::{session_document_for,
  relay_backbone_egress}` + the seed and drain steps in `exchange`; `Drop` closes bound hub document
  actors
- `🏠️workspace/🦀️.rs` — region `🗂️GuestDocumentCodec`: `headless_codec_budget`, `GuestCodecRoute`,
  `guest_codec_routes`, `guest_codec_route_snapshot`, `guest_print_mirror`,
  `guest_apply_ops_binary`, `guest_compile_dsl`, `guest_edit_text_from_envelope`,
  `register_guest_document_codec`; and `HeadlessWorkspace::register_hub_document_codec`
- `🗿️artifact/🦀️.rs` — `artifact_open` binds the hub document and reports `sessionDocument`;
  `session_document_report`
- `🧬️schema/🦀️.rs` — `session_document_shape`, declared on `artifact_open_output_shape`
- `🧬️schema/🟦️.ts`, `🧬️schema/🔣️.json` — regenerated by `bun ./📜️script.ts schema-mirror`
- `💡️inference/🦀️.rs` — `owned_artifact_kinds`, `contributed_inference_descriptors`,
  `installed_descriptors_for_contributions`, and the union step in
  `declared_inferences_for_workspace`
- `🏠️workspace/🧪️tests/🔬️quick/🦀️.rs` — 4 new laws + 2 widened binding literals
- `💡️inference/🧪️tests/🔬️quick/🦀️.rs` — 2 new laws

**No hub Rust and no `🏪️store` change.** Nothing for HT16 to route. The `🔌️plugin/🖥️host` change
is additive (two new methods and one new `impl` block); see §7.12 for its rebuild blast radius.

**New — ticket artefacts**
- `🐍️m10-hub-write-path-probe.ts` — the live write-path probe (§5.2)
- `🐍️m10-inference-roster-probe.mjs` — reads a live gateway's `inference_list` roster (§4.3)
- `🗑️generated/m10-*` — `component-codec-export`, `inference-declarations`,
  `inference-roster-live`, `inference-law-1`, `backbone-law-1`, `mcp-check-{1..4}`,
  `mcp-build-{1..3}`, `schema-mirror`, `client-e2e-1`, `gate-participant-7621`,
  `write-path-7621`, `write-path-7621-codec`, `plugin-host-check-{1,2,3}`, `plugin-host-law-1`,
  `guest-codec-law-1`, `mcp-check-{5,6,7}`, `mcp-build-{4,5}`, `7671-provision`, `7671-auth-fault`,
  `delegate`, `agent-credential.json`

**No `.vscode/launch.json` row** — this slice registered no new permanent runnable command; both new
files are ticket-scoped probes.
