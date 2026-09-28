#!/bin/zsh
# 🎚️ H13 hold 5 (session 14b, items 3, 4, 6): the guest-twin populated-pair codec law (Check In retire fix) and the os-mcp
# readers of the re-sealed lease corpus, the permanent hostile-input gate (fixture seeds, then a ten-seed fuzz range), the
# db reopen-storm laws (unit, fs, sqlite; row 2.6) and the default-feature hub suite (`os-hub:test`, row 2.2) on today's tree.
# usage: h13-hold-5.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
HUB="/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust"
OS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
zsh $W/h13-cargo.sh "$1-plugin-codec-law" test -p semio-framework-plugin --lib --no-fail-fast -- the_codec_table_mirrors_and_passes_through_a_populated_pair_without_aborting viewer_rejects_every_contract_mutating_verb
zsh $W/h13-cargo.sh "$1-mcp-lease-readers" test -p semio-framework-os-mcp --lib --no-fail-fast -- authenticated_hub
zsh $W/h13-verb.sh "$1-hostile-fixture-seeds" "$HUB" hostile-input-check
zsh $W/h13-verb.sh "$1-hostile-fuzz-100-109" "$HUB" hostile-input-check --seed-range 100..109 --locale de
zsh $W/h13-verb.sh "$1-reopen-storm-all" "$OS" reopen-storm-check all
zsh $W/h13-cargo.sh "$1-hub-default" test -p semio-hub --no-fail-fast
