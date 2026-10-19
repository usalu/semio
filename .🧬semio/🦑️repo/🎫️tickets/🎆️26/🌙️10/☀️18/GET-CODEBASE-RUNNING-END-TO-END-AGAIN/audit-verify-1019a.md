# audit-verify-1019a: verification of ten executor reports (2026-10-19)

Auditor `aud-verify1`, read-only. No builds, no `bun repo run`, no cargo, no Go build, no git-modifying command. Only `bun test <file>` (sequential, `perl alarm 300`) and `bun build --target=bun` were run. Logs were written to `🗑️generated/` and removed afterwards.

Method per executor: (1) files and change exist, no leftover `[DEBUG]` in the changed code; (2) AGENTS compliance; (3) claimed-passing tests re-run; (4) correctness risks.

## Test re-runs (all sequential)

| Test file (abbrev.) | Claimed | Re-run |
|---|---|---|
| `B/💾️storage/🧪️tests/🟦️.ts` (bzl-cache) | 5 pass, 1 skip | 6 pass, 2 skip, 0 fail (count drift, passes) |
| `B/🧪️tests/📥️actions/🟦️.ts` (bzl-act) | 9 new pass | 9 pass, 2 skip, 0 fail |
| `BS/🩺️readiness/🧪️tests/🟦️.ts` (bzl-zt) | 8 pass | 8 pass, 0 fail |
| `BS/🧪️tests/🟦️.ts` (bzl-zt) | 1 pass | 1 pass |
| `🧩️runtime/🧪️tests/🟦️.ts` demonstrator (bzl-man) | fails at line 75 `41 !== 35` | confirmed: 0 pass, 1 fail, `41 !== 35` |
| `NX/🔌️sdk/🧪️tests` (nx-bound) | 4 pass | 4 pass |
| `NX/🔁️probe/🧪️tests` (nx-bound) | 3 pass | 3 pass |
| `NX/⚙️operations/🧪️tests` (nx-bound) | 5 pass | 5 pass |
| `B/⚙️operations/🧪️tests` (nx-bound) | 4 pass | 4 pass |
| `B/🧭️selection/🧪️tests` (nx-bound) | 6 pass | 6 pass |
| `B/🧭️invocation/🧪️tests` (nx-bound) | 2 pass | 3 pass (count drift, passes) |
| `os/.../📡️stdio/🚪️gateway/🧪️tests` (semio-mcp) | 14 pass | 14 pass |

Not re-run (no builds allowed or envelope-gated): Bazel adapter suite `B/🧪️tests/🟦️.ts` (needs `bun repo test bazel`), Go and Rust tests of semio-mcp, `verify-nx-cache.ts`, `bun repo tools setup bazel`, play/demonstrator/course e2e, rootimp touched tests (executor claimed build only), glue `build-targets.ts`.

## Per-executor verdicts

### 1. bzl-act: ISSUES

Verified: adapter `B/📜️script.ts` imports and calls `bazelCacheFlagsV1` (line 485); `--batch` only remains in the one-shot module-catalog setup (line 56); Starlark tags are `no-remote-exec`, `no-sandbox` (host_state), `no-cache` (requires `uncached_reason`); no `local` or `no-remote` tag; `--incompatible_strict_action_env` present.

Issues:
1. Cacheable-by-default is fail-open. `B/📜️script.ts` line 332 onward plus `bazelActionsV1` treat any target without `cache` as cacheable. I counted 634 non-continuous targets without a `cache` key across the tracked `project.json` files (`🗑️generated/audit-pj-real.txt`, excluding ticket folders; examples: `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/📋️project.json :: test`, `:: check`). A cache hit skips the action, so any effect outside the declared `<label>.snapshot` (product stores under `.cache/b/p/m/**`, `dist/`, shared stores) silently never happens. Fix: default to `cache:false` with `uncached_reason:"undeclared"` (fail-closed) and opt in per target after an input/output audit. The bzl-act report itself names this as open item 3.
2. The emoji-label risk (report open item 1) is still unverified. Run the 5-minute scratch Bazel check before the first real run, or make the label regex ASCII-only now.
3. no-sandbox plus cacheable: a cache hit restores only the snapshot. Gitignored in-checkout writes from a `host_state` task are lost on hit. Accepted by design in the report, but it must be covered by the fail-closed default in item 1.

### 2. bzl-cache: CONFIRMED (with a follow-up)

