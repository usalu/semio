#!/bin/sh
# 🗣️ Staged batch `cp-lang` (lane compliance): the English-only prose literals that grew since HEAD (process, caching, bootstrap, cargo, Bazel, witness, graph memo, command closure, sealed layer) become catalog messages in English and German; the language-law allowlist follows the files.
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
