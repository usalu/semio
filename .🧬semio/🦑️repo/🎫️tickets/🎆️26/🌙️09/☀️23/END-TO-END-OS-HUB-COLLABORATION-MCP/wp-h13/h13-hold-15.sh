#!/bin/zsh
# 🎚️ H13 hold 15 (session 14c, after the 22:42 reboot): semio-hub check with the typed 429 body, the auth + bin rate-limit laws,
# then hold 14's laws again (engine declared-batch law in its own process, transport-refill kernel + os-mcp laws).
# usage: h13-hold-15.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
zsh $W/h13-cargo.sh "$1-check-hub" check -p semio-hub --all-features --lib --bins --tests || exit 1
zsh $W/h13-cargo.sh "$1-rate-laws" test -p semio-hub --all-features --no-fail-fast -- the_rate_limit_refusal_is_the_declared_schema_body a_directory_command_burst_is_refused_with_a_typed_rate_limit_body credential_sign_in_locks_out_after_its_burst every_rate_limit_class_is_a_schema_member
zsh $W/h13-hold-14.sh "$1"
