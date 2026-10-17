#!/bin/sh
# Plans a batch and prepares its before-state inputs (dry-run report, tsconfig, test list). Usage: prep.sh <fresh-label> <batch> <sourceOwner>...
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
cd "$F" || exit 2
L="$1"; B="$2"; shift 2
OWN=""
set -- $(printf '%s\n' "$@")
args=""
for o in "$@"; do args="$args --owner"; done
ARGV=""
for o in "$@"; do ARGV="$ARGV|$o"; done
python3 - "$G" "$L" "$B" "$@" <<'PY'
import sys, subprocess
G, L, B, *owners = sys.argv[1:]
import re
if owners == ["--all-ts"]:
    owners = []
    for line in open(f"{G}/psc3-groups-{L}.txt", encoding="utf8"):
        c = line.rstrip("\n").split("\t")
        m = re.search(r"ts=(\S+)", line)
        if len(c) > 1 and m and m.group(1) != "NONE":
            owners.append(c[1].replace("F/M/", "🧰️framework/🔨️modules/").replace("F/P/", "🧰️framework/🛍️products/"))
cmd = ["bun", "psc3-tools/plan.ts", f"{G}/psc3-policy-{L}.json", f"{G}/psc3-scan-{L}.json", f"{G}/psc3-plan-{B}.json"]
for o in owners: cmd += ["--owner", o]
cmd += ["--exclude-owner-target", "🧰️framework=🧰️framework/🔨️modules/🖱️ui/"]
import os
if os.environ.get("WITH_SKIPPED"): cmd += ["--with-skipped-importers"]
r = subprocess.run(cmd, capture_output=True, text=True)
print(r.stdout[-3000:], r.stderr[-500:])
PY
bun psc3-tools/apply.ts "$G/psc3-plan-$B.json" | tail -n 6
bun psc3-tools/rewrite.ts "$G/psc3-plan-$B.json" "$G/psc3-scan-$L.json" "$G/psc3-policy-$L.json" --report "$G/psc3-$B-dry-report.json" | head -n 12
bun psc3-tools/mktsc.ts "$G/psc3-$B-dry-report.json" "$G/psc3-tsconfig-$B.json" --facades "$G/psc3-plan-$B.json"
bun psc3-tools/picktests.ts "$G/psc3-$B-dry-report.json" "$G/psc3-policy-$L.json" "$G/psc3-tests-$B.txt" 400
