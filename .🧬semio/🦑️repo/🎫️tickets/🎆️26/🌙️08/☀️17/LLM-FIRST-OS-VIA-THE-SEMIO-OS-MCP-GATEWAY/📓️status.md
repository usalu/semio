# 📓️ Status — coordinator log (sol only, append-only)

## 2026-08-17 — W0 open

- Ticket opened: `🎫llmfirstosviathesemioosmcpgateway`, goal `🎯r2602🎯runningsketchpad`, issue #2568. Registry `llm` enum has no `opus-5`; the coordinator model is Claude Opus 5, recorded in the ticket prompt (`llm` field set to `sonnet-5`, the executor model).
- Plan of record copied to `📋️master.md` (source: `/Users/ueli/.claude/plans/we-want-our-os-iterative-tower.md`).
- **Disk**: 339 GiB free. Nothing deleted.
- **Peer-ticket state read before any dispatch** (`MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️status.md`):
  - `A1-actor` **done and coordinator-verified** (52 tests pass, purity constraint verified). `🎭️actor` crate exists and is a root `Cargo.toml` member.
  - `A3-kernel-types` landed additively — verified directly by me in the working tree: `🎠️kernel/🦀️component.rs` L934+ `//#region 🔖️Broker` (`CapabilityId` L944, `CapabilityRequest` L954, `BrokerCapabilityGrant` L971, `QuotaSchema` L998, `BrokerHooks` L1086) and `🛂️manifest/🦀️component.rs` L3710+ `//#region 🔖️PackageDescriptor` (`ExecutionMode` L3728, `ExtensionPointDeclaration` L3744, `ContributionSet` L3802, `PackageDescriptor` L3834). Our catalog compiler and policy engine build on these **now**.
  - `A2-abi-sdk`, `B1-host-native`, `A4-channel` in flight → their G1 not reached. Everything of ours that touches the plugin ABI, the channel or the plugin host waits.
  - Their hard-won lesson, adopted verbatim into our `📌️important.md`: **executors must run builds in the foreground**; background jobs do not survive a subagent turn.
- Concurrency budget: they have 4 live executors → we dispatch **read-only luna audits first**, then ≤ 2 building terra packets.

### W0 dispatch

Six luna audits (Haiku 4.5, read-only, one output file each):
`L0-actions`, `L0-shellstate`, `L0-hub`, `L0-testinfra`, `L0-mcpspec`, `L0-channel`.

### W0 complete — six luna audits landed (3 742 lines)

`📓️luna-{actions,shellstate,hub,testinfra,mcpspec,channel}-audit.md`. Findings that changed the plan are recorded in `📓️design-decisions.md` (D1–D8) and **override `📋️master.md`**. The four that mattered most:

1. **MCP 2026-07-28 has no `initialize` handshake** — it is stateless, per-request `_meta` protocol version, `server/discover` MUST. I verified this against the spec page myself rather than accepting the audit's summary. The installed SDK 1.30.0 (`dist/esm/types.js:2-4`) tops out at `2025-11-25`, i.e. **every client we can actually serve today is legacy-era**. The gateway is therefore **dual-era**, which the spec itself specifies and its compatibility matrix requires. Recorded as D1 with the explicit note that this is not a CLAUDE.md compat shim and must not be "cleaned up".
2. **There is no `extrude` action in cad** (only an engagement fixture). The demo action is `translateSelection` (dx/dy/dz). `🗒️note`'s 36 actions declare **no args at all**, so note cannot be the first agent-invocable demo. D2.
3. **14 action ids are declared by more than one plugin** → the `<plugin>.<app>.<action>` capability grammar is mandatory, not cosmetic. D3.
4. **Hub agent principal deferred** (D7): `FINISH-HUB-SPACES-COLLABORATION-END-TO-END` is actively rewriting `resolve_auth` and 23 routes. The gateway needs nothing from the hub to work — loopback token auth + local audit lane — so P4 waits rather than fighting a live rewrite.

