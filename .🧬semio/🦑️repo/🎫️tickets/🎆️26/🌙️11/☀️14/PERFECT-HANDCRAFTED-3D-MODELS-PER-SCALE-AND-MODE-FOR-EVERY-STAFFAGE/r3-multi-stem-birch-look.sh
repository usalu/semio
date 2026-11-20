#!/bin/sh
# 👀 Round-3 multi-stem birch: the six-view sheet of the CANDIDATE document (rig and volumes as built, before anything is written to the catalog). Usage: sh r3-multi-stem-birch-look.sh --scale N [r3-sheet options]; writes under 🗑️generated/r3-sheet/vegetation/.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
mkdir -p "🗑️generated/r3-multi-stem-birch"
sed -e 's|const loaded = load(|const loaded = loadCandidate(|; s|const { entry, asset } = load(|const { entry, asset } = loadCandidate(|' r3-sheet.ts > r3-multi-stem-birch-sheet.gen.ts
cat >> r3-multi-stem-birch-sheet.gen.ts <<'T'
import { readFileSync as readCandidate } from "node:fs";
import { entryOf as entryOfCandidate } from "./volumes-lib.ts";
function loadCandidate(spec: string | undefined, _usage: string): any {
  const { entry } = entryOfCandidate(spec!);
  const asset = JSON.parse(readCandidate("🗑️generated/r3-multi-stem-birch/🔣️.json", "utf8")).asset;
  return { entry, asset, catalog: asset, source: { volumes: asset.volumes } };
}
T
"$BUN" r3-multi-stem-birch-sheet.gen.ts vegetation/multi-stem-birch "$@"
rm -f r3-multi-stem-birch-sheet.gen.ts
