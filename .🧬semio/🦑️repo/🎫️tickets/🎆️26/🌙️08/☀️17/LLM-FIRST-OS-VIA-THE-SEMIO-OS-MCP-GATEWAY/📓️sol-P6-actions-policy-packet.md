# 📓️ sol packet brief — P6-actions-policy (verbatim)

You are "terra", an executor on ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` in /Users/ueli/Documents/semio. Packet id: **P6-actions-policy**. Model: Sonnet 5.

## 0. First action
Read in full: `…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📌️important.md`; `…/📓️design-decisions.md`; `…/📓️luna-channel-audit.md` (**your specification — it contains the exact frame sequence pseudo-code for prepare→preview→commit→verify→undo**); `…/📓️terra-P2-report.md` (the catalog API you consume, incl. its "what P6 needs from me" section); `…/📓️terra-P1a-report.md` + `📓️terra-P1b-report.md` (crate API, handles, audit); `📋️master.md` §3.3–3.4; `/Users/ueli/Documents/semio/CLAUDE.md`.
Save this brief verbatim as `…/📓️sol-P6-actions-policy-packet.md`.

## 1. State of the world (verified by sol just now — do not re-litigate)
The peer microkernel ticket has reached its G1: `cargo check -p semio-framework-plugin-host --lib` **finishes clean**, and `CHANNEL_VERSION` is now **12** (A4 landed). The frames you need all survive v12 per `📓️luna-channel-audit.md` §9. The gateway crate `semio-framework-os-mcp` passes 115 Rust + 26 TS tests, serves both MCP eras on stdio + HTTP, and registers 20 tools of which 17 currently return a structured `PLUGIN_UNAVAILABLE`. **Your job is to make the mutation tools real.**

## 2. Owned writable paths (EXCLUSIVE)
```
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎬️actions/🦀️component.rs     (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🛡️policy/🦀️component.rs      (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️component.rs               (register the real tools; mount facets)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs}
.🧬semio/…/📓️sol-P6-actions-policy-packet.md, 📓️terra-P6-report.md, 📓️lease-P6-*.md, *.txt
```
Do NOT edit other `🌉️mcp` facets (`🧭️protocol`, `🚚️transport`, `🎫️handles`, `📒️audit`, `🧵️bridge`, `🗂️catalog`, `🔎️search`, `🧠️context`, `🧪️conformance`, `⚠️errors`, `🧬️schema`) — consume their public APIs. Nothing outside `🌉️mcp`.

## 3. Required result — the mutation protocol, for real

### 3.1 `ActionAdapter` (`🎬️actions`)
Implement the lifecycle from `📋️master.md` §3.3 against the **real** channel frames, driving an app instance through a `GatewayBackend` (P1a's seam) whose concrete implementation is P7's headless workspace. Since P7 lands in parallel, define the narrow port you need (e.g. `trait ArtifactChannel { fn exchange(&mut self, instance: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, Fault>; }`) and implement the adapter against **that**, with a scripted `MockArtifactChannel` for tests. This keeps you independent of P7's schedule; P7 plugs its real channel in.
- **prepare**: validate input against the capability's `input_schema` (`SchemaCatalog`/`jsonschema`) → `INPUT_INVALID` with the validator message; resolve the target instance; check policy; capture the current `RevisionStamp` (from `AppCommand::ReadHistory` → `AppFrame::HistorySnapshot`); mint a `prep_` handle via P1b's `HandleTable`.
- **preview**: `AppCommand::PureCommand{command, document/config/draft packs}` on a **preview instance** → `AppFrame::Emit{document_ops, …}`; split the ops with the repo's `decode_ops_vec` equivalent; summarise the diff (`opsCount`, touched paths) into `PreparedActionReport`. `PureCommand` is a true dry-run — the audit proves it hydrates and dispatches without persisting.
- **commit**: `TransactionPrepare{txn_id, prepared_ops, label, origin}` (pre-planned form: `mutation_id` empty) → `TransactionPrepared` → `TransactionCommit` → `TransactionCommitted{edit_id}`. Map `transaction.generation-mismatch` → **`REVISION_CONFLICT`** carrying expected/actual and a diff URI; `transaction.instance-busy` → bounded retry then `PRECONDITION_FAILED`.
- **verify**: re-read the revision + `DispatchReport` → `InvocationReport`.
- **undo/redo**: `undoToken` ↔ `txn_id`; `TransactionUndo{group_id}` / `TransactionRedo`.
- **expectedRevision**: when supplied and stale → `REVISION_CONFLICT` **before** any mutation is attempted.
- **idempotency**: use P1b's `IdempotencyStore`; a replayed key returns the stored `InvocationReport` with `replayed: true` and performs no second mutation (test this).
- **transactions**: `transaction_begin/commit/rollback` binding several prepared handles; on a member failure, compensate in reverse order and return `COMPENSATION_FAILED` if compensation itself fails. Never claim atomicity you cannot deliver.
- **cancel**: `action_cancel` drops a prepared handle; for job-class capabilities, cancel the job.

### 3.2 Policy engine (`🛡️policy`)
- `AgentPrincipal { id, label, scopes: Vec<kernel::CapabilityId>, delegated_by, }`, built from the CLI flags P1a already parses.
- The scope ↔ `CapabilityId` table from `📋️master.md` §3.4 verbatim (`artifact.write` → `documents.write` + `jobs.spawn`, `ui.raw-control` → `shell.raw`, …). No `semio.full-access`.
- Enforcement: a capability's `policy.scopes` (from `ActionSemantics`, landed by P3) must be a subset of the principal's effective scopes, else **`PERMISSION_DENIED`** — and the denial is **audited** via P1b's `AuditSink` before returning.
- `ApprovalMode`: `Never` → proceed; `WhenDestructive` → require approval iff `effects.destructive`; `Always` → always. When approval is required and the client advertised `elicitation`, issue `elicitation/create` with the diff summary; otherwise park an `ApprovalRecord` (P1b handle kind `Approval`), return **`APPROVAL_REQUIRED{approvalHandle}`**, and expose it for the shell to resolve later. A `--auto-approve never|readonly|all` flag governs the headless case; **default must be the safe one**.
- **Every** invocation writes an `AgentAuditEvent` (allowed, denied, or approved) with the input hash and the revision before/after.

### 3.3 Wire the tools
Replace the `PLUGIN_UNAVAILABLE` stubs for `action_prepare`, `action_invoke`, `action_cancel`, `transaction_begin`, `transaction_commit`, `transaction_rollback`, `history_undo`, `history_redo` with real handlers. Leave `artifact_*`, `job_*`, `ui_*` to P7/P10 (but if `artifact_validate`/`artifact_snapshot` fall out naturally from the channel work, implement them and say so).

## 4. Tests (this is the packet where correctness matters most)
`//#region 🧪️Tests`, `mod quick`/`mod long`, over `MockArtifactChannel` with scripted frames: preview-vs-commit ops equality; stale `expectedRevision` → `REVISION_CONFLICT` **with no mutation sent** (assert on the recorded frame log, not just the error); idempotent replay performs exactly one mutation; undo token round-trip; approval gate blocks a destructive capability without approval and proceeds with it; a capability whose scopes exceed the principal's → `PERMISSION_DENIED` **and** an audit row; saga compensation ordering (reverse) and `COMPENSATION_FAILED`; cancel of a prepared handle; every `Fault` code maps to the right `GatewayErrorCode`.
Assert on **observable behaviour** (frames sent, audit rows written), not on internal call counts.

## 5. Acceptance (FOREGROUND, paste output + exit codes)
```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp
CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp 2>&1 | grep -c "^warning"   # → 0 from OUR code (a warning in 📡️spr/📡️wire is the peer's, note it and move on)
bun nx run @semio-tech/framework-os-mcp:test-quick
```
All 115 existing tests must still pass. Plus a live transcript: `tools/call action_prepare` against a mock-backed instance showing a `PreparedActionReport`, and one showing `PERMISSION_DENIED` for a scope the principal lacks.

## 6. Hard rules
All of `📌️important.md`. Especially: **never background a build or test** (foreground with a long timeout); no git-modifying commands; nothing outside §2 (lease instead); no `.log`; `[DEBUG] ` removed before done; never claim an unrun result; no `AGENTS.md` edits; no compat shims. Add no external crates without a lease. Docstrings start with a unique emoji; `//#region` structure; no comments inside definitions.

## 7. Report
`…/📓️terra-P6-report.md`: baseline HEAD, SHA-256s, line counts, the exact frame sequence you implemented (as a table), the error-mapping table, full acceptance output, the live transcripts, leases, and a precise statement of what P7 must implement to satisfy your `ArtifactChannel` port.
