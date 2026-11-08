#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
"$T/runtime/tst.sh" reviewfix2-101 "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/📦️packages/🦀️rust/Cargo.toml" native -p semio-s-fem-analyses-model
"$T/runtime/tst.sh" reviewfix2-102 "🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust/Cargo.toml" native -p semio-ui-locale
"$T/chk.sh" reviewfix2-103 "🧰️framework/🔨️modules/🗺️surface/🎨️paint/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-surface-paint
"$T/chk.sh" reviewfix2-104 "🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🕸️mesh/📦️packages/🦀️rust/Cargo.toml" native -p semio-ui-scene-mesh
echo done > "$C/🗑️generated/1105b/reviewfix2-all.rc"
