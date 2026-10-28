#!/bin/sh
# 🔄️ Rebuilds every compliance batch from the CURRENT live files (the gate lands every ~30 min: run this right before apply): sh rebuild_all.sh
set -e
here=$(cd "$(dirname "$0")" && pwd)
cd "$here"
python3 -I lang_spec.py
python3 -I names_spec.py | head -1
python3 -I dedupe_batches.py
for b in fwprocess fwmodules libother libtools libbuild libgraph lang names; do python3 -I make_batch.py "$b" | tail -1 | cut -c1-60; done
