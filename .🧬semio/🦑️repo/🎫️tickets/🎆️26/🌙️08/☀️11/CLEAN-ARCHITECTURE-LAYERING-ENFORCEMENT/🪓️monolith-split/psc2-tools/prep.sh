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
    for line in open(f"{G}/psc2-groups-{L}.txt", encoding="utf8"):
        c = line.rstrip("\n").split("\t")
        m = re.search(r"ts=(\S+)", line)
        if len(c) > 1 and m and m.group(1) != "NONE":
            owners.append(c[1].replace("F/M/", "🧰️framework/🔨️modules/").replace("F/P/", "🧰️framework/🛍️products/"))
cmd = ["bun", "psc2-tools/plan.ts", f"{G}/psc2-policy-{L}.json", f"{G}/psc2-scan-{L}.json", f"{G}/psc2-plan-{B}.json"]
for o in owners: cmd += ["--owner", o]
cmd += ["--exclude-owner-target", "🧰️framework=🧰️framework/🔨️modules/🖱️ui/"]
import os
if os.environ.get("WITH_SKIPPED"): cmd += ["--with-skipped-importers"]
r = subprocess.run(cmd, capture_output=True, text=True)
print(r.stdout[-3000:], r.stderr[-500:])
PY
bun psc2-tools/apply.ts "$G/psc2-plan-$B.json" | tail -n 6
bun psc2-tools/rewrite.ts "$G/psc2-plan-$B.json" "$G/psc2-scan-$L.json" "$G/psc2-policy-$L.json" --report "$G/psc2-$B-dry-report.json" | head -n 12
bun psc2-tools/mktsc.ts "$G/psc2-$B-dry-report.json" "$G/psc2-tsconfig-$B.json" --facades "$G/psc2-plan-$B.json"
bun psc2-tools/picktests.ts "$G/psc2-$B-dry-report.json" "$G/psc2-policy-$L.json" "$G/psc2-tests-$B.txt" 400
