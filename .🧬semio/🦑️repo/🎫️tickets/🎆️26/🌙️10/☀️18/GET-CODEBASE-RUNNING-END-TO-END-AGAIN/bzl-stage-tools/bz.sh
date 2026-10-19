#!/bin/bash
# usage: bz.sh <bazel args...>   (runs the provisioned Bazel 8.4.2 against the generated breadth workspace)
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
BZ=${SEMIO_TEST_BAZEL_BINARY:-$ROOT/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel}
WS=${BZL_STAGE_WS:-$C/🗑️generated/bzl-stage-ws}
OB=${BZL_STAGE_OB:-/private/tmp/bzlstage-ob}
export LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8
cmd=$1; shift
cd "$WS" || exit 2
extra=()
[ "$cmd" = build ] || [ "$cmd" = cquery ] && extra=(--incompatible_strict_action_env)
exec "$BZ" --output_base="$OB" --install_base=/private/tmp/bzlstage-install --nosystem_rc --nohome_rc --bazelrc="$ROOT/.bazelrc" --max_idle_secs=120 --host_jvm_args=-Xmx6g "$cmd" --lockfile_mode=off --incompatible_autoload_externally= "${extra[@]}" --keep_going "$@"
