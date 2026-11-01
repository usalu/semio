#!/bin/bash
D=$(dirname "$0")
export MAN=Cargo.toml
"$D/t.sh" model2 -p semio-framework-os-kernel-neural-engine-model
"$D/t.sh" registry2 -p semio-framework-os-kernel-neural-engine-registry
"$D/t.sh" budgeted2 -p semio-framework-os-kernel-neural-engine-budgeted
touch "$D/../../🗑️generated/1101/coretest-run2.done"
