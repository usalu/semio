#!/usr/bin/env bash
# Rerun of the coordinator gates that were not conclusive: unit tests, the end-to-end gate without the specs of the
# QUIZ-PETS ticket (still in progress in another session), and the capacity gate twice.
cd /c/git/semio || exit 1
G="$(pwd)/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/🗑️generated/coordinator"
mkdir -p "$G"
S="$G/summary-rerun.txt"
: > "$S"
export RUSTC_WRAPPER="" NX_PLUGIN_NO_TIMEOUTS=true
step() {
  local name="$1"; shift
  local start=$(date +%s)
  "$@" > "$G/$name.log" 2>&1
  local code=$?
  echo "$name exit=$code seconds=$(( $(date +%s) - start ))" >> "$S"
}
step tests2 timeout 3600 bun nx run-many -t test -p @semio-tech/quiz @semio-tech/quiz-react @semio-tech/quiz-rs @teaching/proctor @teaching/architecture-quiz @semio-tech/framework-server @semio-tech/framework-server-rs --skip-nx-cache --parallel=1
step e2e2 timeout 3600 bun nx run @teaching/architecture-quiz:test-e2e -- --grep-invert pet-walk
step capacity2 timeout 2400 bun nx run @teaching/proctor:capacity
step capacity3 timeout 2400 bun nx run @teaching/proctor:capacity
echo "done" >> "$S"
