#!/bin/zsh
# ⚖️ LW1 → T6 rows 13 + 11 (C12, writer/jack half): C12's runner (nextest per process, RUST_MIN_STACK 128 MiB): writer, trinity jack
# (app-assembly), trinity rewriting + plugin; jack TS laws; repo-test `🔌️mutate-jack-1` (oracle + subject). SDK + framework: block d2;
# os-kernel lib: block h.
R=/Users/ueli/Documents/semio
source "$R/.tmp-ticket/wp-lw1/lw1-budget.sh"
export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true NX_DAEMON=false
cd "$R" || exit 2
step writer-lib 1200 cargo nextest run --profile long --no-fail-fast --test-threads 4 -p semio-s-artifact-writer-writer --lib
step jack-lib 900 cargo nextest run --profile long --no-fail-fast --test-threads 4 -p semio-s-artifact-trinity-jack --features component-app-assembly --lib
step trinity-rewriting-plugin 900 cargo nextest run --profile long --no-fail-fast --test-threads 4 -p semio-s-artifact-trinity-rewriting -p semio-s-plugin-trinity --lib
J="$R/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any"
cd "$J/🧬️schema/🧪️tests/🪪️document-contract" && step ts-jack-document-contract 180 bun -e 'import { testJackDocumentContract } from "./🟦️.ts"; testJackDocumentContract(); console.log("testJackDocumentContract ok");'
cd "$J/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🧪️tests/🔬️window-config-ownership" && step ts-jack-window-config 180 bun -e 'import { testJackGraphWindowConfigOracle } from "./🟦️.ts"; testJackGraphWindowConfigOracle(); console.log("testJackGraphWindowConfigOracle ok");'
cd "$R" || exit 2
step mutate-jack-1-oracle 420 bun "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts" oracle exhaustive --case 🔌️mutate-jack-1
step mutate-jack-1-subject 900 bun "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts" subject exhaustive --case 🔌️mutate-jack-1
finish
