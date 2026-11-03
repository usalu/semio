#!/usr/bin/env bash
T=/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS
O=$T/🗑️generated/ss
for m in 🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.🧪️tests.🐍️ 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️labels 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️integration; do
  echo "== $m" >> $O/tests.txt
  bash $T/m9_tests.sh $m >> $O/tests.txt 2>&1
done
echo DONE >> $O/tests.txt
