#!/bin/bash
# 🦀️ P6 cargo call: one fleet slot, private target/build dirs under ⚡️cache/p6, log under ⚡️cache/p6/logs/<tag>.log.
# With `--message-format=json` in the args the summary counts compiler-message errors per crate from the JSON log.
# usage: 🦀️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; C="$ROOT/.🧬semio/🦑️repo/⚡️cache/p6"; mkdir -p "$C/logs"
export CARGO_TERM_COLOR=never
cd "$ROOT" || exit 2
s=$(date +%s)
bash "$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p6-$TAG" -- cargo "$@" > "$C/logs/$TAG.log" 2>&1
rc=$?
if [[ " $* " == *"--message-format=json"* ]]; then
  python3 - "$C/logs/$TAG.log" "$rc" "$(( $(date +%s) - s ))" <<'PY'
import json, sys, collections
errors, warnings, failed = collections.Counter(), collections.Counter(), []
for line in open(sys.argv[1], encoding="utf-8", errors="replace"):
    if not line.startswith("{"):
        if line.startswith("error: could not compile") or line.startswith("error:"): failed.append(line.strip()[:200])
        continue
    try: m = json.loads(line)
    except ValueError: continue
    if m.get("reason") == "compiler-message":
        lvl = m["message"].get("level"); pkg = m.get("package_id", "?").split("#")[-1].split("@")[0]
        (errors if lvl == "error" else warnings)[pkg] += lvl in ("error", "warning")
print(f"rc={sys.argv[2]} wall={sys.argv[3]}s errors={sum(errors.values())} warnings={sum(warnings.values())}")
for pkg, n in errors.most_common(): print(f"  errors {n:5d} {pkg}")
for f in failed[:5]: print("  " + f)
PY
else
  echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$C/logs/$TAG.log") warnings=$(grep -c '^warning' "$C/logs/$TAG.log")"
fi
exit $rc
