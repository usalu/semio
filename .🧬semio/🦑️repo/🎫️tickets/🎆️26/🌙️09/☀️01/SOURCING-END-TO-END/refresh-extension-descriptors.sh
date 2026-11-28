#!/bin/sh
# 🛂️ Rebuilds the given extension components under the build lock, stages them, then copies each staged descriptor into the committed component folder.
# Usage: sh refresh-extension-descriptors.sh "<package> ..."   (from the workspace root)
# @see ./rebuild-sourcing-retry.sh
set -u
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
sh "$here/rebuild-sourcing-retry.sh" "$1" 4
M="$PWD/.cache/b/p/m/browser/prototype/modules"; E="$PWD/✏️s/🔌️plugins/🪵️sourcing/🧩️extensions"
for pair in "🪵️beams:🪜️sourcing-module-beams" "🧱️slabs:🧇️sourcing-module-slabs" "🪟️windows:🪟️sourcing-module-windows"; do
  d="$E/${pair%%:*}/🧩️component"; s="$M/${pair#*:}"
  cmp -s "$d/🛂️.descriptor.semio" "$s/🛂️.descriptor.semio" || { cp "$s/🛂️.descriptor.semio" "$s/🔣️.json" "$d/"; echo "refreshed $d"; }
done
