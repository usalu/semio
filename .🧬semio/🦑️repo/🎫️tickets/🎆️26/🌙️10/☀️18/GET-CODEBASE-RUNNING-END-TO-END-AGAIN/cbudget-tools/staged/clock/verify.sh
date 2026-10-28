#!/bin/sh
# ⏲️ Laws of batch `clock` after apply: VERIFY_ROOT=<repository or mirror root> sh verify.sh  (pass the mirror root: a script that walks up from its own path resolves the LIVE root)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
export SEMIO_TEST_ARTIFACT_DIR=${SEMIO_TEST_ARTIFACT_DIR:-$here/../../../🗑️generated/cbudget}
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
T=./🧰️framework/🔨️modules/🏃️process/🧪️testing
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$here/indexcheck.py" "$root"
bun test "$T/🎚️budget/⏲️contention/🧪️tests/🟦️.ts" "$T/🎚️budget/🧪️tests/🟦️.ts" "$T/🎛️execution/🧪️tests/🟦️.ts" "$T/🧪️vitest/🧪️tests/🟦️.ts" "$T/🚦️protocol/🧪️tests/🟦️.ts"
