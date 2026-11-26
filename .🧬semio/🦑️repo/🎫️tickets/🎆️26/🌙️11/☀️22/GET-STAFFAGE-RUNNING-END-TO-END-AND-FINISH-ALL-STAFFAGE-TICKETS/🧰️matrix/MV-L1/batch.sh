#!/bin/zsh
# 🚚️ The proof of several plants of batch MV-L1 as two chains, each inside ONE reservation of the memory queue (two requests at a time, the figures of MM in the takeover brief): `batch.sh a <id>…` (1600 MB) writes each catalog file (`make.sh` twice: the second run must not change a byte), marks it in `$GEN/proof/<id>.made` and runs the checks 1 to 4 and 11 of `prove.sh` (gate, scan, guard with validator and sweep, check CLI, matrix); `batch.sh b <id>…` (2500 MB) waits for that mark and runs the checks 5 to 10 (the six suites with a name filter). The lines of both land in `$GEN/proof/<id>.txt`.
source "${0:A:h}/env.sh"
chain="$1"; shift; mkdir -p "$GEN/log" "$GEN/proof"; export MEMLOG="$GEN/log/mem-batch-$chain.txt"
if [[ "$chain" == a ]]; then
  cd "$R" && INSIDE=1 repo capacity run --memory=1600 -- zsh -c 'for a in "$@"; do "$W/make.sh" "$a" > "$GEN/proof/$a-make.txt" 2>&1; "$W/make.sh" "$a" 2>&1 | tail -1 | sed "s/^/second run: /" >> "$GEN/proof/$a-make.txt"; touch "$GEN/proof/$a.made"; ONLY="1 2 3 4 11" "$W/prove.sh" "$a"; done' batch "$@"
else
  cd "$R" && INSIDE=1 repo capacity run --memory=2500 -- zsh -c 'for a in "$@"; do until [[ -e "$GEN/proof/$a.made" ]]; do sleep 5; done; ONLY="5 6 7 8 9 10" "$W/prove.sh" "$a"; done' batch "$@"
fi
