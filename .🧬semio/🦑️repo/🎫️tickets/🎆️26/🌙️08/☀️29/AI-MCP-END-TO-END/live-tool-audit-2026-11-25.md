# Live Tool Audit of the semio MCP Server (2026-11-25)

Read-only audit of the 28 `mcp__semio__*` tools against
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md` and each tool's `inputSchema`.
Session: `sess_086b7202c99e28c05c04f57e16e7d7b6b0b0d6f395737921bd92a7b0bbdce8d2` (context_resolve after reconnect).
Latencies come from the newest MCP log `~/Library/Caches/claude-cli-nodejs/-Users-ueli-Documents-semio/mcp-logs-semio/2026-11-25T07-34-41-494Z.jsonl`.

Artifacts created (3 of max 10): `e2e-probe-1` (kind `os.agent.probe/v1`), `e2e-probe-2` (same kind, `initial: {"title":"probe two"}`). Nothing deleted. No destructive capability was invoked (action_invoke never got past the transport, see below).

## Transport state during the audit

- The stdio connection dropped and was re-established 25+ times. Uptime between drops: 16 s to 209 s.
- Each connect logs `[script] {"owner":"workspace-root","command":"dev","stage":"loading"|"running"}`, so every reconnect re-runs the `dev` pipeline (connect 0.8 to 2.9 s).
- Every handshake reports `serverInfo.version: "warming"`, even after tools worked.
- Log line: `Connection error: Received a progress notification for an unknown token ... "message":"succeeded"`. The client treats it as a connection error and tears the transport down.

## Per-tool table

Result key: PASS = outcome matches README and schema. PASS* = happy path correct but defects in the same tool. FAIL = outcome wrong or unusable. "err" = error path only (no happy path reachable in this audit).

| # | Tool | Input (abridged) | Result / evidence | Verdict | Latency (log) |
|---|------|------------------|-------------------|---------|---------------|
| 1 | context_resolve | `{locale:"en"}`, `{locale:"de"}` | Both return `principal agent:local`, `channel headless`, scopes list, `catalogHash`. The `de` call returns the same English-keyed body (no German text, not an error). After reconnect `activeArtifactId` became `e2e-probe-1` and `sessionId` changed. | PASS* | 23 ms, 46-122 ms |
| 2 | capabilities_search | `query:"action", limit:3` | Returns 3 items, `nextCursor:"3"`, `total:41`. | PASS |  ~24-850 ms |
| 2a | capabilities_search paging | `query:"action", limit:3, cursor:"semio.page.3"` | Returns the SAME first 3 items again (`Set Open Action` a/b/*), `nextCursor:"3"`. Cursor offset ignored. | FAIL | ~0.4 s |
| 2b | capabilities_search bad cursor | `cursor:"bogus-cursor"` | `{"nextCursor":null,"results":[],"total":0}`. README says a cursor the server did not mint is `INVALID_PARAMS`. | FAIL | ~0.4 s |
| 2c | capabilities_search limit bounds | `limit:0`, `limit:101` | Schema says min 1, max 100. `limit:0` returned a 1-item page (`nextCursor:"1"`), `limit:101` was accepted. Silent clamp, no error. | FAIL |  - |
| 2d | capabilities_search no hits | `query:"zzqxnomatchterm"` | `{"nextCursor":null,"results":[],"total":0}` | PASS | - |
| 2e | capabilities_search filters | `kind:["s.probe"]`; `owner:"norm"`; `artifactKind`, `requiresScope` combined | Filters accepted. `owner:"norm"` returned total 112. | PASS* | - |
| 3 | capabilities_describe | `artifact.create`, `artifact.open`, `artifact.export`, `capabilities.search` | Full CapabilityDefinition returned. `artifact.create` describe shows a `template` enum of about 300 values that is absent from the MCP `inputSchema`. | PASS* | 41-71 ms |
| 3a | capabilities_describe unknown id | `zz.does.not.exist/v1` | Plain text `no such capability: zz.does.not.exist/v1`. No `code`, no `retryable`. | FAIL (error shape) | 0 s, but 14 s on another call |
| 4 | artifact_create | `e2e-probe-1`, `kind:"os.agent.probe/v1"` | `status:SUCCEEDED`, `revision.cursor "1"`, `headEditId`, `jobId job_7b9c85...`. | PASS |  130 ms (up to 1 s) |
| 4a | artifact_create initial | `e2e-probe-2`, `initial:{"title":"probe two"}` | SUCCEEDED, `jobId job_89f649...`. Whether `initial` was applied was not checked (no snapshot taken). | PASS* | 344 ms |
| 4b | artifact_create missing kind | `artifactId` only | Plain `kind is required`. | FAIL (error shape) | 0 s |
| 4c | artifact_create wrong type | `kind:42` | Plain `no installed plugin declares artifact kind \`42\``. The server reports a domain error, not a type error (schema says `kind` is a string). | FAIL | 1 s |
| 4d | artifact_create duplicate | `artifactId:"e2e-probe-1"` | Plain `artifact \`e2e-probe-1\` already exists`. No conflict code. | FAIL (error shape) | 0 s |
| 5 | artifact_open | `e2e-probe-1` | `artifactKind:null` but `kind:"os.agent.probe/v1"` (contradictory). `sizeBytes:412`, consistent with snapshot pack 4 + spr 408. Unknown id: plain `no such artifact: e2e-does-not-exist`. | PASS* | 64 ms |
| 6 | artifact_snapshot | `e2e-probe-1` | Returns `untrusted` envelope (`schema semio.mcp.untrusted-content/v1`, `notice` bilingual, provenance with `contentSha256`, `authors.local-principal`). Pack is literal `null` (`bnVsbA==`). | PASS |  125-235 ms |
| 6a | artifact_snapshot current revision given | `revision` equal to current | Same payload. | PASS | 125 ms |
| 6b | artifact_snapshot unknown revision | `cursor:"999", headEditId:"edit-bogus"` | Fails after 32 s: `only the artifact's current revision can be snapshotted — historical revision snapshots are not retrievable yet`. Plain text. Slow failure. | FAIL | 32 s |
| 7 | artifact_validate | `e2e-probe-1` | `valid:true`, `validation:"canonical-pair"`, `diagnostics:[]`, untrusted envelope. Unknown id: plain `no such artifact`. | PASS | 73 ms |
| 8 | artifact_export | `e2e-probe-1`; `format:"no-such-format"` | Both fail with `artifact \`e2e-probe-1\` cannot be matched to one of this workspace's 49 registered plugins (...) — no artifact-to-plugin mapping exists yet`. The message dumps ~1 KB of plugin names. Bad `format` is never validated: the plugin lookup runs first. Expected for a probe kind, but untyped. | FAIL (error shape; export not reachable for probe) | 0 s |
| 9 | action_prepare | `capabilityId:"capabilities.search"`, `input:{query}` | 8 attempts, every one `Connection closed` (0-20 s). Only `unknown capability: zz.nope/v1` (18 s) and `capabilityId is required` reached the server. Happy path NOT reached. | FAIL (transport) | 0-20 s |
| 9a | action_prepare unknown | `capabilityId:"zz.nope/v1"` | Plain `unknown capability: zz.nope/v1`. | FAIL (error shape) | 18 s |
| 9b | action_prepare missing capabilityId | `{input:{}}` | Plain `capabilityId is required`. | FAIL (error shape) | 0 s |
| 10 | action_invoke | `capabilities.search`, `idempotencyKey` | `Connection closed` after 24 s. Not reachable. | FAIL (transport) | 24 s |
| 11 | action_cancel | `prep_does_not_exist` | Plain `unknown handle: prep_does_not_exist`. Happy path not reachable. | PASS* (error shape) | 23 s |
| 12 | transaction_begin | `preparedHandles:[]`; `["prep_fake_1"]` | `transaction.begin requires at least one prepared handle`; `unknown handle: prep_fake_1`. Both plain. Happy path not reachable (no prepared handle). | err |  0 s |
| 13 | transaction_commit | `tx_does_not_exist` | Plain `unknown handle`. Happy path not reachable. | err | 0 s |
| 14 | transaction_rollback | `tx_does_not_exist` | Plain `unknown handle`. Happy path not reachable. | err | 0 s |
| 15 | history_undo | `undo_does_not_exist` | Plain `unknown handle`. No committed invocation existed, so the happy path was not reachable. | err | 0 s |
| 16 | history_redo | `undo_does_not_exist` | Plain `unknown handle`. | err | 0 s |
| 17 | inference_list (unfiltered) | `{}` | About 40 KB. Every declared service embeds full `inputSchema` and `outputSchema` as escaped JSON strings (gis, procedural, stdio gltf 50+ entries, wfc, cad, flow). Result is not paged. | FAIL (size) | 2-4 s |
| 17a | inference_list artifact-scoped | `artifactId:"e2e-probe-1"` | `{"artifactId":...,"artifactKind":"os.agent.probe/v1","declared":[]}`. Correct. | PASS | 0.3 s |
| 18 | inference_get | `e2e-probe-1`, `no.such.schema/v1` | Plain `no inference service \`no.such.schema/v1\` is declared for artifact kind \`os.agent.probe/v1\``. README calls this a typed, retryable gap. | FAIL (error shape) | 6 s |
| 19 | inference_run | pluginId `probe`, kind probe, bogus schema | Plain `no inference service ... declared`. Happy path not reached (no declared service for the probe kind). | err | 7 s |
| 20 | inference_submit | `documentId:"e2e-probe-1"`, bogus schema | Plain `no inference service ... declared`. | err | 5 s |
| 21 | inference_events | `job_nonexistent` | Plain `unknown handle: job_nonexistent`. | err | 0 s |
| 22 | inference_cancel | `job_nonexistent` | Plain `unknown handle`. | err | 0 s |
| 23 | inference_approve | `job_nonexistent`, zero hash | Plain `unknown handle`. | err | 0 s |
| 24 | job_get | `job_7b9c85ced3118a7c08a4e8da01` (jobId returned by artifact_create) | `no such job: job_7b9c85...`. The job id the create call returned cannot be read back. | FAIL | 0 s |
| 24a | job_get unknown | `job_nonexistent` | Plain `no such job`. | FAIL (error shape) | 0 s |
| 25 | job_cancel | `job_nonexistent` | Plain `no such job`. | err | 0 s |
| 26 | ui_focus | no args; `windowId:"win_does_not_exist"` | Both: `no shell is attached to \`/bridge\` yet — start \`bun run dashboard run playground:s\` (it dials this gateway on its own) and retry`. Plain text, no `PLUGIN_UNAVAILABLE`, no `retryable`. The suggested command uses `bun run`, which breaks the repo rule that every command goes through `repo`. | FAIL | 0 s |
| 27 | ui_reveal | `anchor:"left"`; `anchor:"middle"` | `left` gives the no-shell error above. `middle` gives plain `anchor must be one of left|right|top|bottom`, although the schema enum already declares this. | FAIL (error shape / schema not enforced) | 0 s |
| 28 | conversation_reply | `text`, `replyId:"e2e-reply-1"`, `complete:true` | `{"complete":true,"ok":true,"replyId":"e2e-reply-1","shells":0}`. Matches README (shells:0 is not an error). Missing `text`: plain `text is required`. Streaming (`complete:false`) not exercised. | PASS | 202 ms |

