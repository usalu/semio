#!/bin/bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"; G="$T/🗑️generated"
cd /Users/ueli/Documents/semio
touch "$G/exclusive-plan.active"
P="env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute"
$P repo run-many --targets=test --projects=@semio-tech/s-plugin-composition,@semio-tech/gis-gisterrain,@semio-tech/gis-gismap-js,@semio-tech/gis-js --parallel=1 --lang=en > "$G/excl-ts-test.log" 2>&1 &
ts=$!
$P repo run @semio-tech/framework-os-dev:dev-s-react-prototype --lang=en > "$G/excl-s-react.log" 2>&1 &
echo $! > "$G/excl-s-react.pid"
while kill -0 $ts 2>/dev/null; do
  grep -q "Reading the project graph slice" "$G/excl-ts-test.log" && ! tail -1 "$G/excl-ts-test.log" | grep -qE "Reading the project graph slice|Planning" && rm -f "$G/exclusive-plan.active"
  sleep 20
done
wait $ts; echo "ts rc=$?" >> "$G/excl-results.txt"
rm -f "$G/exclusive-plan.active"
