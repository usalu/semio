#!/bin/sh
# usage: apply.sh apply [--dry-run] [--ignore-sync]  (staged-merge entry for staged/apply.py; --dry-run runs its --check and writes nothing; root = nearest ancestor with bunfig.toml + bun.lock)
here=$(cd "$(dirname "$0")" && pwd)
[ "$1" = apply ] || { echo "usage: apply.sh apply [--dry-run] [--ignore-sync]" >&2; exit 64; }
shift
dry=0; ignore=0
for word in "$@"; do
  case $word in
    --dry-run) dry=1 ;;
    --ignore-sync) ignore=1 ;;
    *) echo "refusing unknown flag $word (this set has no rebase mode)" >&2; exit 64 ;;
  esac
done
root=$here
while [ ! -f "$root/bunfig.toml" ] || [ ! -f "$root/bun.lock" ]; do
  [ "$root" = / ] && { echo "repository root not found" >&2; exit 66; }
  root=$(dirname "$root")
done
if [ $ignore = 0 ] && ps -ww -eo command | grep -E '(^|/)bun .*dependencies (sync|lock)' | grep -v grep >/dev/null; then
  echo "refusing: a dependency sync is running" >&2
  exit 2
fi
cd "$root" || exit 66
if [ $dry = 1 ]; then exec python3 -I "$here/staged/apply.py" --check; fi
exec python3 -I "$here/staged/apply.py"
