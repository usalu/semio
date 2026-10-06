#!/bin/zsh
# 🚦️ Build gate v6 (rule 66): blocks until fewer than `${1:-3}` cargos run on the SHARED build dir — a cargo with its own
# `CARGO_BUILD_BUILD_DIR` (peers outside the fleet build in private dirs) takes no lock there and is not counted — and until the disk
# has the floor `${2:-12}` GiB free; while `🗑️generated/coord/activation.flag` exists (an activation builds) it stays closed. Prints one line when it opens; exits 5 after `${3:-900}` s without opening (no cargo then: stage,
# report, end the turn). Usage: `zsh T/🚦️gate.sh && CARGO_BUILD_JOBS=3 cargo check …`; test builds: `zsh T/🚦️gate.sh 3 25 && …`.
limit="${1:-3}"; floor="${2:-12}"; patience="${3:-900}"
shared() {
  local count=0 pid
  for pid in $(pgrep -x cargo); do
    ps eww -o command= -p "$pid" 2>/dev/null | /usr/bin/grep -q 'CARGO_BUILD_BUILD_DIR=' || count=$((count + 1))
  done
  echo "$count"
}
free() { df -g /System/Volumes/Data | awk 'NR>1{print $4}' }
waited=0
until [ ! -e "${0:A:h}/🗑️generated/coord/activation.flag" ] && [ "$(shared)" -lt "$limit" ] && [ "$(free)" -ge "$floor" ]; do
  [ "$waited" -ge "$patience" ] && { echo "GATE CLOSED $(date '+%T') shared-cargo=$(shared) free=$(free)GiB after ${waited}s"; exit 5 }
  sleep 15; waited=$((waited + 15))
done
echo "GATE OPEN $(date '+%T') shared-cargo=$(shared) free=$(free)GiB waited=${waited}s"
