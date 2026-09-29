#!/bin/zsh
# ⚖️ LW1 → S19 laws (T3 norm-examples/args/cleanup/assets + flow-extensions), each in S19's prescribed form, on the live tree.
R=/Users/ueli/Documents/semio
ENGINE="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
V="$R/node_modules/.bin/vitest"
cargo test --offline --no-fail-fast -p semio-s-plugin-norm -p semio-s-artifact-norm-en1998 --lib; echo "LW1-STEP norm-lib rc=$?"
RUST_MIN_STACK=33554432 cargo test --offline --no-fail-fast -p semio-s-artifact-flow-flow --lib --test set-contributions-registry; echo "LW1-STEP flow-lib-registry rc=$?"
export SEMIO_TEST_LEVEL=standard
cd "$R/🧰️framework/🔨️modules/🎠️kernel" && "$V" run --config "🧪️tests/🎚️config/🟦️.ts" scope-contributions; echo "LW1-STEP vitest-kernel-scope-contributions rc=$?"
cd "$R/$ENGINE" && "$V" run --config "../../🧪️tests/🎚️config/🟦️.ts" contributions-push window-fault wgpu-extension-dispatch spawned-program-session; echo "LW1-STEP vitest-engine-contributions rc=$?"
