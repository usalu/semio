#!/bin/zsh
# ⚖️ LW1 → C12 laws: T4 c12-catchup-status (kernel corpus law, worker + os root vitest, hub lease check) and T3-late c12-splice
# (ui-scene text_splice, writer lib, bun text-splice twin, `✒️mutate-writer-1` case).
R=/Users/ueli/Documents/semio
export CARGO_NET_OFFLINE=true
cargo test --offline --no-fail-fast -p semio-framework-os-kernel --features sync,ureq --lib -- execution_target_status_vocabulary_matches_the_corpus; echo "LW1-STEP kernel-status-corpus rc=$?"
cd "$R/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" && bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "👷️worker" "💻️os/🟦️.ts" "backbone-envelope-io"; echo "LW1-STEP os-worker-root-vitest rc=$?"
cd "$R/🌎️hub/📦️packages/🦀️rust" && bun ./📜️script.ts execution-target-lease-check; echo "LW1-STEP hub-lease-check rc=$?"
cd "$R" && cargo test --offline --no-fail-fast -p semio-framework-ui-scene --lib; echo "LW1-STEP ui-scene-lib rc=$?"
cargo test --offline --no-fail-fast -p semio-s-artifact-writer-writer --lib; echo "LW1-STEP writer-lib rc=$?"
cd "$R/🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/✂️text-splice" && bun test ./🟦️.test.ts; echo "LW1-STEP text-splice-bun rc=$?"
cd "$R" && bun "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts" subject exhaustive --case ✒️mutate-writer-1; echo "LW1-STEP case-mutate-writer-1 rc=$?"
