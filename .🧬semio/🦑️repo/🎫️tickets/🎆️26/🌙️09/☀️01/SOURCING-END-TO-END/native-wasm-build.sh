#!/bin/sh
# 🦀️ Builds the sourcing closure components (wasm32-wasip2, prototype configuration) through the admitted entry `repo cargo`, one cargo at a time; the capacity gate grants the jobs, the unit-cache wrapper stays active.
# Usage: [ONLY="pkg ..."] [ONLY_HUB=1] native-wasm-build.sh   (run from anywhere)
# @see ./🧩️native-activation-2026-11.md
# @see ./native-cargo.sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
for m in ${ONLY_HUB:+"🌎️hub/Cargo.toml:semio-hub-sourcing-component"} \
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.toml:semio-s-plugin-sourcing-beams-component" \
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.toml:semio-s-plugin-sourcing-slabs-component" \
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.toml:semio-s-plugin-sourcing-windows-component"; do
  manifest=${m%%:*}; pkg=${m#*:}
  case " ${ONLY:-$pkg} " in *" $pkg "*) ;; *) continue ;; esac
  echo "== $pkg"
  OUT="$here/🗑️generated/native-wasm" "$here/native-cargo.sh" "$pkg" . build --manifest-path "$manifest" -p "$pkg" --lib --target wasm32-wasip2
done
