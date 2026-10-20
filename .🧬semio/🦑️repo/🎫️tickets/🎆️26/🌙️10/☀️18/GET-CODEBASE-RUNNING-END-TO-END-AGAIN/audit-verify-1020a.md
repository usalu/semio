# Audit verify-1020a: verification of seven result files

Auditor `aud-verify10`, read-only. `C` = this ticket folder. Brief: `C/brief-1019.md`. Nothing was edited; the only write is this report. Commands run by me: `bun test ./<file>` on two files (below), read-only `git diff`/`git show`/`git grep`, `shasum`, and inline `bun -e` read scripts. Not run: any cargo build or check, `bun repo run`, `verify build-owners`, dependency sync. Where a result claims a run I did not repeat, the verdict says so.

## Verdicts

| Result | Verdict | Blocks apply? |
|---|---|---|
| `C/result-own-crash.md` | VERIFIED. Fix is in the live tree, 22/22 admission tests pass (run by me), relocated owner manifests exist. One design concern (type widened to `| undefined`). | No |
| `C/result-fixups5.md` | Docstrings VERIFIED (22/22 declarations). Test-support gating consistent, not compiled. Needs `Cargo.lock` refresh; one undocumented visibility widening. | Yes, for Rust: `--locked` fails until the lock is refreshed |
| `C/result-fixups6.md` (actor part) | Paths and `include_str!` VERIFIED; not compiled. The exporter passes `--features typegen`, which the actor package does not define. Likely broken, pre-existing in HEAD. | Yes, for the typegen exporter |
| `C/result-tst7.md` | Totals consistent with the JSON. Classification NOT reliable: about 10 of 33 "real-defect" rows are not source defects, 11 envelope-gated suites contain ReferenceError or ENOENT signatures that the classifier hides, and the law-drift bucket is mixed. | No, but the census numbers should be re-labelled before use |
| `C/result-exports-stage.md` | MOSTLY VERIFIED. Targets, aliases, pins and glue checked. Naming defects (12 `/.json` keys), count mismatch, one dead devDependency. | Fix the 12 keys before apply |
| `C/result-bzl-real.md` | NOT SAFE TO APPLY AS STAGED. Staged `store-receipt.mjs` hard-codes `/Users/ueli/...`; staged adapter docstring is stale; two evidence rows do not add up; patch target is renamed by tax-stage. | Yes |
| `C/result-tax-stage.md` | Anchors VERIFIED (121/121 match the live tree). Manifest has 167 ops, not 168, and no `package.json` op. Conflicts with bzl-real and exports-stage. Rule conflict with AGENTS on renamed scripts. | Yes |

## 1. result-own-crash (C/result-own-crash.md)

Verified:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧱️package-implementation-admission/🟦️.ts` has `if (admission == null || ...) return false;` and the parameter `PackageImplementationAdmission | null | undefined`. The docstring starts with an emoji.
- Fixture case `omitted-admission-never-admits` is present in `…/📚️library/🧪️tests/🧫️fixtures/🧱️package-implementation-admission/🔣️.json`.
- I ran `bun test ./🧪️tests/🧱️package-implementation-admission/🟦️.ts` (library dir): 22 pass, 0 fail.
- Taxonomy: `🔣️taxonomy.json` `packageBoundaryRules` has 6 rules; `🟦️typescript`, `🟨️javascript`, `🐍️python`, `🔷️dotnet` omit `implementationAdmission`. This matches the root-cause text.
- `📋️build-owners.json` (repo root): git diff shows 5 rows changed (10 lines). Each new `manifest` path exists on disk.

Not verified: `verify build-owners` and `verify zero-touch` were not re-run. The result's own open item says they must run after the freeze lifts.

Concrete fix (after freeze):
1. Normalise at the boundary, not in the consumer. In the loader that produces `packageBoundaryRules` (the path behind `loadCatalogTaxonomy()`), map an absent `implementationAdmission` to `null` through the schema. Then drop `| undefined` from `packageImplementationAdmitted` and from the test casts. AGENTS says schema-first, and widening the type to hide an unparsed document goes against that.
2. The docstring "(the taxonomy document is read unparsed)" describes the workaround. Rewrite it as the contract once step 1 is done.
3. In the test "the live schema admits Go sub-packages...", replace `expect(rule.implementationAdmission ?? null)` with a strict `toBeNull()` on the loaded TypeScript and other rules.

## 2. result-fixups5 (C/result-fixups5.md)

Verified:
- Item 1 (docstrings). Of the 35 body-level blocks, 30 are restored and 5 dropped (30 + 5 = 35, matches). The 30 restored blocks land on 22 named declarations. I checked each declaration programmatically: the `/** */` block is directly above the declaration and its first content character is an emoji. 22/22. Examples checked by reading: `WorldCanvas` (`…/🎬️canvas/🟦️.tsx:550`), `pollHeartbeatSab` (`…/⏱️watchdog/🟦️.ts:58`), `usePanelPathsAndOverrideReveals` (the `applyAgentShellCommandRef` it documents is a parameter of that hook), `useToolTabSelectionEffects` (`agentArtifactRouteRef` is a parameter).
- Item 4 (test-support). `📬️paged-one-item/🦀️.rs` lines 107 to 111 hold `#[cfg(feature = "test-support")] #[doc(hidden)] pub mod test_support { … }`. The crate manifest `…/🏪️store/📬️one-item-owners/📦️packages/🦀️rust/Cargo.toml` has `[features] test-support = []` and a self dev-dependency with `features = ["test-support"]`. Consumers use `paged::test_support::Stage` (`🧬️mutation-apply/🧪️tests/🦀️.rs`) and `paged_one_item::test_support::{…}` (`🧪️tests/🔗️source-custody/🦀️.rs`). Consistent with the claim. Not compiled.

