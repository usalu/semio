#!/bin/bash
W="$1"; shift
BZ=/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel
cd "$W/ws"
export LANG=${BZL_LANG-en_US.UTF-8} LC_ALL=${BZL_LANG-en_US.UTF-8}
[ -z "${BZL_LANG-x}" ] && unset LANG LC_ALL
[ -n "$BZL_INSTALL" ] && set -- --install_base="$BZL_INSTALL" "$@"
exec "$BZ" --output_user_root="$BZL_OUR" --nosystem_rc --nohome_rc --bazelrc="$W/ws/.bazelrc" --max_idle_secs=120 "$@"
