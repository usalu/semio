#!/bin/zsh
# [DEBUG] native wasm-pack web builds of the three renderer engine packages (surface, editor, node graph) into the managed product directories
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-pane/native-wasm
mkdir -p $N
R=/Users/ueli/Documents/semio
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_BUILD_JOBS=${JOBS:-3} CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0
export PATH="$R/.cache/b/r/m/cargo/home/bin:$PATH"
P=$R/.cache/b/p/m/browser/prototype/wasm-packages
build() { # owner-relative componentDir, wasmBase, hash
  local comp="$R/$1" base=$2 out=$N/out-$2
  rm -rf $out; (cd "$comp" && wasm-pack build --mode no-install --dev --target web --out-dir $out --out-name $base --no-pack --no-opt -- --locked --manifest-path "$comp/Cargo.toml" --target wasm32-unknown-unknown) || { echo "FAIL $base"; return 1; }
  echo "built $base"
}
build "🧰️framework/🔨️modules/🗺️surface/🧩️component/📦️packages/🦀️rust" framework_surface
build "🧰️framework/🔨️modules/✍️editor/🧩️component/📦️packages/🦀️rust" framework_editor
build "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧩️component/📦️packages/🦀️rust" framework_os_node_graph
echo done
