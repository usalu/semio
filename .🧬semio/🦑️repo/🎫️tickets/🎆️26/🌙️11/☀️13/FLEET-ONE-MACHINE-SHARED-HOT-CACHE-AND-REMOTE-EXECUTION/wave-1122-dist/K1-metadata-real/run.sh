#!/bin/sh
# usage: run.sh <scratch> <label> <depth>
# One copy of the crates (k1dep, k1app) lives in <scratch>/shared; each label is a root package in its own directory <depth> levels deep that depends on k1app by path,
# so real Cargo derives another -C metadata and -C extra-filename for the very same crates in every workspace; the build goes through the unit-cache wrapper of one shared cache.
set -e
S="$1"; L="$2"; DEPTH="$3"; H="$(cd "$(dirname "$0")" && pwd)"
TC=/Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin
BUN=$(sed -n "s/^exec '\([^']*\)'.*/\1/p" /Users/ueli/Documents/semio/.cache/b/r/m/u/bin/rustc-wrapper-2562c3884f0fca5c)
SCRIPT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts"
[ -d "$S/shared" ] || { mkdir -p "$S/shared"; cp -R "$H/src/dep" "$H/src/app" "$S/shared/"; }
W="$S"; i=0; while [ $i -lt "$DEPTH" ]; do W="$W/d$i"; i=$((i+1)); done; W="$W/ws-$L"; rm -rf "$W"; mkdir -p "$W/src"
REL=$(python3 -c "import os,sys;print(os.path.relpath(sys.argv[1],sys.argv[2]))" "$S/shared/app" "$W")
printf '[package]\nname = "k1root%s"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\nk1app = { path = "%s" }\n[profile.dev]\ntrim-paths = "all"\n' "$L" "$REL" > "$W/Cargo.toml"
echo '' > "$W/src/lib.rs"
printf '#!/bin/sh\nexec "%s" "%s" "$@"\n' "$BUN" "$SCRIPT" > "$S/wrapper.sh"; chmod +x "$S/wrapper.sh"
cd "$W"
SEMIO_UNIT_CACHE_ROOT="$S/cache" SEMIO_UNIT_CACHE_LANE=d SEMIO_UNIT_CACHE_INNER="[\"$TC/bin/rustc\"]" SEMIO_UNIT_CACHE_TOOLCHAIN=k1-real SEMIO_CAS_ROOT="$S/cas" \
RUSTC_WRAPPER="$S/wrapper.sh" RUSTUP_TOOLCHAIN=nightly-2026-07-20 CARGO_TARGET_DIR="$S/target-$L" \
"$TC/bin/cargo" build -Z trim-paths -p k1app --message-format=json > "$S/out-$L.json" 2> "$S/err-$L.txt"
echo "built $L"
