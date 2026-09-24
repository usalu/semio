#!/bin/zsh
# H6 hub-mutex phase: rebuild bin tests, 200x chain bin laws, hub test quick + long, full browser-document-open-check.
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h6; G=$W/generated; R=/Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$W/target RUST_MIN_STACK=268435456
cd "$R/🌎️hub/📦️packages/🦀️rust"
echo "BUILD start $(date +%T)"
cargo test -p semio-hub --bin os-hub --no-run > $G/build-2.txt 2>&1; echo "BUILD rc=$? $(date +%T)"
exe=$(grep -o '(/[^)]*os_hub-[0-9a-f]*)' $G/build-2.txt | tail -1 | tr -d '()')
rm -f $W/bin/os_hub_test_2; cp "$exe" $W/bin/os_hub_test_2
L=(tests::document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped tests::document_open_plan_admin_revocation_invalidates_session_and_share_bindings tests::document_open_plan_receipt_exchange_admits_one_exact_bounded_secret_free_socket_grant tests::document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use tests::document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes tests::document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable tests::document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint)
rm -f $G/hangs.txt
zsh $W/loop.sh $W/bin/os_hub_test_2 200 F ${=L} > $G/loop-F.txt 2>&1; echo "LOOP200 rc=$? $(cat $G/loop-F.txt) $(date +%T)"
cd "$R"
bun nx run os-hub:test-quick --skip-nx-cache -- --no-fail-fast > $G/hub-quick-1.txt 2>&1; echo "QUICK rc=$? $(date +%T)"
bun nx run os-hub:test-long --skip-nx-cache > $G/hub-long-1.txt 2>&1; echo "LONG rc=$? $(date +%T)"
bun nx run os-hub:browser-document-open-check --skip-nx-cache > $G/browser-document-open-check-1.txt 2>&1; echo "CHAIN rc=$? $(date +%T)"
echo "PHASE DONE $(date +%T)"
