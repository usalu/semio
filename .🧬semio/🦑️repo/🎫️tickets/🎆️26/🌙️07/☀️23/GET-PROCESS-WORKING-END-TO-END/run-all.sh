#!/bin/zsh
# 🧪️ Runs the frame-per-record verification set one job at a time on the private build dir.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
F="/Users/ueli/Documents/semio/🧰️framework/Cargo.toml"
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/📦️packages/🦀️rust/Cargo.toml"
$T/retry2.sh m1 test -p semio-s-artifact-process-process3d-examples -- --test-threads=1
$T/retry2.sh m2 test -p semio-s-artifact-process-process3d-editor --test process3d_bearbeiten_pane_smoke -- --test-threads=1
$T/retry2.sh m3 test --manifest-path $F --no-fail-fast -p semio-framework-value-native-receiving-frame -p semio-framework-value-native-owned -p semio-framework-value-native-receiving-original
$T/retry2.sh m4 test --manifest-path $F -p semio-framework-job --features testkit --no-fail-fast -- --test-threads=1
$T/retry2.sh m5 test --manifest-path $R --no-fail-fast
