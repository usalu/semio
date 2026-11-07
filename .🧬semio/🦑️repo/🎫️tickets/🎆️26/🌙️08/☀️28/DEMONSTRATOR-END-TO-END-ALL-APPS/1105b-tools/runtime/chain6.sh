#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
while [ ! -f "$C/🗑️generated/1105b/W-runtime-chk3.rc" ]; do sleep 20; done
"$C/1105b-tools/chk.sh" W-runtime-chk4 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/📦️packages/🦀️rust/Cargo.toml" wasm -p semio-plugin-retained-command
"$C/1105b-tools/chk.sh" W-runtime-chk5 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/📦️packages/🦀️rust/Cargo.toml" native -p semio-plugin-retained-command --features artifact-app-testing
"$C/1105b-tools/runtime/tst.sh" W-runtime-own2 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/🧱️owner/📦️packages/🦀️rust/Cargo.toml" native -p semio-plugin-ownership --lib -- terminal_frame_guard
"$C/1105b-tools/runtime/tst.sh" W-runtime-t20 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🫐️tests-20/📦️packages/🦀️rust/Cargo.toml" native -p semio-plugin-tests-20 --lib
"$C/1105b-tools/runtime/tst.sh" W-runtime-media2 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍊️tests-03/📦️packages/🦀️rust/Cargo.toml" native -p semio-plugin-tests-03 --lib -- media_unready_cancellation_issuer_renewal
"$C/1105b-tools/runtime/tst.sh" W-runtime-t01 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍎️tests-01/📦️packages/🦀️rust/Cargo.toml" native -p semio-plugin-tests-01 --lib -- original_media_command_custody
