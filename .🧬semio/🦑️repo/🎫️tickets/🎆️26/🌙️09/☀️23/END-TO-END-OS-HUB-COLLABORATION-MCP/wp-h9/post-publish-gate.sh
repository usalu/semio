#!/bin/zsh
# 🚦️ H9's first post-publish hub gate (coordinator: W2 builds the 7800 binary only after this is green).
# 1) all-driver os-hub build, 2) the full os-hub bin law suite (RW gates, refusal layer, isolation, approval answer,
# revocation both orders, directory page budget), 3) hub lib laws, 4) install the binary for the e2e runs.
# usage: post-publish-gate.sh    (foreground; logs under .🧬semio/🌐hub/s12-h9-logs/gate-*.txt)
cd /Users/ueli/Documents/semio
OUT=".🧬semio/🌐hub/s12-h9-logs"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/target RUST_MIN_STACK=268435456
echo "=== build $(date +%T)"
nice -n 15 cargo build -p semio-hub --bin os-hub --no-default-features --features sqlite,postgres,neo4j,native-artifact-execution > "$OUT/gate-build.txt" 2>&1 || { echo "BUILD RED"; /usr/bin/grep -E '^error' -A8 "$OUT/gate-build.txt" | head -40; exit 1; }
echo "=== bin laws $(date +%T)"
nice -n 15 cargo test -p semio-hub --bin os-hub --features native-artifact-execution,integration-fixtures --no-fail-fast > "$OUT/gate-bin-laws.txt" 2>&1
/usr/bin/grep -E 'test result|FAILED|panicked' "$OUT/gate-bin-laws.txt" | head -20
echo "=== lib laws $(date +%T)"
nice -n 15 cargo test -p semio-hub --lib --features native-artifact-execution,integration-fixtures --no-fail-fast > "$OUT/gate-lib-laws.txt" 2>&1
/usr/bin/grep -E 'test result|FAILED|panicked' "$OUT/gate-lib-laws.txt" | head -20
B=".🧬semio/🌐hub/s12-h9-bin"
cp .tmp-ticket/wp-h9/target/debug/os-hub "$B/os-hub.new" && codesign -s - -f "$B/os-hub.new" && mv -f "$B/os-hub.new" "$B/os-hub"
echo "=== done $(date +%T)"