Verified: `bazelCacheFlagsV1`, `bazelServerBaseV1`, `bazelDiskCacheMaximumBytesV1` exported from `💾️storage/🟦️.ts` (line 42 onward) and used by the adapter. Adapter `startup` includes `--output_user_root` and `cache.startup` (`--output_base=...`). Storage tests re-run: pass.

Follow-up: reset (`💾️storage/🗑️reset/🟦️.ts`) still deletes `bazel/outputs`, which now holds live server bases. It must run `bazel shutdown` or kill servers first (report request 6, not done). Run `bun repo cache clear`-style reset only after that fix.

### 3. bzl-zt: CONFIRMED (with decisions for the user)

Verified: `.devcontainer/docker-compose.yml` line 13 `privileged: true` with an explanatory header comment. `🏗️build/📜️script.ts` `tools setup` Bazel branch calls `requireBazelPlatformV1` (line 138) and `prepareBazelModulesV1` (line 141). Readiness tests re-run: pass.

Decisions and risks:
- `privileged: true` widens the container's attack surface. Needs explicit user approval; the alternative is a documented host-side mount step.
- Windows refusal and the Linux preflight are unit-tested only, never executed on those OSes.
- `bun repo tools setup bazel` not run (correct, dependency lock was running).

### 4. bzl-man: ISSUES

Verified: all four `project.json` files and `📋️build-owners.json` parse as JSON; the owners row is `@teaching/architecture-course`. Demonstrator test failure `41 !== 35` reproduced exactly (pre-existing, unrelated to these edits per report).

Issues:
1. Proctor `build` is `cache:true, outputs:[]` (`🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json`). The artifact lives in the compiler store, not under the project. A cache hit reports success while the store entry may be missing or pruned, so nothing is built. Fix: make `build` `cache:false` until the store is ingested into a declared output, or add a store-presence check to the cache key.
2. `typecheck` is cached with a non-transitive `dependentTasksOutputFiles` input (course). Acceptable only if the adapter honours the non-transitive flag; the report says it does not expand these in Bazel (open item 1). Verify before the first run.
3. Demonstrator `build` output string `dist/{mode}/site` is wrong and test-pinned (`🧩️runtime/🧪️tests/🟦️.ts:43`). Fix both together, as the report says.

### 5. nx-bound: CONFIRMED

Verified: `nxAffectedArgumentsV1` (`⚙️operations/🟦️.ts` lines 8-20) maps default `affected` to `--base=HEAD`. I checked this is sound, not an accident: the fixture `⚙️operations/🧪️tests/🧫️fixtures/🔣️.json` pins `default` to tracked changes plus untracked files (`b.txt`, `d.txt`), and the installed-Nx `parseFiles` parity test passes. So the default base change does not drop uncommitted work. `--projects` is refused with a clear message.

The deleted `📥️scope/🔌️sdk/🟦️.ts` has no live importers (git grep finds only historical ticket documents).

Follow-ups: `verify-nx-cache.ts` not run (needs green dependency sync). Bazel side of the operation catalogue (`status`, `clear`, `--dry-run` TaskGraph, `exec` dry-run, `cache-verify`) is still `missing` per the report, which is honest.

### 6. prod-fix: ISSUES

Verified: `SEMIO_RENDERER` removed from play's `📦️site/📜️script.ts`; the course `package.json` scripts changed to `bun --cwd "../../../../.." repo run ...` (lines 9-12); the proctor-side entries listed in the report not re-checked.

Issues:
1. AGENTS.md (checked into the repo) states "`package.json` MUST call `nx` to run `📜️scripts.ts`". The executor changed course and fem scripts away from `nx` and wrote the conflict as an open decision, but applied it anyway. Fix: revert the `package.json` script edits to the `nx` form, or get the user's explicit decision to amend AGENTS.md first.
2. The untracked 41 MB `🎓️teaching/🏛️architecture/❓️quiz/` directory was deleted, including a `node_modules` symlink into `.cache`. Build output only, so low risk, but it was not listed as a user-approved deletion. Record it in the ticket summary.

### 7. rootimp: CONFIRMED (wiring unverified)

Verified: `caller-retained-media/🟦️.ts` re-pointed to `🔍️discovery/🌐️rust-module-graph/🟦️.ts`, which exports `inspectRustModuleGraph`. `tool-job-latest-wins` now has a single `childCloseExact` definition used three times (lines 220, 232, 249). Root and CLI `bun build` claimed clean; not re-run here.

