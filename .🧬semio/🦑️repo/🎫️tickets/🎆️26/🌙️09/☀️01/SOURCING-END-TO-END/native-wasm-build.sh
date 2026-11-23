#!/bin/sh
# 🦀️ Builds (profile selectors = renderCargoProfileConfigurationV1 for a prototype wasm32-wasip2 component, minus host-only package overrides)
# 🦀️ Builds the sourcing closure components (wasm32-wasip2, prototype profile) natively with the pinned nightly, bypassing the repo graph.
# Usage: [ONLY="pkg ..."] [ONLY_HUB=1] [JOBS=N] native-wasm-build.sh   (run from anywhere; sequential, one cargo at a time)
# @see ./🧩️native-activation-2026-11.md
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../../../.." && pwd)
ticket="$root/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"
TC=$(ls -d "$root"/.cache/b/t/m/r/*/toolchains/nightly-2026-07-20-*/bin | head -1)
target="$ticket/🗑️generated/native-wasm"; jobs=${JOBS:-2}
export CARGO_INCREMENTAL=0
cd "$root"
for m in ${ONLY_HUB:+"🌎️hub/Cargo.toml:semio-hub-sourcing-component:x"} \
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.toml:semio-s-plugin-sourcing-beams-component:x" \
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.toml:semio-s-plugin-sourcing-slabs-component:x" \
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.toml:semio-s-plugin-sourcing-windows-component:x"; do
  manifest=${m%%:*}; rest=${m#*:}; pkg=${rest%%:*}
  case " ${ONLY:-$pkg} " in *" $pkg "*) ;; *) continue ;; esac
  echo "== $pkg"
  PATH="$TC:$PATH" "$TC/cargo" --config 'build.rustc-wrapper=""' --config "build.rustc=\"$TC/rustc\"" --config "build.target-dir=\"$target\"" --config "build.build-dir=\"$target/b\"" \
    --config 'profile.prototype.inherits="dev"' --config 'profile.prototype.opt-level="s"' --config 'profile.prototype.debug=0' --config 'profile.prototype.codegen-units=16' --config 'profile.prototype.incremental=false' --config 'profile.prototype.lto="thin"' --config 'profile.prototype.debug-assertions=true' --config 'profile.prototype.overflow-checks=true' --config 'profile.prototype.strip="symbols"' --config 'profile.prototype.trim-paths="object"' --config 'profile.prototype.build-override.opt-level=0' --config 'profile.prototype.build-override.debug=0' --config 'profile.prototype.build-override.codegen-units=256' --config 'profile.prototype.build-override.incremental=false' \
    build -j "$jobs" --manifest-path "$root/$manifest" -p "$pkg" --lib --target wasm32-wasip2 --profile prototype
done
