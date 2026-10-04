#!/usr/bin/env bash
# Final coordinator gates for the quiz ticket: runs every gate in sequence and records exit codes.
cd /c/git/semio || exit 1
G="$(pwd)/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/🗑️generated/coordinator"
mkdir -p "$G"
S="$G/summary.txt"
: > "$S"
export RUSTC_WRAPPER="" NX_PLUGIN_NO_TIMEOUTS=true
step() {
  local name="$1"; shift
  local start=$(date +%s)
  "$@" > "$G/$name.log" 2>&1
  local code=$?
  echo "$name exit=$code seconds=$(( $(date +%s) - start ))" >> "$S"
}
step tests timeout 3600 bun nx run-many -t test -p @semio-tech/quiz @semio-tech/quiz-react @semio-tech/quiz-rs @teaching/proctor @teaching/architecture-quiz @semio-tech/framework-server @semio-tech/framework-server-rs --skip-nx-cache --parallel=1
step typecheck-react timeout 1800 bun nx run @semio-tech/quiz-react:typecheck --skip-nx-cache
step typecheck-site timeout 1800 bun nx run @teaching/architecture-quiz:typecheck --skip-nx-cache
( cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" && SEMIO_TEST_BUDGET_MS=900000 step parity timeout 2400 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz" )
step taxonomy-quiz timeout 900 bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"
step taxonomy-teaching timeout 900 bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"
step taxonomy-server timeout 900 bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🖥️server"
step e2e timeout 3600 bun nx run @teaching/architecture-quiz:test-e2e
step deploy-check timeout 5400 bun nx run @teaching/architecture-quiz:deploy-check
step capacity timeout 2400 bun nx run @teaching/proctor:capacity
echo "done" >> "$S"
