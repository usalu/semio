#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/store-split-x"
bash "$C/apps-a-tools/retry.sh" "$G/check3.cargo.log" check --offline --keep-going --message-format=short -p semio-framework-os-store-engine-close -p semio-framework-os-store-engine-member-group -p semio-framework-os-store-engine -p semio-framework-os-store-space-member -p semio-framework-os-store-testing-prelude -p semio-framework-os-kernel
echo "check3 rc=$?" > "$G/chain2.rc"
