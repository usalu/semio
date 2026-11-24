#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../../../.." && pwd)
mountpoint="$root/.cache/b/p/m"
if [ -d "$mountpoint/browser/prototype/modules" ]; then exit 0; fi
hdiutil attach -nobrowse -noverify -mountpoint "$mountpoint" "$root/.cache/b/p/fixed.dmg"
