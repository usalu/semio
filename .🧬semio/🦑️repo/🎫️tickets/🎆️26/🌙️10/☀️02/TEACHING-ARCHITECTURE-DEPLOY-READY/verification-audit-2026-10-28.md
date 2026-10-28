# Independent Verification Audit — 2026-10-28

## Manifest Repair

Read backend-2026-10-28.md and backend-manifests.cjs. Independently inspected read-only git diff and HEAD contents for every one of the eleven listed Cargo manifests. Exactly twelve dependency declaration lines were removed, with no added lines. Each removed line is byte-identical to a retained declaration in the same TOML section. A second independent section-aware reconstruction removed only repeated identical declaration lines from HEAD; all eleven reconstructed files exactly equalled their current working copies. This bounds the reviewed repair to duplicate removal and preserves dependency options, paths and section membership.

backend-manifests.cjs recursively scans teaching/framework Cargo manifests, excluding hidden directories, node_modules, target and dist, and parses each with both @iarna/toml and smol-toml. It fails on either parser failure. This is useful independent syntax validation; the script itself does not assert an expected count of 1,000, encode the before-repair duplicate fixture, or prove Cargo compilation. The backend report claims those parser results; this audit inspected the script and independently verified the exact repair, without rerunning dependency-backed checks while shared installation proceeds. The report correctly leaves cargo check pending.

## Exact Browser Boot Boundary

Inspected current course site-boot spec, Playwright configuration and gate orchestration. A successful development boot means the real throw-away proctor/site stack starts and Chromium reaches the introduction. The spec warms one English device with expected transient failures, waits for network idle, then creates fresh English and German devices. Each must display the logo with expected alt text and decoded naturalWidth at least the fixture minimum, translated introduction title and paragraphs, and no unresolved labels. The logo assertion checks loading/dimensions; it does not compare actual pixel colors or image contents. Accordingly the other audit phrase actual decoded logo pixels overstates the assertion.

Boot is a warm-up project, not the desktop, phone, layout, presence, shortage, away or faults journeys. It does not complete quizzes, persist completed progress, prove leaderboard convergence, exercise reconnect/fault recovery or establish production CORS. The gate uses fresh temporary stacks and tears them down; success does not leave a user-facing app running. Playwright records failure traces/screenshots, has zero retries and forbids focused tests.

## Required Remaining Acceptance

1. Obtain terminal outcomes from the fresh frontend tests/typecheck, backend Cargo check and live boot session. Dependency admission/install progress alone is not a test pass. Run the existing course check/build targets as required by their owned target dependencies.
2. Run desktop, phone, layout, presence, shortage, away and faults through development. Preserve the user exclusion of clips; do not add broad unrelated acceptance.
3. Run the same applicable browser journeys in release rehearsal, which uses immutable static frontend custody, a baked separate proctor origin and production-mode allowed-origin checks. Development proxy success cannot establish this deployment shape.
4. If the intended delivery includes a retained usable local app, start the managed development stack and verify both site and proctor readiness after browser acceptance; the E2E gate teardown cannot fulfill that delivery.
5. Claim deployment readiness only after the existing deployment check/image/stack acceptance succeeds. Public deployment, DNS/TLS and upload are separate actions and are not established by local rehearsal.

No application sources or git state were modified by this audit. The sole authored output is this retained audit. Current live owner results supersede historical October 2 and October 8 passes.

## Follow-Up Source Readiness Review

The latest retained frontend report still has queued/admitted tests and typecheck, with no terminal pass. Its 309 course and 408 shared-runtime import existence checks, managed JSON import probe and independent AJV acceptance of 11 authored documents establish useful static prerequisites, not compiler or browser success. Corrected the other audit wording from actual decoded logo pixels to loaded logo dimensions and alternative text.

Checked the apparent seven-argument runOwnedCommand mismatch and resolved it as a false positive: E2E explicitly imports runRepositoryCommand as runOwnedCommand from @semio-tech/process/execution. The repository-tooling implementation at line 87 accepts invocation, command, args, cwd, label, policy and optional options, matching ensureBrowser. The older six-argument function exported alongside it is a different symbol. No compiler/runtime defect is established by this call, and no source fix is warranted.

The native proctor readiness probe requires a successful /instance response containing the proctor package identity, with a two-second fetch timeout. Site readiness is an HTTP answer; browser boot supplies subsequent content assertions. E2E provisions private SQLite data, loopback ports for site/proctor/control, and owned child processes, so those runtime capabilities remain required after dependency installation succeeds. Chromium and chromium-headless-shell revisions must match the installed Playwright core manifest; downloading binaries is separate from operating-system browser library provisioning. The existing ci-browser-prepare invokes Playwright install-deps chromium and proves no learner journey.

Development E2E also requires retained assignment PDFs: dev-site wraps Vite in withCourseAssignmentPdfsV1, reads published Print documents through retained artifact ownership, opens an authenticated loopback PDF transport and forwards its runtime channel. The Vite plugin rejects a missing channel and invalid/oversized PDF bytes. The ordinary dev/build targets declare English and German assignment producers, while test-e2e declares assets, styling generation and proctor build only. This is a factual target asymmetry, not yet a confirmed defect: the Print publication reader may acquire or require its own producer. Inspect the eventual retained-material outcome before changing dependencies or bypassing PDFs.

No duplicate heavy build or browser install was started by this follow-up. Remaining acceptance above still applies.

## Scheduler Cold Dependency Provisioning Review

Inspected buildSchedulerV1 and the native-helpers caller without running another build. The cold receipt-miss path currently captures the Cargo compilation graph before spawning the scheduler build. Receipt hits return first. A pre-capture locked fetch can address a missing downloaded crate while retaining offline planning, but a raw owned child is not equivalent to the existing registry provisioning contract.

The repository already provides withRepositoryCargoInputsV1 in library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts. It refreshes manifest locks, validates bounded storage/environment, holds exact configuration views, fetches all platforms of the locked resolve, retains registry readiness custody through its consumer, handles stale-lock retry and honors the original offline/frozen network grant. Its additionalManifests option supports multiple exact selections. cargoDependencyFetchArgumentsV1 deliberately avoids target filtering because offline metadata resolves host and target dependencies together.

Prefer wiring scheduler helper acquisition through this supported API at the repository native-helpers boundary, or the existing compiler-input port at the neutral boundary, rather than importing repository infrastructure into the domain-neutral scheduler. Native-helpers has the original invocation, workspace, store, network authority access and controls needed for that API. Its current path enters shared compiler writer custody then exact capacity before calling buildSchedulerV1; provisioning should occur before capacity consumption so a capacity holder does not wait for a registry fetch. Preserve the receipt hit shortcut where feasible. The generic buildSchedulerV1 also has a component-pass caller, so fixing only native-helpers would not establish every cold invocation path.

If using direct fetch as the chosen implementation, require the original offline grant to remain unchanged, the same admitted Cargo home/temp/configuration paths, proper registry writer/read custody and lease order, lock refresh before locked fetch, cancellation/deadline, finite output and storage rechecks. ensureCargoLockCurrentV1 may return absent when its repository ensurer is not registered, so it is not independently a guarantee of successful lock refresh. Planning should remain offline after preparation.

The retained assignment reader was also traced further: withPublishedPrintDocumentV1 calls withRepositoryArtifactV1, which acquires a shared artifact lease and reads/inspects the existing manifest and bytes. No producer acquisition appears in that reader. This strengthens the target dependency asymmetry as a real cold-store risk, although the outer build closure may still stage producers; verify the full runner dependency expansion before declaring the E2E target defective.
