# 📓️ M8 — the MCP agent as a real third participant in a hub space

Slice M8 (session 7, 2026-09-21). Picks up exactly where **M6b §M6b.4 step 6** stopped: an agent
delegated over `POST /auth/agent-delegations` reaches the hub, but has never *edited a hub document*
and has never appeared on a *document presence roster*. Also closes **CE1 §8 gap 4** (the headless
lane's split artifact identity) and turns `features.mcpWorkspace` from a hard-coded `false`
(`🌎️hub/🏗️bootstrap/🦀️.rs:2449`) into a derived readiness.

Status legend: ✅ landed and executed live · 🚧 landed and compiled, not executed · ❌ not done.

## 0. Headline

**An MCP agent is now a real, legible, authenticated participant in a hub space that can READ the
space's documents — and still cannot write one.** Measured live, not reasoned about:

- `features.mcpWorkspace` is a derived readiness with a law instead of a hard-coded `false` (§4).
  **Compiled only** — rule 26 keeps me from the hub binary. **Needs coordinator hub rerun.**
- Two of the three gates between "agent principal" and "agent participant" are open and
  live-proven against a real `gis.map` hub document on hub 7621: `artifact_open` answers
  `kind=gis.map`, `artifact_snapshot` answers 80 801 real pack bytes off the digest-verified
  canonical pair. Both used to be flat refusals (§5.1, §5.2, §5.4).
- The third, **dispatch, is open and named**: a hub-bound gateway has no plugin wasm because the hub
  serves the execution target and the gateway never asks (§5.3, gap 1). No agent edit crossed, so no
  human saw one, and no screenshot is offered.
- The headless lane now has **one artifact identity** end to end, proven by a new required
  `client-e2e` row (§6). A second new row, "the snapshot shows the mutation", is **red** and names a
  deeper defect it uncovered (gap 3).
- New permanent gate `hub-agent-participant-check`, **14/17 green** live; both reds are gaps named
  here (§7).

## 1. Inherited state

Nothing: `ls 🗑️generated | grep -i m8` empty, no `📓️m8-*.md`, no `🐍️m8-*` probe. Context inherited
from reports only — M6/M6b (agent credential + delegation chain, 6/7 steps live), C3 (two humans on
one hub document, `features.mcpWorkspace: false` blocking step 10), CE1 (§8 gap 4 split identity),
S7, AP1, LB1.

## 2. The rig, and the first live measurement

### 2.1 Hub 7631

`os-hub` from `⚡️cache/hs1/os-hub-7611`, data root `.🧬semio/🌐hub/hs1-boot`, **pid 94972**, catalog
generation `94a9788a14060725869ab35b12b491b1d884d487bca3a1535ece346a2b757a71`, space
`01a0c00f-4f3c-7834-a7e6-2ccf9de925db` holding one `gis.map` document
`artifact-2fb248125b8b2b4d56de25933d30ed21` authored by `user1@semio.dev`
(`01a0c00d-cd33-7948-91cb-da23affa54ec`). `/readyz`:

```
"status":"ready" … "artifactAuthority":{"ready":true}
"features":{"openPlan":true,"openPlanExchange":true,"rebootstrap":true,"mcpWorkspace":false,"inference":true}
```

**Boot defect found on the way in (not fixed, not mine to fix):** the ticket's own
`🐍️ds1-hub-hold.ts` route died TWICE on this data root with
`Error: ArtifactAuthority(DeadlineExceeded)` before binding a port
(`🗑️generated/m8-hub-7631.txt`, first two runs). `configured_artifact_authority`
(`🌎️hub/🏗️bootstrap/🦀️.rs:588`) gives the whole trusted-catalog load a **fixed 30 000 ms** budget —
`OperationContext::new(started.saturating_add(30_000), …)` — and `hs1-boot`'s catalog is 612 MB. The
page cache was already warm (a full `cat` of the tree takes 0.79 s), so this is CPU contention under
the fleet, not I/O. The hub **exits** on it rather than reporting a closed readiness gate, which is
the opposite of every other gate's behaviour (`blocked_by` names a reason and the hub still binds).
Booting the same binary on the same data root in production posture
(`OS_HUB_MODE=production OS_HUB_ADMIN_SUBJECTS=credential:…`) succeeded in ~50 s. No hub code was
changed for this; it is recorded so the next owner does not read it as a corrupt data root.

### 2.2 Delegation — M6b's chain re-run into an OCCUPIED space

`🐍️m8-agent-delegate.ts` (`🗑️generated/m8-delegate.txt`). M6b's probe delegated into a fresh empty
space; this one delegates into the space a human already edits, because a third participant only
means something in an occupied room.

| step | observed |
|---|---|
| `POST /auth/sessions` as `user1@semio.dev` | `200`, `session.v1.…` |
| `POST /auth/agent-delegations` (audience `edit`, ttl 7200 s) | **`201`**, `delegationId 01a0c1aa-6c51-79a1-ad87-495302395219`, receipt `agentPrincipalId agent:01a0c1aa-…` |
| credential file `semio.hub.agent-credential/v1` | written, `mode=600` |

### 2.3 The gateway, live against 7631 — the measurement this slice exists for

`🐍️m8-hub-participant-probe.ts`, run 2 (`🗑️generated/m8-participant-run2.txt`), driving the staged
`semio-os-mcp` over stdio through `McpClientSession` — the same client machinery `client-e2e` uses.

| # | step | verdict | witness |
|---|---|---|---|
| 1 | `initialize` over `--hub --space --credential-file` | **PASS** | `server=semio-os-mcp@0.1.0` |
| 2 | `context_resolve` names the AGENT | **PASS** | `principal=agent:01a0c1aa-6c51-79a1-ad87-495302395219`, `channel=headless`, and — unprompted — `activeArtifactId=artifact-2fb248125b8b2b4d56de25933d30ed21`, the hub's own document |
| 3 | `semio://workspace/artifacts` lists the hub's documents | **PASS** | one row with `scope.documentId`, `descriptorDigestV1`, `descriptorResource`, `checkpointResource` |
| 4 | `artifact_open` of that hub document | **FAIL** | `PLUGIN_UNAVAILABLE: artifact \`artifact-2fb…\` is not the authenticated MCP probe schema` |
| 5 | `capabilities_search` over the HUB-selected catalog | **PASS** | 1 hit, `gis.s.gis.gismap@1/*#editor.addFeature`, `artifactKind s.gis.gismap` — the catalog is the hub's, not a local registry's |
| 6 | `action_prepare` of that verb | **FAIL** | `PLUGIN_UNAVAILABLE: repo root not found — cannot locate the plugin registry or compiled wasm` |
| 7 | `artifact_snapshot` of the hub document | **FAIL** | `PLUGIN_UNAVAILABLE: canonical artifact bodies remain unavailable until P4-B` |
| 8 | the hub's own document status, read back as the human | **PASS** | `HTTP 200 {"document_id":"artifact-2fb248…","head_seq":0,"commit_seq":0,"epoch":0}` |

**The headline of this measurement:** the agent is a real hub principal with the hub's own catalog
and the hub's own document list in hand, and then every tool that would make it a *participant* is
stopped by a refusal **inside the gateway**, not by the hub. Three distinct gates, named below.

## 3. The three gates between an agent principal and an agent participant

| # | refusal | site | what it means |
|---|---|---|---|
| **D1** | `artifact \`…\` is not the authenticated MCP probe schema` | `🌉️mcp/🏠️workspace/🦀️.rs:2306-2317` (`authenticated_probe_document_is_known`) | a hub workspace admits exactly ONE artifact schema, its own `os.agent.probe/v1`. Every real hub document is refused by kind. |
| **D2** | `repo root not found — cannot locate the plugin registry or compiled wasm` | `🌉️mcp/🏠️workspace/🦀️.rs:2251` sets `workspace.repo_root = None` for a hub binding; `open_plugin_artifact_channel` (`:1992`) then has nowhere to read a `.wasm` from | the hub SERVES the execution target it authorized — `POST /spaces/{s}/documents/{d}/execution-target/component` (`🌎️hub/🏗️bootstrap/🦀️.rs:9763`) — and the gateway never asks for it. A hub-bound agent can dispatch **no action at all**, for any plugin. |
| **D3** | `canonical artifact bodies remain unavailable until P4-B` | `🌉️mcp/🏠️workspace/🦀️.rs:2385-2389` | `read_artifact_bytes` has a hub arm that refuses by construction, in the same file that already owns `mount_canonical_pair` and `read_canonical_checkpoint`. |

## 4. Item 1 — `features.mcpWorkspace` is a real readiness ✅ (compiled; needs coordinator hub rerun)

### 4.1 Every reader, measured

`grep -rn "mcp_workspace\|mcpWorkspace"` over `*.rs`/`*.ts`/`*.tsx`/`*.json` outside the ticket tree:

| site | role |
|---|---|
| `🌎️hub/🏗️bootstrap/🦀️.rs:2391` | the field |
| `🌎️hub/🏗️bootstrap/🦀️.rs:2449` | **the only producer** — a literal `mcp_workspace: false` |
| `🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json:223,227` | declares it required + boolean |
| `🌎️hub/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json:119,132,146` | three dev-hub fixtures, all `false` |

**There is no reader anywhere.** By contrast its neighbour `open_plan_exchange` IS read, at
`🌎️hub/🏗️bootstrap/🦀️.rs:3003`, where it closes the open-plan exchange route. So the field was a
published fact with one possible value and no consumer — and C3 §2 row 10 is what a published fact
with no meaning costs: that slice read `false` on a hub that could in fact mint an agent session and
scored "AI agent as third participant" **not run** on the strength of it.

### 4.2 What it should mean, and how it is now derived

The two things a `semio-os-mcp --hub … --credential-file …` gateway asks this hub for, in order:

1. `POST /auth/agent-sessions` — exchange the human's delegation for the agent's own session. Only a
   directory backend that implements the delegation family can do it (sqlite does; postgres and
   neo4j carry AU1's erroring defaults, M6 gap 6).
2. an authenticated descriptor/catalog binding that resolves a document open target — exactly what
   `open_plan` already answers for.

`mcp_workspace_ready(agent_delegation_ready, open_plan_ready)` (`🌎️hub/🏗️bootstrap/🦀️.rs`, a `const
fn` next to the struct) is their conjunction. `agent_delegation_ready` is **a live probe of the real
method**, not a declared capability flag beside it:

```rust
let agent_delegation_ready = directory.list_agent_delegations(AGENT_DELEGATION_READINESS_PROBE_SCOPE, AGENT_DELEGATION_READINESS_PROBE_SCOPE, 1).await.is_ok();
```

one bounded read with a sentinel scope (`readiness-probe/mcp-workspace`, not a well-formed id, so it
can collide with no row in any data root). A backend carrying the erroring default answers
`Err(Backend)`; one that implements it answers the empty page. The field therefore cannot drift from
what the next `POST /auth/agent-sessions` would actually do.

### 4.3 The law

`mcp_workspace_readiness_is_derived_from_delegation_and_open_plan_never_constant`
(`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`) pins all four corners plus the served JSON — a constant of either
polarity fails at least two rows. 24 existing `hub_readiness(…)` call sites in that file were widened
by one positional argument.

**Verification:** `cargo check -p semio-hub --all-targets` → **0 errors** (67 pre-existing warnings,
`🗑️generated/m8-hub-check-1.txt`). Rule 26 forbids me building the hub binary, so the derived value
has **not** been observed on a live `/readyz`: hub 7631 runs the 20:53 binary and still publishes
`mcpWorkspace: false`. **Needs coordinator hub rerun** — and a rerun of `cargo test -p semio-hub`,
which is where this law's verdict lives.

## 5. Item 2 — the agent and a HUB document: two of three gates opened

`artifact_open` and `artifact_snapshot` of a real hub document are landed (§5.1, §5.2). The third,
dispatch (D2), is **not** — §5.3 says exactly why and what it needs.

### 5.1 D1 — a hub document is no longer refused by kind ✅

`authenticated_probe_document_is_known` (`🌉️mcp/🏠️workspace/🦀️.rs`) answered a retryable
`PLUGIN_UNAVAILABLE` for any hub document whose schema was not `os.agent.probe/v1`, which
`artifact_open_handler` (`🌉️mcp/🗿️artifact/🦀️.rs:239`) propagates before it ever reaches
`read_artifact_bytes`. A `gis.map` document is not a probe document — that is a **fact**, not an
error, and the predicate's own name is the contract. It now answers `Ok(false)` and the handler
falls through.

### 5.2 D3 — a hub document's bytes come off the verified canonical pair ✅

`read_artifact_bytes`' hub arm refused every known document with `canonical artifact bodies remain
unavailable until P4-B` — in the same file that already owns `mount_canonical_pair` and
`read_canonical_checkpoint`. New `NativeHubBindingDriver::read_canonical_pair_bytes`
(`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`) takes the SAME digest-verified mount `read_canonical_checkpoint`
takes and projects it to `(pack, spr)` instead of to the checkpoint's descriptive JSON;
`HeadlessWorkspace::read_hub_canonical_pair` is the one-line workspace-side call. The
`semio://artifact/{id}` and `semio://artifact/{id}/schema` resources answer for hub documents too —
the body from that mount, the schema from `DocumentDescriptor.artifact_schema`, the hub's own
authority over what the document is. `history` and `validation` stay typed gaps, and they are gaps in
the WIRE, not in this binding.

### 5.3 D2 — dispatch is still closed, and this is the whole remaining distance ❌

`HeadlessWorkspace::open_hub` sets `workspace.repo_root = None`, so `open_plugin_artifact_channel`
has nowhere to read a `.wasm` from and **every** `action_prepare` from a hub-bound agent answers
`repo root not found — cannot locate the plugin registry or compiled wasm`. This is not a missing
line; it is a missing chain. The hub already serves the execution target it authorized —
`POST /spaces/{s}/documents/{d}/open-plan` → `…/execution-target/{manifest,component,descriptor}` →
`…/socket-grants` → `…/socket/v1` (`🌎️hub/🏗️bootstrap/🦀️.rs:9760-9766`) — and the gateway asks for
none of it. Making the gateway a first-class replica means:

1. `POST …/open-plan` for the document, then `…/execution-target/component` for the authorized
   component bytes, cached by the catalog generation that authorized them;
2. instantiating that component through the existing `shared_compiled_component` path instead of
   `resolve_plugin_wasm_path(repo_root, entry)`;
3. `AppCommand::LoadDocument` of the canonical pair §5.2 now reads, so the guest's session document
   IS the hub document rather than the plugin's genesis;
4. `…/socket-grants` + the binary document socket as the store's backbone, so the committed ops
   leave as a `Commands` frame and the agent's presence beat carries `principalKind: agent`.

Steps 1–3 are mechanical against surfaces that exist. Step 4 is the one with a real unknown: the
MCP workspace's hub binding today pins `PersistenceBinding::Hub { surface: Some(PROBE_SURFACE_ID) }`
(`🌉️mcp/🏠️workspace/🦀️.rs:501`) and `ArtifactHost::open` needs a registered native codec for the
document's schema, which for `gis.map` this process does not have. **None of it is landed and none
of it is claimed.**

### 5.4 D1/D3 live-proven, and a client↔hub compatibility boundary found on the way

The gateway was rebuilt (`🗑️generated/m8-mcp-build-1.txt`, staged 10:31) and the probe re-run.

**On hub 7631 (the 20:53 binary) the rebuilt gateway no longer binds at all**:

```
[semio-os-mcp] acting as agent principal agent:01a0c318-… ("M8 participant agent") in space 01a0c00f-…
[semio-os-mcp] PreconditionFailed: hub directory response was invalid: execution-target response failed validation
```

exit 1, before `initialize`. The same source bound to the same hub fine at 03:50 with the Sep-20
23:37 binary. Between them a peer landed
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/**` (C5/JC1's jco-1.34
work), which widened the execution-target lease the client validates
(`DocumentOpenPlan::lease_fields(component_byte_length, descriptor_byte_length,
browser_actor_byte_length)` → `fields.validate()`), and the decode refusal is mapped at
`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:718`. **A gateway built from today's tree cannot bind to a hub
binary built before 2026-09-21 ~03:00** — the same 1.34 cut preamble rule 28 names for catalogs and
actors also cuts the MCP↔hub directory contract. Not fixed here (neither side is mine to roll back);
recorded so nobody reads it as a broken data root.

Re-run therefore went against **C5's hub 7621** (`jc1-boot`, generation `8086b61f336e08b5…`, binary
`target-jc1/debug/os-hub` — read only, C5 owns its lifecycle), space
`01a0c314-e41f-780d-a980-3adda40ca9f7`, document `artifact-0954e2d10d8fff9605f101b0dba34f3b`
(`gis.map`), both humans authors. Capture `🗑️generated/m8-participant-run4-7621.txt`:

| # | step | before M8 | after M8 |
|---|---|---|---|
| 4 | `artifact_open` of the hub document | `PLUGIN_UNAVAILABLE: … is not the authenticated MCP probe schema` | **PASS** — `kind=gis.map`, `sizeBytes=81038` |
| 7 | `artifact_snapshot` of the hub document | `PLUGIN_UNAVAILABLE: canonical artifact bodies remain unavailable until P4-B` | **PASS** — `packBytes=80801 sprBytes=237` |

Those are the two gates that were closed, closed against a real hub document, read off the
digest-verified canonical pair. Row 6 (`action_prepare`) is still D2's
`repo root not found — cannot locate the plugin registry or compiled wasm`.

**Two further live-only findings, neither fixed:**

1. **The hub document's kind and the catalog verb's kind are spelled differently.**
   `artifact_open` answers `gis.map` (the hub descriptor's `artifact_schema`); the only mutation the
   hub-selected catalog offers is `gis.s.gis.gismap@1/*#editor.addFeature` with
   `artifactKind = s.gis.gismap`. An agent cannot match a verb to a hub document by kind, and the
   probe's row 5b is red on exactly that. Whether `gis.map` is the pack schema and `s.gis.gismap`
   the artifact-kind id — i.e. two vocabularies that must be related by a lookup nobody wrote — is
   the question for whoever owns the catalog↔descriptor seam.
2. **A retryable binding state kills the process at startup.** One gate run died with
   `PluginUnavailable: authenticated hub descriptor index is refreshing; retry after authority
   refresh`, exit 1, before `initialize` (`🗑️generated/m8-gate-run4.txt`). The condition is marked
   retryable by its own constructor and the next run succeeded, but nothing retries it: a client
   whose hub happens to be mid-refresh sees the server die.

## 6. Item 3 — one artifact identity in the headless lane (CE1 gap 4)

### 6.1 What was wrong

`PluginArtifactChannel`'s `ReadHistory` arm stamped `artifact_id: self.entry.plugin_id.clone()`, so
in the headless lane `RevisionStamp.artifactId` was `"note"` while `artifact_create`/
`artifact_snapshot` addressed `"…-typed"` — two ids for one document. `client-e2e` had a nine-line
comment explaining that it deliberately does **not** follow `revisionAfter.artifactId` because
snapshotting it answers `no such artifact: note`.

### 6.2 The fix

- `HeadlessWorkspace.plugin_artifacts` became `Arc<Mutex<…>>`, shared with the workspace's own
  `RoutingArtifactChannel` — one map, not a copy.
- `RoutingArtifactChannel::session_artifact_for(plugin_id)` is the inverse lookup: the artifact this
  workspace bound to that plugin, `None` when none — or when more than one, because no single stamp
  could then name either truthfully.
- It is re-read on **every** exchange, not once at open: `artifact_create` binds the artifact AFTER
  the channel that seeded it from the plugin's genesis was already opened, so an open-time snapshot
  would name nothing for the very first mutation.
- `PluginArtifactChannel::stamped_artifact_id()` answers that artifact, or `plugin:<id>` when there
  is none — a form a client cannot mistake for an artifact id and pass back into
  `artifact_snapshot`. **No alias**: the bare plugin id is never stamped again.
- `ActionAdapter::read_session_artifact(instance)` + `HeadlessWorkspace::bind_root_action_adapter`
  let `read_artifact_bytes` read the LIVE guest session the mutation went to instead of the row
  frozen at create time.

Law: `a_routed_channel_resolves_the_artifact_its_plugin_session_document_is`
(`🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`) — nothing bound → `None`; one bound → that artifact; a
second artifact on the same plugin → `None` again.

### 6.3 `client-e2e`, measured

`🗑️generated/m8-client-e2e-1.txt`, **35/38** (was 34/36; two rows are new, both mine).

- ✅ **`os: the prepared revision names the ARTIFACT, not the plugin`** — new required row, **PASS**:
  `revision.artifactId=mcp-client-e2e-muaztbyr-typed`, `artifact_create` used the same id, pinned
  plugin is `note`. Every later stamp in the run carries it too —
  `action_invoke … mcp-client-e2e-muaztbyr-typed@/0 → …@transaction:txn_4c27…/1`, `live head
  advanced`, `history_undo`, `history_redo`, `transaction_rollback`. The nine-line "this field is
  the plugin id" comment is gone, and `resources/subscribe`/`notifications/resources/updated` now
  ride the same id. **`artifact_create → artifact_open → action_prepare → action_invoke →
  artifact_snapshot → history_undo/redo → transaction_*` all address one artifact id.**
- ❌ **`os: the snapshot shows the mutation`** — new required row, **FAIL**:
  `mcp-client-e2e-muaztbyr-typed is byte-identical across the commit (400 base64 chars)`. The live
  read IS being taken — `artifact_snapshot` answers `packBytes=299 sprBytes=718` where the frozen
  row was `sizeBytes=522` — but the `note` guest's own `ReadDocument` returns identical bytes on both
  sides of an `addBlock` whose head demonstrably advanced (`@/0` → `@transaction:txn_4c27…/1`) and
  whose undo/redo both work. So the remaining defect is **not** "the snapshot reads a frozen row"
  any more; it is that a committed guest transaction is not visible in that guest's `ReadDocument`
  answer. That is a new, measured, unresolved finding and the row is left red naming it rather than
  loosened.
- The two other reds are pre-existing and not mine: `capability catalog health` (4 diagnostics,
  `puzzle` — CE1 gap 1) and `inference_run` (`🀄️wfc`'s own
  `job.explicit-state-machine-required` — CE1 gap 3 red 2).

## 7. Item 5 — the permanent gate `hub-agent-participant-check`

`live-agent-loop-check` owns the shell route; this owns the other one. New files:
`🌉️mcp/🧪️tests/🤖️hub-agent-participant/🟦️.ts` (the 17-row chain) and
`…/🏃️execution/🟦️.ts` (`OsMcpHubAgentParticipantScript`), registered in
`🌉️mcp/📦️packages/🦀️rust/📜️script.ts`, an nx target in that package's `📋️project.json`, and
`.vscode/launch.json` row `⚖️gate🌉️os-mcp🌎️hub-agent-participant` at order `411.107585`, directly
under `⚖️gate🌉️os-mcp🤖️live-agent-loop`. It boots nothing (`OS_MCP_HUB_ORIGIN`, `…_EMAIL`,
`…_PASSWORD`, `…_SPACE`) and names the origin it looked for when none answers.

**Executed**, against live hub 7621 (`🗑️generated/m8-gate-run5.txt`):

```
hub-agent-participant-check: 14/17 rows green against http://127.0.0.1:7621
```

Green: `/readyz`, the feature declaration, sign-in, space authorship, `POST /auth/agent-delegations`
→ 201, the agent principal being distinct from its human, the 0600 credential file, the gateway
serving over the delegated session, `context_resolve` naming `agent:<delegationId>` exactly, the
agent seeing the space's documents, `artifact_open` (`kind=gis.map sizeBytes=81038`),
`artifact_snapshot` (`packBytes=80801`), the hub's own catalog answering `capabilities_search`, and
`DELETE /auth/agent-delegations/{id}` → 204.

Red, and each is exactly one of this report's named gaps:
- **0c** `features.mcpWorkspace is true` — 7621 runs a binary predating §4's change. Goes green on
  the coordinator's hub rerun; it is the live check for item 1.
- **11/12** `action_prepare` / `action_invoke` — D2 (§5.3).

Three defects in the gate itself were found by running it and fixed: a counted `..` chain that
walked past the repo root (now located by `.mcp.json`), reading `hits` where
`capabilities_search_output_shape` (`🌉️mcp/🧬️schema/🦀️.rs:340`) declares `results`, and the
transient §5.4(2) startup race.

## 8. Honest gaps

1. **The agent still cannot EDIT a hub document.** D2 (§5.3) is open: `action_prepare` from a
   hub-bound gateway answers `repo root not found`. Everything downstream of it in the brief — the
   `Commands` frame on the document socket, the hub ledger row, a second MCP client reading the
   agent's edit back, the `agent` presence row, and therefore the human-browser half on 7621 — is
   **not done and not claimed**. There is no agent edit for a human to see yet, so no screenshot was
   taken; taking one of anything else would have been theatre.
   Concrete next step, measured: the Rust `DirectoryClient` has
   `document_execution_target_manifest` and `document_execution_target_descriptor`
   (`📇️directory/🔌️client/🦀️.rs:1043,1069`) and **no** `…/execution-target/component` method at all,
   although the hub serves that route (`🌎️hub/🏗️bootstrap/🦀️.rs:9763`). That one client method, plus
   instantiating the returned bytes through the existing `shared_compiled_component` path, is what
   unblocks steps 1–3 of §5.3.
2. **`features.mcpWorkspace` has never been observed true on a live `/readyz`.** Rule 26 forbids me
   building the hub binary. `cargo check -p semio-hub --all-targets` is 0 errors and the law is
   written, but its verdict and the served value both need the coordinator. **Needs coordinator hub
   rerun.** The unverified-live hub change is exactly: `mcp_workspace_ready`, the
   `list_agent_delegations` startup probe, the widened `hub_readiness` signature, and the new law.
3. **A committed guest transaction is not visible in that guest's own `ReadDocument`** (§6.3). The
   snapshot now reads the live session rather than a frozen row — demonstrably, by byte count — and
   is still byte-identical across a commit whose head moved. Root cause unknown; the `client-e2e`
   row is red and names it.
4. **The MCP↔hub directory contract has a hard version boundary at 2026-09-21 ~03:00** (§5.4). Every
   measurement in §5.4 onward is against 7621 for that reason. Hub 7631 is up and healthy
   (pid recorded in `🗑️generated/m8-hub-7631.txt`) but unusable by a gateway built from today's tree.
5. **The hub document kind and the catalog verb kind are spelled differently** (§5.4.1) — `gis.map`
   vs `s.gis.gismap`. Not investigated beyond measuring it.
6. **`.mcp.json`'s `semio` entry cannot express a hub-bound agent**: it hard-codes `--folder .`, and
   `--folder`/`--hub` are mutually exclusive (`🌉️mcp/🏗️bootstrap/🦀️.rs:108`). A delegation UI must
   print a whole different argv, not an extra flag. M6b §4.4's scope gap is adjacent and untouched.
7. **The hub aborts on a fixed 30 s trusted-catalog load budget** (§2.1) instead of reporting a
   closed gate. Measured twice on a 612 MB warm catalog under fleet load. Not fixed — it is a hub
   change and I could not have verified it live anyway (gap 2).
8. **No unit law covers `read_hub_canonical_pair` or `read_canonical_pair_bytes`.** Both are proven
   by the live rows in §5.4 and by the gate, not by a test — they need a hub transport to exercise.

## 9. Files changed

**New**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🤖️hub-agent-participant/🟦️.ts` — the 17-row gate
- `…/🤖️hub-agent-participant/🏃️execution/🟦️.ts` — `OsMcpHubAgentParticipantScript`

**Modified — hub** (all unverified live; needs coordinator hub rerun)
- `🌎️hub/🏗️bootstrap/🦀️.rs` — `AGENT_DELEGATION_READINESS_PROBE_SCOPE`, `mcp_workspace_ready`, the
  `agent_delegation_ready` parameter on `hub_readiness`, its derivation from a live
  `list_agent_delegations` probe at startup, the derived `features.mcpWorkspace`
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — the new law + 24 widened `hub_readiness` call sites

**Modified — os-mcp**
- `🌉️mcp/🏠️workspace/🦀️.rs` — `authenticated_probe_document_is_known` answers a fact not an error
  (D1); the hub arm of `read_artifact_bytes` and of the `semio://artifact/{id}[/schema]` resources
  (D3); `read_hub_canonical_pair`; `plugin_artifacts` shared by `Arc`; `RoutingArtifactChannel`'s
  `session_artifact_for` + the per-exchange rebind; `PluginArtifactChannel.session_artifact_id`,
  `bind_session_artifact`, `stamped_artifact_id` and the `ReadHistory` stamp; `root_actions`,
  `bind_root_action_adapter`, `read_live_session_artifact_bytes`
- `🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` — `NativeHubBindingDriver::read_canonical_pair_bytes`
- `🌉️mcp/🔀️dispatch/🦀️.rs` — `ActionAdapter::read_session_artifact`
- `🌉️mcp/🦀️.rs` — binds the root adapter into the workspace
- `🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs` — the session-artifact law + 4 widened call sites
- `🌉️mcp/🟦️.ts` — `client-e2e`'s two new required rows and the pre-mutation snapshot
- `🌉️mcp/📦️packages/🦀️rust/📜️script.ts` + `📋️project.json` — the `hub-agent-participant-check` verb
- `.vscode/launch.json` — `⚖️gate🌉️os-mcp🌎️hub-agent-participant` (re-added after a peer overwrote it)

**Compile gates:** `cargo check -p semio-hub --all-targets` **0 errors**
(`m8-hub-check-1.txt`); `cargo check -p semio-framework-os-mcp --all-targets` **0 errors**, 75
warnings (`m8-mcp-check-4.txt`); `bun ./📜️script.ts build` for the gateway, exit 0
(`m8-mcp-build-1.txt`).

**Ticket artefacts:** `🐍️m8-agent-delegate.ts`, `🐍️m8-hub-participant-probe.ts`, and
`🗑️generated/m8-*` (hub log, 2 delegation captures, 4 participant runs, 5 gate runs, 4 compile/build
captures, 1 `client-e2e` capture).
