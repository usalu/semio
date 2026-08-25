#!/bin/zsh
D="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w15-audit"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
: > "$D/14-nonstdio-subject.txt"
for o in 🌀️procedural 🌍️gis 🎪️demonstrator 🏭️process 📐️cad 🧩️puzzle 🪵️sourcing; do
  echo "=== $o ===" >> "$D/14-nonstdio-subject.txt"
  bun ./📜️script.ts subject exhaustive --owner "$o" >> "$D/14-nonstdio-subject.txt" 2>&1
  echo "EXIT=$?" >> "$D/14-nonstdio-subject.txt"
done
echo NSDONE > "$D/NSDONE.txt"
bun ./📜️script.ts subject exhaustive --owner 🗄️stdio > "$D/03-subject-stdio.txt" 2>&1
echo "EXIT=$?" >> "$D/03-subject-stdio.txt"
echo POSTDONE > "$D/POSTDONE.txt"
