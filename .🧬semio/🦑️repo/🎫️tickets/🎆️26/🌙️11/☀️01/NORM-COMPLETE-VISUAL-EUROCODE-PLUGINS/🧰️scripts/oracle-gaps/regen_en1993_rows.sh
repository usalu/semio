#!/bin/bash
# 🔁 Rebuilds the `*-v30-NN` rows of the EN 1993 check-outcome feature from the committed oracle (fixtures: showcase, families-fail).
cd "$(dirname "$0")"
PY=/home/deck/semio/.venv/bin/python
FEATURE=$(ls /home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🚦️check-outcome-en1993-1/🥒️.feature)
grep -v -- '-v30-' "$FEATURE" > "$FEATURE.tmp" && mv "$FEATURE.tmp" "$FEATURE"
$PY -I fixtures_en1993.py
S="shared://🎪️showcase/🔣️.json"; F="shared://📄️oracle-documents/families-fail/🔣️.json"
$PY -I spec_from_oracle.py en1993 spec_main.tsv en1993.1-5.,en1993.1-7.,en1993.1-9.,en1993.1-3.cf,en1993.1-1.5.2.horne,en1993.1-1.5.2.alpha,en1993.1-1.5.3,en1993.6.2.7,en1993.1-1.6.3,en1993.1-11,en1993.1-8.3.6.t,en1993.1-8.3.6.i,en1993.1-8.3.6.rod,en1993.1-8.3.13,en1993.1-8.7.4 "$S" "$F"
grep "1-5.9.2" spec_main.tsv > spec_tee.tsv; grep -v "1-5.9.2" spec_main.tsv > spec_main2.tsv
$PY -I spec_from_oracle.py en1993 spec_fire.tsv en1993.1-2. "$S" "$F"
$PY -I rows.py en1993 spec_main2.tsv
TOL=2e-2 $PY -I rows.py en1993 spec_fire.tsv
TOL=5e-2 $PY -I rows.py en1993 spec_tee.tsv
