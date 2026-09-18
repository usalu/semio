# 📓️ M1 — both `.mcp.json` MCP servers start and answer Claude Code

Slice M1 of `OS-HUB-COLLABORATION-AI-END-TO-END`, executing `📓️audit-ai-mcp.md` §7 P0.1, P0.2, P1.3 and P2.7.
All commands run from `/Users/ueli/Documents/semio`. Result: **both servers answer `initialize` +
`notifications/initialized` + `tools/list` + `resources/list` from a Claude-Code-shaped client**, and the
`semio` gateway additionally serves a real read against a bound headless workspace.

---

## 1. Root causes found (five, not the two the audit predicted)

| # | Server | Root cause | Evidence |
| --- | --- | --- | --- |
| R1 | `semio` (os) | The process-entry credential seal substring-matched `TOKEN`/`SESSION`/`CREDENTIAL`/`BEARER`/`CAPABILITY`/`AUTHORIZATION`/`COOKIE` across the **whole** environment, so `CLAUDE_CODE_SESSION_ID`, `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_HOST_SESSION_ID`, `CLAUDE_CODE_SESSION_ATTENDED` each tripped it. Exit 1 with `protected parent environment was not sealed` before any argv parsing. | audit §1b; `🌉️mcp/🏗️bootstrap/🦀️.rs:104-118` (old) |
| R2 | `repo` | `resolveMcpBin()` pointed at the flat, checked-in-shaped path `💻️client/mcp`, which nothing ever produced. Two Go modules both declared `module github.com/usalu/semio/repo/mcp`; `go.work` listed the stale one (`🔌️mcp/📦️packages/🐹️go` + its `🚀️bin`) and never the canonical one (`💻️client/🔌️mcp`), so `go test`/`go build` in the canonical module failed at setup. | audit §1a; `go.work:23-24` (old) |
| R3 | `repo` | `📜️script.ts`'s `DevScript.run` resolved the playground-app catalog **before** checking the `mcp` verb, so every `dev mcp stdio …` launch paid a full catalog walk. Measured: the server binary only started ~25 s after spawn — past a client's MCP connect budget, and past the point where a piped `initialize` had already hit EOF (`-32004 session closed`). | measured below, §4 |
| R4 | `repo` | The 26/09/06 `initialize` strict-decode defect had **regressed**: `DecodeParams` → `decodeExact` with `DisallowUnknownFields` rejected any client sending `clientInfo.title`, `capabilities.roots.listChanged`, `sampling`, `elicitation` or any later field — i.e. exactly what Claude Code sends — with `-32602 invalid initialize params`. | reproduced §4 |
| R5 | `repo` | Tool-surface drift: the canonical module exposed 6 tools, missing `goal_open`/`goal_close`/`goal_reopen` that the 26/09/06 ticket documents as the 9-tool canonical set (the stale duplicate module still had all 9). | audit §2/§7 P1.3 |

Two further compile blockers were peer churn in shared crates and were repaired in place so the `semio`
binary could be rebuilt at all (`ShellToGateway::AgentMessage` was fixed by a peer mid-session;
`DirectorySessionAuthorityV1::expires_at_ms` likewise; the `ArtifactCompositionFields` bound was not — see
C3 below).

---

## 2. Changes

### `semio` (os) — precise credential seal (R1)

`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs`

