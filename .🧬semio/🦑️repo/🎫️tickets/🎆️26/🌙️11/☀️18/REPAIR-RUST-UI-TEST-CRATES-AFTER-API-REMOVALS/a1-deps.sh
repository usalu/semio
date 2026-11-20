#!/bin/zsh
# usage: a1-deps.sh <round>; runs the fixture-dependent crates two at a time through one.sh, tags a1-dep<round>-<n>
T="${0:A:h}"; r=$1
m=(
"🔨️modules/🌱️value/🧪️testing/📥️admission/📦️packages/🦀️rust/Cargo.toml"
"🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/📦️packages/🦀️rust/Cargo.toml"
"🔨️modules/🌱️value/🛬️decode/🧳️owned/📦️packages/🦀️rust/Cargo.toml"
"🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🎟️reservation/📦️packages/🦀️rust/Cargo.toml"
"🔨️modules/🌱️value/📋️list/🔺️edit/📦️packages/🦀️rust/Cargo.toml"
"🔨️modules/⏯️tool-run/🎞️tick/📦️packages/🦀️rust/Cargo.toml"
"🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml"
"🔨️modules/🎠️kernel/🖼️ui/📦️packages/🦀️rust/Cargo.toml"
"🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/Cargo.toml"
"🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📦️packages/🦀️rust/Cargo.toml"
)
i=0; for x in $m; do i=$((i+1)); "$T/one.sh" "a1-dep$r-$i" "$x" > /dev/null 2>&1 & if (( i % 2 == 0 )); then wait; fi; done; wait
for j in $(seq 1 $i); do echo "== $j ${m[$j]}"; grep -E '^test result|^error(\[|:)' "$T/🗑️generated/a1-dep$r-$j.log" | sort | uniq -c | head -8; tail -1 "$T/🗑️generated/a1-dep$r-$j.log"; done
