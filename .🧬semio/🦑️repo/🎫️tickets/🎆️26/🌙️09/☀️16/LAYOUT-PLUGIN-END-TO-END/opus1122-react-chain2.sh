#!/usr/bin/env bash
cd /home/deck/semio
R=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/opus1122-react-run.sh"; T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"; G="$T/🗑️generated/opus1122"
until grep -q '^exit' "$G/react2-wasm-nodegraph.log" 2>/dev/null; do sleep 20; done
for n in surface editor nodegraph; do grep -q '^exit 0' "$G/react2-wasm-$n.log" || { echo retry $n; }; done
S="🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"
grep -q '^exit 0' "$G/react2-wasm-surface.log" || "$R" wasm-surface2 "$S" @semio-tech/framework-surface-rs:wasm ./📜️script.ts wasm
"$R" fonts "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust" semio-os-infinite:fonts ./📜️script.ts fonts
until [ -n "$(ls .cache/b/p/m/browser/prototype/modules 2>/dev/null | grep -i layout)" ] || [ -f "$G/component-done" ]; do sleep 30; done
"$R" prepare "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" x ./📜️script.ts prepare layout react prototype
"$R" activate "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" x ./📜️script.ts activate layout react prototype
nohup "$R" serve "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" x ./📜️script.ts serve layout react prototype >/dev/null 2>&1 &
