#!/usr/bin/env bash
# [scratch] preview: roe-deer-view.sh NAME [preview args]  -> $O/roe-deer/NAME.png
D="$(cd "$(dirname "$0")" && pwd)"; O="$D/../../🗑️generated/animals"; n="$1"; shift
cd /home/deck/semio
repo tools exec bun -- "$D/preview.ts" --file "$O/roe-deer.json" --asset roe-deer --out "$O/roe-deer/$n.svg" "$@" 2>&1 | grep -v '^\['
rsvg-convert -z ${Z:-3} "$O/roe-deer/$n.svg" -o "$O/roe-deer/$n.png" && echo "$O/roe-deer/$n.png"
