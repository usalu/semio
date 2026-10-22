#!/bin/sh
# usage: suites.sh ROOT [only-substring]   runs the touched suites one at a time (nice 10), only while the 1-minute load is below 45
ROOT="${1:-/private/tmp/i18n-core/t6}"
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library'
P='🧰️framework/🔨️modules/🏃️process'
cd "$ROOT" || exit 1
for f in \
  "$P/🏗️build/🗣️language/🧪️tests/🟦️.ts" \
  "$P/🏗️build/🗣️language/📖️catalog/🧪️tests/🟦️.ts" \
  "$P/🏗️build/🗣️language/🚨️error/🧪️tests/🟦️.ts" \
  "$P/🏗️build/🗣️language/📣️progress/🧪️tests/🟦️.ts" \
  "$L/⚖️laws/🗣️language-catalogs/🧪️tests/🗣️corpus/🟦️.ts" \
  "$L/🎛️commands/✅️verify/📖️texts/🧪️tests/🟦️.ts" \
  "$L/🎛️commands/📝️commit/📖️texts/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/🚀️bootstrap/📖️texts/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/💾️storage/🧾️ledger/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🧹️cache-gc/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/🧪️tests/🪟️free-space/🟦️.ts" \
  "$L/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts" \
  "$P/🪪️agent-sessions/🧪️tests/🔗️adapters/🟦️.ts" \
  "$P/🪪️agent-sessions/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🧾️ci-report/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🪪️agent-sessions/⌨️command/🧪️tests/🟦️.ts" \
  "$L/🏗️build/📥️inputs/🦀️resolution/🧪️tests/🟦️.ts"
do
  case "$f" in *"${2:-}"*) ;; *) continue;; esac
  load=$(uptime | sed 's/.*load averages*: *//' | awk '{gsub(",","",$1); print int($1)}')
  if [ "$load" -ge 45 ]; then echo "SKIP load $load: $f"; continue; fi
  printf '%s  ' "$(echo "$f" | awk -F/ '{print $(NF-2)"/"$(NF-1)}')"
  nice -n 10 bun test "./$f" 2>&1 | grep -E "^\(fail\)| pass| fail|Ran" | tr '\n' ' '
  echo
done
