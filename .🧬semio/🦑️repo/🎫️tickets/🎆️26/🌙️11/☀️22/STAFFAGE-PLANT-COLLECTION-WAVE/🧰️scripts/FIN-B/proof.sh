#!/bin/sh
# 🧾 Finisher proof of PW-10 … PW-17 plants, one tool process at a time with the pinned bun: the staffage check CLI (`--file`, 9 samples per clip) and then probe.ts (schema, validateAsset, checkVolumes, silhouette gate) per asset; output in 🗑️generated/FIN-B/<id>.cli.txt and <id>.probe.txt, one summary line per asset on stdout.
# usage: sh proof.sh <asset-id> [<asset-id> ...]   (FILE_DIR=<dir with <id>/🔣️.json> proves candidates instead of catalog files)
HERE="$(cd "$(dirname "$0")" && pwd)"
K="$(cd "$HERE/../.." && pwd)"
S="$(cd "$K/../../../../../../.." && pwd)/🎓️teaching/🏛️architecture/🧍️staffage"
G="$K/🗑️generated/FIN-B"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
DIR="${FILE_DIR:-$S/🗂️catalog/🌳️vegetation}"
mkdir -p "$G"
for ID in "$@"; do
  (cd "$S" && "$BUN" 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file "$DIR/$ID/🔣️.json") > "$G/$ID.cli.txt" 2>&1
  (cd "$HERE" && "$BUN" probe.ts "$DIR/$ID/🔣️.json") > "$G/$ID.probe.txt" 2>&1
  echo "== $ID | $(head -1 "$G/$ID.cli.txt" | cut -c1-60) | $(grep -c . "$G/$ID.cli.txt") cli lines | $(grep "^VALIDATE\|^VOLUMES\|^GATE $ID\|^PROBE" "$G/$ID.probe.txt" | tr '\n' ' ')"
done
