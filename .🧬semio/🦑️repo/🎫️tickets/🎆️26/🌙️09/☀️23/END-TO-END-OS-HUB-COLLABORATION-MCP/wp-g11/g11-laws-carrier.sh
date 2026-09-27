#!/bin/zsh
# 🧪️ G11 carrier/budget/fault-mapping laws, filtered `--lib` tests (rule 25), build-landing + private target.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-landing" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g11/target
date; S=$(date +%s)
nice -n 10 cargo test -p semio-framework-os-mcp --lib --no-fail-fast -- a_verb_whose_lane every_fault_code an_action_that_edits_only_owned_children an_action_whose_preview_produced_no_operation preview_vs_commit 2>&1 | /usr/bin/grep -E "^test |test result|error|panicked" | head -40
echo "LAW_MCP_RC=${pipestatus[1]} secs=$(( $(date +%s)-S ))"
nice -n 10 cargo test -p semio-hub --lib --no-fail-fast -- rate every_rate_limit_class an_agent_session_is_paced 2>&1 | /usr/bin/grep -E "^test |test result|error|panicked" | head -40
echo "LAW_HUB_RC=${pipestatus[1]} secs=$(( $(date +%s)-S ))"
nice -n 10 cargo test -p semio-framework-os-kernel --lib --no-fail-fast -- transaction_prepare paged_ingress_admits_every_transaction_route app_frame_emit_round_trips channel_transaction_fixtures app_command_fixture_corpus app_frame_fixture_corpus channel_version 2>&1 | /usr/bin/grep -E "^test |test result|error|panicked" | head -40
echo "LAW_KERNEL_RC=${pipestatus[1]} secs=$(( $(date +%s)-S ))"
nice -n 10 cargo test -p semio-framework-plugin --features artifact-app-testing --lib --no-fail-fast -- an_agent_transaction_carries_owned_child_op_groups composite_gesture_produces_one_undo_group transaction_ 2>&1 | /usr/bin/grep -E "^test |test result|error|panicked" | head -60
echo "LAW_PLUGIN_RC=${pipestatus[1]} secs=$(( $(date +%s)-S ))"; date
