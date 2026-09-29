#!/bin/zsh
# 🔬️ Re-runs one red of the stdio red count (existing lib binary, crate cwd, backtrace): red1.sh <crate_ident> <test path> [binary-override]
ROOT=/Users/ueli/Documents/semio
crate=$1; test=$2
bin=${3:-$(/usr/bin/grep -o "(\.🧬semio[^)]*/${crate}-[0-9a-f]*)" $ROOT/.🧬semio/🌐hub/s14-u6-logs/t4-stdio-redcount-raw.txt | tr -d '()' | head -1)}
dir=$(/usr/bin/grep "^${crate}	" /private/tmp/claude-501/-Users-ueli-Documents-semio/9bb2d328-ac37-4388-a429-b069582cbcf9/scratchpad/crate-dirs.txt | cut -f2)
cd "$ROOT/$dir" && RUST_MIN_STACK=67108864 RUST_BACKTRACE=1 "$ROOT/$bin" --exact "$test" --nocapture --test-threads 1 2>&1
