#!/bin/sh
# 🦀️ Builds the sourcing closure components (wasm32-wasip2, prototype configuration) through the admitted entry `repo cargo`, one cargo at a time; the capacity gate grants the jobs, the unit-cache wrapper stays active; each final is exported out of its reclaimable lane into 🗑️generated/native-wasm.
# With NATIVE=1 it runs plain cargo instead (effective prototype profile from ./native-profile.toml, CARGO_TARGET_DIR=/Users/ueli/t/src-wasm, -j 4, no rustc wrapper).
# Usage: [NATIVE=1] [ONLY="pkg ..."] [ONLY_HUB=1] native-wasm-build.sh   (run from anywhere)
# @see ./🧩️native-activation-2026-11.md
# @see ./native-cargo.sh
# @see ./native-profile.toml
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
for m in ${ONLY_HUB:+"🌎️hub/Cargo.toml:semio-hub-sourcing-component"} \
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.toml:semio-s-plugin-sourcing-beams-component" \
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.toml:semio-s-plugin-sourcing-slabs-component" \
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.toml:semio-s-plugin-sourcing-windows-component"; do
  manifest=${m%%:*}; pkg=${m#*:}
  case " ${ONLY:-$pkg} " in *" $pkg "*) ;; *) continue ;; esac
  echo "== $pkg"
  if [ -n "${NATIVE:-}" ]; then
    mkdir -p "$here/🗑️generated/native-wasm"; tgt=/Users/ueli/t/src-wasm
    (cd "$root" && env -u RUSTC_WRAPPER CARGO_TARGET_DIR=$tgt CARGO_BUILD_BUILD_DIR=$tgt/b CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_INCREMENTAL=0 SEMIO_BUILD_MODE=prototype \
      cargo build -j 4 --config "$here/native-profile.toml" --profile prototype --manifest-path "$manifest" -p "$pkg" --lib --target wasm32-wasip2) > "$here/🗑️generated/native-wasm-$pkg.log" 2>&1 || { tail -30 "$here/🗑️generated/native-wasm-$pkg.log" >&2; exit 1; }
  else
    OUT="$here/🗑️generated/native-wasm" "$here/native-cargo.sh" "$pkg" . build --manifest-path "$manifest" -p "$pkg" --lib --target wasm32-wasip2
  fi
  base=$(printf %s "$pkg" | tr - _)
  if [ -n "${NATIVE:-}" ]; then built=/Users/ueli/t/src-wasm/wasm32-wasip2/prototype/$base.wasm; else
    built=$(ls -t "$root"/.cache/b/r/m/u/l/*/t*/wasm32-wasip2/prototype/"$base.wasm" 2>/dev/null | head -1); fi
  [ -n "$built" ] || { echo "missing $base.wasm in the repo cargo lanes" >&2; exit 1; }
  cp "$built" "$here/🗑️generated/native-wasm/$base.wasm"
  echo "exported $base.wasm ($(wc -c < "$built") bytes)"
done
