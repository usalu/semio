#!/bin/bash
# ✂️ Cuts one semio subset into its standard crate family: descriptor extraction, config, plan audit (violations stop
# it unless FORCE=1), cut, manifests, fixture targets, consumer manifests and the consumer index refresh.
# usage: ✂️cut-subset.sh <subset dir name> [layer=dir|regex ...]
set -eo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; P8="$(dirname "$HERE")"; cd "$HERE"
SUB="$1"; shift
S="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/$SUB"
[ -f "$S/🧬️schema/📇️descriptor/🦀️.rs" ] || python3 "$P8/📇️descriptor-extract.py" "$S/🦀️.rs" "$S/🧬️schema/🦀️.rs" schema apply
SLUG=$(python3 "$HERE/🏭️subset-config.py" "$SUB" "$@" | cut -d: -f1)
python3 "$P8/🔎️stdio-consumers.py" > /dev/null
python3 "$P8/📐️layer-sizes.py" "$SLUG-cut.json" --refresh
PLAN=$(python3 "$P8/✂️crate-cut.py" "$SLUG-cut.json" plan 2>&1)
if echo "$PLAN" | grep -q "^VIOLATION"; then echo "$PLAN" | grep "^VIOLATION"; [ -z "$FORCE" ] && exit 3; fi
python3 "$P8/✂️crate-cut.py" "$SLUG-cut.json" apply | tail -2
python3 -c "import json;d=json.load(open('$P8/🗑️generated/semio_s_artifact_stdio_semio_$SLUG-layer-edges.json'));[print('  edge',k,v) for k,v in d.items()]"
python3 "$P8/🧾️crate-manifests.py" "$SLUG-cut.json" apply | grep -v "^== ✏️" || true
M="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/$SUB/🧬️schema/🧬️mutations"
[ -f "/Users/ueli/Documents/semio/$M/🧪️tests/🔬️fixture/🦀️.rs" ] && python3 "$P8/🧪️fixture-targets.py" "$M" "semio $SLUG" apply
python3 "$P8/🧾️consumer-manifests.py" "$SLUG-cut.json" "$P8/🗑️generated/$SLUG-cut.consumers.json" apply 2>&1 | grep -v "UNOWNED\|lib=\[\] dev=\[\]" || true
python3 "$P8/🔎️stdio-consumers.py" > /dev/null
echo "cut $SUB done"
