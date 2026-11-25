#!/bin/zsh
# 🔁️ The takeover of ML-1 (2026-11-25 09:00), every step under its own grant of the memory queue, one after the other, logs in 🗑️generated/matrix/ML-1/take: takeover.sh <step>… with step out of verify (the chain of every written asset reproduces the catalog document byte for byte, the spec writers reproduce their spec, the gate over poses, clip instants and transitions, the heater rays), light (scan, validate, print at 800 MB), suites (the real tests with a name filter at 2500 MB), bounds (the real model-bounds cases of the heater), sheets (the sheets of the batch MF that nobody opened and the harp from the catalog; TILTED_ONLY=1 renders the tilted poses of the four bands only), check (check CLI), matrix (the matrix of each of the seven with its peak memory, 1200 MB each). The reservations are those of the queue rule of 2026-11-25 09:20; one request at a time.
source "${0:A:h}/env.sh"
d="$GEN/take"; mkdir -p "$d"
written=(cafe-chair office-swivel-chair used-fridge-freezer used-tumble-dryer used-washing-machine used-concert-harp)
all=($written used-harmonium)
matrix() { for a in "$@"; do out="$GEN/matrix/$a"; rm -rf "$out"; mkdir -p "$out"; qs 1200 3700 "cd '$T' && /usr/bin/time -l timeout 3600 '$B' 📊️matrix.ts --language en --out '$out' --jobs 1 --fresh --assets $a" > "$out/run.log" 2>&1; echo "matrix $a | exit $? | peak $(( $(grep 'maximum resident' "$out/run.log" | awk '{print $1}') / 1048576 )) MB | $(python3 "$M/cells.py" "$out")"; done; }
for step in "$@"; do case "$step" in
  verify)
    cmd="cd '$T'"; for a in $written; do cmd="$cmd; '$B' '$W/fix.ts' $a --out '$d/$a.json' --quiet > '$d/chain-$a.txt' 2>&1"; done
    cmd="$cmd; cp -R '$W/spec' '$d/spec-before'; python3 '$W/make/office-swivel-chair.py'; python3 '$W/make/used-concert-harp.py'"
    cmd="$cmd; '$B' '$W/why.ts' ${(j:,:)all} > '$d/why.txt' 2>&1; '$B' '$W/heater.ts' '$(asset electric-fan-heater)' > '$d/heater.txt' 2>&1"
    qs 800 3600 "$cmd" > "$d/verify.log" 2>&1
    for a in $written; do cmp -s "$d/$a.json" "$(asset $a)" && echo "chain $a: reproduces the catalog document byte for byte | $(tail -1 "$d/chain-$a.txt" | cut -c1-90)" || echo "chain $a: DIFFERS"; done
    diff -r "$d/spec-before" "$W/spec" > /dev/null && echo "spec writers: specs unchanged" || echo "spec writers: SPEC CHANGED"
    echo "why: $(tail -1 "$d/why.txt")"; grep "cases pass" "$d/heater.txt";;
  light) MB=800 zsh "$W/prove.sh" take-light scan,validate,print $all;;
  suites) MB=2500 zsh "$W/prove.sh" take-suites suites $all;;
  bounds) qs 2500 1800 "cd '$P' && timeout 1500 '$B' x vitest run --reporter verbose --config '../../🧪️tests/🎚️config/🟦️.ts' '../../🧪️tests/🧊️model-bounds/🟦️.ts' -t 'heater'" > "$d/bounds.log" 2>&1; grep -E '^ *(Test Files|Tests) |✓|×|AssertionError' "$d/bounds.log" | cut -c1-220 | head -30;;
  sheets)
    list=(); for s in 50 100 200 500; do
      for a in used-bedside-table used-dishwasher used-electric-cooker used-high-chair used-hinged-wardrobe used-kitchen-base-cabinet used-microwave-oven used-recliner-chair used-wingback-armchair; do list+=("${a}:${s}:carried"); done
      list+=("used-park-armchair:${s}:dragged" "used-recliner-chair:${s}:reclined" "used-concert-harp:${s}:played")
    done
    [[ -n "$TILTED_ONLY" ]] || for a in used-bedside-table used-dishwasher used-electric-cooker used-high-chair used-hinged-wardrobe used-kitchen-base-cabinet used-microwave-oven used-recliner-chair used-wingback-armchair used-electric-guitar-and-combo used-electronic-drum-kit used-electronic-organ used-concert-harp; do list+=("$a:50:stored"); done
    [[ -n "$TILTED_ONLY" ]] || list+=("used-park-armchair:50:in-service" "electric-fan-heater:50:off" "gas-hob:50:high" "gas-hob:50:off" "used-concert-harp:50:upright" "used-concert-harp:100:stored" "used-concert-harp:50:x:tilt-to-shoulder:2.2" "used-bedside-table:50:drawer-open" "used-kitchen-base-cabinet:500:stored")
    MB=1200 TO=5400 zsh "$W/looks.sh" take $list >> "$d/sheets.log" 2>&1; echo "sheets: $(grep -c '\.png$' "$d/sheets.log") rendered, $(grep -c '^no sheet' "$d/sheets.log") not";;
  check) MB=1000 zsh "$W/prove.sh" take-check check $all;;
  matrix) matrix $all;;
esac; done
