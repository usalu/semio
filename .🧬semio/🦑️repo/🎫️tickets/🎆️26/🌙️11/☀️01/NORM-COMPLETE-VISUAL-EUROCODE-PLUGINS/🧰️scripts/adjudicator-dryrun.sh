#!/bin/sh
# Dry runs of the Python oracle side of the three catalogue features touched by the adjudicator. Output: 🗑️generated/adjudicator/dryrun.txt
cd "$(dirname "$0")/.."
mkdir -p 🗑️generated/adjudicator
OUT=🗑️generated/adjudicator/dryrun.txt
: > $OUT
for c in "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏋️en1991-catalogue" "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️en1996-catalogue" "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️catalogue-en1995-1"; do
  echo "== $c" >> $OUT
  /home/deck/semio/.venv/bin/python -I 🧰️scripts/wave2-features-dryrun.py "$c" >> $OUT 2>&1
done
echo DONE >> $OUT
