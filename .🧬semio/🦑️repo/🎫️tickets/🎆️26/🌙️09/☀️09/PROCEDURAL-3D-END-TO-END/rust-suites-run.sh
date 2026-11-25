#!/bin/zsh
# usage: rust-suites-run.sh NAME MANIFEST_REL [extra cargo test args...]
R=/Users/ueli/Documents/semio
TF="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
name=$1; man=$2; shift 2
while true; do
  free=$(memory_pressure | tail -1 | grep -oE '[0-9]+' | head -1)
  swap=$(sysctl -n vm.swapusage | sed -E 's/.*used = ([0-9.]+)M.*/\1/')
  other=$(pgrep -x rustc | wc -l | tr -d ' ')
  if [ "$free" -ge 55 ] || { [ "$free" -ge 35 ] && [ "${swap%.*}" -lt 2560 ] && [ "$other" = 0 ]; }; then
    [ "${swap%.*}" -lt 2560 ] && break
  fi
  sleep 20
done
cd "$R"
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path "$R/$man" "$@" --no-fail-fast > "$TF/🗑️generated/rust-suites/$name.log" 2>&1
echo "exit $?" >> "$TF/🗑️generated/rust-suites/$name.log"
