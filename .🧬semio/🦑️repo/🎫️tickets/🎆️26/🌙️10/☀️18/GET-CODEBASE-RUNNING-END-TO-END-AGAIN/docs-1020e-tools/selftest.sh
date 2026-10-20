#!/bin/sh
# Rehearses make-edits.py and apply.py on a COPY of the README under /private/tmp/docs1020e-t (never touches the checkout).
# Probes still read the live tree (read only); DOCS1020E_REHEARSE_LIVE forces features live to exercise the refresh path.
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
RD="🧰️framework/🛍️products/🦑️repo/README.md"
T=/private/tmp/docs1020e-t
rm -rf "$T"; mkdir -p "$T"
cp -R "$HERE" "$T/tools"; rm -rf "$T/tools/backup" "$T/tools/applied.json" "$T/tools/applied-rehearsal.json" "$T"/tools/rendered-*
cp "$ROOT/$RD" "$T/README.md"
cd "$T/tools"
A="--root $ROOT --readme $T/README.md"
echo "1. make-edits:";            python3 -I make-edits.py $A | sed -n 1p
echo "2. dry run:";               python3 -I apply.py $A | sed -n 1p
echo "3. write:";                 python3 -I apply.py $A --write | tail -1
echo "4. rerun:";                 python3 -I apply.py $A | tail -1
echo "5. a second write must say already applied:"; python3 -I apply.py $A --write | tail -1
echo "6. features go live (leases, concurrency, cache-ledger): edits regenerate, the old state words refresh:"
DOCS1020E_REHEARSE_LIVE=leases,concurrency,cache-ledger python3 -I make-edits.py $A | sed -n 1p
echo "   apply without regenerating after another state change must refuse:"
DOCS1020E_REHEARSE_LIVE=leases,concurrency,cache-ledger,graph-policy python3 -I apply.py $A | tail -1 || true
DOCS1020E_REHEARSE_LIVE=leases,concurrency,cache-ledger python3 -I apply.py $A | sed -n 1,2p
DOCS1020E_REHEARSE_LIVE=leases,concurrency,cache-ledger python3 -I apply.py $A --write | tail -1
DOCS1020E_REHEARSE_LIVE=leases,concurrency,cache-ledger python3 -I apply.py $A | tail -1
echo "7. drift: an anchor sentence changed by a peer must be refused:"
cp "$T/README.md" "$T/README.good"
python3 -I - "$T/README.md" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf8").read()
assert s.count("The aim is that an edit costs") == 1
open(p, "w", encoding="utf8").write(s.replace("The aim is that an edit costs", "The aim is that an edit costs (edited by a peer)"))
PY
DOCS1020E_REHEARSE_LIVE=leases,concurrency,cache-ledger python3 -I apply.py $A | tail -1 || true
cp "$T/README.good" "$T/README.md"
echo "8. undo restores the original byte for byte:"
python3 -I apply.py $A --undo | tail -1
cmp "$T/README.md" "$ROOT/$RD" && echo "restored byte-identical"
rm -rf "$T"