### W1 — P1a and P9 accepted (coordinator-verified, not taken on the executors' word)

Both packets were written entirely in new, uncontested directories, so neither could damage a peer session. Both finished blocked on the same registrar action; I applied both leases:

- root `Cargo.toml`: members `🖥️shell` and `🌉️mcp` + both `[workspace.dependencies]` aliases; root `package.json`: the `🖥️shell` TS workspace entry.
- `cargo metadata --no-deps` exit **0** immediately after — the workspace stayed valid for every other live session, which is the only thing a root-file edit can break for them.

Verified by me in the real workspace (the executors could only build out-of-tree before the lease):

| check | result |
|---|---|
| `cargo test -p semio-framework-os-mcp` | **34 passed, 0 failed** |
| `cargo test -p semio-framework-os-shell` | **10 passed, 0 failed, 1 ignored** (the ignored one is the fixture *writer*, correct) |
| `cargo build -p semio-framework-os-mcp -p semio-framework-os-shell` | exit 0, **0 warnings** |
| dual-era runtime smoke on the real binary | see below |

Runtime behaviour confirmed with the actual binary, per CLAUDE.md (never claim a feature works without observing it):
- modern `server/discover` + `_meta` version `2026-07-28` → result echoes `2026-07-28`;
- legacy `initialize` with `2025-11-25` → result echoes `2025-11-25` (so the installed SDK client will negotiate);
- version `1900-01-01` → `-32022` with `data.supported = ["2026-07-28","2025-11-25","2025-06-18"]`.

`P1a` published the `GatewayBackend` seam with a `NullBackend`, and has **zero** dependency on `semio-framework`/kernel/plugin/channel/actor — verified against its `[dependencies]` table. That is what lets every later packet proceed while the microkernel ticket is still mid-rewrite.
`P9` delivered a 63-variant `ShellCommand`, a pure `reduce`, a TS twin, and **75 shared fixtures executed by both** the Rust and the TS test suites — the mechanism that keeps the twins from drifting.

### W1 wave 2 — P1b, P3, P5 accepted; two tree-level defects found and fixed by me

