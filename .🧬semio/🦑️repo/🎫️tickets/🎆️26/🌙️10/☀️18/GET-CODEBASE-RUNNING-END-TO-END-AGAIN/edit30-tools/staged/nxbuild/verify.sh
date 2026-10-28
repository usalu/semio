#!/bin/sh
# 📋 Laws of batch `nxbuild`: sh verify.sh <repository or mirror root>
# 1. the Nextest metadata law (build plan shape; real Nextest 0.9.x builds from the planned metadata with only `cargo test --no-run` and lists the same binaries as from its own metadata; wiring) and the step-admission law (the new build step is admitted, Python oracle);
# 2. nxmeta's 29 capture/testing/launcher/step-admission suites: no failure outside `known-failures.txt` (the pre-existing envelope set, identical on base).
# Rebase after drift: python3 -I edit.py <fresh root> apply && python3 -I edit_law.py <fresh root> apply, then make-stage.
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}; out="$here/../../../🗑️generated/edit30/nx-verify"; mkdir -p "$out"
cd "$root" || exit 2
python3 -I "$here/stage.py" check --root "$root" | grep '^base' && { echo "VERIFY FAIL: batch not applied"; exit 1; }
SEMIO_TEST_ARTIFACT_DIR="$out" bun test --timeout 300000 ./🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/♻️cargo-metadata/🟦️.ts ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/🪜️steps/🟦️.ts || { echo "VERIFY FAIL: law"; exit 1; }
SEMIO_TEST_ARTIFACT_DIR="$out" bun test --timeout 300000 --reporter=junit --reporter-outfile="$out/suites.xml" $(tr '\n' ' ' < "$here/suites.txt") > "$out/suites.out" 2>&1
python3 -I "$here/../freshmtime/junit-set.py" "$out/suites.xml" > "$out/suites.set"; head -1 "$out/suites.set"
new=$(grep '^FAIL' "$out/suites.set" | sed 's/^FAIL //' | sort | comm -13 "$here/known-failures.txt" -)
if [ -z "$new" ]; then echo "VERIFY OK"; else echo "VERIFY FAIL: new failures:"; echo "$new"; exit 1; fi
