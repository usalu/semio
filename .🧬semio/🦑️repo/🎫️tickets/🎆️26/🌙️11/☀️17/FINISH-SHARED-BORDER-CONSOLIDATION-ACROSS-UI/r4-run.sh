#!/bin/zsh
cd "/Users/ueli/Documents/semio/🧰️framework/🔨️modules"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI/🗑️generated"
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui"
run(){ n=$1; shift; timeout 1500 repo cargo --store=isolated --configuration=prototype -- test --offline --manifest-path "$1" --lib ${@:2} > "$T/r4n-$n.log" 2>&1; echo "== $n exit $?" >> "$T/r4n-summary.log"; grep -E "^error|test result|FAILED|panicked|failed" "$T/r4n-$n.log" | cut -c1-250 | head -25 >> "$T/r4n-summary.log"; }
run light "$P/🎯️targets/🧊️wgpu/🪶️light/📦️packages/🦀️rust/Cargo.toml"
run border "$P/🔨️modules/📏️border-presentation/📦️packages/🦀️rust/Cargo.toml"
run draw "$P/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/📦️packages/🦀️rust/Cargo.toml"
run layout "$P/🎯️targets/🧊️wgpu/🧱️layout-stack/📦️packages/🦀️rust/Cargo.toml"
run mounted "$P/🎯️targets/🧊️wgpu/📌️mounted_layout/📦️packages/🦀️rust/Cargo.toml"
run widgets "$P/🎯️targets/🧊️wgpu/🪀️widgets/📦️packages/🦀️rust/Cargo.toml"
run paint "$P/🎯️targets/🧊️wgpu/🖌️paint/📦️packages/🦀️rust/Cargo.toml"
run paintlaws "$P/🎯️targets/🧊️wgpu/🖌️paint/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml"
run hosting "$P/🎯️targets/🧊️wgpu/🧭️hosting/📦️packages/🦀️rust/Cargo.toml" --features semio-ui-wgpu-mounted-layout/testkit
run tui "$P/⌨️tui/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml"
echo ALLDONE >> "$T/r4n-summary.log"
