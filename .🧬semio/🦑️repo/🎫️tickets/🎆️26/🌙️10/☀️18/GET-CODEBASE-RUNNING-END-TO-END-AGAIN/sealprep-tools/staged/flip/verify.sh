#!/bin/sh
# 🧪️ [sealprep] Verifies the sealed FLIP in a root where it is applied (a mirror BEFORE the gate flips live; live right after it, then it only reads): policy default `sealed`; the sealed + door + memo + witness laws (the startup red is the known flip-independent baseline); the real tool with NO switch set (default sealed): `--help`, `leases status` under the dashboard's SEMIO_REPO_IMPLEMENTATION=rust|go, the `live` opt-out, a bad value refused; and the graph-free hot-path simulation with no switch (digest stays through two peer edits and moves only at the explicit promotion). usage: sh verify.sh <root>   (the root must hold ✏️s/🧑‍💻dev/🧩️catalog/📣️publication; hotsim edits and restores two closure files of THAT root, so never pass the live root)
R="${1:?root}"; H="$(cd "$(dirname "$0")" && pwd)"; T="$(cd "$H/../../.." && pwd)"; B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; bad=0; D="$B/🚪️door/📜️script.ts"
[ "$R" != "/Users/ueli/Documents/semio" ] || { echo "refusing the live root (hotsim edits files); run it on a mirror"; exit 2; }
REQ="✏️s/🧑‍💻dev/🧩️catalog/📣️publication"; [ -d "$R/$REQ" ] || { echo "the mirror lacks $REQ: python3 -I C/mirror-tools/mirror.py create <name> --only library process launcher \"$REQ\""; exit 2; }
PB="${PB:-$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun 2>/dev/null | head -1)}"; [ -x "$PB" ] || PB="$(command -v bun)"
echo "policy default: $(python3 -I -c "import json,sys;print(json.load(open(sys.argv[1]))['defaultMode'])" "$R/$B/🏛️sealed/🔣️.json")  (want sealed)"; grep -q '"defaultMode": "sealed",' "$R/$B/🏛️sealed/🔣️.json" || bad=1
for f in "🏛️sealed/🧪️tests/🗂️store" "🏛️sealed/🧪️tests/🚦️trampoline" "🏛️sealed/🧪️tests/👥️processes" "🏛️sealed/🧪️tests/🔀️interleavings" "🏛️sealed/🧭️mode/🧪️tests" "🚪️door/🧪️tests" "🚪️door/🧪️tests/🚦️startup" "🧭️invocation/🕸️graph/🧠️memo/🧪️tests" "🔭️witness/🧪️tests"; do
 out=$(cd "$R" && bun test --timeout 300000 "./$B/$f/🟦️.ts" 2>&1); p=$(echo "$out" | grep -E '^ *[0-9]+ pass' | tr -dc 0-9); n=$(echo "$out" | grep -E '^ *[0-9]+ fail' | tr -dc 0-9)
 echo "${p:-?} pass ${n:-?} fail  $f"; case "$f:${n:-x}" in *startup:1) ;; *:0) ;; *) bad=1;; esac
done
out=$(SEMIO_MIRROR_HOME="$(dirname "$R")" sh "$T/red-g-tools/mrun.sh" "$(basename "$R")" env "$B/🧭️invocation/🧪️tests/🔗️process-chain/🟦️.ts" 2>&1 | head -1); echo "process-chain (batch chainlaw; both modes): $out"; case "$out" in *" 0 fail"*) ;; *) bad=1;; esac
cd "$R" || exit 2; run(){ want="$1"; shift; env -u SEMIO_REPO_SEAL -u SEMIO_REPO_IMPLEMENTATION "$@" >/dev/null 2>&1; code=$?; echo "exit $code (want $want)  ${PROBE}"; [ "$code" = "$want" ] || bad=1; }
PROBE="default: repo --help"; run 0 "$PB" "$D" repo --help
PROBE="default: leases status"; run 0 "$PB" "$D" leases status
PROBE="dashboard switch rust: leases status"; run 0 env SEMIO_REPO_IMPLEMENTATION=rust "$PB" "$D" leases status
PROBE="dashboard switch go: leases status"; run 0 env SEMIO_REPO_IMPLEMENTATION=go "$PB" "$D" leases status
PROBE="SEMIO_REPO_SEAL=live opt-out: leases status"; run 0 env SEMIO_REPO_SEAL=live "$PB" "$D" leases status
PROBE="SEMIO_REPO_SEAL=bogus refused"; run 2 env SEMIO_REPO_SEAL=bogus "$PB" "$D" leases status
python3 -I "$T/sealprep-tools/hotsim.py" "$R" verify --default --var SEMIO_REPO_SEAL --promote-step > /dev/null || bad=1
python3 -I - "$T/🗑️generated/sealprep/hotsim-verify.jsonl" <<'PY' || bad=1
import json, sys
rows = [json.loads(l) for l in open(sys.argv[1])]; runs = {r["step"]: r for r in rows if r["step"].startswith("run-")}
ok = runs["run-1"]["served"] == runs["run-2"]["served"] == runs["run-3"]["served"] == runs["run-4"]["served"] != runs["run-5"]["served"] and not any(r["promoted"] for r in runs.values()) and runs["run-4"]["live"] != runs["run-1"]["live"] and runs["run-1"]["live"] == runs["run-1"]["served"] and runs["run-5"]["live"] == runs["run-5"]["served"] and all(r["exit"] == 0 for r in rows if "exit" in r)
print("hot path (no switch set): digest", " ".join(runs[k]["served"][:8] for k in sorted(runs)), "| live oracle", " ".join(runs[k]["live"][:8] for k in sorted(runs)), "->", "OK" if ok else "FAIL"); sys.exit(0 if ok else 1)
PY
echo "VERIFY exit $bad"; exit $bad
