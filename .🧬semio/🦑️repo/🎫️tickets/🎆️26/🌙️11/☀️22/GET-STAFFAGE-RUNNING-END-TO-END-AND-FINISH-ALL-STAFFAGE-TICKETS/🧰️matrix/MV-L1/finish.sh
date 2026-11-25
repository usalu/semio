#!/bin/zsh
# 🏁️ The whole chain of one plant of batch MV-L1 inside ONE reservation of the memory queue (the reservation is the largest of its processes, which run one after the other; peaks go to `$GEN/log/mem-<id>.txt`): `make.sh` (writes the catalog file, unless NOMAKE=1), a second `make.sh` (the idempotence proof: no byte may change), then `prove.sh`. finish.sh <megabytes> <id>   (FROM=<n> as in prove.sh)
source "${0:A:h}/env.sh"
mb="$1"; a="$2"; mkdir -p "$GEN/log"; export MEMLOG="$GEN/log/mem-$a.txt"
cd "$R" && INSIDE=1 repo capacity run --memory="$mb" -- zsh -c '[[ -n "$NOMAKE" ]] || { "$W/make.sh" "$1"; "$W/make.sh" "$1" | tail -1 | sed "s/^/second run: /"; }; "$W/prove.sh" "$1"' finish "$a"
