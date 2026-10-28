#!/bin/sh
# 🔁 Rebuilds every staged batch from the CURRENT live files (rules are deterministic): sh rebuild_all.sh [root]   (default: the checkout above the ticket)
set -eu
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
python3 -I "$here/stage_all.py" "$root" "$here/staged"
python3 -I "$here/law_edits.py" "$here"/staged/*
