#!/bin/sh
# usage: run_full.sh <tag> <group name> <extra-real comma list> [configs]
cd "$(dirname "$0")"
TAG=$1; GRP=$2; EXTRA=$3; CFG=${4:-lib,test}
OV=${PI2_SCRATCH:-/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/pi2w7}/pi2/overlay-$GRP.json
python3 uo.py $TAG --mode full --overlay $OV --extra-real $EXTRA --configs $CFG > ../🗑️generated/pi2-$TAG.log 2>&1
echo finished >> ../🗑️generated/pi2-$TAG.log
