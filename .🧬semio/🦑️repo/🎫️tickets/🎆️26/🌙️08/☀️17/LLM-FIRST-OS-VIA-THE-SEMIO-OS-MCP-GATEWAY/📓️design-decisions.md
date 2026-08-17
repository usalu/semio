# 📓️ Design decisions — deltas discovered in W0

`📋️master.md` is the plan of record. This file records the decisions W0 forced, with evidence. Where the two disagree, **this file wins**.

## D1. The gateway must be **dual-era** MCP (verified against the spec, not assumed)

`https://modelcontextprotocol.io/specification/2026-07-28/basic/lifecycle` ("Versioning and Compatibility"), fetched 2026-08-17, states verbatim: *"There is no negotiation handshake. Every request carries its protocol version"*; **modern** = `2026-07-28`+ (per-request `_meta` protocol version, no `initialize`, no session); **legacy** = `2025-11-25` and earlier (`initialize` handshake). `server/discover` is **MUST implement** for modern servers. Version mismatch → `UnsupportedProtocolVersionError`, JSON-RPC code **-32022**, `data: {supported: [...], requested: "..."}`.

Verified in our tree: `node_modules/@modelcontextprotocol/sdk` **1.30.0** has `LATEST_PROTOCOL_VERSION = '2025-11-25'`, `DEFAULT_NEGOTIATED_PROTOCOL_VERSION = '2025-03-26'`, `SUPPORTED_PROTOCOL_VERSIONS = ['2025-11-25','2025-06-18','2025-03-26','2024-11-05','2024-10-07']` (`dist/esm/types.js:2-4`). **Every client we must serve today — the installed SDK, Claude Code, the IDE MCP clients — is legacy-era.**

**Decision**: `semio-os` is a **dual-era server**, which the spec explicitly specifies (*"A dual-era server MAY serve both eras concurrently on the same endpoint or process"*): a request carrying modern per-request `_meta` is served statelessly per `2026-07-28`; an `initialize` request selects legacy semantics. Supported set: `["2026-07-28", "2025-11-25", "2025-06-18"]`.

**This is not a CLAUDE.md "compatibility layer" and must not be "cleaned up" later.** Both eras are current revisions of an external protocol we do not own; serving both is the spec's own interoperability contract (its compatibility matrix makes dual-era the only cell that works for every client). The single-era alternative would make the server unusable by every MCP client that exists today.

## D2. Demo action is `📐️cad translateSelection` — there is no `extrude`

`📓️luna-actions-audit.md`: cad declares 38 actions including `translateSelection`/`rotateSelection`/`scaleSelection` with numeric component args (`dx`,`dy`,`dz`); **no `extrude` action exists** (only an engagement fixture `🔣️extrudeCrv.json`). Every acceptance criterion in `📋️master.md` that says "extrude" means **`translateSelection`**.

`🗒️note` declares 36 actions but **none with manifest args** (engagement-driven). Consequence: note is *not* usable as the first agent-invocable demo without arg declarations. **W1 demo target = cad `translateSelection`**; note gets declared args in the enrichment wave (P13) and is used for the *artifact lifecycle* demo (create/open/validate) only.

## D3. Capability ids must be plugin+app scoped from day one

14 action ids are declared by more than one plugin (`deleteSelection`, `translateSelection`, …). The `<plugin_id>.<app_id>.<action_id>` grammar in `📋️master.md` §3.1 is therefore mandatory, not an optimisation; a bare action id is never a capability id.

## D4. All 37 `command_from_action` implementers read their args

No action silently drops its declared args, so the arg→command bridge is sound today. The `assert_action_args_reach_commands` contract test (master §5) is a **regression guard**, not a repair job.

## D5. `CapabilityDefinition` lives in the gateway crate, not in `🛂️manifest`

Split refined after W0: `🛂️manifest` owns what *plugins declare* (`ArgSchema`, `ActionSemantics`, and the fields on `ActionDefinition`/`ActionArgDef`); the gateway crate owns what it *compiles* (`CapabilityRef`, `CapabilityDefinition`, `Catalog`). This keeps the manifest edit minimal (it is the single most contended framework file) and puts the projection type next to its only consumer. `📋️master.md` §3.1 is amended accordingly.

## D6. `ActionArgDef.schema` is stored, `control()` is derived — in one atomic packet

Blast radius measured: ~12 Rust files and ~9 TS files reference `ActionArgControl`/`.control`, several inside microkernel H1/H3 territory (`Shell/🧊️component.rs`, `⚛️react/📦️index.tsx`, `ShellHelpers`) and inside plugin crates that freeze during their W3. Storing both `control` and `schema` would be duplicate state (forbidden). P3 therefore converts stored→derived and updates **every** reader in one packet, emitting a `lease-request` for any reader in a contested file. P3 runs in the window **after their A3 (accepted) and before their W3 fan-out**.

## D7. Hub agent principal is **deferred**, gateway auth is loopback-token first

`📓️luna-hub-audit.md`: collision risk **HIGH** — the live ticket `FINISH-HUB-SPACES-COLLABORATION-END-TO-END` is actively rewriting the hub auth flow (`resolve_auth`, 23 REST routes), and `SHARED-PRESENCE-SESSION-COLORS-…` touches presence streaming. Adding `DirectoryActorKind::Agent` is a 1-line schema change, but the mint routes and `AuthOutcome::Agent` land in the middle of someone else's rewrite.

**Decision**: P4-hub-agent moves out of W1 to after those tickets close (sol re-checks each wave). The gateway is fully functional without it: the agent principal is a **gateway-local** `AgentPrincipal` with scopes, authenticated on loopback by the bridge/HTTP bearer token, and hub-bound operation initially reuses a normal delegated user session. `semio://audit/*` writes to the local folder lane from day one. Nothing in the gateway's design changes when the hub principal later arrives — only where the token comes from.

## D8. P1 splits into P1a (protocol core + stdio) and P1b (HTTP + handles + audit + bridge)

The original P1 was XL and defines the traits every other packet consumes. P1a lands the crate, the JSON-RPC/dual-era core and stdio; P1b builds on its published traits and can then run in parallel with P2/P3.

## Standing acceptance note

`📓️luna-testinfra-audit.md` §"Cookbook" holds the exact boilerplate (Cargo.toml keys, the four required test levels in `📋️project.json`, `📜️script.ts` command shape, root wiring insert positions) — every packet creating files follows it rather than inventing structure.
