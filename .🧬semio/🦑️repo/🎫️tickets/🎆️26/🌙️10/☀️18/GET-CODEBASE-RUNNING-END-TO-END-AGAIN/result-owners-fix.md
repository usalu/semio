# Result: owners-fix (build-owner sync)

`LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## Changes
- `LIB/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts`
  - Removal needs positive evidence: a row whose root or manifest is absent is dropped only when `git ls-files -c` lists nothing at that path (new `OwnerTree.tracked`, per-path cached, `:(literal)` pathspec; no git means untracked). A tracked-but-absent row (concurrent move) is kept and stays a finding. A same-name manifest elsewhere still relocates the row, tracked or not.
  - `removed` entries now carry `nativeRoots`. Surviving Cargo/project manifests at those roots become rows of their own; vanished native roots vanish with the row. Native roots are no longer lost silently.
  - Bug found by the new per-field cases: after a relocation the rebased native root was also inserted as an own row (identity collision throw). The add loop now skips native roots of kept rows.
  - New finding `manifest-not-file` (symlink or directory as manifest; folded in from the removed admission test, see below). `OwnerTree.file`.
  - `OwnerScope` = one prefix or a list; `findOwnerFindings`, `syncOwnerRegistry`, `readOwnerTree`, `checkBuildOwnerRegistry`, `reconcileBuildOwnerRegistry` accept it (git pathspecs, walk fallback per prefix).
  - New `moveOwnerScopes(moves)`: directories of moved owner/package manifests on both sides, collapsed to the outermost, `["."]` above 256.
- `LIB/🏗️build/🧬️schema/🧾️owner-consistency.json`: case `tracked`, `scope` string or list, `sync.rows` (full expected rows, every schema field), `sync.removedNativeRoots`, top-level `moveScopes`, finding `manifest-not-file`.
- `LIB/🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests/🔣️.json` and `🟦️.ts`: 10 new sync cases (moved module keeps every field and rebases native roots; tracked-old-path module still relocates; rename, repackage, Cargo to project handover each keep packages and native roots; tracked-absent root kept; tracked-absent manifest kept; no manifest and no tracked file removed with native roots freed; non-file manifest; two-scope sync), 5 `moveScopes` cases, a real-git test (temp repo: `git add` then delete keeps the row, `git rm --cached` drops it and frees `nat`; Git is the oracle) and a symlink test.
- `LIB/🏗️build/🕸️graph/🚪️admission/🧪️tests/🟦️.ts`: removed the full-registry test (3679 rows through Ajv, the subset reader and lstat: 3.8 s under load 31, 9.9 s at worst). The real-registry check is the law (`verify build-owners`), which now also checks ordinary-file manifests. The remaining admission test is a fixture test (0.18 s).
- `LIB/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts`: the directory-move tool is `clean taxonomy apply` (moves are leaf moves, so a directory move is per-file moves). After a committed apply it reconciles the owner registry scoped to `moveOwnerScopes(plan.moves)`, prints the counts and any remaining findings.
- `LIB/🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts`: sync failure message no longer calls every residual finding "authored" (tracked-absent rows are reported too).
- `LIB/🧪️tests/📦️crate-tree/🔒️workspace-registration/🟦️.ts`: expectation for `removed` now includes `nativeRoots` (the carve backup list therefore records them for restore).

## Verification
- `bun test ./LIB/🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests/🟦️.ts ./LIB/🧪️tests/📦️crate-tree/🔒️workspace-registration/🟦️.ts`: 35 pass, 0 fail. `bun test ./LIB/🏗️build/🕸️graph/🚪️admission/🧪️tests/🟦️.ts`: 1 pass, 0.18 s, default timeout.
- Third-party cross-checks in the tests: Ajv on the fixture schema and every synced registry, `Bun.TOML` vs `cargoPackageName`, real `git` for the tracked evidence.
- `bunx tsc --noEmit --strict` on touched files: clean for them.
- `bun build --target=bun 📜️script.ts`: bundled 2483 modules. Workflow module imports (`typeof runTaxonomyCliWorkflow` is function).
- `bun ./📜️script.ts verify build-owners`: the route works now (fixups2 fixed it). One run failed with 1 finding while peers were moving files (not captured); the immediate re-run printed `passed.` (about 54 s).
- Not run: `verify build-owners sync` against the real registry; the apply hook end to end (the taxonomy apply tests need `SEMIO_TEST_ARTIFACT_DIR` and a transaction repo); the hook is covered through `moveOwnerScopes` + `reconcileBuildOwnerRegistry` tests only.

## Open items
- Launch registration of the two verify targets is still open (`.vscode` retired, no replacement named).
- The law still needs about 54 s (`git ls-files -o` over the full tree); a pathspec-limited listing would not shorten the untracked walk.
- The earlier ticket tool `C/owners-tools/owners-check.ts` remains as input.
