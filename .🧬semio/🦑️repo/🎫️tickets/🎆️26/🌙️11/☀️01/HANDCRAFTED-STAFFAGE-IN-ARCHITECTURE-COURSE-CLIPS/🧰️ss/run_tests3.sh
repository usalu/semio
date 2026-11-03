#!/usr/bin/env bash
T=/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS
O=$T/🗑️generated/ss
rm -f $O/tests3.txt
for m in 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️integration 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️scenes 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️section_consumers 🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🧪️tests.🐍️ 🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.⚖️regulations.🧪️tests.☀️sun.🐍️; do
  echo "== $m" >> $O/tests3.txt
  bash $T/m9_tests.sh $m >> $O/tests3.txt 2>&1
done
echo DONE >> $O/tests3.txt
