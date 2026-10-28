#!/bin/sh
# 🇩🇪 Laws of batch `docs` (lane germanqa): VERIFY_ROOT=<repository or mirror root> sh verify.sh   (default root: the checkout above the ticket)
# before apply the law is RED on this batch's files (counts printed, informational); after apply it must be GREEN, then the test directories next to the changed files run.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
law="$here/../../law_germanqa.py"
python3 -I "$law" --selftest
python3 -I "$here/../../law_edits.py" "$here"
echo "### before apply (expected RED unless already applied)"
python3 -I "$here/stage.py" check --root "$root" >/dev/null 2>&1 && python3 -I "$law" "$root" --files-from "$here/files.json" --limit 1 | tail -1 || true
python3 -I "$here/stage.py" apply --root "$root"
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$law" "$root" --files-from "$here/files.json" --limit 5
cd "$root"
if [ -n "${SEMIO_TEST_ARTIFACT_DIR:-}" ] && [ -s "$here/tests.txt" ]; then
  while IFS= read -r d; do [ -n "$d" ] && (cd "$root/$d" && echo "### $d" && bun test --timeout 60000 ./🟦️.ts 2>&1 | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$' || true); done < "$here/tests.txt"
else echo "tests skipped: set SEMIO_TEST_ARTIFACT_DIR (short path under the ticket generated folder) to run $(wc -l < "$here/tests.txt") test directories"; fi
echo "batch docs: law GREEN"
