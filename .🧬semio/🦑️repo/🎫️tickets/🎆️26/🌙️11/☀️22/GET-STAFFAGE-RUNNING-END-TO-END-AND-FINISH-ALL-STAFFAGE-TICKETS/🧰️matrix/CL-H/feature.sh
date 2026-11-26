#!/bin/zsh
# 🔬️ The visible feature width of one shape of a document per pose and view (`../MH-3/probe-feature.ts`), inside the grant of its caller: feature.sh <id|file.json> <scale> <shape id>
source "${0:A:h}/env.sh"
cd "$T" && timeout 900 "$B" "$U/🧰️matrix/MH-3/probe-feature.ts" "$@"
