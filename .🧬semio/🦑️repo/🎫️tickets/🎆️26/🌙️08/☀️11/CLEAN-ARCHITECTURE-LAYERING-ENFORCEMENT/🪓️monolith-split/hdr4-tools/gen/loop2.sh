#!/bin/zsh
# loop.sh <family> <tag> <owner-crate> <companion-path-fragment> [cfgs] : oracle -> widen privacy rows -> widen unresolved names -> cfg(test) gates, until stable
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
fam="$1"; tag="$2"; owner="$3"; frag="$4"; cfgs="${5:-lib,test}"
cd /Users/ueli/Documents/semio
for i in 1 2 3 4 5 6 7 8; do
  python3 "$F/hdr4-tools/pos/pos.py" run "$F/hdr4-tools/pos/families/$fam.json" --cfgs "$cfgs" --label "$tag$i" > "$G/hdr4-loop-$fam-$tag$i.txt" 2>&1
  errs=(); for c in ${(s:,:)cfgs}; do f="$G/hdr4pos-$fam-$tag$i-cur-$c-priv.err"; [ -f "$f" ] && errs+=("$f"); done
  python3 "$F/hdr4-tools/pos/pos2widen.py" plan "$F/hdr4-tools/pos/families/$fam.json" cur "$tag$i" $errs > "$G/hdr4-loop-$fam-$tag$i-plan.txt" 2>&1
  n=$(head -1 "$G/hdr4-loop-$fam-$tag$i-plan.txt" | awk '{print $1}')
  [ "$n" != "0" ] && python3 "$F/hdr4-tools/pos/pos2widen.py" apply "$G/hdr4pos-$tag$i-widen-plan.json" | head -1
  bun "$F/hdr4-tools/cl.ts" lib "$owner" > "$G/hdr4-loop-lib-$fam.txt" 2>&1
  m=0
  for c in ${(s:,:)cfgs}; do
    ef="$G/hdr4pos-$fam-$tag$i-cur-$c-final.err"
    [ -f "$ef" ] && python3 /Users/ueli/.cache/hdr4/tools/widen_names.py "$ef" "$frag" "$G/hdr4-loop-lib-$fam.txt" --apply > "$G/hdr4-loop-$fam-$tag$i-names-$c.txt" 2>&1 && m=$((m + $(/usr/bin/grep -o 'widened [0-9]*' "$G/hdr4-loop-$fam-$tag$i-names-$c.txt" | awk '{s+=$2} END{print s+0}')))
  done
  echo "round $i: privacy edits $n, name edits $m"
  if [ "$n" = "0" ] && [ "$m" = "0" ]; then break; fi
done
/usr/bin/grep -E '^-- cfg|privacy:|resolve:|type:|trait-scope:|other:' "$G/hdr4-loop-$fam-$tag$i.txt"
