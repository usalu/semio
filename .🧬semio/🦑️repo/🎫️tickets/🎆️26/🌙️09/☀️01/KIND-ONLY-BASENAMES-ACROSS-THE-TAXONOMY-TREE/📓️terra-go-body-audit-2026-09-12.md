# Go Route And Package-Body Independent Audit

## Scope and evidence

This is a read-only audit of the live shared worktree. No source, schema, configuration, Git state, ticket state, or goal state was changed. Probe-only output is retained in `🗑️generated/terra-go-body`.

The focused registered targets both passed with `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, and an isolated `NX_WORKSPACE_DATA_DIRECTORY` below that output directory:

- `nx run @semio-tech/repo-lib:test-go-dispatch`: one Bun assertion passed. Its actual Go invocation passed `TestCanonicalGoTestDispatcher` and the canonical selected package set.
- `nx run @semio-tech/repo-lib:test-package-body-policy`: 45 tests passed, zero failed, with 138 expectations. This ran the portable fixture and the independent TypeScript, Rust, Go, Python, and C native-oracle checks.

The retained logs are `🗑️generated/terra-go-body/nx-go-dispatch.log` and `🗑️generated/terra-go-body/nx-package-body-policy.log`.

## Go scoped route

The reachable native Go route is correctly centralized for normal execution. `runTestScope` selects bundle, file, section, and definition scopes; every Go branch in `runBundleTests`, `runFileTests`, `runSectionTests`, and `runDefinitionTest` calls `runCanonicalGoTestDispatch`. That routine executes the registered repo-library `📜️script.ts go-test <module-root> <input> …` endpoint. The endpoint computes `canonicalGoPlan`, limits a file input to its selected planned package, includes transitive local `go.mod` replacements, writes an overlay, and invokes standard `go test` only for planned packages.

The focused native fixture proves the bundle and definition cases against semantic-only Go leaves; it deliberately confirms that bare `go test ./...` rejects the fixture's semantic paths. Static inspection confirms the file and section branches use the same dispatcher and argument construction. This audit did not add a separate runtime file/section fixture.

## Finding: detached nested Go process survives budget cancellation

**Priority: P1.** The native route leaks a detached nested Go process when an ancestor budget cancels it. The definitive probe invoked the registered outer route:

```text
bun <repo-library>/📜️script.ts go-test <repo-client-cli> - -count=1 -run ^TestCanonicalGoTestDispatcher$ -v
```

It waited for the fixture's native Go scope to start its inner Bun dispatcher and its second `go test -overlay`, then stopped only that second, probe-owned Go process. The recorded tree was:

```text
outer Bun:       81286
inner Bun:       81631
nested Go:       81669 (PGID 81669)
```

At the outer fundamental-level 15-second budget, the outer route exited with status 1. PID 81669 remained with `PPID 1`, `PGID 81669`, and state `TNs`; it survived the parent cancellation because its `runTestBudgeted` spawn was detached. The probe resumed and terminated only that recorded process group. Exact evidence is `🗑️generated/terra-go-body/cancellation-overlay/result.txt`.

`runTestBudgeted` currently sends a group kill only to its immediate detached child. That kills the outer Go test group, its test executable, and the inner Bun before the inner Bun can clean up its own detached Go child. Adding signal handlers only to the inner Bun cannot solve an upstream `SIGKILL`.

Required repair and acceptance:

1. Before terminating an immediate process group, resolve the complete probe-owned descendant tree and terminate every detached descendant group as well, then the immediate group. Keep Windows tree termination equivalent and do not rely on a POSIX-only orphan cleanup.
2. Add a native scoped-route cancellation test that cancels the parent `command.Command` context after the inner Go child has started. It must return promptly, observe no marker writes after cancellation, and prove that every recorded descendant PID/process group is gone.
3. Add a nested-budget cancellation test that triggers the outer budget after that same child starts and proves the same return, marker-stability, and no-survivor properties.
4. Use only PIDs/process groups registered by that fixture; do not use global process-name matching or terminate unrelated work.

I also attempted the direct parent-context variant under `SEMIO_TEST_LEVEL=long`, preventing the outer 15-second budget from being the first cancellation. The inner Bun was no longer present after the native 30-second context deadline, but after 37.5 seconds the outer Bun, outer Go test group, and client test process still had not returned. I cleaned only their recorded probe PIDs/groups (`82700`, `82740`, and `83066`). Since cleanup happened before a natural final state could be recorded, this is evidence of a prompt-return failure, not a second conclusive no-survivor result. Its acceptance case remains required. Evidence is `🗑️generated/terra-go-body/cancellation-context/result.txt`.

## Package-body corrections verified

The seven previously reproduced admissions now reject through the live public API:

| Counterexample | Live decision |
| --- | --- |
| Python import plus function | `implementation` |
| Vitest configuration plus hidden class | `unresolved` |
| Root script plus hidden class | `unresolved` |
| Unknown Rust `semio::register!` computation | `unresolved` |
| C function-like arithmetic macro | `implementation` |
| Known Rust registration macro with `execute()` body | `implementation` |
| C function-like domain-call macro | `implementation` |

The result includes exact role/evidence metadata in `🗑️generated/terra-go-body/classifier-counterexamples.json`. This confirms the Python whole-source fix, whole-file tool-disposition rejection, closed Rust macro grammar, and C directive visibility. The Rust oracle metadata correction is also present: `Cargo.toml` and `Cargo.lock` reside in `🔮️oracles/📦️packages/🦀️rust`, while the anonymous Rust source remains at `🔮️oracles/🦀️.rs`; the manifest explicitly points its binary path to `../../🦀️.rs`.

## Finding: valid renderer Vitest configuration is a policy false positive

**Priority: P2.** The live, fixed Vitest configuration at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts` is referenced by its package `📜️script.ts` through `runVitest(..., "vitest.config.ts")` and has the fixed `vitest-config` contract. Yet `classifyPackageSourceDisposition` with the correct `vitest-config` / `tool-config-vitest` disposition returns `unresolved`.

