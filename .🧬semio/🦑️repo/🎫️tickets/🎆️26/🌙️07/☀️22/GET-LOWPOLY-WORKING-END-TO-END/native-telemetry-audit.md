# Native Telemetry Audit

Read-only source/own-process audit, 2026-11-05. No retry, kill, Native/server command, foreign mutation or broad filesystem walk.

## Current Evidence

repo leases status --json at1793842777838 identifies worker6260, parent42100, exact command cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path Cargo.toml -p semio-framework-plugin-command --lib. Standing phase cargo:workspace-selection, implementation age1411s, silent34s, progressing. No waiters. Besides old images/rustup/configuration-view16, a new shared repo:compiler-writer lease was acquired at1793842725226 (52seconds old at snapshot). This is concrete progress beyond initial resource holdings while phase label remains.

Own-PID psutil sample:6260 creation1793841366.1418006 matches requested identity; running, CPU user372.546875/system1864.5seconds, RSS435380224, no immediate children. Parent42100 creation1793841362.49111, running, CPU user6.28125/system3.890625, RSS146350080, child6260 bun.exe. Cumulative CPU is not precise current stack evidence.

## Scope And Label Semantics

Workspace owner /🗂️workspaces/🦀️cargo/🟦️.ts:472 withCargoWorkspaceSelectionV1 wraps entire withCargoWorkspaceSelectionOnceV1 in concurrent-edit snapshot retry.

Once function:
- captures physical topology and manifest text via selectPlan;
- advance yields and publishes resource cargo:workspace-selection every≥250ms;
- retains maps and a live issued phase through await body(view);
- exposes view.recheck that replays every observed state/presence and every captured manifest text, using same advance/publish label;
- performs final recheck after body completes.
Thus label is not a unique initial-selection state and may overwrite more substantive outer phase repeatedly during held-body rechecks.

Sole production caller found is compiler ingress /⚙️configuration/🟦️.ts:43. Its callback reads selected workspace policy, invokes actual compiler version/config probes, captures profile sources and opens the configuration body under selected.control. Profile recheck explicitly awaits selected.recheck. Configuration view recheck calls storage.recheck, profileCapture.recheck, nativeCompiler.recheck and captured-view digest read. Body view stays live across nested owned operations and final recheck. Repeated selection label can therefore represent genuine source freshness work around compiler configuration/consumer execution.

## Conclusion And Limits

Current phase alone does NOT prove true selection loop or deadlock. New writer lease plus actual process lifetime/CPU supports ongoing work, while zero child sample is consistent with in-process source recheck at that instant. It does not prove Rust compilation or native test execution has begun.

No exact active stack/recheck invocation was captured. Determining whether repeated full rechecks dominate elapsed time needs bounded phase/count instrumentation owned by repo infrastructure; this audit does not add it or infer an algorithm defect solely from elapsed time. Root retains running producer; no second execution started.

## Safe Diagnostics Used

repo leases status --json (parsed/filter own6260/42100 rows in orchestration, no logs persisted).
repo tools exec --light python -- -c <psutil ownPID query>
repo tools exec --light git/node for exact source files.
