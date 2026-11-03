#!/usr/bin/env bash
set -u
cd /c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS || exit 1
OUT=./🗑️generated/ct
mkdir -p "$OUT"
echo "== non-source files outside generated (all depths)" > "$OUT/inv-nonsource.txt"
find . -path ./🗑️generated -prune -o -type f -print | grep -Ev '\.(md|py|sh|mjs|ts|tsx|cjs|ps1|feature)$' | sort | while IFS= read -r f; do printf '%s\t%s\n' "$(stat -c %s "$f")" "$f"; done >> "$OUT/inv-nonsource.txt"
echo "== md files outside generated (keep, count)" > "$OUT/inv-md-count.txt"
find . -path ./🗑️generated -prune -o -type f -name '*.md' -print | wc -l >> "$OUT/inv-md-count.txt"
echo "== empty dirs outside generated" > "$OUT/inv-empty-dirs.txt"
find . -path ./🗑️generated -prune -o -type d -empty -print | sort >> "$OUT/inv-empty-dirs.txt"
echo "== __pycache__ / node_modules / .vite dirs outside generated" > "$OUT/inv-caches.txt"
find . -path ./🗑️generated -prune -o -type d \( -name __pycache__ -o -name node_modules -o -name .vite -o -name .pytest_cache \) -print | sort >> "$OUT/inv-caches.txt"
echo "== tool folders (🧰️*) with file counts and sizes" > "$OUT/inv-tool-folders.txt"
for d in ./🧰️*/; do printf '%s\t%s\t%s\n' "$(find "$d" -type f | wc -l)" "$(du -sk "$d" | cut -f1)" "$d"; done | sort -k3 >> "$OUT/inv-tool-folders.txt"
echo "== generated size" > "$OUT/inv-generated.txt"
du -sk ./🗑️generated >> "$OUT/inv-generated.txt"
find ./🗑️generated -maxdepth 1 -mindepth 1 -print | sort >> "$OUT/inv-generated.txt"
