#!/bin/sh
# usage: bz.sh <slot> <workspace> <verb> [args...]   (env: CB=/private/tmp/conc-bazel, HEAP=1024)
CB=${CB:-/private/tmp/conc-bazel}
BAZEL=/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel
slot=$1; ws=$2; verb=$3; shift 3
cd "$ws" || exit 9
exec "$BAZEL" --nosystem_rc --nohome_rc --noworkspace_rc --bazelrc=/dev/null \
  --output_base="$CB/ob/$slot" --install_base="$CB/install" --max_idle_secs=${IDLE:-120} \
  --host_jvm_args=-Xmx${HEAP:-1024}m "$verb" \
  --disk_cache="$CB/disk" --repository_cache="$CB/catalog/repositories" --repository_disable_download \
  --lockfile_mode=error --incompatible_autoload_externally= --experimental_convenience_symlinks=ignore \
  --nobuild_runfile_links --color=no --curses=no --local_resources=cpu=2 --local_resources=memory=1024 --jobs=2 \
  ${REPO_CONTENTS:+--repo_contents_cache="$REPO_CONTENTS"} $EXTRA "$@"
