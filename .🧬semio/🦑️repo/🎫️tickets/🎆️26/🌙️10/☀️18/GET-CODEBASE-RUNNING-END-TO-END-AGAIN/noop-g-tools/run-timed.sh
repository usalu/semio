#!/bin/sh
# usage: run-timed.sh <tag> <repo args...>   (cwd = repo root); writes C/🗑️generated/noop-g/<tag>.{ts,start,end}
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/noop-g"; tag="$1"; shift
date +%s > "$G/$tag.start"
SEMIO_REPO_PROFILE=1 nice -n 10 bun repo "$@" 2>&1 | python3 "$C/noop-g-tools/ts.py" > "$G/$tag.ts"
date +%s > "$G/$tag.end"
