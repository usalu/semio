#!/bin/zsh
# 🧾️ The light part of the proof of several written assets inside ONE grant of 2500 MB (start it with qsh), every process one after the other: per asset the gate over every pose, clip instant and transition (skipped when the catalog file is the dry run whose all-subject gate log shows 0 hard findings and the asset is not named in REGATE), then one official scan, one measure (silhouette, validate, sweep), one print suitability and ONE vitest process for all of them: the six test files of the proof (catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation, asset-power) run one after the other (`--no-file-parallelism`) with the name filter `(<id>|…)( |$|:)|accepts the sample figure|validates the manifest` (the union of the filters of vt.sh), and the passed and failed tests are counted per asset and file from the verbose listing: TAG=<name> REGATE="<id> …" QTIME=<s> qsh in-prove.sh <id>…   (logs in $GEN/proof/<TAG>-*.log)
[[ -n "$MH3_GRANT" ]] || { echo "in-prove.sh runs only inside a grant: use qsh"; exit 2; }
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d"; tag="${TAG:-many}"
for a in "$@"; do
  if [[ " $REGATE " != *" $a "* ]] && cmp -s "$S/🗂️catalog/👤️humans/$a/🔣️.json" "$GEN/try/$a.json" && grep -q "^hard findings: 0" "$GEN/gate/$a.try.log" && [[ $(grep -c " subjects " "$GEN/gate/$a.try.log") -ge 4 ]] && ! grep -q -- "--poses" "$GEN/gate/$a.flags" 2>/dev/null; then
    echo "$a gate: catalog file = dry run of $(date -r "$GEN/gate/$a.try.log" '+%d %H:%M'); $(tail -1 "$GEN/gate/$a.try.log") ($(grep " subjects " "$GEN/gate/$a.try.log" | sed -E 's/.* 1:([0-9]+) subjects ([0-9]+).*/1:\1 \2/' | tr '\n' ' ')subjects)"
  else
    start=$(date +%s); (cd "$T" && timeout ${GATE_TIMEOUT:-3000} "$B" "$M/gate.ts" "$a" --twins) > "$d/$a-gate.log" 2>&1; echo "$a gate exit $? $(tail -1 "$d/$a-gate.log") ($(( $(date +%s) - start )) s; $(grep " subjects " "$d/$a-gate.log" | sed -E 's/.* 1:([0-9]+) subjects ([0-9]+).*/1:\1 \2/' | tr '\n' ' ')subjects)"
  fi
done
(cd "$T" && timeout 3000 "$B" 🔎️scan.ts --language en "$@") > "$d/$tag-scan.log" 2>&1; echo "scan exit $? $(grep -E '^\[scan\]' "$d/$tag-scan.log" | tail -1 | cut -c1-200)"; grep -E "^person-[a-z-]+ \(humans\)" "$d/$tag-scan.log" | cut -c1-200
(cd "$S/📦️packages/🟦️typescript" && timeout 3000 "$B" "$M/measure.ts" "$@") > "$d/$tag-measure.log" 2>&1; echo "measure exit $?"; grep -E "silhouette:|validate:|sweep:" "$d/$tag-measure.log"
(cd "$S/📦️packages/🟦️typescript" && timeout 1800 "$B" "$here/suit.ts" "$@") > "$d/$tag-suit.log" 2>&1; grep -E "UNSUITABLE|^suitability:" "$d/$tag-suit.log" | cut -c1-300
pat="(${(j:|:)@})( |\$|:)|accepts the sample figure|validates the manifest"
files=(); for suite in 🧊️catalog-silhouette 🧊️three-views 🌡️catalog-physics 🫁️breathing-catalog ✅️asset-validation ⚡️asset-power; do files+=("../../🧪️tests/$suite/🟦️.ts"); done
(cd "$S/📦️packages/🟦️typescript" && timeout ${VT_TIMEOUT:-3600} "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" --no-file-parallelism --reporter=verbose "${files[@]}" -t "$pat") > "$d/$tag-vitest.log" 2>&1
echo "vitest exit $? $(grep -E '^ *(Test Files|Tests) ' "$d/$tag-vitest.log" | tr -s ' ' | tr '\n' ';')"
python3 - "$d/$tag-vitest.log" "$@" <<'PY'
import re, sys
log, ids = sys.argv[1], sys.argv[2:]
lines = [re.sub(r"\x1b\[[0-9;]*m", "", line).rstrip() for line in open(log, errors="replace")]
lines = [line for line in lines if re.match(r"\s*[✓×✗]", line)]
suites = ["catalog-silhouette", "three-views", "catalog-physics", "breathing-catalog", "asset-validation", "asset-power"]
def tally(test):
    out = []
    for suite in suites:
        hit = [line for line in lines if suite + "/" in line and test(line)]
        bad = sum(1 for line in hit if line.lstrip()[0] != "✓")
        out.append(f"{suite} {len(hit) - bad}✓" + (f" {bad}✗" if bad else ""))
    return "; ".join(out)
for a in ids:
    print(f"{a}: " + tally(lambda line: re.search(re.escape(a) + r"( |$|:)", re.sub(r"\s+\d+m?s$", "", line)) is not None))
print("shared cases: " + tally(lambda line: "accepts the sample figure" in line or "validates the manifest" in line))
PY
