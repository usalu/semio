#!/bin/bash
T='/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END'
. "$T/a2core-env.sh"
name=$1; shift
cargo test --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -j 6 --no-fail-fast "$@" > "$T/🗑️generated/a2core-$name.log" 2>&1
echo "EXIT $?" >> "$T/🗑️generated/a2core-$name.log"
