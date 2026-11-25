#!/bin/zsh
# 🖼️ The sheets of batch MV-1 the takeover looked at, rendered one after the other inside ONE reservation of the memory queue (2500 MB): every line of `sheets.tsv` is `<id> <tag: after|before|try> <pose or - for the default pose> <scale>…` and is handed to `look.sh`. sheets.sh [file]
export BATCH=MV-1; source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/takeover"; export MEMLOG="$GEN/takeover/mem-sheets.txt"
cd "$R" && INSIDE=1 repo capacity run --memory=2500 -- zsh -c 'while read -r a tag pose scales; do [[ -z "$a" ]] && continue; if [[ "$pose" == "-" ]]; then TAG="$tag" "$1/look.sh" "$a" ${=scales}; else TAG="$tag" POSE="$pose" "$1/look.sh" "$a" ${=scales}; fi; done < "$2"' sheets "$W" "${1:-$W/sheets.tsv}"
