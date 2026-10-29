#!/bin/bash
# usage: test.sh <n> <crate> -- <test args>
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; CRATE=$2; shift 2
M='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📦️packages/🦀️rust/Cargo.toml'
wait_space(){ while [ "$(df -k '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m' | tail -1 | awk '{print $4}')" -lt 6000000 ]; do sleep 60; done; }
for attempt in 1 2 3 4; do
  wait_space
  rm -f "$C/🗑️generated/1029/semiodsl-chk$N.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p "$CRATE" --lib --no-fail-fast "$@" > "$C/🗑️generated/1029/semiodsl-chk$N.txt" 2>&1
  echo $? > "$C/🗑️generated/1029/semiodsl-chk$N.rc"
  grep -qE 'No space left on device|ENOSPC|compiler image (ran out|is still full)' "$C/🗑️generated/1029/semiodsl-chk$N.txt" || break
  sleep 300
done
