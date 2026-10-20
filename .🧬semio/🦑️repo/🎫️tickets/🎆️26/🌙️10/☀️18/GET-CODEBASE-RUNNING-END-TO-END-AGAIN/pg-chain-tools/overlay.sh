#!/bin/bash
# usage: overlay.sh [--rebuild]   builds the scratch overlay /private/tmp/pgc-ov (a COPY of the inference/plugin/process closure; short path because absolute paths are limited to 256 units) and applies the staged pg-chain set to it
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
T="$C/pg-chain-tools"
OV=/private/tmp/pgc-ov
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
if [ "$1" = "--rebuild" ]; then rm -rf "$OV"; fi
if [ ! -d "$OV/.git" ]; then
  mkdir -p "$OV"
  git ls-files -z -- "$L" "🧰️framework" ":(glob)✏️s/**/Cargo.toml" ":(glob)✏️s/**/📋️project.json" "✏️s/🧑‍💻dev/🧩️catalog" Cargo.toml rust-toolchain.toml bunfig.toml package.json tsconfig.json nx.json .gitignore | python3 -I -c "
import sys,os
for f in sys.stdin.buffer.read().split(b'\0'):
    if f and os.path.isfile(f): sys.stdout.buffer.write(f+b'\n')
" > "$OV.files"
  rsync -aL --files-from="$OV.files" ./ "$OV/" || exit 4
  mkdir -p "$OV/node_modules/@semio-tech"
  git -C "$OV" init -q
  for pkg in process plugin-registry-publication plugin-registry framework-plugin framework-os-dev schema-validator-ts nx-bootstrap-tools repo-lib framework-filesystem framework; do
    target="$(readlink node_modules/@semio-tech/$pkg)"; [ -z "$target" ] && continue; relative="${target#/Users/ueli/Documents/semio/}"
    [ -d "$OV/$relative" ] && ln -sfn "$OV/$relative" "$OV/node_modules/@semio-tech/$pkg"
  done
  python3 -I "$C/bzl-coverage-tools/nodemods.py" "$OV" ajv typescript minimatch @iarna/toml lodash
fi
python3 -I "$T/apply.py" apply --root "$OV" --write --overwrite "${@:2}"
