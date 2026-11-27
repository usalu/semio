# 📏 Agent Rules for Get Sourcing Plugin Working End to End

Ticket folder `$T` = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END`.
Plugin root `$P` = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing`.

- Read `/Users/ueli/Documents/semio/AGENTS.md`, `CLAUDE.md` and `$P/AGENTS.md` and obey them: no git command that changes the repo or worktree; others edit the same files live (replicated within ~1 s), so re-read a file right before editing it; no compat layers, deprecations or legacy; concise code, no comments inside definitions; docstrings start with a unique emoji; no new runtime dependencies on external libraries.
- Build/test through `repo` first (`repo cargo --store=shared --configuration=prototype -- <cargo args>` from the dir holding the Cargo.toml; `repo run <project>:<target>`; `repo test-file <file>`; `repo tools exec <tool> -- <args>`). If `repo` is stuck or broken for >10 min on one step, fall back to native `cargo`/`bun` and say so. Never wait idle. The shared work fleet is under development by others: if it fails, build locally.
- HOST: macOS, 10 cores, 16 GB RAM (tight), ~36 GB free disk, other sessions share it. Use `-j 4`, never run two of your own builds at once, build only targets you need. Native cargo fallback env: `CARGO_TARGET_DIR=/Users/ueli/t/src-<you>`, `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`. If disk < 15 GB free: delete your own target dir or stale `/Users/ueli/t/*` dirs older than 2 days.
- Run long commands in the background, output to `$T/🗑️generated/<agent>-<step>.log` (unique names; never delete others' logs).
- Temporary prints start with `[DEBUG] ` and are removed before you finish.
- Never open/close/reopen tickets or goals. Never ask the user anything; decide yourself.
- Stay in your scope; fix framework root causes when they break your scope, minimal and correct, and list them.
- Never kill by name/pattern (`pkill cargo`, `killall`): kill only PIDs you started.
- Finish with `$T/<report>.md`: what you ran, pass/fail counts, fixes (file list), what is open and why. Final chat message: 5 lines max + report path.
