#!/usr/bin/env bash
# usage: rss-force-sweep.sh <limit> <label-prefix> <force-bytes>...   one harness run per JSC heap size, sequential
T="$(cd "$(dirname "$0")/.." && pwd)"; limit="$1"; prefix="$2"; shift 2
for force in "$@"; do RSS_LIMIT="$limit" RSS_TICK=2000 FORCE="$force" CAP=4G "$T/🧰️scripts/rss-profile.sh" "$prefix-$((force/1048576))m" nodes; done
