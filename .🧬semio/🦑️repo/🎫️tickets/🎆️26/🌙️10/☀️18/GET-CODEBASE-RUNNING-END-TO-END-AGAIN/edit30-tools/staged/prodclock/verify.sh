#!/bin/sh
# ⏱ Law of batch `prodclock`: sh verify.sh [root]  (the law fails on base statically, before anything is started)
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:-}
python3 -I "$here/stage.py" check ${root:+--root "$root"}
target=$(python3 -I -c 'import json,sys;print(next(iter(json.load(open(sys.argv[1])))))' "$here/manifest.json")
base=${root:-$(cd "$here/../../../../../../../../../.." && pwd)}
python3 -I "$here/test_prodclock.py" "$base/$target"
echo "VERIFY OK"
