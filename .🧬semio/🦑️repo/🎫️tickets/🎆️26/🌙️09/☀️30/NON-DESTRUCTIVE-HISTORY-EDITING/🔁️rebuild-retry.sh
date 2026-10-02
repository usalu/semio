#!/bin/zsh
# 🔁️ Runs the registered rebuild chain `--from guest-framework --to <to>` until the guest-linked framework crates compile:
# a failure inside the guest-framework gate (a fleet/peer framework crate mid-edit) waits 15 min and retries; any later
# outcome (components built, or component failures) ends the loop so the coordinator can triage. Log per attempt.
setopt no_bg_nice
to="${1:-components}"
dir="${0:A:h}/🗑️generated/act"
root="/Users/ueli/Documents/semio"
mkdir -p "$dir"
for attempt in $(seq 1 24); do
  log="$dir/rebuild-retry-$attempt.log"
  (cd "$root" && bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx run @semio-tech/plugin-registry:rebuild-all --from guest-framework --to "$to") > "$log" 2>&1
  code=$?
  echo "$(date '+%F %T') attempt=$attempt exit=$code log=${log:t}" >> "$dir/rebuild-retry.events"
  [ $code -eq 0 ] && exit 0
  if sed 's/\x1b\[[0-9;]*m//g' "$log" | /usr/bin/grep -q -E 'rebuild-all [0-9]+/[0-9]+ components .* started'; then exit $code; fi
  sleep 900
done
exit 1
