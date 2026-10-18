#!/bin/zsh
# iter.sh <family-name> <tag> [cfgs] : run oracle, plan+apply widening for cur privacy rows, repeat (max 6)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
fam="$1"; tag="$2"; cfgs="${3:-lib,test}"
cd /Users/ueli/Documents/semio
for i in 1 2 3 4 5 6; do
  python3 "$F/hdr3-tools/pos/pos.py" run "$F/hdr3-tools/pos/families/$fam.json" --cfgs "$cfgs" --label "$tag$i" > "$G/hdr3-iter-$fam-$tag$i.txt" 2>&1
  errs=()
  for c in ${(s:,:)cfgs}; do f="$G/hdr3pos-$fam-$tag$i-cur-$c-priv.err"; [ -f "$f" ] && errs+=("$f"); done
  python3 "$F/hdr3-tools/pos/pos2widen.py" plan "$F/hdr3-tools/pos/families/$fam.json" cur "$tag$i" $errs > "$G/hdr3-iter-$fam-$tag$i-plan.txt" 2>&1
  n=$(head -1 "$G/hdr3-iter-$fam-$tag$i-plan.txt" | awk '{print $1}')
  echo "round $i: planned $n"
  if [ "$n" = "0" ]; then break; fi
  python3 "$F/hdr3-tools/pos/pos2widen.py" apply "$G/hdr3pos-$tag$i-widen-plan.json" | head -1
done
/usr/bin/grep -E '^-- cfg|privacy:|resolve:|type:|trait-scope:|other:' "$G/hdr3-iter-$fam-$tag$i.txt"
