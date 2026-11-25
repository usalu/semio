#!/bin/zsh
# 🪵️ The six suites of the brief for the leftover `glulam-beam` and the cases of the multiview suite that name it, one vitest process each: beam.sh <tag>   (run it through q.sh with 2500 MB)
source "${0:h}/env.sh"
SUITES=silhouette,three-views,physics,breathing,validation,power "$W/suites.sh" "$1" glulam-beam
(cd "$P" && timeout 3000 "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/🔭️multiview-coverage/🟦️.ts" -t "glulam") > "$GEN/vt/$1-coverage.log" 2>&1
echo "coverage exit $? $(grep -E '^ *(Test Files|Tests) ' "$GEN/vt/$1-coverage.log" | tr -s ' ' | tr '\n' ';')"
