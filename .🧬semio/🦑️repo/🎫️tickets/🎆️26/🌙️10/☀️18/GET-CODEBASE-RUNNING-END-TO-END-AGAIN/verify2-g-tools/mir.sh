#!/bin/sh
# `mir.sh create|overlay|run|drop|path ...`: the verify2-g mirror (name v2g) in its own home so a peer's `mirror.py drop --all` cannot sweep it; `overlay` copies work/ over it, `run` is mirror.py run.
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)
export SEMIO_MIRROR_HOME="${TMPDIR:-/tmp}/v2g-sm"
MIR="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/mirror-tools/mirror.py"
verb=$1; shift
case "$verb" in
  path) echo "$SEMIO_MIRROR_HOME/v2g";;
  create) python3 -I "$MIR" create v2g "$@" && python3 -I "$HERE/overlay.py" "$SEMIO_MIRROR_HOME/v2g";;
  overlay) python3 -I "$HERE/overlay.py" "$SEMIO_MIRROR_HOME/v2g";;
  run) python3 -I "$MIR" run v2g "$@";;
  drop) python3 -I "$MIR" drop v2g;;
  *) echo "usage: mir.sh create|overlay|run|drop|path" >&2; exit 2;;
esac
