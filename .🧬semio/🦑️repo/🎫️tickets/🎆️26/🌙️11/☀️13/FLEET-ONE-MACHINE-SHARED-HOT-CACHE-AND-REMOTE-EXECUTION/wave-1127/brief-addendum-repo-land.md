# Addendum: `repo land` (H74): changesets land only once they hold

Replaces the manual landing scripts for build, caching, process and manifest areas. Same rules as before (slots :00-:10, per-area locks, bilingual output, no git writes); the command enforces them.

## Usage
```
S=<scratch>/stage-<lane>
repo land --lane H99 --title "Short title" checkout $S path/to/live/file.ts other.json   # copies live files to $S/tree, remembers their hashes (bases)
# edit $S/tree/<path>; create new files in $S/tree/<path>
repo land delete $S path/to/obsolete.ts                                                   # stage a deletion
repo land check $S                                                                        # every gate, applies nothing (exit 0 / 1)
repo land --wave "Wave 1127" $S                                                           # check, wait for the slot, lock, apply atomically, rollback point, landings.md entry
repo land --outage $S        # fleet-wide outage fix only: ignore the slot
repo land --no-wait $S       # outside the slot: queue (landing.slot-queue) and exit 3
repo land --rebase $S        # a stale file whose live version changed is 3-way merged (git merge-file); conflicts refuse
repo land --bench $S         # also run `repo bench latency --strict`
repo land rollback <id>      # undo a landing while none of its files changed since
repo land status             # slot state and held locks
```
MCP tool `land` takes the same words as `words` plus `dry_run outage rebase no_wait bench lane title wave`. Exit codes: 0 landed/checked/rolled back, 1 a gate failed, 2 refused (stale base, locks busy or stale, usage, cancelled), 3 queued.

## What it does, in order
1. Stale bases: every staged file's base hash is compared with the live file (changed = refused, or merged with `--rebase`).
2. Overlay: a copy-on-write clone of `🧰️framework` (plus the top folder of every staged path and the root files) in the machine cache, with the staged files applied. The live tree is only read.
3. Gates against the overlay: temporary debug lines (only in the changeset's own files), dangling imports (also importers of deleted files), `repo typecheck` of written scripts and importers, `repo cargo check -p` of touched crates, the suite route law when a suite table or project target is touched, the declared tests of every touched member (offload to workers is switched off: they would run the live copy).
4. Slot: slot-bound areas wait for minute 0-10 (progress every minute, Ctrl-C cancels); a landing that began in the slot may finish within 15 min. Gates run again after a wait.
5. Locks `landing.lock.<area>` for every touched area in alphabetical order, held only for the apply (wait 3 min, queue entry `landing.queue.<area>`, a lock older than 20 min is reported, never stolen).
6. Atomic apply: all new contents are written and fsynced to hidden temp files first; then renames (new files first, imported before importer, deletions last) while `landing.generation.<area>` is odd. Readers needing one consistent view read the generation before and after and retry unless equal and even. A failing rename restores every replaced file.
7. Rollback point in the machine cache and an append-only `landings.md` entry (`## <wave> <lane>: <title>`).

Agents no longer need `dangling-imports.py` or a manual `repo typecheck` before releasing a lock: the gate did it, on the overlay, before the lock was taken.
