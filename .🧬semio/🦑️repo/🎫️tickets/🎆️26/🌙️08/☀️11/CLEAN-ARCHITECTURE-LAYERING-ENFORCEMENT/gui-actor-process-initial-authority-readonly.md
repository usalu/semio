# GUI Actor and Process Initial Authority Read-Only Audit

Scope: only the Product original actor integration and Process compiler capacity GUI rows, initial caller bootstrap, task budgets, and existing directly related portable laws. No frontend, native work, tests, initialization, or source edits were performed. Conclusions are source-confirmed, not runtime-confirmed.

## Conclusions

- Product command `bun repo run @semio-tech/framework-plugin-host:test-original-actor-boundary-source` has a valid existing finite parent policy of 600000 but explicitly authors `SEMIO_SCRIPT_CAPABILITIES="{}"`. `runBuildCliV1` awaits `createBuildBootstrapCapabilitiesV1` before creating the first envelope. Defined capability JSON selects `readBuildAuthoringCapabilitiesV1`, whose authoring schema requires workspaceRoot, arguments, buildMode, native, selection, and launches; `{}` fails with `Build authoring capabilities refused`. Remove this override in both authored GUI files so the configured full factory supplies the original grant. Keep the existing finite policy.
- Process command `bun repo run @semio-tech/framework-process:test-compiler-capacity` has only artifact environment and no original policy. Without an incoming envelope, `runBuildCliV1` defaults to owner repository:build, maximumElapsedMilliseconds 0. `createScriptProcessEnvelope` maps exactly zero to a null deadline, and `scriptInvocationBudget` then returns each requested child allowance without a parent ceiling. This is unbounded initial operation authority even though the child test has a finite cap.
- Process owning TestScript selects compiler-capacity with `TEST_LEVEL_BUDGET_MS.quick`, exactly 300000. It adds bounded lease tests to the same source list and invokes one owned bun-test phase. This selection is absent from the type-check predicate. Therefore 300000 is the maximum directly authored phase budget available for this initial caller; use a finite 300000 parent policy for both GUI copies rather than guessing 600000 or borrowing another task's quota. Startup/bootstrap consume this same initial deadline and cannot renew it.
- Product OriginalActorBoundarySourceScript owns two sequential phases, types and source, each requesting exactly 30000 through scriptInvocationBudget. Each is intersected with remaining original authority. These phase ceilings do not justify changing the already finite 600000 parent policy.

## Existing Meaningful Laws

Build graph tests already cover explicit editor admission before CLI envelope, portable relative initial GUI authority through task handoff, missing launches refusal, Windows/POSIX JSON path serialization, and three registered relative GUI rows checked against an Ajv-validated portable corpus. Registered JavaScript dependency callers separately assert both GUI copies use an explicit finite600000 policy and absent capabilities override, then build a full native factory grant before the first envelope. Those laws provide the existing pattern to extend narrowly to the two present commands. Generic routing laws independently validate zero-policy null deadline, finite original process handoff, and child allowance intersection. No existing law located in these reads asserts these two selected row policies directly.

## Exact SHA-256 Snapshots

Hashes are the contents read for this audit; concurrent work may change current files.

- `.vscode/launch.json`: `254aca4b5bcbbad60d3faf2384456536bbaf0c0355aa9ed4b1cfbdc74cf9ceba`
- `.vscode/🧩️launch.seed.jsonc`: `f70d8b06f7950d93d4ac0890c580fe63cb8a0375367150c1bf10c353d13fa6ec`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`: `e318ed29c795370fd2536bec8699da2975d8d5cb32dcf8522ac43c7ffe8b431b`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts`: `6059b852535e26bc0eda386c4f6af8a8be28bb92c5a4808d10c27bdbd5bc97ec`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authoring.json`: `444fc853a854ade9966609d560c0a75fac62e2bfc03e5b4c1b241d2625baa82d`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts`: `3c161d5872c8fca2390dc5723a1a437d2b7098ef0f718a4c83fc37caef78dd23`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`: `66d001007c289158aeb927fa9ded6c782e33db14f461b10301b53967684f958a`
- `🧰️framework/🔨️modules/🏃️process/📜️script.ts`: `447a89f03ce0f25a7792f6adcce3151edcad7305a0625c6b97468a838a3b7215`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts`: `5f7c306eca4755e6dfeaeab67155b3460f5414119abe16aa72151b7423669834`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts`: `a39948b21195a58975cabde4e1a3c48d43a1d1cd4eeb505333705e366e7e4675`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts`: `c7db0c4214424f79f057c22568b4af298e9ffd6e5d382b6844ced59432baa15d`
