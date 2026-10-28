#!/bin/sh
# ⚖️ [backlog] runs the single-file laws of one landing group against the CURRENT tree, one at a time, one `pass/fail` line per file (every operand starts with ./). Compare with the baseline in backlog-tools/laws/final-cumulative-laws.jsonl (field `base`: [pass, fail, mode]); a file that needs the process envelope shows 0 pass on a plain run: use red-g-tools/mrun.sh <root-name> env for those. usage: live-laws.sh <A1|B1|C1|D1|D2|D3|E1|F1> [root]
T="$(cd "$(dirname "$0")" && pwd)"; R="${2:-$(cd "$T/../../../../../../.." && pwd)}"; cd "$R" || exit 2
run() { f="$1"; out=$(cd "$(dirname "$f")" && nice -n 10 bun test --timeout 170000 "./$(basename "$f")" 2>&1); printf '%s pass %s fail  %s\n' "$(printf '%s\n' "$out" | grep -E '^ *[0-9]+ pass$' | tr -dc 0-9)" "$(printf '%s\n' "$out" | grep -E '^ *[0-9]+ fail$' | tr -dc 0-9)" "$f"; }
export -f run 2>/dev/null
while IFS= read -r f; do [ -f "$f" ] && run "$f"; done < "$T/laws/$1.txt"
