#!/bin/sh
# Rehearses make-edits.py and apply.py on a COPY of the current README under /private/tmp/docs1021a-t (never touches the checkout).
# The copy of the tools starts with the record of the first apply (history of round 1), so the round-2 flow is rehearsed as it will run.
# DOCS1020E_REHEARSE_LIVE forces features live to exercise the refresh path.
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
RD="🧰️framework/🛍️products/🦑️repo/README.md"
T=/private/tmp/docs1021a-t
rm -rf "$T"; mkdir -p "$T"
cp -R "$HERE" "$T/tools"; rm -rf "$T"/tools/rendered-* "$T/tools/applied-rehearsal.json"
cp "$T/tools/applied.json" "$T/tools/applied-rehearsal.json"
cp "$ROOT/$RD" "$T/README.md"
cd "$T/tools"
A="--root $ROOT --readme $T/README.md"
echo "1. make-edits:";            python3 -I make-edits.py $A | sed -n 1,2p | cut -c1-120
echo "2. dry run:";               python3 -I apply.py $A | sed -n 1p
echo "3. write:";                 python3 -I apply.py $A --write | tail -1
echo "4. rerun:";                 python3 -I apply.py $A | tail -1
echo "5. wave-1021a features go live: the earlier text is refreshed and the gated rows appear:"
L=launcher,runtime-seam,tool-registry,tool-spawn-law,resident,agent-sessions,remote-cache-server,bench,conformance,units,pm-seam
DOCS1020E_REHEARSE_LIVE=$L python3 -I make-edits.py $A | sed -n 1p | cut -c1-120
echo "   apply without regenerating after another state change must refuse:"
DOCS1020E_REHEARSE_LIVE=$L,ide-free python3 -I apply.py $A | tail -1 | cut -c1-120 || true
DOCS1020E_REHEARSE_LIVE=$L python3 -I apply.py $A | sed -n 1p
DOCS1020E_REHEARSE_LIVE=$L python3 -I apply.py $A --write | tail -1
DOCS1020E_REHEARSE_LIVE=$L python3 -I apply.py $A | tail -1
echo "6. drift: an anchor sentence changed by a peer must be refused:"
cp "$T/README.md" "$T/README.good"
python3 -I - "$T/README.md" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf8").read()
assert s.count("The aim is that an edit costs") == 1
open(p, "w", encoding="utf8").write(s.replace("The aim is that an edit costs", "The aim is that an edit costs (edited by a peer)"))
PY
DOCS1020E_REHEARSE_LIVE=$L python3 -I apply.py $A | tail -1 | cut -c1-120 || true
cp "$T/README.good" "$T/README.md"
echo "7. undo restores the README of before this round byte for byte:"
python3 -I apply.py $A --undo | tail -1
cmp "$T/README.md" "$ROOT/$RD" && echo "restored byte-identical"
rm -rf "$T"
