#!/bin/sh
# usage: run_crates.sh <tag> <baseline tag> <group name> <extra-real> [configs]
cd "$(dirname "$0")"
TAG=$1; BASE=$2; GRP=$3; EXTRA=$4; CFG=${5:-lib,test}
OV=${PI2_SCRATCH:-/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/pi2w7}/pi2/overlay-$GRP.json
python3 uo.py $TAG --mode crates --overlay $OV --extra-real $EXTRA --configs $CFG --vs $BASE > ../🗑️generated/pi2-$TAG.log 2>&1
echo finished >> ../🗑️generated/pi2-$TAG.log
