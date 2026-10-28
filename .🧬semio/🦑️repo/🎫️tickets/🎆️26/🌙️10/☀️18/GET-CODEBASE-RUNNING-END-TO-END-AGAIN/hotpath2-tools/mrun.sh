#!/bin/sh
# 🏃️ Runs a command in the mirror h2work with the artifact dir below this ticket's generated folder allowed through the seal: sh mrun.sh <cmd...>
HERE=$(cd "$(dirname "$0")" && pwd); C=$(dirname "$HERE"); REL=${C#"$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$C")"/}
mkdir -p "$C/🗑️generated/hotpath2/art"
SEMIO_TEST_ARTIFACT_DIR="$C/🗑️generated/hotpath2/art" exec python3 -I "$C/mirror-tools/mirror.py" run h2work --peers-ok --seal-allow "$REL/🗑️generated/hotpath2" -- "$@"
