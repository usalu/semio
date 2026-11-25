#!/bin/zsh
# 🛬️ Takeover landing of assets under ONE grant of the memory queue (2500 MB): per id the catalog file is put to the state the batch started from ($GEN/backup/<id>.json, only when it differs from it and is the file the predecessors wrote, md5 in $GEN/now/md5.log), fit.ts replays ops/<id>.json with --write once and a second time without writing (it must change nothing), the result is compared with the measured trial $GEN/try/<id>.json (md5), then the light proof of proof.sh (gate, scan, measures, the six real tests) and the sheets given in SHEETS run on the written file; a file that does not equal its trial is put back to what it was and not proven: take.sh <id>…   (an asset whose ops draw a loft is refused until MX-2 wrote `loft recipe ready` into requests.md; TAKE_MB states the reservation and STEPS the steps of the light proof: without the vitest tests 1300 MB carry it; SHEETS="<scale>…" renders before and after sheets of the default pose at these scales; inside the grant the script calls itself with INSIDE=1)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
if [[ -z "$INSIDE" ]]; then INSIDE=1 exec "$W/q.sh" ${TAKE_MB:-2500} ${TAKE_TIMEOUT:-21600} "$W" "${0:A}" "$@"; fi
mkdir -p "$GEN/take"; landed=()
grep -q '^## .*loft recipe ready\|^loft recipe ready' "$U/requests.md" || for a in "$@"; do grep -q '"as":"loft"\|"as": "loft"' "$W/ops/$a.json" && { echo "$a: its ops draw lofts and MX-2 has not written 'loft recipe ready' into requests.md: nothing written"; exit 3 }; done
for a in "$@"; do
  f="$S/🗂️catalog/👤️humans/$a/🔣️.json"; was="$(md5 -q "$f")"; cp "$f" "$GEN/take/$a.was.json"
  if [[ "$was" != "$(md5 -q "$GEN/backup/$a.json")" ]]; then
    grep -q "^$a $was " "$GEN/now/md5.log" || { echo "$a: the catalog file is neither the backup nor the predecessors' file, left alone"; continue }
    cp "$GEN/backup/$a.json" "$f"
  fi
  echo "$a write: $(cd "$T" && "$B" "$W/fit.ts" "$a" --ops "$W/ops/$a.json" --write 2>&1 | tail -1)"
  echo "$a second run (nothing written): $(cd "$T" && "$B" "$W/fit.ts" "$a" --ops "$W/ops/$a.json" 2>&1 | tail -1)"
  if [[ "$(md5 -q "$f")" == "$(md5 -q "$GEN/try/$a.json")" ]]; then echo "$a written, equal to its trial $(md5 -q "$f" | cut -c1-8) $(date +%H:%M)"; landed+=("$a")
  else echo "$a NOT equal to its trial ($(md5 -q "$f" | cut -c1-8) against $(md5 -q "$GEN/try/$a.json" | cut -c1-8)), put back"; cp "$GEN/take/$a.was.json" "$f"; fi
done
(( ${#landed} )) && INSIDE=light "$W/proof.sh" "${landed[@]}"
for a in "${landed[@]}"; do for s in ${=SHEETS}; do INSIDE=1 "$W/sheets.sh" "$a" "before:default:$s" "after:default:$s"; done; done
