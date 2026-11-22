# Disk Headroom Audit 2026-11-22 (read-only)

Status at 19:35 CET: `/System/Volumes/Data` 926 GiB, 831 GiB used, **69 GiB free (93%)**. Target: about 100 GiB free before the release build. Shortfall: about 31 GiB.
Nothing was deleted, moved, detached or committed. All commands were read-only (`du`, `stat`, `ps`, `lsof`, `hdiutil info`, `repo cache status`, `repo cache cas status`, `repo cache gc --dry-run`, `repo cache dedupe --dry-run`, `repo cache cas gc --dry-run`).

## Key finding

Almost all space in the repo's `.cache` is **mounted, live disk images** (`.cache/b/*/fixed.dmg`, 15 images mounted through `diskimagesiod`). Their files were written in the last 15 minutes and bun processes run out of them. None of `.cache/b/*` is safe to delete or detach while the build runs. Tools that exist for this (`repo cache gc`) are currently blocked (see below).

## Ranked candidate table

Size = `du` allocated. "Live" means a process or a fresh mtime shows current use.

| Rank | Candidate | Size | Last modified | Live holder | Judgement |
|---|---|---|---|---|---|
| 1 | `.cache/b/d` (`fixed.dmg` 36G + mounted `m` 22G) | 58 GiB | fixed.dmg 19:30 today | pid 74517 diskimage, bun | **UNSAFE** (live, mounted) |
| 2 | `.cache/b/g` (graph partition, `fixed.dmg` 19G) | 22 GiB | 19:33 today | pid 65448, bun | **UNSAFE** now. Ledger says graph image 18 GiB, 6 GiB declared, "recreate pending". Candidate for shrink AFTER the build |
| 3 | `.cache/b/t` (tools, bun runtime) | 8.0 GiB | 19:33 | pid 67327, bun 18683/32321/41187/49378 | **UNSAFE** (live). Dedupe can reclaim up to 0.49 GiB, see below |
| 4 | `.cache/b/r` (rust partition) | 6.7 GiB | 19:33 | pid 81239 | **UNSAFE** (live, cargo) |
| 5 | `.cache/b/p` (products) | 3.3 GiB | 19:33 | pid 58304 | **UNSAFE** (live) |
| 6 | `.cache/b/n` (native) | 1.4 GiB | 17:31 | pids 67079-86112 | **UNSAFE** (mounted) |
| 7 | `.cache/b/l`, `o`, `c`, image.json files | 0.15 GiB | 19:19-19:33 | mounted | **UNSAFE** (mounted) |
| 8 | `.cache/f` (1.5 GiB), `.cache/scratch` (0.44 GiB) | 1.9 GiB | 19:23 / 19:17 | active | **UNSAFE** now (written in the last 20 min) |
| 9 | `/Users/Shared/semio-store/v1` (replica store, 53 GiB) | 53 GiB | heads/origins 19:22 | live replica, 50+ machines | **UNSAFE**. Only the store's own compaction may touch it |
| 10 | `~/.cache/semio/cas` (repo CAS, 79,652 objects, 23.3 GiB of 32 GiB cap) | 23.3 GiB | live | — | **Safe via repo gc, not rm.** `repo cache cas gc` would free 5.07 GiB now (see below) and is the only path that brings `/` down to its 80% low watermark |
| 11 | `~/Library/Caches/semio` (1.9 GiB: repo 995M, execution-targets 848M) | 1.9 GiB | repo 15:46 today, execution-targets 2026-09-26 | runtimes/bun + lease-registry live | **Unsafe to rm** while leases are held; use `repo cache gc`. execution-targets is 2 months old and is the best candidate once gc runs |
| 12 | `.🧬semio/🦑️repo/⚡️cache/tools` (elan 2.7G, ms-playwright 572M) | 3.3 GiB | ledger 10 h ago | none | **Safe-ish**. Elan is a Lean toolchain (2.7 GiB, not used by semio builds per ledger 91% of 3G budget); playwright re-downloads. Reclaim only via `repo cache gc --area=tools` after the build |
| 13 | `.🧬semio/🌐hub` (1.3 GiB total) | 1.3 GiB | s14-* from Sep 27-29 and Nov 11; `play-cdn-1122` today | none for s14-* | **Safe after owner confirm.** Oldest s14-* ticket artifacts: `s14-u6-base2` 567M, `s14-r10-state` 264M, `s14-sh2-overlay` 248M, `s14-h13-test-artifacts` 51M, `s14-s20-overlay-faults*.baseline.json` 58M, `s15-ex1-backup` 24M, `s13-r9-git-index` 32M. Do NOT touch `play-cdn-1122` |
| 14 | `~/Library/Caches` (18 GiB total, non-semio app caches, apps not running) | see table below | mostly Jul-Nov | no process found | **Safe to clear when apps are closed** (caches regenerate) |
| 15 | `.🧬semio/🦑️repo/🎫️tickets/**/🗑️generated` (ticket generated output) | 13.6 GiB total | mostly 2026-08-28 and 2026-10-18 | none | **Safe** when ticket is closed (CLAUDE.md rule). Biggest: DEMONSTRATOR-END-TO-END-ALL-APPS 10.08 GiB (08-28), GET-CODEBASE-RUNNING-END-TO-END-AGAIN 1.79 GiB (10-18), LAYOUT-PLUGIN-END-TO-END 1.28 GiB (09-16) |
| 16 | `/Users/ueli/.sinc` | — | — | — | Not present |
| 17 | Target / `incremental` dirs | — | — | — | None found within depth 6 of the repo. Cargo target data lives inside the mounted images (`.cache/b/*/m`), so it is covered by rows 1-6 |

