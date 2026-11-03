#!/bin/sh
# Dry-runs the Python oracle role of the touched catalogue features (no cargo, no platform host). Output: 🗑️generated/demo-values/dryrun.txt
cd "$(dirname "$0")/../.."
OUT=🗑️generated/demo-values/dryrun.txt
: > $OUT
for c in "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏋️en1991-catalogue" "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️en1996-catalogue" "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️catalogue-en1995-1" "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🎯️oracle-comparison-en1992-1" "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🎯️oracle-comparison-en1997-1" ; do
  echo "== $c" >> $OUT
  /home/deck/semio/.venv/bin/python -I 🧰️scripts/wave2-features-dryrun.py "$c" >> $OUT 2>&1
done
echo DONE >> $OUT
