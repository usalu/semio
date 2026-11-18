#!/bin/zsh
h=$(echo "$1" | md5 -q)
dir=${1%%::*}; pat=""; [[ "$1" == *::* ]] && pat=${1#*::}
if [ -n "$pat" ]; then
 cd "$LIB/$dir" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test ./🟦️.ts -t "$pat" --timeout 180000 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)$" > "$SC/res.$h"
else
 cd "$LIB/$dir" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test ./🟦️.ts --timeout 180000 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)$" > "$SC/res.$h"
fi
