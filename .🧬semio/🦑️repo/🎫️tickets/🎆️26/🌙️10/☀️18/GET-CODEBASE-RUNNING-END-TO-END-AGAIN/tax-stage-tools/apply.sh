#!/bin/sh
# usage: apply.sh check [-v] | diff [substring...] | apply [--restage] [--dry-run] [--ignore-sync] | mirror DIR [--no-ops] [--copy RELATIVE_DIR]... | restage
# restage re-derives the manifest from the live tree (script bodies, importers and fixtures are re-read; the reference index in tax-stage-tools/index/ is reused).
# apply --restage = restage, then apply: use it when `check` reports conflicts because peers edited a hot file since staging.
HERE="$(cd "$(dirname "$0")" && pwd)"
if [ "${1:-}" = restage ]; then exec bun "$HERE/stage.ts"; fi
if [ "${1:-}" = apply ]; then
  case " $* " in *" --restage "*) bun "$HERE/stage.ts" > /dev/null || exit 1;; esac
  shift; set -- apply $(for a in "$@"; do [ "$a" = "--restage" ] || printf '%s\n' "$a"; done)
fi
exec python3 "$HERE/apply.py" "$@"
