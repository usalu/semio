# audit-verify-1019b (aud-verify2), 2026-10-19

Read-only audit of the ten finished executor reports. No source edited, no builds, no `bun repo run`. Paths are repo-relative to `/Users/ueli/Documents/semio`. Line numbers refer to the working tree at audit time.

## Verdicts

| Executor | Verdict | Summary |
|---|---|---|
| cache-fw | CONFIRMED (counts not re-derived) | Manifest edits are as described; the only stale-cache defect in the sample is owned by cache-rest (below). |
| cache-s | CONFIRMED | Sampled Rust `*-native` targets are covered by project `nativeSources` extras (checked `🏙️bim` `test-complete-sqlite-native`). Cross-cutting note in follow-up 5. |
| cache-rest | ISSUES | Stale-cache hole in the stdio wiring check targets (item 1). |
| xplat-boot | CONFIRMED | Pins match upstream for 5 of 5 native rows checked. PowerShell and Windows FFI are unexecuted, as the report says. |
| rules-fix | CONFIRMED | All 34 TS/MJS files parse. No `[DEBUG] ` left in the 38 listed files. Rust removals are whole statements with balanced braces. |
| gofix7 | CONFIRMED | Guard uses `sandbox.Run` / `sandbox.Problems`, and the `go.mod` replace exists. Go tests not re-run (builds not allowed). |
| owners | ISSUES (minor) | Registry has 3674 owners, not the 3675 claimed. The `trinity-jack-lsp` row is still present while its manifest is gone. |
| bzl-label | CONFIRMED | Fail-closed decision at `…/🏗️bazel/📜️script.ts:178`, reasons `undeclared` / `unbounded` present. Schema regex not re-read. |
| xplat-ci | CONFIRMED | Action SHAs match the course workflow pins. Bun SHA-256 pins match upstream for linux-x64, darwin-aarch64 and windows-x64. |
| deps3 | ISSUES | Reader-scope fix and displaced-lock guard are correct (no weakening). The workspace cleanup left `…/rust/pkg` entries in four `package.json` files (item 2). |

## Focus 1: cache correctness

Sample: 40 `cache: true` targets flipped against HEAD (14 framework, 14 `✏️s`, 12 rest), chosen by even stride over `git diff` of `project.json` targets. Every sampled handler was scanned for file-system reads (`readFile*`, `readdir*`, `Bun.file`, `existsSync`, `join/resolve` with literal `../` paths). Roughly ten were read in depth. The other thirty were checked structurally only (outputs present or explicitly `[]`, inputs present or policy default, no write verbs in handler). Reads that go through imported TS modules are covered by the assembly's import closure. Reads of non-TS files (JSON fixtures, `.rs`, `.sql`) are covered only if an input glob or a `reads` named input names their root. Sampled cases checked against this rule:

- `⏯️tool-run` `test`: reads stay inside `⏯️tool-run/**` (covered by `default`). OK.
- `🌱️value` `test-native-recipient-source`: reads `../../🧬️schema/…` and sibling modules. Covered by `reads`. OK.
- `📡️replication` `test-retained-diagnostic-source`: reads `📐️format/🔎️verification`, `🎮️mutation`, `📡️wire`. Covered by `reads` (added by cache-fw). OK, but only as far as the static analysis saw.
- `🎒️pack` `test-contract`, `🖱️ui/🧬️contract` `generate`, `🏙️bim` `test-complete-sqlite-native` (standards subsets listed in project `nativeSources`). OK.
- `🌎️hub/🧩️compositions/🗄️stdio` `artifact-directory-wiring-check` and `subset-directory-wiring-check`: ISSUE (item 1).
- `🗄️stdio` `test-snapshot-sqlite-gif-source` (a different class in the same script): not affected.

Not verified by the auditor, cross-cutting: every `repo:script` test handler runs `node_modules/typescript/bin/tsc` (for example `🧰️framework/…/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts:30`). No input covers the TypeScript binary version. Low risk, follow-up 5.

### Violations