Not exercised: a successful action_prepare/invoke, history undo/redo on a real undo token, transaction commit/rollback with real handles, inference_run/submit/events/approve on a real declared service, job_get on a live job, ui_focus/ui_reveal with an attached shell, streaming conversation_reply. All were blocked by transport churn (action tools) or by missing services/shell.

## Ranked defect list

1. **stdio connection churn blocks every action tool.** Connection dropped every 16 to 209 s. action_prepare failed 8 of 8 times and action_invoke 1 of 1 with `Connection closed` after 0 to 24 s. Log: `STDIO connection closed after 24s (cleanly)` then `Failed to fetch tools: Connection closed`.
2. **Progress notification for an unknown token kills the transport.** Log: `Connection error: Received a progress notification for an unknown token: [REDACTED],"params":{"progress":1,"total":1,"message":"succeeded"...}`. The server sends a final progress row for a token the client no longer tracks (README: "ending with one final row at 1.0").
3. **Every connect re-runs the dev pipeline and reports `version: "warming"`.** Log on each connect: `[script] {"owner":"workspace-root","command":"dev","stage":"loading"}` then `"running"`. Handshake: `serverInfo.version: "warming"`. README says warming should end once preparation finishes, so the launcher is rebuilding on every reconnect.
4. **capabilities_search cursor is ignored.** `cursor:"semio.page.3"` returns the same first 3 items. `cursor:"bogus-cursor"` returns an empty page instead of `INVALID_PARAMS`.
5. **No typed errors anywhere.** Every error is plain text with no `code`, no `retryable`, no `details`. README promises typed errors (`PLUGIN_UNAVAILABLE`, `INVALID_PARAMS`, `REVISION_CONFLICT`, `PERMISSION_DENIED`, `APPROVAL_REQUIRED`). Examples: `no such artifact: e2e-does-not-exist`, `kind is required`, `already exists`, `unknown handle: undo_does_not_exist`, `no such job: job_nonexistent`.
6. **Schema constraints are not enforced server-side.** `limit:0` and `limit:101` are accepted (schema 1..100). `kind:42` reaches the domain code (`no installed plugin declares artifact kind \`42\``). `anchor:"middle"` is rejected only by a hand-written check.
7. **ui_focus / ui_reveal error text tells the dev to run `bun run dashboard run playground:s`.** This contradicts CLAUDE.md (use `repo`, never `repo dashboard`) and README (`repo dev mcp stdio os`). The README promises `PLUGIN_UNAVAILABLE`, retryable. The actual response is plain text.
8. **artifact_create returns a jobId that job_get cannot read.** `job_7b9c85ced3118a7c08a4e8da01` returned by create, then `no such job`. Also job_cancel on that id is unverified.
9. **inference_list unfiltered returns about 40 KB** with full JSON Schema strings (inputSchema and outputSchema) for about 90 declared services. Not paged, even though the README says list tools are paginated. Should return metadata only and let a detail call fetch schemas.
10. **artifact_open returns `artifactKind: null` alongside `kind: "os.agent.probe/v1"`.** Two fields disagree for the same artifact; snapshot provenance carries the kind correctly.
11. **artifact_snapshot with an unknown revision takes 32 s to fail.** Message: `only the artifact's current revision can be snapshotted — historical revision snapshots are not retrievable yet`. Should fail fast with a typed `UNSUPPORTED` or `NOT_FOUND`.
12. **artifact_export validates the plugin before the format, and dumps 49 plugin names.** `format:"no-such-format"` gives the same plugin error as no format. The error text lists about 1 KB of plugin ids.
13. **Missing German text.** `capabilities_search` and `capabilities_describe` for `capabilities.search` are English only (`"description":"Deterministic BM25 search over the compiled capability catalog — no LLM."`). artifact.* descriptions are bilingual. Errors are English only in every case, although the README requires bilingual text for the UI layer. (The untrusted-content `notice` is bilingual.)
14. **Empty descriptions.** `stdio-pdf.s.stdio.pdf@1.7/*#editor.set-open-action` (and the `a`, `e`, `h`, `ua`, `vt`, `x` variants) has `"description":""`. Also `energy.s.energy.model@1/*#editor.select-weather-file` and `clear-weather-file`.
15. **Duplicate titles and capabilities.** `Create Artifact` (gateway `artifact.create` and `space.s.space.space@1/*#editor.createArtifact`), `Open Artifact` (gateway `artifact.open` and space `openArtifact`), `Import Artifact File` and `Add Widget` (procedural and demonstrator with identical descriptions). `Viewport Action` appears in 40+ appId variants. These flood search results: a `capabilities_search` for `action` has 41 hits, most of them `Viewport Action` or `Set Open Action`.
16. **artifact.create describe exposes a `template` enum (about 300 values) that the MCP `inputSchema` omits.** The published schema and the described schema disagree.
17. **Inconsistent owner ids.** inference_list uses `cad-extension-aec-building` and `flow-extension-brep` as owners. The artifact_export error lists 49 registered plugins that include neither id. Either the list is wrong or the owners are unregistered.
18. **Latency spikes on simple calls.** `capabilities_describe` for a nonexistent id took 14 s on one call. `action_cancel` on an unknown handle took 23 s. `context_resolve` took 12 s once. These coincide with reconnects, so the cause is likely item 1.
19. **Active-artifact side effect not documented.** After `artifact_create`, `context_resolve` reported `activeArtifactId: "e2e-probe-1"` following a reconnect. The README does not say creating an artifact binds the session to it.
20. **`context_resolve` sessionId changes on every reconnect** (`sess_2d176f...` to `sess_086b72...`). Handles from the old session are invalid, which is not surfaced as an error.
