#!/bin/sh
# 🧩️ Builds the browser modules (compile, describe, transpile, publish) of one flow extension component crate.
cd "$(dirname "$0")/../../../../../../.." || exit 1
W="$(pwd)"
G="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated"
export SEMIO_LANGUAGE=en
n="${1:-math}"
repo run "semio-s-plugin-flow-$n-component:component" -c prototype --exclude-dependencies > "$G/ext-browser-$n.log" 2>&1
echo "exit $?" >> "$G/ext-browser-$n.log"
