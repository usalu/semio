#!/bin/sh
# 🎭️ Staged batch `cp-libbuild` (lane compliance): unique leading emoji for every docstring of the changed build-library files; the leftover `[DEBUG]` tags of the memo edit law are gone; two orphaned JSDoc tag blocks become docstrings.
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
python3 -I "$tools/verify_batch.py" "$here" "$root"
