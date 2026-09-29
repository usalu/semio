#!/bin/zsh
# ⚖️ LW1 → T6 row 9 (P9 fail-closed): P9's overlay-t6-4 command on the live tree + the AJV twins (preview verdict + carriage).
R=/Users/ueli/Documents/semio
source "$R/.tmp-ticket/wp-lw1/lw1-budget.sh"
export RUST_MIN_STACK=268435456
cd "$R" || exit 2
step p9-laws 2100 cargo test --offline --no-fail-fast -p semio-framework-plugin -p semio-s-artifact-trinity-jack -p semio-s-artifact-puzzle-2d -p semio-s-artifact-puzzle-3d -p semio-s-artifact-puzzle-5d -p semio-s-artifact-writer-writer -p semio-s-artifact-flow-flow -p semio-s-artifact-architect-program -p semio-s-artifact-reasoning-wires -p semio-framework-os-mcp --features semio-framework-plugin/artifact-app-testing,semio-s-artifact-trinity-jack/component-app-assembly,semio-s-artifact-puzzle-2d/component-app-assembly,semio-s-artifact-puzzle-3d/component-app-assembly,semio-s-artifact-puzzle-5d/component-app-assembly --lib -- agent_lane declared_verb every_declared_flow every_declared_architect every_declared_wires an_agent_names patch_nodes the_text_gesture_verbs a_verb_whose_lane map_fault
cd "$R/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin" && step p9-ajv-twins 300 bun "./🧪️tests/🤖️agent-lane-preview/🟦️.ts"
finish
