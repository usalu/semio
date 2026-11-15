#!/bin/zsh
# usage: determinism.sh <package> <crate_name_underscored> <out dir>   offload the unit, save the worker's artifacts, compile it locally, compare bytes
PKG="$1"; CR="$2"; OUT="$3"; cd /Users/Shared/semio || exit 2
unset SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_TOKEN SEMIO_REMOTE_CACHE_ACCESS SEMIO_REMOTE_CACHE_INSTANCE SEMIO_REMOTE_EXECUTION_ENDPOINT
mkdir -p "$OUT/remote" "$OUT/local"
repo cargo --offload=always --store=shared --configuration=prototype -- build -p "$PKG" > "$OUT/remote.txt" 2>&1
LANE=$(ls -dt .cache/b/r/m/u/l/p* | head -1)
python3 - "$LANE" "$OUT" <<'PY'
import json,sys
lane=sys.argv[1].split('/')[-1]; ran=[json.loads(l) for l in open('/Users/Shared/semio/.cache/b/r/m/u/x/offload.jsonl') if '"offload-run"' in l]
print('offload-run in lane:',[r['origin'][-40:] for r in ran if r.get('lane')==lane])
PY
for f in $(find "$LANE/prototype/build/$PKG" -name "lib${CR}-*.r*" -o -name "lib${CR}-*.d" 2>/dev/null); do cp -p "$f" "$OUT/remote/"; done
python3 /Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/I/wipe-local-units.py /Users/Shared/semio/.cache/b/r/m/u "$(date -v-20M '+%Y-%m-%d %H:%M')" "$CR" | tail -1
rm -rf "$LANE/prototype/build/$PKG"; find "$LANE" -maxdepth 6 -path "*fingerprint*" -name "$PKG*" -exec rm -rf {} + 2>/dev/null
SEMIO_UNIT_CACHE_REMOTE=off repo cargo --offload=never --store=shared --configuration=prototype -- build -p "$PKG" > "$OUT/local.txt" 2>&1
for f in $(find "$LANE/prototype/build/$PKG" -name "lib${CR}-*.r*" 2>/dev/null); do cp -p "$f" "$OUT/local/"; done
for f in "$OUT"/remote/*; do b=$(basename $f); if cmp -s "$f" "$OUT/local/$b"; then echo "IDENTICAL $b"; else echo "DIFFERENT $b $(stat -f %z $f) $(stat -f %z $OUT/local/$b 2>/dev/null)"; fi; done
