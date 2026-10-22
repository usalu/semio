# Proposed Developer Tools Live Repair

Read-only design, 2026-10-21. Lock 24430 is live; no Source, declaration, schema, fixture, product, or process-core writes and no runtime command were performed. This proposal awaits parent release. The Native plan remains the authority for required protocols; actual debugger or compiler availability was not inspected.

## Existing Obligation

`e2e-native-runtime-plan.json#developer-tools-live` requires real Node Inspector attachment/pause/cancellation, Python Pdb 42/Unicode argv/exit, LLDB native main breakpoint/argc/spaced argv/42/continue/exit, real read-only Git and real no-install LLDB Python startup 42. GDB availability/skips must be explicit. The existing plan assigns durable task receipts and PTY shell proof to installed-journeys; a new local pipe-shell observation must not replace that registered gate. The owning target remains `@semio-tech/repo-developer-tools:test`, 300000ms, same incoming Original and ticket artifacts. No cap, target, package, dependency, or core runtime change is proposed.

## Smallest Truthful Production Seam

The actual `DeveloperProcess` implementation currently exists only as the private closure inside developerCommand at script.ts:20–21. Merely replacing spawnSync with a test-owned runOwnedCommand callback would still bypass production Git environment filtering, LLDB selection/probe and actual adapter behavior. `runDeveloperTools` is public but exposes no per-operation stdin/output observation; mutating global stdin/stdout to test it would violate concurrent multi-user isolation.

Recommended minimal additional production permission: extract that ONE existing adapter into an exported first-party `createDeveloperProcessV1` function in the dedicated developer-tools/🏃️process/🟦️.ts semantic owner, and have developerCommand and tests both call it. Do not add a second implementation, alias, compatibility route, app/core helper, or generic arbitrary-command test facility. Keep request parsing/error localization in the actual command path, and retain actual Git environment sanitization and LLDB no-install probe in the shared adapter construction/execution. The existing ScriptInvocation and DeveloperProcess are repository-owned types.

Constructor inputs: explicit root, actual issued invocation, immutable validated DeveloperRequest, and an optional first-party I/O observation object (input bytes/text and `(bytes, stdout|stderr)` receiver). The I/O object is a genuine caller port, not a mock spawn/command provider. Default behavior preserves ordinary inherited stdin and streamed output. Only the input/output observation is configurable; executable selection, cwd, environment sanitization, finite output maxima, budgets, LLDB host/probe and child ownership remain production-controlled. Use the existing owned process implementation; no core changes. Require actual invocation and exact signal, check before/after the child, and await its terminal outcome.

Alternative only if no production extraction is approved: an already authenticated admitted-child fixture can invoke runDeveloperTools with a real owned stdio channel. It must consume real captured source/Original; raw Bun fixture paths or a test-manufactured source map are not acceptable. This alternative needs exact existing fixture-admission proof before choosing it, and is not assumed ready. The shared-adapter extraction avoids that additional Source handoff and is the recommended bounded implementation.

## Schema and Language-Neutral Test First

Extend the existing developer-tools schema and fixture before implementation. Add a closed live-protocol policy naming only the four existing debugger profiles and three existing host platforms. Rows express mandatory Node/Pdb/LLDB, qualified optional GDB, exact expected evidence tags, and the distinction among executed, unmet prerequisite and explicitly qualified GDB. Missing mandatory executable/compiler/no-install startup or refused launch is an unmet runtime failure, never a skipped success. Do not infer readiness from an availability lookup.

Use the existing Ajv/SQLite law to validate these closed rows and independently derive required/qualified outcome accounting with SQL CASE and JSON tables. Include counterexamples for a missing mandatory profile, skipped required body, PATH-only LLDB despite an absolute selected candidate, and Windows frontend-only GDB being mistaken for a native breakpoint. Portable DATA is policy/oracle proof only; actual tool output remains the live proof. Preserve all existing test names, fixtures, output/argv assertions and refusal cases. Add no mock tool output as runtime acceptance.

## Exact Test Changes

1. Native Git: retain the three existing fixture cases and output substring assertions. Run each real request through executeDeveloperRequest and the shared production factory under withBuildTestInvocationV1. Remove spawnSync from this leaf. The real port retains core.hooksPath/diff.external/no-optional-locks argv from the planner and the actual production GIT_* sanitization. Capture its actual bytes through the owned observation port and await terminal status. No test creates or mutates repository commits/index/hooks.

2. Existing terminal plan rows remain pure cross-platform model checks. Add a host-only, bounded non-PTY execution through the same production factory and request {command:terminal,shell:system}: feed an authored literal command that prints an ASCII token and exits zero (printf on POSIX shells, Write-Output/exit on PowerShell). Verify actual stdout token, terminal completion and no task signal replacement. Keep stdout/stderr/total output finite. Its closed fixture rows and SQLite expected token are independent of the real shell. Qualify this as local owned stdin/output/exit only; installed-journeys still proves interactive PTY and durable task receipt.

