#!/bin/zsh
# ⚖️ LW1 → SDK-wide T6 laws: rows 11 (C12 SDK composition), 9 (P9 fail-closed SDK half), 6 (U6 SDK row laws), 16 B (C13 declaration
# document-schema guard), 3 (LB2 builder/host_artifact): the whole plugin SDK lib and `semio-framework` lib per process (nextest,
# RUST_MIN_STACK 128 MiB), then C12's canonical-edit digest TS oracle.
R=/Users/ueli/Documents/semio
source "$R/.tmp-ticket/wp-lw1/lw1-budget.sh"
export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true NX_DAEMON=false
cd "$R" || exit 2
step sdk-lib 1800 cargo nextest run --profile long --no-fail-fast --test-threads 4 -p semio-framework-plugin --lib
step framework-lib 600 cargo nextest run --profile long --no-fail-fast --test-threads 4 -p semio-framework --lib
cd "$R/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit" && step ts-canonical-edit-digest 180 bun -e 'import { testCanonicalEditFixtures } from "./🟦️.ts"; testCanonicalEditFixtures(); console.log("canonical-edit fixtures (incl. edit-digest-chains) ok");'
finish
