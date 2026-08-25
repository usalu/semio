# P9-A6 OS-Host Codec ABI Fourth Independent Audit

## Verdict

**RED — do not accept P9-A6.** The narrow removal of the browser ABI declarations and exports is real, and the A1-based service has good retained-transfer mechanics. It nevertheless violates the explicit A6 prohibition on a renamed whole-buffer compatibility path: the public `OsHostCodecService` constructs `UiForbiddenOsHostWorkflowBatchBackend`, accumulates every admitted input byte, and supplies that complete `Vec<u8>` to `ArtifactPack::decode_pack` or `ArtifactDsl::parse_dsl` in one `step`. The private nominal type is therefore reachable from the public interactive service and is a loophole, not a boundary.

This is a read-only audit. It changed no production source, manifest, lockfile, Cargo workspace, Nx configuration, Wasm output, or browser state. The only audit-local additions are this report and the retained wrapper-link probe under this ticket.

## Evidence

| Gate | Result | Evidence |
| --- | --- | --- |
| Direct OS-host dependencies removed | GREEN | The baseline host manifest had two target dependency rows; current manifest has zero `wasm-bindgen` or `serde-wasm-bindgen` rows. |
| Four direct exported bridge functions removed | GREEN | Baseline source had four `#[wasm_bindgen]` attributes and eleven `JsValue` token occurrences in the removed module; current host tree has zero `wasm_bindgen`, `serde_wasm_bindgen`, `JsValue`, `web_sys`, or `js_sys` matches. |
| Public ABI type quarantine | GREEN | The public service only accepts/returns owned A1 and OS-host records. `OsHostCodecBackend`, the retained generic service, and the batch backend are non-public; no external browser/serde type appears in a public codec signature. |
| Wire schema and ledger | GREEN | Bun standard-library parser accepted version 1, 4 operation codes (1537–1540), 9 owned errors, 7 finite limits, and all 8 TSV rows. |
| Bounds and retained transfer mechanics | GREEN | `offer` performs handle/state/page/aggregate pre-admission before retaining a page; reader output is byte-credit/ACK gated. The 23 debug and optimized laws both passed. |
| A1 lifecycle delegation | GREEN | Request ledger is direct modulo-256; A1 supplies indexed handles, generation/ABA distinctions, cancel, loss, duplicate ACK, deadline/interruption and terminal close behavior. The focused laws cover the named cases. |
| Canonical workflow bytes, static route | GREEN, static only | The production backend decodes either real `WorkflowFixture` representation and emits `ArtifactDsl::print_dsl(...).into_bytes()` for both routes. Existing real-crate pair law also compares parse/decode equivalence, print fixpoint, and fresh pack bytes, but it was not rerun because this packet disallows Cargo. |
| Filter/normalization semantics, static route | GREEN | Filter resolves every supplied descriptor and appends its extensions in input/declaration order, matching `format_accept_filter`; normalizer returns descriptor `short_id`, matching the removed bridge. |
| `UiForbidden...` inaccessible from interactive public path | **RED** | `OsHostCodecService` is public, `new` is public, and its private field is instantiated with `UiForbiddenOsHostWorkflowBatchBackend`. Its public `step` reaches `OsHostCodecSession::execute`, which passes all retained input to the backend. |
| Micro-granted pack/DSL execution | **RED** | One nonzero budget admission executes the entire pack/DSL parser. There is no decode cursor, byte/item work decrement, interruption checkpoint, or deadline check inside the real decoder call. |
| Generated worker stale calls | DEFERRED, production-safe | Checked-in `🟨️frame-worker.js` still has two old calls, but they occur inside an `if (import.meta.vitest)` block. Production boot loads this worker, where the guard is false; no old export is resolved. The file predates the source update and must be regenerated in an authorized derived-artifact packet. |

## Decisive Trace

1. `OsHostCodecService::new()` constructs `RetainedOsHostCodecService<UiForbiddenOsHostWorkflowBatchBackend>` (`🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:5171-5185`).
2. Public callers submit A1 requests/pages and call public `step` (`:5188-5213`).
3. Each input page is retained/copy-progressed one byte at a time into `session.input` (`:4773-4799`, `:4927-4947`), then `seal` permits dispatch (`:4802-4813`).
4. The first following `step` calls `execute` after only `validate_budget` (`:4927-4969`).
5. `execute` passes `&self.input`—the complete reassembled request—to `decode_workflow_fixture_pack` or `parse_workflow_fixture_dsl` (`:4859-4869`).
6. The backend calls `ArtifactPack::decode_pack(bytes)` or `ArtifactDsl::parse_dsl(text)` over that whole buffer (`:4621-4632`).

Thus, renaming the compatibility implementation `UiForbidden...` does not make it unreachable. A caller of the public service can cause it to run and can make one `step` consume up to the 1,048,576-byte input bound. This fails the requested retained byte/item/page micro-grant and interaction/cancellation contract even though its boundary is byte-only.

## Executed Checks

- Ticket-local dependency-free debug law binary: **23 passed, 0 failed**.
- Ticket-local dependency-free optimized law binary: **23 passed, 0 failed**.
- Feature-wrapper artifact link and public `OsHostCodecService::new` reference: **compiled and invoked successfully**. This also confirms the actual public reachability used in the RED trace; it is not a substitute for a Cargo integration build.
- `rustfmt --edition 2021 --check` on the live host component: **GREEN**.
- Bun schema/ledger parser: **GREEN** — `{operations: 4, errors: 9, fixtureRows: 8, limits: 7}`.
- Focused source/manifest external ABI census: **GREEN** — zero current matches/rows.
- Focused `git diff --check`: **GREEN**.

No Cargo workspace/package, Nx, Wasm, or browser command was run.

## Scope And Derived Artifact

The A6 production diff is the host component, host manifest, and the OS TypeScript test-only fixture check; schema and fixture are new owned files. Root `Cargo.toml` has no diff. Root `Cargo.lock` is modified in the concurrent shared worktree but is outside this packet diff, so a repository-wide “lock untouched” claim is not supportable from the worktree; the bounded A6 patch itself does not edit it.

The generated frame worker is a real production boot dependency (`🟦️boot.ts` uses `new URL("./🟨️frame-worker.js", import.meta.url)`), but its two stale removed-export references are nested beneath `if (import.meta.vitest)` (`🟨️frame-worker.js:11660-11796`). They are dead in production rather than a broken production path. Rebuilding it is deferred, not a reason to claim source/artifact parity.

## Required Remediation

Remove the public service path to `UiForbiddenOsHostWorkflowBatchBackend`; it cannot remain as the implementation behind the advertised interactive ABI. Replace pack/DSL dispatch with a resumable decoder/parser whose state is owned by the session and whose every byte/item unit consumes an admitted grant and observes cancel, deadline, and interruption before advancing. Preserve the existing canonical `ArtifactDsl::print_dsl` result and descriptor ordering/short-id rules, then add real-backend—not `FixtureBackend`—laws covering interruption/deadline/cancel during decode. Only after that should a fresh independent audit consider the feature wrapper and a derived worker rebuild.