| packet | result |
|---|---|
| `P1b-http-handles-bridge` | Streamable HTTP (axum), handle table, audit lane, shell-bridge codec + TS twin. **77/77** tests (P1a's 34 preserved), 0 warnings, real bind-and-curl smoke incl. an evil-`Origin` rejection, and 20 bridge fixtures proven byte-identical across Rust and TS. |
| `P3-manifest-schema` | `ArgSchema`/`ActionSemantics` regions; `ActionArgDef.control` **field → derived method**; `record_spec_json_schema` over all 26 `Shape` variants. `semio-framework` 153 tests, `semio-framework-os-kernel` 1011 tests, both green. |
| `P5-conformance-tests` | 26/26, driving the real binary with the real `@modelcontextprotocol/sdk` client — the first *independent* proof, not the server agreeing with itself. |

P1b's most valuable output was not code: it fetched the live Streamable-HTTP spec and found that `2026-07-28` **removed** the GET stream, sessions and `Last-Event-ID` resumability — those are legacy-era only. It implemented them on the legacy path and documented the split rather than inventing a hybrid.

P5 flagged, loudly and correctly, that the `tools/call` "tool error vs protocol error" distinction is only half-observable today because the shipped binary registers zero tools. That is honest reporting of a coverage gap, not a failure; P2 closes it.

**Two defects I found by verifying rather than accepting the reports:**

1. **We broke the wgpu renderer.** P3 correctly refused to edit `Shell/🧊️component.rs` (registrar-only, contested by their H3) and leased 6 sites — but the lease left the tree broken, because the `control` field it reads no longer exists. I applied all six `arg.control` → `arg.control()` edits myself and then swept the whole tree with a regex for *any* surviving field access: **none remain** (every other `.control` hit is an unrelated type — `controller_id`, `theme.control_height`, `Tree item.control`, `engagement.control`). A lease that leaves the tree red is not a completed handoff; that is on me as registrar, and it is now closed.
2. **A3's `BrokerCapabilityGrant` blocked typegen for both tickets.** It derives `TS` but its `token: CapabilityToken` is a `u128` newtype with no `TS` impl. P3 called this "pre-existing"; it is not — the type is old, the *requirement* is new, from A3's Broker region (L972). I reproduced the exact `E0277` myself, then fixed it with the repo's own convention for u128 mirrors (`#[cfg_attr(feature = "typegen", ts(type = "string"))]`, as `PluginDependency.version` does).

**A false green I caught in my own verification:** my first typegen run reported exit 0 while printing `154 filtered out` — it had matched zero tests, because the real export test is `exports_typescript_bindings` under project `@semio-tech/framework-rs`, not a test named `typegen`. Re-run correctly: `bun nx run @semio-tech/framework-rs:generate` exits **0**, and `🤖️generated/🟦️manifest.ts` now carries `ArgSchema`/`ArgPresentation`/`ActionSemantics`/`CapabilityEffects` with `ActionArgDef.control` gone from the mirror. An exit code is only evidence if you also check what actually ran.

**Note for `ticket_close`:** `**/🤖️generated/` is gitignored repo-wide (`.gitignore:87`), so every generated mirror is a build artifact — they must NOT appear in the closing file list or they are silently dropped.

**Peer-ticket attribution, settled with evidence rather than assumption:** `cargo check --workspace` fails with ~109 errors, all inside `🔌️plugin/🖥️host/🦀️component.rs` (WIT bindgen / `semio::framework::*` unresolved). That file was committed at 22:47 today — after our ticket opened — and is modified in the working tree right now; it contains **zero** references to `ActionArgControl`/`.control`/`ArgSchema`, so our change cannot be its cause. It is their B1 packet mid-flight. Our crates that do not depend on plugin-host all build clean.

### W1 wave 3 — P2-catalog accepted; **GA reached**

`P2-catalog` compiled the first real capability catalog: 91 capabilities from genuine `cad` (40 actions) and `note` (35 actions) manifest data, deterministic (byte-identical blake3 across two compiles), zero conformance findings, 115/115 Rust tests, 26/26 TS conformance still green.

Verified by me against the running binary, not from the report:
- `tools/list` returns **20 tools** — the real, stable surface (`capabilities_search`, `capabilities_describe`, `context_resolve` live; the other 17 declared and returning a structured `PLUGIN_UNAVAILABLE` tool-error until P6/P7 land, which is what finally lets P5 exercise the tool-error path it correctly flagged as unobservable).
- `capabilities_search{"query":"move the selection"}` → `cad.editor.translateSelection` at score 6.62, top of five hits. **D2 satisfied.**
- The 2 remaining build warnings are in `📡️spr/📡️wire/🦀️component.rs` (`semio-framework-os-kernel`, peer-owned), not in our code — P2's "0 warnings from owned code" checked out.

**Eval: 50.0 % top-1, 73.5 % top-3 over 68 en/de cases.** P2 measured this honestly instead of tuning fixtures to flatter the number, which is the right call and the reason the figure is trustworthy. It is also well below the ≥0.8/≥0.95 target in `📋️master.md` §5, and the cause is structural rather than algorithmic: **no real plugin action carries `use_when` or a description yet** — the enrichment packets (P13/P14) are what populate the fields BM25 is supposed to rank on, and they are gated on the peer ticket's G3. Ranking quality is therefore a *data* problem to re-measure after enrichment, not a search-implementation problem to fix now. Do not tune the scorer against the current empty-metadata corpus; that would overfit to the absence of data.

**GA gate: met.** Gateway crate green; dual-era conformance proven with the real SDK client and with raw modern JSON-RPC; catalog deterministic; shell SSOT landed with Rust↔TS fixture parity.

### What is gated, and on what

Everything remaining touches surfaces the peer ticket is actively rewriting, so it waits on their gates rather than racing them:
- **their G1** (A2/A4/B1 land; `cargo check --workspace` green again) → `P6-actions-policy` (prepare/preview/commit over channel v12), `P7-headless` (Kernel + WasmtimeRuntime workspace), `P8-spi` (agent contributions in the plugin SDK — must land in the window *before* their W3 freezes the SDK).
- **their G2** (H1/H3 shells land) → `P10-react-shell`, `P11-wgpu-shell` (adopt `ShellState`/`ShellCommand`, mount `AgentBridge`), then `P12-e2e`.
- **their G3** (plugin crates unfrozen) → `P13`/`P14` enrichment, then `P15` re-measures the eval.
- **independent of them**: `P4-hub-agent`, still deferred per D7 until the live hub tickets close.

## 2026-08-18 — W2: their G1 met, our mutation/headless/shell wave

Re-checked their gates before dispatching anything: `cargo check -p semio-framework-plugin-host --lib` **finishes clean**, `CHANNEL_VERSION` is **12** (A4 landed), `✏️s/🔌️plugins/🗒️note/🔣️descriptor.json` is a real committed descriptor, and their status records **W2 complete** (H1–H4). So G1 and G2 are both effectively met and P6/P7/P8/P10 were dispatched.

### The convergent blocker, and why it was mine

P6, P7 and P8 all finished blocked on the **same** 4 type errors in `🗂️catalog/🦀️component.rs`, each correctly refusing to fix a file outside its scope and each filing a lease. Three independent confirmations is a strong signal, and it was right: their E1 had given `ContributionSet`'s four contribution categories their real typed shapes (exactly the follow-up A3's report predicted), so P2's untyped-`DescriptorEntry` helper no longer type-checked.

