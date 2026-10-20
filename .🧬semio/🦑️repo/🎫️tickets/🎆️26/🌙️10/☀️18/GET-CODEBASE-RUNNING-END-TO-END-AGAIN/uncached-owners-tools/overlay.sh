#!/bin/bash
# usage: overlay.sh   builds the scratch overlay /private/tmp/ucov: a COPY (never a link) of the inference closure, the library tree, every tracked manifest and the shared config, then applies in order the cache-fix set and this set. node_modules is read through one link (read-only).
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
OV=/private/tmp/ucov; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; I="$L/🏗️build/🕸️graph/📥️inference"
G="$C/🗑️generated/uncached-owners"; mkdir -p "$G"
chmod -R u+w "$OV" 2>/dev/null; rm -rf "$OV" /private/tmp/ucov-meta; mkdir -p "$OV" /private/tmp/ucov-meta
git ls-files '*project.json' | command grep -v '^\.🧬semio/' > "$G/pj-list.txt"
bun build --target=bun --packages=external --metafile="$G/meta.json" --outdir=/private/tmp/ucov-meta "$I/🟨️.mjs" "$I/🧪️tests/💾️cache/🟦️.ts" "$L/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts" > /dev/null 2>&1 || { echo "metafile build failed"; exit 3; }
python3 -I - "$G/meta.json" "$G/overlay-files.txt" "$G/pj-list.txt" <<'PY'
import json,sys,os,subprocess
meta=json.load(open(sys.argv[1])); files=set(meta["inputs"])
for extra in ["bunfig.toml","package.json","bun.lock","tsconfig.json","nx.json"]:
    if os.path.exists(extra): files.add(extra)
files.update(l for l in open(sys.argv[3]).read().split("\n") if l)
out=subprocess.run(["git","ls-files","-z","--","🧰️framework"],capture_output=True).stdout.decode().split("\0")
files.update(f for f in out if f)
files={f for f in files if os.path.isfile(f)}
open(sys.argv[2],"w").write("\n".join(sorted(files))+"\n"); print(len(files),"files")
PY
rsync -aL --files-from="$G/overlay-files.txt" ./ "$OV/" || exit 4
ln -sfn /Users/ueli/Documents/semio/node_modules "$OV/node_modules"
git -C "$OV" init -q && git -C "$OV" add -A >/dev/null 2>&1
SEMIO_ROOT="$OV" bun "$C/cache-fix-tools/apply-library.ts" || exit 5
SEMIO_ROOT="$OV" bun "$C/cache-fix-tools/apply-manifests.ts" | tail -1
git -C "$OV" add -A >/dev/null 2>&1
SEMIO_ROOT="$OV" bun "$C/uncached-owners-tools/apply-library.ts" || exit 6
SEMIO_ROOT="$OV" bun "$C/uncached-owners-tools/apply-manifests.ts" | tail -1
echo "overlay ready: $OV"
