#!/usr/bin/env bash
# Gates of the teaching architecture deliverable, one after the other, exit codes recorded in a summary.
#   bash gates.sh <run name> <gate>...
# Gates: tests typecheck parity taxonomy e2e deploy-check capacity
cd /c/git/semio || exit 1
RUN="$1"; shift
G="$(pwd)/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/$RUN"
mkdir -p "$G"
S="$G/summary.txt"
export RUSTC_WRAPPER="" NX_PLUGIN_NO_TIMEOUTS=true
step() {
  local name="$1"; shift
  local start=$(date +%s)
  "$@" > "$G/$name.log" 2>&1
  local code=$?
  echo "$name exit=$code seconds=$(( $(date +%s) - start ))" >> "$S"
}
TEST_PROJECTS="@semio-tech/quiz @semio-tech/quiz-react @semio-tech/quiz-rs @semio-tech/pets @semio-tech/pets-react @semio-tech/pets-rs @teaching/proctor @teaching/architecture-quiz @semio-tech/framework-server @semio-tech/framework-server-rs"
for gate in "$@"; do
  case "$gate" in
    tests)
      for project in $TEST_PROJECTS; do
        step "test-${project##*/}" timeout 3600 bun nx run "$project:test" --skip-nx-cache
      done ;;
    typecheck)
      for project in @semio-tech/quiz-react @semio-tech/pets @semio-tech/pets-react @teaching/architecture-quiz; do
        step "typecheck-${project##*/}" timeout 1800 bun nx run "$project:typecheck" --skip-nx-cache
      done ;;
    parity)
      ( cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" && SEMIO_TEST_BUDGET_MS=900000 step parity-quiz timeout 2400 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz" )
      ( cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" && SEMIO_TEST_BUDGET_MS=900000 step parity-pets timeout 2400 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" ) ;;
    taxonomy)
      step taxonomy-quiz timeout 900 bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"
      step taxonomy-pets timeout 900 bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"
      step taxonomy-teaching timeout 900 bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"
      step taxonomy-server timeout 900 bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🖥️server" ;;
    e2e) step e2e timeout 5400 bun nx run @teaching/architecture-quiz:test-e2e ;;
    deploy-check) step deploy-check timeout 7200 bun nx run @teaching/architecture-quiz:deploy-check ;;
    capacity) step capacity timeout 2400 bun nx run @teaching/proctor:capacity ;;
  esac
done
echo "done" >> "$S"
