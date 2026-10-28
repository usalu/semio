#!/bin/sh
# 🧰 cbudget lane: run a command in mirror cb-m with the lane artifact dir (usage: mrun.sh <cmd...>)
here=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$here/.." && pwd)
R=$(cd "$C/../../../../../../.." && pwd)
art="$C/🗑️generated/cbudget"
mkdir -p "$art"
rel=${art#"$R"/}
SEMIO_TEST_ARTIFACT_DIR="$art" exec python3 -I "$C/mirror-tools/mirror.py" run "${MIRROR:-cb-m}" --peers-ok --seal-allow "$rel" -- "$@"
