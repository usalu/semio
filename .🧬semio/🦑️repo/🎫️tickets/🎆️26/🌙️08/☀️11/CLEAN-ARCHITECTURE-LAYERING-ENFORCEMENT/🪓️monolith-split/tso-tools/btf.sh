#!/bin/zsh
# usage: btf.sh <path-relative-to-os> -> compact list of failing tests with the failing line number and the offending needle snippet
OS=/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os
export SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/tso-artifacts
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"; export SEMIO_TICKET_DIR="$SEMIO_TEST_ARTIFACT_DIR"
export REPO_ROOT=/Users/ueli/Documents/semio; cd /Users/ueli/Documents/semio
perl -e 'use POSIX; POSIX::setsid(); alarm 300; exec @ARGV' -- nice -n 10 bun test --timeout 120000 "$OS/$1" 2>&1 | python3 -c '
import sys,re
lines=sys.stdin.read().split("\n")
last_at=None;last_exp=[]
for i,l in enumerate(lines):
    m=re.search(r"at <anonymous> \(.*?([^/]*\.ts:\d+:\d+)\)",l)
    if m: last_at=m.group(1)
    if l.startswith("Expected") or l.startswith("Received"): last_exp.append(l[:160])
    if l.startswith("error:") : last_exp=[l[:160]]
    if l.startswith("(fail)"):
        print("FAIL:",l[:110],"@",last_at,"|"," / ".join(last_exp[-3:])); last_exp=[];last_at=None
print(lines[-8:])
'
