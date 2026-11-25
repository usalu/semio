# Progress and Discovery Defects (D1, D2, D3)

## D1 — "progress notification for an unknown token"

### Measured ordering (native binary and full gateway, stdio, numeric progressToken)
Native `.cache/mcp-fallback-bin/semio-os-mcp` and `repo dev mcp stdio os` (gateway + native, 504-capability catalog), `capabilities_search`:

    26.051 progress {token 10, progress 0, "capabilities_search"}
    26.510 progress {token 10, progress 1, total 1, "succeeded"}
    26.510 response id 10

The terminal row was already written BEFORE the response, in order, on the same thread (job finish inside `handle_tools_call`, `finish_tool_call`), the gateway relays strictly in order, and the token is echoed with its exact JSON type (number stays number, string stays string; `progress_binding` clones the `serde_json::Value`).

### Actual cause
Row `succeeded` and the response are written back to back (0 to 1 ms apart), so the client usually reads both in one chunk. The client SDK dispatches a response synchronously (drops the progress handler) but a notification one microtask later, so "progress 1 + response in one read" makes the notification arrive after its handler was removed: "unknown token", which Claude Code treats as a connection error. The client log shows `STDIO connection dropped after 48s` at the same millisecond as that error, and context_resolve (separate reads) was unaffected, i.e. a chunking race, not a server reordering. (Client SDK behaviour is inferred from the log and the observed race; not verified in the client source.)

### Also found
Before the fix, a job that outlived its call still pushed rows under the answered token (job finishing later emitted `succeeded`); `by_job` bindings lived until the job ended.

### Fix (Rust)
- `📣️notify/🦀️.rs`: new `ProgressGate` per call (inside `ProgressBinding`, new constructor `ProgressBinding::new`). Rows are pushed only while the call is unanswered. `answer_active_call()` waits until the last pushed row is at least `minimumGapMilliseconds` (25, declared in `📣️notify/🧬️schema/🔣️.json`) old, then closes the gate and drops the job to token bindings of that request (jobs stay cancellable through `jobs_by_request`).
- `🖥️ui/📋️job-registry/🦀️.rs`: a terminal transition no longer pushes a progress row (the response / `job_get` is the terminal signal); it still releases the binding.
- `🧭️protocol/🎯️mcp-server/🦀️.rs`: `handle_tools_call` calls `answer_active_call()` right after `tools.call` returns; stdio, batch and HTTP all go through it.
- Tests: language-neutral corpus `📣️notify/🧪️tests/🔣️.json` (in-flight rows, no terminal row, late job pushes nothing, token type kept incl. `0` and strings) replayed by `the_progress_lifecycle_corpus_holds`; `answering_waits_out_the_gap_since_the_last_row`; the old terminal-row assertion was changed to expect none.

### Status
Rust compile/test: not completed, see "Not verified" below.

## D2 — empty discovery

### Fix (gateway)
- Schema `📡️stdio/🚪️gateway/🧬️schema/🔣️.json`: `DiscoveryHold.maximumMilliseconds = 20000` (below the 30 s connect and 60 s request timeouts).
- `📡️stdio/🚪️gateway/🟦️.ts`: while warming and a native server is being started (`usable` published server, staging finished, or handshake running), discovery requests (`tools/list`, `resources/list`, `resources/templates/list`, `prompts/list`) are queued like tool requests and answered by the native server; after the bound the empty list answers and list_changed follows. Cold path with nothing usable still answers empty at once. Cancel while held removes the request; handshake refusal while held answers it with -32603; after ready no hold applies (swap path untouched).
- Tests (`🧪️tests/🔣️.json` `discoveryHolds`, run in `🧪️tests/🟦️.ts`): held then real answer, held past the bound, held with handshake running, cancel while held, refusal while held, cold empty. Whole gateway suite: 60 pass, 0 fail (`repo test-file`).

### What the ~17 s consists of (measured, machine load average 20 to 30 because of concurrent cargo builds, so absolute values are inflated)
- `repo dev` bootstrap before the native server is spawned: 2.7 s resource admission, 7 s until the first cargo published-reader lease, then `repo dependencies ensure` ran inside the launch (source admission 20 s+ in the cold run, 35 s until the native server started). A warm launch skips the ensure.
- Native server start with the catalog (`SEMIO_PLUGIN_CATALOG`): 8.7 to 10.1 s to answer `initialize` (0.77 s without catalog). Sampling shows 85 % of that in `server_for_workspace_options`; 1166 of 2575 samples in `catalog::compile`, of which 1028 (40 % of the whole start) in `compute_catalog_entries_hash` (`🗂️catalog/📚️catalog-compile/🦀️.rs`, `serde_json::to_vec` of all capability definitions including schemas) in the unoptimised `prototype` profile.
- Binary snapshot copy: `copyFileSync` of the 690 MB binary measured 1 ms under Bun (APFS clone), 0 ms with FICLONE; no hashing of the binary (size, mtime, inode only; only the two small catalog files are sha256-hashed).
- Change made: `COPYFILE_FICLONE` in `createMcpPublicationProbeV1.snapshot` (`🟦️.ts`), so Node or other filesystems clone when possible instead of really copying.
- Not changed (suggestion): the catalog hash serialization is the largest avoidable native cost; options are an optimised serde_json in the prototype profile or hashing entries incrementally/caching the hash in the published catalog. This touches the Rust catalog and was left out to avoid colliding with the other agents.

## D3 — "STDIO connection dropped"
It is a consequence of D1: the drop is logged in the same millisecond as the progress error, the client treats the SDK error as a connection error. The following calls (`inference_list`, `artifact_create`, `capabilities_search`) completed in the same client session, so the client reconnects (or restarts the transport) and the server survives; there is no server-side crash or EOF. Not verified: whether the reconnect started a new native process (not observable in the client log).

## Not verified
- `cargo check`/tests for the Rust edits: started through the sanctioned ingress `SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check -p semio-os-mcp --lib` (output `🗑️generated/check.txt`); after about 15 minutes it was still checking dependency crates (machine under heavy load) and had not reached the semio-os-mcp crate, so the result is unknown. No ingress stall error was reported. The full test run (`repo run …:test`) was stopped to avoid parallel heavy builds; the new Rust tests have therefore not been executed.
- The client SDK race itself was not reproduced with a real Claude Code client.
- Warm-start timings on a quiet machine.
