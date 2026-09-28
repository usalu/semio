#!/bin/zsh
# 🔐 Fleet mutex: FIFO lanes with N slots (N from /tmp/semio-<lane>-build.slots, default 1; slot 1 = the historical lock dir). `wasm` shares the fleet-wide lock of ticket 26/09/18.
# usage: zsh 📜️fleet-mutex.sh <wasm|native|overlay|hub|…> <slice> -- <command …>
name="$1"; slice="$2"; shift 2; [ "$1" = "--" ] && shift
case "$name" in
  wasm) lock="/tmp/semio-wasm-build.lock"; queue="/tmp/semio-wasm-build.queue" ;;
  *) lock="/tmp/semio-$name-build.lock"; queue="/tmp/semio-$name-build.queue" ;;
esac
slotsfile="/tmp/semio-$name-build.slots"
mkdir -p "$queue"
ticket="$queue/${FLEET_TICKET_STAMP:-$(date '+%Y%m%d%H%M%S')}-$$-$slice~v2"
echo $$ > "$ticket"
trap 'rm -f "$ticket"' EXIT
trap 'rm -f "$ticket"; exit 143' INT TERM
held=""
while [ -z "$held" ]; do
  for t in "$queue"/*(N); do
    tp=$(cat "$t" 2>/dev/null)
    if [ -z "$tp" ] || ! kill -0 "$tp" 2>/dev/null; then rm -f "$t"; fi
  done
  slots=$(cat "$slotsfile" 2>/dev/null); [[ "$slots" == <1-> ]] || slots=1
  names=("${(@f)$(ls "$queue" 2>/dev/null | sort)}")
  me="${ticket:t}"; ahead=0; ahead_v2=0
  for n in $names; do [ "$n" = "$me" ] && break; ahead=$((ahead + 1)); [[ "$n" == *~v2 ]] && ahead_v2=$((ahead_v2 + 1)); done
  first=1
  if [ "$ahead" -ge "$slots" ]; then first=2; [ "$ahead_v2" -ge $((slots - 1)) ] && first=0; fi
  if [ "$first" -gt 0 ]; then
    for i in {$first..$slots}; do
      l="$lock"; [ "$i" -gt 1 ] && l="$lock.$i"
      if mkdir "$l" 2>/dev/null; then held="$l"; break; fi
      holder_pid=$(cat "$l/pid" 2>/dev/null)
      if [ -n "$holder_pid" ] && ! kill -0 "$holder_pid" 2>/dev/null; then rm -rf "$l"; continue; fi
      if [ -z "$holder_pid" ] && [ -n "$(find "$l" -maxdepth 0 -mmin +3 2>/dev/null)" ]; then rm -rf "$l"; continue; fi
    done
  fi
  [ -z "$held" ] && sleep 15
done
echo $$ > "$held/pid"; echo "$slice $(date '+%H:%M:%S')" > "$held/owner"
rm -f "$ticket"
trap 'rm -rf "$held"' EXIT
trap 'rm -rf "$held"; exit 143' INT TERM
case "$name" in native|overlay|wasm) j="${CARGO_BUILD_JOBS:-4}"; [ "$j" -gt 4 ] 2>/dev/null && j=4; export CARGO_BUILD_JOBS="$j" ;; esac
while [ "$(sysctl -n vm.loadavg | awk '{printf "%d", $2}')" -ge "${FLEET_LOAD_GATE:-32}" ]; do sleep 20; done
"$@"
rc=$?
trap - EXIT INT TERM
rm -rf "$held"
exit $rc
