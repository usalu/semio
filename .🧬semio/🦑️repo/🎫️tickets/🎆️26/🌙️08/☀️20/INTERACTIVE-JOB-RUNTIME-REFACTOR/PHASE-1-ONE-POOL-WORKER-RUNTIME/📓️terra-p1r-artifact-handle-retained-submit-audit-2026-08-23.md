# Terra Audit — P1r ArtifactHandle Retained Submit

Date: 2026-08-23

## Verdict

**ACCEPT — source packet only.** This accepts the narrow retained-submit source cohort, not Phase 1. No compiler, runtime-timing, backend-I/O, Cargo, Nx, Wasm, browser, network, or root-lint claim is implied.

## Evidence Reviewed

- P1r implementation report: \`📓️p1r-artifact-handle-retained-submit-2026-08-23.md\`.
- The accepted P1q implementation and all three Terra P1q audit reports.
- Current staged/live source for DB engine, DB artifact authority, DB facade/CLI, Hub, plugin host/kernel references, and \`📜️script.ts\`.

The P1r staged implementation inventory is exactly:

- \`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs\`
- \`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs\`
- \`📜️script.ts\`

Hub and DB CLI were inspected as callers, but their surrounding P1q/process-root changes are not attributed to this cohort.

## Call Graph and Blocking Boundary

The authored Hub route \`submit_commands\` awaits \`handle.submit(...)\`; its submit body contains zero \`block_on\`, \`ask_blocking\`, or \`submit_blocking\` matches. The live DB CLI profile route reaches the same handle, but its outer \`db_actor::block_on(handle.submit(...))\` remains at the single-shot CLI process boundary. It is an explicit process law, not a call inside \`ArtifactHandle → ArtifactAuthority → ArtifactRunner → ArtifactEngine\`; it must not be represented as zero global DB blocking.

From \`ArtifactHandle::submit\` downward, the inspected production regions have zero \`block_on(\`, \`ask_blocking\`, \`submit_blocking\`, \`thread::spawn\`, and \`WorkerPool::new(\` matches. No authored plugin reference resolves to the DB \`ArtifactHandle\`; the plugin matches name the distinct kernel \`ArtifactHandle(u128)\`.

The six actor operations (\`submit\`, \`get\`, \`frontier\`, \`query\`, \`snapshot_now\`, and \`drain_outbox\`) each occur once in the retained actor turn. The runner has separate build/turn branches, each with one future poll and a return; a grant cannot run both branches. \`SubmitFuture::drive_one\` performs either Request-to-\`AskFuture\` construction or one \`AskFuture\` poll, never both.

## Ownership and Boundedness

\`SubmitFuture\` preflights before moving the request:

- 64 generation-keyed operation slots;
- 16 KiB pages, 64 pages / 1 MiB per operation, and 1024 pages / 16 MiB aggregate;
- 256 envelopes and 4096 envelope-plus-dependency items;
- every Vec backing allocation plus identifiers, schemas, forward/inverse payloads, and dependency strings.

The checked slot generation prevents ABA release. Freshness checks precede mutable schedule/transport work. A weak generation-tagged one-shot waker, \`compare_exchange\` scheduling gate, and pool timer-wheel retry retain the exact rejected closure through finite contended/saturated retries. The API exposes terminal job, work, result, and actor-runner job take/resume/close paths; cancellation parks the exact pre- or post-actor owner rather than discarding it.

\`SubmitFuture\` timestamps and retry deadlines use \`WorkerPool::now_ms()\`. The retained database authority uses \`create_retained\` / \`open_retained\`; the only synchronous \`ArtifactEngine::{create,open}\` wrappers are \`pub(crate)\` process/test conveniences, and the production caller census found zero calls to them.

For the rejected actor-runner closure, both \`ArtifactAuthority::close_step\` and \`ArtifactRunnerTerminalJob::close\` upgrade/finish the weak runner cursor before \`drop(job)\`. The verifier's close-order mutation reverses that sequence and is rejected.

## Fixture and Verifier Assessment

The direct Rust fixtures include a real admission-slot ABA check and real 256-envelope / 1 MiB-plus-one rejection construction. The remaining named Rust fixtures are primarily structural assertions, so they do not establish runtime scheduling or backend behavior.

The root verifier is meaningful for this source packet: its self-test deliberately injects outer and inner blocking, missing nested-byte accounting, stale-after-mutation ordering, a poll loop, duplicate scheduling, missing timer retry, missing terminal work, a blocking mailbox, a synchronous constructor, and reversed runner-close order. All mutations were rejected by the passing self-test.

## Commands Run

| Command | Result |
| --- | --- |
| \`rustfmt --edition 2021 --check --config skip_children=true <DB engine> <DB artifact>\` | PASS |
| \`bun ./📜️script.ts verify interactivity --self-test\` | PASS — deny mode; one pre-existing allowlisted renderer process-entry finding, zero P1r finding |
| \`bun ./📜️script.ts verify interactivity\` | PASS — same baseline |
| Scoped production scans and caller census | PASS for the retained chain; one explicit CLI process-boundary \`block_on(handle.submit(...))\` remains as described above |
| \`git diff --check\`, \`git diff --cached --check\`, and \`git diff HEAD --check\` | PASS |
| The same three checks scoped to P1r engine/artifact/script paths | PASS |

## Residuals Outside This Acceptance

- A compiler may advance several immediately-ready internal states inside one future poll; compilation was not run.
- P1q's retained DB I/O still terminates in indivisible \`std::fs\`/Rusqlite syscalls, so no latency ceiling, fairness, or cancellation-under-stall claim is established.
- The CLI's outer process-boundary \`block_on\` and other non-submit DB bridges (catalog setup, VCS, compaction, and hello) remain separate work.
- DB engine create/open test/process wrappers, other DB bridges, generated Compose, runtime thread census, and real saturation/cancellation timing remain unaccepted.

Phase 1 remains **RED** pending those independent build, runtime, and whole-program gates.
