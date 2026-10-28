#!/bin/sh
# 🌱 Laws of batch `incremental` after apply: SEMIO_TEST_ARTIFACT_DIR=<dir for reports> sh verify.sh [<repository or mirror root>]
# 1. the incremental-miss law (real Cargo + rustc, recording inner compiler, plain Cargo as oracle for which units keep a session, Python oracle);
# 2. every unit-cache suite + the compiler lane suite: no failure (all green on base too). The suites run WITHOUT SEMIO_TEST_ARTIFACT_DIR: their scratch
#    workspaces must stay outside every checkout, else the repository `.cargo/config.toml` reaches the scratch Cargo.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
out=${SEMIO_TEST_ARTIFACT_DIR:?SEMIO_TEST_ARTIFACT_DIR required}; mkdir -p "$out"; unset SEMIO_TEST_ARTIFACT_DIR
cd "$root"
python3 -I "$here/stage.py" check --root "$root"; python3 -I "$here/docs.py" check --root "$root"
bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🌱️incremental/🟦️.ts
bun test --timeout 300000 --reporter=junit --reporter-outfile="$out/incremental-suites.xml" $(tr '\n' ' ' < "$here/suites.txt") > "$out/incremental-suites.out" 2>&1 || true
failed=$(python3 -I "$here/junit-set.py" "$out/incremental-suites.xml" | grep -c '^FAIL' || true)
python3 -I "$here/junit-set.py" "$out/incremental-suites.xml" | head -1
if [ "$failed" = "0" ]; then echo "suites: all green"; else echo "suites: $failed failures"; exit 1; fi
