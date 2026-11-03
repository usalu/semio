#!/usr/bin/env bash
# The one-time canonicalising batch: snapshot of the committed state, harvest (annotate), generate, strict check, independent checker, classification.
# Usage: batch.sh <scratch dir>   (everything niced, sequential; writes logs into the ticket's generated folder)
set -uo pipefail
scratch="${1:?scratch dir}"
root="$(cd "$(dirname "$0")/../../../../../../../../.." && pwd)"
ticket="$(cd "$(dirname "$0")/../.." && pwd)"
norm="✏️s/🔌️plugins/📕️norm"
log="$ticket/🗑️generated/schema-codegen-v2"
mkdir -p "$log" "$scratch"
cd "$root"
run() { nice -n 19 ionice -c3 "$@"; }
date +%T > "$log/batch-start.txt"
echo "recently touched twins/schemas/structs (last 3 minutes, before the snapshot):" > "$log/mid-edit.txt"
find "$norm" \( -name target -o -name node_modules -o -name 🗑️generated \) -prune -o -type f \( -name '🟦️.ts' -o -name '🛰️.proto' -o -name '🔗️.graphql' -o -name '🦀️.rs' -o -name '🔣️.json' \) -mmin -3 -print >> "$log/mid-edit.txt"
rm -rf "$scratch/before" && mkdir -p "$scratch/before/$norm"
run rsync -a --exclude target --exclude node_modules --exclude 🗑️generated "$norm/" "$scratch/before/$norm/"
run python3 -I "$norm/🧪️tests/🧬️schema-sync/🐍️check.py" --root "$norm" --limit 0 > "$log/check-before.txt" 2>&1
run python3 -I "$ticket/🧰️scripts/schema-codegen/importcheck.py" "$scratch/before/$norm" > "$log/imports-before.txt" 2>&1
run repo ./📜️script.ts schema twins annotate > "$log/annotate.txt" 2>&1
run repo ./📜️script.ts schema twins generate > "$log/generate.txt" 2>&1
run repo ./📜️script.ts schema twins check --strict > "$log/check-strict.txt" 2>&1
run python3 -I "$norm/🧪️tests/🧬️schema-sync/🐍️check.py" --root "$norm" --limit 0 > "$log/check-after.txt" 2>&1
run python3 -I "$ticket/🧰️scripts/schema-codegen/importcheck.py" "$norm" > "$log/imports-after.txt" 2>&1
run python3 -I "$ticket/🧰️scripts/schema-codegen/classify.py" "$scratch/before/$norm" "$norm" "$log/check-before.txt" "$log/imports-before.txt" > "$log/classification.txt" 2>&1
date +%T > "$log/batch-end.txt"
head -3 "$log/annotate.txt" "$log/generate.txt" "$log/check-strict.txt"; grep -h "mismatches=" "$log/check-before.txt" "$log/check-after.txt" | cut -c1-160; head -1 "$log/imports-before.txt" "$log/imports-after.txt"
