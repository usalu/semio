#!/bin/bash
D=$(dirname "$0"); export MAN=🧰️framework/Cargo.toml
"$D/t.sh" e5-2d-booleans -p semio-framework-2d-booleans
"$D/t.sh" e5-2d-trace -p semio-framework-2d-trace
"$D/t.sh" e5-2d-path-stroke -p semio-framework-2d-path-stroke
"$D/t.sh" e5-pixels-tests -p semio-framework-pixels-tests -- --skip blocking_decode --skip encoded_orientation --skip truncated_fixture_prefixes
echo done > "$D/../../🗑️generated/1101/p2d-e5.done"
