#!/bin/zsh
# ⚖️ `s4/prove.ts` on copies of the current documents, one process per asset in one queue grant, lines into $GEN/prove/<id>.txt: prove.sh [<id>…]
me="${0:A}"; source "${0:A:h}/env.sh"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=800 -- timeout 7200 "$me" --inside "$@"; fi
shift; ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($IDS); mkdir -p "$GEN/prove" "$GEN/copies"; cd "$R"
for a in $ids; do src=(); [[ -n "$SOURCE" ]] && src=(--source "$SOURCE/$a.json"); timeout 900 "$B" "$S4/prove.ts" "$GEN/copies" "$a" $src > "$GEN/prove/$a.txt" 2> "$GEN/prove/$a.err"; echo "exit $? $(date +%H:%M) $(grep -m1 '^PROOF' "$GEN/prove/$a.txt" | cut -c1-330) $(grep -c '^RULE' "$GEN/prove/$a.txt") rules"; done
