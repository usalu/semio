#!/bin/bash
# usage: mkoverlay.sh <overlay-root> [merged-dir]
# A verification root made only of COPIES: the live library (base snapshot) + the integrated files + the relative-import closure outside the library (copied from the live tree). Bare packages resolve through the tsconfig `paths` below (read-only, no symlink).
ov="$1"; merged="${2:-/private/tmp/ig/merged}"; here="$(cd "$(dirname "$0")" && pwd)"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
rm -rf "$ov"; mkdir -p "$ov"
rsync -a "${BASEDIR:-/private/tmp/ig/base0}/" "$ov/"
rsync -a --exclude _conflicts --exclude _report.json "$merged/" "$ov/"
entries=$(cd "$merged" && find . -type f \( -name '*.ts' -o -name '*.tsx' \) ! -path './_conflicts/*' | sed 's#^\./##')
python3 "$here/mkverify.py" "$ov" "📜️script.ts" $entries --merged "$merged" | head -3
python3 - "$ov" <<'PY'
import json,sys
opts=json.load(open("/Users/ueli/Documents/semio/tsconfig.json"))["compilerOptions"]
opts["paths"]={"*":["/Users/ueli/Documents/semio/node_modules/*"]}
json.dump({"compilerOptions":opts},open(sys.argv[1]+"/tsconfig.json","w"),indent=1)
PY
for f in bunfig.toml nx.json package.json bun.lock rust-toolchain.toml Cargo.toml .bazelversion MODULE.bazel .nxignore .cargo/config.toml .devcontainer/Dockerfile; do [ -f "/Users/ueli/Documents/semio/$f" ] && mkdir -p "$ov/$(dirname "$f")" && cp "/Users/ueli/Documents/semio/$f" "$ov/$f"; done
echo "overlay ready: $(find "$ov" -type f | wc -l) files"
