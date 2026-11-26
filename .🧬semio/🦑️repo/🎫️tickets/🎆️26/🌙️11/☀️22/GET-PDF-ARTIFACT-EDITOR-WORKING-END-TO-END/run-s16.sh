#!/bin/sh
# usage: run-s16.sh  (slice 16: codec libs, then the institution and thesis routes, one gated cargo run each; logs s16-* in 🗑️generated)
cd "$(dirname "$0")"
sh ./run-s14-codec.sh s16-codec
sh ./retry-route.sh s16-inst registered_real_world_institution 3
sh ./retry-route.sh s16-thesis registered_bachelor_thesis 3
echo finished > 🗑️generated/s16-all.done
