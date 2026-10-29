#!/bin/bash
D=$(cd "$(dirname "$0")"; pwd); S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"; OUT="$S/🗂️catalog/🌳️vegetation/🔣️.json"; mkdir -p "$(dirname "$OUT")"
{ printf '{\n"$schema": "../../🧬️schema/🔣️.json#/$defs/CategoryDocument",\n"version": "semio.staffage/v1",\n"category": "vegetation",\n"assets": [\n'
  first=1; for a in oak poplar birch pine palm willow hedge grass; do [ $first = 0 ] && printf ',\n'; first=0; cat "$D/assets/$a.json" | sed '$d' ; printf '}'; done
  printf '\n]\n}\n'; } > "$OUT"
python3 -c "import json,sys;d=json.load(open(sys.argv[1]));print([a['id'] for a in d['assets']])" "$OUT"
