# Registry Invocation Rust Owning Execution

Read-only current Source/config review. No Cargo, Go or Windows process was launched. Existing full Go9723 remains retained and must not be duplicated.

Shortest complete ordinary Rust owner command, from repository root:

```sh
SEMIO_LANGUAGE=en NX_DAEMON=false repo run @semio-tech/repo-workspace-rs:test --configuration=release --parallel=1
```

The declared target uses the ordinary package script test without filters. TestScript selects crate semio-framework-repo-workspace and calls runRepositoryCargoTests with untouched remaining arguments; no --lib or name filter is introduced. The manifest at repo/workspace/packages/Rust/Cargo.toml declares lib path ../../Rust.rs, rust-version1.88 and the explicit framework workspace. Its lib mounts workspace/tests/unit/Rust.rs under cfg(test). Canonical package selection/admission retains Original/control and selects the owning manifest/package.

Direct complete canonical Cargo owner alternative (not an additional job), from repository root:

```sh
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=release --parallelism=1 -- test --manifest-path 🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🦀️rust/Cargo.toml --no-fail-fast -- --test-threads=1
```

Choose one supported owner, not both. Require actual named tests and full owner success; Source admission/compilation alone is not execution. The ordinary target also proves its package router/discovery, whereas direct Cargo isolates package behavior.

Current mounted unit inventory contains seven earlier laws plus these new unconditional laws:

- declared_invocation_projection_matches_shared_neutral_corpus
- mapped_registered_invocation_keeps_prefix_before_caller_arguments

The shared projection law reads all eight current neutral rows, including mapped uppercase CMD/BAT, primary EXE and genuine rustc sibling, and malformed row/list refusals. The actual command law checks prefix before caller arguments and sanitized Bun run marker. On Linux this gives nine declared unit laws before any additional harness output; count actual names rather than inferring from compile output.

Two further laws are cfg(windows) only:

- windows_invocation_recorder (child recorder support, ordinary test invocation is a no-op without its recording flag)
- windows_batch_invocation_preserves_literal_arguments (actual temporary cmd invoking current Rust test binary and comparing exact arguments; special inputs must roundtrip exactly or return supported InvalidInput, with newline refusal required)

Thus a Linux successful owner does not execute these Windows branches or prove Windows argv safety. On Windows run the SAME complete ordinary owner under the supported declared Rust toolchain; preserve actual native standard-library batch handling, Original environment and all current arguments. Require physical recorder output and literal argument/refusal assertions. No cross-compile-only result counts as this. Go native encoder/physical Windows laws have their separate full owner9723; do not substitute one language implementation for another.

One test-only residual to consider: mapped_registered_invocation_keeps_prefix_before_caller_arguments still accepts an old manual /c argument shape even though production spawn_command now delegates batch handling to Rust std. Current std command get_args should expose only prefix/caller; this branch broadens the assertion unnecessarily and should not be described as proof of the old manual wrapper. This audit changes no Source.

## Exact Prefix Assertion Follow-Up

Current mapped_registered_invocation_keeps_prefix_before_caller_arguments now requires get_args exactly [x,--version] on every host; the obsolete manual /c alternative is removed. Windows recorder and physical batch laws remain cfg(windows). This resolves the prior test-only broadened assertion without claiming execution. Parent reports refreshed unit Source5d481d49; no test was launched by this review.

## Actual Level and Filter Trace

Read-only follow-up traces package TestScript→resolveTestLevel (default fundamental)→runRepositoryCargoTests→repositoryCargoTestPolicyV1→runCargoTestsV1/cargoTestPlanV1. Unlike the earlier simplified handoff, the generic Cargo plan DOES add libtest --skip quick::, --skip long:: and --skip exhaustive:: at fundamental; quick still skips long::/exhaustive::. Nextest additionally selects the same named profile from the selected owner config, falling back to repository .config/nextest.toml. Current root profiles define timeouts and no default name filter. Thus the default is not generically unfiltered.

The currently mounted workspace unit file has nine unconditional unit laws with no quick/long/exhaustive namespace or ignore attribute, plus two cfgWindows laws. Consequently these present unit laws survive the default level, but the complete no-level-exclusion ordinary command should be explicit: `SEMIO_LANGUAGE=en NX_DAEMON=false repo run @semio-tech/repo-workspace-rs:test --configuration=release --parallel=1 -- exhaustive`. This uses supported level semantics, not timeout override. ResolveTestLevel enables coverage for exhaustive only when SEMIO_COVERAGE is absent; that is a real owning-route side effect and must be retained/reported or explicitly selected by the execution owner. No command was executed.

Normal ignored-test behavior is unchanged: no --ignored/--include-ignored or Nextest --run-ignored is introduced automatically; the current mounted unit has no ignored law. Physical Windows cfg branches still cannot execute on Linux. Separate feature-gated fixture/oracle files in neighboring test directories are not silently mounted by this unit owner. Nine names alone were not used to infer the generic execution-plan contract.

## Supported Complete Selection Without Coverage

Actual coverage owner explicitly documents SEMIO_COVERAGE=0 as opt-out. ResolveTestLevel changes exhaustive to coverage1 only when the variable is undefined; an explicit0 is preserved. Go coverageEnabled and repositoryCargoTestPolicyV1 both enable instrumentation only for exact1. The Cargo policy schema admits coverageEnabled as a boolean; no level change is coupled to false. cargoTestPlanV1 computes higher-level skips solely from policy.level, so exhaustive still produces zero level skips with coverage false. Go goLevelTestArgs likewise depends solely on level and exhaustive produces neither -short nor -skip; goCoverageArgs independently returns[] for0.

Exact normal commands from repo root, when authorized and coordinated:

- `SEMIO_LANGUAGE=en NX_DAEMON=false SEMIO_COVERAGE=0 repo run @semio-tech/repo-workspace-go:test --parallel=1 -- exhaustive`
- `SEMIO_LANGUAGE=en NX_DAEMON=false SEMIO_COVERAGE=0 repo run @semio-tech/repo-workspace-rs:test --configuration=release --parallel=1 -- exhaustive`

These use the ordinary authority/admission/toolchain/storage/cancellation routes and declared exhaustive budgets. They disable only optional coverage instrumentation, not tests, Source checks, cfg rules or normal ignored-test behavior. They do not request a deadline/claim override. No execution was performed.
