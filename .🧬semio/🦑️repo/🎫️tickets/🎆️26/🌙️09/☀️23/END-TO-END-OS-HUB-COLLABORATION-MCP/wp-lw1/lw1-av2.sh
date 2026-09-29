#!/bin/zsh
# ⚖️ LW1 → AV2 laws (T3 av2 video render): av2-land.sh's test block, the TS laws (incl. the FFmpeg decode oracle) and tsc-live.
R=/Users/ueli/Documents/semio; W=$R/.tmp-ticket/wp-av2
cargo test --offline --no-fail-fast -p semio-framework --lib -- video_render; echo "LW1-STEP kernel-video_render rc=$?"
cargo test --offline --no-fail-fast -p semio-framework-raster --lib -- video; echo "LW1-STEP raster-video rc=$?"
cargo test --offline --no-fail-fast -p semio-framework-plugin --lib -- wire_effect_round_trip; echo "LW1-STEP plugin-wire rc=$?"
cargo test --offline --no-fail-fast -p semio-s-artifact-animate-presentation --lib -- export_video program_unit a_deck a_tileless a_stated every_command retained; echo "LW1-STEP animate rc=$?"
bun "$W/av2-laws.ts" "$R"; echo "LW1-STEP av2-ts-laws rc=$?"
cd "$W/tsc" && "$R/node_modules/.bin/tsc" -p tsconfig-live.json --pretty false; echo "LW1-STEP tsc-live rc=$?"
