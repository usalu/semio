#!/bin/zsh
# 🩺️ W2: one-shot health line for a running catalog publish with a warm lane beside it: packages complete, stage, memory, swap, and every
# `cargo rustc` of a plugin with its rustc child count, so a lock wait (cargo alive, 0 children) is visible. usage: zsh w2-publish-health.sh
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-logs"
echo "$(date '+%T') $(tail -1 "$L/publish-all-4-samples.txt" | cut -d' ' -f2-3) mem=$(memory_pressure | tail -1 | /usr/bin/grep -o '[0-9]*%') swap=$(sysctl -n vm.swapusage | awk '{print $6}')"
for c in $(ps -axo pid,command | /usr/bin/grep '[c]argo rustc' | awk '{print $1}'); do
  pkg=$(ps -o command= -p $c | tr ' ' '\n' | /usr/bin/grep -A1 -- '^-p$' | tail -1)
  [ -z "$pkg" ] && pkg=$(ps -o command= -p $c | /usr/bin/grep -o 'plugins/[^ ]*' | head -1 | cut -c1-40)
  owner=other; a=$c; for i in 1 2 3 4 5 6 7 8 9 10; do a=$(ps -o ppid= -p $a | tr -d ' '); [ -z "$a" ] || [ "$a" = 1 ] && break; cmd=$(ps -o command= -p $a); case "$cmd" in *trusted-catalog-bootstrap*) owner=bootstrap; break;; *w2-warm-behind*) owner=lane; break;; esac; done
  echo "  $owner $c ${pkg:-?} rustc=$(ps -axo ppid | awk -v p=$c '$1==p' | wc -l | tr -d ' ') age=$(ps -o etime= -p $c | tr -d ' ')"
done
