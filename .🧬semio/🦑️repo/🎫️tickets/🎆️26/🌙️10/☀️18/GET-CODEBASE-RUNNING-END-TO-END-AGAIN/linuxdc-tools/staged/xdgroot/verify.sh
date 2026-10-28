#!/bin/sh
# 🔒️ Laws of batch `xdgroot` after apply: sh verify.sh <mirror-root>   (a MIRROR root with the batch applied, never the live checkout: the negative control swaps the base files in and out)
ROOT=${1:?mirror root}; HERE=$(cd "$(dirname "$0")" && pwd); XPLAT5=$(cd "$HERE/../../../xplat5-tools" && pwd); . "$XPLAT5/verify-common.sh"
P='🧰️framework/🔨️modules/🏃️process'
cd "$ROOT" || exit 2
[ -e "$ROOT/.git" ] && { echo "refusing the live checkout"; exit 2; }
python3 -I "$HERE/stage.py" check --root "$ROOT" || exit 2
law "registry roots: empty/relative XDG_CACHE_HOME vs the Python reading" bun test "./$P/🔒️leases/🧭️order/🧪️tests/🪟️registry-root/🟦️.ts"
law "authorizer memo base location" bun test "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧊️memo/🟦️.ts" -t "cache location are closed"
law "agent sessions (unchanged)" bun test "./$P/🪪️agent-sessions/🧪️tests/🟦️.ts"
law "host capacity gate (unchanged)" bun test "./$P/🏗️build/🌡️host-capacity/🧪️tests/🚪️gate/🟦️.ts"
sources() { python3 -I - "$HERE" "$ROOT" "$1" <<'PY'
import json, os, shutil, sys
batch, root, side = sys.argv[1:]
for rel, row in json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8")).items():
    if "/\U0001f9ea\ufe0f" in rel or "\U0001f9ea\ufe0ftests" in rel: continue
    if "tests" in rel.split("/")[-3:-1] or any(part.startswith("\U0001f9ea") for part in rel.split("/")): continue
    shutil.copyfile(os.path.join(batch, "base" if side == "base" else "files", row["file"]), os.path.join(root, rel))
PY
}
sources base; out=$(bun test "./$P/🔒️leases/🧭️order/🧪️tests/🪟️registry-root/🟦️.ts" 2>&1); code=$?; sources new
if [ $code -eq 0 ]; then echo "RED   negative control DID NOT BITE with the base sources"; FAILED=1; else echo "GREEN negative control: red with the base sources ($(printf '%s\n' "$out" | grep -cE '^\(fail\)') failing tests)"; fi
exit $FAILED