- `:113` `PROTECTED_CREDENTIAL_NAMES` — the six exact hub-authority variable names the OS/hub actually
  export: `S_USER`, `VITE_S_USER` (`🏛️ShellHost`'s identity pin), `S_HUB_URL`, `VITE_S_HUB_URL`
  (`💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:216`), `S_SESSION`, `VITE_S_SESSION`.
- `:116` `PROTECTED_CREDENTIAL_HEADER_NAMES` — `AUTHORIZATION`, `COOKIE`: authority by their own bare name.
- `:120` `PROTECTED_CREDENTIAL_NAMESPACES` — `S_`, `VITE_S_`: the only namespaces this product mints.
- `:123` `PROTECTED_CREDENTIAL_MARKERS` — the seven credential words, now matched **only inside** a
  namespace above, never bare across the environment.
- `:130` `protected_credential_environment_name(key)` — the new pure predicate.
- `:140-147` `protected_credential_environment_is_absent()` keeps its shape (the
  `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order` source-order gate requires exactly one definition of
  this function containing `return value == "3";`) and now delegates to the predicate.

The defence is preserved and still strictly stronger than needed: the sealing side
(`🌎️hub/🔐️auth/📤️credential-delivery/🟦️.ts:26` `isProtectedDirectChildEnvironmentKey`) stays broad and
*strips* more than this guard *rejects*, which is the safe asymmetry — the guard asserts absence of the hub
carriers, it does not have to re-derive the strip list.

Regression test: `🌉️mcp/🧪️tests/🔬️bin-quick/🦀️.rs`
`the_process_entry_seal_rejects_hub_carriers_and_admits_host_harness_variables` — nine carriers
(`S_USER`, `VITE_S_USER`, `S_HUB_URL`, `VITE_S_HUB_URL`, `S_SESSION`, `S_BRIDGE_TOKEN`,
`VITE_S_CAPABILITY_GRANT`, `AUTHORIZATION`, `COOKIE`) must trip it; thirteen benign names
(`CLAUDE_CODE_SESSION_ID`, `CLAUDE_CODE_HOST_SESSION_ID`, `CLAUDE_CODE_MESSAGING_TOKEN`,
`CLAUDE_CODE_SESSION_ATTENDED`, `VSCODE_SESSION_ID`, `GH_TOKEN`, `NPM_TOKEN`, `SESSION_MANAGER`,
`SEMIO_DIRECT_CHILD_BENIGN`, `SEMIO_PLAYGROUND_SESSION_OUTPUT_ROOT`, `S_DATA_DIR`, `S_OS_MCP_PORT`,
`PATH`) must not.

### `repo` — one Go module, one output path (R2)

- **Deleted** `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/` (the stale duplicate module,
  `🐹️.go` 2184 lines + `🚀️bin` sub-module + `📋️project.json` `@semio-tech/repo-mcp-go` + `📜️script.ts`).
  Kept `💻️client/🔌️mcp` — the richer, modular, tested implementation (`🖥️server` 860 / `🗄️repository` 368 /
  `📜️protocol` 368 / `📡️event` 193 / `🚚️transport` 140 / `🧩️component` 72 lines plus a 600+ line
  `🤝️protocol-contract` suite), and the one the repo taxonomy already names as canonical
  (`📚️library/🔣️taxonomy.json:23761` `repo-mcp-go-module` → `💻️client/🔌️mcp/go.mod`).
- `go.work` — removed both stale entries, added `💻️client/🔌️mcp`, `💻️client/⌨️cli` and `📚️library` (the
  module's own `replace` targets, previously absent from the workspace, which is why
  `go test` reported *"current directory is contained in a module that is not one of the workspace modules"*).
- `🦑️repo/🔨️modules/📚️library/🟦️.ts:152` `defaultMcpBin()` → `.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp`
  (gitignored: `.gitignore:43`), so consumer and producer agree on one cache path and no binary is ever
  checked in.
- **New** `💻️client/🔌️mcp/📜️script.ts` — `build` (`runCanonicalGoBuild(MODULE_DIR, ["-o", resolveMcpBin(root), "."])`)
  and `test` (`runCanonicalGoTests`). Both go through the canonical Go projection because this module's
  sources are taxonomy-named `<domain>/🐹️.go` files overlaid into the module root.
- `💻️client/🔌️mcp/📋️project.json` — added the `build` target (`bun ./📜️script.ts build`, project `repo-mcp`);
  its existing `test`/`test-quick`/`test-long`/`test-exhaustive` targets already route to the root script,
  and the workspace aggregate `📋️project.json:1187-1199` already depends on `test-quick` of `*`, so this
  module's Go suite is now in `workspace:test-quick` with no further wiring.
- References repointed: `📜️script.ts:231-235` (`requireRepoMcpBinary` hint → `bun nx run repo-mcp:build`),
  `📚️library/⚡️caching/🚀️bootstrap/📜️script.ts:254` (`@semio-tech/repo-mcp-go` → `repo-mcp`),
  `.vscode/🧩️launch.seed.jsonc` + `.vscode/launch.json` (`🦑️mcp dev` → `bun ./📜️script.ts dev mcp stdio client`;
  `🛠️dev🧰️repo🔌️mcp🐹️go` → `🛠️build🧰️repo🔌️mcp` / `bun nx run repo-mcp:build`),
  `🔌️mcp/🧪️tests/🤝️jsonrpc-handshake/🐹️.go:46,56-57` and `🔌️mcp/🧪️tests/📋️capability-listing/🐹️.go:33-34`
  (their build fallback now runs `bun ./📜️script.ts build` in `💻️client/🔌️mcp`).
- **Restored** `💻️client/🔌️mcp/🧫️fixtures/{2️⃣g2-contract.json,🚪️entrypoint-contract.json}` from
  `git show bb961413d4^:…` — they had been deleted in auto-commit `bb961413d4`, which is why
  `TestRepositoryEntrypointContract` and `TestG2CanonicalGoldenVectors` failed at `os.ReadFile`.

### `repo` — startup latency (R3)

`📜️script.ts` `DevScript.run` — the `segments[0] === "mcp"` arm moved **above** `resolvePlaygroundDevApp()`.
`mcp` is a reserved verb, never a playground app, so the catalog walk was pure cost on the one path that
must answer a client handshake promptly. Also `runMcpStdioRepo` now passes its env through
`daemonBudgetOpts({ GOWORK, SEMIO_REPO_MCP_CLIENT })` instead of an `env:` key the `...daemonBudgetOpts()`
spread silently overwrote — which is why the profile never reached the server (`serverInfo.name` was
`repo` for every profile).

### `repo` — forward-extensible `initialize` (R4)

- `💻️client/🔌️mcp/📜️protocol/🐹️.go:168-190` — new `DecodeOpenParams`: same object/trailing-value checks as
  `decodeExact`, without `DisallowUnknownFields`. Used for the handshake only; every other frame keeps the
  hostile strict decoder.
- `💻️client/🔌️mcp/📜️protocol/🐹️.go` `Implementation` — added the spec's optional `title`.
- `💻️client/🔌️mcp/🖥️server/🐹️.go:416` — `initialize` decodes with `DecodeOpenParams`.
- `💻️client/🔌️mcp/🧪️tests/🤝️protocol-contract/🐹️.go` — new
  `TestInitializeAdmitsForwardExtensibleClientFrames`, driven by the `🔌️mcp` module's own
  `🧫️fixtures/🤝️initialize-lenient.json` (`clientInfo.title`, `capabilities.{roots,sampling,elicitation,tasks}`),
  and asserting in the same test that a non-`initialize` frame with an unknown field is still rejected.

### `repo` — canonical 9-tool surface (R5)

`💻️client/🔌️mcp/🗄️repository/🐹️.go:142-181` (dispatch) and `:344-346` (schemas) — added `goal_open`
(`client.ToolGoalCreate`), `goal_close` (`client.ToolGoalClose`), `goal_reopen` (`client.ToolGoalReopen`),
matching the 26/09/06 documented set `file_integrate, goal_close, goal_open, goal_reopen, section_extract,
section_move, ticket_close, ticket_open, ticket_reopen`. The `repo://goals` listing resource and the other
seven `repo://…` resources were already registered (`:381-390`) and are verified present below.

### Peer-churn compile repair (C3)

`💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:202-206` — a peer added an `ArtifactCompositionFields` bound to
`ArtifactStore::undo`/`redo` via `store::SpaceMember`; `ProbeSnapshot` (an opaque `serde_json::Value` leaf
with no child or link slots) did not implement it, so `undo_probe_mutation`/`redo_probe_mutation` no longer
compiled. Added the empty projection impl. Additive only, no existing behaviour changed.

---

## 3. Commands and output tails

```
$ bun nx run repo-mcp:build
> nx run repo-mcp:build
> bun ./📜️script.ts build
 NX   Successfully ran target build for project repo-mcp
  Run duration: 24.3s
$ ls -l .🧬semio/🦑️repo/⚡️cache/🗃️bin/
-rwxr-xr-x@ 1 ueli staff 12812466 Sep 18 21:29 semio-repo-mcp
```

```
$ bun ./📜️script.ts test quick repo-mcp
ok  	github.com/usalu/semio/repo/mcp	0.242s
ok  	github.com/usalu/semio/repo/client	(cached)
ok  	github.com/usalu/semio/repo/client/internal/command	(cached) [no tests to run]
ok  	github.com/usalu/semio/repo/client/internal/eventstore	(cached) [no tests to run]
```
(`test long repo-mcp` is identical plus `TestMcpStdioInitializeHandshake`, which is the `💻️client/⌨️cli`
case that spawns `bun ./📜️script.ts dev mcp stdio cursor` — red before R3/the env fix, green after.)

```
$ cargo build --message-format short --manifest-path 🧰️framework/…/🌉️mcp/📦️packages/🦀️rust/Cargo.toml \
    --package semio-framework-os-mcp --bin semio-os-mcp
    Finished `dev` profile [unoptimized] target(s) in 1m 26s

$ cargo test --message-format short … --bin semio-os-mcp
running 2 tests
test quick::the_process_entry_seal_rejects_hub_carriers_and_admits_host_harness_variables ... ok
test quick::authenticated_hub_workspace_cli_contains_no_hub_credential_carrier ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ bun nx run @semio-tech/framework-os-mcp-rs:build
 NX   Successfully ran target build for project @semio-tech/framework-os-mcp-rs and 3 tasks it depends on
$ ls -lT 🧰️framework/…/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp
-rwxr-xr-x@ 1 ueli staff 70100320 Sep 18 21:53:41 2026
```

Before/after on R4 (same frame, the one Claude Code sends):
```
$ printf '{…"clientInfo":{"name":"claude-code","title":"Claude Code","version":"2.0"},"extraUnknown":1}}\n' | semio-repo-mcp
before: {"jsonrpc":"2.0","id":1,"error":{"code":-32602,"message":"invalid initialize params"}}
after : {"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-06-18",…,"serverInfo":{"name":"repo","version":"1.0.0"},…}}
```

Before/after on R3 (wall-clock to first byte, `dev mcp stdio cursor`, warm bun cache):
```
before: start 1789760506 → reply 1789760531   (25 s; reply was -32004 session closed, stdin had EOF'd)
after : start 1789760567 → reply 1789760569   ( 2 s; reply was a real initialize result, name repo-cursor)
```

---

## 4. Handshake probe — `🐍️m1-mcp-handshake.ts`

`bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️m1-mcp-handshake.ts`

Reads the literal `command`/`args` out of `.mcp.json`, spawns each server with the **current environment
plus `CLAUDE_CODE_SESSION_ID=x CLAUDE_CODE_MESSAGING_TOKEN=y`**, and drives
`initialize` (protocolVersion `2025-06-18`, `clientInfo.title` + `roots`/`sampling`/`elicitation`
capabilities) → `notifications/initialized` → `tools/list` → `resources/list`. For `semio` it appends
`--folder <repo root>` to bind a real headless workspace, then calls the read tool `capabilities_search`
and reads the `semio://workspace` resource.

```
=== repo — bun ./📜️script.ts dev mcp stdio client
  initialize      ok — server=repo@1.0.0 protocol=2025-06-18
  tools/list      ok — 9 tools: file_integrate, goal_close, goal_open, goal_reopen, section_extract,
                       section_move, ticket_close, ticket_open, ticket_reopen
  resources/list  ok — 8 resources: repo://, repo://bundles, repo://contributors, repo://files,
                       repo://folders, repo://goals, repo://policies, repo://tickets

=== semio (os, headless workspace bound to the repo root) — bun ./📜️script.ts dev mcp stdio os --folder /Users/ueli/Documents/semio
  initialize      ok — server=semio-os-mcp@0.1.0 protocol=2025-06-18
  tools/list      ok — 27 tools: action_cancel, action_invoke, action_prepare, artifact_create,
                       artifact_export, artifact_open, artifact_snapshot, artifact_validate,
                       capabilities_describe, capabilities_search, context_resolve, history_redo,
                       history_undo, inference_approve, inference_cancel, inference_events, inference_get,
                       inference_list, inference_run, inference_submit, job_cancel, job_get,
                       transaction_begin, transaction_commit, transaction_rollback, ui_focus, ui_reveal
  resources/list  ok — 8 resources: semio://capability, semio://ui/active-context,
                       semio://ui/agent-messages, semio://ui/selection, semio://window,
                       semio://workspace, semio://workspace/artifacts, semio://workspace/artifacts
  capabilities_search ok — isError=false hits=0
  resources/read semio://workspace ok — {"artifacts":[],"localPolicyPrincipal":"agent:local","origin":"folder:///Users/ueli/Documents/semio"}
  stderr tail: [mcp registry] catalog compile failed (duplicate capability id:
    architect.s.architect.program@1/*#editor.setAdjacencyKind) — falling back to gateway-only capabilities |
    [semio-os-mcp] real per-capability ArtifactChannel routing bound for folder /Users/ueli/Documents/semio

both .mcp.json servers answered initialize + tools/list + resources/list
```

---

## 5. Ticket annotation (P2.7)

Created `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/📓️status.md`
(the ticket had none) with a dated 2026-09-18 note, ticket left **closed**. It records the four
`📓️closing-summary.md` claims the live tree contradicts — the Rust-default `.mcp.json` entry point and
`SEMIO_REPO_IMPLEMENTATION` switch (absent outside `🎛️dashboard`), `💻️client` "removed" (present and now
canonical), the 9-tool parity (was 6 until today), and the `initialize` defect this very ticket claims to
have fixed (regressed; re-fixed and pinned today) — and points at this ticket and this report.

---

## 6. Left standing (named, not hidden)

1. **`capabilities_search` returns 0 hits.** The gateway falls back to gateway-only capabilities because the
   plugin catalog fails to compile: `duplicate capability id:
   architect.s.architect.program@1/*#editor.setAdjacencyKind`, plus ~20 `[mcp registry] skipping plugin …`
   lines (`missing field windowKindId` / `executionProtocol` / `artifactSchema`, and a `🧱️block` plugin with
   no `🔣️.json` at all). That is plugin-registry/descriptor drift owned by the plugins slice, not MCP
   transport; the tool answers structurally (`isError=false`) and every other surface is unaffected.
2. **`⌨️cli/📦️packages/🐹️go/🔬️_test.go:225-280`** asserts that `.devcontainer/post-create.sh` and the two
   `🔩️native/🥾️bootstrap` scripts build `🔌️mcp/📦️packages/🐹️go/🚀️bin` *and forbid the literal `💻️client`*.
   Those assertions were already dead before this slice — `.devcontainer/post-create.sh` does not exist and
   neither bootstrap script contains any of the required fragments (`grep -c` → 0) — and they encode the
   opposite of the taxonomy's own `repo-mcp-go-module` declaration. Left untouched: they belong to the
   unlanded 26/09/06 migration, and reconciling that migration is a ticket of its own.
3. **`📚️library/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json:4960-4966`** carries a synthetic
   `💻️client/🔌️mcp/📦️packages/🐹️go` row (sha256 of the empty string, verdict `blocked-empty-package-root`)
   for a path that has never existed. Fixture data for the purity authority, not a live reference.
4. **`🌎️hub/🧪️tests/🧱️foundation-source` has one pre-existing failure** — the launch entry
   `⚖️gate🧱️hub-foundations📐️source` is missing from both `.vscode/🧩️launch.seed.jsonc` and
   `.vscode/launch.json`. Unrelated to the two repo-mcp entries this slice edited; the
   `mcpCredentialSourceOrderConforms` assertions in the same file (which gate the shape of the seal changed
   here) pass.
5. **The `🔌️mcp` module's Rust twin** (`🔌️mcp/📦️packages/🦀️rust`, crate `semio-framework-repo-mcp`, with a
   `semio-repo-mcp` bin declared in `⌨️cli/📦️packages/🦀️rust/Cargo.toml:22`) is untouched and still not what
   `.mcp.json` launches. Deciding whether the Rust twin ever becomes the default — the thing 26/09/06
   claimed — is out of this slice.
