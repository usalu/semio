#!/usr/bin/env bash
# 📦 Validate a built site folder, package it with its SHA256SUMS manifest as a UTC-timestamped tar.gz
# Usage: package-site.sh <site-folder> <output-dir> <accepted-manifest>
#   <site-folder>  built site root, must contain index.html, CNAME and .nojekyll
#   <accepted-manifest> must come from the queue-verified static acceptance run
# Refs: https://www.gnu.org/software/bash/manual/ https://www.gnu.org/software/coreutils/manual/
set -euo pipefail

TICKET="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SITE="$(cd "${1:?usage: package-site.sh <site-folder> <output-dir> <accepted-manifest>}" && pwd)"
OUT_DIR="${2:?output directory required}"
ACCEPTED="${3:?accepted manifest required}"
REPO_ROOT="${REPO_ROOT:-$(cd "$TICKET/../../../../../../.." && pwd)}"
python_tool() {
  if [[ -n "${PACKAGE_PYTHON:-}" ]]; then "$PACKAGE_PYTHON" "$@"; else repo tools exec python -- "$@"; fi
}
if [[ ! -f "$ACCEPTED" ]]; then
  echo "accepted manifest missing: $ACCEPTED" >&2
  echo "it is written only by: VERIFY_QUEUE_RECEIPTS=1 bash $TICKET/native-site-verify.sh, after every roster module is confirmed and the static matrix passed." >&2
  python_tool "$TICKET/release-receipts.py" status "$REPO_ROOT/.cache/b/p/m/browser/release/modules" "$TICKET/🗑️generated/confirmed" >&2 || true
  exit 1
fi
for required in index.html CNAME .nojekyll; do
  [[ -f "$SITE/$required" ]] || { echo "missing $required in $SITE" >&2; exit 1; }
done
python_tool "$TICKET/validate-local-references.py" "$SITE"
mkdir -p "$TICKET/🗑️generated"
scratch="$(mktemp -d "$TICKET/🗑️generated/package.XXXXXXXX")"
temporary=""
cleanup() {
  [[ -z "$temporary" ]] || rm -f "$temporary"
  rm -f "$scratch/SHA256SUMS"
  rmdir "$scratch"
}
trap cleanup EXIT
manifest="$scratch/SHA256SUMS"
python_tool "$TICKET/site-manifest.py" "$SITE" "$REPO_ROOT" "$manifest"
cmp "$ACCEPTED" "$manifest"

mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"
archive="$OUT_DIR/site-$(date -u +%Y%m%dT%H%M%SZ).tar.gz"
temporary="$(mktemp "$OUT_DIR/.site-XXXXXXXX")"
(
  cd "$SITE"
  shopt -s dotglob nullglob
  entries=()
  for entry in ./*; do
    [[ "$entry" == ./SHA256SUMS ]] || entries+=("$entry")
  done
  COPYFILE_DISABLE=1 tar -czf "$temporary" "${entries[@]}" -C "$scratch" ./SHA256SUMS
)
python_tool "$TICKET/verify-release-archive.py" "$temporary" "$ACCEPTED"
mv "$temporary" "$archive"
printf 'archive: %s\n' "$archive"
