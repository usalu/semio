# result-docs-refresh: build guide refresh (EN then DE), executor docs-refresh, 2026-10-22

## Outcome
- Round-2 set (`docs-1021a-tools`, status O27) APPLIED to `🧰️framework/🛍️products/🦑️repo/README.md`: `refresh.sh --write` finished at 02:50 (dry run 02:45, write started 02:46). README now 2283 lines, sha256 de98d16ff34a242f. Re-check at 02:51: `apply.py` reports "0 pending, 22 applied, already applied"; `bun repo --help` unchanged since the first capture at 02:33.
- Ten new sections per language (20 headings, 10 tables, 14 fences, 152 commands each side), EN before `Cross-platform notes`, DE before `Hinweise je Plattform`.
- The doc laws could NOT be proven green: every README law that loads the taxonomy is blocked by a non-doc failure (below). No README-content failure was found.
- Not the tool named in status O27: `docs-1020e-tools` is finished; the tool used is `C/docs-1021a-tools`.

## Changed files
- `🧰️framework/🛍️products/🦑️repo/README.md` (live, applied through the tool, plus direct edits listed below).
- `C/docs-1021a-tools/applied.json` (record rebased, backup `C/🗑️generated/docs-refresh-applied-before.json`).
- `C/docs-1021a-tools/staged/en/040-shared-caches.md`, `staged/de/040-shared-caches.md`: `bun repo` call became `repo` call (peers had changed it live; launcher is live).
- `C/docs-1021a-tools/staged/en/095-recovery.md`: "for inactive Graph or Nx caches" (peer wording ported).
- `C/docs-refresh-tools/rebase-history.py` (new helper).

## Why the tool needed a rebase
`refresh.sh` refused at first: peers edited round-1 text live (sections-en/de, guide-entry-en/de, default-en/de), so the recorded applied text was no longer in the README. `rebase-history.py` set those six records to the current text, then `make-edits.py` regenerated them from the pristine README and fragments. The regenerated text fixes the peers' `repo`-for-`bun repo` fallback sentences and the parity fallback (now conditional on the launcher being live).
Tool issue: `refresh.sh` pipes `make-edits.py` through `sed -n 1,2p`, so its assertion failure did not stop the run (the exit code was hidden). Fix before the next use.

## Doc fixes against CLI help (both languages)
- Help rule for `--lang` (EN "Help" bullet, DE "Hilfe" bullet): was "then the first language of the catalogue"; now `--lang`, `SEMIO_LANGUAGE`, `LANGUAGE` and the locale, then the operating system language, and no default language (matches `bun repo --help`).
- Fallback sentences (guide intro, EN and DE): `bun repo`, the root package script (peers had replaced it with `repo`, which contradicted "where repo is not on PATH").
- Parity fallback `repo ./📜️script.ts` sentence removed when the launcher is live (conditional in `090-parity`); no `repo ./📜️script.ts` form remains in the EN or DE parity text.
- `tools setup [nx|bazel|toolchains|all]` in the command block and the reference table (EN and DE): `toolchains` was missing; peers had already fixed the text, verified now.
- `tools setup all` installs Nx, Bazel and every native toolchain (EN and DE): verified against the root help.
Checked with no disagreement: every `repo <command>` in the guide against `bun repo --help` and `bun repo <command> --help` (run, run-many, affected, exec, watch, graph, show, inputs, reset, tools, preflight, incremental, concurrency, dependencies, cargo, cache, capacity, leases, remote-execution); every `--flag` in the guide EN and DE (only verify/parity and Bazel/Nx flags remain, which are not in `bun repo` help; the parity flags match `🪞️parity/🔣️help.json`); scenario id `fw-ts-comment` and workload `disjoint-projects` exist (`bun repo incremental list` and `bun repo concurrency list`).

## Open (not changed, needs owner decision)
1. Native launcher: `/Users/ueli/.local/bin/repo --help` prints a 4-line summary (no command or flag list), and `repo --version` answers "unknown flag: --version". The guide says `repo --help` prints every command and every flag, and the launcher fragment (`100-launcher`) documents `repo --version`. Only `bun repo --help` was used as reference here.
2. `verify build-parity --lang` help says the default is English; the global rule is "no default language". The guide follows the global rule.
3. `default-en`/`default-de` (FLIP sentence) still say `bun repo` after the rebase; the rest of the guide says `repo`. Both forms run; wording not unified.
4. Doc laws (live tree, explicit `bun test ./<file> --timeout 240000`, with `SEMIO_TEST_ARTIFACT_DIR` under `C/🗑️generated/docs-refresh-artifacts`):
   - `owner-projection-policy` 19 pass / 16 fail (35 tests): "Owner projection policy digest or UTF-8 drift" for `🖼️assets/🏠️owner-projection-policy/🔣️.json` (modified 10-21 17:20, staged `M` in the index; its pinned `authorityCatalogSha256` no longer matches). Not a doc issue. Output: `C/🗑️generated/docs-refresh-law-owner-projection-policy.txt`.
   - `readme-current-source-revision`, `readme-current-source-activation`, `testing-readme-coordinates`, `readme-reviewed-fixture-inputs`: stop at the same digest error (fixture law: 1 fail, same cause).
   - `frozen-markdown-coordinates`: 1 fail, "Producer static import lacks exact workspace authority" for `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts` importing `@semio-tech/framework-plugin/describe/component-receipt`. Code, not docs.
   - `readme-move-source-authority`: 4 fail (taxonomy move `sourceAuthority` schema); the tool's baseline was already 13 pass / 4 fail.
   Doc-caused failures: none found.
5. Last CLI check 02:51: root help unchanged. The integrators' live CLI changes after that are not in the guide; re-run `sh C/docs-1021a-tools/refresh.sh` after the next CLI change (states move with it).

## Processes
None left running. Background help captures finished (`C/🗑️generated/docs-refresh-help-*.txt`); the law logs are in `C/🗑️generated/docs-refresh-law-*.txt`.
