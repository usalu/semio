#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1029"
chk(){ n=$1; m=$2; shift 2; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$m" --keep-going "$@" > "$G/policy2-$n.txt" 2>&1; echo $? > "$G/policy2-$n.rc"; }
FWM="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/📦️packages/🦀️rust/Cargo.toml"
EM="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml"
FWP=""; for c in value-clone-source value-native-owned os-store-one-item os-store-one-item-batch os-store-replay-operation os-store-engine os-store-engine-command os-store-engine-replay os-store-initialization-replay os-store-read os-kernel plugin-job-receiving plugin-job plugin-instance-stores plugin-instance-history plugin-retained-command plugin-time-travel plugin-ownership plugin-instance-construction plugin-instance-initialization plugin-instance-app plugin; do FWP="$FWP -p semio-framework-$c"; done
chk fw "$FWM" --all-targets $FWP
EP=""; for c in window-layer-optics-visible air-loop-nodes plant-loop-design people-gain-fractions lighting-gain-emission ideal-loads-limits year-schedule-holidays site-ground model-simulation surface-properties fenestration-geometry; do EP="$EP -p semio-s-artifact-energy-model-mut-$c"; done
chk energy "$EM" --all-targets $EP -p semio-s-artifact-energy-model-native-mutations-lifecycle -p semio-s-artifact-energy-model-native-mutations-singletons -p semio-s-artifact-energy-model-editor -p semio-s-artifact-energy-model-viewer
chk ifc "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧩️composition/📦️packages/🦀️rust/Cargo.toml" --all-targets -p semio-s-artifact-stdio-ifc-4 -p semio-s-artifact-stdio-ifc-4x3 -p semio-s-artifact-stdio-ifc-catalog
chk hub "🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/🏗️projection/📦️packages/🦀️rust/Cargo.toml" --all-targets -p semio-hub-stdio-catalog-projection -p semio-hub-stdio-catalog-definition -p semio-hub-energy
chk wasm "$FWM" --target wasm32-wasip2 -p semio-framework-value-clone-source -p semio-framework-value-native-owned -p semio-framework-os-kernel -p semio-framework-plugin-job -p semio-framework-plugin-instance-stores -p semio-framework-plugin-instance-history -p semio-framework-plugin-retained-command -p semio-framework-plugin-time-travel -p semio-framework-plugin-instance-construction -p semio-framework-plugin-instance-initialization -p semio-framework-plugin -p semio-s-artifact-energy-model-editor -p semio-s-artifact-energy-model-viewer -p semio-hub-energy
echo done > "$G/policy2-all.rc"
