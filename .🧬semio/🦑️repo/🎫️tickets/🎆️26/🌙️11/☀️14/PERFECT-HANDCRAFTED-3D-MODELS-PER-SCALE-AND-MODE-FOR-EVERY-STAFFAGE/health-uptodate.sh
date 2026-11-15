#!/bin/sh
# usage: health-uptodate.sh   run react test (retry on exit 75), then dry-run plan, whitespace edit, dry-run plan, revert
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
G="$T/🗑️generated/health"
F="$R/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/⏱️clock-determinism/🟦️.tsx"
export PATH="$HOME/.local/bin:$PATH"
cd "$R"
n=0
while :; do
  n=$((n+1)); rm -f "$G/ut-run.start" "$G/ut-run.end"
  "$T/health-run.sh" ut-run @teaching/staffage-react:test
  code=$(tail -1 "$G/ut-run.log" | sed 's/exit //')
  echo "attempt $n exit $code" >> "$G/ut-attempts.txt"
  [ "$code" = 0 ] && break
  [ $n -ge 4 ] && break
  sleep 120
done
[ "$code" = 0 ] || exit 0
repo run @teaching/staffage-react:test --dry-run > "$G/ut-plan-1.out" 2> "$G/ut-plan-1.log"; echo "exit $?" >> "$G/ut-plan-1.log"
cp "$F" "$G/ut-original.tsx"
printf '\n' >> "$F"
repo run @teaching/staffage-react:test --dry-run > "$G/ut-plan-2.out" 2> "$G/ut-plan-2.log"; echo "exit $?" >> "$G/ut-plan-2.log"
cp "$G/ut-original.tsx" "$F"
repo run @teaching/staffage-react:test --dry-run > "$G/ut-plan-3.out" 2> "$G/ut-plan-3.log"; echo "exit $?" >> "$G/ut-plan-3.log"
date +%T > "$G/ut-done"
