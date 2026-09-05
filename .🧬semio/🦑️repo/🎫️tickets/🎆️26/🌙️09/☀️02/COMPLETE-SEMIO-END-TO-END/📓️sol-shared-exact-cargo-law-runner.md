# Shared Exact Cargo Law Runner

## Implemented Boundary

The repo library exports `runExactCargoLaws(options)` and owned group/process/result types. A group names one Cargo package, target and nonempty unique law selection. The runner performs one JSON `cargo test --no-run` build per group, requires exactly one matching executable receipt, records its SHA-256, lists that binary directly, resolves every selector exactly once, and runs each resolved name directly with `--exact --test-threads=1 --show-output`. A passing receipt requires one passed, zero failed and zero ignored assertion for every requested law. The executable identity is checked around listing and each execution. Cargo is never reinvoked per law.

Build, list and assertion failures retain their actual child exit status/signal separately from assertion parsing. Each stage has a deadline, cancellation and bounded output capture. Logs and executable/assertion receipts remain in unique directories below the explicit caller-owned `artifactDir` or `SEMIO_TEST_ARTIFACT_DIR`. Missing configuration fails closed; there is no temporary-directory fallback or embedded ticket identity.

Provider, VCS and strict plugin-admission callers now use the shared helper. The backend independently migrated GIS SQLite/WAL laws. Their Rust assertions have not been rerun by this helper yet; previous native RED evidence remains unchanged.

## Terminal Evidence

Current captured-output enhancement: schema-first TDD **85732 RED** (15 passed/9 failed) proved the missing `--show-output` argument. It also exposed a fixture-only shallow-output-directory assumption; the outside-target negative now constructs a path outside the actual `🗑️generated` boundary at any artifact depth. Registered **70971 GREEN**, exit 0: **24 tests / 416 expectations**, including 19 neutral orchestration cases, strict AJV/dual SHA-256 oracle and four actual child-process status/cancellation/output-cap laws. Captured `[DEBUG]` output is retained in the passing law stdout. A spoofed second terminal is rejected, and the schema rejects `--nocapture`. Actual-exit, exact-one, ignored-law and fingerprint checks are unchanged. Existing in-flight runs keep the arguments captured before this code was loaded.

The previously warmed target and raw receipts were deleted by a separate cleanup task; no deletion was performed by this lane. Historical terminal results below remain reported as observed, but the old raw receipts/executables are unavailable. New evidence lives in explicitly recreated ticket-generated paths. This is an external infrastructure loss, not a passing or failing Rust assertion.

Latest target-ownership boundary: session 58061 was the intentional RED (6 passed, 16 failed, 113 expectations) before Cargo target admission. Session 40832 is GREEN, exit 0: 22 tests, 349 expectations, zero failures. The neutral fixture now has 17 cases. Every subprocess receives a copied caller environment; absent `CARGO_TARGET_DIR` resolves beneath the explicit generated artifact root, and explicit non-generated or relative targets are rejected before build. Target overrides through Cargo arguments are rejected. Receipts record the effective target without modifying global environment.

The task-scoped provider, VCS, strict admission, selected-provider and member-dialect launch entries now explicitly reuse the ticket's `native-openable-provider-sol-target` with one Cargo job. Owner generation 23479 and freshness 75821 are GREEN (59 plugins, 60 playgrounds, 45 framework packages), superseding the earlier taxonomy-blocked generation attempts below. Subsequent fixture-sweep launch generation 16107 and freshness 46634 are also GREEN. No heavy Cargo run was started.

`SEMIO_TEST_ARTIFACT_DIR=<absolute active-ticket>/🗑️generated bun ./📜️script.ts nx run @semio-tech/repo-lib:test-exact-cargo-laws --skip-nx-cache`

- Session 60306: RED before assertions, incorrect test import path; repaired to the actual library package owner.
- Session 4202: GREEN, 17 tests and 290 expectations (neutral contract plus deterministic process-port laws).
- Session 47607: GREEN, exit 0, 21 tests and 313 expectations in 1,037 ms. This includes real Bun child exit-7, timeout, cancellation and output-cap observations, with retained stdout/stderr equality.

