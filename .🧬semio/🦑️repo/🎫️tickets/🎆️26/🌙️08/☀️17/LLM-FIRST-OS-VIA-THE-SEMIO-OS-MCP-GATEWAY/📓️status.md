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
