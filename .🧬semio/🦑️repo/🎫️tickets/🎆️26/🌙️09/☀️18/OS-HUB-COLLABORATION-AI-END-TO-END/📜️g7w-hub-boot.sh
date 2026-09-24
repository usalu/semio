#!/bin/zsh
# 🌍️ G7w: boots a two-principal credential-sign-in hub on 7900 from a copy of H7's binary (never built here).
set -e
cd /Users/ueli/Documents/semio
W=.tmp-ticket-0918/wp-g7w
mkdir -p $W/bin $W/generated
[ -x $W/bin/os-hub ] || cp .tmp-ticket/wp-h7/bin/os-hub $W/bin/os-hub
B="$PWD/$W/bin/os-hub"; D=/private/tmp/g7w-hub-data-7900
rm -rf $D; mkdir -p $D
for who in "ada@example.org:Ada Lovelace" "bo@example.org:Bo Peep"; do
  printf 'correct horse battery staple' | OS_HUB_DATA=$D "$B" credential set --email "${who%%:*}" --display-name "${who#*:}" >/dev/null
done
OS_HUB_CREDENTIAL_SIGN_IN=1 SEMIO_TRACE_LEVEL=info SEMIO_TRACE_SINK="file:$PWD/$W/generated/g7w-hub-7900-trace.txt" nohup bun ".tmp-ticket-0918/🐍️c8-hub-hold.ts" 7900 $D "$B" > $W/generated/g7w-hub-7900.txt 2>&1 &
echo "hold pid $!"
disown
until /usr/bin/grep -q '^HOLD' $W/generated/g7w-hub-7900.txt; do sleep 1; done
/usr/bin/grep -o 'HOLD origin=[^ ]* pid=[0-9]*' $W/generated/g7w-hub-7900.txt
