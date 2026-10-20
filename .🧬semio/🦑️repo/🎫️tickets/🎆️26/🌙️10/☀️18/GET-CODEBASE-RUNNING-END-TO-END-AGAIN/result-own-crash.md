# Result own-crash: `verify build-owners` scan crash

`LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `T` = `C/own-crash-tools`. HARD FREEZE respected: no edit under `LIB/`, no `package.json`, no taxonomy file. The library fix is STAGED.

## Root cause
Every `verify <anything>` first runs `dispatchOwnedScriptRoute` -> `ownedScriptRoutes` -> `discoverPackages` (the repository scan). The scan crashed with `TypeError: undefined is not an object (evaluating 'admission.form')` in `packageImplementationAdmitted` (`LIB/🔍️discovery/🧱️package-implementation-admission/🟦️.ts:17`, called from `🔭️repository-scan` line 205).
- The module is new in the 01:24 auto-commit (extracted `implementationAdmission` rule). Its signature says `PackageImplementationAdmission | null` and it tests `=== null`.
- `loadTaxonomy()` hands out the unparsed `🔣️taxonomy.json` (`readTaxonomyUnchecked`), where only `🦀️rust` and `🐹️go` declare `implementationAdmission`; `🟦️typescript`, `🟨️javascript`, `🐍️python`, `🔷️dotnet` omit the key, so the value is `undefined`, not `null`. The parser (`parseImplementationAdmission`), the audit (`admission !== null && admission !== undefined`) and the existing live-schema test (`?? null`) all define "absent = no admission"; only the new function broke that contract.
- It fires as soon as any package file of a TS/JS/Python/.NET package in a nested directory is classified as implementation (a defect that should have been a `package-implementation` problem became a crash that blocks all `verify` routes).
- Not the workspaceCommand validator (fixups2 fixes still hold), not a registry row, not the owners law.

## Fix
STAGED (`T/staged/…`, anchored `T/manifest.tsv`, `T/apply.sh [--check|--dry-run]`, `T/stage.py` re-anchors):
- `LIB/🔍️discovery/🧱️package-implementation-admission/🟦️.ts`: parameter `PackageImplementationAdmission | null | undefined`, guard `admission == null`; docstring states omission admits nothing.
- `LIB/🧪️tests/🧫️fixtures/🧱️package-implementation-admission/🔣️.json`: language-agnostic case `omitted-admission-never-admits` (no `admission` key).
- `LIB/🧪️tests/🧱️package-implementation-admission/🟦️.ts`: casts accept undefined; new test reads every `packageBoundaryRules` row from `🔣️taxonomy.json` with an independent `JSON.parse` and asserts the loaded taxonomy agrees, the function never throws and refuses a nested file when no admission is declared.

APPLIED (outside the freeze): `📋️build-owners.json` (5 rows relocated: 10 lines). Once the scan no longer crashed, the owners law found 5 stale rows + 5 unregistered crates, all the same module renamed by today's splits (directory renames of mutation artifact crates: `update-graph` -> `update-graph-algorithm`, `change-scene-sun` -> `change-scene-sun-intensity`, three gltf `…tests/<name>-<hash>` -> `…/<short name>`). Fixed with the product sync (`reconcileBuildOwnerRegistry(root,{write:true})`, exactly what `verify build-owners sync` runs): removed 0, added 0, renamed 0, relocated 5, repackaged 0, findings 0.

## Verification
- Crash reproduced: `perl -e 'alarm 300; exec @ARGV' -- bun ./📜️script.ts verify build-owners` -> rc 1, stack above (3 s).
- Staged fix, overlay of the library (symlink farm, only the 3 staged files physical): `bun test ./🧪️tests/🧱️package-implementation-admission/🟦️.ts` 22 pass, 0 fail. Negative control with the live (unpatched) module: 20 pass, 2 fail (`omitted-admission-never-admits` and the taxonomy-document test, the `admission.form` TypeError).
- `T/apply.sh` in a throw-away root: apply, idempotent re-apply, `--check` applied, drift refusal (rc 2); on the real tree `--check` = safe to apply, `--dry-run` lists 3 files.
- Post-fix behaviour through the real code (`T/probe-patched.ts`, `PATCH=1` only sets the 4 missing rule fields to `null` in the loaded taxonomy, the effect of the staged fix; ~65 s): rules=6 patched=4; `ownedScriptRoutes` routes=168 (no throw); `policyBuildOwnerRegistryBreaches` breaches=0 after the sync (was 10); `policyZeroTouchClosureBreaches` breaches=0 (`verify zero-touch` logic).
- fixups2 validator `bun C/fixups2-tools/validate-manifests.ts 🌎️hub 🎓️teaching 🏢️semio-tech ♻️mit-bestand ✏️s 🧰️framework 👴️leutwiler`: manifests=1468 workspaceCommands=172 problems=0 (was 1456 / 171).
- NOT run: the real CLI `verify build-owners` / `verify zero-touch` end to end (still crashes until the staged fix is applied), `bun repo run`.

## Open items
- After the freeze lifts: `T/apply.sh --check`, `T/apply.sh`, then `bun ./📜️script.ts verify build-owners` and `verify zero-touch` (expected `passed.`), plus `bun test ./LIB/🧪️tests/🧱️package-implementation-admission/🟦️.ts`.
- Peers keep renaming mutation artifact crates (rows go stale on each rename): re-run `verify build-owners sync` after the next batch (the taxonomy apply hook only covers `clean taxonomy apply`, not plain directory renames).
- Alternatively the 4 rules could carry an explicit `"implementationAdmission": null` in `🔣️taxonomy.json` (frozen); the code fix is preferred since parser, audit and tests already define absent = none.
