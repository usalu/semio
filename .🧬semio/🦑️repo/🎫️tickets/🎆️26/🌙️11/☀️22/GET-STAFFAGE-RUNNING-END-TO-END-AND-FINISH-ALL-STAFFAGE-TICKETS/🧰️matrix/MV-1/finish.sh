#!/bin/zsh
# 🏁️ The whole chain of one plant of batch MV-1 inside ONE reservation of the memory queue (the queue of the machine is long: a chain of fifteen single requests waits fifteen times): `make.sh` (writes the catalog file, unless NOMAKE=1; STRIP=1 first removes the depth plates an earlier run of `carry.ts` wrote with other flags), then `prove.sh` (gate, scan, guard, check CLI, the six real tests, with MATRIX=1 the matrix). The processes run one after the other, the reservation is the largest of them: finish.sh <megabytes> <id>   (FROM=<n> as in prove.sh)
export BATCH=MV-1; source "${0:A:h}/../MR/env.sh"
mb="$1"; a="$2"; mkdir -p "$GEN/takeover"; export MEMLOG="$GEN/takeover/mem-$a.txt"
cd "$R" && INSIDE=1 repo capacity run --memory="$mb" -- zsh -c '[[ -z "$STRIP" ]] || "$1/q.sh" 800 "$T" 3000 "$B" "$1/carry.ts" "$2" --write --limit 99 | tail -1; [[ -n "$NOMAKE" ]] || "$1/make.sh" "$2"; "$1/prove.sh" "$2"' finish "$W" "$a"