I fixed it rather than round-tripping it, and took the opportunity the type change created: a `ContributionRow` trait implemented once per category, so each row now yields a genuinely descriptive capability (an inference names its schema and artifact kind; an io entry reads `Import x as y` from `owner`/`counterpart`/`direction`) instead of the old `"{category} contribution from {plugin}"` placeholder. Two traps on the way, both caught by compiling rather than assuming: there are **two** different `IoEntryDescriptor` types in the tree (`🚪️io`'s is `from`/`into`/`fidelity`; `🛂️manifest`'s — the one `ContributionSet` actually uses — is `owner`/`counterpart`/`direction`), and `ArtifactDialect`'s field is `artifact_kind`, not `kind`.

### Five real defects, executable for the first time

With the crate compiling, the suite ran and reported **151 passed, 5 failed** — the first honest signal any of that code had ever had. Both agents diagnosed root causes properly instead of adjusting assertions:

- **P6** proved the "schema validation is broken" hypothesis *wrong* by driving the real compiled schema through a real validator: `translateSelection`'s fixture args never call `.required()`, so `{}` is genuinely valid and the **test** encoded the wrong expectation. It rewrote the test to use a real type violation and added two regression tests. Its second bug was real: `invoke()` revoked the one-shot prepared handle on every call including cache hits, so an idempotent replay failed with `NOT_FOUND` before the idempotency store was consulted — fixed by moving the whole body inside the idempotency closure.
- **P7** found that the writer (`open_probes`, in-memory) and the readers (cold `FolderSqliteStorage`) were never the same data source, and — the good one — that `a_headless_commit_propagates…` failed with `Closed`, not a timeout, because the test called `subscribe()` **before** `open()`, which by that API's own contract yields an already-dead channel. Reordering fixed it; it explicitly did not paper over a `Closed` by extending a deadline.

### An interop bug the conformance suite earned its keep on

