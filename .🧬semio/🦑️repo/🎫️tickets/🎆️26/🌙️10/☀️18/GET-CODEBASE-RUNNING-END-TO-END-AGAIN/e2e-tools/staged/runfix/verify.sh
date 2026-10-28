#!/bin/sh
# 🔁 verify.sh <root>: the retry law is red on the base scripts and green on the staged ones, and the staged scripts parse.
here=$(cd "$(dirname "$0")" && pwd)
root=${1:?root}
rc=0
python3 -I "$here/files/test_retry.py" "$here/files" || rc=1
mkdir -p "$here/base-law" && cp "$here/base/retry.sh" "$here/base-law/retry.sh"
if python3 -I "$here/files/test_retry.py" "$here/base-law" >/dev/null 2>&1; then echo "law is not red on base"; rc=1; else echo "law red on base (expected)"; fi
rm -rf "$here/base-law"
sh -n "$here/files/retry.sh" || rc=1
bash -n "$here/files/focus.sh" || rc=1
python3 -I "$root/$(dirname "$(python3 -I -c 'import json,sys;print(next(iter(json.load(open(sys.argv[1]))))) ' "$here/manifest.json")")/../xplat5-tools/tests/test_stage_mode.py" "$here/stage.py" || rc=1
exit $rc
