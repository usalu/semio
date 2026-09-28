#!/bin/zsh
# 🎚️ H13 hold 6 (session 14c, item 6): the permanent hostile-input and reopen-storm gates after their law runner reads
# whole lines (`runLawProcess`): hold 5 measured every law green in cargo while both verbs published `fail` on lines split
# across output chunks. Then the sqlite storm law twice more: its storm/serial ratio (0.64 in hold 5, bound 0.5) is the one
# real red.
# usage: h13-hold-6.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
HUB="/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust"
OS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
zsh $W/h13-verb.sh "$1-hostile-fixture-seeds" "$HUB" hostile-input-check
zsh $W/h13-verb.sh "$1-reopen-storm-fs-sqlite" "$OS" reopen-storm-check fs sqlite
zsh $W/h13-verb.sh "$1-reopen-storm-sqlite-2" "$OS" reopen-storm-check sqlite
zsh $W/h13-verb.sh "$1-reopen-storm-sqlite-3" "$OS" reopen-storm-check sqlite
