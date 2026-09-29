#!/bin/zsh
# ⚖️ H14 session 15 hold A (native lane): applies h14-creation-rule.py (hub creation rule + hosted identities + shared fixture),
# checks semio-hub lib/bins/tests, runs the trusted_catalog lib laws and the whole os-hub bin; reverts when the check or one of
# the three new/changed laws is red.
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-target" W NX_DAEMON=false
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-logs"
python3 $W/h14-creation-rule.py --write || { echo "=== APPLY FAILED"; exit 3; }
nice -n 15 cargo check -p semio-hub --lib --bins --tests > "$L/s15a-check.txt" 2>&1; rc=$?; echo "=== CHECK EXIT $rc $(date +%T)"
/usr/bin/grep -E "^(error|warning)" "$L/s15a-check.txt" | sort | uniq -c | head -20
if [ "$rc" != 0 ]; then python3 $W/h14-creation-rule.py --revert; echo "=== REVERTED (check)"; exit 1; fi
nice -n 15 cargo test -p semio-hub --lib --no-fail-fast -- trusted_catalog > "$L/s15a-catalog.txt" 2>&1; echo "=== CATALOG EXIT $? $(date +%T)"
/usr/bin/grep -E "^test result|FAILED|panicked" "$L/s15a-catalog.txt" | head -20
nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast > "$L/s15a-bin.txt" 2>&1; echo "=== BIN EXIT $? $(date +%T)"
/usr/bin/grep -E "^test result|FAILED|panicked" "$L/s15a-bin.txt" | head -20
if /usr/bin/grep -qE "(the_most_general_dialect_rule_answers_the_shared_fixture|a_hosted_multi_subset_kind_is_created_and_executed_by_its_hosts_most_general_editor|creation_prefers_the_owners_editor_over_a_hosts) \.\.\. (FAILED|ok)" "$L/s15a-catalog.txt" && ! /usr/bin/grep -qE "(the_most_general_dialect_rule_answers_the_shared_fixture|a_hosted_multi_subset_kind_is_created_and_executed_by_its_hosts_most_general_editor|creation_prefers_the_owners_editor_over_a_hosts) \.\.\. FAILED" "$L/s15a-catalog.txt"; then echo "=== NEW LAWS GREEN"; else python3 $W/h14-creation-rule.py --revert; echo "=== REVERTED (laws)"; fi
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
