#!/usr/bin/env bash
# usage: rss-governor-sweep.sh <label>=<ENV=value,...> ...   cold runs (a trailing marker comment in the internals component bumps the implementation revision) over disjoint 40-owner samples
T="$(cd "$(dirname "$0")/.." && pwd)"; S="$T/🧰️scripts/rss-profile.sh"; F="/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧰️internals/🟨️.mjs"; export FORCE=536870912 CAP=${CAP:-4G} RSS_TICK=3000
offset=${OFFSET:-11}
for spec in "$@"; do
  label="${spec%%=*}"; vars="${spec#*=}"; vars="${vars//,/ }"
  sed -i '/^\/\/rss-marker/d' "$F"; echo "//rss-marker-$label-$RANDOM" >> "$F"
  env RSS_OFFSET=$offset RSS_LIMIT=${LIMIT:-40} $vars "$S" "$label" nodes; offset=$((offset+2))
done
sed -i '/^\/\/rss-marker/d' "$F"