After relocating an `ajv` import to satisfy a real policy finding, the SDK suite went red — and the failure was genuine: **the official MCP client could not parse our `tools/list` at all**. Two distinct defects, both fixed at the one choke point every tool passes through (`ToolRegistry::register`), so no future tool can reintroduce either:
1. `schemars` emits the bare boolean `true` for a free-form `serde_json::Value` field. That is legal JSON Schema but the SDK's Zod model requires object sub-schemas — `$ZodError` on `action_prepare.outputSchema.properties.preview`, rejecting the whole response. Now normalised to `{}` / `{"not": {}}`, with the boolean-*keyword* list (`additionalProperties`, `uniqueItems`, …) explicitly excluded so a flag is never mistaken for a schema.
2. `schemars` 0.8 emits **draft-07**, while MCP requires 2020-12. Converted properly — `definitions` → `$defs` and every `#/definitions/` ref repointed — not relabelled. All **25** published schemas now declare 2020-12 with zero stale refs.

P6 independently hit defect (1) too, and separately caught that **Nx had served a cached green against a stale binary** — worth remembering: `--skip-nx-cache` whenever a test result is being used as evidence.

### State

- `cargo test -p semio-framework-os-mcp` → **160 passed, 0 failed, exit 0**; TS conformance → **22/22**.
- The empty-doctest rustdoc step was disabled with its reasoning recorded: the crate has zero `` ``` `` examples (verified), so it only re-linked dependency rlibs — and with several sessions rebuilding `semio-framework` concurrently, that link intermittently failed on an rlib replaced mid-run. Two consecutive runs named *different* missing rlibs, which is what identified it as a concurrency artifact rather than a defect.
- **Registrar work applied**: `dev mcp stdio os` / `dev mcp http os` routed in root `📜️script.ts` (proven end to end — the router returns a correct `server/discover`); `.mcp.json` **and all five IDE mirrors** consolidated to `repo` + `semio` with the six legacy neo4j entries removed (each mirror keeps its own repo client slug: cursor/copilot/kiro/codex); launch seed gained `🛠️dev🌉️os-mcp🧵️stdio`, `🛠️dev🌉️os-mcp🌐️http`, `🖱️mcpinspector🌉️os` and the compound `🧭️compound🖥️s⚛️react🌉️os-mcp`, regenerated into `launch.json` (255 configs, valid).
- I corrupted `.codex/config.toml` mid-edit (my insertion point landed inside an array literal) and repaired it by hand from `git show HEAD:` — `git checkout` is forbidden here, so reading the committed blob and rewriting the file was the only safe route.
- **P10** delivered `AgentBridge`/`AgentPresence`/`AgentApprovals` (51 tests) importing — not reimplementing — both the bridge codec and the shell reducer. I applied its `ShellHost` mount lease and verified the React suite is **11 failed / 325 passed**, byte-identical to the pre-change baseline, with **zero** agent-related failures among them.

### Live end-to-end proof of the safety gate, and one real runtime gap

Drove the real binary against a real folder space (not a fixture, not a unit test):

- `context_resolve` → a genuine session handle, catalog hash and principal.
- `action_prepare` on `cad.editor.translateSelection` with **no scopes** → `isError: true`, `PERMISSION_DENIED`, message `principal agent:local lacks required scope documents.write`. The policy gate works end to end through the real MCP surface, and it names the missing scope rather than failing opaquely.
- The same call **with** `--scopes artifact.read,artifact.write` gets past policy and then fails at `wasmtime: component imports instance wasi:io/poll@0.2.9, but a matching implementation was not found in the linker` — a real gap: the headless workspace's linker does not provide the WASI p2 imports that a `wasm32-wasip2` plugin component needs. Sent back to P7 (as P7b) with the reproduction, and with an explicit instruction to determine whether the fix is adding WASI to our linker or routing through the plugin host's existing linker construction — duplicating that setup would be exactly the divergence CLAUDE.md forbids.

`P1c` is in flight to mount the bridge WebSocket route for real: P10 found (and I confirmed at `🌉️mcp/🦀️component.rs` ~L664/L705) that `run_http` never served `/bridge` and no token minting existed, so the React `AgentBridge` had nothing to dial. The codec was already proven byte-identical across Rust and TS — it was the socket that was missing.

### Taxonomy debt, deliberately deferred

`verify taxonomy report` reports **10 858** findings repo-wide (the repo is far from clean mid-refactor). Exactly **13** name our own paths, and they are legitimate:

| finding | subject | verdict |
|---|---|---|
| `collection-empty` / `collection-manifest-missing` / `collection-authored-behavior` | `🌉️mcp/🎬️actions` | **real name collision** — `🎬️actions` is a reserved *semantic collection* in the plugin taxonomy, so our facet directory is being read as a collection. Needs renaming to a non-reserved facet name. |
| `manifest-child-missing` ×5 | `🌉️mcp`, `🖥️shell`, `AgentBridge`, `AgentPresence`, `AgentApprovals` | each new directory needs its `🔣️component.json` declaring children |
| `packaging-violation` | `🌉️mcp/📦️packages/🟦️typescript/🧬️schema-validation.ts` | my file, from relocating the `ajv` import — needs the correct packaging placement |
| `generated-provenance-missing`, `module-production-consumer-minimum`, `module-consumer-graph-mismatch` ×2 | `🖥️shell`, `🌉️mcp` | generated-file provenance header + consumer declarations |

**Not fixed in this wave, on purpose.** `P1c` and `P7b` are editing inside `🌉️mcp` right now, and a directory rename plus `#[path]` rewiring under a live agent is precisely the "don't fight a live rewrite" failure I have been enforcing on every packet. It is queued as a single cleanup pass once both land.

