#!/bin/bash
# 🎟️ Runs one try of round 18 on g1-m06 with the runner and records before and after it the launcher files (inode, modification time, link target, signing identifier and CDHash) and the line counts of the graph service logs, so the report can say whether a bootstrap run touched the launcher.
# Usage: bash round-18-try.sh <try>
# @see round-18-run.sh
set -u
here=$(cd "$(dirname "$0")" && pwd)
try=$1
if [ "$try" = 1 ]; then log=$HOME/native-bootstrap-round-18.log; else log=$HOME/native-bootstrap-round-18-try-$try.log; fi
notes=$HOME/native-bootstrap-round-18.notes.txt
launcher=$HOME/.local/share/semio/launcher/bin/repo
record() {
  printf 'try %s %s %s\n' "$try" "$1" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$notes"
  ls -li "$launcher" "$HOME/.local/bin/repo" >> "$notes" 2>&1
  stat -f 'inode %i mtime %Sm %N' -t %Y-%m-%dT%H:%M:%S%z -L "$launcher" >> "$notes" 2>&1
  [ -L "$launcher" ] && printf 'readlink %s\n' "$(readlink "$launcher")" >> "$notes"
  codesign -dvvv "$launcher" 2>&1 | grep -E 'Identifier|CDHash' >> "$notes"
  for host in "$TMPDIR"semio-repo/"$(id -u)"/host-*.log; do printf 'service log %s lines %s\n' "$(wc -l < "$host" | tr -d ' ')" "$host"; done >> "$notes"
}
record before
bash "$here/round-18-run.sh" "$log"
record after
