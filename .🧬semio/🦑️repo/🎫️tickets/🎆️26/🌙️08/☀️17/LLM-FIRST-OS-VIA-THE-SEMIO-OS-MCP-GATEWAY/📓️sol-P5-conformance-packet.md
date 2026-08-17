You are "terra", an executor on ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` in /Users/ueli/Documents/semio. Packet id: **P5-conformance-tests**. Model: Sonnet 5. Coordinator ("sol") is the main chat.

## 0. First action
Read in full: `…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📌️important.md`, `…/📓️design-decisions.md` (**D1 — dual-era — is the point of this packet**), `…/📓️terra-P1a-report.md` (the server you are testing), `…/📓️luna-testinfra-audit.md` (§TS package anatomy, vitest conventions, the four test levels), `/Users/ueli/Documents/semio/CLAUDE.md`.
Save this brief verbatim as `…/📓️sol-P5-conformance-packet.md`.

## 1. Why this packet exists
`semio-os-mcp` currently proves its own correctness with its own Rust tests. That is self-referential: it proves the server agrees with itself, not that a **real MCP client** can talk to it. The installed `@modelcontextprotocol/sdk` (1.30.0, legacy-era, `LATEST_PROTOCOL_VERSION = '2025-11-25'`) is the same library every IDE client uses. This packet is the independent interop proof.

## 2. Owned writable paths (EXCLUSIVE)
```
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/**    (new TS package — you create it)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️component.ts                (new — TS surface of the module)
.🧬semio/…/📓️sol-P5-conformance-packet.md, 📓️terra-P5-report.md, 📓️lease-P5-*.md, *.txt scratch
```
You do NOT edit the Rust crate. If you find a **server bug**, do not fix it — document it precisely in your report (request, expected, actual) so sol can route it to the owning packet; a failing test that documents a real bug is a valid deliverable, but say so loudly rather than hiding it or weakening the assertion.

## 3. Required result
A TS package `@semio-tech/framework-os-mcp` at `🌉️mcp/📦️packages/🟦️typescript` following the cookbook in `📓️luna-testinfra-audit.md` exactly (`package.json`, `📋️project.json` with **all four** test levels, `📜️script.ts`, `🧪️vitest.config.ts`), whose tests drive the real binary with the real SDK client.

### 3.1 How to reach the server
Build once, then spawn the binary. The binary lives at `<ticket>/🎯️target/debug/semio-os-mcp` when built with `CARGO_TARGET_DIR=<ticket>/🎯️target`. Resolve it via env `SEMIO_OS_MCP_BIN` with that path as the default, and **skip with a clear message rather than silently passing** if the binary is absent. Use the SDK's `Client` + `StdioClientTransport`.

### 3.2 Conformance suite (legacy era — what the SDK speaks)
- `initialize` succeeds; the negotiated `protocolVersion` is `2025-11-25`; `serverInfo.name === "semio-os-mcp"`; declared capabilities include `tools`, `resources`, `prompts` with their `listChanged`/`subscribe` flags.
- `tools/list` returns a schema-valid list; **every** tool name matches `^[a-zA-Z0-9_-]{1,64}$`; the list is byte-identical across two consecutive calls (determinism).
- Every tool's `inputSchema` (and `outputSchema` when present) is a valid JSON Schema 2020-12 — validate with a real validator (`ajv` is already in the tree via the SDK's deps; if importing it is awkward, use the SDK's own validation path and say which you used).
- `resources/list` + `resources/templates/list` shapes; `resources/read` on a template URI behaves (or returns a well-formed MCP error while the backend is `NullBackend` — assert the *error shape*, which is what matters at this stage).
- `prompts/list` / `prompts/get` shapes.
- `tools/call` on an unimplemented capability returns a **tool error** (`isError: true` with structured content), NOT a JSON-RPC protocol error — this distinction is the thing most servers get wrong and P1a claims to get right; prove it.
- `ping`.

### 3.3 Modern-era suite (raw JSON-RPC, since the SDK cannot speak it)
The SDK is legacy-only, so write a **minimal raw stdio JSON-RPC client** (~50 lines: spawn, write newline-delimited JSON, read lines) and assert the modern contract: `server/discover` returns supported versions and capabilities; a request carrying `_meta["io.modelcontextprotocol/protocolVersion"] = "2026-07-28"` is served statelessly with **no** `initialize`; an unsupported version yields `-32022` with `data.supported`; and — importantly — a modern request works on a **fresh process with no handshake at all**.

### 3.4 Hygiene assertions
- The server never writes non-JSON to stdout (capture stdout across a whole session and assert every line parses as JSON) — a single stray byte breaks every MCP client.
- Malformed input yields a proper JSON-RPC error, not a crash; the process stays alive and answers the next request.
- The process exits cleanly on stdin EOF.

## 4. Acceptance (FOREGROUND ONLY, paste output + exit codes)
```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp --bin semio-os-mcp
bun nx run @semio-tech/framework-os-mcp:test-quick
bun nx run @semio-tech/framework-os-mcp:test-long
```
The TS package needs a root `package.json` workspaces entry → **emit that lease-request in your first minutes** (`…/📓️lease-P5-package-workspace.md` + fenced block in the report, exact line + sorted position), keep working, and run acceptance once sol confirms. If sol has not confirmed by the time you finish, say so plainly and paste what you could run (e.g. a direct `bunx vitest run` against the config). Never invent results.

## 5. Hard rules
All of `📌️important.md`: no git-modifying commands, no ticket MCP write tools, nothing outside §2, **no background processes for builds** (spawning the server binary *inside a test* is fine and expected — that is the test, not a background build), scratch `.txt`/`.md`/`.json` in the ticket folder only, `[DEBUG] ` removed before done, no unpasted claims, no `AGENTS.md` edits. Do not add new external npm dependencies — the SDK and vitest are already present; if you think you need another, emit a lease-request with the justification instead.

## 6. Report
`…/📓️terra-P5-report.md`: baseline HEAD, files created with SHA-256 + line counts, the full test list with pass/fail, complete acceptance output with exit codes, **any server bug found (request/expected/actual, and which packet owns the fix)**, leases emitted, and a short "what a real IDE client will experience today" paragraph.
