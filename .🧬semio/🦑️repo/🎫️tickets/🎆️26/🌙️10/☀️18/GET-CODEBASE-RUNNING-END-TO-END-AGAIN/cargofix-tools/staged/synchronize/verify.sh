#!/bin/sh
# 🔗 Laws of batch `synchronize` after apply: VERIFY_ROOT=<repository or mirror root> SEMIO_TEST_ARTIFACT_DIR=<dir> sh verify.sh
# 1. the witness law (12 edits, Python oracle, Ajv); 2. the existing Cargo workspace/preparation suites: their failure set must equal known-failures.txt (pre-existing under bare `bun test`).
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
out=${SEMIO_TEST_ARTIFACT_DIR:?SEMIO_TEST_ARTIFACT_DIR required}; mkdir -p "$out"
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/👁️witness/🧪️tests/🟦️.ts
bun test --timeout 120000 --reporter=junit --reporter-outfile="$out/synchronize-suites.xml" $(tr '\n' ' ' < "$here/suites.txt") > "$out/synchronize-suites.out" 2>&1 || true
python3 -I "$here/junit-set.py" "$out/synchronize-suites.xml" | grep '^FAIL' > "$out/synchronize-suites.set" || true
new=$(sort "$out/synchronize-suites.set" | comm -13 "$here/known-failures.txt" -)
if [ -z "$new" ]; then echo "suites: no failure outside the $(wc -l < "$here/known-failures.txt" | tr -d ' ') pre-existing ones ($(wc -l < "$out/synchronize-suites.set" | tr -d ' ') failed now)"; else echo "suites: NEW failures:"; echo "$new"; exit 1; fi
