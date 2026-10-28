#!/bin/sh
# usage: rebuild.sh [root]   -- rebuilds the eight staged batches of lane progress from the CURRENT live files
# land order: umlaut, lang, status, rows, sigint, usage, phases, say  (each later batch is based on the files the earlier ones change; `drifted` below means "an earlier batch is not landed yet")
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"; ROOT="${1:-$(cd "$HERE/../../../../../../../.." && pwd)}"
for b in umlaut lang status rows sigint usage phases say; do python3 -I "$HERE/build_$b.py" "$ROOT"; done
for b in umlaut lang status rows sigint usage phases say; do echo "== $b"; python3 -I "$HERE/staged/$b/stage.py" check --root "$ROOT" | awk '{print $1}' | sort | uniq -c; done
