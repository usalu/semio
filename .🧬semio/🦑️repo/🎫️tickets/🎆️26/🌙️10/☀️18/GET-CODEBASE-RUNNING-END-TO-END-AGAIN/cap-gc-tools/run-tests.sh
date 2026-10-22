#!/bin/sh
# 🧪️ Runs the cache-gc test files one process at a time (niced) under a root (a patched mirror or the live checkout); usage: run-tests.sh <root> <log-dir>
root="$1"; out="$2"; mkdir -p "$out"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
cd "$root" || exit 2
for f in $(cd "$root" && find -L "$L/⚡️caching/💾️storage/🧾️ledger" "$L/🏗️build/🧹️cache-gc" "$L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members" -name '*.ts' -path '*🧪️tests*' | sort); do
  case "$f" in */fixtures/*|*🧫️*) continue;; esac
  name=$(echo "$f" | md5 -q 2>/dev/null || echo "$f" | shasum | cut -c1-12)
  printf '%s\n' "$f" >> "$out/index.txt"
  nice -n 10 bun test "./$f" > "$out/$name.log" 2>&1
  printf 'exit=%s %s\n' "$?" "$(/usr/bin/grep -E '^ *[0-9]+ (pass|fail)' "$out/$name.log" | tr '\n' ' ')" >> "$out/index.txt"
done