### The live loop closes — verified by me against a running gateway

`P1c` merged the `/bridge` WebSocket route into the same axum app that serves `/mcp` (`🚚️transport` L223–224) and mints a fresh token per start. I booted the real binary and drove it with a **real WebSocket client using the TypeScript codec twin** — the same module the React `AgentBridge` imports:

```
[semio-os-mcp] bridge listening on ws://127.0.0.1:7411/bridge?token=49d1922c…  (also written to /tmp/semio-tok.I0l2/bridge-token)
socket OPEN
sent Hello (variant-tagged)
received: {"variant":"welcome","bridgeVersion":1,"connection":"conn_0","principal":"agent:local"}
sent Ping
received: {"variant":"pong"}
PASS: live bridge Hello->Welcome and Ping->Pong
```
The token file is `-rw-------` (0600), as required.

Security properties, all checked against the running process rather than asserted:

| probe | result |
|---|---|
| `/bridge?token=WRONG` | **401** |
| `/bridge` with `Origin: https://evil.example` | **403** |
| `POST /mcp` with no bearer | **401** |
| `POST /mcp` with bearer + `MCP-Protocol-Version: 2026-07-28` | real `tools/list` |

`/mcp` and `/bridge` share one socket, so a shell and an LLM client attach to the same gateway.

Suite after all of it: **169 passed, 0 failed** (up from 160 — P1c and P7 added tests).

### Two coordination notes worth keeping

