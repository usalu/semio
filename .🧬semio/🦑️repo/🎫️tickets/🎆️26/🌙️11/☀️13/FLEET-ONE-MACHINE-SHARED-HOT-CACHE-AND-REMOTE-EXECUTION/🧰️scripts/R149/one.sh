#!/bin/zsh
h=$(echo "$1" | md5 -q)
dir=${1%%::*}; pat=""; [[ "$1" == *::* ]] && pat=${1#*::}
if [ -n "$pat" ]; then
 cd "$LIB/$dir" && env -u SEMIO_FLEET_GROUP $BUN test ./🟦️.ts -t "$pat" --timeout 180000 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)$" > "$SC/res.$h"
else
 cd "$LIB/$dir" && env -u SEMIO_FLEET_GROUP $BUN test ./🟦️.ts --timeout 180000 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)$" > "$SC/res.$h"
fi
