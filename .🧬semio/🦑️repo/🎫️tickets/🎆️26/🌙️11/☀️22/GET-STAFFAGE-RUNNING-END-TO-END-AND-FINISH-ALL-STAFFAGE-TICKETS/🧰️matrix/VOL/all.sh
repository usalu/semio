#!/bin/zsh
# ⛓️ The long chain in one queue grant of $MB (default 1500 MB), asked for again when the queue gives up: 🧊️model-catalog of the 27, then 🧊️model-catalog of every other asset (resumable): all.sh
me="${0:A}"; source "${0:A:h}/env.sh"
if [[ "$1" != "--inside" ]]; then cd "$R"; mkdir -p "$GEN"; until [[ -e "$GEN/all.started" ]]; do SEMIO_LANGUAGE=en repo capacity run --memory="${MB:-1500}" -- timeout 80000 "$me" --inside; sleep 5; done; exit; fi
touch "$GEN/all.started"
SLICE=9 "$W/catalog.sh" --inside $IDS
SLICE=8 "$W/catalog.sh" --inside
