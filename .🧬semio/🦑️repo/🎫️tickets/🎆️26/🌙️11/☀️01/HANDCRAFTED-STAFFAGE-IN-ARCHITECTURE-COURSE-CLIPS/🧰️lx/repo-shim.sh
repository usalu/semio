#!/usr/bin/env bash
# Stand-in for `repo tools exec --light bun -- ...` using the Bun the repo launcher provisioned.
[ "$1 $2 $3 $4" = "tools exec --light bun" ] || { echo "shim: unsupported: $*" >&2; exit 2; }
shift 5
"$(ls /home/ueli/.cache/semio/runtimes/bun/*/bun | head -1)" "$@"
exit $?
