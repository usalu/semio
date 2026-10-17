#!/bin/zsh
# usage: tt4-cmp.sh <repo-relative test file>; compares bun failure sets of HEAD copy and working copy
R=/Users/ueli/Documents/semio
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
f="$1"; d="$(dirname "$R/$f")"; b="$(basename "$f")"
cd "$R" || exit 1
git -c core.fsmonitor=false show "HEAD:$f" > "$d/tt4-before-tmp.ts" 2>/dev/null || { echo "no HEAD copy"; exit 0; }
cd "$d"
before=$(perl -e 'alarm 280; exec @ARGV' -- nice -n 10 bun test "./tt4-before-tmp.ts" 2>&1)
after=$(perl -e 'alarm 280; exec @ARGV' -- nice -n 10 bun test "./$b" 2>&1)
rm -f "$d/tt4-before-tmp.ts"
echo "$before" | /usr/bin/grep -E "^\(fail\)" | sed 's/ \[.*//' | sort > "$G/tt4-cmp-before.txt"
echo "$after" | /usr/bin/grep -E "^\(fail\)" | sed 's/ \[.*//' | sort > "$G/tt4-cmp-after.txt"
echo "before: $(echo "$before" | /usr/bin/grep -E '^ [0-9]+ (pass|fail)' | tr '\n' ' ') | after: $(echo "$after" | /usr/bin/grep -E '^ [0-9]+ (pass|fail)' | tr '\n' ' ')"
echo "failing only before: $(comm -23 "$G/tt4-cmp-before.txt" "$G/tt4-cmp-after.txt" | wc -l | tr -d ' ') ; only after: $(comm -13 "$G/tt4-cmp-before.txt" "$G/tt4-cmp-after.txt" | wc -l | tr -d ' ')"
