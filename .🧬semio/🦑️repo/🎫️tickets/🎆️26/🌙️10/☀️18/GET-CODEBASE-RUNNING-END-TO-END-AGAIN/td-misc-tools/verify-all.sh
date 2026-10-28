#!/bin/sh
# ✅ [td-misc] creates (unless it exists) a mirror with the repository's Cargo manifests (extra-only.txt), applies every td-misc batch to it and runs every batch's verify.sh. usage: sh verify-all.sh <mirror name, e.g. tmv>  (drop it afterwards: python3 -I ../mirror-tools/mirror.py drop <name>; detach its .cache/b/*/m images first with hdiutil detach)
set -e
here=$(cd "$(dirname "$0")" && pwd); C=$(dirname "$here"); m="${1:?mirror name}"
if [ ! -d "${TMPDIR%/}/sm/$m" ]; then
 set -- library process launcher 🧰️framework/🔨️modules/⚙️runtime; while IFS= read -r l; do set -- "$@" "$l"; done < "$here/extra-only.txt"
 python3 -I "$C/mirror-tools/mirror.py" create "$m" --only "$@" | tail -2
fi
R=$(cd "${TMPDIR%/}/sm/$m" && pwd -P)
for b in texts artdir resident windows consumption; do echo "== $b"; python3 -I "$here/staged/$b/stage.py" apply --root "$R"; sh "$here/staged/$b/verify.sh" "$R" 2>&1 | grep -aE '^\(fail\)|^ *[0-9]+ (pass|fail|skip)$|DRIFTED|^error'; done
