#!/bin/zsh
# ↩️ Takes back the ops of the last trial of a person (try.sh): drop.sh <slug>
source "${0:A:h}/env.sh"
cp "$GEN/try/$1.json" "$W/ops/$1.json" && echo "$1: ops of the last trial dropped"
