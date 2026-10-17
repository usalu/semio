#!/bin/zsh
# 🧲️ Idempotent sweep of FIXTURE-READER-DEDUPE: rescans the tree, migrates every remaining helper copy and ad-hoc manifest-dir read, merges imports, recomputes ownership and wires the dependency rows.
# usage: frd-tools/sweep.sh [--apply]    (plan by default; excluded.json lists the slices whose crates are skipped; delete a fragment from it once that slice is DONE)
set -e
cd /Users/ueli/Documents/semio
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
ulimit -n 65536
APPLY=""; [ "$1" = "--apply" ] && APPLY="--apply"
git -c core.fsmonitor=false -c core.quotepath=false ls-files -z -- '*.rs' | tr '\0' '\n' | /usr/bin/grep -v '^\.🧬semio' > "$G/frd-tracked-rs.txt" || true
git -c core.fsmonitor=false -c core.quotepath=false ls-files --others --exclude-standard -z -- '*.rs' ':!.🧬semio' | tr '\0' '\n' > "$G/frd-untracked-rs.txt" || true
python3 "$F/frd-tools/scan.py" 2>&1 | tail -1
nice -n 10 bun "$F/frd-tools/migrate.ts" $APPLY --out=frd-sweep-t1
nice -n 10 bun "$F/frd-tools/migrate2.ts" --tiers=2 $APPLY --out=frd-sweep-t2
FRD_OWNERS="$G/frd-owners.json"
[ -f "$FRD_OWNERS" ] || nice -n 10 bun "$F/frd-tools/owners.ts"
nice -n 10 bun "$F/frd-tools/migrate2.ts" --tiers=1b $APPLY --out=frd-sweep-t1b
nice -n 10 bun "$F/frd-tools/migrate2.ts" --tiers=3 $APPLY --out=frd-sweep-t3
if [ -n "$APPLY" ]; then
  nice -n 10 bun "$F/frd-tools/normalize.ts" --apply
  rm -f "$FRD_OWNERS"
  nice -n 10 bun "$F/frd-tools/owners.ts"
  nice -n 10 bun "$F/frd-tools/gating.ts"
  python3 - "$F" <<'PY'
import json, sys, collections
F = sys.argv[1]
rows = json.load(open(F + "/🗑️generated/frd-gating.json", encoding="utf-8"))
by = collections.defaultdict(list)
for r in rows: by[r["manifest"]].append(r)
normal = []; feature = []
for m, rs in by.items():
    kinds = {f'{r["cls"]}:{r["gate"]}' for r in rs}
    if "normal:ungated" in kinds: normal.append(m)
    elif "feature:ungated" in kinds: feature.append(m)
json.dump({"normal": normal, "feature": feature}, open(F + "/🗑️generated/frd-dep-classes.json", "w"), ensure_ascii=False, indent=1)
print("normal", len(normal), "feature", len(feature))
PY
  nice -n 10 bun "$F/frd-tools/wire.ts" --apply --out=frd-sweep-wire
fi
echo "sweep done (${APPLY:-plan})"
