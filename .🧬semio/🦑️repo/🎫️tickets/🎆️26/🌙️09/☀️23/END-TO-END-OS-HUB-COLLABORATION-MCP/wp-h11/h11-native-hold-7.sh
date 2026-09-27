#!/bin/zsh
# 🔐️ H11 item 3: the db in-process `--lib` gate repeated (the flake H9 saw in 4 of 6 runs was backend-control exhaustion).
# usage: h11-native-hold-7.sh <label> <runs>
cd /Users/ueli/Documents/semio
export H11_NICE=0
for run in $(seq 1 ${2:-4}); do
  zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-h11/h11-cargo.sh "$1-db-inprocess-$run" test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-fail-fast -- --skip long:: --skip throughput_tests::
done
