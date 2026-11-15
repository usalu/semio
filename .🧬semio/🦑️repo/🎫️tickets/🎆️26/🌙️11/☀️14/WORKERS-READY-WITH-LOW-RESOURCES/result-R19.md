# R19 Executable bit in the replica

Status: fix PREPARED, not landed (replica digest change; to be batched with R12). Scripts: `T/🧰️scripts/R19/` (`land.py --dry-run` clean, `new-test.ts`). T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`.

## Facts (replica `🔁️replica`)
- Schema carries the bit: manifest entry `executable` (state schema `💾️state/🧬️schema/🔣️.json`; binary frame flag 1; book flag bit 3). Journal `put` events carry it.
- Apply sets it atomically with the content (blob copied with mode 0o755/0o644, line ~1721) and fixes a mode-only difference with `chmod` (~1684). A POSIX scan journals a chmod-only change (scan compares the disk bit with the book, ~1446): proven by the existing tests (git.ts: +x and -x both replicate) and live (below).
- Windows/modeless: never reads or writes the bit, keeps the recorded bit on content edits, takes the staged git 100755 for new files (existing test in git.ts, passes).
- Archive restore (`🥾️bootstrap`): tar header 0o755/0o644 from the entry, `chmod` on settle, drift detection on a bit difference; tests exist (3, pass).
- Provisioning overlay of sealed code (`🗺️topology/🥾️provision/🐚️.sh` overlay_apply): the seal (`sealed.json`) has NO executable field, files are `cp`ed from the sealed copy (mode 644, as sealed on this machine). Identical files are skipped, differing ones arrive 644. GAP, not fixed here (provision module, outside replica file-mode handling): overlay should keep the target's existing mode or the seal should carry the bit.

## Root cause (not "modes are not replicated")
The bit is replicated, but it is last-writer-wins on the writer's DISK bit. An editor that recreates a file (write temp 644 + rename, or the Write tool) publishes `put executable=false` with the new content, so any content rewrite silently drops +x everywhere. Journal of the three affected files: `g1-server put False` after earlier `True` events (unit-cache `📜️script.ts` ...652580513, `g1-m03`/`g1-m07`/`g1-server` False edits afterwards; launcher `📥️install/🐚️.sh` 1794689456694; replica `🥾️entry/🐚️.sh` 1794684980801/…868232).

## Counts (read-only, 2026-11-15 ~06:30)
- git index 100755 entries: 2771. Tracked regular files on disk: 273837 (1 missing).
- git 100755 but not executable on disk: 3. Executable on disk but git 100644: 0.
- The 3: `…/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts` (the wrapper R16 hit; its test now asserts the stand-in runs the script without a mode), `…/🚀️launcher/📥️install/🐚️.sh`, `…/🤝️shared-worktree/🔁️replica/🥾️entry/🐚️.sh`. Manifest also said False for all three (fleet truth was wrong, matching disk).
- Manifest (older state.json) vs git: 2 entries recorded +x where git has 644 (`🗺️topology/📜️script.ts`, `🗺️topology/🥾️provision/🐚️.sh`); not touched (git index may be the stale side).

## Repair now (done, no code needed)
`chmod 755` on the 3 files on this machine (g1-server). Journal shows mode-only `put` events with unchanged sha, `executable=true` (hlc 1794720552289-00000{0,1,2}-g1-server), i.e. chmod-only replicates. Workers apply them through the normal apply path (chmod ~1684). Not observed on a worker (no access from here).

## Prepared fix (principle)
publish `prepare()`: for a changed content the recorded bit is kept unless the disk bit is set (`executable = disk || (recorded && sha changed)`); same-content bit change still follows the disk (chmod -x replicates). After the journal append the publishing machine chmods the file to 0o755 so disk and manifest agree. Modeless branch unchanged in meaning. Existing wrongly-recorded False entries converge by the normal path: a `chmod +x` on any POSIX machine journals a mode-only event (done for the 3); nothing else needs a migration. Residual: a rewrite with identical bytes and 644 is indistinguishable from `chmod -x`.

## Tests
Overlay copy of the replica module (outside the worktree), `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`:
- New `🌿️git.ts` test "an executable file rewritten on a POSIX machine by an editor that creates it anew without the bit keeps the recorded bit everywhere, a bit-only change replicates both ways, and a rewrite that sets the bit wins": RED on the live replica ("machines did not converge"), GREEN with the fix.
- Files: git.ts 22 pass; 🟦️.ts 63 pass 2 fail (same 2 fail without my change: schema `#/definitions/State` ref cannot resolve on the overlay); heal 5; agree 4; race 14; store 8; boot 22 pass. Existing Windows test (keeps recorded +x, new file takes staged mode) and bootstrap archive tests unchanged and green/not rerun (bootstrap tests not run on the overlay).
- Not verified: the overlay gap fix, a worker observing the chmod events, bootstrap/provision test files, chaos/killpoints.

## Incident
For about one minute I edited the live `🔁️replica/🟦️.ts` (patch) before noticing the rule, then restored it byte-exactly to the previous live content (grep confirms no patch strings remain). The daemon may have seen a digest change in that window.