This is a validator-precision issue, not an extraction order. The smallest independently reproduced rejected configuration is a reachable multiline arrow helper:

```ts
import { defineConfig } from "vitest/config";
const suites = (name: string, extension = "ts") =>
  `./${name}/x.${extension}`;
export default defineConfig({ test: { include: [suites("x")] } });
```

The same shape occurs in the live renderer `elementSuite` helper. A one-line arrow helper, conditional selection, arrays, spreads, and a regex `testNamePattern` are accepted. The current statement splitter treats the newline after `=>` as a statement boundary. Results are in `🗑️generated/terra-go-body/vitest-shape-probe-expanded.json` and `renderer-vitest-dispositions.json`.

Repair the narrow config grammar/statement splitting so a reachable multiline helper in a `defineConfig` module is admitted, then add a positive renderer-shaped vector and negative variants containing an unrelated private class, domain function, top-level side effect, or unreferenced helper. Preserve the existing whole-file reachability rule; accepting all `vitest.config.ts` bodies would re-open the repaired hidden-domain bypass.

## Generic-stem fixture closure

The seven active fixture failures are stale coordinates, not evidence that the runtime leaves remain unrelocated. Update the fixture to the current sibling directories and test locations:

| Fixture coordinate | Current owner |
| --- | --- |
| `🔨️modules/⌨️control-keybinding-context` | `🔨️modules/🕹️control-keybinding-context` |
| both `☑️Select` rows | `🔽️Select` |
| `🎚️Toggle` | `🔀️Toggle` |
| `📊️Diagram` | `🕸️Diagram` |
| `🪵️Tree` | `🌳️Tree` |
| `🎨️styling/🧪️test/🟦️s.ts` | an active test under `🎨️styling/🧪️tests`, such as `🧩️suite/🟦️.ts` |

Each current UI element has `🟦️.tsx` plus `🧪️tests/🧩️component/🟦️.tsx`; `Select` also has its two current target leaves. The fixture should update these exact coordinates and retain the direct `node:fs` plus independent glob assertions. I did not rerun its known failing unregistered route or modify the active styling correction.

## Limits

I did not run broad Nx graphs, global inventory/planner enforcement, or full CLI suites. I did not infer a baseline from unrelated TypeScript diagnostics. The normal dispatcher test proves bundle and definition execution; file and section were route-audited statically. The direct-context cancellation probe required manual cleanup of only its recorded descendants, so its final no-survivor state is deliberately not claimed.
