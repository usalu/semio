# Architecture Course Integration Verification

## Scope

Run the existing architecture course frontend and proctor backend end to end, excluding clips. The existing ticket is open: the native repository ticket reopen command returned `ticket is already open`. Repository MCP ticket reopening was attempted first and rejected with `MCP tool call requires approval, but approval policy is never`.

## Current Evidence

- Inspected the course README, TypeScript project targets, task router, and end-to-end runner.
- The existing browser gate supports the development proxy and the release rehearsal with separate frontend and backend origins. It launches real proctor processes and Chromium learner journeys.
- A four-agent fleet covered coordination, frontend execution, backend execution, and read-only audit. No application source was changed in this verification attempt.
- Frontend agent ran `repo run @teaching/architecture-course:test`. Execution stopped before tests at native storage admission with `Exact persistent fixed backing required`.
- Backend agent ran `repo cargo --store=shared --configuration=prototype -- check -p teaching-proctor`. The pinned Rust toolchain was installed, but the Cargo operation stopped before compilation with a workspace manifest `duplicate key` error. Its source location remains unverified.
- The integration boot command `repo run @teaching/architecture-course:test-e2e -- --topology dev -- --project boot` stopped before launching the stack because the repository tool could not write its host-capacity queue.
- Normal execution consistently fails with `EROFS` for `/home/deck/.cache/semio/agent-sessions` and `/home/deck/.cache/semio/host-capacity/queue`. Earlier escalated inspections worked, but the user explicitly requested no further approval prompts. No escalation requests were made after that correction.
- A normal workspace-local XDG cache probe could not provision Bun because network sockets are restricted. This did not establish an alternate working build environment.

## Verification Status

Frontend tests, backend compilation, site build, and both end-to-end topologies remain unproven. This is an active implementation goal, not a completed delivery. Read `audit-2026-10-28.md` for the bounded curriculum audit.

## Next Actions

1. Restore normal repository tool access to its declared cache directories in the execution environment, without interactive approval requests.
2. Inspect the current Cargo workspace manifests and repair the actual duplicate TOML key, preserving concurrent work.
3. Resolve the fixed persistent backing admission error through the repository storage contract.
4. Run course tests and proctor compilation, repair observed product failures, then run browser journeys in development and release rehearsal topologies.
5. Verify that the frontend and backend remain running together before claiming completion.
