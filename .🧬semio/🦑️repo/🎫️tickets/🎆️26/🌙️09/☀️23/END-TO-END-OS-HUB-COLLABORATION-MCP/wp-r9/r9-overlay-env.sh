#!/bin/zsh
# 🪞️ R9 overlay runner: runs one command inside the scratch overlay with git reading the real object store through a private index copy (the real index is never written), private cargo dirs, nice 10.
OVERLAY="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-r9-overlay"
cd "$OVERLAY" || exit 2
export GIT_DIR="/Users/ueli/Documents/semio/.git" GIT_WORK_TREE="$OVERLAY" GIT_INDEX_FILE="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-r9-git-index" GIT_OPTIONAL_LOCKS=0
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-r9-build" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-r9-target"
echo "START $(date '+%H:%M:%S') $*" >&2
nice -n 10 "$@"
rc=$?
echo "END rc=$rc $(date '+%H:%M:%S')" >&2
exit $rc
