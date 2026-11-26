# 📍️ Stretching: the far places of v6 beside those of the variant without the loft that moves the guard (sourced by q.sh).
far() { cap 1200 "$P" "$B" "$W/far.ts" "$GEN/try/person-stretching.$1.json" --scale "$2" --view "$3" --top 40 2>&1 | tr ';' '\n' | sed 's/^ *//' > "$GEN/try/far.$1.$2.$3.txt"; }
far v6 50 plan; far x50-sleeve-r 50 plan; far v6 100 front; far x100-sleeve-r 100 front; far v6 100 elevation; far x100-hoodie 100 elevation
