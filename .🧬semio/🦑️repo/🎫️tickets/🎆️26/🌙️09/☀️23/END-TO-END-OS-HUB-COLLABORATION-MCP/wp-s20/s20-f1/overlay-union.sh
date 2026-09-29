#!/bin/zsh
# 🧾️ S20 faults overlay: the row-12 native proof = L1's train union (228 crates + 59 features, `T6R3a-native-*.txt` or a
# later round's copy) as ONE `check --keep-going --lib --tests` inside the overlay (private build-dir, overlay lane).
# usage: zsh overlay-union.sh <tag> [<round>]   → log under `.🧬semio/🌐hub/s14-s20-overlay-build/logs/`
tag="$1"; round="${2:-T6R3a}"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-l1-logs"
crates=(${(f)"$(/usr/bin/grep -v '^#' "$L/$round-native-crates.txt" | /usr/bin/grep -v '^ *$')"})
features=(${(f)"$(/usr/bin/grep -v '^#' "$L/$round-native-features.txt" | /usr/bin/grep -v '^ *$')"})
args=(); for c in $crates; do args+=(-p "$c"); done
exec zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-s20/s20-f1/overlay-cargo.sh "$tag" check --offline --keep-going --message-format short --lib --tests $args --features "${(j:,:)features}"
