#!/bin/sh
# 🧪️ Reruns crates one at a time at 2600 MB: infrastructure refusals ("Closed ... required", "is not allowed") wait 2 minutes and retry uncounted for up to 60 minutes; a warden kill retries once at 3500 MB; only real compile or test verdicts count, up to 5.
cd "$(dirname "$0")/../../../../../../.."
G="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/🗑️generated"
busy() { pgrep -f "semio-hub-draw-component" >/dev/null || pgrep -f "component --manifest" >/dev/null || pgrep -f "final-e2e-27.sh" >/dev/null || pgrep -f "component-wait-26.sh" >/dev/null; }
gate() {
  while :; do
    while busy; do sleep 60; done
    for w in $(seq 30); do [ "$(memory_pressure | awk '/free percentage/{gsub("%","",$NF);print $NF}')" -ge 45 ] && break; sleep 60; done
    sleep 20
    busy || break
  done
}
cd "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing" || exit 1
for c in "$@"; do
  if [ "$c" = check ]; then args="check --workspace --all-targets"; else args="test --no-fail-fast -p semio-s-artifact-draw-drawing-$c"; fi
  mem=2600; counted=0; waited=0
  while [ $counted -lt 5 ]; do
    gate
    SEMIO_LANGUAGE=en repo capacity run --memory=$mem --cpu=1 --jobs=1 -- repo cargo --store=shared --configuration=prototype --parallelism=1 -- $args > "$G/r26-$c.txt" 2>&1
    code=$?
    if [ "$c" = check ]; then [ $code -eq 0 ] && { echo "rerun $c mem $mem exit $code $(date +%T)" >> "$G/n26.attempts"; break; }; else if grep -aq "^test result" "$G/r26-$c.txt"; then echo "rerun $c mem $mem exit $code $(date +%T)" >> "$G/n26.attempts"; break; fi; fi
    peer=0
    if grep -aqE "could not compile .semio-(framework|plugin|ui|hub)" "$G/r26-$c.txt" && ! grep -aqE "could not compile .semio-(s-artifact|draw)" "$G/r26-$c.txt"; then peer=1; fi
    if [ $peer -eq 1 ] || { ! grep -aqE "^error(\[|:) |could not compile|warden cancelled|Wächter|No space left" "$G/r26-$c.txt" && grep -aqE "Closed .* required|is not allowed|Internal error" "$G/r26-$c.txt"; }; then
      waited=$((waited+120)); [ $waited -gt 3600 ] && { echo "rerun $c infra-wait-exhausted $(date +%T)" >> "$G/n26.attempts"; break; }
      sleep 120; continue
    fi
    if grep -aqE "warden cancelled|Wächter" "$G/r26-$c.txt"; then
      mem=3500; echo "rerun $c warden-kill uncounted $(date +%T)" >> "$G/n26.attempts"
      for w in $(seq 30); do [ "$(memory_pressure | awk '/free percentage/{gsub("%","",$NF);print $NF}')" -ge 45 ] && break; sleep 60; done
      continue
    fi
    counted=$((counted+1))
    echo "rerun $c counted $counted mem $mem exit $code $(date +%T)" >> "$G/n26.attempts"
    if grep -aqE "warden cancelled|Wächter" "$G/r26-$c.txt"; then mem=3500; sleep 60; else sleep 120; fi
  done
done
echo RERUN-DONE >> "$G/n26.attempts"
