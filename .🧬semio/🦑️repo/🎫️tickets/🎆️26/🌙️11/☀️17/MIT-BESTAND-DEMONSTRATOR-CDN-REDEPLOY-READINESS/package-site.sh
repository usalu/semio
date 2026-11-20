#!/usr/bin/env bash
# 📦 Validate a built site folder, write its SHA256SUMS manifest and package it as a UTC-timestamped tar.gz
# Usage: package-site.sh <site-folder> [output-dir]
#   <site-folder>  built site root, must contain index.html, CNAME and .nojekyll
#   [output-dir]   defaults to <ticket>/🗑️generated
# Refs: https://www.gnu.org/software/bash/manual/ https://www.gnu.org/software/coreutils/manual/
set -euo pipefail

TICKET="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SITE="$(cd "${1:?usage: package-site.sh <site-folder> [output-dir]}" && pwd)"
OUT_DIR="${2:-$TICKET/🗑️generated}"
REPO_ROOT="${REPO_ROOT:-$(git -C "$SITE" rev-parse --show-toplevel 2>/dev/null || printf '%s' "$SITE")}"
errors=0

fail() { printf 'ERROR: %s\n' "$*" >&2; errors=$((errors + 1)); }
sha() { if command -v sha256sum >/dev/null 2>&1; then sha256sum "$@"; else shasum -a 256 "$@"; fi; }
list_files() { (cd "$SITE" && find . -type f ! -path './SHA256SUMS' | sed 's|^\./||' | LC_ALL=C sort); }

for required in index.html CNAME .nojekyll; do
  [[ -f "$SITE/$required" ]] || fail "missing $required in $SITE"
done

if ! repo tools exec python -- "$TICKET/validate-local-references.py" "$SITE"; then
  fail "unaudited local references found or scan failed"
fi

count=0
total=0
sizes=""
while IFS= read -r rel; do
  full="$SITE/$rel"
  repo_rel="${full#"$REPO_ROOT"/}"
  (( ${#repo_rel} <= 256 )) || fail "path longer than 256 chars: $repo_rel"
  size="$(wc -c < "$full" | tr -d ' ')"
  total=$((total + size))
  count=$((count + 1))
  sizes+="$size $rel"$'\n'
done < <(list_files)

printf 'files: %s\ntotal bytes: %s\nlargest files (bytes path):\n' "$count" "$total"
printf '%s' "$sizes" | sort -rn | sed -n '1,10p'

if (( errors > 0 )); then
  printf 'validation failed with %s error(s)\n' "$errors" >&2
  exit 1
fi

(cd "$SITE" && list_files | while IFS= read -r rel; do sha "$rel"; done > SHA256SUMS)
printf 'manifest: %s\n' "$SITE/SHA256SUMS"

mkdir -p "$OUT_DIR"
archive="$OUT_DIR/site-$(date -u +%Y%m%dT%H%M%SZ).tar.gz"
temporary="$(mktemp "$OUT_DIR/.site-XXXXXXXX")"
trap 'rm -f "$temporary"' EXIT
COPYFILE_DISABLE=1 tar -czf "$temporary" -C "$SITE" .
repo tools exec python -- "$TICKET/verify-release-archive.py" "$temporary"
mv "$temporary" "$archive"
printf 'archive: %s\n' "$archive"