**I did not fix a peer's broken crate, and that was right.** `semio-framework-ui` went red mid-session (`unresolved import presence_bar::presence_hue_for_actor` — a peer deleted the function, its replacement's own doc comment says so, while a `🎯️targets/🧊️wgpu/📦️glue.rs` re-export still named it). It blocked our builds. But `stat` showed both files had been written **seconds earlier**, so the session was live, not abandoned — the opposite of the stopped-12-hours case where their coordinator correctly did intervene. I warned both running agents (so they would not misattribute it or "helpfully" patch someone else's crate) and waited. The peer fixed it themselves at 13:19:46 and our suite went green again.

**`P7b` reached the right conclusion and stopped at the boundary.** The WASI gap (`wasi:io/poll@0.2.9` missing from the linker) cannot be fixed from `🏠️workspace/**`: our own `world actor` declares exactly one import (`pure`), and the requirement comes from the built `wasm32-wasip2` component itself — so the linker change belongs in `semio-framework-plugin-host`, which is the peer ticket's B1 territory. It filed that rather than duplicating linker setup locally, which would have been exactly the divergence CLAUDE.md forbids. **Real plugin instantiation in headless mode therefore remains open**, pending that lease. Everything up to instantiation — policy, schema validation, prepare, catalog, resources — is proven working.

### Taxonomy debt resolved — one real fix, the rest correctly left alone

I said I would clear the 13 findings once the live agents landed. Doing so honestly meant separating a genuine defect from repo-wide convention, which is what the numbers show:

- **Fixed (uniquely ours, a real collision):** `🌉️mcp/🎬️actions` → **`🌉️mcp/🔀️dispatch`**. `🎬️actions` is a *reserved semantic collection* name in the plugin taxonomy — the only such reserved name matching action/dispatch/invoke — so our facet directory was being parsed as a collection and produced three findings at once (`collection-empty`, `collection-manifest-missing`, `collection-authored-behavior`). The Rust module is named `actions`, so the fix was the directory plus one `#[path]` line; every `crate::actions::…` reference stayed valid. Compiles clean, suite unaffected. `🔀️` was verified as an ordinary facet prefix already in use (`◻2d/🔀️booleans`, `🗣️dsl/🔀️dsl-value-serde`), not reserved.
- **Left alone (repo convention, not our defect):** `manifest-child-missing` fires on **every** established sibling — `🏃️run`, `🏪️store`, `🌿️vcs`, `🔁️workflow`, `🪐️space`, `📇️directory` — and on 91 elements, **4 354 findings repo-wide**. Adding `🔣️component.json` to only our five directories would make us inconsistent with a convention the repo has not adopted, not more correct.
- **Kept with the trade-off recorded:** the `packaging-violation` on `📦️packages/🟦️typescript/🧬️schema-validation.ts`. I moved the `ajv` import there to fix a real `not-to-unlisted` policy error (a module-root file must not reach for an external package). Moving it back would simply trade one lint for the other. Only 11 such findings exist repo-wide and one of them is the peer ticket's `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` — the same shape, placed there by their design on purpose. Keeping the dependency inside the package that declares it is the more defensible half of the trade.

### Final state

| check | result |
|---|---|
| `cargo test -p semio-framework-os-mcp` | **169 passed, 0 failed** |
| `cargo test -p semio-framework-os-shell` | **10 passed, 0 failed** (1 ignored — the fixture writer) |
| `bun nx run @semio-tech/framework-os-mcp:test-quick --skip-nx-cache` | **22 passed** (real SDK client) |
| warnings from our code | **0** (the 2 remaining are `semio-framework-os-kernel`'s `📡️spr/📡️wire` and `semio-framework-plugin-host`'s `🧵️shard`, both peer-owned) |
| React shell after mounting the agent elements | 11 failed / 325 passed — **identical to the pre-change baseline**, none agent-related |

### Open, with owners — not silently dropped

1. **Real plugin instantiation in headless mode** — blocked on a WASI linker change that belongs in `semio-framework-plugin-host` (peer B1 territory), leased by P7b. Everything up to instantiation is proven; the mutation path beyond it is not yet exercised against a real wasm plugin.
2. **`run_http` drops the `BridgeHandle`** — P1c published `send_to`/`broadcast` and flagged that the production path does not retain the handle yet, so a parked approval cannot yet be pushed to a connected shell from the HTTP entrypoint. A few lines, once a real in-process consumer exists.
3. **Eval accuracy 50 % top-1 / 73.5 % top-3** — a data problem, not a search problem: no real plugin action carries `use_when` yet. Waits on their W3 finishing the plugin migration, then P13/P14 enrichment and a P15 re-measure. Do not tune the scorer against the current empty-metadata corpus.
4. **Hub agent principal (P4)** — still deferred per D7 while `FINISH-HUB-SPACES` rewrites the auth flow.
5. **P12 browser e2e** — now unblocked by the live bridge; not yet written.
