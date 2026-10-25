#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d"
list="$1"; shift
set -- "$@"
packages=""
while IFS="$(printf '\t')" read -r name path; do [ -n "$name" ] && packages="$packages -p $name"; done < "$D/$list"
exec nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check "$@" --tests --keep-going $packages
