#!/bin/bash
# Builds the wfc plugin component (all five variants share it); retries because peers edit sources during admission.
ROOT="$HOME/Documents/semio"; T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️19/DEV-WFC-WORKING-FOR-ALL-ARTIFACT-EDITORS"
export SEMIO_BUILD_MODE=prototype SEMIO_PLUGIN_CATALOG="$ROOT/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"
for n in 1 2 3 4; do
  "$T/native-step.sh" component . "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts" component --manifest "🌎️hub/🧩️compositions/🀄️wfc/🧩️component/📦️packages/🦀️rust/Cargo.toml"
  if tail -1 "$T/🗑️generated/component.log" | grep -q '^rc=0'; then echo "component-ok" >> "$T/🗑️generated/component.status"; exit 0; fi
  cp "$T/🗑️generated/component.log" "$T/🗑️generated/component-attempt$n.log"
  tail -1 "$T/🗑️generated/component.log" | grep -q '^rc=75' || break
done
echo "component-failed" >> "$T/🗑️generated/component.status"
