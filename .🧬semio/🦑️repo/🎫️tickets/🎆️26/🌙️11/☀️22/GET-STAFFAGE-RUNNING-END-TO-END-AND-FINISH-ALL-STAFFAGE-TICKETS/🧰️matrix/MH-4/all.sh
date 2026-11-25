#!/bin/zsh
# 🚆️ The rest of the batch in one strictly sequential chain, every run through the memory queue: all.sh   waits for the matrix of person-with-white-cane that was in flight when the queue rule came, records its cells, then finishes the assets whose dry run is clean and, with their remaining findings stated, the others (a full gate of the dry run first where it is missing or older than the copy). Output in $GEN/all.out.
source "${0:A:h}/env.sh"
while pgrep -f "matrix.ts --language en --out $GEN/matrix-person-with-white-cane" > /dev/null; do sleep 10; done
a=person-with-white-cane
{ echo "matrix exit ? $(tail -1 "$GEN/proof/$a-matrix.log")"; python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-300; echo "$a proof took (matrix in flight when the queue rule came, not repeated)"; } >> "$GEN/proof/$a.out"
cp "$S/🗂️catalog/👤️humans/$a/🔣️.json" "$GEN/done/$a.json"
for a in photographer-with-camera person-sweeping picnicker-sitting-on-ground; do "$W/finish.sh" "$a"; done
for a in person-with-arm-sling postal-carrier person-with-rollator person-with-guide-dog person-stretching; do
  if [[ ! -f "$GEN/try/$a.gate-full.log" || "$GEN/try/$a.gate-full.log" -ot "$GEN/try/$a.json" ]] || ! "$W/cap.sh" 1000 300 "$T" "$B" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$W/ops/$a.json" --out "$GEN/try/$a.cmp.json" > /dev/null || ! cmp -s "$GEN/try/$a.cmp.json" "$GEN/try/$a.json"; then "$W/try.sh" "$a" full > "$GEN/try/$a.full.out" 2>&1; fi
  rm -f "$GEN/try/$a.cmp.json"
  ALLOW_OPEN=1 "$W/finish.sh" "$a"
done
echo "all finished $(date +%H:%M)"
