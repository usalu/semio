#!/bin/sh
# usage: run_g1.sh <tag> [configs]
cd "$(dirname "$0")"
TAG=$1
CFG=${2:-lib}
OV=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pi1/overlay-g1.json
python3 uo.py $TAG --mode crates --overlay $OV --extra-real child,composition-emission,composition-publication,publication-mounted --configs $CFG --vs a1 > ../🗑️generated/pi1-$TAG.log 2>&1
echo finished >> ../🗑️generated/pi1-$TAG.log
