#!/bin/sh
# 🧪 ciwf law: `sh run-law.sh [root]` (default: the repository root above this file). Needs bun; the Python oracles run when PyYAML and jsonschema exist (never a silent pass: they print "unavailable").
set -eu
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-$(cd "$here" && while [ ! -d "🧰️framework" ] && [ "$PWD" != / ]; do cd ..; done; pwd)}
bun test "$here/law/check.test.ts"
bun "$here/law/check.ts" "$root"
python3 "$here/law/ctxmodel.py" "$root"/.github/workflows/*.yml
python3 "$here/law/oracle.py" "$root" || [ $? -eq 3 ]
! python3 "$here/law/ctxmodel.py" "$here/fixtures/pushed-build-system.red.yml" >/dev/null
