#!/bin/zsh
# 🏭️ Writes the repair of several plants of batch MV-3 into the catalog, one after the other (`make.sh` each: the chain on the catalog file, then the gate), then checks that every authored scale prints (`suit.ts`) and that a second run of the chain changes no byte; meant for one grant (`grant.sh 1500 <timeout> make-many.sh <id>…`). Result lines per plant go to `$GEN/log/makes.txt`.
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
q() { if [[ -n "$MV3_GRANT" ]]; then local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; else "$W/q.sh" "$@"; fi }
for a in "$@"; do
  f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
  { echo "== $a $(date +%H:%M) $(wc -c < "$f" | tr -d ' ') bytes before"
    "$W/make.sh" "$a"
    first="$(shasum -a 256 < "$f")"
    q 1000 "$T" 900 "$B" "$W/suit.ts" "$a" > "$GEN/log/$a-make-suit.log" 2>&1; echo "suit exit $? $(tail -1 "$GEN/log/$a-make-suit.log")"
    if [[ -z "$ONCE" ]]; then "$W/make.sh" "$a" > "$GEN/log/$a-make-second.log" 2>&1; [[ "$(shasum -a 256 < "$f")" == "$first" ]] && echo "second run: no byte changed" || echo "second run: FILE CHANGED"; fi
    echo "$a: $(wc -c < "$f" | tr -d ' ') bytes after"
  } >> "$GEN/log/makes.txt" 2>&1
done
echo "makes done: $*"
