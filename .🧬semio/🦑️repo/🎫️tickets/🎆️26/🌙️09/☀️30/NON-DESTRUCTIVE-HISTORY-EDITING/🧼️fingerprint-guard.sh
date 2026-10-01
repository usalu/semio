#!/bin/zsh
# 🧼️ Every 10 min: deletes cargo dep-info files of the shared build-dir that name sources inside scratch clones
# (`🗑️generated/`, scratchpads, /private/tmp) so a clone built with the shared build-dir can never mark units Fresh
# against the real tree (false greens/reds); cargo rebuilds exactly those units next time.
build="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build"
log="${0:A:h}/🗑️generated/coord/fingerprint-guard.txt"
mkdir -p "${log:h}"
while true; do
  hits=$(cd "$build" && find debug release wasm-dev wasm-release wasm32-wasip2 wasm32-unknown-unknown -name 'dep-*' -type f -print0 2>/dev/null | xargs -0 /usr/bin/grep -l -a -E '🗑️generated/|/scratchpad/|/private/tmp/' 2>/dev/null)
  if [ -n "$hits" ]; then
    n=$(print -r -- "$hits" | wc -l | tr -d ' ')
    (cd "$build" && print -r -- "$hits" | while read -r f; do rm -f -- "$f"; done)
    echo "$(date '+%F %T') removed $n clone-poisoned dep-info files: $(print -r -- "$hits" | awk -F/ '{print $3}' | sort -u | tr '\n' ' ')" >> "$log"
  fi
  sleep 600
done
