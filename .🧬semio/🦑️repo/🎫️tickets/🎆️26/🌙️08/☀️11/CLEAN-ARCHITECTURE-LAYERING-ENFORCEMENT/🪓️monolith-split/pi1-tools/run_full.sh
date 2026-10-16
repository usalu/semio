#!/bin/sh
# usage: run_full.sh <tag> <group name> <extra-real comma list> [configs]
cd "$(dirname "$0")"
TAG=$1; GRP=$2; EXTRA=$3; CFG=${4:-lib,test}
OV=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pi1/overlay-$GRP.json
python3 uo.py $TAG --mode full --overlay $OV --extra-real $EXTRA --configs $CFG > ../🗑️generated/pi1-$TAG.log 2>&1
echo finished >> ../🗑️generated/pi1-$TAG.log