### Library caches (~/Library/Caches, 18 GiB; no matching process found for these apps)

| Dir | Size | Last modified | Note |
|---|---|---|---|
| com.spotify.client | 4.5 GiB | 2026-11-16 | Safe (app cache) |
| copilot | 2.0 GiB | 2026-11-12 | Safe |
| Google | 1.9 GiB | — | Check Chrome is closed (claude-in-chrome may use it) |
| BraveSoftware | 1.2 GiB | — | Safe when Brave closed |
| Zotero | 1.0 GiB | — | Safe when Zotero closed |
| Codex | 1.0 GiB | 2026-07-14 | Safe |
| ms-playwright + ms-playwright-go | 0.68 GiB | — | Safe, re-downloads |
| Homebrew | 0.54 GiB | — | `brew cleanup -s` |
| Steam, Firefox, Thunderbird, electron | 1.46 GiB | — | Safe when closed |
| antigravity + antigravity-updater + cursor-compile-cache | 1.0 GiB | — | Safe when closed (IDE caches; not used by this workflow) |
| go-build, pip, bun, Tectonic | 0.47 GiB | — | Safe (`go clean -cache`, `pip cache purge`, `bun pm cache rm`) |

Library total of "safe when apps closed": about 13.6 GiB.

## Repo cache ledger (read-only commands)

- `repo cache status` (statfs + ledger, no walk):
  - Partition graph is over budget: 10.53 GiB used of 6 GiB declared, image 18 GiB, "recreate pending (derived data, rebuilt cold once)".
  - Other partitions are within budget. Machine budget 198.31 GiB default, 49.62 GiB in use, 66.31 GiB on disk.
  - Retired root `native:test-admission` and undeclared `native:native-play-artifacts` are still on disk (small).
- `repo cache cas status`: `/Users/ueli/.cache/semio/cas` 79,652 objects, 23.3 GiB of 32 GiB cap. Partition `/` is 92.8% used (collection from 90% down to 80%, writes and builds refused from 97%).
- `repo cache cas gc --dry-run`: would remove 14,271 entries and free **5,072 MiB**. Protected data 18.98 GiB. Still 116,661 MiB short of the 80% low-water target. Hint: `repo cache gc --area=graph-inference` for unreferenced source inventories.
- `repo cache gc --dry-run`: **failed** with `Internal error: Same complete admitted Source resolution root required (reference 26b66dc4)`. Nothing removed. Needs a fix in the repo tooling before it can be used.
- `repo cache dedupe --dry-run`: skips bazel-outputs, bazel-workspaces, vite-products (not mounted). In tools (`.cache/b/t/m`): 35 identical-file groups, 126 files, **at most 494.51 MiB** reclaimable by clone.