1. **Stale cache hit: stdio wiring checks** (owner: cache-rest).
   - `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📋️project.json:20-28` (`artifact-directory-wiring-check`, `cache: true`, `outputs: []`, no `inputs`) and `:37-45` (`subset-directory-wiring-check`, same).
   - Handler walks the whole `✏️s/🔌️plugins/🗄️stdio` tree with `stdioWalkText` and reads every `.ts`, `.json`, `.feature`, `.semio`, `Cargo.toml` and `package.json` in it: `…/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:83-104` (artifact) and `:137-163` (subset). `stdioWalkText` is in `…/🗺️stdio-text-walk-and-surfaces/🟦️.ts:12-20`.
   - Effective inputs are the project `namedInputs.default`, which covers only `{workspaceRoot}/✏️s/🔌️plugins/🗄️stdio/**/*.rs`, `{projectRoot}/**/*` and the `📥️execution` tree. A change to any non-Rust file in the stdio tree outside the crate (for example `🗿️artifacts/🔣️.json`, `📜️artifact-definition.json`, a `.ts` owner file) does not invalidate the check. Result: a stale green.
   - Fix: give both targets explicit `inputs`: `["default","^default","{workspaceRoot}/✏️s/🔌️plugins/🗄️stdio/**/*","{workspaceRoot}/🌎️hub/🧩️compositions/🗄️stdio/**/*"]` (or one project named input `stdioTextTree` shared by both). Keep `outputs: []`.

2. **Stale workspace entries** (owner: deps3, see below).

## Focus 2: AGENTS rules

- Docstrings: all added `/**` lines in the touched production files start with an emoji (checked in deps3 storage, bazel adapter `📜️script.ts` and `💾️storage/🟦️.ts`, and the build script). No violation.
- Comments inside definitions: none found in added lines of the same files. Added `//` lines were not present; all 17 added comment lines are docstrings.
- Compat layers and deprecations: none added in the checked diffs. The only grep hit was the Bazel flag `--incompatible_autoload_externally`. The rules-fix report explicitly keeps some public re-exports (`preview2-shim-patches`, `host-shim-source`) and converts `export *` to explicit names. Acceptable, but these are shim-named surfaces; a future cleanup could remove them if no client needs them.
- Test rules: rules-fix, owners and cache changes rely on TS fixture tests only. Language-agnostic test and third-party cross-check are required by AGENTS. The bazel and xplat-boot tests do have third-party cross-checks (Ajv, Python `re`, Bun.YAML). Owners and cache-fw/s/rest lack one. Follow-up 6.

## Focus 3: rules-fix parse check

- `bun build --no-bundle --target=bun <file>` on the 34 TS/MJS files listed in `C/rules-fix-tools/debug-stripped-files.txt`: 34 pass, 0 fail.
- `command grep '\[DEBUG\] '` over all 38 listed files: 0 hits.
- Rust files (`🔐️publication/🦀️.rs`, three Pack `⏳️cursor/🦀️.rs` files): the removed `#[cfg(debug_assertions)]` plus `if` blocks are whole statements. Textual brace counts balanced in all four files (not compiled; no cargo allowed).
- Fused if-heads: none found in the diff hunks read. The `strip-debug.ts` tool handles braceless heads.

## Focus 4: xplat-boot checksum pins

Checked against upstream (network available):

- Node v24.15.0 (`SHASUMS256.txt`): darwin-arm64 `af5cfaea…546930f`, linux-x64 `47265558…4ba0f6`, win-x64 `cc5149ea…bb0e62`. All match `NAT/🧬️schema/🔣️.json` rows. 3/3.
- Go 1.26.9 (`go.dev/dl?mode=json`): darwin-arm64 `f9bb7c0a…022cc`, linux-amd64 `42d158b4…c5cc2f8d`. Both match the pin rows. 2/2.
- Bun 1.4.2 (`SHASUMS256.txt` of `bun-v1.4.2`, also used by `.github/workflows/build-system.yml`): linux-x64 `36368fae…a913`, darwin-aarch64 `90987a3a…1be12`, windows-x64 `ce4c1749…6405`. All match the workflow. 3/3.

