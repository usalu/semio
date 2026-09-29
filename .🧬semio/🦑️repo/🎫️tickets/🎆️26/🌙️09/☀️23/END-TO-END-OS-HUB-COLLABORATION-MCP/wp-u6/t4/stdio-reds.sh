#!/bin/zsh
# 🧪️ Live stdio lib-test red count (shared build-dir, U6 private target): stdio-reds.sh <label> — capture `.🧬semio/🌐hub/s14-u6-logs/<label>.txt`,
# every red (FAILED line or aborted binary) in `<label>-reds.txt`. Runs under the native lane (rule 25).
ROOT=/Users/ueli/Documents/semio
L=$ROOT/.🧬semio/🌐hub/s14-u6-logs
out=$L/$1.txt
pkgs=(${(f)"$(cat $ROOT/.tmp-ticket/wp-u6/t4/stdio-crates.txt)"})
args=(); for p in $pkgs; do args+=(-p $p); done
cd $ROOT && CARGO_BUILD_BUILD_DIR=$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR=$ROOT/.🧬semio/🌐hub/s14-u6-target CARGO_BUILD_JOBS=4 \
  cargo test --offline --lib --no-fail-fast $args > $out 2>&1
echo "RC $?" >> $out
/usr/bin/grep -E "^test .* FAILED$|^error(\[|:)|fatal runtime error|SIGABRT|signal: |^test result: FAILED|Running unittests" $out > $L/$1-reds.txt
echo DONE >> $out
