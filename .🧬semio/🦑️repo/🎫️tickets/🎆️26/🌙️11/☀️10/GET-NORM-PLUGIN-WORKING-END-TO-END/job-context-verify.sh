#!/bin/sh
cd "$(git rev-parse --show-toplevel)" || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
mkdir -p "$T/🗑️generated"
repo capacity run --memory=2663 --cpu=1 -- repo cargo --offload=never --store=shared --configuration=prototype --parallelism=1 -- check --offline --locked -p semio-framework-job-context-step -p semio-framework-job-context -p semio-framework-job -p semio-framework-job-native-turn -p semio-framework-job-headless -p semio-framework-job-testing-caller -p semio-framework-job-return > "$T/🗑️generated/verify-check.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/verify-check.log"
repo capacity run --memory=2663 --cpu=1 -- repo cargo --offload=never --store=shared --configuration=prototype --parallelism=1 -- test --offline --locked -p semio-framework-job-context --features testkit > "$T/🗑️generated/verify-test.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/verify-test.log"
