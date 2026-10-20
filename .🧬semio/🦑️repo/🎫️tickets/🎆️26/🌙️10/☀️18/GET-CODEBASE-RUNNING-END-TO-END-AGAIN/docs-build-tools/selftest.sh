#!/bin/sh
# Rehearses the apply on a COPY of the README under /private/tmp/docsbuild-t (never touches the checkout).
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
T=/private/tmp/docsbuild-t
CAT="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json"
RD="🧰️framework/🛍️products/🦑️repo/README.md"
rm -rf "$T"; mkdir -p "$T/root/$(dirname "$CAT")" "$T/c"
cp "$ROOT/$RD" "$T/root/$RD"; cp "$ROOT/$CAT" "$T/root/$CAT"; cp -R "$HERE" "$T/c/docs-build-tools"; rm -rf "$T/c/docs-build-tools/backup" "$T/c/docs-build-tools/applied.json"
cd "$T/c/docs-build-tools"
echo "1. dry run:";            python3 -I apply.py --root "$T/root" | sed -n 1,2p
echo "2. write without the remote-execution operation:"; python3 -I apply.py --root "$T/root" --write | tail -1
echo "3. the operation lands:"; python3 -I - "$T/root/$CAT" <<'PY'
import sys
p=sys.argv[1]; s=open(p,encoding="utf8").read(); open(p,"w",encoding="utf8").write(s.replace('"operations": [','"operations": [\n    {"name": "remote-execution"},',1))
PY
python3 -I apply.py --root "$T/root" --write | tail -1
echo "4. rerun:";              python3 -I apply.py --root "$T/root" | tail -1
echo "5. undo:";               python3 -I apply.py --root "$T/root" --undo
cmp "$T/root/$RD" "$ROOT/$RD" && echo "restored byte-identical"
echo "6. all at once:";        python3 -I apply.py --root "$T/root" --write | sed -n 1,1p
rm -rf "$T"