Defects and gaps:
1. The self dev-dependency changes the package's dependency list, so `Cargo.lock` must be refreshed. Until then every `cargo … --locked` in the repo fails for this package. The result states this; keep it as an apply prerequisite.
2. Undocumented widening. `git diff` on `📬️one-item-owners/🦀️.rs` shows `from_returned` and `begin_returned_close` changed from `pub(crate)` to `pub` (both under `#[cfg(feature = "durable-group-testing")]`). Item 4 of the result does not mention it. Either revert to `pub(crate)` if the cross-crate test does not need it, or record the public-API change.
3. Restored docstrings keep history. Examples: `useOperationCompletionSubscriptions` ("used to leave the outliner..."), `resolveDispatchTarget` ("measured on the live `s` host"), `useHubPortInferenceAndDocumentBackbone` (ticket and session numbers). AGENTS asks for docstrings that describe the contract and link to resources. Rewrite these in present tense and move ticket references to the ticket notes.

## 3. result-fixups6, actor part (C/result-fixups6.md, section 1)

Verified:
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml` declares `[[test]] kernel-unit` → `🧪️tests/🔬️unit/🦀️.rs` and `[[test]] typegen` → `🧪️tests/🧬️typegen/🦀️.rs`. Both files exist.
- `🧪️tests/🧬️typegen/🦀️.rs` `include_str!("../../🤖️generated/🎭️actor/🟦️.ts")` resolves to an existing file.
- `🧪️tests/🔬️unit/🦀️.rs` line 2 is `use semio_framework_actor::*;`.

Defect (likely broken, pre-existing in HEAD):
- `🧬️typegen/🏃️execution/🟦️.ts:51` runs `cargo test --locked --features typegen --test typegen …` with `cwd` = the package root. The actor `Cargo.toml` has no `[features]` section at all, and the root `Cargo.toml` has no typegen feature. HEAD has the same `--features typegen`. Cargo rejects unknown features, so the exporter most likely fails before it runs. The `[[test]] typegen` target needs no feature.
- Fix: remove `"--features", "typegen"` from the spawn argv, then run `cargo test --locked --test typegen exports_typescript_bindings` once after the freeze. Not run by me.

## 4. result-tst7 (C/result-tst7.md, data in C/🗑️generated/tst7/)

Verified: the census JSON agrees with the report (2,145 suites; 1,764 green; 381 red; `final.py` output). I did not re-run any suite.

Problems with the classification:

a. "Real defect" rows that are not source defects (about 10 of 33):
- `🤝️interoperability` (fail=3): `posix_spawn` of `cargo/target/debug/semio-io-sqlite-snapshot-oracle`, a cargo-built binary. Belongs in needs-cargo.
- `🏛️graph`: missing `.nx/workspace-data/project-graph.json`, an Nx prerequisite. Envelope or prerequisite, not a defect.
- `🗄️catalog` and `📏️windows`: "JSON Parse error: Unexpected EOF". The result itself attributes this to concurrent writers truncating a cache file. Harness.
- `🔗️markdown-inline-references`: missing directory under the `.bun` cache layout (`markdown-it@15.0.3/node_modules/markdown-it/lib`). Install layout.
- `💾️cache` (`…/🗑️generated/tst7/ta/…/pa-…/📜️script.ts`) and `↪️rust-divergence-callback` (a temp dir under `/private/var/folders`): fixture copy list and vanished temp dirs. Harness.
- `🍎️image` twins under `cargo/🎯️exact/🧪️tests/🍎️image` (two suites): the first error is an envelope refusal ("Original process admission closed", "Original native caller capabilities required"), not the lost-import ReferenceError the path override claims.
- `🏗️build/🏃️execution/🧪️tests/🟦️.ts` (fail=0): first error is "Original prepared execution inventory required" (envelope), but the path override says "runTestScript not defined".
- The path overrides in `final.py` (`REAL` list) assign root causes by path, not by log content. Fix: assign by log, or label the override "by path, unverified".

b. Hidden real defects. The envelope regex comes first, so any log that also contains a refusal string is classed envelope-gated, even when it has a ReferenceError. 11 envelope-gated suites contain ReferenceError or ENOENT signatures, among them:
- `withRequestedIssuedResourceControlV1 is not defined` (`📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts`)
- `captureFilePublication is not defined` (`🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts`)
- `bazelTaskInputsV1 is not defined` (a second copy of the real-defect row)
- Ajv `strictRequired` in `🏗️build/🧪️tests/🕸️graph/🟦️.ts` (the baseline failure fixups6 documents)
Fix: test real-defect signatures (`ReferenceError`, `SyntaxError`, `TypeError: undefined is not an object`) before the envelope list, or emit both tags.

c. Matching on whole logs. `classify.py` matches the whole log, so source-line echoes match (for example "exceeds 256 characters" and "expect(" lines printed by bun). Fix: match only failure lines (`error:` and the `(fail)` lines).

d. Residual "law-drift" (117). I sampled 20 (every sixth row). About 14 are assertion mismatches on source or law needles, consistent with drift. Six are not drift: a tsc subprocess (`📋️callers`), a missing binary (`🤝️interoperability`, counted above), a zero-fail crash (`🧭️authority`, fail=0), an envelope refusal (`📖️taxonomy-loading`), a module-not-found inside a temp artifact dir (`🚦️test-dispatch`), and a subprocess exit-code probe (`🧱️command-composition-source`). The rest are ambiguous. Fix: rename the bucket "unverified residual" until it is triaged.

## 5. result-exports-stage (C/exports-stage-tools/batch.json)

Verified (my checks, read-only):
- Targets: all 379 glue `create` targets exist (`glue.targets[].path`). Sampled value targets (25 apply entries spread across tiers) either exist or are glue creates in the batch.
- Aliases: 0 duplicate export keys inside the batch; 0 export adds whose value duplicates an existing key in the same live `package.json`, across all 393 export adds.
- Pins: 83 dependency entries, all `devDependencies`, all with a lock entry. 80 match the lock text exactly or are workspace links. 3 are caret ranges satisfied by the lock: `@types/reveal.js ^5.2.0` (lock 5.2.2, two entries) and `@vscode/vsce ^3.5.0` (lock 3.9.2). Pins are fine.
- No runtime `dependencies` added. Good against AGENTS.

Problems:
1. Naming. 12 export keys end in `/.json` with an empty stem, for example `./publication/build/schema/contribution/.json` (E0444, value `…/📣️contribution/🔣️.json`). The live convention is the directory stem plus `.json`: `./sdk/binary/schema.json` maps to `…/🧬️schema/🔣️.json` in the live tools `package.json`. Fix: `…/contribution.json`, and the same for the other 11. Ten of the twelve values also point into a directory named `🔣️.json/🟦️.ts` (glue TypeScript under a `.json` name). Re-check each: a `.json` export must export JSON, or the wrapper must follow the `🔣️json/<stem>` rule the report states.
2. Counts do not match the report. The file has 637 entries, 509 apply, 379 glue creates. The report says 636, 508 and 376. Reconcile.
3. `D1058` (`@semio-tech/repo-lib`) and `D1059` (root) add `dotenv` as a devDependency, but tsc-stage patch 14 removes that import. Drop both entries.
4. Not verified: the seven runtime-looking third-party devDependencies (`cross-spawn` at `@semio-tech/process`, `lru-cache`, `semver`, `dotenv`, `safe-regex`, `@iarna/toml`, `scheduler`). If an importer is runtime code rather than a test or script, `devDependencies` is wrong. Check each importer before apply.
5. Ordering. `apply.ts` refuses removals that are still imported, which is the right guard. It does not cover the `package.json` ownership overlap with tax-stage (section 7).

## 6. result-bzl-real (C/result-bzl-real.md, staged in C/bzl-real-tools/staged/)

Verified:
- `bun test ./bzl-real-tools/staged/store-receipt.test.ts` (ticket folder): 6 pass, 0 fail (run by me).
- Live `…/🏗️build/🔧️tools/🏗️bazel/📜️script.ts:306` still runs `writeChanged(join(workspace,".bazelignore"), …)`. The defect is live. The staged `adapter.ts:306` replaces it with `rm(…,{force:true})`, as the result says.

Blocking problems:
1. The staged `store-receipt.mjs` hard-codes a user path. Line 6 reads `new URL("file:///Users/ueli/Documents/semio/…/🧬️schema/🔣️.json")`. The live file uses `new URL("./🧬️schema/🔣️.json", import.meta.url)`. `grep` finds `/Users/ueli` once in `store-receipt.mjs`, zero times in the adapter and the test. Applying it breaks zero-touch and Windows, Linux and devcontainer. Fix: restore the relative URL in the staged copy, and make `stage.ts` stop substituting absolute paths.
2. Stale docstring. `staged/adapter.ts:304` still says the link "is listed in `.bazelignore`". The result's own patch 1 asks for the docstring to be fixed; it was not.
3. Evidence rows do not add up. The logs were deleted, so this cannot be checked against the logs.
   - Q3: "2 processes: 1 action cache hit, 1 internal, 1 local" lists 3 items.
   - F1: "4 processes: 1 action cache hit (A), 1 internal, 3 local" lists 5 items.
   - Q2 (100 % hits) prints only "1 process: 1 internal", so Bazel's process count does not seem to include action-cache hits. Q3 and F1 contradict that convention.
   - Q4's note "T2 disk cache hit (its local entry held the touched key)" is self-contradictory.
   Fix: copy the exact Bazel summary lines from a re-run, or drop the rows.
4. Patch target collision. tax-stage (`C/tax-stage-tools/staged/ops.json`) deletes `…/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` and writes `🟦️.ts` in its place. bzl-real generates `adapter.ts` from that file and its patch 1 targets it. Whichever applies first breaks the other. Fix: retarget the bzl-real patch to `🏗️bazel/🟦️.ts`, or apply bzl-real before tax-stage and regenerate the adapter afterwards.

Known limits, stated in the result and not contested: all spawns ran `host_state` / no-sandbox; the production orchestrator was blocked by the dependency sync; remote cache not run.

## 7. result-tax-stage (C/tax-stage-tools/staged/ops.json)

Verified:
- Anchors: every `patch` and `delete` op (121) has a `base` hash equal to the sha256 of the live file. Zero drift at this time. Three `write` ops overwrite existing files and are anchored by `base`, which matches the live file.
- Staged manifest and blobs exist (`staged/ops.json`, `staged/blobs/`).

Problems:
1. Manifest count and missing op. `ops.json` has 167 ops (kinds: patch 97, write 46, delete 24). The result says 168 with "1 of them package.json". No op touches any `package.json`. The missing op is the one for `⚡️caching/🚀️bootstrap/🛠️tools/package.json`. In the live file, `./bazel/script` points at `./🏗️bazel/📜️script.ts`, which tax-stage renames, and `./script` already points at a file that no longer exists. Fix: add the package.json op, or make exports-stage own that file and say so.
2. Overlap with exports-stage. `C/exports-stage-tools/batch.json` removes `./script` (E0857) and `./bazel/script` (R1066) from the same `package.json` and adds `.`. The two stages must be sequenced on that file. Neither result says so.
3. AGENTS conflict. AGENTS requires permanent scripts in `📜️script.ts` at their directory and forbids other script files. The plan renames two permanent module scripts to `🟦️.ts` (`⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel` and `🏗️build/🔧️tools/🏗️bazel`), adds 18 `📦️child/🟦️.ts` files, and adds 19 `🧪️tests/🔬️script-commands/🟦️.ts` modules. The report calls this a "new convention". It needs a user decision before apply. Recommended fix: keep `📜️script.ts` as the entry in every directory that has one, and move only the non-script logic into modules with non-script names.
4. The "verification" in the report depends on a mirror (`/private/tmp/tsm`, deleted). Not re-run.

## 8. Other notes

- Ticket hygiene: I found no `[DEBUG]` temporary logs in the touched Rust or TypeScript directories I checked (`🎭️actor`, `📬️paged-one-item`, `📬️one-item-owners`, the renderer elements). The only hits were in unmodified test files.
- Global docstring emoji uniqueness (AGENTS) is not checked by any result. The tax-stage report notes a law backlog of about 37k findings on that rule.

## Suggested order after the freeze

1. own-crash: normalise `implementationAdmission` at the loader (section 1), then `verify build-owners` and `verify zero-touch`.
2. exports-stage: rename the 12 `/.json` keys, drop `dotenv`, reconcile counts, check runtime versus test importers (section 5).
3. tax-stage: add the missing package.json op and decide the AGENTS question on renamed scripts (section 7).
4. bzl-real: restore the relative schema URL, fix the adapter docstring, re-target the patch to `🟦️.ts`, fix the evidence rows (section 6).
5. fixups5 and fixups6: refresh `Cargo.lock`, decide the `pub(crate)` widening, drop `--features typegen`, then one `cargo test --locked --test typegen` and one `cargo check -p semio-framework-os-store-one-item-batch --tests` (coordinator run).
6. tst7: fix the classifier precedence and re-label the census before anyone uses the 33 real defects as a work list.
