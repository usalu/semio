#!/usr/bin/env bash
set -u
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
P="🎓️teaching.🏛️architecture.⚡️energy"
A="$P.🏗️authoring"
for module in \
  "$A.🎨️visuals.🧪️tests.🐍️integration" \
  "$A.🎨️visuals.🧪️tests.🐍️scenes" \
  "$A.🎨️visuals.🧪️tests.🐍️labels" \
  "$A.🔤️fonts.🧪️tests.📑️heading.🐍️" \
  "$P.📊️demand.🎬️clip.🔁️recap.🧪️tests.🧮️balance.🐍️" \
  "$P.📊️demand.🎬️clip.🧪️tests.🔗️chain.🐍️" \
  "$P.📊️demand.🎬️clip.🏭️primary-energy.🧪️tests.🐍️chain" \
  "$P.📊️demand.🎬️clip.🏭️primary-energy.🧪️tests.🐍️scale" \
  "$P.🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️" \
  "$P.🌬️ventilation.🎬️clip.🧪️architecture.🐍️" \
  "$P.🌬️ventilation.🎬️clip.🧪️architecture.🐍️numbers" \
  "$P.🌬️ventilation.🎬️clip.🧪️architecture.🐍️transitions" \
  "$P.🧲️physics.🎬️clip.🧲️fundamentals.🧪️tests.🐍️" \
  "$P.🔥️heating.🎬️clip.🧱️conduction.🧪️tests.🐍️" \
  "$P.❄️cooling.🎬️clip.📐️system-sizing.🧪️tests.🐍️" \
  "$A.📚️references.🧪️tests.🐍️"; do
  echo "=== $module"
  bash "$HERE/run-tests.sh" "$module" 2>&1 | tail -25
done
