#!/bin/sh
# 📮 Laws of batch `watchpipe` after apply: VERIFY_ROOT=<repository or mirror root> sh verify.sh  (default: the root above this batch). Needs rustc (the watcher is one std-only file).
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
W="./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔭️watch"
python3 -I "$here/stage.py" check --root "$root"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
python3 -I - "$W/🦀️rust" <<'PY'
import hashlib, json, sys
d = sys.argv[1]
rows = json.load(open(d + "/🔣️.json"))["sources"][0]
data = open(d + "/🦀️.rs", "rb").read()
assert rows["sha256"] == hashlib.sha256(data).hexdigest() and rows["bytes"] == len(data), "provision row drifted from the source"
PY
rustc --edition 2021 --crate-name semio_repo_watch --test "$W/🦀️rust/🦀️.rs" -o "$tmp/unit"
"$tmp/unit"
rustc --edition 2021 --crate-name semio_repo_watch -O "$W/🦀️rust/🦀️.rs" -o "$tmp/watch"
python3 -I "$here/law-stall.py" "$tmp/watch" --files 8000 --stall 3
rustc --edition 2021 --crate-name semio_repo_watch --cfg semio_check_linux --emit=metadata -o "$tmp/linux.rmeta" "$W/🦀️rust/🦀️.rs"
rustc --edition 2021 --crate-name semio_repo_watch --cfg semio_check_windows --emit=metadata -o "$tmp/windows.rmeta" "$W/🦀️rust/🦀️.rs"
bun test "$W/../🧪️tests/🔭️watch/🟦️.ts"
