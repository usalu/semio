# Worker Readiness Protocol (replicated coordination between the coordinator on g1-server and one agent per worker)

This folder is in the shared worktree: what a worker writes here reaches the coordinator within seconds and the other way round. Re-read this file at the start of every round; the coordinator changes it.

`W` = this folder. `<origin>` = the `originId` in `<worktree>/.cache/f/machine.json` (e.g. `g1-m03`). Worktree: macOS `~/Documents/semio`, Linux `~/semio`, Windows `C:\semio`.

## Rules for the worker agent
- Change no file outside `W/workers/<origin>/`. No build code, no fixes: the coordinator's agents fix in one place; you observe and report. (Several workers fixing the same files last night cost hours.)
- No git command that changes anything, no pkill/killall, no sudo, never print tokens. Kill only processes you started.
- One graph command at a time on the machine (`repo run`, `repo show`, `repo cargo`, the bootstrap): they queue behind each other.
- Write numbers as measured; say "not measured" otherwise.

## Round
1. Read `W/workers/<origin>/next.md` if it exists (instructions for this machine only), else `W/ROUND.md` (instructions for all). Each has a line `round: <n>`. If you already reported round n (`W/workers/<origin>/round-<n>.md` exists), wait: check every 60 seconds for a higher round, at most 6 hours, then stop.
2. Do what the round says. Unless it says otherwise the round is: run `W/native-bootstrap.sh "$HOME/native-bootstrap-round-<n>.log"` once, in the foreground.
3. Report in `W/workers/<origin>/round-<n>.md`: machine (memory, free disk, OS), code digest of the replica daemon (`status`), start and end time, `native_rc`, every failed task with its first error line verbatim and the 15 lines before it, the `[capacity]` and `graph:` lines verbatim, peak resident size seen (sample `ps -axo rss,command | sort -nr | head -3` every 30 s), wall time per phase. Copy the last 200 lines of the log to `W/workers/<origin>/round-<n>.tail.txt`.
4. Write `W/workers/<origin>/state.json`: `{"origin":"<origin>","round":<n>,"native_rc":<code>,"ready":<true|false>,"failed":["<task>",…],"at":"<UTC time>"}`. `ready` is true only when `native_rc` is 0 and `<worktree>/.cache/f/native-bootstrap.done` exists.
5. When ready: run `repo preflight --lang en` and the replica `status`, add both to the report, and stop. Otherwise go to 1.
