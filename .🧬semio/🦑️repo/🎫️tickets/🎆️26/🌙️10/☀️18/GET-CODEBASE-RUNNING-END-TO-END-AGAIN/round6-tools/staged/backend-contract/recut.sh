#!/bin/sh
# Re-cuts this set against another tree (a combined overlay) and applies it there: adapters from that tree's script, registry, render, apply, literal ratchet from the applied tree.
# usage: sh recut.sh <root> [--undo-backup-dir]   (needs the tool-iface set applied in <root>; nothing is written outside <root> and this folder)
set -e
ROOT="$1"; [ -n "$ROOT" ] || { echo "usage: recut.sh <root>"; exit 2; }
cd "$(dirname "$0")"
SAVE=$(mktemp -d /private/tmp/bkc/recut-save-XXXXXX)
cp -R src "$SAVE/src"
restore(){ rm -rf src; cp -R "$SAVE/src" src; python3 build.py >/dev/null; rm -rf "$SAVE"; }
trap restore EXIT
export BKC_ROOT="$ROOT"
nice -n 10 python3 gen-registry.py
nice -n 10 python3 gen-backends.py
nice -n 10 python3 build.py | head -1
nice -n 10 python3 apply.py --root "$ROOT" --write --force-running
nice -n 10 python3 mkratchet.py "$ROOT" --write
nice -n 10 python3 build.py | head -1
nice -n 10 python3 - <<PY
import json,os,shutil
m=json.load(open("staged/manifest.json",encoding="utf8"))["backends_test.ts"]
shutil.copyfile("staged/backends_test.ts",os.path.join("$ROOT",m["destination"]))
print("ratchet test refreshed in $ROOT")
PY
nice -n 10 python3 regen-help.py --root "$ROOT" --write | tail -2
