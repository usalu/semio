# MCP Client Journey Terminal Audit

The configured end-to-end target and direct diagnostic caller both use `driveOsMcpClientSessionV1`. A replay with an active job, unchanged document bytes, and a terminal cancellation for a foreign job ID gave both cancellation checks a passing result before this change. The same path also accepted a foreign echoed cancellation handle. The verifier now correlates every submitted, observed and cancelled page with its exact handle, job ID, document, inference service, artifact kind and plugin. A page that does not identify the original job cannot establish active progress or terminal cancellation.

The canonical combined gate now requires the OS cancellation terminal check and Repo's `repo://goals` read for an otherwise green run. The existing early exits generally record a failed step before returning. The new test checks the previously unguarded empty and all-green incomplete prefixes as well as a full success and an existing failure. Existing failed steps retain their own diagnostics.

The language-neutral cancellation and completion cases use independent SQLite and Ajv checks. The foreign terminal job regression failed against the old verdict. The updated client journey suite passed eight tests and 98 assertions. This is client qualification only. It did not launch Note or WFC or change their published identities.

## Coverage still absent from the configured journey

- Approval and elicitation are covered by separate Rust transport and consent unit laws. The configured end-to-end launcher sets `--auto-approve all`. The owned client advertises `roots` only and answers any non-roots server request with `Method not found`. A green invocation in this journey establishes mutation under auto-approval and leaves the interactive approval round trip untested.
- The typed history checks require a minted undo token, head movement and changed snapshot bytes. Those bytes carry journal entries as well as document state. Exact materialized undo to the baseline and redo to the invoked Note are established by the optional `note-mcp-replay-verify.ts` path in `mcp-native-journey.ts`. The configured end-to-end target does not invoke that path.
- The WFC solve check already requires completed success, the requested artifact and service binding, the declared output schema and the pinned expected semantic output. The subsequent `job_get` check still validates status alone. The new cancellation checks cover the separate submission, events and cancellation responses.

The real MCP SDK's direct `--deny-write` smoke ran the rebuilt native server with only `workspace.read` in an isolated ticket workspace. `artifact_create` for an `os.agent.probe/v1` artifact returned `PERMISSION_DENIED`. A subsequent snapshot for that ID returned `NOT_FOUND`, and ping succeeded. This verifies scope enforcement and preservation of an empty workspace through the real stdio boundary. Interactive approval remains outside this probe.
