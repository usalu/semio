#!/usr/bin/env bash
# Runs the official static boot matrix on the last built site and, with ACCEPT=all or ACCEPT="<failing titles allowed>", records the exact manifest package-site.sh requires. Usage: [ACCEPT="generator boot"] accept-site.sh [port]
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; ROOT="$(cd "$T/../../../../../../.." && pwd)"
OUT="$(python3 "$T/captured-site.py" "$ROOT" "$G/native-site.log")"
python3 "$T/site-manifest.py" "$OUT" "$ROOT" "$G/static-before.sha256"
"$T/verify-static.sh" "$OUT" "${1:-8765}" || true
ACCEPT="${ACCEPT:-}" python3 - "$G/static-playwright.json" <<'P'
import json, os, sys
report = json.load(open(sys.argv[1], encoding="utf8"))
allowed = {name.strip() for name in os.environ["ACCEPT"].split(",") if name.strip()}
suites, failing, passed = list(report["suites"]), [], 0
while suites:
    suite = suites.pop(); suites.extend(suite.get("suites", []))
    for spec in suite["specs"]:
        for test in spec["tests"]:
            if test["results"] and test["results"][-1]["status"] == "passed": passed += 1
            else: failing.append((test["projectName"], spec["title"]))
print(f"passed {passed}, failing {sorted(failing)}")
sys.exit(any(title not in allowed for _, title in failing) or not passed)
P
python3 "$T/site-manifest.py" "$OUT" "$ROOT" "$G/static-after.sha256"
cmp "$G/static-before.sha256" "$G/static-after.sha256"
cp "$G/static-after.sha256" "$G/static-accepted.sha256"
echo "ACCEPTED $OUT"
