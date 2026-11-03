# Ingress blocker: `Original compiler ingress requires the repository launcher and authentic enforcing store`

## Verdict
Framework defect, not ours. Our new crates, path dependencies, Cargo.lock edits and the `member-manifests` edit are NOT the cause. Every `:check` target of an artifact Rust package (and of `@semio-s/plugin-norm-mechanics`) runs an **unissued, raw `cargo check`**, which the repo-wide compiler ingress refuses by design. These checks could never have reached rustc since the ingress was introduced (commit 8b89a887830, 2026-10-23); the earlier blockers only hid it.

## Evidence
1. `/home/deck/semio/.cargo/config.toml` sets `build.rustc` to `🚪️ingress/📜️script.ts` and `rustc-wrapper = "repo"`. Cargo therefore runs `repo <ingress>/📜️script.ts -vV`, and that script is only `throw new Error(policy.refusal)`.
2. The ingress is bypassed only when the caller exports `RUSTC` (and an empty `RUSTC_WRAPPER`). That is done solely by `repositoryCompilerLaunchEnvironmentV1` (`⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/🟦️.ts`), reached through the issued launcher `launchRepositoryCargoV1` (`🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`) with an enforcing storage reservation.
3. The check route: `📋️project.json` target `check` -> `repo ./📜️script.ts check` -> `runArtifactRustPackageMain` -> `CheckScript` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts` line ~50:
   `runRepositoryCommand(this.invocation,"cargo",["check","--locked","--manifest-path",...])`.
   `runRepositoryCommand` (`🏃️process/🎛️owned-execution/🟦️.ts`) only runs the `synchronize` preparation and then spawns the plain rustup `cargo` (`/home/deck/.local/bin/cargo`) with the inherited env: no `RUSTC`, no launcher, no store. Cargo reads `.cargo/config.toml` and calls the ingress.
4. Log order matches: `synchronize` stage `complete` (it printed the `Locking 34 packages` lines, i.e. it legitimately refreshed `Cargo.lock`), then immediately `cargo ... -vV` fails. Same signature in en1990 (run 2), en1994, en1997, en1998, mechanics.
5. The sibling routes that work use the issued launcher: `test` -> `runRepositoryCargoTests` -> `runRepositoryCargoTestsV1`; `build` -> `buildRepositoryCargoArtifacts` (`scriptCompilerAdmissionControlV1`). Only `CheckScript` (and the identical raw pattern in `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📦️packages/🦀️rust/📜️script.ts`, `✏️s/🧑‍💻dev/**/📜️script.ts`) calls `runRepositoryCommand(..., "cargo", ...)`.
6. Control run (unaffected project, same pattern, no norm code): see section "Control run".

## Invariant check of our changes (all clean)
- Path dependencies across workspace roots: the failure occurs before cargo reads any manifest; `synchronize` completed and wrote the lock, so workspace authority and member-manifests resolved.
- Missing `📋️project.json` for mechanics: it exists with the same check/build/test targets; the target started and failed at the same ingress.
- Cargo.lock: refreshed by `synchronize` itself (the proper command); nothing to hand-edit.

## Fix
Proposed (framework code owned by others; not applied because it needs a compile-validated change and touches every artifact), in `📦️artifacts/🦀️rust/🟦️.ts`, `CheckScript.run`: replace the raw `runRepositoryCommand("cargo", ["check", ...])` by the issued launcher, preserving the preparation, exactly like `runRepositoryCargoTests` does:

```ts
const control = scriptCompilerAdmissionControlV1(this.invocation, "cargo:check", 0);
const args = ["check", "--locked", "--manifest-path", resolve(this.root, "Cargo.toml"), ...(options.packages ?? []).flatMap(n => ["-p", n]), ...cargoArgs];
await prepareCargoWorkspaceInvocation(repositoryCargoPreparationStorageV1(this.repoRoot), this.repoRoot, args, this.root, scriptInvocationEnvironment(this.invocation, process.env), createRenewableIssuedResourceControlV1(control, progressDeadlinePolicyV1("native-child", 0, process.env, {maximumMilliseconds: buildPreparationBudgetMs(process.env)}), 0));
await runRepositoryCargoCommandV1(this.invocation, control, this.repoRoot, args, this.root, process.env, 0);
```
(`runRepositoryCargoCommandV1` from `🏃️process/🦀️cargo/🎟️admission/🟦️.ts`; it adds `--store=shared --configuration=<mode> --`, sets `RUSTC`, `RUSTC_WRAPPER`, lane and capacity.) The same change applies to the mesh and dev-services scripts. Owner of the framework (integrator or framework agent) must apply and validate with one `check` run.

## Instructions for builders (until the patch lands)
- Do NOT use `:check` for any artifact Rust project or `@semio-s/plugin-norm-mechanics`. It cannot succeed.
- Use `:build` (admitted: `buildRepositoryCargoArtifacts`, compiles the whole workspace and prints rustc diagnostics) as the compile gate: `SEMIO_LANGUAGE=en repo run '@semio-tech/norm-enXXXX-rs:build'`, same background pattern and log files.
- Once `build` is green, `:test` (admitted, `cargo nextest`) is the test gate.
- Keep the one-build-per-artifact rule; a failed `check` costs a full queue slot.

## Control run
One run of `@semio-tech/s-fem-mesh-rs:check` (its `CheckScript` uses the identical raw `runRepositoryCommand("cargo", ["check", ...])`, no norm code, no new crates) was queued by this audit; log `🗑️generated/ingress/mesh-check.txt` (ends with `exit N`). At the time of writing it was still waiting in the capacity queue (7 ahead, memory pressure), so the conclusion rests on the code path above. Expected outcome: the same ingress refusal. If it instead compiles, re-open this audit. Delete the log when done; this run was not cancelled (a kill was denied by policy), it will simply finish.

## Patch applied (2026-11-02, coordinator-approved)
- `⚡️caching/📦️artifacts/🦀️rust/🟦️.ts`: `CheckScript` now builds `args`, calls `prepareCargoWorkspaceInvocation` (renewable control, `buildPreparationBudgetMs`) and then `runRepositoryCargoCommandV1` with `createIssuedResourceControlV1(invocation, "artifact-rust:<name>:check", 0)`, mirroring `runRepositoryCargoTests`. The now-unused `runRepositoryCommand` import was replaced by the new imports.
- Regression test: `⚡️caching/🧪️tests/🚪️artifact-rust-check/🟦️.ts` (bun:test, passes with the repo bun; asserts launcher + preparation present, raw `runRepositoryCommand` absent). The module also bundles cleanly (`bun build`, 783 modules, all imports resolve).
- NOT yet validated by a real run: the queued fem-mesh `check` uses its own script (`✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📦️packages/🦀️rust/📜️script.ts`), not this file, so it does not exercise the patch (it will most likely still show the ingress refusal; that is the control). The first artifact `:check` after this patch (e.g. `@semio-tech/norm-en1997-rs:check`) is the validation: success or genuine rustc diagnostics = fixed; the same ingress error = reopen. Mesh and `✏️s/🧑‍💻dev/**` scripts still carry the raw pattern and are outside this patch.
- Risk: `maximumMilliseconds` 0 is passed to the launcher (unbounded, as before); if the launcher rejects 0, use `buildBudgetMs` from `repositoryCargoTestPolicyV1`.

- Control run result: the fem-mesh check died with SIGKILL (out of memory) before cargo started, so it proved nothing either way. Log `🗑️generated/ingress/mesh-check.txt` can be deleted.
