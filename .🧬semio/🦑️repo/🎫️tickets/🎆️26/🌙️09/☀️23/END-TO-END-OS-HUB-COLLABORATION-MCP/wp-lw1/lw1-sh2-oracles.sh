#!/bin/zsh
# ⚖️ LW1 → SH2 runbook step 4 (source half): surface-schema drift check + space TS oracles (non-native checks of the space 📜️script.ts).
R=/Users/ueli/Documents/semio
cd "$R" && bun "./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts" surface-schema --plugin space --check; echo "LW1-STEP surface-schema-space rc=$?"
cd "$R/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust" || exit 2
for c in home-directory-identity-rows-check plugin-identity-check interactive-job-catalog-check home-directory-event-page-owner-check home-directory-projection-persistence-check persistence-data-class-check; do
  bun ./📜️script.ts $c; echo "LW1-STEP $c rc=$?"
done