## Exact commands to reclaim (run only after the release build is done or when it is safe; each is a candidate, not executed)

1. Once the build is idle, the cache collector (fix the admission error first):
   `cd /Users/ueli/Documents/semio && repo cache gc --dry-run` then `repo cache gc`
2. Repo CAS, to bring `/` below 90% (frees about 5 GiB per dry run, more as the graph is rebuilt):
   `repo cache cas gc --dry-run` then `repo cache cas gc`
3. Dedupe in the tools image (about 0.49 GiB, clones only):
   `repo cache dedupe --dry-run --area=tools` then `repo cache dedupe --area=tools`
4. Library caches, with the app closed (about 13.6 GiB total):
   `rm -rf ~/Library/Caches/com.spotify.client ~/Library/Caches/copilot ~/Library/Caches/Codex`
   `rm -rf ~/Library/Caches/Zotero ~/Library/Caches/Steam ~/Library/Caches/Firefox ~/Library/Caches/Thunderbird`
   `brew cleanup -s` (Homebrew), `go clean -cache`, `pip cache purge`, `bun pm cache rm`
5. Ticket generated folders for closed tickets (about 13.6 GiB), e.g. the largest:
   `rm -rf "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated"`
   Check the ticket is closed first.
6. Hub s14 artifacts, after owner confirms (about 1.2 GiB):
   `rm -rf "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-base2" "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-r10-state" "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-sh2-overlay"`
7. After the build: shrink the graph partition from 18 GiB to its 6 GiB declared size, which frees about 12 GiB on the host (the image is recreated cold). Do this through the repo tooling (`repo cache gc --area=graph`), not by deleting `.cache/b/g/fixed.dmg` by hand.

Never run by hand: `rm` on anything under `.cache/b/`, `/Users/Shared/semio-store`, `~/.cache/semio/cas`, or `~/Library/Caches/semio`. Those are live or mounted.

## Summary of reclaimable headroom

- Safe now, no build impact: Library app caches (about 13.6 GiB, apps closed), ticket generated folders of closed tickets (about 13.6 GiB, not all needed), hub s14 artifacts (about 1.2 GiB). Realistic total about 25 GiB with owner confirmation.
- Safe via repo tooling: CAS gc about 5 GiB now; dedupe 0.49 GiB; graph shrink about 12 GiB after the build.
- Not safe now: all of `.cache/b/*` (about 100 GiB, mounted and live), the replica store (53 GiB), the repo CAS and `~/Library/Caches/semio` (live leases).

## Open items

- `repo cache gc` is broken (Source resolution root admission error, ref 26b66dc4). This is the blocker for the clean collector path.
- Home cache `~/.cache` (76 GiB) has many dirs from Oct 2026 (`bd10`, `c4p*`, `c5*`, `c6db*` ...). Their sizes are in the section below if the scan finished.

## Home cache `~/.cache` (76 GiB) age scan

- Entries older than 30 days: **50.29 GiB**. Entries touched in the last 30 days: 23.03 GiB.
- Largest old entries: `~/.cache/uv` 22.43 GiB, `~/.cache/r5a` 10.27 GiB, `~/.cache/r4` 10.21 GiB, `~/.cache/r9a` 1.11 GiB, `~/.cache/mwu` 0.91 GiB, `~/.cache/r11` 0.54 GiB, `~/.cache/mathlib` 0.44 GiB, `~/.cache/r7b` 0.42 GiB.
- Judgement: `uv` is a package cache and regenerates (safe, `uv cache clean`). The `r*`, `mwu`, `tc1`, `ohp` dirs look like old per-tool or per-ticket caches with no live process found. They are safe to remove after checking nothing is running from them. `~/.cache/semio/cas` is NOT in this set and stays live.
- Reclaim, when the build is idle: `uv cache clean` and `rm -rf ~/.cache/r5a ~/.cache/r4` (after checking those names are not used by a running tool).
