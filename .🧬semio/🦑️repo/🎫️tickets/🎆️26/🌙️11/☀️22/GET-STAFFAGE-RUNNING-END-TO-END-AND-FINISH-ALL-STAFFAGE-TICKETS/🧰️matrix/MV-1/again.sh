#!/bin/zsh
# ♻️ The idempotence proof of the chain of plants of batch MV-1 inside ONE reservation of the memory queue (2500 MB: the anchor search of `english-oak` peaks at 1.8 GB): the digest of the catalog file, `make.sh` once more, the digest again; a chain that is idempotent changes no byte. again.sh <id>…
export BATCH=MV-1; source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/takeover"; export MEMLOG="$GEN/takeover/mem-again.txt"
cd "$R" && INSIDE=1 repo capacity run --memory=2500 -- zsh -c 'w="$1"; shift; for a in "$@"; do f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"; b="$(shasum -a 256 "$f" | cut -c1-16)"; "$w/make.sh" "$a" | tail -1; e="$(shasum -a 256 "$f" | cut -c1-16)"; echo "$a second run of the chain: $b -> $e $([[ "$b" == "$e" ]] && echo "no byte changed" || echo CHANGED)"; done' again "$W" "$@"