Follow-up: the microsecond-budget test dropped its binding block, and the law moved to `⚖️laws/.../tool-job-microsecond-worker-binding`. Confirm `toolJobMicrosecondWorkerBindingSelfTests` is actually wired into `tool-job-coverage` (executor did not run the os suite). Coverage count dropped from 398 expectations to 398 minus 102, which must be justified.

### 8. glue: ISSUES

Verified: `🧰️framework/📦️packages/🟦️typescript/package.json` now has BOTH `"./config/mutations"` (line 200) and `"./config/schema/mutations"` (line 210), both pointing at the same target. The re-exported diff API exists in `🎚️config/🧬️schema/🧬️mutations/🟦️.ts`. The seven dead manifest entries are still present (manifest edits are pending for the deps owner).

Issues:
1. AGENTS forbids compatibility layers and aliases. Adding `./config/schema/mutations` beside the pinned `./config/mutations` is exactly an alias. Fix: rename the key to `./config/schema/mutations`, rewrite the 34 importers (`git grep -l '@semio-tech/framework/config/mutations'`) in the same change, and update the docstring that still names `config/mutations`.
2. The seven `package.json` dead entries and the `.//` generator fix remain as the report describes (not verified here beyond the report; `check-exports.ts` not re-run).

### 9. e2e-prep: CONFIRMED (with open items)

Verified: F1 fixed. None of the six listed files still contains the retired `🧰️framework/📦️packages/🟦️typescript/🛍️products` path. The proctor entry `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts` now bundles cleanly (`bun build` 334 modules), so the report's F3 blocker for proctor is resolved at least at entry level.

Open: the other ~62 stale-import files (`e2e-tools/stale-imports.ts` dry run not re-run); Playwright browsers 1248/1013 not installed; `run-e2e.sh` never exercised for real. The course `test-e2e` should depend on `workspace:deps-browsers` (report section 6).

### 10. semio-mcp: ISSUES

Verified: gateway files exist; gateway test corpus passes (14 pass); the root cause (staging build before `initialize`) is addressed by the warm gateway.

Issues:
1. Silent data loss: `structuredContent` is now omitted for any non-object result (the report's own open item). Arrays and scalars returned by repo tools as structured data are dropped. Fix: wrap non-object results in an object (for example `{"result": value}`) at `repositoryResult` and the Rust `json()` filter, and update the shared fixture `🧱️tool-result-shape.json`.
2. Rust edits (os `mcp-server/🦀️.rs`, repo-mcp Rust twin) and the Go change were not compiled or tested (no cargo, by rule). Run `gooverlay.ts test` and the Rust unit test in the coordinator's serialized window.
3. The `semio` tools stay empty until the cargo build of `@semio-tech/framework-os-mcp-rs:build` succeeds (report open item). Expected, not a regression.

## Follow-up tasks

1. Bazel cacheability: switch default to fail-closed `cache:false`; opt in targets after an audit of the 634 targets listed in `🗑️generated/audit-pj-real.txt` (regenerate from `git ls-files -- '*project.json'`).
2. Run the 5-minute scratch Bazel check for emoji labels (bzl-act open item 1), or switch the label regex to ASCII now.
3. Proctor `build`: make it `cache:false` or add a compiler-store presence check to the cache key.
4. Glue: rename `./config/mutations` to `./config/schema/mutations` and rewrite 34 importers; delete the alias key; update the docstring.
5. prod-fix: revert course and fem `package.json` scripts to `nx` form, or get the user's decision to amend AGENTS.md first.
6. semio-mcp: wrap non-object structured results in an object; update the shared fixture; run Go and Rust tests in the serialized cargo window.
7. Reset: stop Bazel servers (`bazel shutdown`) before deleting `bazel/outputs`.
8. Demonstrator manifest test: fix the `41 !== 35` fixture count (unrelated to these edits, but blocks `bun repo test`).
9. Devcontainer `privileged: true`: user decision.
10. Confirm `toolJobMicrosecondWorkerBindingSelfTests` is wired into the os tool-job-coverage suite.
11. Run `stale-imports.ts` dry run and decide on the 62 remaining stale-import files.
12. Run `verify-nx-cache.ts` and the Bazel `cache-verify`/`affected`/`exec` dry-run once the dependency sync is green.
