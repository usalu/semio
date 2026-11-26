# 🛬️ Writes the proven trial of one asset into the catalog and proves it (sourced by q.sh, grant 1000 MB): land.sh <id> <variant>   refuses unless the catalog file still equals $GEN/base/<id>.json but for its tags (`$H/same.py`) and unless $W/lofts/<id>.json, $W/after/<id>.json, $W/say/<id>.json are the ops of the variant; keeps the file in $GEN/before; runs the chain with --write (`$H/lofta.ts`, `$H/fit.ts`, `say.ts`), runs it a second time (it must change nothing), compares the catalog file with the proven trial copy and puts the file back when they differ; then prove.sh. Lines in $GEN/proof/<id>.land.out.
a="$1"; v="$2"; f="$C/$a/🔣️.json"; proven="$GEN/try/$a.$v.json"
{
  echo "== land $a $v $(date +%H:%M)"
  for pair in "trials/$a.$v.json:lofts/$a.json" "trials/$a.$v.after.json:after/$a.json" "trials/$a.$v.say.json:say/$a.json"; do
    [[ -f "$W/${pair%%:*}" ]] || continue
    cmp -s "$W/${pair%%:*}" "$W/${pair##*:}" || { echo "STOP: $W/${pair##*:} is not the ops of the trial $v"; return 1; }
  done
  python3 "$H/same.py" "$f" "$GEN/base/$a.json" || { echo "STOP: the catalog file changed since the trial base"; return 1; }
  mkdir -p "$GEN/before"; cp "$f" "$GEN/before/$a.json"
  chain() {
    cap 600 "$P" "$B" "$H/lofta.ts" "$a" --ops "$W/lofts/$a.json" --write | tail -2
    [[ -f "$W/after/$a.json" ]] && cap 300 "$T" "$B" "$H/fit.ts" "$a" --ops "$W/after/$a.json" --write | tail -2
    [[ -f "$W/say/$a.json" ]] && cap 300 "$T" "$B" "$W/say.ts" "$a" --ops "$W/say/$a.json" --write | tail -2
  }
  chain; m1="$(md5 -q "$f")"; echo "-- second run"; chain; m2="$(md5 -q "$f")"
  [[ "$m1" == "$m2" ]] && echo "second run: file unchanged ($m2)" || echo "WARNING: the second run changed the file"
  if python3 "$H/same.py" "$f" "$proven"; then echo "catalog file equals the proven trial copy"; else echo "STOP: catalog file DIFFERS from the proven copy; put back"; cp "$GEN/before/$a.json" "$f"; return 1; fi
  echo "== landed $a $(date +%H:%M)"
} > "$GEN/proof/$a.land.out" 2>&1
cat "$GEN/proof/$a.land.out"
source "$W/prove.sh" "$a"
