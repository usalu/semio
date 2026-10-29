#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1029"
chk(){ n=$1; m=$2; shift 2; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$m" --keep-going "$@" > "$G/policy4-$n.txt" 2>&1; echo $? > "$G/policy4-$n.rc"; }
FWM="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/📦️packages/🦀️rust/Cargo.toml"
EM="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml"
IM="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧩️composition/📦️packages/🦀️rust/Cargo.toml"
HM="🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/🏗️projection/📦️packages/🦀️rust/Cargo.toml"
EP=""; for c in window-layer-optics-visible air-loop-nodes plant-loop-design people-gain-fractions lighting-gain-emission ideal-loads-limits year-schedule-holidays site-ground model-simulation surface-properties fenestration-geometry; do EP="$EP -p semio-s-artifact-energy-model-mut-$c"; done
chk fw "$FWM" -p semio-framework-plugin-instance-construction -p semio-framework-plugin-instance-app -p semio-framework-plugin -p semio-framework-os-kernel
chk energy "$EM" $EP -p semio-s-artifact-energy-model-native-mutations-lifecycle -p semio-s-artifact-energy-model-native-mutations-singletons -p semio-s-artifact-energy-model-editor -p semio-s-artifact-energy-model-viewer
chk hub "$HM" -p semio-hub-energy -p semio-hub-stdio-catalog-projection -p semio-hub-stdio-catalog-definition
chk ifc "$IM" -p semio-s-artifact-stdio-ifc-4
chk etests "$EM" --tests -p semio-s-artifact-energy-model-native-mutations-lifecycle -p semio-s-artifact-energy-model-native-mutations-singletons
chk itests "$IM" --tests -p semio-s-artifact-stdio-ifc-4
chk wasm "$FWM" --target wasm32-wasip2 -p semio-framework-plugin-instance-construction -p semio-framework-plugin-instance-initialization -p semio-framework-plugin
chk wasme "$EM" --target wasm32-wasip2 -p semio-s-artifact-energy-model-editor -p semio-s-artifact-energy-model-viewer
chk wasmh "$HM" --target wasm32-wasip2 -p semio-hub-energy
echo done > "$G/policy4-all.rc"
