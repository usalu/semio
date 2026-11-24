#!/bin/zsh
# usage: run.sh LABEL PASS owner... ; one fresh process per owner; store $SP/gr19/LABEL/store kept across passes; outputs LABEL/pPASS-owner.{json,err,time}
G="$(cd "$(dirname "$0")" && pwd)"; SP=/private/tmp/claude-501/-Users-ueli-Documents-semio/caf759d9-ccba-4796-9909-827125acd710/scratchpad
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L=$1; P=$2; shift 2; D=$SP/gr19/$L; mkdir -p $D/store
cd /Users/ueli/Documents/semio
for name in "$@"; do
  [ -n "$PROFILE_OWNER" ] && [ "$PROFILE_OWNER" = "$name" ] && export PROFILE=$D/p$P-$name.cpuprofile || unset PROFILE
  /usr/bin/time -l env ${=FLEET_ENV} SEMIO_GRAPH_WALK_FARM=off DRAIN=1 WAVE=$SP/gr16/wave UNITS_FILE=$SP/gr19/units-all.json UNITS=@semio-tech/$name STORE=$D/store $B --preload "$G/preload.mjs" "$G/one-owner-v7.mjs" > $D/p$P-$name.json 2> $D/p$P-$name.err
  echo "$P $name $(date +%T) load $(sysctl -n vm.loadavg) store $(du -sk $D/store | cut -f1)KB" >> $D/log
done
