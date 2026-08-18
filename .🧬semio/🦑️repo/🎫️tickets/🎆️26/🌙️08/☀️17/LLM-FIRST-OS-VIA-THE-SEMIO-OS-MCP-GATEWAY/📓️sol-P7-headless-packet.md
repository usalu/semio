# 📓️ sol brief — packet P7-headless-workspace (verbatim, as dispatched)

You are "terra", an executor on ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` in /Users/ueli/Documents/semio. Packet id: **P7-headless-workspace**. Model: Sonnet 5.

## 0. First action
Read in full: `…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📌️important.md`; `…/📓️design-decisions.md`; `…/📓️luna-channel-audit.md` (**§8 is the headless-driving precedent you copy**); `…/📓️terra-P1a-report.md` (the `GatewayBackend` seam you implement); `📋️master.md` §2.1 and §3.6; `/Users/ueli/Documents/semio/CLAUDE.md`. Also read the peer ticket's `📓️status.md` (`…/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️status.md`) — the runtime you are embedding is theirs and it changed this week.
Save this brief verbatim as `…/📓️sol-P7-headless-packet.md`.

## 1. State of the world (verified by sol just now)
Their G1 is met: `cargo check -p semio-framework-plugin-host --lib` finishes clean, `CHANNEL_VERSION` is **12**, and `✏️s/🔌️plugins/🗒️note/🔣️descriptor.json` is a **real committed descriptor** (269 KB, real activation events and capability requests). The gateway passes 115 Rust + 26 TS tests. It currently has **no live workspace** — every artifact/context tool returns `PLUGIN_UNAVAILABLE`. You fix that.

## 2. Owned writable paths (EXCLUSIVE)
```
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️component.rs   (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs                     (wire --folder/--hub flags to a real workspace)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️component.rs               (mount the facet only)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs}
.🧬semio/…/📓️sol-P7-headless-packet.md, 📓️terra-P7-report.md, 📓️lease-P7-*.md, *.txt
```
`🎬️actions`/`🛡️policy` are P6's and are being written **right now in parallel** — do not touch them; if you need something from them, define your side of the seam and say so. Nothing outside `🌉️mcp` (the `🏃️run` plugin-path extraction is a **lease**, see §4).

## 3. Required result — a real headless workspace
`HeadlessWorkspace` implementing P1a's `GatewayBackend` so the gateway can open, read and drive real artifacts with no UI:
1. **Bindings**: `--folder <dir>` → `store::sync::ArtifactHost` with `PersistenceBinding::Folder` (`watch_external: true`, so a native shell on the same folder sees agent edits); `--hub <url> --space <id> --token <t>` → the Hub binding. Follow `🏃️run`'s precedent exactly rather than inventing a second way to open a space.
2. **Plugin discovery**: reuse `🏃️run`'s `PLUGIN_WASM_ARTIFACTS` / `resolve_plugin_paths` / `find_repo_root`. Those live in `🏃️run` (a peer-owned file) — **do not copy-paste them** (duplication is forbidden by CLAUDE.md); emit a `lease-request` asking their sol to extract them into a `//#region 🔖️PluginPaths` public API, and in the meantime depend on whatever is already public. If nothing is public, say so plainly and use the narrowest workable approach, documenting it as a temporary seam **with the lease already filed**.
3. **Instance lifecycle**: resolve `(plugin, app)` for an artifact via the plugin host's `OpeningResolver`, activate the actor through the runtime the peer ticket now provides (`GuestRuntime`/`WasmtimeRuntime`, one shared `Engine`, one `Store` per actor), open the instance with the agent actor id `agent:<principal>#<session>` and the principal's capability grants, and hydrate document/config/draft lanes from the binding.
4. **Channel driving**: implement the exchange port P6 needs (`ArtifactChannel`-shaped: send `AppCommand`s, receive `AppFrame`s) on top of the real runtime. Read P6's report if it has landed by the time you get here; otherwise implement the obvious shape and reconcile.
5. **Backend methods**: implement `GatewayBackend`'s workspace/artifact operations for real — `resolve_context` (active space/artifacts/revisions), `list_resources`/`read_resource` for `semio://workspace*` and `semio://artifact/{id}` (+ `/schema`, `/history`, `/validation`), and `artifact_open` / `artifact_create` / `artifact_validate` / `artifact_snapshot` / `artifact_export` where the channel supports them. Anything genuinely not reachable yet returns a well-formed typed error — **never fabricate data**.
6. **Backbone**: guest `SendMessage{Backbone}` effects route to `ArtifactHost::send`; inbound `ArtifactEvent::RemoteMutations` come back as guest events — so a commit made headlessly propagates to any live shell on the same folder/hub. This is the property that makes the whole design work; prove it in a test if you can.

## 4. Leases
File early, keep working: (a) `🏃️run` `🔖️PluginPaths` extraction (§3.2); (b) anything else outside `🌉️mcp`. Post each as `…/📓️lease-P7-<topic>.md` **and** a fenced block in your report, with exact text and rationale.

## 5. Acceptance (FOREGROUND ONLY, long timeouts, paste everything)
```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp
CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp --bin semio-os-mcp 2>&1 | grep -c "^warning"
```
**The deliverable that matters** is a live end-to-end transcript, pasted verbatim: point the binary at a real space and show `context_resolve` returning a genuine artifact with a real revision. Use an existing example space — look under `🧰️framework/🛍️products/💻️os/📚️examples/` and `✏️s/🔌️plugins/🗒️note/` for fixtures, and prefer `🗒️note` (it is the plugin with a committed descriptor). Building a plugin wasm may be needed (`bun ./📜️script.ts` dev targets or a cargo `--target wasm32-wasip2` build); if the wasm cannot be produced in reasonable time, say so explicitly, show how far you got, and land the workspace with its unit tests rather than claiming an unproven end-to-end.
All 115 pre-existing tests must still pass.

## 6. Hard rules
All of `📌️important.md`: **never background a build** (foreground, long timeout — a slow build is not a hung build); no git-modifying commands; nothing outside §2 (lease instead); scratch `.txt`/`.md`/`.json` in the ticket folder, never `.log`; `[DEBUG] ` removed before done; **never claim a result you did not run and paste**; no `AGENTS.md` edits; no compat shims, no duplicated logic. Note there is a pre-existing warning in `📡️spr/📡️wire` owned by the peer — not yours.

## 7. Report
`…/📓️terra-P7-report.md`: baseline HEAD, SHA-256s, line counts, the exact runtime APIs you consumed (with paths), the live transcript (or an honest account of what blocked it), leases filed, and a "what is still stubbed and why" list.
