#!/bin/bash
# usage: vegetation-fine.sh <slug> <eps> <scale>/<view> ...   re-traces the given views with a finer simplification tolerance (all bones, all clips)
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/vegetation-work"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
slug="$1"; eps="$2"; hf="${HF:-0.1}"; shift 2
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
scales=$(for sv in "$@"; do echo "${sv%/*}"; done | sort -u | tr '\n' ' ')
views=$(IFS=,; echo "$*")
EPS=$eps $B "$HERE/vegetation-trace.ts" "$slug" "$G/trace-$slug-fine.json" $scales | tail -1
forces=""; for sv in "$@"; do forces="$forces --force=$sv"; done
python3 "$HERE/vegetation-apply.py" "$slug" "$G/trace-$slug-fine.json" 0.0 $hf --views=$views $forces | tail -2
for i in 1 2 3 4; do
  $B "$HERE/vegetation-refine.ts" "$slug" "$G/edits-$slug.json" "$G/corr-$slug.json" | tail -1
  if [ "$(python3 -c "import json;print(len(json.load(open('$G/corr-$slug.json'))))")" = "0" ]; then break; fi
  python3 "$HERE/vegetation-apply.py" "$slug" --corrections "$G/corr-$slug.json" | tail -1
done
