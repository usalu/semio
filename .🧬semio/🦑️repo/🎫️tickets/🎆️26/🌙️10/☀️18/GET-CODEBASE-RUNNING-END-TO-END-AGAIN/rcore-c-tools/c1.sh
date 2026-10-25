#!/bin/sh
# usage: c1.sh LOG   (one JSON `check --tests` over every crate of red-crates.txt that is not store-owned; dedup of the wrapper retry loop in rcore-b-tools/run.sh)
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rcore-c-tools"
LOG=$1
set --
for n in $(cat "$D/red-crates.txt"); do set -- "$@" -p "$n"; done
exec sh "$D/../rcore-b-tools/run.sh" "$LOG" check --manifest-path Cargo.toml --tests --keep-going --message-format=json "$@"
