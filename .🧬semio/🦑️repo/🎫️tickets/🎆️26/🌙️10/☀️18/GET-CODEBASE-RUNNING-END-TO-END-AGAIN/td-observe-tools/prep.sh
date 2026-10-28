#!/bin/sh
# 🏗️ [td-observe] creates the lane mirror under the short home /private/tmp/sm (the default $TMPDIR home adds 35 path units and pushes inference block staging paths over 256), then clones what the import closure misses: the pinned Nx SDK installation (genuine `setup-sdk` births and the Node-child project laws) and the data files of the runtime module (`🦀️.rs` of the native addon). usage: prep.sh <name>
C="$(cd "$(dirname "$0")/.." && pwd)"; L=/Users/ueli/Documents/semio; export SEMIO_MIRROR_HOME=/private/tmp/sm; name="$1"
python3 -I "$C/mirror-tools/mirror.py" create "$name" 2>&1 | grep -E "files from|cloned in|longest" | cut -c1-160
M="$(cd "$SEMIO_MIRROR_HOME/$name" && pwd -P)"; ID=$(basename "$(readlink "$L/.nx/installation")")
mkdir -p "$M/.cache/b/t/m/nx" "$M/.nx" "$M/.tmp/to" && cp -cR "$L/.cache/b/t/m/nx/$ID" "$M/.cache/b/t/m/nx/$ID" && ln -sfn "$M/.cache/b/t/m/nx/$ID" "$M/.nx/installation"
(cd "$L/🧰️framework/🔨️modules/⚙️runtime" && git ls-files . | while IFS= read -r f; do t="$M/🧰️framework/🔨️modules/⚙️runtime/$f"; [ -e "$t" ] || { mkdir -p "$(dirname "$t")"; cp -c "$f" "$t"; }; done)
echo "prepared $M (sdk $ID)"
