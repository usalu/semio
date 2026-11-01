#!/bin/bash
D=$(dirname "$0")
export MAN=Cargo.toml
"$D/t.sh" registry3 -p semio-framework-os-kernel-neural-engine-registry
"$D/t.sh" budgeted3 -p semio-framework-os-kernel-neural-engine-budgeted
"$D/t.sh" model3 -p semio-framework-os-kernel-neural-engine-model
"$D/t.sh" engine1 -p semio-framework-os-kernel-neural-engine
touch "$D/../../🗑️generated/1101/coretest-run3.done"
