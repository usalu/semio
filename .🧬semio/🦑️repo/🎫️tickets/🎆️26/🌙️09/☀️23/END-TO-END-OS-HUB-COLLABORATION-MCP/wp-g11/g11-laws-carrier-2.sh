#!/bin/zsh
# 🧪️ G11 remaining carrier/budget laws during the rebuild (rule 30): hub rate-limit laws + the SDK transaction fixtures whose
# call sites the carrier changed; build-fleet-b, private target, through the fleet `native` mutex.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g11/target
M=/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh
date; S=$(date +%s)
zsh $M native g11 -- nice -n 15 cargo test -p semio-hub --lib --no-fail-fast -- rate every_rate_limit_class an_agent_session_is_paced 2>&1 | /usr/bin/grep -E "^test |test result|error|panicked" | head -40
echo "LAW_HUB_RC=${pipestatus[1]} secs=$(( $(date +%s)-S ))"
zsh $M native g11 -- nice -n 15 cargo test -p semio-framework-plugin --features artifact-app-testing --lib --no-fail-fast -- generation_mismatch_is_rejected_with_the_frozen_code second_prepare_while_pending_is_rejected_instance_busy a_mutating_command_while_pending_is_rejected_but_reads_still_work 2>&1 | /usr/bin/grep -E "^test |test result|error|panicked" | head -40
echo "LAW_PLUGIN_TXN_RC=${pipestatus[1]} secs=$(( $(date +%s)-S ))"; date
