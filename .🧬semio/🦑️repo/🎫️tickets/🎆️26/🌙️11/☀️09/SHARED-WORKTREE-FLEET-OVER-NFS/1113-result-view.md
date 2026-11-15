# Result: working-tree view in the fleet attestation (2026-11-13)

Contract section updated: `1113-git-state-contract.md` (Attestation). Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`.

## What changed

- `🌿️git/🟦️.ts`: `gitViewV1(worktree, signal?)` runs `git --no-optional-locks status --porcelain=v1 -z --untracked-files=all --no-renames` (plus `GIT_OPTIONAL_LOCKS=0`), returns `{staged, modified, deleted, untracked, digest, lines}` or `null` (no git, no root `.git`, git failure, abort); never throws. `parseGitViewV1(text)` is the pure reader (rename/copy second field skipped, `!!` skipped, NFC paths, bytewise sort). `lines` = `XY path` of everything not purely staged (Y not blank, or `??`); `digest` = sha256 of `lines.join("\n")`. Counts: staged = X not blank; deleted = Y is `D`; modified = other Y; untracked = `??`. `--no-renames` keeps the lines independent of `status.renames`/`diff.renames` config.
- `🩺️attest/🟦️.ts`: attestation gains `view:{staged,modified,deleted,untracked,digest}|null` (required, schema `View`); `Row` gains `view`; field `view.digest` joins the verdict. An origin with `view: null` (no git) is exempt from `view.digest` only (head/indexTree keep their old behaviour: null counts as a value). New `viewPathV1`, `writeViewLinesV1`, `readViewLinesV1` (checks sha256 of the file against the attested digest), `viewDiffV1`.
- Scheduling (attester): git status runs at most every 60 s (`viewMilliseconds`) and once more when manifest, files, cursors, commit or index tree changed and stayed quiet for 5 s (`viewDebounceMilliseconds`, a trailing debounce, so steady churn falls back to the 60 s cadence); never concurrently (`viewBusy`), runs in the background so ticks do not block, cancellable through the daemon signal (an aborted result is dropped). A failed read keeps the last view. With git present the first attestation waits for the first view (a missing view must not look like "exempt"). `view.txt` is written (tmp+fsync+rename, own origin directory) before the attestation that carries its digest, only when the digest changed.
- `🔁️replica/🟦️.ts`: daemon wires `gitViewV1`; options `gitView`, `viewMilliseconds`, `viewDebounceMilliseconds` for tests; relay copies `view.txt` before `attest.json`; compaction does not touch it (tested).
- `🔁️replica/⌨️command/🟦️.ts` + texts (en, de): table column `view` = `modified/deleted/untracked`; `fleet --diff`.
- Schema, fixture (4 new verdict cases, 6 view vectors), Python oracle (independent porcelain reader with `hashlib`, `jsonschema`), README, contract.

## Command usage

- `repo ... fleet` : table now ends each row with `view` (`m/d/u`, `-` without git); a differing view is `diverged` with `view.digest` named.
- `repo ... fleet --diff [--worktree --store --lang]` : reads `view.txt` of this machine and of every other non-retired origin that attests a view; per origin that differs prints `<origin> differs from this machine: N lines only there, M lines only here`, then `only on <origin>: <line>` and `only on <origin> (here): <line>` (40 per side, then `+N more`). An origin whose attested view has no matching `view.txt` is listed as unavailable. Exit 0 when all equal, exit 1 when any differs or is unavailable, or when this machine has no view. Refused together with `--json` or `--wait`.
- Paths containing a newline are kept raw in `view.txt` (digest equals sha256 of the file); the line diff then splits such a path into two lines on both sides, which still diffs correctly.

## Tests (macOS, all green)

journal 127, boundary 39, git 10 (was 9: +1 real-git oracle test), attest 58 (was 43: +6 view vectors, +4 verdict vectors, scheduling, no-git/failed-read, three-replica verdict + `--diff` + equal exit 0, exempt origin, relay + compaction of `view.txt`), replica `🟦️.ts` 64, `🏁️race.ts` 14, `🌿️git.ts` 13 (was 12: +1 three real worktrees with a local-only absolute symlink, `view.digest` named by the verdict, `--diff` lists `?? local-link`, removal restores exit 0), bootstrap 12, topology 56. Python oracle `🩺️attest/🧪️tests/🐍️.py`: 38 vectors, 0 mismatches (jsonschema 4.25.1). The git unit test compares `gitViewV1` to `git ls-files -m/-d/-o`, `git diff --cached --name-status` and checks that a stat-dirty `.git/index` is not rewritten and that an existing `index.lock` is neither needed nor touched.

Development ran in a temp copy outside the repo (imports kept by symlinks), the finished files were copied into the live module in one batch after confirming the live files were unchanged since the copy; the temp copy is deleted. Boundary was re-run against the live module (the temp copy lacked the repo `.gitignore`); git and attest were re-run live after the copy.

## Live check after the copy

Daemons restarted themselves; g1-m01..m03 already attested the same view digest (staged 1391, modified 12); g1-server shows `untracked 1` (a ticket script file `1113-latency-probe.txt` present only there at that moment), which is exactly what `fleet` / `fleet --diff` is for. g1-m04 had not restarted yet when I stopped looking.

## Not verified / notes

- Linux and Windows not run; the porcelain parser is OS-neutral but `git status` timing on 270 000 files was not benchmarked here (the live first view took under ~40 s on the loaded server, subsequent runs not measured).
- Old daemons write attestations without `view`; the new schema rejects them, so until an origin restarts on the new code it is read as `missing` by new readers (no compatibility layer by design).
- The first attestation after a daemon start now waits for the first `git status`; on a cold 270 000-file tree that delays it (seen: about 30 s on g1-server under load).
- Nested-repository case is covered by the parser vector (`?? nested/`), not by a live three-replica test (the refused-symlink scenario is).
