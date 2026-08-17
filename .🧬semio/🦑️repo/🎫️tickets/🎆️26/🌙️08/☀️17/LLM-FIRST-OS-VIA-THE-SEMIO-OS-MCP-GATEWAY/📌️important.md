# 📌️ Binding rules — LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY

**Empty this file before `ticket_close`.**

## Hard prohibitions (every agent)

1. **No git-modifying commands.** No `commit`, `stash`, `checkout`, `reset`, `worktree`, `add`, `rm`. Other sessions are live in this tree and an auto-commit bot runs. `git status` is NOT a churn detector — use `git log --date=iso --oneline -3 -- <path>` plus file hashes. Auto-commit *messages* carry a frozen fake date; only `--date=iso` is real.
2. **No `ticket_close` / `ticket_reopen` by anyone but sol** (the main-chat coordinator). A subagent closing this ticket closes the whole umbrella.
3. **Never edit outside your packet's `path_scope`.** A region name inside a shared file is not ownership. Need a shared-file change → emit a `lease-request` fenced block and stop.
4. **Never run bare workspace cargo.** Always `CARGO_TARGET_DIR=<ticket>/🎯️target` **and** `-p <crate>`. A slow build is not a hung build. Up to ~13 concurrent cargo processes exist across sessions.
5. **Run builds and tests in the FOREGROUND.** Never `run_in_background: true`, never `&`, never a poll/wake loop. Background jobs do not survive a subagent turn — the peer ticket burned ~338 k tokens learning this. Use a long `timeout` on the Bash call instead.
6. **Scratch files are `.txt` / `.md` / `.json` inside the ticket folder.** Never `.log` (repo-wide gitignored — `ticket_close` silently drops them).
7. **Never claim a test passed without pasting its output and exit code.**
8. Temporary logs carry the `[DEBUG] ` prefix and are removed before a packet reports done.
9. **Never edit any `AGENTS.md`.**
10. **No legacy support, no compatibility layers, no deprecations, no migration scripts** (CLAUDE.md). Replace, do not wrap.

## Registrar-only files (sol edits these; everyone else sends a `lease-request`)

`/📜️script.ts`, `/Cargo.toml`, `/Cargo.lock`, `/📋️project.json`, `/package.json`, `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json`, `.mcp.json` + IDE mirrors, `🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, all `🤖️generated/**`, `ShellHost/🟦️component.tsx`, `Shell/🧊️component.rs`, `⚛️react/📦️index.tsx`, dev `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`.

## ⚠️ Live peer ticket — `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ("their" ticket)

That ticket is mid-flight in the same tree with its own coordinator and 4+ executors. It owns these paths **exclusively**; we must NOT edit them — a change there is a `lease-request` posted to their ticket folder and applied by their sol:

| their path_scope | their packet | our dependency |
|---|---|---|
| `🧰️framework/🔨️modules/🎭️actor/**` | A1 (done, verified) | `Origin::Agent` variant (lease) |
| `🎠️kernel/{🦀️,🟦️}component.*`, `🛂️manifest/{🦀️,🟦️}component.*` | A3 | our manifest regions wait for A3 "done" |
| `📡️spr/🧵️channel/🦀️component.rs`, `💻️os/🟦️component.ts` codec region | A4 (channel v12) | read-only consumer after their G1 |
| `🔌️plugin/📦️packages/🦀️rust/📜️wit/**`, `🔌️plugin/🦀️component.rs`, `🔌️plugin/{⚛️reactor,🌐host,🏗️builder}/**` | A2 (SDK frozen during their W3) | agent SPI lease in the G1→W3 window |
| `🔌️plugin/🖥️host/**`, `💻️os/🖥️host/🦀️component.rs`, `🧩️extension/🦀️component.rs`, `🏃️run/🦀️component.rs` | B1 | headless workspace after their G1 |
| `🧱️elements/{PluginRuntime,WasmSessionLoader,ShellHost}/**`, `⚛️react/📦️index.tsx`, `🧑️‍💻️dev/🟦️component.ts`, wgpu targets | H1–H4 | shell adoption after their G2 |
| `📇️describe/**`, `📇️registry/**` | E1 | descriptor `agent` block after their G2 |
| `✏️s/🔌️plugins/**` | A2 + M0–M8 | plugin enrichment after their G3 |

**Also live nearby:** `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` (io mechanism, artifact/dsl tree) and `FINISH-HUB-SPACES-COLLABORATION-END-TO-END` / `SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION` (hub, directory, presence). Churn-check `🗣️dsl/🧬️schema/🦀️component.rs`, `🌎️hub/**` and `📇️directory/**` immediately before every edit.

## Our disjoint new territory (safe to create)

`💻️os/🔨️modules/🌉️mcp/**` · `💻️os/🔨️modules/🖥️shell/**` · `🧱️elements/Agent*/**` (new dirs) · `💻️os/🎮️commands/🤖️agent-*/**` · `💻️os/🎚️config/🧬️schema/🧬️mutations/🤖️grant-agent-scope|🚫️revoke-agent-scope` · this ticket folder.

## Naming hazards

- `kernel::ActorId` is the presence/collab actor; the runtime actor id is `RuntimeActorId`. Never shadow either.
- `🤖️generated` is a reserved folder name — our module dir is `🌉️mcp`, never `🤖️mcp`.
- `CapabilityId` (kernel Broker, a permission) ≠ `CapabilityDefinition` / `CapabilityRef` (manifest, an invocable operation). Never conflate.
- `ShellState` exists three times today (TS `Shell/🟦️component.tsx`, wgpu `Shell/🧊️component.rs`, and ours). Ours is the SSOT in `💻️os/🔨️modules/🖥️shell`; the other two become projections.

## Environment

- Disk 339 GiB free at ticket open. sol checks `df -h` at every wave start and asks the user before deleting anything.
- Ports: gateway 6300; e2e/parity use the 7300+ pool via `findFreeParityPortPair`; never the catalog ports 6012–6205.
- ≤ 6 concurrent building agents **across both tickets** (cargo lock + disk). While their W1 runs we keep ≤ 2 builders.
