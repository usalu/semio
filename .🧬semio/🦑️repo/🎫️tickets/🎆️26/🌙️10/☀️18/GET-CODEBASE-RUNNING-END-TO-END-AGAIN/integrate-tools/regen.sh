#!/bin/bash
# usage: regen.sh <overlay-root> [merged-dir]   regenerates the files that are the OUTPUT of integrated code from the integrated code (help snapshots) and copies them into the merged tree.
ov="$1"; merged="${2:-/private/tmp/ig/merged}"; here="$(cd "$(dirname "$0")" && pwd)"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
target="$L/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json"
cp "$here/../rcache-tools/gen-help.ts" "$ov/gen-help.ts" && (cd "$ov" && bun gen-help.ts "$target") || exit 1
rm -f "$ov/gen-help.ts"
cp "$ov/$target" "$merged/$target"
echo "regenerated help snapshots: $(python3 -c "
import json,sys;d=json.load(open(sys.argv[1]));print(sum(len(v) for v in d['snapshots'].values()),'entries')" "$merged/$target")"
