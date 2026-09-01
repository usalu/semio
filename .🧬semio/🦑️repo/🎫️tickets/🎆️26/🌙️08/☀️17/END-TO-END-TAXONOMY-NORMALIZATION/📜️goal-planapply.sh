set -u
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/b3777651-e26e-4d76-aa75-86723494357b/scratchpad"
MOD="$1"; TAG="$2"
for attempt in 1 2 3; do
  B=$(git rev-parse HEAD)
  bun ./📜️script.ts clean taxonomy plan --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION --scope "🧰️framework/🔨️modules/$MOD" --baseline "$B" --plan "$T/🗑️temp/🔣️pa-$TAG.json" --workers 6 > "$S/pa-$TAG-plan.txt" 2>&1
  line=$(grep -o "moves=[0-9]* .*unresolved=[0-9]*" "$S/pa-$TAG-plan.txt" | head -1)
  echo "attempt $attempt plan: $line"
  case "$line" in *"unresolved=0"*) ;; *) echo "  not applyable, stop"; exit 0;; esac
  bun ./📜️script.ts clean taxonomy apply --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION --baseline "$B" --plan "$T/🗑️temp/🔣️pa-$TAG.json" > "$S/pa-$TAG-apply.txt" 2>&1
  if [ $? -eq 0 ]; then echo "  APPLIED on attempt $attempt"; exit 0; fi
  echo "  apply failed: $(grep -E '^error:' "$S/pa-$TAG-apply.txt" | head -1)"
done
echo "gave up after 3 attempts"
