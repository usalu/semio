#!/bin/zsh
# 🛬️ Writes the proven loft trial of one asset into the catalog and proves it, inside a held grant (third takeover): land.sh <id> <proven-copy.json> <scale>…   refuses unless the catalog file still equals $GEN/base/<id>.json; runs lofta.ts with $W/lofts/<id>.json and --write, then $W/after/<id>.json with fit.ts when it exists, runs lofta.ts once more (it must change nothing), compares the catalog file with the proven copy, renders the sheets of the scales named in the default pose (before from the base, after from the catalog) and runs `final.sh light` (gate, scan, silhouette, validate, sweep, check CLI, matrix). Lines in $GEN/proof/<id>.land.out.
source "${0:A:h}/env.sh"
a="$1"; proven="$2"; shift 2; f="$S/🗂️catalog/👤️humans/$a/🔣️.json"; P="$S/📦️packages/🟦️typescript"
{
  echo "== land $a $(date +%H:%M)"
  cmp -s "$f" "$GEN/base/$a.json" || { echo "STOP: the catalog file changed since the trial base"; exit 1; }
  "$W/cap.sh" 800 600 "$P" "$B" "$W/lofta.ts" "$a" --ops "$W/lofts/$a.json" --write | tail -2
  [[ -f "$W/after/$a.json" ]] && "$W/cap.sh" 800 300 "$T" "$B" "$W/fit.ts" "$a" --ops "$W/after/$a.json" --write | tail -2
  "$W/cap.sh" 800 600 "$P" "$B" "$W/lofta.ts" "$a" --ops "$W/lofts/$a.json" --write | tail -2
  if cmp -s "$f" "$proven"; then echo "catalog file equals the proven trial copy"; else echo "STOP: catalog file DIFFERS from the proven copy; put back"; cp "$GEN/base/$a.json" "$f"; exit 1; fi
  for s in "$@"; do BEFORE_DIR="$GEN/base" TAG=before "$W/look.sh" "$a" "$s" | sed 's/.*sheets\//sheet /'; "$W/look.sh" "$a" "$s" | sed 's/.*sheets\//sheet /'; done
  rmdir "$GEN/claim/$a" 2>/dev/null
  "$W/final.sh" light "$a"
  cat "$GEN/proof/$a.light.out" "$GEN/proof/$a.matrix.out" | cut -c1-300
  echo "== landed $a $(date +%H:%M)"
} > "$GEN/proof/$a.land.out" 2>&1
