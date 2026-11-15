#!/bin/sh
# usage: validator3-c29-run.sh <slugs> <scale> <tag> [VOLUMES_DIR]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/validator3"
VOLUMES_DIR="${4:-🧊️volumes}" nice -n 5 "$BUN" validator3-c29-dump.ts "$1" "$2" "$G/c29-$3.json" || exit 1
uv run --quiet --project "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🐍️oracles" --locked --only-group geometry python -I validator3-c29-oracle.py "$G/c29-$3.json" "$G/c29-$3.out.json" || exit 1
python3 - "$G/c29-$3.out.json" <<'PY'
import json,sys
d=json.load(open(sys.argv[1]))
for a,rs in d.items():
    for r,v in rs.items():
        flag = '' if v['ratio'] is None or abs(v['ratio']-1)<0.03 else '  <<<<'
        print(f"{a:22s} {r:22s} oracle {v['oracle']:9.4f} measured {v['measured']:9.4f} ratio {v['ratio'] if v['ratio'] is None else round(v['ratio'],4)} areaM2 {v['areaM2']}{flag}")
PY
