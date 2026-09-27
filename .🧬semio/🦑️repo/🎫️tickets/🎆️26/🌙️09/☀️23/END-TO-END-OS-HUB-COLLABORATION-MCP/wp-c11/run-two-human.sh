#!/bin/zsh
# 👥️ C11: V1's permanent two-human journey (every creatable kind) against a hub through two serves.
# usage: zsh run-two-human.sh <tag> <en|de> <hubUrl> <serveA> <serveB> <adminCapabilityFile> [kinds]
cd /Users/ueli/Documents/semio || exit 1
echo "START $(date '+%F %T') tag=$1 locale=$2 pid=$$"
NX_DAEMON=false nice -n 10 bun nx run @semio-tech/framework-os-dev:two-human -- --hub "$3" --serve "$4" --serve-b "$5" --locale "$2" --admin-capability "$6" --tag "$1" ${7:+--kinds "$7"}
echo "EXIT rc=$? $(date '+%F %T')"
