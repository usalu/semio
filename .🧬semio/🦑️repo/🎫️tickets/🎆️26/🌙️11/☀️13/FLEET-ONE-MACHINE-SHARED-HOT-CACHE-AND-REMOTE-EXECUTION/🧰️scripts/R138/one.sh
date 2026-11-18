#!/bin/zsh
h=$(echo "$1" | md5 -q)
cd "$LIB/$1" && env -u SEMIO_FLEET_GROUP $BUN test ./🟦️.ts --timeout 180000 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)$" > "$SC/res.$h"
