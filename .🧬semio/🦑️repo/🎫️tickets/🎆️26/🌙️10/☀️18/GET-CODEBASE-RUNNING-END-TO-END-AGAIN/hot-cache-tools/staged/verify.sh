#!/bin/sh
# Verifies the whole staged set without touching the repository: natives, scratch roots, every suite.
# usage: verify.sh [REPO_ROOT] [OUT]   (REPO_ROOT defaults to the current directory)
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=${1:-$(pwd)}
OUT=${2:-${TMPDIR:-/tmp}/hot-cache-verify}
TICKET=$(cd "$HERE/../.." && pwd)
rm -rf "$OUT" && mkdir -p "$OUT/bin" "$OUT/root"
echo "== natives"
python3 "$HERE/build-native.py" "$OUT/bin" --test
echo "== resident member: new files, suites in the taxonomy layout"
python3 "$HERE/apply.py" --root "$OUT/root" --no-edits --write
python3 "$HERE/run-staged-tests.py" "$OUT/root" "$OUT/bin"
echo "== service invocation: the authority core in a real copy of the library (tracked files), anchored edits applied, core suite, law and every resident suite"
python3 "$HERE/make-real-overlay.py" "$REPO" "$OUT/real"
python3 "$HERE/apply.py" --root "$OUT/real" --skip-missing --write | tail -2
python3 "$HERE/run-staged-tests.py" "$OUT/real" "$OUT/bin"
python3 "$HERE/core-mutation-check.py" "$OUT/real" | tail -1
echo "== go launcher: repo-launcher overlay plus the resident client, route fixture and entrypoint oracle"
python3 "$TICKET/repo-launcher-tools/make-overlay.py" "$REPO" "$OUT/launcher-root" >/dev/null
python3 "$TICKET/repo-launcher-tools/apply.py" --root "$OUT/launcher-root" --apply --ignore-running >/dev/null
for f in "📜️script.ts" "🧭️invocation/🟦️.ts" "🕸️graph/🟦️.ts" "⚙️operations/🔣️catalog.json" "⚙️operations/🧬️schema/🔣️.json"; do
  B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
  mkdir -p "$OUT/launcher-root/$B/$(dirname "$f")"; [ -s "$OUT/launcher-root/$B/$f" ] || cp "$REPO/$B/$f" "$OUT/launcher-root/$B/$f"
done
cp "$REPO/.gitignore" "$OUT/launcher-root/.gitignore"
python3 "$HERE/apply.py" --root "$OUT/launcher-root" --skip-missing --write
MODULE="$OUT/launcher-root/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"
nice -n 10 bash "$TICKET/repo-launcher-tools/flatten-test.sh" "$MODULE" "$OUT/go-work" "$OUT/launcher-root" -timeout 120s
echo "== tool registry: tool-iface files plus the repo-watch entry, their registry, resolve, run and law suites"
rm -rf "$OUT/tool-files" && mkdir -p "$OUT/tool-files" && cp -R "$TICKET/tool-iface-tools/src/files/"* "$OUT/tool-files/"
rm -rf "$OUT/mine-root" && cp -R "$OUT/tool-files" "$OUT/mine-root"
python3 "$HERE/apply.py" --root "$OUT/mine-root" --skip-missing --write >/dev/null
python3 "$HERE/mirror.py" "$REPO" "$OUT/tool-root" "$OUT/mine-root"
cd "$OUT/tool-root"
T="🧰️framework/🔨️modules/🏃️process/🧰️tools"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
for f in "$T/🧪️tests/📖️registry/🟦️.ts" "$T/🧪️tests/🔎️resolve/🟦️.ts" "$T/🧪️tests/🏃️run/🟦️.ts" "$L/⚖️laws/🧰️direct-tool-spawn/🧪️tests/🟦️.ts"; do
  nice -n 10 bun test "./$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)"
done
echo "== done"
