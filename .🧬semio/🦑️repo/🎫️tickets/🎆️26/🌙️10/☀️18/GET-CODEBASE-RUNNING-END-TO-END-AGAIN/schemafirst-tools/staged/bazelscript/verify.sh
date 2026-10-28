#!/bin/sh
# 🧬️ Verifies the staged batch `bazelscript` (lane schemafirst): stage check, Bun parse-check + docstring/emoji/[DEBUG] laws of the changed files, then the policy literal law over the whole repository (the module tests are listed for the envelope runner).
# usage: VERIFY_ROOT=<repository or mirror root> sh verify.sh   (default: the root above this batch)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
tools=$(cd "$here/../.." && pwd)
c=$(cd "$tools/.." && pwd)
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$tools/docheck.py" "$here" "$root"
(cd "$root" && bun test "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧬️policy-literals/🟦️.ts")
echo 'module tests, each file through the envelope runner (C/red-g-tools/mrun.sh <mirror> env <file>), failure sets equal to the base: ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests'
