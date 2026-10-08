#!/usr/bin/env bash
# usage: [V1_TARGET=journeys|load] r2-v1-jt.sh <log name> <test exe args...>
# Builds the journeys crate test, runs the executable itself (outside cargo's job object) outside the harness job.
here="$(cd "$(dirname "$0")" && pwd)"
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard"
crate="$(cygpath -w "/c/git/semio/$D/🧪️tests/🧭️journeys/📦️packages/🦀️rust")"
cache="$(cygpath -w /c/git/semio/.🧬semio/🦑️repo/⚡️cache/cargo)"
name="$1"; shift
args=""
for a in "$@"; do args="$args '$a'"; done
list="$(echo "$args" | sed "s/' '/','/g; s/^ *//")"
OOB_WAIT=${OOB_WAIT:-60} "$here/r2-v1-oob.sh" "$name" /c/git/semio "& '$(cygpath -w "$here/r2-v1-jt.ps1")' -Crate '$crate' -Binary '$cache\\target-fleet-v1\\debug\\semio.exe' -TargetDir '$cache\\target-fleet-v1-journeys' -Target '${V1_TARGET:-journeys}' -TestArgs @($list)"
