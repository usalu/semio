# MCP Launch and Publication

## Current Implementation

The normal native BuildScript and interactive stdio launcher call one MCP-owned buildMcpArtifactV1 producer. Its schema fixes the owning crate manifest, package, executable, sources evidence and stderr diagnostic lane. The producer uses buildAdmittedCargoArtifactsV1 with the genuine Original invocation, signal-refined compiler control and exact scriptInvocationEnvironment. It avoids global graph dispatch without raw Cargo, copied binaries or forged capabilities.

Canonical startup builds before acquiring the executable reader. The existing Cargo-admission provider now exposes a neutral withAdmittedPublishedCargoExecutableV1 port backed by the repository exact publication reader. That reader validates marker ownership, inventory, file identity and digest while holding the shared artifact lease and compiler backing through consumption. Startup and the TypeScript test task use this same port; neither preopens a raw compiler store around the producer. Ordinary EOF leaves an acquired reader live until native termination and final verification. Pending build or reader admission is cancelled and drained.

The root declaration names the catalog producer through --catalog-target. Both .mcp.json and .codex/config.toml are generated from that declaration, with no client drift. Catalog staging remains an owned neutral target. The interactive compiler intent explicitly uses prototype when the shell names no mode.

The gateway supports both native protocol openers: legacy initialize waits for notifications/initialized before its backend handshake; modern server/discover starts the native connection without injecting a legacy handshake and preserves the exact native answer. Modern requests queue until staged. No backend starts after client EOF. Native EOF shutdown has an authored policy of 250 ms grace, 250 ms termination and 250 ms force-kill bounds.

Compiler diagnostic text, compiler JSON observations, publication notices and recipe/provenance progress flow through an explicit owned diagnostic callback. MCP supplies stderr; protocol stdout is never globally patched.

## Validation

The latest combined focused run passes 48 tests and 408 assertions across staging/publication, gateway and wire protocol, with Ajv, SQLite and official SDK oracles. Regressions were observed red before fixing premature legacy startup, late startup after EOF, unbounded native EOF, modern discover-first queuing and the missing owned diagnostic port. The source tests also execute the actual TypeScript task method with ports to prove its reader encloses Vitest and that its test-subject override is restored before reader release. Both neutral and repository publication-provider implementations load successfully.

An actual configured initialization/EOF smoke previously exited 0 with no signal in about 2.7 s. This proves bounded lifetime only, not native readiness. A first bounded 10 min bare native-readiness attempt reached admitted Cargo work but timed out before publication; its child exited 1 without a termination signal. A subsequent attempt was intentionally cancelled after source review confirmed potential compiler-stdout contamination. Those results are not native success evidence.

The current corrected bare readiness run is session 70070. It uses one monitored 30 min diagnostic discovery deadline, live stderr, guarded SDK parsing and 15 s outer preparation cleanup before force-kill. Native build/publication and native readiness remain pending. The root agent owns native Rust diagnostics and full catalog readiness.

## Smoke Modes

The retained semio-smoke.ts accepts --ready, --bare and --timeout-ms. Default discovery bound is 600000 ms; explicitly allowed diagnostics extend it up to 1800000 ms. Ready mode requires real server/discover with a native version different from warming and a validated nonempty tools/list containing artifact, action, capability discovery, inference, UI and history operations. Bare mode omits catalog flags to isolate native production; the full configured catalog is still a separate required gate. Protocol errors and signals reject pending requests, close stdin and drain terminal state.

## Changed Source Areas

- Root 📋️project.json, .mcp.json and .codex/config.toml.
- Repository dev MCP command composition.
- MCP TypeScript module, TypeScript task router/project declaration and Rust task router.
- MCP stdio staging producer, schema, neutral fixtures, source tests and publication tests.
- MCP warm gateway, shutdown schema, neutral journeys and source tests.
- Framework Cargo-admission provider contract and TypeScript package facade.
- Repository Cargo-admission provider and exact published executable reader environment forwarding.
- Framework native artifact diagnostic option/output sink and provenance diagnostic sink.
- Ticket semio-smoke.ts, launch-research.md and this report.

## Native Task Consumers and Strict Types

The native task router now shares one PublishedMcpScript consumer helper across dev/HTTP, capability audit and schema mirror/check. Dev uses the proper publication reader, explicit authored host-driver policy and async owned execution with the same Original environment, cancellation and control. Audit and schema emission capture complete bounded stdout/stderr and preserve actual nonzero audit status while the reader remains live. No source-tree binary default is used by those consumers. Transport and capture fixtures cover default stdio, HTTP, explicit policy, complete 70 KB output and byte-copy ownership.

The latest staging suite passes 28 tests and 280 assertions. The focused strict native-task and typed-fixture compiler run passed with exit 0 and zero diagnostics; core-interface typechecking had no owned diagnostics and three unrelated missing inline-test ambient types. Details are in launch-types.md. One existing provider fixture was updated for the new reader method and for Windows-safe generated driver execution; its five tests and 48 assertions pass.

## Source Memo and Cancellation Follow-up

The diagnostic readiness run 70070 reached its explicit 1,800,000 ms deadline without native discovery or an artifact publication. It repeatedly admitted the 451-unit Source graph before observed compiler dispatch. The SDK saw clean protocol stdout. The child drained within the 15-second outer cleanup reserve, but exited 1 with a Source-refusal wrapper around the client EOF AbortError. Retained terminal evidence: generated/smoke-70070-timeout.json. No native readiness claim follows from this run.

Artifact options now carry the existing CargoSourceAdmissionMemoV1 through both initial admission and every guarded artifact recheck. The repository wrapper selects the existing repository planning and physical Source memo factories from the actual compiler-store view, preserving caller ports, Original authority, toolchain/policy identity, no-follow present/absent physical stamps, racy-write refusal, live control checks, completion/storage rechecks, and final plan/receipt comparison. The Source read boundary preserves only the exact current signal reason; genuine failures remain contextualized, including a separate failure concurrent with cancellation.

Four focused laws pass with 37 assertions: actual artifact-plan memo forwarding across initial admission and two rechecks; neutral Ajv/SQLite warm/change/oversize/cancel vectors; physical memo changed-source/new-probe/policy/toolchain/racy refusal; exact cancellation and genuine failure distinction. Registered strict TSC for both owners and changed laws exited 0 with no diagnostics (launch-memo-tsc.txt). The short 15-second cancellation diagnostic 36759 still observed child exit 1: a separate cancellation path remains and is now owned by the root agent. No further long readiness run was started.
