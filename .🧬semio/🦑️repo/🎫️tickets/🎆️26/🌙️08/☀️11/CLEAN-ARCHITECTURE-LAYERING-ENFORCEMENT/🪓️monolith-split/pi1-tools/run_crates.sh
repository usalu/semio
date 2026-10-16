#!/bin/sh
# usage: run_crates.sh <tag> <baseline tag> <group name> <extra-real> [configs]
cd "$(dirname "$0")"
TAG=$1; BASE=$2; GRP=$3; EXTRA=$4; CFG=${5:-lib,test}
OV=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pi1/overlay-$GRP.json
python3 uo.py $TAG --mode crates --overlay $OV --extra-real $EXTRA --configs $CFG --vs $BASE > ../🗑️generated/pi1-$TAG.log 2>&1
echo finished >> ../🗑️generated/pi1-$TAG.log
