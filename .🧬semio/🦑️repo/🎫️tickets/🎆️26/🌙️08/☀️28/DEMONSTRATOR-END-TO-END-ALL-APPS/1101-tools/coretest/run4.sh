#!/bin/bash
D=$(dirname "$0")
MAN=Cargo.toml "$D/t.sh" budgeted4 -p semio-framework-os-kernel-neural-engine-budgeted
MAN=Cargo.toml "$D/t.sh" encode1 -p semio-framework-value-native-encoding
if [ "$(cat "$D/../../🗑️generated/1101/coretest-encode1.rc")" != 0 ]; then MAN=🧰️framework/Cargo.toml "$D/t.sh" encode2 -p semio-framework-value-native-encoding; fi
touch "$D/../../🗑️generated/1101/coretest-run4.done"
