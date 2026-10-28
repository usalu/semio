#!/bin/sh
# 🔁️ [sealprep] The gate's promote step after EVERY landing that touches the build tool's closure (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/**, 🧰️framework/🔨️modules/🏃️process/**, their schemas and packages), BEFORE that group's verdict run and only between runs. Exit 0 = fresh / promoted / served; exit 1 = BLOCKED (the output names the blocking path:line:col: that landing is broken, revert it); a promote of an unchanged tree is `fresh` in under 1 s.
# usage: sh promote.sh [ROOT]   (default: the live repository this script lives in)
ROOT="${1:-$(cd "$(dirname "$0")/../../../../../../.." && pwd)}"; cd "$ROOT" || exit 2
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts"
PB="$(ls "$ROOT"/.cache/b/t/m/bun/*/bun 2>/dev/null | head -1)"; [ -x "$PB" ] || PB="$(command -v bun)"
out=$(env -u SEMIO_REPO_IMPLEMENTATION SEMIO_REPO_SEAL=promote "$PB" "$D" repo 2>&1); rc=$?
echo "$out" | grep -E "^\[implementation\]" | grep -v -E "Versiegelter|Arbeitsbaum|befördert|nicht befördert" | cut -c1-240
echo "promote exit $rc"; exit $rc
