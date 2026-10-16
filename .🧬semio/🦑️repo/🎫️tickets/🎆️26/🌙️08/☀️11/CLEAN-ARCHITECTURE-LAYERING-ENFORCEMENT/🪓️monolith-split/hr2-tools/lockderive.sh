#!/bin/zsh
# usage: lockderive.sh <outdir>   (copies every non-ticket package.json + lock inputs into <outdir>/tree and derives bun's own lock per scope into <outdir>/<scope>.lock)
OUT="$1"; ROOT=/Users/ueli/Documents/semio; HERE="$(cd "$(dirname "$0")" && pwd)"
rm -rf "$OUT"; mkdir -p "$OUT/tree"
cd "$ROOT" || exit 2
( git -c core.quotepath=false ls-files '*package.json' ; git -c core.quotepath=false ls-files -o --exclude-standard '*package.json' ) 2>/dev/null | /usr/bin/grep -v '^\.🧬semio/' | sort -u > "$OUT/manifests.txt"
( cat "$OUT/manifests.txt"; bun "$HERE/wsmanifests.ts" ) | sort -u > "$OUT/manifests2.txt"; mv "$OUT/manifests2.txt" "$OUT/manifests.txt"
while IFS= read -r f; do mkdir -p "$OUT/tree/$(dirname "$f")"; cp "$f" "$OUT/tree/$f"; done < "$OUT/manifests.txt"
cp bunfig.toml "$OUT/tree/bunfig.toml" 2>/dev/null
mkdir -p "$OUT/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches"
cp 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/* "$OUT/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/"
for scope in . ✏️s 🌎️hub ♻️mit-bestand 🏢️semio-tech 🎓️teaching; do
  [ -f "$ROOT/$scope/bun.lock" ] || continue
  NAME=$([ "$scope" = "." ] && echo root || echo "$scope")
  cp "$ROOT/$scope/bun.lock" "$OUT/tree/$scope/bun.lock"
  ( cd "$OUT/tree/$scope" && perl -e 'alarm 300; exec @ARGV' -- bun install --lockfile-only --offline > "$OUT/install-$NAME.log" 2>&1; echo "exit=$?" >> "$OUT/install-$NAME.log" )
  cp "$OUT/tree/$scope/bun.lock" "$OUT/$NAME.lock"
  cp "$ROOT/$scope/bun.lock" "$OUT/tree/$scope/bun.lock"
done
echo derived
