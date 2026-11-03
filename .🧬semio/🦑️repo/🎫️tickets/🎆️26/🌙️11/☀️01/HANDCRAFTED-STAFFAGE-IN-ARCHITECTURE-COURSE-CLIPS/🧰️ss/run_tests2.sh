#!/usr/bin/env bash
T=/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS
O=$T/🗑️generated/ss
rm -f $O/tests2.txt
for m in 🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.👥️internal-gains.🧪️tests.🐍️ 🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️ 🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🧪️tests.🐍️ 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️ 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️scenes 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️sections 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️section_consumers 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️icons 🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.⚖️regulations.🧪️tests.☀️sun.🐍️; do
  echo "== $m" >> $O/tests2.txt
  bash $T/m9_tests.sh $m >> $O/tests2.txt 2>&1
done
echo DONE >> $O/tests2.txt
