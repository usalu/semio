#!/bin/zsh
# 🎚️ H13 hold 18 (session 15, after the WAL writer lease landed): the full kernel-db lib (sqlite), the full
# `os-hub:test-all-features` suite, and the all-driver os-hub (sqlite + postgres + neo4j) copied (rm + cp + codesign) to
# s14-h13-bin for the live gates on t6. Tolerated: the load-bound throughput wall-ratio laws only.
# usage: zsh 📜️fleet-mutex.sh native h13 -- zsh h13-hold-18.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-logs"
T="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-target"
TOLERATED='db_engine::throughput_tests::(fs|sqlite)_commits_and_reopen_storms_stay_within_their_throughput_bounds'
zsh $W/h13-cargo.sh "$1-db-lib" test -p semio-framework-os-kernel-db --locked --features sqlite --lib --no-fail-fast
failed=$(/usr/bin/grep -aE '^test .* FAILED$' "$L/$1-db-lib.txt" | sed 's/^test //; s/ \.\.\. FAILED$//' | /usr/bin/grep -vE "^($TOLERATED)$")
echo "=== kernel-db lib untolerated failures: ${failed:-none}"
zsh $W/h13-cargo.sh "$1-hub-all-features" test -p semio-hub --locked --all-features --no-fail-fast
BIN="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-bin/os-hub-all-drivers-$(date +%H%M)"
zsh $W/h13-cargo.sh "$1-os-hub" build -p semio-hub --locked --bin os-hub --features postgres,neo4j || exit 1
rm -f "$BIN" && cp "$T/debug/os-hub" "$BIN" && codesign -f -s - "$BIN" && echo "binary $BIN"
