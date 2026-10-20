# Wave 1020d brief (on top of brief-1019.md, brief-wave-1020b.md, brief-wave-1020c.md)

User (10-20 17:45, new coordinator session after an app restart killed every wave-1020c agent and chain13): get the codebase running; finish Bazel behind `bun repo` (Bazel-native caching, lazy minimal builds, incremental); the architecture course (`@teaching/architecture-course`), semio-tech play and the mit-bestand demonstrator run end to end.

## State at 17:50
- chain14 (`C/chain-tools/chain14.sh`: show projects → tools setup nx → tools setup bazel) started 17:48, log `C/🗑️generated/chain14/`. QUIET TREE until the coordinator sends "GO".
- Finished wave-1020c sets (staged, not applied): own-output, testfix-defect, hub-bzl, demo-bzl, docs-build, play-bzl, bzl-rbe, orphan-child (+ peer course sets under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-*-tools`).
- Unfinished (agent died mid-work, resume from your `C/<prefix>-tools/` and the 1020c row in `coordination.md`): input-precision, bzl-incr, parity, sync-incr, mem-gov, testfix-law, cli-zt, boot-sweep, pg-chain.

## Rules (unchanged from 1020c, STRICT — the machine kernel-panicked at 16:03 under load)
- Until "GO": write NOTHING in the repo outside C/ (stage under `C/<prefix>-tools/staged/`, anchored apply: content-hash check, refuse on drift, `--dry-run` default, all-or-nothing). No heavy processes: no `bun repo …`, no bazel/cargo over the real repo, no `find`/`grep -r` over the whole repo, `.cache` or `/private/tmp`; overlay copies under `/private/tmp/<prefix>/`, ≤ 1 test process at a time, ≤ 2 GB RSS.
- After "GO": re-check drift, apply your set yourself, verify live, measure.
- Never kill processes you did not start. Never edit `.cache/b/*/image.json` or journals. Never hardlink repo sources. No git modifying commands. No sub-agents. Foreground commands only (background children die with your turn).
- Never delete or sweep `C/🗑️generated/` (only your own `C/🗑️generated/<prefix>/`).
- Report `C/result-<prefix>.md` (update the existing one if you resume); final message ≤ 8 lines; do not close the ticket.
