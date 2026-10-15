# Current Command Audit Invocation Contract

The accepted ALL16 immutable snapshot was audited successfully: terminal exit 0, owned session 5122. This audit reads the supplied immutable graph and current literal command declarations; it does not execute targets, infer a graph, or publish a snapshot.

The previous audit refused before results because the current `resolveCommandConfigurationV1` requires a genuine invocation as its third argument. The ticket-only `command-audit/📜️script.ts` now creates one existing `repository:build` envelope through `createBuildBootstrapCapabilitiesV1` for `show targets --json`, with the original 120,000 ms policy. Every resolver receives that original invocation. Per-recipe checks and awaited progress every 64 targets retain cancellation, deadline and continuation checks. No production implementation, policy budget or compatibility export changed.

All original absolute-path, environment handoff, snapshot SHA-256, ALL admission, independent `string-argv`, literal script existence, execution cwd and nested owner cwd checks remain. The nested command resolver remains synchronous. The audit does not claim execution coverage for these commands.

Current audit script SHA-256: `3d441bb44130027eddb0b779df95ebe45f692b426fe647da632052761a0632d1`. Low independently reviewed this exact adapter as clean.

Accepted pin: `8af9074d9e322568cf841e7fd05110c3a940d87a522bac60a65670c8b875d0fc`.

Result: 2,205 projects; 12,988 base targets; 25,946 configurations; 58,657 dependencies; 1,394 continuous targets; 11,737 unique command recipes; 19,037 literal script references; 1 nested command boundary; 0 dynamic references; 0 failures. Target and configuration aggregates agree with the accepted ALL summary.

Retained generated evidence: `🗑️generated/graph-command-audit-current.log`, `🗑️generated/graph-command-audit.json`, and the accepted `🗑️generated/qualified-graph-current.json`. Root's earlier resolver-arity refusal remains a separate failed attempt; this successful run does not relabel it.

## Current ALL17 Audit

Root accepted fresh current v3 ALL17 and released the unchanged adapted audit. The actual audit exited **0**, session **44245**, under its original 120,000-ms envelope, with the exact required absolute workspace/artifact/pin/digest/reuse handoff. No Source or test change preceded this run.

Current accepted digest: `fa53c2c277685b941612a86ba130c62f75b7f3a6ccc5cc87b33d0bd51c0d252f`. Snapshot bytes: 224,637,980. Exact summary agreement: 2,205 projects / 58,657 dependencies / 12,988 base targets / 25,946 configurations. Audit inspected 11,737 distinct command recipes and 19,037 literal script references, including 1 nested owned-command boundary; 0 dynamic references and zero failures.

Evidence: `🗑️generated/graph-command-audit-all17.log` and current `🗑️generated/graph-command-audit.json`. This records command/cwd/argv and immutable-pin checks, not execution of every audited task. Root was notified immediately for High's exhaustive plan handoff. Core has no active runtime and holds Source/tests.
