#!/bin/bash
# Rebuilds 1029-audit-matrix.md from the per-batch .rc/.txt files: status line, static section 1, generated section 2.
# A batch's latest attempt is its -r2 rerun when present; the first attempt rc is shown alongside.
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1029"
T="$C/1029-tools/matrix"
OUT="$C/1029-audit-matrix.md"
ORDER="hub framework root-flow root-nodegraph cad-aec-building cad-aec-energy cad-aec-structure cad-spatial-shape flow-bim flow-brep flow-dictionary flow-draw flow-list flow-logic flow-math flow-primitive flow-text playbook-procedural process-concrete process-metal process-robotic process-wood sourcing-beams sourcing-slabs sourcing-windows"
DONE=0; TOTAL=0; FAILING=""; BODY=""
for b in $ORDER; do
  TOTAL=$((TOTAL+1))
  first=""; [ -f "$G/matrix-$b.rc" ] && first=$(tr -d ' \n' < "$G/matrix-$b.rc")
  key="$b"; rc=""
  if [ -f "$G/matrix-$b-r2.rc" ]; then key="$b-r2"; fi
  [ -f "$G/matrix-$key.rc" ] && rc=$(tr -d ' \n' < "$G/matrix-$key.rc")
  if [ -z "$rc" ]; then
    BODY="$BODY
### $b

Status: not finished yet.
"
    continue
  fi
  DONE=$((DONE+1))
  if [ "$rc" = "0" ]; then res="rc=0, GREEN"; else res="rc=$rc, RED"; FAILING="$FAILING $b"; fi
  if [ "$key" != "$b" ]; then if [ -n "$first" ]; then res="$res (latest attempt $key; first attempt rc=$first)"; else res="$res (latest attempt $key)"; fi; fi
  BODY="$BODY
### $b

Result: $res

| crate | errors (cargo count) |
|---|---|
"
  counts=$(awk '/^error: could not compile/{ c=$0; sub(/^error: could not compile `/, "", c); sub(/`.*/, "", c); n="?"; if (match($0,/due to [0-9]+ previous/)) { n=substr($0,RSTART+7,RLENGTH-7); sub(/ previous/,"",n) } print c"\t"n }' "$G/matrix-$key.txt")
  if [ -z "$counts" ]; then
    BODY="$BODY| (none) | no \`could not compile\` line in log |
"
  else
    while IFS=$'\t' read -r cn cc; do BODY="$BODY| \`$cn\` | $cc |
"; done <<< "$counts"
  fi
  BODY="$BODY
Top distinct errors per crate (up to 3, count, message, first location):

"
  awk -f "$T/summarize.awk" "$G/matrix-$key.txt" | awk -F'\t' '
    $1=="CRATE"{crate=$2; next}
    $1=="ERR"{key=crate"\t"$2; if(!(key in loc)){loc[key]=$3}; cnt[key]++; if(!(key in seen)){seen[key]=1; keys[++n]=key}}
    END{for(i=1;i<=n;i++){k=keys[i]; split(k,p,"\t"); printf "%s\t%d\t%s\t%s\n", p[1], cnt[k], p[2], loc[k]}}' \
  | sort -t$'\t' -k1,1 -k2,2nr | awk -F'\t' '{ if($1!=last){c=0; last=$1}; c++; if(c<=3){ m=$3; if(length(m)>180) m=substr(m,1,177)"..."; l=$4; if(length(l)>200) l=substr(l,length(l)-199); printf "- `%s` (%d): %s at `%s`\n", $1, $2, m, l }}' > "$G/matrix-$key.top.txt"
  if [ -s "$G/matrix-$key.top.txt" ]; then BODY="$BODY$(cat "$G/matrix-$key.top.txt")
"; else BODY="$BODY- (no error lines; see log for runner failures)
"; fi
done
if [ "$DONE" -eq "$TOTAL" ]; then
  if [ -z "$FAILING" ]; then STATUS="STATUS: GREEN. All $TOTAL batches rc=0 on their latest attempt."; else STATUS="STATUS: RED. $DONE/$TOTAL batches finished. Failing batches (latest attempt):$FAILING"; fi
else
  STATUS="STATUS: RUNNING. $DONE/$TOTAL batches finished. Failing so far:${FAILING:- none}. Matrix runner pid $(cat "$T/pid" 2>/dev/null)."
fi
{
  echo "# 1029 Audit Matrix: wasm32-wasip2 check of the mit-Bestand activation closure"
  echo
  echo "$STATUS"
  echo
  cat "$T/section1.md"
  echo
  echo "## 2. Results per batch"
  echo
  echo "Each batch is one \`repo cargo ... check --manifest-path <ws> --target wasm32-wasip2 --keep-going -p ...\` run, logged to \`🗑️generated/1029/matrix-<batch>.txt\` with rc in \`.rc\`. A \`-r2\` log is the second attempt for that batch and supersedes the first. Crate rows come from cargo's \`could not compile\` lines. Components that depend on a failing crate are blocked by cargo (no own compile result) and are not listed as failing crates."
  echo "$BODY"
  cat "$T/notes.md"
} > "$OUT"
echo "wrote $OUT done=$DONE/$TOTAL"
