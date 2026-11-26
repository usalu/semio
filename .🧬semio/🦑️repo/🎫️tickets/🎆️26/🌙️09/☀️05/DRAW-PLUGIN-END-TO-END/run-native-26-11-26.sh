#!/bin/sh
# 🧪️ Runs each Draw native crate in turn through the capacity gate, retrying warden cancellations up to 5 times; "check" runs a workspace check.
cd "$(dirname "$0")/../../../../../../.."
G="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/🗑️generated"
cd "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing" || exit 1
for c in "$@"; do
  if [ "$c" = check ]; then args="check --workspace --all-targets"; else args="test --no-fail-fast -p semio-s-artifact-draw-drawing-$c"; fi
  for try in 1 2 3 4 5; do
    SEMIO_LANGUAGE=en repo capacity run --memory=4608 --cpu=1 --jobs=1 -- repo cargo --store=shared --configuration=prototype --parallelism=1 -- $args > "$G/n26-$c.txt" 2>&1
    code=$?
    echo "$c try $try exit $code $(date +%T)" >> "$G/n26.attempts"
    grep -aq "warden cancelled" "$G/n26-$c.txt" || break
    sleep 120
  done
done
echo DONE >> "$G/n26.attempts"
