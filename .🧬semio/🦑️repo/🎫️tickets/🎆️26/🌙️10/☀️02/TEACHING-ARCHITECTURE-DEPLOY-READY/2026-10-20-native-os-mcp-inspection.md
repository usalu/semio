# Native OS MCP Inspection

No commands ran. This source inspection used filesystem APIs and left retained tools and private Source unchanged.

## Finding

The OS MCP route does not establish an authoritative way to inspect the existing builder/course process. Its 28 stable gateway tools contain no host-process inventory, PID inspection, container inspect/status or Docker operation. Native-runtime independently found only fresh-child process observer ownership, not existing-PID reattachment, and no callable OS container port.

The closest concrete read-only tools are:

| Tool | Required args | Actual effect / limitation |
|---|---|---|
| `capabilities_search` | `{query:string}` | Deterministic BM25 search over the configured plugin/gateway catalog; optional kind[], owner, artifactKind, requiresScope, limit1–100 and cursor. Does not enumerate host processes or containers. |
| `capabilities_describe` | `{capabilityId:string}` | Reads one full capability declaration. No known host inspector capability was found. |
| `job_get` | `{jobId:string}` | Reads `job_registry().snapshot(jobId)` in this gateway process. Unknown IDs return NOT_FOUND. It accepts neither PID nor container identity and cannot attach to the old course runner. |

All three schemas close additional properties. Their declarations have default effects/policy. `job_get` is a gateway job registry, not an OS process census. Generic `tools/list` is read-only protocol discovery. A live plugin catalog could contain additional capabilities, but source inspection has not established a process/container inspector in that catalog; guessing an ID is not justified.

## Native Launch Path And Effects

`repo dev mcp stdio os [extra]` is admitted as native workspace ingress in `⌨️cli/🧭️repo-cli/🚪️entrypoint/🦀️.rs:9` (Go twin line185). It reaches root workspace router → `DevScript.runMcpOs` in repo library `🎛️commands/🛠️dev/🟦️.ts`. This imports `@semio-tech/framework-os-mcp`, runs a staged server if fresh, otherwise starts `serveWarmMcpStdio`. The warm path **starts `repo run @semio-tech/framework-os-mcp-rs:build --output=stream` asynchronously**, then serves the resulting native `semio-os-mcp stdio`. It is not an inspection-only launch.

The default staged `🌉️mcp/📦️packages/🦀️rust/dist/build` directory is absent in this filesystem snapshot. Explicit `SEMIO_OS_MCP_BIN` changes resolution and skips staging; this inspection did not establish any supplied override. Starting the ordinary route can therefore create a concurrent build.

Current Rust `parse_stdio_args` requires **`--host-driver-policy <JSON path>`**, parsed with duplicate members rejected; omitted policy refuses. This required flag is absent from the older README usage prose. `--no-bridge` suppresses loopback listener and bridge-offer publication. Omitting `--folder` and `--hub` avoids opening a headless document workspace; folder binding can create the folder and event store. `run_stdio` still creates a file audit sink under `~/.semio/agent/audit`; without `--no-bridge`, it starts a loopback listener and publishes an owner-only offer. No `--audit-dir` option exists in stdio mode (HTTP has one).

## Recommendation

Do not start this route to recover course22 process state: no authoritative inspector was found, default staged output is absent, and its warm startup can build concurrently. A later semantic MCP discovery could use no folder/hub, `--no-bridge`, default never approval and a reviewed host-driver policy, after staging and audit-location constraints are settled. It would still establish semantic catalog facts only. Root must continue the native container/process port mapping or implement a narrowly owned read-only observation surface before any retry/cancel/Source mutation.

## Primary Source Locations

- Native workspace ingress: `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧭️repo-cli/🚪️entrypoint/🦀️.rs`.
- Launcher: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts`.
- Staging: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts`.
- Tool census/run_stdio: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs`.
- Mandatory stdio policy parser: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs`.
- Core schemas: `🌉️mcp/🧬️schema/🧭️core-and-mutation-tool-schemas/🦀️.rs`; job schema `🌉️mcp/🧬️schema/🖥️ui-and-catalog-tool-schemas/🦀️.rs`.
- Job snapshot handler: `🌉️mcp/🖥️ui/🦀️.rs:82`.
