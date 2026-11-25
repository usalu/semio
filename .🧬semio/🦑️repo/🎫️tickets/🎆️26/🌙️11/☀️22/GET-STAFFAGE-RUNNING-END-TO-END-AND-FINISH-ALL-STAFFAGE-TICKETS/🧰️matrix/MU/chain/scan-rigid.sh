#!/bin/zsh
# 🔎️ The authors' gate (`T/🔎️scan.ts`) over every asset of the lane that is not a plant, resumable rows in $GEN/scan/rigid.jsonl, report $GEN/scan/rigid.json: chain/scan-rigid.sh   (run it inside a grant, e.g. as a job of worker.sh)
source "${0:A:h}/../env.sh"
mkdir -p "$GEN/scan"
ids=($(awk -F'\t' '$1!="vegetation"{print $2}' "$U/🧰️matrix/batches/MU.tsv"))
cd "$T" && timeout 6000 "$B" 🔎️scan.ts --language en --quiet --out "$GEN/scan/rigid.jsonl" --report "$GEN/scan/rigid.json" "${ids[@]}" > "$GEN/scan/rigid.log" 2>&1
echo "scan exit $? | $(tail -1 "$GEN/scan/rigid.log" | cut -c1-200)"
