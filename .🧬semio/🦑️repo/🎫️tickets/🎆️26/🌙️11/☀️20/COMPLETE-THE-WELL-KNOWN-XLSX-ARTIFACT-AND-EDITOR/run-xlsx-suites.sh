#!/bin/bash
cd "$(dirname "$0")/../../../../../../.." || exit 1
export SEMIO_LANGUAGE=en
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/🗑️generated/claude-suites"
X="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/Cargo.toml"
rm -f "$G/suites-done"
args=""
for p in ${PKGS:-base base-mutations strict strict-mutations transitional}; do args="$args -p semio-s-artifact-stdio-xlsx-$p"; done
for n in 1 2 3 4; do
  repo cargo --store=shared --configuration=prototype -- test --manifest-path "$X" $args --lib --no-fail-fast > "$G/suites-$n.log" 2>&1
  if grep -aqE "^test result" "$G/suites-$n.log"; then echo "done $n" > "$G/suites-done"; exit 0; fi
  sleep 120
done
echo "gave up" > "$G/suites-done"
