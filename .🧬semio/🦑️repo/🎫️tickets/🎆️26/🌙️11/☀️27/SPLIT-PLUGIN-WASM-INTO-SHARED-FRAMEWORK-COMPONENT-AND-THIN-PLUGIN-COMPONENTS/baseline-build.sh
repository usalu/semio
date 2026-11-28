#!/bin/zsh
# ⏱️ Times wasm32-wasip2 builds of a plugin component through `repo cargo` (shared store), retrying while a peer's half-landed edit breaks the shared tree; argument 1 is the label, 2 the component manifest, 3 the attempts.
T="${0:A:h}"; L=${1:-cold}; M=${2:-🌎️hub/🧩️compositions/🏭️process/🧩️component/📦️packages/🦀️rust/Cargo.toml}; N=${3:-6}
cd /Users/ueli/Documents/semio
for i in $(seq 1 $N); do
  S=$(date +%s)
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- build --manifest-path "$M" --target wasm32-wasip2 --timings > "$T/🗑️generated/build-$L-$i.log" 2>&1
  E=$?
  echo "$L attempt=$i exit=$E seconds=$(( $(date +%s) - S )) compiled=$(grep -c '^ *Compiling' "$T/🗑️generated/build-$L-$i.log")" >> "$T/🗑️generated/timings.txt"
  [ $E -eq 0 ] && exit 0
  grep -qE "^error\[E|because --locked was passed" "$T/🗑️generated/build-$L-$i.log" || exit $E
  sleep 120
done
exit 1
