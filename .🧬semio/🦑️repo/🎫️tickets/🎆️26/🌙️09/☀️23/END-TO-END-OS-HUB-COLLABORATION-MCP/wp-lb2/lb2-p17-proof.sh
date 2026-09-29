#!/bin/zsh
# 🪪️ LB2 p17 scratch proof (live tree + p17): every touched crate compiles with its tests, law (f) and the whole shipped
# fleet, the stdio registry's native codec receipts (factory verification), the framework's codec-table and pack-identity
# laws, and the hashlib oracle over law (f)'s answers. Run inside the overlay lane:
# `zsh lb2-p9-scratch.sh run <capture> zsh <this> <tag> [step…]`. Captures: s14-lb2-captures/<tag>-<step>.out
setopt no_bg_nice
C="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-captures"
W="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2"
tag="$1"; shift
only=("$@")
answers="$C/$tag-codec-answers"
mkdir -p "$answers"
step() {
  local name="$1"; shift
  (( ${#only} == 0 || ${only[(Ie)$name]} )) || return 0
  echo "STEP $name START $(date '+%T')"
  "$@" > "$C/$tag-$name.out" 2>&1
  echo "STEP $name rc=$? END $(date '+%T')"
  /usr/bin/grep -E "^test result|^error(\[|:)|^packages " "$C/$tag-$name.out" | head -60
}
roots=(avi bcf binary bmp csv deflate docx dwg dxf epw gif gltf html ifc jpg json las md mp3 mp4 obj pdf ply png pptx semio step stl svg tiff tsv txt wav xlsx xml zip)
families=(image media cad bim mesh pdf office semio binary)
crate_args=(-p semio-framework-os-kernel -p semio-framework-plugin -p semio-s-plugin-stdio)
for r in $roots; do crate_args+=(-p "semio-s-artifact-stdio-$r"); done
for f in $families; do crate_args+=(-p "semio-s-plugin-stdio-$f"); done
step check cargo check --offline --keep-going --tests $crate_args
step fleet env SEMIO_STDIO_CODEC_HASH_OUT="$answers" cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --test shipped_fleet
step oracle python3 "$W/lb2-p17-pack-schema-oracle.py" "$answers" "$PWD"
step registry cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --lib
step framework cargo test --offline --no-fail-fast -p semio-framework-plugin -p semio-framework-os-kernel --lib -- pack_schema codec_calls_construct_no_app a_shared_schema_is_created
echo "PROOF END $(date '+%T')"
