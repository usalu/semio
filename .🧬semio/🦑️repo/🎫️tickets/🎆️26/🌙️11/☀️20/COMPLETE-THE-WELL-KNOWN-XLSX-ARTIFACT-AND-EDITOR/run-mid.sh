#!/bin/bash
cd "$(dirname "$0")/../../../../../../.." || exit 1
export SEMIO_LANGUAGE=en
T="$(cd "$(dirname "$0")" 2>/dev/null; pwd)"
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/🗑️generated/claude-suites"
X="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/Cargo.toml"
mkdir -p "$G"
rm -f "$G/mid-done"
for n in 1 2 3 4 5 6; do
  repo cargo --store=shared --configuration=prototype -- test --manifest-path "$X" -p semio-s-artifact-stdio-xlsx-strict --lib closes_mid_reserved_commit --no-fail-fast > "$G/mid-s$n.log" 2>&1
  if grep -aqE "^test result" "$G/mid-s$n.log"; then echo "done $n" > "$G/mid-done"; exit 0; fi
  sleep 120
done
echo "gave up" > "$G/mid-done"