3. Node/Pdb: remove missing-program skip behavior for the declared supported host. Use the shared production factory and same immutable request/argv. Preserve Node attach+Break on start and actual cancellation; derive a scoped cancellation invocation using the existing genuine signal port and acknowledge it after child settlement, without fabricating a control. Preserve Pdb evaluation42, exact Unicode/literal argument equality and normal exit. Missing executable fails with a specific unmet prerequisite; no setup/install occurs.

4. LLDB: first call actual probeNativeDebuggerV1(root, original, createNativeDebuggerSetupHostV1(root,original)). Require a returned executable and actual Python-startup42. This already implements schema-owned absolute candidates, PATH fallback and Homebrew-prefix discovery; do not duplicate it with Bun.which(lldb). Use the production adapter for the subsequent debug request, so its own selection agrees. If an absolute candidate is selected, do not skip because PATH lacks lldb.

5. LLDB native fixture: retain the existing main/argc2/spaced argv/42/continue/marker/exit assertions. No blanket Windows skip is allowed for a platform that the schema declares supported. On Windows select the real sibling clang.exe from the already selected managed LLVM directory (or the existing legitimate compiler discovery), use an .exe artifact and actual portable argv; on macOS/Linux use a real available clang. All physical fixture paths are checked <=256 before creation. If a compiler or launch facility is absent/refused, fail explicitly as unmet prerequisite and claim no Windows native success. Platform-specific output spellings belong to closed fixture patterns with the same semantic evidence; do not relax to mere frontend startup. No new compiler installation or package manager call.

6. GDB: preserve its explicit qualified optional behavior. Non-Windows requires gdb+gcc for the native breakpoint body. Windows existing frontend42 remains frontend-only and the ASCII-path plan/refusal law remains unchanged. Record exact unavailable prerequisite/qualification in bounded ticket evidence; never count a skip as a required debugger pass.

7. Real setup verification: call only the real no-install probe on the configured host and require42. Existing injected-host setup/cancellation/provisioning model tests remain and cannot substitute for this observation. Do not call setupNativeDebugger or package managers from the live acceptance path merely to turn an unavailable host green.

## Bounded File Delta for Later Release

| Relative Path | Proposed Role |
|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📜️script.ts` | Actual dispatcher uses the one production process factory |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧪️tests/🟦️.ts` | Existing live laws use actual factory, preserving PTY boundary |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧫️fixtures/🔣️.json` | Closed portable protocol rows |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧬️schema/🔣️.json` | Closed schema-first protocol contract |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🏃️process/🟦️.ts` | Sole extracted production process and debugger host implementation |

No root package, Bun/Cargo locks, process runtime, app/core files, installer, receipt implementation, dashboard state, Source roster or metadata edit is currently proposed. If a focused fixture is later split dynamically from this leaf, its exact existing target freshness must be checked before release; do not assume cache or transitive closure.

## Risks and Acceptance Boundary

Source cannot prove host debugging privileges or installed LLVM/Python compatibility. Real mandatory bodies decide. Sharing a configurable I/O port must not permit command/cwd/environment/budget replacement, and stdout capture must remain per-operation rather than global. Real shell requests are interactive by product definition; the local bounded pipe test must feed exit and retain child-tree cancellation so it cannot hang indefinitely. Preserve the overall 300000ms target bound and existing per-operation bounds; no timeout increase is proposed.

After explicit source-write release: schema/fixture+independent oracle first, narrow adapter/test patch, Low review/freeze, then the already registered owning test route at its authorized Native/Source boundary. Retain named bodies, all skips/unmet prerequisites, physical output and actual whole. No current availability or completion is claimed here.

## Current Read Baseline

| Relative Path | Bytes | SHA-256 |
|---|---:|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📜️script.ts` | 17768 | `f538d4b94e67583f8869e9b357c635965e4de634f20c902172858dbfa90259a8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧪️tests/🟦️.ts` | 23738 | `4732d106d1c0d1bd501560fde073a9448072397838b3f0973c35ea7a9eca01f4` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧫️fixtures/🔣️.json` | 25487 | `ebb68f7fc2f0408760127e47af62d4a6eb0dda5d61f5ce35db7976dff72aff7e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧬️schema/🔣️.json` | 12742 | `22d6b77ca5de9795fc65ae6852de006c169f45667a6912cafb5c7bf7f87e6844` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🟦️.ts` | 11373 | `f513b03bade0fe2eb2e8d5fd0eadb49e365f6a457f3d371e61a1a43aa34b4a8c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📋️project.json` | 26016 | `b4c381f44bc41eabe062ea44ce8e5d0363011a1e823994fa2bfe55a90421308a` |
