#!/bin/zsh
# ⏪️ Runs the history-edit acceptance law (S2-AGNOSTIC, design §16.3) in each named plugin crate, one gated cargo at a time with two
# jobs, appending one result line per crate to 🗑️generated/s2-agnostic/acceptance-results.tsv; the full log lands beside it.
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/s2-agnostic"
for crate in "$@"; do
  features=()
  dir=$(/usr/bin/grep -l "^name = \"$crate\"" ✏️s/🔌️plugins/*/🗿️artifacts/*/📦️packages/🦀️rust/Cargo.toml | head -1)
  /usr/bin/grep -q "component-app-assembly" "$dir" && features=(--features component-app-assembly)
  until [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt 8 ]; do sleep 20; done
  start=$(date '+%T')
  CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-s2-agnostic cargo test --manifest-path ✏️s/Cargo.toml -p "$crate" "${features[@]}" --lib -j 2 --message-format=short -- history_edits_end_to_end --nocapture > "$G/acceptance-$crate.log" 2>&1
  code=$?
  summary=$( { /usr/bin/grep -E "^\[history-edit-acceptance\]|^test result|^[^ ]+: error|^error" "$G/acceptance-$crate.log"; /usr/bin/grep -A2 "panicked at" "$G/acceptance-$crate.log"; } | head -6 | tr '\n\t' '  ' | cut -c1-900)
  printf '%s\t%s\t%s\t%s\t%s\n' "$crate" "$start" "$(date '+%T')" "$code" "$summary" >> "$G/acceptance-results.tsv"
done
echo done
