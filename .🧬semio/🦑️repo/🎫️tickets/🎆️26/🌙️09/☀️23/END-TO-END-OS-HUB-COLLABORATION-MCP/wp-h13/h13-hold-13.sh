#!/bin/zsh
# 🎚️ H13 hold 13 (session 14c, P1 live): declared-batch admission — kernel-db + semio-hub compile-atomic checks, the engine /
# refusal / hub laws, then the full kernel-db lib and `os-hub:test-all-features`, then the all-driver os-hub for the live re-probe.
# usage: h13-hold-13.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
zsh $W/h13-cargo.sh "$1-check-db" check -p semio-framework-os-kernel-db --features sqlite --lib --tests || exit 1
zsh $W/h13-cargo.sh "$1-check-hub" check -p semio-hub --all-features --lib --bins --tests || exit 1
zsh $W/h13-cargo.sh "$1-engine-law" test -p semio-framework-os-kernel-db --features sqlite --lib --no-fail-fast -- a_declared_maximal_batch_commits_through_the_database artifact_submit_item_cap_plus_one
zsh $W/h13-cargo.sh "$1-hub-laws" test -p semio-hub --all-features --no-fail-fast -- a_batch_beyond_its_declaration_is_refused_permanently the_batch_limit_refusal_is_the_declared_schema_message the_transient_apply_refusal_is_the_declared_schema_message a_document_socket_admits_a_declared_maximal_batch a_transiently_refused_batch_names_the_declared_resend_code
zsh $W/h13-cargo.sh "$1-db-lib" test -p semio-framework-os-kernel-db --features sqlite --lib --no-fail-fast
zsh $W/h13-cargo.sh "$1-hub-all-features" test -p semio-hub --all-features --no-fail-fast
BIN="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-bin/os-hub-all-drivers-$(date +%H%M)"
zsh $W/h13-cargo.sh "$1-os-hub" build -p semio-hub --bin os-hub --features postgres,neo4j || exit 1
rm -f "$BIN" && cp "$W/target/debug/os-hub" "$BIN" && codesign -f -s - "$BIN" && echo "binary $BIN"
