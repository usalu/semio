#!/bin/sh
# 👯 Red check of the kernel law against the multiwriter copy that LANDED in G46 (audit10): in a MIRROR, swaps batch `multiwriter` for `C/audit10-tools/staged/multiwriter`, installs the kernel law without its `waitAnyLeaseV1` case (that helper is new), runs it, and restores the batch.
# usage: sh redlanded.sh <mirror root with batch multiwriter applied>
set -u
here=$(cd "$(dirname "$0")" && pwd); root=${1:?mirror root}; landed="$here/../audit10-tools/staged/multiwriter"
[ -d "$root/.git" ] && { echo "refusing the live checkout"; exit 2; }
G="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations"; law="$G/🔐️custody/🧪️tests/🔑️kernel/🟦️.ts"
python3 -I "$here/staged/multiwriter/stage.py" revert --root "$root" >/dev/null && python3 -I "$landed/stage.py" apply --root "$root" >/dev/null || { echo "swap failed"; exit 2; }
mkdir -p "$(dirname "$law")"
python3 -I - "$here/src/🔐️custody/🧪️tests/🔑️kernel/🟦️.ts" "$law" <<'PY'
import sys,re
s=open(sys.argv[1],encoding="utf8").read().replace(",waitAnyLeaseV1}","}")
i=s.index('test("the shared wait helper');j=s.index('test(',i+5);s=s[:i]+s[j:]
open(sys.argv[2],"w",encoding="utf8").write(s)
PY
(cd "$root" && bun test "./${law#$root/}" 2>&1 | grep -E '^\(fail\)|^\(pass\)|^ *[0-9]+ (pass|fail)|^error:|Received:' | cut -c1-200 | head -12)
rm -f "$law"; rmdir "$(dirname "$law")" 2>/dev/null
python3 -I "$landed/stage.py" revert --root "$root" >/dev/null && python3 -I "$here/staged/multiwriter/stage.py" apply --root "$root" >/dev/null && echo "restored batch multiwriter"
