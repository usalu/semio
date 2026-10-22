#!/bin/zsh
# ✅ Re-runs every live set in place and every staged set in its overlay, one process at a time, and prints one line each.
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lawcov-build-tools
live=("patch|⚡caching/🚀bootstrap/🛠tools/🩹patch" "bun|⚡caching/🚀bootstrap/🛠tools/🟦bun" "rust|🏗build/📥inputs/🎯selected/🦀rust" "stream|⚡caching/📦bundle/🟦bun/📤stream" "recency|⚡caching/🧹pruning/⏱recency" "projection|⚡caching/🚀bootstrap/🛠tools/📦sdk/📥source/📐projection" "workspace|⚡caching/🦀cargo/📦units/🏗compiler/📍workspace" "plan|⚡caching/🚀bootstrap/🛠tools/📦sdk/📥source/🧊pristine/📋plan" "snapshot|⚡caching/🚀bootstrap/🛠tools/🔏authorizers/📦snapshot" "binding|⚡caching/📦artifacts/🟦typescript/🔗binding" "preflight|🏗build/🩺preflight" "witness|🏗build/🔧tools/🏗bazel/🚦test-classes/🧾witness" "evidence|🏗build/🔧tools/🏗bazel/👁evidence" "reporting|🏗build/📊reporting" "cli|🏗build/🚀bootstrap/⌨cli" "hostcap|🏗build/🌡host-capacity" "bazelcap|🏗build/🔧tools/🏗bazel/🌡host-capacity" "pool|⚡caching/💾storage/🔒enforcement/🧵session/🏊pool" "identity|⚡caching/🚀bootstrap/🛠tools/📦sdk/🪪identity" "nxcap|⚡caching/🚀bootstrap/🛠tools/🌡host-capacity" "nxops|⚡caching/🚀bootstrap/🛠tools/⚙operations" "nxworkers|⚡caching/🚀bootstrap/🛠tools/👷workers" "ciresolve|⚡caching/🚦ci/🧭baseline/🏃resolve" "pruneplan|⚡caching/🧹pruning" "prunescan|⚡caching/🧹pruning/🌐workspace" "handoff|⚡caching/🕸graph/🏁startup/🚦service")
staged=("runner|🏗build/🧹cache-gc/🏃runner" "gccommand|🏗build/🧹cache-gc/⌨command" "leasecli|🏗build/🔒leases/⌨command" "retry|🏗build/🕸graph/🔁retry" "genfiles|🏗build/🕸graph/🗂generation-files")
for row in $live; do
  for wait in {1..40}; do load=$(uptime | sed 's/.*averages: //' | cut -d' ' -f1 | cut -d. -f1); [ "$load" -lt 45 ] && break; sleep 15; done; [ "$load" -ge 45 ] && { echo "load $load too high, stopping"; exit 3; }
  echo -n "live   ${row%%|*}: "; ./live.sh "${row%%|*}" "${row#*|}" 6 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' '; echo
done
for row in $staged; do
  for wait in {1..40}; do load=$(uptime | sed 's/.*averages: //' | cut -d' ' -f1 | cut -d. -f1); [ "$load" -lt 45 ] && break; sleep 15; done; [ "$load" -ge 45 ] && { echo "load $load too high, stopping"; exit 3; }
  echo -n "staged ${row%%|*}: "; ./run.sh "${row%%|*}" "${row#*|}" 6 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' '; echo
done
