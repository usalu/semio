#!/bin/zsh
# 🔁️ Proves the repair chain of assets without writing the catalog. (1) Fixed point: the ops writer `make/<id>.py` runs on the catalog document as it is, its ops are applied into $GEN/idem/<id>.json (apply.ts --out) and that copy is compared byte for byte with the catalog document. (2) Reproduction: starting from the backup of the untouched document ($GEN/backup/<id>.json) the writer and the apply run pass after pass on their own result (MC_SOURCE) until a pass changes nothing (at most five), and the result is compared with the catalog document. One line per asset, appended to $GEN/log/idem.txt; the ops of the last catalog pass stay in ops/: idem.sh [<id>…]   (run it through q.sh with 1000 MB)
source "${0:h}/env.sh"
ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($IDS)
mkdir -p "$GEN/idem/chain" "$GEN/log"
for a in $ids; do
  f="$(asset "$a")"; c="$GEN/idem/chain"
  cp "$GEN/backup/$a.json" "$c/$a.json"; passes=0; ops=""
  while (( passes < 5 )); do
    MC_SOURCE="$c" python3 "$W/make/$a.py" > /dev/null 2>&1 || { ops="THE OPS WRITER FAILED"; break; }
    ops+="$(grep -c '"op"' "$W/ops/$a.json") "
    (cd "$P" && timeout 300 "$B" "$W/apply.ts" "$a" --in "$c/$a.json" --out "$c/$a.next.json") > /dev/null 2>&1
    passes=$(( passes + 1 ))
    if cmp -s "$c/$a.json" "$c/$a.next.json"; then break; fi
    mv "$c/$a.next.json" "$c/$a.json"
  done
  chain="CHAIN FROM THE BACKUP DIFFERS FROM THE CATALOG"; cmp -s "$c/$a.json" "$f" && chain="chain from the backup gives the catalog document"
  python3 "$W/make/$a.py" > /dev/null 2>&1 || { echo "$a: the ops writer failed on the catalog document" | tee -a "$GEN/log/idem.txt"; continue; }
  last="$(cd "$P" && timeout 300 "$B" "$W/apply.ts" "$a" --out "$GEN/idem/$a.json" 2>&1 | tail -1 | cut -c1-160)"
  same="A FURTHER PASS CHANGES THE CATALOG DOCUMENT"; cmp -s "$GEN/idem/$a.json" "$f" && same="a further pass leaves the catalog document as it is"
  echo "$(date +%H:%M) $a | passes from the backup until unchanged: $passes (ops ${ops% }) | $chain | $last | $same" | tee -a "$GEN/log/idem.txt"
done
