#!/bin/sh
# usage: [SEMIO_CHAIN_ONLY="base sweep"] publish-chain.sh   (one heavy lane: base, energy, eurocode, structural, materials, then the strict 15-route sweep)
# Each step retries without stopping on admission timeouts, warden cancellations and source-boundary refusals of a
# carve in flight, and up to 5 times on a descriptor emitter prerequisite build failure (a killed or raced build leaves no rustc diagnostics; a real compile error fails on the sixth); a refusal naming the driver's requirement N sets the next outer grant to max(N + 200, current spare MiB above the floor). Every step starts at 4400 MiB; the driver refuses at once with "needs N" when a pass needs more, and the next grant follows N; base runs last, after the hub-norm carve and enters the capacity queue only once the spare memory covers its grant (at most 5 min, then it queues), so it never blocks smaller jobs at the head of the queue.
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="${SEMIO_PUBLISH_LOGS:-/private/tmp/claude-501/-Users-ueli-Documents-semio/9ea38fee-6468-41de-903a-f7fec4948a18/scratchpad}"; mkdir -p "$G"
N="🌎️hub/🧩️compositions/📕️norm"
RETRY='SIGTERM|capacity deadline|timed out|waited 1,?8|Wächter|warden|source boundary|over its ceiling|Nothing is retried|concurrent|exited 75|artifact bytes or identity changed|loan deadline exhausted'
spare() { repo capacity status 2>/dev/null | head -1 | sed -n 's/.*floor [0-9]* MB, \(-\{0,1\}[0-9]*\) MB spare.*/\1/p'; }
step() { # name, memory, command...
  name=$1; mem=$2; shift 2; n=0; em=0
  case " ${SEMIO_CHAIN_ONLY:-$name} " in *" $name "*) ;; *) return 0;; esac
  while [ $n -lt 200 ]; do
    n=$((n+1))
    case "$name" in sweep) "$@" > "$G/chain-$name.log" 2>&1; rc=$?;;
      *) w=0; while [ $w -lt 15 ]; do sp=$(spare); [ "${sp:-0}" -ge "$mem" ] && break; w=$((w+1)); sleep 20; done
         SEMIO_COMPILER_PARALLELISM=1 repo capacity run --memory="$mem" --cpu=1 -- repo "$T/native-exec.ts" "$PWD" './🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts' component --manifest "$1" > "$G/chain-$name.log" 2>&1; rc=$?;; esac
    echo "$(date +%H:%M:%S) $name attempt $n mem=$mem rc=$rc" >> "$G/chain.attempts"
    [ $rc -eq 0 ] && return 0
    need=$(grep -a -o "Component build needs [0-9]* MiB for the local pass" "$G/chain-$name.log" | tail -1 | grep -o "[0-9]*")
    if [ -n "$need" ]; then sp=$(spare); sp=${sp:-0}; mem=$((need+200)); [ "$sp" -gt "$mem" ] && mem=$sp; continue; fi
    if grep -qa "Descriptor emitter prerequisite build failed" "$G/chain-$name.log" && [ $em -lt 5 ]; then em=$((em+1)); cp "$G/chain-$name.log" "$G/chain-$name.attempt$n.log"; sleep 60; continue; fi
    if grep -qaE "$RETRY" "$G/chain-$name.log"; then cp "$G/chain-$name.log" "$G/chain-$name.attempt$n.log"; sleep 60; continue; fi
    cp "$G/chain-$name.log" "$G/chain-$name.failed.log"; echo "$(date +%H:%M:%S) $name FAILED (not retried)" >> "$G/chain.attempts"; return 1
  done
  return 1
}
step energy 4400 "$N/🧩️extensions/📒️energy/🧩️component/📦️packages/🦀️rust/Cargo.toml"
step eurocode 4400 "$N/🧩️extensions/📗️eurocode/🧩️component/📦️packages/🦀️rust/Cargo.toml"
step structural 4400 "$N/🧩️extensions/📓️eurocode-structural/🧩️component/📦️packages/🦀️rust/Cargo.toml"
step materials 4400 "$N/🧩️extensions/📙️eurocode-materials/🧩️component/📦️packages/🦀️rust/Cargo.toml"
step base 4400 "$N/🧩️component/📦️packages/🦀️rust/Cargo.toml"
step sweep 0 repo "$T/verify-standards-publication.ts"
echo "$(date +%H:%M:%S) chain done" >> "$G/chain.attempts"
