#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
R="$C/ts-health-tools/run1.sh"
suffix="$1"
zsh $R "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json" repo-lib-$suffix
zsh $R "🧰️framework/🛍️products/💻️os/tsconfig.json" os-$suffix
zsh $R "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/tsconfig.json" pets-$suffix
zsh $R "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" pets-react-$suffix
zsh $R "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" ui-react-$suffix
zsh $R "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" renderer-react-$suffix
zsh $R "🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" course-react-$suffix
zsh $R "✏️s/tsconfig.json" s-$suffix
