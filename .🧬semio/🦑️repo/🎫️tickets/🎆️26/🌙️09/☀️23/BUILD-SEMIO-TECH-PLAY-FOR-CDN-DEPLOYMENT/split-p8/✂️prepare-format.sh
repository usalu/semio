#!/bin/bash
# ✂️ Source-level preparation of one stdio format before its crate cut: backup, barrel dissolution, descriptor
# extraction for every owning subset, root schema constants into the first owning snapshot, extending-subset
# composers into the root registry, model/sqlite test exodus, config generation and the size/edge plan.
# usage: ✂️prepare-format.sh <artifact dir name, e.g. 📽️pptx>
set -eo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; cd "$HERE"
NAME=$1; F="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/$NAME"; A="/Users/ueli/Documents/semio/$F"; FMT=$(python3 -c "import re;print(re.sub(r'^[^a-z0-9]+','','$NAME'))")
mkdir -p "$NAME"; (cd /Users/ueli/Documents/semio && [ -f ".🧬semio/🦑️repo/⚡️cache/p8/backup/$FMT-pre.tar" ] || tar cf ".🧬semio/🦑️repo/⚡️cache/p8/backup/$FMT-pre.tar" "$F")
python3 🔎️stdio-consumers.py > /dev/null
python3 🧭️barrels.py "$F" apply | grep -E "UNRESOLVED|removed" || true
FIRST=""
for schema in "$A"/🏅️standards/*/🪆️subsets/*/🧬️schema/🦀️.rs; do
  sub=$(dirname "$(dirname "$schema")")
  [ -d "$sub/🧬️schema/📸️snapshot" ] || continue
  grep -q "_artifact_schema_descriptor()" "$schema" || continue
  mod=$(python3 - "$A/🦀️.rs" "$schema" <<'PY'
import os,re,sys
ns={}; exec(open(os.path.join(os.environ.get("HERE","."),"🕸️module-graph.py")).read().rsplit("\nmain()",1)[0],ns)
c=ns["Crate"](sys.argv[1]); print("::".join(s for s in c.files[os.path.normpath(sys.argv[2])] if s!="component"))
PY
)
  [ -f "$sub/🧬️schema/📇️descriptor/🦀️.rs" ] || python3 📇️descriptor-extract.py "$A/🦀️.rs" "$schema" "$mod" apply | head -1
  [ -z "$FIRST" ] && FIRST="$sub|$mod"
done
CONSTS=$(grep -oE "^pub const (STDIO_[A-Z0-9_]+_DOCUMENT_SCHEMA|[A-Z0-9_]+_ARTIFACT_SCHEMA_ID)\b" "$A/🦀️.rs" | awk '{print $3}' | tr '\n' ' ')
if [ -n "$CONSTS" ] && [ -n "$FIRST" ]; then
  sub=${FIRST%%|*}; mod=${FIRST##*|}
  python3 🪨️root-items.py "$F" "${sub#$A/}/🧬️schema/📸️snapshot/🦀️.rs" "$mod::snapshot" $CONSTS --apply | wc -l | sed 's/^/const writes: /'
fi
for io in "$A"/🏅️standards/*/🪆️subsets/*/🚪️io/🦀️.rs; do
  grep -q "^pub mod io_registry" "$io" && python3 🎹️subset-composers.py "$F" "${io#$A/}" --apply >/dev/null 2>&1 && echo "lifted composers from ${io#$A/}" || true
done
echo "first owning: $FIRST"
