#!/bin/zsh
# 🧪️ Base for the row 6 re-proof: the same 38 lib suites on the LIVE post-round-2 tree (shared build-dir, U6 target), reds
# re-run alone — every row-6 red must be red here too.
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-u6/t4
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs
cd /Users/ueli/Documents/semio || exit 2
export RUST_MIN_STACK=67108864 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-target
args=(); for p in $(cat $T/crates.txt) semio-framework-plugin semio-s-artifact-stdio-bcf semio-s-artifact-stdio-xlsx semio-s-plugin-playbook-procedural; do args+=(-p $p); done
cargo test --offline --no-run --message-format short --lib $args 2>&1 | tee "$L/r6-base-bins-raw.txt" | /usr/bin/grep -E "^error" -A3 | head -20
/usr/bin/grep "Executable" "$L/r6-base-bins-raw.txt" | sed 's/.*(\(.*\))$/\1/' | sort -u > "$L/r6-base-bins.txt"
/usr/bin/grep -c . "$L/r6-base-bins.txt" | sed 's/^/BINARIES /'
for b in $(cat "$L/r6-base-bins.txt"); do zsh $T/suite.sh "${${b:t}%-*}" "$b"; done
echo "R6-BASE-DONE $(date +%T)"
cargo test --offline -p semio-framework --features typegen --lib 2>&1 | tee "$L/r6-framework-live.txt" | /usr/bin/grep -E "FAILED|test result|^error" | sed 's/^/FRAMEWORK-LIVE /'
echo "R6-BASE-FRAMEWORK-DONE $(date +%T)"
