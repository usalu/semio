#!/bin/bash
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR"; S="$T/g5-g7-retry.sh"; M="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/Cargo.toml"
bash $S vo-stores test --offline -p semio-plugin-store-stage-laws --lib original_
bash $S vo-surface test --offline -p semio-plugin-surface-fixture --lib natural_file_
bash $S vo-reactor test --offline -p semio-plugin --lib --features artifact-app-testing,original-actor-context-testing original_guest_failure_debit_
bash $S vo-job test --offline -p semio-plugin-job --lib natural_
bash $S vo-instance-jobs test --offline -p semio-plugin-instance-jobs --lib
bash $S vo-instance-render test --offline -p semio-plugin-instance-render --lib
bash $S vo-instance-natural-publication test --offline -p semio-plugin-instance-natural-publication --lib
bash $S vo-route test --offline --manifest-path "$M" -p semio-s-artifact-stdio-pdf-viewer --lib natural_file_route
echo ALLDONE > $T/🗑️generated/vo-chain.done