The language-neutral fixture has 16 cases. Strict AJV validates its schema independently; Node crypto and WebCrypto independently reproduce the pinned executable digest. Hostile cases cover missing/duplicate/wrong-target Cargo artifacts, relative executable paths, missing/duplicate law discovery, ignored/zero/multiple assertion terminals, failed Cargo/native status, executable replacement and cancellation. The process-port fixture proves orchestration without claiming any production Cargo compilation or provider runtime acceptance.

Permanent command ownership is the repo-library `📜️script.ts` with Nx target `@semio-tech/repo-lib:test-exact-cargo-laws`. Launch seed entry is `⚖️gate🦀️exact-native-law-runner`. Owner generation 93013 was GREEN; freshness 96361 correctly detected the concurrently updated member-dialect launch seed. After the coordinator's explicit task-scoped launch configuration, provider/VCS/strict/helper entries now supply this ticket's generated output roots. Native entries cap Cargo jobs at one. Owner generation 54962 and freshness 44053 are GREEN. Generic runner code still contains no task identity or inferred selection.

Post-migration registered oracle-only checks are GREEN: provider 47879 (8 claim vectors, 26 receipts, 26 hashes, 13 hostile/no-partial outcomes), VCS 83937 (1 positive, 11 hostile, 1 protocol hash), and strict admission 67836 (15 vectors, 39 first-party roots). They deliberately do not start Cargo.

Final launch reread found the helper's environment edit had matched an earlier identical line in the unrelated job-microseconds entry. The source correction restores that unrelated entry and anchors the exact-runner ticket environment to its own command. Provider/VCS/strict environment edits were correctly scoped. Selection-entry generation 89894 passed before this last correction; final freshness 9d2d68 and corrected generation 713430 both stop before dispatch on concurrent reserved README/LICENSE fixed-filename taxonomy patterns. Thus current seed intent is correct, but these last two generated environment lines are explicitly pending owner regeneration; historical 54962/44053 cannot certify the final source. No generated launch file was hand-edited.

## Active Ticket Resolution Audit

The current repo infrastructure does not expose an authoritative active task-to-ticket binding usable by a generic Bun/Nx launch:

- Repo CLI `🐹️component.go:43362` implements `currentTicketSessionID()` using a test override or `generateHookSessionID()`. The latter, at line 44466, generates a fresh ID; it does not resolve the calling agent task.
- `HookContext` at line 37672 carries raw event input and repository metadata, not a selected ticket identity. `extractSessionIDFromInput()` at line 44550 extracts IDs from hook input, but generic Nx launches receive no such input.
- `trackHookInOpenTicket()` at line 38714 selects `latestOpenTicket()` before attaching a session. `latestOpenTicket()` at line 44526 selects by date and slug among open tickets. That is not authority for the current task in a concurrent workspace.
- Ticket close/reopen helpers at lines 35071 and 35086 similarly use explicit paths or latest-ticket selection. They are not safe output-directory resolvers.
- Repo configuration `.🧬semio/🦑️repo/📋️config.toml` contains logging policy only; session logging is disabled. No configured task/ticket path was found there.
- Existing repo-library `nextestArtifactLocation()` at line 1621 accepts explicit environment configuration or falls back to OS temporary storage. The fallback is deliberately not reused for exact-law evidence.
- Current MCP resource discovery returned no `repo://` resources, and enabled-tool discovery exposes no repo ticket/context tools in this agent session.

Therefore the exact runner retains required explicit `SEMIO_TEST_ARTIFACT_DIR` or `artifactDir`. A development launch must supply the active ticket directory explicitly until a separately specified authoritative context mechanism exists. This packet does not infer the newest/open ticket, inspect unrelated agent history, or weaken ticket-scoped evidence requirements.

## Nonclaims

No stdio/VCS provider, hub readiness, browser/native/MCP transport, all-plugin activation or historical all-features D0 acceptance follows from the runner tests. This is shared test-infrastructure evidence only. The warm native target remains preserved and heavy native execution remains coordinated with the parent lane.
