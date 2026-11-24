# 🧹 Old Checkout `C:\git\semio` — Merged into `C:\semio` and Removed (2026-11-23)

The old checkout was on `🐙ueli/⛳wip` at commit 746 (5ececaa) with 706 uncommitted entries and 25 stashes. It builds on the first rescue of 2026-11-22 (`GET-FEM-PLUGIN-WORKING-END-TO-END/git-semio-rescue.md`). No file in `C:\semio` that was newer than its old-checkout counterpart was overwritten.

## Uncommitted work (706 entries, `old-checkout-classify.mjs`)

| Class | Count | Action |
|---|---|---|
| Identical in `C:\semio` | 663 | none |
| Already contained (merge equals live) | 13 | none |
| Old side is real later work, live still at commit 747 | 21 | 20 written, 1 (`render.mjs`) already contained |
| Old side equals an older commit in live history (superseded) | 7 | live kept: 3 quiz JSONs, `media_staffage.py`, `browser/🟦️.tsx`, `.cases.json`, `shared-border.ts` |
| Only in old, tool logs (`tests-final/rerun.txt`, `rerun3.txt`) | 2 | skipped |

The 21 files are the 2026-11-14 "fix-last" review fixes and the `_din_ref` → `reference_chip` refactor of the energy course (the first rescue had left them as open items because it merged against base 746; the true common ancestor is 747, where live and base are equal, so the merge is the old side). Written with compare-and-swap against the live content, LF line endings, under `🎓️teaching/🏛️architecture/⚡️energy/`:

- `🌬️ventilation/🎬️clip/{⚙️mechanical,❓️need,🌡️heat-flow,📦️volume,🤝️handoff,🪟️natural}/🐍️.py`
- `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json`, `🏗️authoring/🎬️clip/README.md`
- `📊️demand/🎬️clip/{♨️system-losses,⚖️regulations,⚡️demand-power,🏭️primary-energy,🏷️certificate,🔁️recap,🧪️practice}/🐍️.py`
- `🔥️heating/🎬️clip/{☀️solar-gains,🌬️convection,👥️internal-gains,🧲️fundamentals}/🐍️.py`
- `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`

Checks: every `.py` parses, the JSON parses, no `_din_ref(` is left, no conflict markers. After the write, all 21 paths still hold the merged content and `published-check.mjs` reports all 21 as published to the fleet; none went to `.cache/f/lost`. The Manim clips were not rendered.

## Commits, refs, stashes

All 192 refs of the old checkout are reachable in `C:\semio`'s objects, including the 25 stashes (`refs/old-repo/*/stash-*`), except:
- 6 `refs/agents/*` checkpoint refs → `C:\git-semio-rescue-archive\agent-checkpoints.bundle` (1 GB);
- 2 GitKraken empty-tree helper refs (no content).

`C:\semio` has no object alternates pointing at the old checkout.

## Git-ignored content

374 ignored entries existed only in the old checkout. Caches and build output were dropped: `.repo/cache`, `.p<hash>` install dirs, `node_modules`, `🤖️generated`, `🗑️generated`, cargo targets, wasm bindings, bun lock backups, `.🧬semio/🎓️teaching` (e2e cache, 3.5 GB), `.🧬semio/🦑️repo/⚡️cache`, `*.out` tool logs. The 12 ignored ticket documents (`target*` names) were already in `C:\semio`.

Archived outside the repos in `C:\git-semio-rescue-archive\`:

| File | Content |
|---|---|
| `ignored-leftovers.zip` (66 MB, 3329 files, `archive-ignored-leftovers.py`) | old `.repo/🎫` tickets (2026-05), `FINISH-ARCHITECTURE-COURSE-VIDEOS/generated` (rendered mp4/svg), stray ignored sources of old layouts (`🔤️tokens`, `🌐️locale`, `storybook-interaction`, os fixtures, generated `.cs/.go/.py/.rs`, 3 `Cargo.lock`) |
| `temp.zip`, `launch-kits.zip`, `claude-settings.local.json` | from the first rescue |
| `agent-checkpoints.bundle` | see above |

## Removal

No scheduled task referenced `C:\git\semio`. All content, including `.git`, was deleted with `rd /s /q`. One process held `.git/tgrep-daemon-v1.lock`: the VS Code Copilot search indexer `tgrep.exe serve --shared C:\git\semio` (pid 50884, started 2026-11-22 16:34); it was stopped by that exact pid and the lock file removed. The now empty folder `C:\git\semio` itself could not be removed ("in use by another process"): GitKraken was closed and it still failed. `process-cwd.ps1` (reads each process's current directory) shows the holder: the Claude desktop session "Fem plugin end-to-end" has `C:\git\semio` as its working folder (`claude.exe` pid 31448 and its terminal `pwsh.exe` pid 31352, both started 2026-11-23 16:09). They were not stopped. Once that session is archived or closed, the empty folder can be removed. `C:\git\semio.tech` and `C:\git\tmp_none` were not touched; `C:\git\semio.worktrees` did not exist any more.

## Notes

- `origin/🦢niloufar/⛳wip-from-ueli-748` (Niloufar's 2026-11-20 polish branch) is not merged into the live tree; it is on GitHub. Out of scope here.
- Archivist commit 752's unapplied files stay in `GET-FEM-PLUGIN-WORKING-END-TO-END/git-semio-rescue-commit-752-only-files.zip` (first rescue).
- The repo ticket tools were unavailable in this session, so the ticket was not reopened or closed through them.
