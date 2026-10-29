#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1029"
FWM="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/📦️packages/🦀️rust/Cargo.toml"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$FWM" --keep-going --target wasm32-wasip2 -p semio-framework-plugin-instance-construction -p semio-framework-plugin-instance-initialization -p semio-framework-plugin -p semio-framework-os-kernel -p semio-framework-plugin-job -p semio-framework-plugin-time-travel -p semio-framework-plugin-instance-stores -p semio-framework-plugin-instance-history -p semio-framework-plugin-retained-command > "$G/policy5-wasm.txt" 2>&1; echo $? > "$G/policy5-wasm.rc"
