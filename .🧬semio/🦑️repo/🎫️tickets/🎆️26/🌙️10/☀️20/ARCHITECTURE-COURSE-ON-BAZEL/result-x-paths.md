# Result x-paths: Bazel Paths Within 256 Units On All Four Environments

Status: DONE, staged only. Handoff: `handoff-bazel-short-paths.md`. Tools: `x-paths-tools/` (`apply.py`, `check.sh`, `edits.py`, `spec.py`, `genfixture.py`, `measure.py`, `mkoverlay.py`, `mknodemodules.py`, `revert.py`, `staged/`). Logs: `🗑️generated/x-paths/` (`red.txt`, `check.log`, `measure.json`).

## Incident (my fault, repaired)

Between 15:52 and 16:19 my `apply.py` run reached the live adapter instead of the verification overlay (the overlay held a path that resolved into the checkout; the exact link was lost with the scratchpad at the reboot). The patch was live for about 27 minutes in the 9 files below plus `🧪️tests/📥️paths/`. At 16:19 I removed it with `x-paths-tools/revert.py` (files that only my patch changed were restored from `git show HEAD`, after proving HEAD + patch equals the file; `📜️script.ts`, the schema and the test aggregator got the exact inverse of each anchored edit, so peer edits stayed). Verified afterwards: no `📥️paths`, no `bazelPathBudgetV1`/`PathBudget`/`aliasRule` in the live files, `apply.py --check` on the live tree shows 45 pending and 0 anchor failures, live `📜️script.ts` md5 unchanged by every later step. Backup of the patched live files: `🗑️generated/x-paths/live-patched/`. A peer that read those files in that window saw my patch. Guards added: `apply.py` refuses a git checkout without `--live` and refuses any write that resolves outside `--root`; `mkoverlay.py` skips `.git`. Nothing live has been touched since; the tree stays quiet as requested.

## 1. Verified numbers (independent `measure.py`, UTF-16 units, closure = 51 packages, 3,458 files, 2,945 directories)

| Host | Output base | Old source (`s/<repo path>`) | Old runfiles-source | New longest | Governing family |
|---|---|---|---|---|---|
| macOS `/Users/ueli/Documents/semio` | 78 | 260 | 331 | 231 | runfiles-snapshot |
| Linux `/home/ueli/Documents/semio` | 77 | 259 | 320 | 220 | runfiles-snapshot |
| devcontainer `/workspaces/semio` | 68 | 250 | 316 | 216 | runfiles-snapshot |
| Windows `C:\Users\ueli\Documents\semio` (assumed typical) | 80 | 262 | none (`--noenable_runfiles`) | 198 | snapshot |

The audit is confirmed (260, 259, 250, 262). It missed the test runfiles tree (`bin/t_<16>.runfiles/_main/s/<repo path>`, 316 to 331). Longest repo path in the closure: 164 units (166 with `s/`), longest basename 34 units; longest basename in the whole repo 83 (71 when labellable). Layout facts: `linkBazelSourcesV1` symlinks the checkout as `<ws>/s`; Bazel symlinks `execroot/_main/s` to it (observed in a live output base), so the long path is the logical path through the link, not an entry Bazel creates, except in sandboxes and runfiles trees where it creates one link per file. Output base is `<root>/.cache/b/p/m/bazel/outputs/server/<16 hex>`; configuration segment `darwin_arm64-fastbuild` observed. Remaining slack on macOS is 25 units (checkout root up to 52 characters), Linux 36 (62), devcontainer 40 (57), Windows 58 (87).

## 2. Design

See the handoff. In short: host-state actions use `a/<10 hex of sha256(dir)>/<name>` through a directory-symlink farm (junctions on Windows), the generator measures the closure into `paths.json`, the platform profile evaluates all five path families per OS from schema templates (repository rule 256; macOS 1023 bytes, Linux 4095 bytes, Windows 259 units or 32766 with long paths), refuses only on real overflow and names the family and the shortening needed. Hermetic actions keep the repository layout and are budget-checked. Readiness no longer demands LongPathsEnabled. The 256-unit rule is read from the taxonomy policy in a test, not duplicated silently.

## 3. Tests and results

- RED first: `Export named 'bazelPathBudgetV1' not found` against the unpatched adapter (`🗑️generated/x-paths/red.txt`).
- GREEN: `check.sh` status 0 (`🗑️generated/x-paths/check.log`): new test file 11 pass, 1 skip (real Bazel); 16 regression files identical between unpatched and patched overlays (platform 4 to 3 pass because its constant-based MAX_PATH test was replaced); aggregator 22 failures in both, identical names (overlay-only: process routing, nx parity); `tsc` on the touched files: 0 messages in base and patched.
- Oracles: Ajv (fixture and component schemas, strict), Python hashlib and an independent Python budget evaluation, iconv-lite lengths, taxonomy 256 rule, a generator integration test (one hermetic and one host-state task, every BUILD label resolves).
- Not run (brief): real Bazel, Windows, `bun repo` (frozen/rebooted tree).

## 4. Findings for the coordinator

- x-inputs' hardlink mirror made `🏗️build/📥️inputs/🔏️revision/🟨️.mjs` nlink 2 and broke every `bun test`/`bun repo` boot (already fixed by you at 15:58).
- After the reboot the dependency partition was unmounted (`node_modules` dangling); `check.sh` falls back to a private node_modules from the bun cache, so it runs with or without the mount.
- Windows git checkout needs per-user `core.longpaths` (deepest tracked path 226 units plus the root); not machine-wide.

## Backend edits

None live (staged only; see the Incident for the reverted slip).
