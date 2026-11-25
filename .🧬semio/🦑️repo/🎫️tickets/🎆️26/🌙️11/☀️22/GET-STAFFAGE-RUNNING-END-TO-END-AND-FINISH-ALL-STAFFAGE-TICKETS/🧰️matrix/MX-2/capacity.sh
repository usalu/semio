#!/bin/zsh
# 📏️ The capacity table of the decision D54 for the sampled parts, one queue request: capacity.sh writes 🗑️generated/matrix/MX-2/capacity.txt
export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN"
cd "$S/📦️packages/🟦️typescript" || exit 2
{
for row in "tour-guide 50 hand-r" "tour-guide 50 forearm-l" "tour-guide 50 foot-r" "nurse-walking 50 hand-l" "nurse-walking 50 foot-l" "nurse-walking 50 arm-l" "person-with-rollator 50 torso" "person-with-rollator 200 torso" "person-with-rollator 50 hand-l" "used-concert-harp 50 soundbox"; do
  timeout 600 "$B" "$W/capacity.ts" ${=row} ${CELL:+--cell $CELL} 2>&1 | tail -14; echo
done
} > "$GEN/capacity.txt"
