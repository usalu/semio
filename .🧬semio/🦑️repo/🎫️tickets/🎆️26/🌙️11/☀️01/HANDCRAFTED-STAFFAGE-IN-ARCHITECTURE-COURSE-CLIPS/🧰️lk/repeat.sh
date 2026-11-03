#!/usr/bin/env bash
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
M="🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️"
cd /c/git/semio
pass=0
for i in $(seq 1 20); do
  if bash "$T/m9_tests.sh" "$M.Exclusive" > "$T/🗑️generated/lk/run-$i.txt" 2>&1; then pass=$((pass+1)); else echo "run $i failed"; fi
done
echo "passed $pass/20"
