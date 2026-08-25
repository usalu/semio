#!/bin/zsh
D="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w15-audit"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
bun ./📜️script.ts parity exhaustive --owner 🗄️stdio > "$D/04-parity-stdio.txt" 2>&1
echo "EXIT=$?" >> "$D/04-parity-stdio.txt"
bun ./📜️script.ts subject exhaustive --owner 🗄️stdio > "$D/03-subject-stdio.txt" 2>&1
echo "EXIT=$?" >> "$D/03-subject-stdio.txt"
bun ./📜️script.ts oracle exhaustive > "$D/02-oracle-repo.txt" 2>&1
echo "EXIT=$?" >> "$D/02-oracle-repo.txt"
bun ./📜️script.ts dependency > "$D/05-dependency.txt" 2>&1
echo "EXIT=$?" >> "$D/05-dependency.txt"
echo DONE > "$D/ALLDONE.txt"
