#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1029"
chk(){ n=$1; m=$2; shift 2; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$m" --keep-going "$@" > "$G/policy3-$n.txt" 2>&1; echo $? > "$G/policy3-$n.rc"; }
FWM="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/📦️packages/🦀️rust/Cargo.toml"
EP=""; for c in window-layer-optics-visible air-loop-nodes plant-loop-design people-gain-fractions lighting-gain-emission ideal-loads-limits year-schedule-holidays site-ground model-simulation surface-properties fenestration-geometry; do EP="$EP -p semio-s-artifact-energy-model-mut-$c"; done
chk lib "$FWM" -p semio-framework-plugin-instance-construction -p semio-framework-plugin-instance-app -p semio-framework-plugin -p semio-framework-os-kernel $EP -p semio-s-artifact-energy-model-native-mutations-lifecycle -p semio-s-artifact-energy-model-native-mutations-singletons -p semio-s-artifact-energy-model-editor -p semio-s-artifact-energy-model-viewer -p semio-hub-energy -p semio-s-artifact-stdio-ifc-4 -p semio-hub-stdio-catalog-projection -p semio-hub-stdio-catalog-definition
chk tests "$FWM" --tests -p semio-s-artifact-energy-model-native-mutations-lifecycle -p semio-s-artifact-energy-model-native-mutations-singletons -p semio-s-artifact-stdio-ifc-4
chk wasm "$FWM" --target wasm32-wasip2 -p semio-framework-plugin-instance-construction -p semio-framework-plugin-instance-initialization -p semio-framework-plugin -p semio-s-artifact-energy-model-editor -p semio-s-artifact-energy-model-viewer -p semio-hub-energy
echo done > "$G/policy3-all.rc"
