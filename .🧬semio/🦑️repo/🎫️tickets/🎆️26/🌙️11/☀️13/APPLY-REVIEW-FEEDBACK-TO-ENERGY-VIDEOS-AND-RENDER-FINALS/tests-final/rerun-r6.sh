#!/bin/bash
# Usage: rerun-r6.sh; re-runs the modules that need their own launch setup plus the modules added in round 6, into results-r6-rerun.txt.
cd /c/git/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
E="🎓️teaching.🏛️architecture.⚡️energy"
export RES=results-r6-rerun.txt
for m in "$E.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🕰️motion.🐍️" "$E.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🖱️hover.🐍️" "$E.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🪡️axis.🐍️" "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️integration"; do
  EXTRAENV="" bash "$T/tests-final/run-one-path.sh" "$m"
done
export EXTRAENV="SEMIO_TEACHING_PROFILE=final SEMIO_TEACHING_LANGUAGE=en"
for m in "$E.🏗️authoring.🎬️clip.📁️output.🧪️tests.🐍️" "$E.❄️cooling.🎬️clip.🧪️tests.🐍️" "$E.🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️" "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️reference_chip" "$E.🔥️heating.🎬️clip.🧮️calculation.🧪️tests.🐍️" "🎓️teaching.🏛️architecture.🔀️morph.🧪️tests.🎭️stage.🐍️" "$E.🏗️authoring.🎬️clip.🪟️presentation.🖼️posters.🧪️tests.🐍️"; do
  bash "$T/tests-final/run-one-path.sh" "$m"
done
cat "$T/🗑️generated/tests-final/$RES"
