#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1028"
chk(){ n=$1; m=$2; shift 2; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$m" --keep-going --all-targets "$@" > "$G/policyfix-$n.txt" 2>&1; echo $? > "$G/policyfix-$n.rc"; }
EM="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml"
P=""; for c in window-layer-optics-visible air-loop-nodes plant-loop-design people-gain-fractions lighting-gain-emission ideal-loads-limits year-schedule-holidays site-ground model-simulation surface-properties fenestration-geometry; do P="$P -p semio-s-artifact-energy-model-mut-$c"; done
chk energy "$EM" $P -p semio-s-artifact-energy-model-native-mutations-lifecycle -p semio-s-artifact-energy-model-native-mutations-singletons
chk fw "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/📦️packages/🦀️rust/Cargo.toml" -p semio-framework-plugin-job-receiving -p semio-framework-os-store-engine-replay -p semio-framework-plugin-instance-construction -p semio-framework-plugin-instance-initialization -p semio-framework-os-store-engine-branch -p semio-framework-os-kernel
chk stdio "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧩️composition/📦️packages/🦀️rust/Cargo.toml" -p semio-s-artifact-stdio-ifc-catalog
chk hub "🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/🏗️projection/📦️packages/🦀️rust/Cargo.toml" -p semio-hub-stdio-catalog-projection -p semio-hub-stdio-catalog-definition
echo done > "$G/policyfix-all.rc"
