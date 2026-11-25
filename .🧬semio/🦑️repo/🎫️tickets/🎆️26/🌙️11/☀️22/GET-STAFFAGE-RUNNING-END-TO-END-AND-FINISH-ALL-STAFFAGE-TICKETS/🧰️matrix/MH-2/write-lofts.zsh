#!/bin/zsh
# 🖋️ Writes the loft plan of one walker into the catalog and proves the written file: write-lofts.zsh <id>… (inside a grant of the queue). Per asset: backup $GEN/backup/<id>.pre-lofts.json, `limbs.ts --write` twice (the second must change nothing), then prints, validateAsset, silhouette guard, the gate over every subject with twins and the authors' scan; logs $GEN/proof/<id>-{lofts,prints,validate,guard,gate,scan}.log, one line per step on stdout.
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"; P="$S/📦️packages/🟦️typescript"; d="$GEN/proof"
for a in "$@"; do f="$S/🗂️catalog/👤️humans/$a/🔣️.json"; echo "=== W $a $(date +%H:%M) md5 before $(md5 -q "$f" | cut -c1-8)"
  [[ -e "$GEN/backup/$a.pre-lofts.json" ]] || cp "$f" "$GEN/backup/$a.pre-lofts.json"
  cd "$T"; for i in 1 2; do "$B" "$W/limbs.ts" "$a" --plan "$W/plans/$a.lofts.json" --write > "$d/$a-lofts-$i.log" 2>&1; echo "lofts $i exit $? $(grep -c REFUSED "$d/$a-lofts-$i.log") refused: $(tail -1 "$d/$a-lofts-$i.log" | cut -c1-200)"; done
  echo "md5 after $(md5 -q "$f" | cut -c1-8)"
  "$B" "$U/🧰️matrix/MC/prints.ts" "$f" > "$d/$a-prints.log" 2>&1; echo "prints exit $? $(grep -v '^\[' "$d/$a-prints.log" | tail -1 | cut -c1-200)"
  cd "$P"; "$B" "$U/🧰️matrix/MC/validate.ts" "$f" > "$d/$a-validate.log" 2>&1; echo "validate exit $? $(tail -1 "$d/$a-validate.log" | cut -c1-200)"
  "$B" "$U/🧰️matrix/MX-2/guard.ts" "$f" > "$d/$a-guard.log" 2>&1; echo "guard exit $?"; cut -c1-220 "$d/$a-guard.log"
  cd "$T"; "$B" "$M/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")"
  "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)"
done
