#!/bin/zsh
# 🎚️ H13 hold 2 (items 4, 6, 3): the guest-twin populated-pair codec law (server-minted genesis id), the permanent hostile-input gate (fixture seeds, then a ten-seed fuzz range), the db reopen-storm
# laws (unit, fs, sqlite; row 2.6) and the default-feature hub suite (`os-hub:test`, row 2.2).
# usage: h13-hold-2.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
HUB="/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust"
OS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
zsh $W/h13-cargo.sh "$1-plugin-codec-law" test -p semio-framework-plugin --lib --no-fail-fast -- the_codec_table_mirrors_and_passes_through_a_populated_pair_without_aborting
zsh $W/h13-verb.sh "$1-hostile-fixture-seeds" "$HUB" hostile-input-check
zsh $W/h13-verb.sh "$1-hostile-fuzz-100-109" "$HUB" hostile-input-check --seed-range 100..109 --locale de
zsh $W/h13-verb.sh "$1-reopen-storm-all" "$OS" reopen-storm-check all
zsh $W/h13-cargo.sh "$1-hub-default" test -p semio-hub --no-fail-fast
