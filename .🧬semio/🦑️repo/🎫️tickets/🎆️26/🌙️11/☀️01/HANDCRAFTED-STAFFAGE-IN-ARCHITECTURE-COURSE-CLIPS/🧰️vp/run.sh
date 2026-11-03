#!/usr/bin/env bash
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/vp"
V="🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests"
F="🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🧪️tests.📑️heading.🐍️"
for lang in en de; do
  for m in "$V.🐍️staffage_only" "$V.🐍️scenes" "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️" "$F"; do
    n=$(echo "$m" | tail -c 6 | tr -d '/')
    SEMIO_TEACHING_LANGUAGE=$lang timeout 1500 bash "$T/m9_tests.sh" "$m" > "$G/$lang-$RANDOM.log" 2>&1 < /dev/null
    echo "$lang $m rc=$? $(grep -E '^(Ran|OK|FAILED)' $(ls -t $G/$lang-*.log | head -1) | tr '\n' ' ')" >> "$G/summary.txt"
  done
done
echo done >> "$G/summary.txt"