Format check: pin manifest is one row per line; schema `Pins.const.assets`. Not verified: PowerShell parse (no `pwsh`), Windows FFI, dotnet-install commit `5147e323`.

## Focus 5: deps3 guards

- Reader-scope fix: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts:121-124` returns from the `dependencies` branch before the `"dependencies"` reader at `:131`. The diagnostics reader at `:92` still wraps every command. Correct, guard untouched.
- Displaced-lock identity: `…/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` (new `sameDisplacedJavascriptDependencyFileIdentityV1`) drops only `ctime`. The guard still requires the SHA-256 digest to equal `row.sha256`, no trailing bytes past the recorded size, and the identity match (line with `Exact displaced Source lock content retained`). The selected-payload comparison at `:103` still includes `ctimeNs`, unchanged. Not weakened.
- Workspace cleanup: the report says the deleted `…/🔌️jack/🧠️lsp/📦️packages/🦀️rust` entry was removed. Actual state: `✏️s/package.json:211` and `🌎️hub/package.json:92` (and the same in `🏢️semio-tech/package.json` and `♻️mit-bestand/package.json`) now list `…/🧠️lsp/📦️packages/🦀️rust/pkg`. `pkg/` is untracked build output of the deleted crate. ISSUE: follow-up 2.

## Other findings

- `📋️build-owners.json`: 3674 owners at audit time (owners report: 3675). `@semio-tech/trinity-jack-lsp` row still present (grep count 2 in the file), its manifest deleted. Owners open item 2 is still open.
- `generate-scale-fixture` in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` is `cache: true` with no `outputs` and no `inputs` key (HEAD had neither cache nor outputs). It writes a fixture. Nx restores nothing on a hit. Bazel treats it as `unbounded` (not cached, safe). Unclear whether the plugin injects outputs (generator contract). Follow-up 4.
- Generators with `outputs: []` that write tracked files (`generate-viz`, `generate-font-metrics` in `📓️print`, `preview-generated` family): accepted by cache-fw as policy, but a hit does not restore a deleted tracked output. Follow-up 5.

## Follow-up tasks

1. (cache-rest owner) Add explicit `inputs` covering the stdio text tree to `artifact-directory-wiring-check` and `subset-directory-wiring-check` in `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📋️project.json`. Verify the first graph load shows the inputs.
2. (deps owner) Delete the four `…/🧠️lsp/📦️packages/🦀️rust/pkg` workspace lines (`✏️s/package.json:211`, `🌎️hub/package.json:92`, `🏢️semio-tech/package.json`, `♻️mit-bestand/package.json`). Also delete the orphan `pkg/` build output and the `node_modules` symlink in the deleted crate directory. Re-check bun lock files.
3. (owners owner) Re-run `C/owners-tools/owners-check.ts --list` after the peer change settles. Reconcile 3674 vs 3675 and remove or repoint the `trinity-jack-lsp` row.
4. (cache-fw/cache-rest owner) Confirm whether `generate-scale-fixture` (and the other `generate*` cache:true targets with no `outputs` key) get outputs injected by `project-assembly`. If not, set `outputs` or `cache: false`.
5. (cache owners, cross-cutting) Add `typescript` package manifest and `bun.lock` to the inputs of `repo:script` test targets that spawn `node_modules/typescript/bin/tsc`, or a shared named input `tscTooling`.
6. (repo-build owner) Add a fixture-driven, language-agnostic check plus a third-party cross-check for the owners registry gate and for the cache-coverage gate (AGENTS rule).
7. (xplat-boot owner) First Windows CI run is the proof for the PowerShell and FFI paths. Nothing else to do before that.

## Not run by the auditor

No builds, no `bun repo` commands, no cargo or Go test runs, no `bun test` of executor test files. Verdicts on test pass counts rely on the executors' reports. Verified statically and against upstream as described above.
