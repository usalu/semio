# Cache Command Source Extraction — Independent Acceptance Audit

> **Current status — accepted with a Windows-runtime limit.** The earlier process-tree finding is repaired; current direct and isolated registered evidence is 11/169, with a POSIX descendant-PID observation.

## Scope

This audit covers the extraction from `📚️library/⚡️caching/📜️script.ts` into thirteen anonymous semantic owners, the language-neutral `command-source` schema/fixture/test, and the `repo:test-cache-command-source` route.

## Current static evidence

The fixture declares exactly thirteen existing `🟦️.ts` owners and 22 taxonomy contexts:

- ticket output;
- source discovery;
- inventory composition and inventory orchestration;
- graph verification;
- storage report;
- environment inspection;
- cache verification orchestration;
- workspace pruning and pruning orchestration;
- artifact package inventory, contract capture, and package orchestration.

The router is a thin command composition surface. Its explicit consumer rows bind its seven command-class owners, while the existing cache-contract test directly imports ticket output, source discovery, and inventory composition. The direct owner graph rejects a return import into `⚡️caching/📜️script.ts` and checks acyclicity.

The registered target is `repo:test-cache-command-source`, with command `bun ./📜️script.ts test cache-command-source`. Its `cacheCommandSources` input currently contains all thirteen owner leaves, router/project/policy/bootstrap/test/schema/fixture inputs, the external workspace-cleanup command consumer, shared package/discovery/taxonomy/Nx-plugin inputs, and both launch files. This closes the source files the command-source test actually reads.

The source walker ignores generated directories, `.🧬semio`, `compose`, `temp/compose`, and symlinks before descent. Artifact discovery also skips symlink directory entries and package contract checks assert each Rust package compiles the taxonomy source directly.

## Historical finding: Windows process-tree termination was incomplete

`📦️artifacts/🏃️contract-capture/🟦️.ts:11` handles Windows cancellation with `child.kill("SIGTERM")`. The same cache domain's established owned-command boundary, `🦀️cargo/📜️script.ts:28`, invokes `taskkill /pid <pid> /t /f` on Windows so descendants are terminated too.

Artifact contract capture can run a wrapper such as Cargo or Bun which can create child processes. The current timeout control at `🧪️tests/🧱️command-source/🟦️.ts:178-180` only exercises a direct Node child, so it cannot establish descendant cleanup on Windows. This is a test-budget and correctness gap for the stated cancellation responsibility.

**Disposition:** repaired as documented in [Repair verification](#repair-verification) and accepted by the current registered route below.

## Limits

- No direct package or registered Nx execution was run by this audit; the executor owns those current checks.
- No Windows runtime was available here. The finding follows the current source's explicit platform branch and contrast with the repository's existing cross-platform command runner.
- Artifact inventory uses the real repository topology; its direct static no-follow behavior is established for encountered directory entries, while adversarial artifact-root injection is not independently executed by this audit.


## Repair verification

The current captureArtifactContract now uses taskkill /pid <pid> /t /f on Windows, matching the repository's owned-command process-tree behavior. Its portable control starts a parent that starts a Node descendant, causes timeout after 100 ms, extracts the descendant PID from bounded output, and proves that PID is gone before cleanup. The same control statically requires the Windows tree-kill command and retains the direct-child timeout plus output bound checks.

This establishes actual descendant cleanup on the current POSIX host and source-level Windows branch presence. A Windows runtime result remains outside this audit. The earlier 11/168 results are historical; current post-consumer evidence is recorded below.

## Final source-data consumer delta

After the first isolated 11/168 Nx checkpoint, the fixture gained one additional current source-data consumer: `📚️library/🟨️.mjs` names the cache-verification owner in its generator-output cache-coupling rationale. This is a source-document reference, not an executable cache command. The fixture now has three consumer rows, and `cacheCommandSources` hashes that library source. The source test resolves its exact relative owner specifier like every other consumer row.

The earlier isolated Nx result remains historical because it predates this row.

## Final acceptance

The current direct command-source route passed 11/169 in 20.77 seconds. The current isolated registered `repo:test-cache-command-source` invocation passed 11/169 in 20.90 seconds of test time and 21.9 seconds through Nx, cache skipped. These results postdate the `📚️library/🟨️.mjs` source-data consumer row.

I independently inspected the repaired capture boundary and its control. On POSIX, the implementation starts the subprocess detached, signals its process group, and schedules a bounded `SIGKILL` fallback. The current control launches a parent and descendant, extracts the observed descendant PID, and proves the PID is gone after its two-second timeout; that case completed in 2.72 seconds. The Windows branch now invokes `taskkill /pid <pid> /t /f`, and the control requires that exact tree-kill argument sequence.

The final owner graph remains thirteen anonymous leaves in 22 declared contexts, with the router free of a return import and three resolved source-data consumers. This accepts the extraction and current POSIX cancellation behavior.

## Remaining limits

- No Windows runtime ran here; the `taskkill /t /f` branch has source-level and portable-contract coverage only.
- The descendant observation is a focused Node process-tree control. It does not establish every external tool's cancellation semantics.
